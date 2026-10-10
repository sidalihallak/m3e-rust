# Slider — current fidelity update

## Accessibility follow-up — 2026-10-10

See [the accessibility audit](accessibility-verification.md) and
[raw samples](accessibility-samples.json) for tested behavior and remaining gaps.
Native inputs now declare vertical/horizontal orientation. The copied range
example includes an accessible name for both price handles.


Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/sliders/overview)
- [Material specs](https://m3.material.io/components/sliders/specs)
- [Material guidelines](https://m3.material.io/components/sliders/guidelines)
- [Material accessibility](https://m3.material.io/components/sliders/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/slider.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R2/R3/R13; coordinated native range bounds/geometry, ≥48px targets, exact step ticks, 48px value bubble, size/centered/inset-icon API. Start End and end Home remain 70–70. Step30 ticks are 0/.3/.6/.9. Track heights 16/24/40/56/96px and handle heights 44/44/44/68/108px measured.

## Checks, dependencies and gaps

- Locked offline Wasm check and fresh Dioxus web build passed; live preview
  was restarted/reloaded and checked for one app-shell.
- [Shared fix matrix, measured results and reproduction steps](component-fidelity-fixes.md)
- [Raw runtime measurements](component-fidelity-fixes-samples.json)
- [Copy dependencies and source export](copy-components.md); exported library
  and all 19 current usage examples compile independently for Wasm.
- Platform scope: desktop pointer/keyboard checks named above. Android remains
  paused; touch, assistive technology and OS reduced-motion true are untested.
  Source gating is not claimed as an OS-preference runtime measurement.
- Historical screenshots/samples below establish only the state and version
  in which they were captured. See the shared fix report for current gaps.

## Historical report before this fix

# Slider verification

Scope: `Slider` in [`src/components/slider.rs`](../src/components/slider.rs) and
[`assets/slider.css`](../assets/slider.css). Verified in the Dioxus preview on
2026-10-09, with headless Chromium 1194 using real pointer and keyboard input.
Desktop only. Android Chrome has not been tested.

## 1. References

- Material Web slider tokens, revision `47adb65`
  ([`_md-comp-slider.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-slider.scss)):
  16dp tracks, 4×44dp handle, 2dp pressed handle, 4dp stop indicator with 4dp trailing
  space, 40dp state layer, colours, disabled 0.38 and 0.12.
- shadcn-m3e `slider.tsx`, revision `8f1b3fb`: the 8dp gap (6dp plus half the handle),
  the stop indicator hidden at the maximum, and the handle width spring.
- **Not read:** the official Material slider pages at `m3.material.io`. The egress proxy blocks that host.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Track | 16dp tall, full radius outer corners, 2dp inner corners | Tokens |
| Handle | 4×44dp; 2dp while pressed | Tokens |
| Gap | 8dp either side of the handle centre | Upstream |
| Handle travel | Centre runs from 2dp to (width − 2dp), so the handle stays in the track | Layout |
| Stop indicator | 4dp, 4dp from the end; hidden at the maximum | Tokens and upstream |
| State layer | 40dp circle on the handle; hover 0.08, focus and pressed 0.10 | Tokens |
| Handle, active track | primary | Tokens |
| Inactive track | secondary-container | Tokens |
| Disabled | 38% handle and active track; 12% inactive track | Tokens |
| Motion | handle width and track ends follow the expressive fast spatial spring (360ms) | Upstream |
| Keyboard | Native range input: arrows, Home, End, page keys | Native |

## 3. Input, state and platform matrix

Raw data: [`slider-samples.json`](slider-samples.json).

| Check | Result |
| --- | --- |
| Handle centre at 40% | 307px, matching the expected position exactly |
| Active track end | 8px before the handle centre |
| Pointer press at 20% | Value 19.76: the pointer maps through the same 2dp inset as the handle |
| Drag from 20% to 70% | Value 70.16; handle centre matches the expected position |
| Handle width while held | 2px (after the spring settles); 4px after release |
| Arrow, Home, End | Right twice from 0 gives 2; Home gives 0; End gives 100 |
| Focus state layer | 0.10 |
| Maximum | Stop indicator hidden; handle inside the track |
| Disabled | Pointer click does not change the value |
| Page errors | 0 |

## 4. Diagnosed defects

- **Mismatched thumb.** The native range thumb would map the pointer to a value
  using its own width, so a thumb wider or narrower than the drawn handle would make
  the drawn handle and the value disagree. The native thumb is set to 4×44px, so
  pointer and drawn positions agree.
- **Test scrolling.** Early drag results read as "no change" because the slider was
  below the fold. The mouse events landed outside the viewport. The test now scrolls the
  control into view first. This was a test error, not a component error.

## 5. Value label, ticks, range and vertical

Added in this round. Each item is measured in section 9.

- **Value label:** shown above the handle while it is pressed or focused. Hidden at rest.
- **Tick marks:** one per `step`, inside the track. Ticks in the active range use the on-primary colour.
- **Range slider:** `value_end` adds a second native input and handle. The active track runs between the handles.
- **Vertical:** `vertical: true` rotates the control. The value still increases upward. Labels stay upright.

## 6. Unresolved

- Touch, Android Chrome, reduced motion, and screen-reader announcements are not measured.
- Range and vertical drag are measured with the mouse only. Keyboard use of the second handle is not measured.
- Visual check is not recorded in this round; the handle position, gap and pressed width were measured.
- Official Material slider pages not read.

## 7. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed |
| Slider usage snippet compiled against the public API | Passed |
| Range and vertical usage snippet compiled against the public API | Passed |

## 8. Drag alignment and highlight (update)

Reported: during a drag the handle and track did not stay aligned with the pointer, and
the state-layer circle showed on the handle.

- **Cause:** the track and handle used a 360ms spring on position, so they trailed the
  value while the pointer moved.
- **Fix:** while the pointer is down, the track and handle update without transitions.
  The state-layer circle is removed from the handle. The focus outline stays for keyboard users.
- **Measured:** over 18 drag steps (value 44 to 88 and back), the handle centre stays
  within 0.02px of the expected position, and the active track stays exactly 8px from
  the handle centre.
- **Not measured:** touch dragging, and the visual result on the user's device.

## 9. Value label, ticks, range and vertical (measured)

Raw data: [`slider-ext.json`](slider-ext.json). Script: the extended harness run against the
preview at 1280×900, mouse input only.

| Check | Expected | Measured |
| --- | --- | --- |
| Range, start handle centre | 2 + (W − 4) × 0.20 | 0px offset |
| Range, end handle centre | 2 + (W − 4) × 0.70 | 0px offset |
| Range, active track width | end − start − 16px | 0px difference |
| Ticks at step 10 over 0–100 | 11 ticks | 11 ticks |
| Ticks in active range 20–70 | 6 ticks active | 6 ticks active |
| Drag end handle from 70 to about 90 | value 90 | value 90; handle offset −0.02px after release |
| End value label while dragging | visible | opacity 1, reads "90" |
| End value label at rest | hidden | opacity 0 |
| Vertical handle centre, value 60 | 60% up from the bottom | 0.01px offset |
| Vertical drag up 48px from value 60 | value about 80 | value 80.34; handle offset 0.02px |
| Vertical label rotation | upright | none |
| Page errors | 0 | 0 |

- **Handle height in vertical mode:** a single read 300ms after release showed 4.19px.
  A later probe showed 4px at 400ms, so this was the 360ms spring still settling, not a
  steady-state offset. The probe is in the scratchpad (`slider-hcheck.mjs`).
- **Visual:** the section screenshot shows the range track with its active segment
  between the handles, ticks with the active ones highlighted, and both vertical forms.
  The label is not shown at rest, as intended.
- **Not measured:** label placement against the handle at the vertical extremes, touch
  drag, and keyboard use of the second handle.
