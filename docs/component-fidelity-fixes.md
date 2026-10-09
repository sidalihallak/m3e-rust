# Material 3 fidelity fixes — 2026-10-09

## Scope and references

Follow-up to [the fidelity review](component-fidelity-review.md), covering its
R1–R13 findings and the additional actionable desktop differences in its
coverage table. Implementation is in the Dioxus Rust components, component CSS,
and gallery. The accepted Button implementation was preserved. The existing
`rskit-ui` project was not changed.

The official Overview, Specs, Guidelines and Accessibility sections were read
in the preceding review on this date; the same readings establish this fix's
contract. Sources and choices are recorded in each component's report below.
Actual upstream component and style files were inspected again while fixing:

- [shadcn-m3e c37c0d2](https://github.com/Crysta1221/shadcn-m3e/tree/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src)
- [Material Web 47adb65 tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)
- [Accepted Button motion](button-motion-verification.md)

## Resolution matrix

| Finding | Change | Desktop evidence |
| --- | --- | --- |
| R1 disabled tabs | Arrows, Home and End navigate only enabled indices; invalid selected index gets an enabled roving focus fallback | Specs → ArrowRight selects/focuses Overview; End selects/focuses Specs. Reviews stays disabled, unselected, tabindex −1 |
| R2 inverted range | Normalize controlled values; give native start/end inputs coordinated bounds and geometry | Start End produces 70–70; end Home remains 70–70. Native bounds follow the other handle |
| R3 undersized targets | Reserve extended target space for small icons, checkbox, radio, chips, segments, small FAB; full-height tabs; 48px native slider thumbs with correct track mapping | Actual pointer click 6px outside XS visual edge hits XS and starts a ripple. Radio/segment inputs measure 48px; slider target cross-axis is ≥48px. Visual geometry remains independent |
| R4 lost feedback | Shared copyable Ripple helper uses native pointer/Space/click fallback, 225ms generation-owned minimum press, pointer cancellation, focus-out release, unmount cleanup | Final XS click/Enter reaches radius 8.0185/8.01963px and ripple opacity 0.10, at unchanged 32×32px. FAB Enter reaches 0.10 at unchanged 56×56px. Space, repeated presses and drag/leave checked; disabled icon/checkbox remain inactive |
| R5 selected icons | Selected labeled filter chips and segments replace their original icon with CHECK | Vegan selects with check; Month, Favourite and Add selected segments render check |
| R6 filled toggle roles | Unselected filled toggle uses surface-container/on-surface-variant; selected uses primary/on-primary | Unselected RGB(242,236,244), selected primary token #65558f; selected glyph favorite-fill |
| R7 chip dismissal | Delete/Backspace and remove button share dismissal and move focus to the next available control | Delete and Backspace remove Design review; focus moves to a surviving control (Medium chip in the final gallery) |
| R8 FAB menu | Trigger precedes items; arrows/Home/End, forward Tab and Escape/selection focus return; absolute floating content; state elevation; dual glyph crossfade | ArrowDown opens/focuses Favourite; ArrowDown Details; End Add; Escape returns Actions and closes. Forward Tab enters own menu. Closed/open root height 56px; focused item elevation 3 |
| R9 text-field surface | Native label accepts clicks; 56px container forwards surface clicks; readonly prop; ResizeObserver measures notch changes | Email label click focuses Email. Readonly Reference stays REF-123 after x. Label change updates notch-text width 61.492 → 156.711px. Earlier notch and number-spinner fixes retained |
| R10 reduced-motion sweep | Frame helper checks matchMedia every frame and pauses elapsed time/path writes; mode changes stop old loop | Source gate reviewed; normal sweep measured. OS preference true was **not** exercised; no runtime reduced-motion certification claimed |
| R11 progress rendering | Full inactive track for indeterminate linear; full wave height and transparent moving bar backgrounds; exact upstream two-half circular spinner and keyframes | Linear heights 4/10/14px; indeterminate track present. Circular has two half SVGs with 1.333s expand, 5.332s arc rotation, 1.56824s outer rotation. Dynamic wavy mode has 9 distinct paths/19 transforms in ~4s; switching determinate removes sweep path and sets loop false |
| R12 docs/copy workflow | Complete dependency guide and source export; current API usage; updated reports; precise scope instead of blanket verified claims | Exported standalone library and all 19 usage snippets pass locked offline Wasm check. Copy control reports Copied after clipboard API resolves; automation's virtual clipboard is not synchronized with this write, so end-to-end paste comparison is not established |
| R13 tick arithmetic | Normalize min+i×step instead of equal fractions; show non-dividing-step example | 0..100 step30 renders 0%,30%,60%,90%; native value is on that grid |

## Additional changes from the coverage table

- Icon-button Narrow/Default/Wide API with current per-size tokens; explicit
  `selected_icon` support. Small narrow/wide measure 32×40 and 52×40px.
- Chip 32/40/56px sizes matching upstream icon and padding choices.
- Slider XS/S/M/L/XL tracks 16/24/40/56/96px and handles 44/44/44/68/108px,
  centered selection and inset icons. Value bubble minimum width 48px.
- Tabs accept stable `tab_ids` and `panel_ids`; gallery and copy example render
  associated panels. Primary indicator placement received a later correction: 2px horizontal
  label insets, 24px minimum and rounded-top profile flush with the divider.
  See the current tabs report; the earlier bottom-gap interpretation was wrong.
- All-disabled segmented outline uses the disabled outline role.
- Divider start inset uses margin-inline-start: actual RTL margin-right 16px,
  margin-left 0px.
- Text field readonly and changing-label notch are now implemented.

## Source-specific motion choices

Icon shape now uses the upstream **240ms DefaultEffects** curve, matching
`transition-shape`; the earlier 360ms spatial curve could overshoot the pressed
radius. This is source correction, not guessed tuning. Shared ripple growth is
450ms standard easing, fade in/out 105/375ms linear, opacity .10, minimum press
225ms, pointer origin to control center, same geometry and gradient as Button.
Touch delay is 150ms in source; touch/scroll behavior remains untested.

FAB menu uses directly copied upstream generated curves: 360ms FastSpatial
trigger, 440ms DefaultSpatial items with 35ms stagger, 240ms DefaultEffects glyph
crossfade/rotation. Menu item elevation follows **official** 0 rest, 3
focus/press, 4 hover; upstream's elevation 3 at rest is intentionally not copied.
Linear inactive color follows official surface-container-highest, while upstream
uses secondary-container. Circular track retains upstream secondary-container.

Loading indicator normal motion: 12 distinct paths and rotations over ~1.3s at
48×48px. Flat spinner sampling: 12 distinct outer rotations and 11 distinct
left/right half rotations. Menu glyph opacity changed in 4 of 16 samples;
menu root height remained 56px throughout.

Accepted Filled Button regression check: minimum radius 8.0367px, peak ripple
.10, unchanged 71.078125×40px. No Button source or CSS was changed.

## Build and evidence

- `cargo check --locked --offline --target wasm32-unknown-unknown`: passed.
- `dx build --platform web --locked --offline`: passed.
- `./scripts/dev.sh`: fresh successful build; preview reloaded and left running.
- A concurrent dx build/serve appended its loader twice during this work.
  Stopped the old server, removed the generated JS bundle and restarted one
  server. Final browser check: **one** app-shell. Do not build into the live
  serve output concurrently when reproducing.
- Exported independent library + 19 usage examples: locked offline Wasm check
  passed; no dependency on the original checkout's component sources.
- [Raw fix measurements](component-fidelity-fixes-samples.json): actual DOM
  style polls and real input observations; times are polling times, not exact
  event/frame timestamps. Earlier and final curve samples are named separately.
- [Slider gallery screenshot](component-fidelity-fixes.png): resting desktop
  appearance; not a motion frame certification.

## Remaining scope

These fixes resolve the identified implementation defects in the desktop scope.
They do not establish exhaustive Material or upstream parity. Android/emulator
work remains paused. Native Android, touch scrolling, assistive technology,
OS reduced-motion true, exhaustive dark/seed contrast, full RTL navigation and
all dynamic prop permutations remain untested. Special interactions beyond
this kit's current scope (switch drag, card drag, chip avatar/drag, scrollable or
vertical tabs) remain feature work; these are not represented as verified.
A copied application's CSS/assets/theme installation still needs checking in
that application. The source export was compile-checked, not independently
browser-launched. Browser automation's clipboard adapter could not validate
native paste despite the application receiving clipboard API success.

## Reproduce

1. Start ./scripts/dev.sh; confirm build success, reload, and verify one gallery.
2. Product Specs → ArrowRight/End; inspect focus, selection, disabled Reviews,
   and matching aria-controls/tabpanel IDs.
3. Price start End; Price end Home. Check ordered values and bound attributes.
4. Activate XS icon and Standard FAB by quick click/Enter/Space; sample during
   press and release. Repeat rapidly, drag out, and check disabled controls.
5. Select Vegan and labeled segments; inspect CHECK. Remove Design review with
   Delete/Backspace; inspect surviving focus.
6. Actions → ArrowDown, ArrowDown, End, Escape; open with Enter then Tab; select
   an item and click outside. Inspect glyph transition and 56px root height.
7. Click Email label and field edges. Readonly Reference must remain REF-123.
   Change field label; compare --notch-text-w after ResizeObserver runs.
8. Inspect flat two-half spinner and linear inactive tracks. Toggle progress
   mode in the same component instance and inspect data-m3-frame-loop.
9. Verify non-dividing ticks, size examples and RTL divider; compile an exported
   kit with the gallery snippets as described in copy-components.md.

## User alignment follow-up — 2026-10-09

The user identified badge placement and segmented content centering after the
initial fixes. Both received measured corrections. Badge count leading edges
now use the official fixed offsets and mirror for RTL; segmented content's
4px shift from an empty flex child is removed. See the current
[badge report](badge-verification.md),
[segmented report](segmented-button-verification.md), and
[raw alignment evidence](badge-segmented-alignment-samples.json).

## User primary-indicator follow-up — 2026-10-09

The current [tab report](tabs-verification.md) corrects the prior mistaken
interpretation of a 2dp bottom inset. The official diagram specifies horizontal
label insets. Current geometry, source ambiguity, motion and tested scope are
recorded there with [raw measurements](tabs-indicator-verification-samples.json).
