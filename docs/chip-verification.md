# Chip verification

Scope: `Chip` and `ChipVariant` in [`src/components/chip.rs`](../src/components/chip.rs)
and [`assets/chip.css`](../assets/chip.css). Verified in the Dioxus preview on
2026-10-09, with headless Chromium 1194 using real pointer and keyboard input.
Desktop only. Android Chrome has not been tested.

## 1. References

- Material Web chip tokens, revision `47adb65`:
  [`_md-comp-assist-chip.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-assist-chip.scss),
  [`_md-comp-filter-chip.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-filter-chip.scss),
  [`_md-comp-input-chip.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-input-chip.scss),
  [`_md-comp-suggestion-chip.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-suggestion-chip.scss).
- shadcn-m3e `chip.tsx`, revision `8f1b3fb`, for the size set and the elevated style.
- **Not read:** the official Material chip pages at `m3.material.io`. The egress proxy blocks that host.

## 2. Values

| Property | Implemented | Source |
| --- | --- | --- |
| Height | 32px | tokens: `container-height` |
| Corner | 8px (corner-small) | tokens: `container-shape` |
| Label | label-large 14/20, weight 500, tracking 0.00625rem | tokens: `label-text-*` |
| Icon | 18px | tokens: `with-icon-icon-size` |
| Flat outline | 1px outline-variant | tokens: `flat-outline-color` |
| Elevated container | surface-container-low, level 1 at rest, level 2 on hover | tokens: `elevated-*`. The ambient shadow geometry matches the Material elevation levels. |
| Filter selected | secondary-container, no outline, label and icon on-secondary-container | tokens: `flat-selected-*` |
| Filter unselected label and icon | on-surface-variant | tokens: `unselected-*` |
| Assist label / icon | on-surface / primary | tokens: `label-text-color`, `with-icon-icon-color` |
| Suggestion and input label | on-surface-variant | tokens: `label-text-color` |
| State layers | hover 0.08, focus 0.10, pressed 0.10 | state tokens |
| Disabled | 12% container and outline, 38% label and icon | tokens: `disabled-*` |
| Leading inset with icon | 8px | **M3 spec value, not in the token files** |
| Label inset | 16px | **M3 spec value, not in the token files** |

## 3. Input, state and platform matrix

Raw data: [`chip-samples.json`](chip-samples.json).

| Check | Result |
| --- | --- |
| Height, corner, label type | 32px, 8px, 500 14/20 |
| Elevated container | `#f8f2fa`, which matches surface-container-low |
| Elevated shadow | Rest: level 1. Hover: level 2. |
| Selected filter container | `rgb(232, 222, 248)`, which matches secondary-container `#e8def8` |
| Filter toggle | `aria-pressed` false then true; check icon appears |
| Filter starting selected | `aria-pressed` true |
| Hover state layer | 0.08 |
| Pressed state layer | 0.10, with the pressed class on |
| Input remove | Hides the chip; restore brings it back |
| Disabled | Container 12% and label 38% on-surface, not-allowed cursor |
| Page errors | 0 |
| Keyboard Space, focus ring | Not measured separately in this round |

Visual check: [`chip-section.png`](chip-section.png). This is a resting state,
not a mid-animation capture.

## 4. Unresolved

- **Sizes.** The upstream reference also has 40px and 56px chips. The official
  token files define only the 32px size, so only 32px is implemented.
- **Inset values.** The 16px label inset and the 8px leading inset are M3 spec
  values, not token values. They have not been checked against the official
  spec pages, which are blocked from this session.
- **No shape change on press.** The chip token files define no pressed corner.
- **Input chip avatar, dragged state and keyboard focus** are not implemented or measured.
- **Official Material pages** not read.
- **Android touch and reduced motion** not measured.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed |
| Chip usage snippet compiled against the public API | Passed |
| Browser load | 0 page errors |
