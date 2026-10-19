use gilrs::{Axis, Button, Event, EventType, GamepadId, Gilrs};
use std::collections::HashMap;

pub struct Controller {
    gilrs: Gilrs,
    // The gamepad whose input events we process. `None` means no controller
    // is bound yet — it will be set exactly once, on the first intentional
    // input event (button press or strong stick deflection) from any
    // connected gamepad. Once set, events from any other GamepadId are
    // dropped for the lifetime of the binding. The binding is cleared only
    // when the bound gamepad is reported as Disconnected by gilrs.
    active_gamepad: Option<GamepadId>,
    axis_states: HashMap<Axis, f32>,
    button_states: HashMap<Button, bool>,
}

#[derive(Debug, Clone)]
pub enum ControllerEvent {
    ButtonPressed(String),
    ButtonReleased(String),
    AxisChanged(String, f32),
    Connected(String),
    Disconnected,
}

impl Controller {
    pub fn new() -> Result<Self, String> {
        let result = std::panic::catch_unwind(|| Gilrs::new().map_err(|e| e.to_string()));

        let gilrs = match result {
            Ok(Ok(g)) => g,
            Ok(Err(e)) => return Err(format!("Failed to initialize gilrs: {}", e)),
            Err(_) => return Err("Gilrs initialization panicked".to_string()),
        };

        let controller = Controller {
            gilrs,
            active_gamepad: None,
            axis_states: HashMap::new(),
            button_states: HashMap::new(),
        };

        // Log enumeration only — no speculative binding. Active gamepad is
        // chosen lazily on the first intentional input event.
        controller.log_enumeration();

        Ok(controller)
    }

    fn log_enumeration(&self) {
        // On Windows, gilrs may expose the same physical Xbox controller through
        // multiple backends (XInput + rawinput/WGI) as separate GamepadIds. Log
        // every gamepad gilrs sees so the user can diagnose detection issues.
        let mut connected_ids: Vec<GamepadId> = Vec::new();
        for (id, gamepad) in self.gilrs.gamepads() {
            log::info!(
                "Enumerated gamepad: name={:?}, id={:?}, connected={}",
                gamepad.name(),
                id,
                gamepad.is_connected()
            );
            if gamepad.is_connected() {
                connected_ids.push(id);
            }
        }
        // Surface the ambiguous case loudly. If multiple connected gamepads
        // share the same model signature, gilrs/the OS can't tell "one device
        // seen via two backends" from "two identical devices" — whichever
        // fires the first intentional input will bind.
        if connected_ids.len() >= 2 {
            for i in 0..connected_ids.len() {
                for j in (i + 1)..connected_ids.len() {
                    let a = connected_ids[i];
                    let b = connected_ids[j];
                    if self.is_same_physical(a, b) {
                        log::warn!(
                            "Multiple connected gamepads share a model signature \
                             ({} / id={:?} vs {} / id={:?}). Cannot distinguish \
                             dual-backend duplicate from a real second controller; \
                             the first intentional input will bind. Disconnect one \
                             if you want a deterministic primary.",
                            self.gilrs.gamepad(a).name(),
                            a,
                            self.gilrs.gamepad(b).name(),
                            b
                        );
                    }
                }
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        // Reports any connected gamepad — UI shows "connected" even before
        // the first intentional input has actually bound a primary.
        self.gilrs.gamepads().any(|(_, gp)| gp.is_connected())
    }

    pub fn gamepad_name(&self) -> Option<String> {
        // Prefer the bound gamepad's name; otherwise show the first connected
        // gamepad's name as a UI label (informational only — it does not
        // imply that gamepad will become the bound primary).
        if let Some(id) = self.active_gamepad {
            return Some(self.gilrs.gamepad(id).name().to_string());
        }
        self.gilrs
            .gamepads()
            .find(|(_, gp)| gp.is_connected())
            .map(|(_, gp)| gp.name().to_string())
    }

    pub fn poll_events(&mut self) -> Vec<ControllerEvent> {
        let mut events = Vec::new();

        loop {
            // Wrap each event poll in panic catch
            let next_event = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.gilrs.next_event()
            })) {
                Ok(ev) => ev,
                Err(_) => {
                    log::error!("Gilrs next_event panicked");
                    break;
                }
            };

            let Some(Event { id, event, .. }) = next_event else {
                break;
            };

            match event {
                EventType::Connected => {
                    let name = self.gilrs.gamepad(id).name().to_string();
                    log::info!("Gamepad connected: {} (id: {:?})", name, id);
                    // Emit a Connected event for the UI label only when this
                    // is the first connected gamepad we see. This does NOT
                    // bind the gamepad — binding only happens on the first
                    // intentional input. A second backend (e.g. the Windows
                    // dual-backend duplicate) firing Connected for the same
                    // physical controller therefore doesn't update the UI.
                    let other_already_connected = self
                        .gilrs
                        .gamepads()
                        .any(|(other_id, gp)| other_id != id && gp.is_connected());
                    if !other_already_connected && self.active_gamepad.is_none() {
                        events.push(ControllerEvent::Connected(name));
                    }
                    continue;
                }
                EventType::Disconnected => {
                    log::info!("Gamepad disconnected (id: {:?})", id);
                    let was_bound = self.active_gamepad == Some(id);
                    if was_bound {
                        self.active_gamepad = None;
                    }
                    // Re-check what's still connected. If something remains
                    // and we just lost the bound gamepad, surface a Connected
                    // event with the remaining gamepad's name so the UI label
                    // updates without stopping the active mapping session
                    // (Disconnected would stop it). The next intentional
                    // input from the remaining gamepad will bind it.
                    let remaining = self
                        .gilrs
                        .gamepads()
                        .find(|(_, gp)| gp.is_connected())
                        .map(|(other_id, gp)| (other_id, gp.name().to_string()));
                    match remaining {
                        Some((_, name)) => {
                            if was_bound {
                                events.push(ControllerEvent::Connected(name));
                            }
                        }
                        None => {
                            events.push(ControllerEvent::Disconnected);
                        }
                    }
                    continue;
                }
                _ => {}
            }

            // Input event.
            //
            // Binding rules (deliberately simple, no swap path):
            //   - If no gamepad is bound yet: bind to this one, but only if
            //     the event is intentional (button press or strong stick
            //     deflection). Idle/phantom events on connect (e.g. trigger
            //     axes settling at -1.0, slight stick drift) do not bind.
            //   - If a gamepad is already bound: process the event only if
            //     it came from the bound gamepad. Events from any other
            //     GamepadId are dropped — including a second physical
            //     controller of the same model, since once we've bound a
            //     primary we never swap to a different one for the lifetime
            //     of the binding. The bound gamepad disconnecting is the
            //     only thing that clears the binding.
            //
            // KNOWN LIMITATION: with two physically identical controllers
            // both actively used, whichever fires the first intentional
            // input wins. This is the same pattern as a single controller
            // seen via two backends (the Xbox-on-Windows case), and gilrs
            // exposes no per-device identity to tell them apart. `log_
            // enumeration` warns when the situation is detected. A
            // deterministic fix requires a UI for the user to pick the
            // primary controller explicitly.
            match self.active_gamepad {
                None => {
                    if !is_intentional_input(&event) {
                        continue;
                    }
                    if !self.gilrs.gamepad(id).is_connected() {
                        continue;
                    }
                    let name = self.gilrs.gamepad(id).name().to_string();
                    log::info!(
                        "Bound to gamepad on first intentional input: {} (id: {:?})",
                        name,
                        id
                    );
                    self.active_gamepad = Some(id);
                    events.push(ControllerEvent::Connected(name));
                }
                Some(active_id) if active_id == id => {}
                Some(_) => continue,
            }

            match event {
                EventType::ButtonPressed(button, _) => {
                    self.button_states.insert(button, true);
                    events.push(ControllerEvent::ButtonPressed(button_to_string(button)));
                }
                EventType::ButtonReleased(button, _) => {
                    self.button_states.insert(button, false);
                    events.push(ControllerEvent::ButtonReleased(button_to_string(button)));
                }
                EventType::AxisChanged(axis, value, _) => {
                    self.axis_states.insert(axis, value);
                    events.push(ControllerEvent::AxisChanged(axis_to_string(axis), value));
                }
                _ => {}
            }
        }

        events
    }

    /// Heuristic: do two `GamepadId`s likely point at the same physical
    /// controller? Used to allow swapping the speculative active gamepad onto
    /// the backend that actually emits input (e.g. XInput vs rawinput on
    /// Windows for the same Xbox controller) while still rejecting events
    /// from a genuinely separate controller.
    fn is_same_physical(&self, a: GamepadId, b: GamepadId) -> bool {
        if a == b {
            return true;
        }
        let ga = self.gilrs.gamepad(a);
        let gb = self.gilrs.gamepad(b);
        // Strong signal: matching UUIDs (gilrs derives these from device IDs).
        if ga.uuid() == gb.uuid() && ga.uuid() != [0u8; 16] {
            return true;
        }
        // Strong signal: matching VID+PID. If both backends report them, they
        // must agree.
        if let (Some(va), Some(vb), Some(pa), Some(pb)) = (
            ga.vendor_id(),
            gb.vendor_id(),
            ga.product_id(),
            gb.product_id(),
        ) {
            return va == vb && pa == pb;
        }
        // Fallback for backends that hide VID/PID (notably Windows XInput,
        // which reports synthesized values). Compare names with substring
        // matching so e.g. "Xbox Controller" and "XInput Controller (Xbox)"
        // are treated as the same device.
        let na = ga.name().to_lowercase();
        let nb = gb.name().to_lowercase();
        !na.is_empty() && !nb.is_empty() && (na.contains(&nb) || nb.contains(&na))
    }

    pub fn get_axis(&self, axis: Axis) -> f32 {
        *self.axis_states.get(&axis).unwrap_or(&0.0)
    }

    pub fn get_axis_by_name(&self, name: &str) -> f32 {
        if let Some(axis) = string_to_axis(name) {
            self.get_axis(axis)
        } else {
            0.0
        }
    }
}

/// Threshold above which a stick deflection is considered an intentional
/// input rather than drift or noise. Deliberately well above any realistic
/// deadzone so an idle controller cannot accidentally bind itself during the
/// speculative window.
const INTENTIONAL_STICK_THRESHOLD: f32 = 0.5;

fn is_intentional_input(event: &EventType) -> bool {
    match event {
        // A button press is always deliberate. This is also the path that
        // Xbox triggers fire through (Button::LeftTrigger2 / RightTrigger2),
        // so they remain a valid way to bind even though we exclude trigger
        // *axis* events below.
        EventType::ButtonPressed(_, _) => true,
        // Only sticks count as intentional via deflection. Triggers
        // (LeftZ/RightZ) settle at backend-dependent values (often -1.0,
        // i.e. abs=1.0) on connect, which would otherwise look just like a
        // fully-pulled trigger. DPadX/DPadY likewise can sit at non-zero
        // idle values on some backends.
        EventType::AxisChanged(
            Axis::LeftStickX | Axis::LeftStickY | Axis::RightStickX | Axis::RightStickY,
            value,
            _,
        ) => value.abs() > INTENTIONAL_STICK_THRESHOLD,
        _ => false,
    }
}

fn button_to_string(button: Button) -> String {
    match button {
        Button::South => "south",
        Button::East => "east",
        Button::North => "north",
        Button::West => "west",
        Button::C => "c",
        Button::Z => "z",
        Button::LeftTrigger => "left_bumper",
        Button::LeftTrigger2 => "left_trigger",
        Button::RightTrigger => "right_bumper",
        Button::RightTrigger2 => "right_trigger",
        Button::Select => "select",
        Button::Start => "start",
        Button::Mode => "mode",
        Button::LeftThumb => "left_thumb",
        Button::RightThumb => "right_thumb",
        Button::DPadUp => "dpad_up",
        Button::DPadDown => "dpad_down",
        Button::DPadLeft => "dpad_left",
        Button::DPadRight => "dpad_right",
        Button::Unknown => "unknown",
    }
    .to_string()
}

fn axis_to_string(axis: Axis) -> String {
    match axis {
        Axis::LeftStickX => "left_stick_x",
        Axis::LeftStickY => "left_stick_y",
        Axis::RightStickX => "right_stick_x",
        Axis::RightStickY => "right_stick_y",
        Axis::LeftZ => "left_z",
        Axis::RightZ => "right_z",
        Axis::DPadX => "dpad_x",
        Axis::DPadY => "dpad_y",
        Axis::Unknown => "unknown",
    }
    .to_string()
}

fn string_to_axis(name: &str) -> Option<Axis> {
    match name {
        "left_stick_x" => Some(Axis::LeftStickX),
        "left_stick_y" => Some(Axis::LeftStickY),
        "right_stick_x" => Some(Axis::RightStickX),
        "right_stick_y" => Some(Axis::RightStickY),
        "left_z" => Some(Axis::LeftZ),
        "right_z" => Some(Axis::RightZ),
        "dpad_x" => Some(Axis::DPadX),
        "dpad_y" => Some(Axis::DPadY),
        _ => None,
    }
}
