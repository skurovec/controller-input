use enigo::{
    Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings,
    Button as MouseButton,
};

pub struct InputSimulator {
    enigo: Enigo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    KeyPress(String),
    KeyRelease(String),
    MouseMove(i32, i32),
    MouseButtonPress(MouseButtonType),
    MouseButtonRelease(MouseButtonType),
    MouseScroll(i32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButtonType {
    Left,
    Right,
    Middle,
}

impl InputSimulator {
    pub fn new() -> Result<Self, String> {
        let result = std::panic::catch_unwind(|| {
            let settings = Settings::default();
            Enigo::new(&settings)
        });

        match result {
            Ok(Ok(enigo)) => Ok(InputSimulator { enigo }),
            Ok(Err(e)) => Err(format!("Failed to create enigo: {}", e)),
            Err(_) => Err("Enigo initialization panicked (accessibility permissions needed?)".to_string()),
        }
    }

    pub fn execute(&mut self, action: InputAction) {
        log::debug!("Executing action: {:?}", action);
        match &action {
            InputAction::KeyPress(key) => self.key_press(key),
            InputAction::KeyRelease(key) => self.key_release(key),
            InputAction::MouseMove(dx, dy) => self.mouse_move(*dx, *dy),
            InputAction::MouseButtonPress(button) => self.mouse_button_press(*button),
            InputAction::MouseButtonRelease(button) => self.mouse_button_release(*button),
            InputAction::MouseScroll(amount) => self.mouse_scroll(*amount),
        }
    }

    fn key_press(&mut self, key_name: &str) {
        if let Some(key) = string_to_key(key_name) {
            if let Err(e) = self.enigo.key(key, Direction::Press) {
                log::warn!("Failed to press key {}: {}", key_name, e);
            }
        }
    }

    fn key_release(&mut self, key_name: &str) {
        if let Some(key) = string_to_key(key_name) {
            if let Err(e) = self.enigo.key(key, Direction::Release) {
                log::warn!("Failed to release key {}: {}", key_name, e);
            }
        }
    }

    fn mouse_move(&mut self, dx: i32, dy: i32) {
        if dx != 0 || dy != 0 {
            if let Err(e) = self.enigo.move_mouse(dx, dy, Coordinate::Rel) {
                log::warn!("Failed to move mouse: {}", e);
            }
        }
    }

    fn mouse_button_press(&mut self, button: MouseButtonType) {
        let mb = match button {
            MouseButtonType::Left => MouseButton::Left,
            MouseButtonType::Right => MouseButton::Right,
            MouseButtonType::Middle => MouseButton::Middle,
        };
        if let Err(e) = self.enigo.button(mb, Direction::Press) {
            log::warn!("Failed to press mouse button: {}", e);
        }
    }

    fn mouse_button_release(&mut self, button: MouseButtonType) {
        let mb = match button {
            MouseButtonType::Left => MouseButton::Left,
            MouseButtonType::Right => MouseButton::Right,
            MouseButtonType::Middle => MouseButton::Middle,
        };
        if let Err(e) = self.enigo.button(mb, Direction::Release) {
            log::warn!("Failed to release mouse button: {}", e);
        }
    }

    fn mouse_scroll(&mut self, amount: i32) {
        use enigo::Axis;
        if let Err(e) = self.enigo.scroll(amount, Axis::Vertical) {
            log::warn!("Failed to scroll: {}", e);
        }
    }
}

fn string_to_key(name: &str) -> Option<Key> {
    // Use raw virtual keycodes for macOS to avoid TSM main thread requirement
    // These are macOS virtual key codes (kVK_* constants)
    #[cfg(target_os = "macos")]
    {
        match name.to_lowercase().as_str() {
            // Letters - macOS virtual key codes
            "a" => Some(Key::Other(0x00)),
            "b" => Some(Key::Other(0x0B)),
            "c" => Some(Key::Other(0x08)),
            "d" => Some(Key::Other(0x02)),
            "e" => Some(Key::Other(0x0E)),
            "f" => Some(Key::Other(0x03)),
            "g" => Some(Key::Other(0x05)),
            "h" => Some(Key::Other(0x04)),
            "i" => Some(Key::Other(0x22)),
            "j" => Some(Key::Other(0x26)),
            "k" => Some(Key::Other(0x28)),
            "l" => Some(Key::Other(0x25)),
            "m" => Some(Key::Other(0x2E)),
            "n" => Some(Key::Other(0x2D)),
            "o" => Some(Key::Other(0x1F)),
            "p" => Some(Key::Other(0x23)),
            "q" => Some(Key::Other(0x0C)),
            "r" => Some(Key::Other(0x0F)),
            "s" => Some(Key::Other(0x01)),
            "t" => Some(Key::Other(0x11)),
            "u" => Some(Key::Other(0x20)),
            "v" => Some(Key::Other(0x09)),
            "w" => Some(Key::Other(0x0D)),
            "x" => Some(Key::Other(0x07)),
            "y" => Some(Key::Other(0x10)),
            "z" => Some(Key::Other(0x06)),
            // Numbers
            "0" => Some(Key::Other(0x1D)),
            "1" => Some(Key::Other(0x12)),
            "2" => Some(Key::Other(0x13)),
            "3" => Some(Key::Other(0x14)),
            "4" => Some(Key::Other(0x15)),
            "5" => Some(Key::Other(0x17)),
            "6" => Some(Key::Other(0x16)),
            "7" => Some(Key::Other(0x1A)),
            "8" => Some(Key::Other(0x1C)),
            "9" => Some(Key::Other(0x19)),
            // Function keys
            "f1" => Some(Key::F1),
            "f2" => Some(Key::F2),
            "f3" => Some(Key::F3),
            "f4" => Some(Key::F4),
            "f5" => Some(Key::F5),
            "f6" => Some(Key::F6),
            "f7" => Some(Key::F7),
            "f8" => Some(Key::F8),
            "f9" => Some(Key::F9),
            "f10" => Some(Key::F10),
            "f11" => Some(Key::F11),
            "f12" => Some(Key::F12),
            // Modifiers
            "shift" => Some(Key::Shift),
            "control" | "ctrl" => Some(Key::Control),
            "alt" => Some(Key::Alt),
            "meta" | "super" | "command" | "cmd" => Some(Key::Meta),
            // Special keys
            "space" => Some(Key::Space),
            "escape" | "esc" => Some(Key::Escape),
            "tab" => Some(Key::Tab),
            "enter" | "return" => Some(Key::Return),
            "backspace" => Some(Key::Backspace),
            "delete" | "del" => Some(Key::Delete),
            "home" => Some(Key::Home),
            "end" => Some(Key::End),
            "pageup" => Some(Key::PageUp),
            "pagedown" => Some(Key::PageDown),
            // Arrow keys
            "up" | "uparrow" => Some(Key::UpArrow),
            "down" | "downarrow" => Some(Key::DownArrow),
            "left" | "leftarrow" => Some(Key::LeftArrow),
            "right" | "rightarrow" => Some(Key::RightArrow),
            // Mouse buttons as special strings (not actual keys)
            "leftclick" | "left_click" => None,
            "rightclick" | "right_click" => None,
            "middleclick" | "middle_click" => None,
            _ => {
                log::warn!("Unknown key: {}", name);
                None
            }
        }
    }

    // Windows: use virtual-key codes via Key::Other so SendInput emits real VK
    // events (not KEYEVENTF_UNICODE, which games like Minecraft ignore).
    #[cfg(target_os = "windows")]
    {
        match name.to_lowercase().as_str() {
            // Letters (VK A-Z = 0x41..0x5A)
            "a" => Some(Key::Other(0x41)),
            "b" => Some(Key::Other(0x42)),
            "c" => Some(Key::Other(0x43)),
            "d" => Some(Key::Other(0x44)),
            "e" => Some(Key::Other(0x45)),
            "f" => Some(Key::Other(0x46)),
            "g" => Some(Key::Other(0x47)),
            "h" => Some(Key::Other(0x48)),
            "i" => Some(Key::Other(0x49)),
            "j" => Some(Key::Other(0x4A)),
            "k" => Some(Key::Other(0x4B)),
            "l" => Some(Key::Other(0x4C)),
            "m" => Some(Key::Other(0x4D)),
            "n" => Some(Key::Other(0x4E)),
            "o" => Some(Key::Other(0x4F)),
            "p" => Some(Key::Other(0x50)),
            "q" => Some(Key::Other(0x51)),
            "r" => Some(Key::Other(0x52)),
            "s" => Some(Key::Other(0x53)),
            "t" => Some(Key::Other(0x54)),
            "u" => Some(Key::Other(0x55)),
            "v" => Some(Key::Other(0x56)),
            "w" => Some(Key::Other(0x57)),
            "x" => Some(Key::Other(0x58)),
            "y" => Some(Key::Other(0x59)),
            "z" => Some(Key::Other(0x5A)),
            // Numbers (VK 0-9 = 0x30..0x39, top row)
            "0" => Some(Key::Other(0x30)),
            "1" => Some(Key::Other(0x31)),
            "2" => Some(Key::Other(0x32)),
            "3" => Some(Key::Other(0x33)),
            "4" => Some(Key::Other(0x34)),
            "5" => Some(Key::Other(0x35)),
            "6" => Some(Key::Other(0x36)),
            "7" => Some(Key::Other(0x37)),
            "8" => Some(Key::Other(0x38)),
            "9" => Some(Key::Other(0x39)),
            // Function keys
            "f1" => Some(Key::F1),
            "f2" => Some(Key::F2),
            "f3" => Some(Key::F3),
            "f4" => Some(Key::F4),
            "f5" => Some(Key::F5),
            "f6" => Some(Key::F6),
            "f7" => Some(Key::F7),
            "f8" => Some(Key::F8),
            "f9" => Some(Key::F9),
            "f10" => Some(Key::F10),
            "f11" => Some(Key::F11),
            "f12" => Some(Key::F12),
            // Modifiers
            "shift" => Some(Key::Shift),
            "control" | "ctrl" => Some(Key::Control),
            "alt" => Some(Key::Alt),
            "meta" | "super" | "command" | "cmd" => Some(Key::Meta),
            // Special keys
            "space" => Some(Key::Space),
            "escape" | "esc" => Some(Key::Escape),
            "tab" => Some(Key::Tab),
            "enter" | "return" => Some(Key::Return),
            "backspace" => Some(Key::Backspace),
            "delete" | "del" => Some(Key::Delete),
            "home" => Some(Key::Home),
            "end" => Some(Key::End),
            "pageup" => Some(Key::PageUp),
            "pagedown" => Some(Key::PageDown),
            // Arrow keys
            "up" | "uparrow" => Some(Key::UpArrow),
            "down" | "downarrow" => Some(Key::DownArrow),
            "left" | "leftarrow" => Some(Key::LeftArrow),
            "right" | "rightarrow" => Some(Key::RightArrow),
            // Mouse buttons as special strings (not actual keys)
            "leftclick" | "left_click" => None,
            "rightclick" | "right_click" => None,
            "middleclick" | "middle_click" => None,
            _ => {
                log::warn!("Unknown key: {}", name);
                None
            }
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        match name.to_lowercase().as_str() {
            // Letters
            "a" => Some(Key::Unicode('a')),
            "b" => Some(Key::Unicode('b')),
            "c" => Some(Key::Unicode('c')),
            "d" => Some(Key::Unicode('d')),
            "e" => Some(Key::Unicode('e')),
            "f" => Some(Key::Unicode('f')),
            "g" => Some(Key::Unicode('g')),
            "h" => Some(Key::Unicode('h')),
            "i" => Some(Key::Unicode('i')),
            "j" => Some(Key::Unicode('j')),
            "k" => Some(Key::Unicode('k')),
            "l" => Some(Key::Unicode('l')),
            "m" => Some(Key::Unicode('m')),
            "n" => Some(Key::Unicode('n')),
            "o" => Some(Key::Unicode('o')),
            "p" => Some(Key::Unicode('p')),
            "q" => Some(Key::Unicode('q')),
            "r" => Some(Key::Unicode('r')),
            "s" => Some(Key::Unicode('s')),
            "t" => Some(Key::Unicode('t')),
            "u" => Some(Key::Unicode('u')),
            "v" => Some(Key::Unicode('v')),
            "w" => Some(Key::Unicode('w')),
            "x" => Some(Key::Unicode('x')),
            "y" => Some(Key::Unicode('y')),
            "z" => Some(Key::Unicode('z')),
            // Numbers
            "0" => Some(Key::Unicode('0')),
            "1" => Some(Key::Unicode('1')),
            "2" => Some(Key::Unicode('2')),
            "3" => Some(Key::Unicode('3')),
            "4" => Some(Key::Unicode('4')),
            "5" => Some(Key::Unicode('5')),
            "6" => Some(Key::Unicode('6')),
            "7" => Some(Key::Unicode('7')),
            "8" => Some(Key::Unicode('8')),
            "9" => Some(Key::Unicode('9')),
            // Function keys
            "f1" => Some(Key::F1),
            "f2" => Some(Key::F2),
            "f3" => Some(Key::F3),
            "f4" => Some(Key::F4),
            "f5" => Some(Key::F5),
            "f6" => Some(Key::F6),
            "f7" => Some(Key::F7),
            "f8" => Some(Key::F8),
            "f9" => Some(Key::F9),
            "f10" => Some(Key::F10),
            "f11" => Some(Key::F11),
            "f12" => Some(Key::F12),
            // Modifiers
            "shift" => Some(Key::Shift),
            "control" | "ctrl" => Some(Key::Control),
            "alt" => Some(Key::Alt),
            "meta" | "super" | "command" | "cmd" => Some(Key::Meta),
            // Special keys
            "space" => Some(Key::Space),
            "escape" | "esc" => Some(Key::Escape),
            "tab" => Some(Key::Tab),
            "enter" | "return" => Some(Key::Return),
            "backspace" => Some(Key::Backspace),
            "delete" | "del" => Some(Key::Delete),
            "home" => Some(Key::Home),
            "end" => Some(Key::End),
            "pageup" => Some(Key::PageUp),
            "pagedown" => Some(Key::PageDown),
            // Arrow keys
            "up" | "uparrow" => Some(Key::UpArrow),
            "down" | "downarrow" => Some(Key::DownArrow),
            "left" | "leftarrow" => Some(Key::LeftArrow),
            "right" | "rightarrow" => Some(Key::RightArrow),
            // Mouse buttons as special strings (not actual keys)
            "leftclick" | "left_click" => None,
            "rightclick" | "right_click" => None,
            "middleclick" | "middle_click" => None,
            _ => {
                log::warn!("Unknown key: {}", name);
                None
            }
        }
    }
}

pub fn is_mouse_action(name: &str) -> Option<MouseButtonType> {
    match name.to_lowercase().as_str() {
        "leftclick" | "left_click" => Some(MouseButtonType::Left),
        "rightclick" | "right_click" => Some(MouseButtonType::Right),
        "middleclick" | "middle_click" => Some(MouseButtonType::Middle),
        _ => None,
    }
}

pub fn is_scroll_action(name: &str) -> Option<i32> {
    match name.to_lowercase().as_str() {
        "scrollup" | "scroll_up" => Some(1),
        "scrolldown" | "scroll_down" => Some(-1),
        _ => None,
    }
}
