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

## 5. Not implemented

- **Value label** (the upstream and reference value bubble) and **tick marks**.
- **Range sliders** (two handles).
- **Vertical orientation.**

## 6. Unresolved

- Touch, Android Chrome, reduced motion, and screen-reader announcements are not measured.
- Visual check is not recorded in this round; the handle position, gap and pressed width were measured.
- Official Material slider pages not read.

## 7. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed |
| Slider usage snippet compiled against the public API | Passed |

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
