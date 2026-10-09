# Progress indicator verification

Scope: `LinearProgress` and `CircularProgress` in
[`src/components/progress.rs`](../src/components/progress.rs) and
[`assets/progress.css`](../assets/progress.css). Verified in the Dioxus preview on
2026-10-09, with headless Chromium 1194. Desktop only. Android Chrome has not been tested.

## 1. References

- Material Web progress indicator tokens, revision `47adb65`
  ([`_md-comp-progress-indicator-linear.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-progress-indicator-linear.scss),
  [`_md-comp-progress-indicator-circular.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-progress-indicator-circular.scss)).
- shadcn-m3e `progress.tsx` and `m3e.css`, revision `8f1b3fb`. The linear
  indeterminate keyframes, the 2.1s loop, the 1.15s delay and the 1.5s wave slide are from here.
- **Not read:** the official Material progress pages at `m3.material.io`. The egress proxy blocks that host.

## 2. Values

| Property | Implemented | Source |
| --- | --- | --- |
| Linear flat thickness | 4px | tokens: `active-indicator-thickness` |
| Linear thick thickness | 8px | tokens: `thick-active-indicator-thickness` |
| Track | surface-container-highest | tokens: `track-color` |
| Gap between active and track | 4px | tokens: `track-active-indicator-space` |
| Stop dot | 4px, primary | tokens: `stop-indicator-size` |
| Wave amplitude | 3px | tokens: `active-indicator-wave-amplitude` |
| Wave wavelength | 40px determinate, 20px indeterminate | tokens: `active-indicator-wave-wavelength` |
| Wavy height | thickness + 6px (10px flat, 14px thick) | tokens: `with-wave-height`, `thick-with-wave-height` |
| Indeterminate bars | 2.1s linear loop, second bar delayed 1.15s | upstream `m3e.css` keyframes |
| Wave slide | one wavelength per 1.5s | upstream `m3e.css` `m3-wave-slide` |
| Circular size | 40px, 4px stroke | tokens: `size`, `active-indicator-thickness` |
| Circular thick | 52px, 8px stroke | tokens: `thick-size`, `thick-active-indicator-thickness` |
| Circular wave | 1.6px amplitude, 15px wavelength, 48px wavy size | tokens. **Not implemented.** |

## 3. Input, state and platform matrix

Raw data: [`progress-samples.json`](progress-samples.json).

| Check | Result |
| --- | --- |
| Linear flat determinate, value 0.4 | Active bar is 40.0% of the track width |
| After Advance (0.6) | Active bar is 60.0% of the track width |
| Stop dot | Present below 100% |
| Flat and thick heights | 4px and 8px |
| Wavy active height | 10px |
| Wavy wave position | Moves over time (samples differ) |
| Indeterminate linear bars | Positions change over time. Both bars leave the track at the start of each loop, as the keyframes specify. |
| Circular determinate, value 0.6 | Dash offset 45.24, which equals circumference × (1 − 0.6) |
| Circular indeterminate | Rotation changes over time |
| Thick circular | 52px |
| Page errors | 0 |

Visual check: [`progress-section.png`](progress-section.png). The indeterminate bars
appear in only part of the cycle. The screenshot is a resting capture, not a mid-animation capture.

## 4. Diagnosed defect

The wavy tracks rendered black. The wave was a data-URI SVG with a black stroke,
and a background image cannot take the theme colour. The wave is now a CSS mask,
and the element is filled with the active colour. The slide animates the mask
position. The screenshot confirms the primary colour.

## 5. Unresolved

- **Wavy circular is not implemented.** Upstream uses a masked full-circle wave
  with a counter-rotation and an arc that grows and shrinks.
- **Circular indeterminate** rotates a fixed 85-unit arc. The official arc-length animation is not implemented.
- **Linear wave slide duration** (1.5s) comes from upstream, not from the official tokens.
- **Official Material pages** not read.
- **Android and reduced motion** not measured. Reduced motion pauses the animations.

## 6. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed |
| Progress usage snippet compiled against the public API | Passed |
| Browser load | 0 page errors |
