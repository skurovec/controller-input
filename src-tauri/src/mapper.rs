use crate::config::{AxisMapping, Config};
use crate::controller::ControllerEvent;
use crate::input_sim::{is_mouse_action, is_scroll_action, InputAction};
use std::collections::HashMap;

pub struct Mapper {
    config: Config,
    // Track which keys are currently pressed due to axis thresholds
    axis_key_states: HashMap<String, bool>,
    // Track accumulated mouse movement for smooth sub-pixel handling
    mouse_accumulator_x: f32,
    mouse_accumulator_y: f32,
}

impl Mapper {
    pub fn new(config: Config) -> Self {
        Mapper {
            config,
            axis_key_states: HashMap::new(),
            mouse_accumulator_x: 0.0,
            mouse_accumulator_y: 0.0,
        }
    }

    pub fn update_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn map_event(&mut self, event: &ControllerEvent) -> Vec<InputAction> {
        let mut actions = Vec::new();

        match event {
            ControllerEvent::ButtonPressed(button) => {
                if let Some(action_str) = self.config.buttons.get(button) {
                    if let Some(mouse_button) = is_mouse_action(action_str) {
                        actions.push(InputAction::MouseButtonPress(mouse_button));
                    } else if let Some(scroll_amount) = is_scroll_action(action_str) {
                        // Scroll actions trigger once on press
                        actions.push(InputAction::MouseScroll(scroll_amount));
                    } else {
                        actions.push(InputAction::KeyPress(action_str.clone()));
                    }
                }
            }
            ControllerEvent::ButtonReleased(button) => {
                if let Some(action_str) = self.config.buttons.get(button) {
                    if let Some(mouse_button) = is_mouse_action(action_str) {
                        actions.push(InputAction::MouseButtonRelease(mouse_button));
                    } else if is_scroll_action(action_str).is_some() {
                        // Scroll actions don't have a release
                    } else {
                        actions.push(InputAction::KeyRelease(action_str.clone()));
                    }
                }
            }
            ControllerEvent::AxisChanged(axis, value) => {
                if let Some(mapping) = self.config.axes.get(axis) {
                    match mapping {
                        AxisMapping::Keys { negative, positive } => {
                            let threshold = self.config.settings.stick_to_key_threshold;
                            let deadzone = self.config.settings.deadzone;
                            let value = apply_deadzone(*value, deadzone);

                            // Handle negative direction
                            let neg_key = format!("{}_{}", axis, "neg");
                            let neg_pressed = value < -threshold;
                            let neg_was_pressed = *self.axis_key_states.get(&neg_key).unwrap_or(&false);

                            if neg_pressed && !neg_was_pressed {
                                actions.push(InputAction::KeyPress(negative.clone()));
                                self.axis_key_states.insert(neg_key, true);
                            } else if !neg_pressed && neg_was_pressed {
                                actions.push(InputAction::KeyRelease(negative.clone()));
                                self.axis_key_states.insert(neg_key, false);
                            }

                            // Handle positive direction
                            let pos_key = format!("{}_{}", axis, "pos");
                            let pos_pressed = value > threshold;
                            let pos_was_pressed = *self.axis_key_states.get(&pos_key).unwrap_or(&false);

                            if pos_pressed && !pos_was_pressed {
                                actions.push(InputAction::KeyPress(positive.clone()));
                                self.axis_key_states.insert(pos_key, true);
                            } else if !pos_pressed && pos_was_pressed {
                                actions.push(InputAction::KeyRelease(positive.clone()));
                                self.axis_key_states.insert(pos_key, false);
                            }
                        }
                        AxisMapping::MouseX { .. } | AxisMapping::MouseY { .. } => {
                            // Mouse movement is handled in update_mouse_movement
                        }
                    }
                }
            }
            _ => {}
        }

        actions
    }

    pub fn update_mouse_movement(&mut self, axis_values: &HashMap<String, f32>) -> Option<InputAction> {
        let deadzone = self.config.settings.deadzone;
        let base_sensitivity = self.config.settings.mouse_sensitivity;

        let mut dx = 0.0f32;
        let mut dy = 0.0f32;

        for (axis, mapping) in &self.config.axes {
            match mapping {
                AxisMapping::MouseX { sensitivity } => {
                    if let Some(&value) = axis_values.get(axis) {
                        let value = apply_deadzone(value, deadzone);
                        dx += value * sensitivity * base_sensitivity;
                    }
                }
                AxisMapping::MouseY { sensitivity } => {
                    if let Some(&value) = axis_values.get(axis) {
                        let value = apply_deadzone(value, deadzone);
                        dy += value * sensitivity * base_sensitivity;
                    }
                }
                _ => {}
            }
        }

        // Accumulate sub-pixel movements
        self.mouse_accumulator_x += dx;
        self.mouse_accumulator_y += dy;

        // Extract integer part for actual movement
        let move_x = self.mouse_accumulator_x as i32;
        let move_y = self.mouse_accumulator_y as i32;

        // Keep the fractional part for next frame
        self.mouse_accumulator_x -= move_x as f32;
        self.mouse_accumulator_y -= move_y as f32;

        if move_x != 0 || move_y != 0 {
            // Invert Y axis for natural camera control
            Some(InputAction::MouseMove(move_x, -move_y))
        } else {
            None
        }
    }

    pub fn release_all_keys(&mut self) -> Vec<InputAction> {
        let mut actions = Vec::new();

        // Release all axis-triggered keys
        for (axis, mapping) in &self.config.axes {
            if let AxisMapping::Keys { negative, positive } = mapping {
                let neg_key = format!("{}_{}", axis, "neg");
                if *self.axis_key_states.get(&neg_key).unwrap_or(&false) {
                    actions.push(InputAction::KeyRelease(negative.clone()));
                }

                let pos_key = format!("{}_{}", axis, "pos");
                if *self.axis_key_states.get(&pos_key).unwrap_or(&false) {
                    actions.push(InputAction::KeyRelease(positive.clone()));
                }
            }
        }

        self.axis_key_states.clear();
        self.mouse_accumulator_x = 0.0;
        self.mouse_accumulator_y = 0.0;

        actions
    }
}

fn apply_deadzone(value: f32, deadzone: f32) -> f32 {
    if value.abs() < deadzone {
        0.0
    } else {
        // Rescale the value so it goes from 0 at the deadzone edge to 1 at full deflection
        let sign = value.signum();
        let magnitude = (value.abs() - deadzone) / (1.0 - deadzone);
        sign * magnitude
    }
}
