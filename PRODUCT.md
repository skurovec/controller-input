# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

The UI is a static HTML/CSS/JS frontend (`src/`) rendered by a Tauri 2 desktop
webview on macOS and Windows. Window defaults to 760 × 760, minimum 460 × 560,
resizable.

## Users

A child playing Minecraft Java Edition with a gamepad, operating the app on
their own: they open it, check the controller is recognised, press start and
switch to the game. Reading ability and patience are limited; pictures, colour
and short words carry the meaning. A parent installs the app and may change the
mapping or tuning, but is not the primary audience of the window.

## Product Purpose

Lets a gamepad drive a game that only understands keyboard and mouse, without
mods, Steam Input or accounts. Success: the child gets from "app open" to
"playing" without help, and can tell at a glance whether the controller is
connected and whether mapping is on.

## Positioning

A small standalone app that leaves the game untouched. Its default mapping is
tuned for Minecraft Java Edition, but the product name and UI must not use
Minecraft branding (trademark constraint; the app was renamed for this reason).

## Operating Context

- Started before or alongside the game; the window usually sits behind the
  game while mapping is active.
- Works with Xbox-style controllers, 8BitDo and DualSense; button names differ
  per family (A/B/X/Y vs Cross/Circle/Square/Triangle).
- Closing the window quits the app and releases held keys.

## Capabilities and Constraints

- Status: controller connected (with device name) / not connected; mapping
  active / inactive. Start requires a connected controller.
- Sixteen remappable buttons, each to a keyboard key, mouse button, scroll
  direction or nothing. Left stick is fixed to WASD, right stick to mouse look.
- Tuning: deadzone (0–0.5), mouse sensitivity (0.5–10), stick threshold
  (0.1–0.9). Reset to defaults.
- Configuration persists to `config.toml`; unknown values from the file must
  still be shown.
- UI shows the game action for a button (e.g. "Jump") alongside its key when the
  key matches the default mapping; otherwise only the key.
- UI language follows the system locale: Czech for `cs`, English otherwise.

## Brand Commitments

- Name: "Controller Input".
- No Minecraft, Mojang, Xbox or PlayStation logos or trade dress.

## Evidence on Hand

- README origin story: built so the author's son could play Minecraft with a
  controller.
- Screenshots in `docs/screenshots/` are captured from the real running app.

## Product Principles

1. The child can succeed without reading a paragraph.
2. State is unmistakable: connected or not, playing or not.
3. Tuning exists but never stands between the child and the Start button.
4. The game stays untouched; the app is a quiet companion.

## Accessibility & Inclusion

Young users: large hit targets, high-contrast state, meaning never carried by
colour alone, keyboard-operable controls.
