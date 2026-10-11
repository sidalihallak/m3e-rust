# Time picker verification — 2026-10-11

## Scope and references

Public `TimePicker`, `TimeValue`, `TimePickerMode`, `TimePickerLayout`.
Sources: `src/components/time.rs`, `time_picker.rs`, **`time_picker.js`**,
`assets/time-picker.css` and Scheduling gallery examples.

Read current official [overview](https://m3.material.io/components/time-pickers/overview),
[specs](https://m3.material.io/components/time-pickers/specs),
[guidelines](https://m3.material.io/components/time-pickers/guidelines) and
[accessibility](https://m3.material.io/components/time-pickers/accessibility)
in the browser. Inspected the vertical dial measurement illustration and
measurement tables for dial, horizontal layout and input.
Read upstream [m3e.css](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css) and [tw-animate-css 1.4.0](https://unpkg.com/tw-animate-css@1.4.0/dist/tw-animate.css): animate-in defaults to ease.
Read actual upstream [time-picker.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/time-picker.tsx).
GitHub main resolved to c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972 on this date.

## Reference contract and implementation

| Element | Implemented contract |
| --- | --- |
| Container | Native modal with scrim, 28px corners, surface-container-high, 24px padding; source elevation 3 |
| Dial | 256px circle, surface-container-highest; 48px primary handle, 2px track, 8px center |
| Selectors | 96×80px / 57px display text; 24-hour vertical selectors 114px wide; input 96×72px / 45px display text |
| Selection colors | primary-container/on-primary-container for active field; highest/on-surface for other field |
| Period | 52×80px dial / 52×72px input; tertiary-container/on-tertiary-container for chosen period; outline border |
| Dial labels | Native 48×48 buttons, full hour/minute names, pressed state and one roving Tab stop |
| Rings | Source arrangement: outer 12, 1–11; inner 00, 13–23; radii 100/64px |
| Draft | Separate from controlled committed value; only OK calls onconfirm; Cancel/Escape/outside discard |
| Time model | Validated 24-hour storage, minute precision, no date/timezone conversion |
| Input | Native numeric-keyboard text inputs, no steppers; preserve raw typing, pad on blur/Enter; reject invalid values |
| Label changes | Upstream 200ms fade using the animation dependency’s default ease; owned cancellation and reduced-motion handling |
| Motion | Hand/track/handle use existing FastSpatial 360ms spring; field/period colors use FastEffects 150ms; shared Dialog uses existing entry/exit tokens |
| Adaptation | Portrait dial, horizontal arrangement, or input fallback; no dial density reduction |

Official pages recommend modal pickers; upstream renders an embeddable group.
The Rust API owns native modal presentation and keeps edits as a draft.
Official portrait 24-hour selectors use 114px; upstream uses 96px. Official
36px spacing before the dial takes precedence over upstream’s 20px gap. Native
dial buttons follow official labeling/Enter/Space guidance; upstream instead
exposes its dial as a slider. Twelve-hour arrow changes retain AM/PM, unlike the
upstream arrow handler's direct 24-hour increment.

Selector/period geometry follows official tables. Period segments are ~39px
tall in the 80px vertical container (~35px for input), rather than claiming all
controls are 48px. The explicit official 48px dial targets are implemented.
Footer/switch hit areas are 48px.

Adaptive thresholds (620px width, 600px height and 410px fallback height) are
implementation choices, not official breakpoint tokens. Explicit horizontal
layout only applies when it fits. Under 352px width, the period selector moves
below the fields; the 256px dial remains unchanged. Under 320px width, input is
preferred. English labels are currently fixed.

The first dial render has its selected angle. Subsequent angles remain
continuous, so 59→00 moves 6°, rather than making a full turn. Drag updates
disable hand interpolation until release. Pointer capture has owned cleanup
and cancellation restores the starting draft; cancellation events were not
independently simulated in this audit.

## Actual checks and measurements

| Platform/input/state | Observed result |
| --- | --- |
| Desktop pointer | Click hour 11 advances to minutes; native mouse drag from hour 12 to 3 yields 03:00 and focuses minute 0 |
| Keyboard | ArrowRight hour 9→10 moves focus; period ArrowDown changes AM→PM; Enter chooses hour 12; minute End→59 then ArrowRight→00 |
| 24-hour | AM/PM absent; pointer selection of inner-ring hour 17 updates to 17 and advances to minutes |
| Text | Hour 24 and minute 60 set aria-invalid and disable OK/switch; typed 5 stays raw until Enter, then 05 |
| Commit/rollback | Cancel leaves 09:30; reopen restores 09; input OK saves 23:59; 12 AM saves 00:00 |
| Dismissal | Escape restores Open 24-hour picker focus; actual outside mouse click closes and restores Open time picker |
| Geometry | Resting portrait 328×528, 28px corners; dial 256×256, handle/ticks 48×48, field 96×80 |
| 390×844 desktop viewport | Portrait dial fits without body scrolling |
| 320×844 desktop viewport | 304px container, unchanged 256px dial, period row 216×48, no horizontal overflow |
| 844×480 desktop viewport | Horizontal 560×412 dialog, no body scrolling |
| 844×390 desktop viewport | Input fallback, dial absent, no body scrolling |
| Light/dark | Rendered high/highest/primary/tertiary roles match scheme; restored light after checking dark |
| OS Reduce Motion | Browser media query true; entry samples stay at opacity 1/transform none; clock changes without intermediate interpolation; global reduced-motion override resolves to 0.01ms; restored off |
| Copy/export | Both copy controls exactly matched snippets; all 59 independent exported examples compile |

Default rendered contrast: light on-surface/high 13.92:1; active field 7.26:1;
selected period 7.24:1; white dial text/primary handle 6.46:1.
Dark on-surface/high 11.09:1; active field/period pairs remain above 7:1.
These measurements cover sampled default colors, not custom seeds or all overlays.

## Build and evidence

Wasm cargo check passes, including the final source in the wasm-dev profile with explicit dev inheritance. Two focused Rust time-model tests pass (noon/midnight,
strict input/bounds). JavaScript syntax check passes. Dioxus serve rebuilt
successfully: initial 20.73s, later 13.90s, 11.18s, 12.39s, 13.95s and 4.57s; final restarts/rebuilds succeeded in 24.31s and 15.41s.
Reloaded after successful build; one app shell confirmed. A disk-space failure
was resolved by removing only regenerable project build caches; sources and the
active preview bundle were preserved.

Independent source export plus 59 examples passes Wasm check; one existing
unused ChipSize import warning remains. Android was not launched.

[Raw samples](time-picker-samples.json) include entry, actual drag, 59→00 hand
motion, outside dismissal exit, reduced motion and responsive results.
Entry samples interpolate scale ~0.80→1 and opacity ~0.05→1; exit reaches scale
0.90/opacity 0 before native closure. Short-path hand samples start at a -6°
matrix and finish at 0°. Label opacity samples rise through 0.070, 0.215, 0.489, 0.699, 0.802 and 0.983
before reaching 1. The measurement caught CSS minification converting 200ms to
.2s; unit-aware parsing fixes an initial 0.2ms runtime duration. Reduced-motion
label samples remain at opacity 1. Tool timestamps are observation times, not
exact native input-event timestamps.

Screenshots:
[desktop](evidence/time-picker/desktop.png),
[dark](evidence/time-picker/dark.png),
[390px](evidence/time-picker/phone.png),
[320px](evidence/time-picker/narrow.png),
[landscape](evidence/time-picker/landscape.png).

## Remaining gaps

Android touch/scroll cancellation, on-screen keyboards and native Dioxus Android
remain paused. Desktop viewport and mouse results do not establish mobile touch.
Audible screen-reader output, forced colors, text enlargement, RTL/localized
digits/labels and live format changes while editing are not verified.
Pointercancel/lostcapture rollback, rapid repeated activation and held selector
ripples remain unmeasured. The source label fade is ported at 200ms with ease;
hand, label and modal motion are sampled. Complete pixel parity is not
claimed.

## Reproduce

Run `./scripts/dev.sh`, wait for successful build, reload `/#time-pickers`.
Open dial picker; click an hour and use minute arrows. Choose AM/PM with arrows.
Cancel/reopen and check the committed value. Open 24-hour input; try 24:60 then
23:59 and confirm. Check 12 AM/PM. Drag from the top to the right of the clock.
Inspect 390×844, 320×844, 844×480 and 844×390 layouts. Copy both snippets and use
`scripts/export-kit.py` for an editable library with runtime/CSS dependencies.
