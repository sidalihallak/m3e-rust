# Split buttons — 2026-10-10

## Scope and references

Implementation: `src/components/split_button.rs`, shared `action_control.rs`,
`menu.rs`/`menu.js`, `assets/split-button.css`, and composite motion CSS.
Supported: filled, tonal, outlined and elevated; five sizes; leading icons/icon
actions; RTL; disabled whole control or trailing half; controlled related menu.

Read the rendered Material [overview](https://m3.material.io/components/split-button/overview),
[specs](https://m3.material.io/components/split-button/specs),
[guidelines](https://m3.material.io/components/split-button/guidelines), and
[accessibility](https://m3.material.io/components/split-button/accessibility).
The path is singular `split-button`. Inspected upstream
[split-button.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/split-button.tsx)
and motion styles at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.
Official [Material Web tokens](https://github.com/material-components/material-web/tree/main/tokens)
were also inspected (local revision `47adb65`, token file version 34.0.21).
Retrieval date: 2026-10-10.

## Implemented values and reference choices

| XS/S/M/L/XL | Values in px |
| --- | --- |
| Heights | 32 / 40 / 56 / 96 / 136 |
| Trailing widths | 48 / 48 / 56 / 96 / 136 |
| Trailing icons | 22 / 22 / 26 / 38 / 50 |
| Resting inner corners | 4 / 4 / 4 / 8 / 12 |
| Hover/focus/pressed inner corners | 8 / 12 / 12 / 20 / 20 |
| Closed optical offset | −1 / −1 / −2 / −3 / −6; mirrored in RTL |
| Leading start/end padding | 12/10, 16/12, 24/24, 48/48, 64/64 |

The gap is 2px. The open trailing button is fully rounded, with its existing
container color and a 10% state layer. Opening centers and rotates the chevron
180° inward: positive in LTR, negative in RTL. The guideline specifically uses
the **Standard** scheme; the extracted Standard FastSpatial curve lasts 230ms.
Shape/ripple reuse the accepted kit baseline.

Official small trailing widths/icons take precedence over upstream's XS/S
32/40px widths and XS 20px icon. Both native buttons have a minimum 48px target.

## Runtime evidence and fixes

Measured all five geometries, the 2px gap and both directions of rotation. Native
Tab moves from the primary action to its trailing control. A leading click
increments only Save; trailing activation opens the related menu. Whole/partial
disabled controls use native `disabled`. The menu announces expanded state and
supports keyboard navigation and focus return.

Sampling exposed a CSS specificity conflict that kept the open corner at 12px;
the corrected result is 20px on the 40px control. It also exposed Dioxus's CSS
bundler lowering `:dir(rtl)` into language selectors: inherited direction tokens
now correctly produce mirrored optical offsets and negative RTL rotation. The
container color remained unchanged throughout sampled opening.

Actual OS Reduce Motion was checked and restored; the chevron reaches its end
state immediately. [Raw samples](composite-samples.json) record final runtime
measurements. Build/copy results: [index](composite-verification.md).

## Reproduction and remaining scope

Run the preview, wait for a successful build, reload and visit `/#split-buttons`.
Exercise both halves, Tab, Down/Up, Escape and the RTL example. Compare computed
corner radii and icon matrices during activation, rather than only the final image.

Android and native packaging are unverified. Audible screen-reader output,
forced colors and every live prop mutation remain open. Consumers should close
an open menu when dynamically disabling its trigger. Longer held/touch-cancel
inputs require separate device checks. See [menu scope](menu-verification.md).
