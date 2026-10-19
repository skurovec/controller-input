pub mod commands;
pub mod config;
pub mod controller;
pub mod input_sim;
pub mod mapper;

use commands::AppState;
use controller::Controller;
use input_sim::InputSimulator;
use mapper::Mapper;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tauri::RunEvent;

pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Controller Input...");

    let app_state = Arc::new(AppState::new());
    let state_for_thread = Arc::clone(&app_state);
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();

    // Start the input loop in a separate thread
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        input_loop(state_for_thread, shutdown_receiver);
    });

    log::info!("Building Tauri app...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Arc::clone(&app_state))
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::start_mapping,
            commands::stop_mapping,
            commands::toggle_mapping,
            commands::get_config,
            commands::update_config,
            commands::update_settings,
            commands::update_button_mapping,
            commands::reset_config,
        ])
        .setup(move |_app| {
            log::info!("Setup complete!");
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |_app_handle, event| {
            if let RunEvent::Exit = event {
                release_input_before_exit(&shutdown_sender);
            }
        });
}

/// Asks the input loop to release held keys and stop, and waits briefly for
/// its confirmation so no key or mouse button stays pressed after exit.
fn release_input_before_exit(shutdown: &Sender<Sender<()>>) {
    let (released_sender, released_receiver) = mpsc::channel();
    if shutdown.send(released_sender).is_err() {
        // The input loop has already ended and dropped its receiver.
        return;
    }
    if let Err(e) = released_receiver.recv_timeout(Duration::from_millis(250)) {
        log::warn!("Input loop did not confirm key release before exit: {}", e);
    }
}

fn input_loop(state: Arc<AppState>, shutdown: Receiver<Sender<()>>) {
    if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        input_loop_inner(state, shutdown);
    })) {
        log::error!("Input loop panicked: {:?}", e);
    }
}

fn input_loop_inner(state: Arc<AppState>, shutdown: Receiver<Sender<()>>) {
    log::info!("Input loop starting...");

    // Initialize controller
    log::info!("Initializing controller...");
    let mut controller = match Controller::new() {
        Ok(c) => {
            log::info!("Controller initialized successfully");
            c
        }
        Err(e) => {
            log::error!("Failed to initialize controller: {}", e);
            return;
        }
    };

    // Input simulator will be initialized only when user starts mapping
    let mut input_sim: Option<InputSimulator> = None;

    // Initialize mapper
    let config = state.config.lock().clone();
    let mut mapper = Mapper::new(config);

    // Track axis values for continuous mouse movement
    let mut axis_values: HashMap<String, f32> = HashMap::new();

    // Update interval for mouse movement (approximately 120Hz)
    let target_interval = Duration::from_micros(8333);
    let mut last_update = Instant::now();

    // Track if we were running last frame (to release keys only once when stopping)
    let mut was_running = false;

    // Update initial controller state
    state
        .controller_connected
        .store(controller.is_connected(), Ordering::SeqCst);
    if let Some(name) = controller.gamepad_name() {
        *state.controller_name.lock() = Some(name);
    }

    loop {
        let loop_start = Instant::now();

        if let Ok(released) = shutdown.try_recv() {
            state.is_running.store(false, Ordering::SeqCst);
            let release_actions = mapper.release_all_keys();
            if let Some(ref mut sim) = input_sim {
                for action in release_actions {
                    sim.execute(action);
                }
            }
            if released.send(()).is_err() {
                log::warn!("Released held keys after the exit handler stopped waiting");
            }
            log::info!("Input loop stopped for exit");
            return;
        }

        // Poll controller events
        let events = controller.poll_events();
        // A Connected event may signal a swap to a different physical
        // controller (the active one disconnected and another took over). In
        // that case any axis values and any keys/mouse buttons the mapper is
        // currently holding belong to the previous controller and would
        // otherwise stay stuck at the OS level.
        let mut controller_swap = false;
        for event in &events {
            match event {
                controller::ControllerEvent::Connected(name) => {
                    state.controller_connected.store(true, Ordering::SeqCst);
                    *state.controller_name.lock() = Some(name.clone());
                    axis_values.clear();
                    controller_swap = true;
                }
                controller::ControllerEvent::Disconnected => {
                    state.controller_connected.store(false, Ordering::SeqCst);
                    *state.controller_name.lock() = None;
                    // Stop mapping when controller disconnects
                    state.is_running.store(false, Ordering::SeqCst);
                }
                controller::ControllerEvent::AxisChanged(axis, value) => {
                    axis_values.insert(axis.clone(), *value);
                }
                _ => {}
            }
        }

        // Release any keys/mouse buttons the mapper still holds for the
        // previous controller before processing this iteration's input
        // events. Done unconditionally so mapper internal state stays in
        // sync with the OS even when input_sim isn't initialized yet.
        if controller_swap {
            let release_actions = mapper.release_all_keys();
            if let Some(ref mut sim) = input_sim {
                for action in release_actions {
                    sim.execute(action);
                }
            }
        }

        // Only process inputs if mapping is active
        let is_running = state.is_running.load(Ordering::SeqCst);

        if is_running {
            // Try to initialize input simulator if not yet done
            if input_sim.is_none() {
                match InputSimulator::new() {
                    Ok(sim) => {
                        log::info!("Input simulator initialized successfully");
                        input_sim = Some(sim);
                    }
                    Err(e) => {
                        log::error!("Failed to initialize input simulator: {}", e);
                        // Stop mapping since we can't simulate input
                        state.is_running.store(false, Ordering::SeqCst);
                        continue;
                    }
                }
            }

            if let Some(ref mut sim) = input_sim {
                // Check if config was updated
                let current_config = state.config.lock().clone();
                let config_release_actions = mapper.update_config(current_config);
                for action in config_release_actions {
                    sim.execute(action);
                }

                // Process button events
                for event in &events {
                    let actions = mapper.map_event(event);
                    for action in actions {
                        sim.execute(action);
                    }
                }

                // Update mouse movement at fixed interval
                let elapsed = last_update.elapsed();
                if elapsed >= target_interval {
                    if let Some(mouse_action) = mapper.update_mouse_movement(&axis_values) {
                        sim.execute(mouse_action);
                    }
                    last_update = Instant::now();
                }
            }

            was_running = true;
        } else if was_running {
            // When stopped, release any held keys (only once)
            if let Some(ref mut sim) = input_sim {
                let release_actions = mapper.release_all_keys();
                for action in release_actions {
                    sim.execute(action);
                }
            }
            was_running = false;
        }

        // Sleep to prevent CPU spinning, but account for processing time
        let elapsed = loop_start.elapsed();
        if elapsed < Duration::from_millis(1) {
            thread::sleep(Duration::from_millis(1) - elapsed);
        }
    }
}
