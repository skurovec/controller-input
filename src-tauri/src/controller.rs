use gilrs::{Axis, Button, Event, EventType, Gilrs, GamepadId};
use std::collections::HashMap;

pub struct Controller {
    gilrs: Gilrs,
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
        let result = std::panic::catch_unwind(|| {
            Gilrs::new()
        });

        let gilrs = match result {
            Ok(Ok(g)) => g,
            Ok(Err(e)) => return Err(format!("Failed to initialize gilrs: {}", e)),
            Err(_) => return Err("Gilrs initialization panicked".to_string()),
        };

        let mut controller = Controller {
            gilrs,
            active_gamepad: None,
            axis_states: HashMap::new(),
            button_states: HashMap::new(),
        };

        // Find first connected gamepad
        controller.find_gamepad();

        Ok(controller)
    }

    fn find_gamepad(&mut self) {
        for (id, gamepad) in self.gilrs.gamepads() {
            if gamepad.is_connected() {
                self.active_gamepad = Some(id);
                log::info!("Found gamepad: {} (id: {:?})", gamepad.name(), id);
                return;
            }
        }
        self.active_gamepad = None;
    }

    pub fn is_connected(&self) -> bool {
        self.active_gamepad.is_some()
    }

    pub fn gamepad_name(&self) -> Option<String> {
        self.active_gamepad.map(|id| {
            self.gilrs.gamepad(id).name().to_string()
        })
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

            // Update active gamepad if needed
            match event {
                EventType::Connected => {
                    if self.active_gamepad.is_none() {
                        self.active_gamepad = Some(id);
                        let name = self.gilrs.gamepad(id).name().to_string();
                        log::info!("Gamepad connected: {}", name);
                        events.push(ControllerEvent::Connected(name));
                    }
                }
                EventType::Disconnected => {
                    if self.active_gamepad == Some(id) {
                        self.active_gamepad = None;
                        log::info!("Gamepad disconnected");
                        events.push(ControllerEvent::Disconnected);
                        // Try to find another gamepad
                        self.find_gamepad();
                    }
                }
                _ => {}
            }

            // Only process events from active gamepad
            if self.active_gamepad != Some(id) {
                continue;
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
    }.to_string()
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
    }.to_string()
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
