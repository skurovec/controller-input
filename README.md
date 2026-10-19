# Controller Input

A Tauri desktop application that maps controller/gamepad inputs to keyboard and mouse. The default mappings target Minecraft Java Edition.

## Features

- Maps controller buttons to keyboard keys
- Maps analog sticks to WASD movement and mouse look
- Configurable sensitivity and deadzone settings
- Cross-platform (Windows, macOS)
- Closing the window quits the app and releases any held keys and mouse buttons

## Default Controller Mapping

| Controller Input | Action |
|-----------------|--------|
| Left Stick | WASD (movement) |
| Right Stick | Mouse movement (camera) |
| A / Cross | 5 (hotbar) |
| B / Circle | Space (jump) |
| X / Square | E (inventory) |
| Y / Triangle | F (swap hands) |
| LB / L1 | Scroll down (next hotbar slot) |
| RB / R1 | Scroll up (previous hotbar slot) |
| LT / L2 | Left click (attack/mine) |
| RT / R2 | Right click (use/place) |
| D-Pad Up | 1 (hotbar) |
| D-Pad Down | Q (drop item) |
| D-Pad Left | 2 (hotbar) |
| D-Pad Right | 3 (hotbar) |
| Start | Escape (pause) |
| Select/Back | Tab (player list) |
| L3 (Left Stick Click) | Ctrl (sprint) |
| R3 (Right Stick Click) | Shift (sneak) |

## Installation

Download the latest build from the
[releases page](https://github.com/skurovec/controller-input/releases):

- **macOS**: `.dmg` (universal, Apple Silicon and Intel)
- **Windows**: `.msi` or `-setup.exe` (XInput backend, see below)

Release builds are not code-signed. On first launch:

- **macOS**: open the app, close the warning, then go to System Settings ->
  Privacy & Security and click **Open Anyway**.
- **Windows**: if SmartScreen blocks the installer, click **More info** ->
  **Run anyway**.

## Development

### Prerequisites

- Rust (latest stable)
- Node.js 18+ and Yarn
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Setup

```bash
# Install JS dependencies
yarn install

# Run in development mode
yarn dev

# Build for production
yarn build
```

### Windows controller backends

The default build uses XInput for background input from Xbox-compatible controllers.
Native DualSense controllers require the alternative Windows Gaming Input backend:

```bash
cd src-tauri
cargo build --release --no-default-features --features windows-wgi,tauri/custom-protocol
```

Do not enable `windows-xinput` and `windows-wgi` together. Gilrs requires exactly
one Windows backend. The WGI variant includes native PS5 controller mappings, but
background input depends on the device and driver. Verify buttons and sticks both
with the mapper focused and with Minecraft focused before relying on this variant.
The default XInput build remains available if WGI background input fails.

For Windows GNU cross-compilation, also pass `--target x86_64-pc-windows-gnu`
and configure a compatible linker. Distribute `WebView2Loader.dll` alongside the
executable. Both variants require the WebView2 Runtime and use the same saved
configuration. Close one variant completely before starting the other.

### Releasing

Bump the version in `package.json`, `src-tauri/Cargo.toml` and
`src-tauri/tauri.conf.json`, then push a `v<version>` tag. The release
workflow builds the macOS and Windows bundles and attaches them to a draft
GitHub release for review.

### macOS Permissions

On macOS, you need to grant Accessibility permissions for the app to simulate keyboard/mouse input:

1. Open System Settings -> Privacy & Security -> Accessibility
2. Add the Controller Input app to the list

Builds made before the app identifier changed to `cz.skurovec.controller-input`
are treated as a different app. After upgrading from such a build, remove the
old entry and grant Accessibility permission to Controller Input again.

## Configuration

Configuration is stored in:
- **macOS**: `~/Library/Application Support/controller-input/config.toml`
- **Windows**: `%APPDATA%/controller-input/config.toml`

If the new configuration file does not exist, the app reads the previous
`minecraft-input/config.toml` from the same parent directory. Saving settings
writes to `controller-input/config.toml` and leaves the old file unchanged.

### Example Configuration

```toml
[settings]
deadzone = 0.15
mouse_sensitivity = 2.5
stick_to_key_threshold = 0.5

[buttons]
south = "5"
east = "Space"
west = "E"
north = "F"
left_trigger = "LeftClick"
right_trigger = "RightClick"
left_bumper = "ScrollDown"
right_bumper = "ScrollUp"
start = "Escape"
select = "Tab"
left_thumb = "Control"
right_thumb = "Shift"
dpad_up = "1"
dpad_down = "Q"
dpad_left = "2"
dpad_right = "3"

[axes]
left_stick_x = { type = "keys", negative = "A", positive = "D" }
left_stick_y = { type = "keys", negative = "S", positive = "W" }
right_stick_x = { type = "mouse_x", sensitivity = 2.5 }
right_stick_y = { type = "mouse_y", sensitivity = 2.5 }
```

## License

[MIT](LICENSE)

## Trademarks

NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG
OR MICROSOFT. Minecraft, Xbox, PlayStation and DualSense are trademarks of
their respective owners and are used only to describe compatibility.
