---
name: Controller Input
description: A wordless assembly booklet that walks a child from "controller on" to "playing".
colors:
  page: "#cfe6f5"
  paper: "#ffffff"
  ink: "#1b1b1f"
  ink-soft: "#414852"
  ink-faint: "#7b8794"
  plastic: "#e4ebf2"
  well: "#ccd6e0"
  play: "#2fb457"
  buttons: "#f7c625"
  tune: "#ff8a3d"
  stop: "#e8483b"
  error: "#b3261e"
typography:
  display:
    fontFamily: "Atkinson Hyperlegible Next, -apple-system, Segoe UI, sans-serif"
    fontSize: "7.5rem"
    fontWeight: 800
    lineHeight: 1
    letterSpacing: "-0.04em"
    fontFeature: "tnum"
  headline:
    fontFamily: "Atkinson Hyperlegible Next, -apple-system, Segoe UI, sans-serif"
    fontSize: "1.85rem"
    fontWeight: 800
    lineHeight: 1.12
    letterSpacing: "-0.015em"
  title:
    fontFamily: "Atkinson Hyperlegible Next, -apple-system, Segoe UI, sans-serif"
    fontSize: "1.6rem"
    fontWeight: 800
    lineHeight: 1.15
    letterSpacing: "-0.01em"
  title-small:
    fontFamily: "Atkinson Hyperlegible Next, -apple-system, Segoe UI, sans-serif"
    fontSize: "1.1rem"
    fontWeight: 800
    lineHeight: 1.2
  body:
    fontFamily: "Atkinson Hyperlegible Next, -apple-system, Segoe UI, sans-serif"
    fontSize: "16px"
    fontWeight: 500
    lineHeight: 1.4
  label:
    fontFamily: "Atkinson Hyperlegible Next, -apple-system, Segoe UI, sans-serif"
    fontSize: "0.9rem"
    fontWeight: 600
    lineHeight: 1.4
rounded:
  icon-tile: "9px"
  keycap: "13px"
  tab: "16px"
  part: "18px"
  callout: "22px"
  pill: "999px"
spacing:
  sm: "10px"
  md: "14px"
  lg: "20px"
components:
  tab:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
    typography: "{typography.title-small}"
    rounded: "{rounded.tab}"
    height: "54px"
    padding: "0 12px"
  tab-play-selected:
    backgroundColor: "{colors.play}"
  tab-buttons-selected:
    backgroundColor: "{colors.buttons}"
  tab-tune-selected:
    backgroundColor: "{colors.tune}"
  part-go:
    backgroundColor: "{colors.play}"
    textColor: "{colors.ink}"
    rounded: "{rounded.part}"
    width: "100%"
    height: "84px"
  part-stop:
    backgroundColor: "{colors.stop}"
    textColor: "{colors.ink}"
    rounded: "{rounded.part}"
    width: "100%"
    height: "62px"
    padding: "0 30px"
  part-reset:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
    typography: "{typography.title-small}"
    rounded: "{rounded.part}"
    height: "64px"
    padding: "10px 20px"
  part-reset-armed:
    backgroundColor: "{colors.stop}"
  callout:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
    rounded: "{rounded.callout}"
    padding: "16px 20px"
  stage:
    backgroundColor: "{colors.paper}"
    rounded: "{rounded.callout}"
    padding: "22px 26px 26px"
  step-chip:
    textColor: "{colors.ink-soft}"
    rounded: "{rounded.pill}"
    height: "46px"
    padding: "0 12px 0 6px"
  step-chip-current:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink}"
  part-row:
    backgroundColor: "{colors.paper}"
    rounded: "{rounded.part}"
    padding: "9px 12px 9px 10px"
    height: "78px"
  magnifier:
    backgroundColor: "{colors.page}"
    rounded: "{rounded.pill}"
    size: "58px"
  keycap:
    backgroundColor: "{colors.buttons}"
    textColor: "{colors.ink}"
    rounded: "{rounded.keycap}"
    width: "178px"
    height: "56px"
    padding: "0 34px 4px 14px"
  keycap-empty:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ink-soft}"
  keycap-fixed:
    backgroundColor: "{colors.plastic}"
    height: "46px"
    padding: "0 10px 4px"
  slider-thumb:
    backgroundColor: "{colors.tune}"
    rounded: "{rounded.pill}"
    size: "32px"
  notice:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.paper}"
    rounded: "{rounded.tab}"
    padding: "13px 20px"
  notice-error:
    backgroundColor: "{colors.error}"
---

# Design System: Controller Input

## Overview

**Creative North Star: "The Assembly Booklet"**

The window is a page from a toy's assembly booklet: a sky-blue sheet, white callout boxes ruled in black ink, chunky plastic parts you press, and monumental step numerals that tell a child where they are without asking them to read. The reader is a child operating a gamepad alone, so pictures, colour and a few heavy words carry the meaning; a grown-up finds button mapping and tuning behind the other two tabs, drawn in the same booklet language rather than a separate "settings" look.

Density is low and deliberate. One stage callout fills the first viewport with one numeral, one controller drawing, one sentence and one full-width part to press. Everything is outlined in the same ink line, so the page reads like printed line art with colour dropped only into the parts that can be pressed. The system explicitly refuses the dark settings-utility look: no dark chrome, no status pills, no bare native form controls. Sliders and dropdowns exist, but only dressed as toy parts.

**Key Characteristics:**
- Sky-blue page, white paper callouts, near-black ink outlines on everything.
- Saturated hue appears only on pressable parts; each tab owns one hue.
- Pressable parts are moulded plastic with an inset bottom lip that compresses on press.
- One heavy weight (800) for anything that names a thing; soft ink at 500 for help text.
- A single line-art controller drawing is the recurring illustration, reused at every scale.
- Layout never shifts when state changes; panels and slots are pre-sized.

## Colors

A cool, paper-and-ink base with four saturated toy hues reserved for things a hand can press.

### Primary
- **Go Green** (play): the Play tab, the Start part, the controller's home button in the drawing. It means "do this now" and "on".
- **Sticker Yellow** (buttons): the Buttons tab, every remappable keycap, the lit button in the controller diagram and magnifiers, and text selection.
- **Crayon Orange** (tune): the Fine-tune tab and the slider thumb.

### Secondary
- **Stop-Sign Red** (stop): the Stop part and the armed (second-press) state of the reset part. Destructive or halting intent only.

### Tertiary
- **Warning Brick** (error): background of the error notice only, under white text.

### Neutral
- **Sky Page** (page): the booklet page behind everything, the tab rail, magnifier backgrounds and slider tracks.
- **Paper White** (paper): every callout, unselected tab, part row, empty keycap and the controller body.
- **Booklet Ink** (ink): all outlines, all primary text, the step numerals, tick badges and the default notice background.
- **Soft Ink** (ink-soft): help text, secondary labels, upcoming step chips and the waiting slot.
- **Faint Ink** (ink-faint): strokes of the asleep (disconnected) controller drawing only; never body text.
- **Moulded Plastic** (plastic): unpressed controller buttons in the drawing and fixed (non-remappable) keycaps.
- **Stick Well** (well): the recessed ring around each thumbstick in the drawing.

### Named Rules
**The Pressable Hue Rule.** Saturated colour (play, buttons, tune, stop) lands only on parts you can press, or on the drawn controller button a part refers to. Surfaces, text and frames stay page, paper and ink.

**The One Hue Per Section Rule.** Each tab owns one hue, exposed as the current section hue. The selected tab fills with it and the 6px rail under the tabs takes it, so the whole frame says which section you are in.

**The Never Colour Alone Rule.** Every colour state has a shape partner: done steps get an ink tick, upcoming steps are dashed, the current step is raised and solid, Start and Stop carry play and stop glyphs.

## Typography

**Display Font:** Atkinson Hyperlegible Next (self-hosted variable woff2, weights 200 to 800, with -apple-system, Segoe UI, sans-serif)
**Body Font:** Atkinson Hyperlegible Next (same family)

**Character:** A single legibility-first humanist sans, used almost entirely at 800 for names and 500 for explanation. Heavy, friendly and unambiguous at a glance, chosen for a young reader rather than for style.

### Hierarchy
- **Display** (800, 7.5rem, line-height 1, -0.04em, tabular figures): the monumental step numeral pinned top-left in the stage. Drops to 4.2rem under 560px wide.
- **Headline** (800, 1.85rem, 1.12, -0.015em): the stage title, one short sentence per step. 1.5rem under 560px wide, 1.4rem under 600px tall. Balanced wrapping.
- **Title** (800, 1.6rem, 1.15, -0.01em): page titles on the Buttons and Fine-tune pages.
- **Title Small** (800, ~1.1rem, 1.2): tab labels, part-row action names, tuning labels, the reset part.
- **Body** (500, 16px, 1.4): help text in soft ink, capped at 60ch on pages and 40ch in the stage (where it steps up to 1.05rem).
- **Label** (600 to 700, ~0.9rem): controller-button names under row actions, slider end labels and the numeric slider value (tabular figures).

Part labels scale with their part: the Start part sets its label at 2.1rem, Stop at 1.4rem; both 800.

### Named Rules
**The Heavy Name Rule.** Anything that names a thing or an action is 800 in full ink. Anything that explains is 500 to 700 in soft ink. There is no middle tier.

**The Short Words Rule.** Copy is a single sentence or a word or two. If a label needs a paragraph, it belongs on the grown-up tabs, not the stage.

## Layout

The window is a two-row grid: a fixed tab rail on top and a scrolling sheet below. The sheet has 20px gutters, 96px of bottom padding to clear the notice, and a stable scrollbar gutter so content never shifts when scrolling appears. Page content is centred at a 760px maximum width.

Spacing runs on a small, reused rhythm: 10px between sibling items (tabs, step chips, part rows), 14px between an icon and its label or between grid columns, 20px for sheet gutters and callout side padding. The stage is the roomiest container (22px top, 26px sides and bottom).

The Play page is a three-column step strip above one stage callout. The Buttons page is a 220px sticky controller diagram beside a list of part rows. The Fine-tune page is a single column of slider callouts followed by a full-width reset part.

Responsive behaviour is driven by the desktop window (default 760 × 760, minimum 460 × 560):
- Under 680px wide, the Buttons layout collapses to one column and the diagram docks sticky at the top on a page-coloured band that covers rows scrolling beneath it.
- Under 560px wide, tabs stack icon over label (64px tall), the numeral and headline shrink, and each slider moves above its two end labels.
- Under 600px tall, the stage art shrinks from 200px to 112px and the Start part and waiting slot shrink from 84px to 70px.

### Named Rules
**The Pre-Sized Stage Rule.** All three step panels occupy the same grid cell, so the stage is always as tall as its tallest panel. The waiting slot in step 1 and the Start part in step 2 share the same height, so changing steps never moves anything on the page.

## Elevation & Depth

Depth is physical, not atmospheric. Paper callouts lie flat on the page, defined by their ink line. Pressable parts are moulded plastic: a hard inset lip along the bottom edge in a darker mix of their own hue, plus one soft drop shadow underneath. Pressing pushes the part down 4px and shortens the lip, so a press reads as a real key travelling. Flat callouts that are not the focus (tuning cards) carry no shadow at all.

### Shadow Vocabulary
- **Drop** (`box-shadow: 0 7px 16px -6px rgb(27 27 31 / 0.38)`): the single lifted-object shadow. Used on raised parts, the stage, the controller diagram, the current step chip and the notice.
- **Plastic lip** (`inset 0 -7px 0` in the part's hue mixed 72% with black, in oklab): the bottom lip of large parts. Compresses to `-3px` with a tight `0 2px 6px -3px` shadow while pressed or busy.
- **Keycap lip** (`inset 0 -6px 0` at 70% mix, plus `0 5px 10px -5px rgb(27 27 31 / 0.4)`): the smaller lip on keycaps; the slider thumb uses a 4px version.

### Named Rules
**The Plastic Lip Rule.** Only pressable parts get the inset lip, and the lip is always derived from the part's own hue. A lip on a non-pressable surface is a lie about what can be pressed.

**The One Drop Rule.** There is one drop shadow. Do not invent stronger or coloured shadows for emphasis; emphasis comes from hue and size.

## Shapes

Everything is outlined. The standard boundary is a 2.5px solid ink line; secondary boundaries (part rows, step-chip number rings, tab icon tiles) use 2px. Dashed 2px lines at 40% ink mean "not yet": upcoming steps and the waiting slot.

Corners are generously rounded and scale with the object: 22px for callouts, 18px for parts and part rows, 16px for the top corners of tabs (their bottom edge is open and sits on the rail), 13px for keycaps, 9px for tab icon tiles, and full pills or circles for step chips, slider tracks, slider thumbs, magnifiers and tick badges. Nothing is square-cornered.

Icons are inline SVG line art drawn in the same ink with round caps and joins, never icon fonts or glyphs.

## Components

### Buttons (Parts)
Chunky, tactile toy parts that look like they could be pressed with a thumb.
- **Shape:** gently rounded plastic (18px), 2.5px ink outline, plastic lip and drop shadow.
- **Start (go):** Go Green, full stage width, 84px tall, play glyph plus 2.1rem label. The single action of step 2.
- **Stop:** Stop-Sign Red, full width, 62px tall, square glyph plus 1.4rem label. The single action of step 3.
- **Reset:** Paper White, full width, at least 64px tall, left-aligned label with a stroked reset glyph. First press arms it (turns Stop-Sign Red and asks again); second press resets.
- **Hover / Active / Focus:** hover lifts 1px; active and busy push down 4px with a compressed lip (120ms, ease-out). Focus is a 3px ink outline offset 3px.

### Tabs
Index tabs on the booklet's top edge.
- **Style:** Paper White, 2.5px ink outline with no bottom edge, 16px top corners, 54px tall, a 30px icon tile in the tab's hue to the left of an 800 label.
- **States:** unselected tabs sit 6px lower; hover raises them to 2px; the selected tab rises flush with the rail, fills with its hue, and its icon tile turns white. Focus outline is inset 7px so it stays inside the tab.

### Step Chips
Three pill chips (Controller, Start, Game) above the stage.
- **Upcoming:** dashed 40% ink outline, soft ink text, number in an outlined circle.
- **Current:** white paper, solid 2.5px ink outline, raised 2px with the drop shadow, number in a filled ink circle.
- **Done:** solid ink outline and full ink text; a filled ink tick badge scales in over the number.

### Cards / Containers (Callouts)
- **Corner Style:** 22px.
- **Background:** Paper White on the Sky Page.
- **Shadow Strategy:** the stage and the controller diagram take the drop; tuning callouts stay flat.
- **Border:** 2.5px ink.
- **Internal Padding:** 16px 20px for tuning callouts; 22px 26px 26px for the stage.

### Inputs / Fields
- **Slider:** a pill track in Sky Page with a 2px ink outline, a 32px Crayon Orange thumb with a 4px plastic lip; the thumb scales to 1.1 while dragged. Plain-language end labels ("Slow", "Fast") flank the track; the numeric value sits top-right in label type.
- **Keycap select:** the remapping control is a yellow keycap (178 × 56px) with the key label, a mouse glyph when the target is a mouse action, and a chevron. A transparent native select covers the keycap, so the platform picker opens but the visible control is always the keycap. Focus outlines the keycap. An unmapped keycap turns Paper White with soft ink text.
- **Fixed keycaps:** non-remappable bindings (W A S D, Mouse) are smaller plastic-grey keycaps that do not move on hover or press.

### Navigation
The tab rail is the only navigation: three equal-width tabs, arrow-key operable, with a 6px rail in the current section hue below them. There is no sidebar, header bar or menu.

### Part Row
One row per controller button on the Buttons page: a 58px magnifier circle, the action name (Title Small) over the controller-button name (Label), and a keycap on the right. Pointing at or focusing a row lights the matching button in the diagram and washes the row pale yellow while nudging it 3px right; pointing at the diagram does the reverse, and clicking a drawn button scrolls to its row and focuses its keycap.

### Controller Drawing (signature)
A single generic line-art gamepad (no brand trade dress, no lettered face buttons) reused everywhere: large in the stage, small in step 3, as the sticky diagram, and cropped inside each magnifier. Body in Paper White, buttons in Moulded Plastic, stick wells in Stick Well, all outlined in ink. The home button is always Go Green. A lit button turns Sticker Yellow with a heavier stroke. Asleep (disconnected), the drawing fades to Faint Ink strokes while the home button pulses and an ink arrow bobs toward it.

### Magnifier
A 58px circle with a 2.5px ink outline on Sky Page, showing a crop of the controller drawing centred on one button, which is lit Sticker Yellow. It replaces button names and icons for readers who cannot rely on text.

### Notice
A toast centred at the bottom: ink background, white 700 text, 16px corners, drop shadow. It rises 24px and fades in (320ms); errors use the Warning Brick background.

### Motion
One easing curve throughout (`cubic-bezier(0.16, 1, 0.3, 1)`, a strong ease-out). Presses take 120ms, state changes 160 to 260ms, the step change is a 460ms page-turn wipe from the right. Looping motion is limited to the step 1 waiting dots, the asleep controller's home button and arrow, and the step 3 switch arrow. With reduced motion requested, all animation and transition is switched off.

## Do's and Don'ts

### Do:
- **Do** outline every surface and part in ink (2.5px standard, 2px secondary); a shape without an ink line does not belong in the booklet.
- **Do** keep saturated hue on pressable parts only, and take a new section's colour from the current section hue so the tab, rail and its parts agree.
- **Do** give every pressable part the plastic lip in a darker mix of its own hue, the 1px hover lift and the 4px press.
- **Do** pair every colour state with a shape: tick, dash, glyph, numeral or raised position.
- **Do** pre-size anything that changes state (stage panels, waiting slot vs Start part, fixed-height chips and keycaps) so no state change moves the layout.
- **Do** draw new illustration as ink line art on the existing controller drawing, and show "which button" with a magnifier crop rather than a name.
- **Do** style only through classes and the root custom properties; the app runs under a strict content security policy with no inline styles.
- **Do** keep tap targets large: 46px minimum for chips and fixed keycaps, 54px and up for tabs and parts.

### Don't:
- **Don't** build dark chrome, status pills or bare native controls; state lives in the step strip and the stage, and sliders and pickers are always dressed as toy parts.
- **Don't** put saturated colour on backgrounds, text or decoration that cannot be pressed.
- **Don't** add a second typeface or a mid weight for names; names are 800, explanation is 500 in soft ink.
- **Don't** invent new shadows; use the one drop and the plastic lip.
- **Don't** use Minecraft, Mojang, Xbox or PlayStation logos, colours or lettered face-button trade dress anywhere in the drawing or UI.
- **Don't** use Faint Ink for readable text; it is for the asleep drawing only.
