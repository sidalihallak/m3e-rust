# Segmented button verification

Scope: `SegmentedButtonSet` and `SegmentedButton` in
[`src/components/segmented_button.rs`](../src/components/segmented_button.rs) and
[`assets/segmented-button.css`](../assets/segmented-button.css). Verified in the Dioxus
preview on 2026-10-09, with headless Chromium 1194 using real pointer and keyboard input.
Desktop only. Android Chrome has not been tested.

## 1. References

- Material Web labs outlined segmented button tokens, revision `47adb65`
  ([`_md-comp-outlined-segmented-button.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-outlined-segmented-button.scss)):
  40dp height, 1dp outline, label-large, 18dp icon, 12dp inset, state layers 0.08 hover and
  0.10 focus/pressed, disabled 38% label and icon, 12% outline and selected container.
- Material Web labs segmented button component
  ([`labs/segmentedbutton`](https://github.com/material-components/material-web/tree/47adb65/labs/segmentedbutton)).
- shadcn-m3e `toggle-group.tsx`, revision `8f1b3fb`: the connected layout (2dp gap, small inner
  corners, full outer corners, fully round selected item) and the 4dp pressed corner.
- shadcn-m3e `button.tsx` and kit `button.css`: the DefaultEffects shape spring (240ms) used for
  `transition-shape`.
- **Not read:** the official Material segmented button page at `m3.material.io`. The egress proxy
  blocks that host. The labs token file is the closest official source available here.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Height | 40px | Tokens |
| Outline | 1px outline; none when selected | Tokens |
| Gap between items | 2px | Upstream connected layout |
| Outer corners at rest | Full (20px) | Upstream |
| Inner corners at rest | 8px | Upstream `radius-sm` |
| Selected corners | Fully round (20px) | Upstream |
| Pressed corners | 4px, all corners | Upstream `radius-xs` |
| Shape change | 240ms DefaultEffects spring | Upstream `transition-shape` |
| Label | label-large, 14/20, weight 500 | Tokens |
| Icon | 18px; a check when selected unless an icon is given | Tokens, upstream check |
| Selected | secondary-container fill, on-secondary-container label | Tokens |
| State layer | hover 0.08; focus and pressed 0.10 | Tokens |
| Focus indicator | 3px secondary outline, 2px offset | Kit button focus |
| Disabled | 38% label and icon; 12% outline; 12% container when selected | Tokens |
| Single select | Native radios with one `name`; arrows move the selection | Native |
| Multi select | Native checkboxes; Space toggles | Native |

## 3. Input, state and platform matrix

Raw data: [`segmented-samples.json`](segmented-samples.json) and
[`segmented-release-samples.json`](segmented-release-samples.json).

| Check | Result |
| --- | --- |
| Item height | 40px for all items |
| Gap between items | 2px |
| Rest corners, first / middle / last | `20px 8px 8px 20px` / `20px` / `8px 20px 20px 8px` |
| Hover state layer | 0.08 |
| Held press on an unselected item, 400ms | Corners 4px; state layer 0.10; not selected while held |
| Release on the item | Selected; corners settle at 20px; fill is secondary-container |
| Release curve (samples after release at 0/60/120/240/480ms) | 6.4 → 17.5 → 19.8 → 20.0 → 20px; no overshoot |
| Previous selection after a new selection | Corners return to inner 8px; fill and check removed |
| Press, drag off the item, release | Selection unchanged |
| Arrow Left / Right with focus on the radio | Selection moves to the previous / next item |
| Focus-visible outline | solid 3px |
| Multi select, click | Toggles the clicked item only |
| Multi select, Space on focused checkbox | Toggles that item |
| Selected item with an icon | Shows the icon instead of the check |
| Disabled item, pointer click | Selection unchanged |
| Disabled item | Label 38% on-surface |
| Page errors | 0 |

Single-select state was checked with the native input state and the selected class on each
item, after the fix in section 4.

## 4. Diagnosed defects

- **Radio change reported unchecked.** The first version passed `event.checked()` to the
  change handler. For a radio, the change event fires only for the newly checked radio, so the
  signal never moved and the previous item kept its selected class. The handler now reports `true`
  for radios. Checkboxes still use `event.checked()`, which was verified to toggle correctly.
- **Stale preview build.** After the radio fix, the preview still served the old wasm: the
  dev server stopped rebuilding. The first post-fix run was not valid. The server was restarted,
  and the results in section 3 come from the rebuilt bundle, with the wasm timestamp checked.
- **Overshooting corners.** The first corner transition used the 360ms slider spring, which
  overshot to 21.2px on release. It now uses the button shape spring (240ms, no overshoot).

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Usage snippet compiled against the public API | Not compiled separately. The copyable snippet uses the same calls as the preview's single-select card, which compiles |
| Preview served from a rebuilt bundle | Passed after restart |

## 6. Raw measurements, screenshot and evidence

- [`segmented-samples.json`](segmented-samples.json): geometry, rest and held corners, arrow keys,
  multi-select, disabled.
- [`segmented-release-samples.json`](segmented-release-samples.json): corner curve through release
  and the press-drag-off case.
- [`segmented-section.png`](segmented-section.png): the preview section after the checks. The
  multi-select card is all-selected because the harness left all three toggled on.

## 7. Unresolved differences and untested behaviour

- Touch input, Android Chrome, and the on-screen keyboard are not measured.
- Reduced motion is implemented as a 0.01ms transition, but it is not measured.
- Screen-reader output for the group and each radio or checkbox is not measured.
- Only the standard 40dp size is provided. Material Web labs has no other size for this component.
- The official Material segmented button page was not read (blocked host). The shape values come
  from the upstream connected group, not from the Material spec text.
- The corner morph on a press of an already-selected item is not measured separately.

## 8. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run the harness against the preview:
   `S=<output dir> node segmented-verify.mjs` followed by `S=<output dir> node segmented-release.mjs`,
   using `playwright-core` with the Chromium at `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`.
3. Compare the output with [`segmented-samples.json`](segmented-samples.json) and
   [`segmented-release-samples.json`](segmented-release-samples.json).
