# Icon button verification

Scope: `IconButton`, `IconButtonVariant`, `IconButtonSize`, `IconButtonShape` in
[`src/components/icon_button.rs`](../src/components/icon_button.rs) and
[`assets/icon-button.css`](../assets/icon-button.css). Verified in the Dioxus
preview on 2026-10-09, with headless Chromium 1194 using real pointer and keyboard
input. Desktop only. Android Chrome has not been tested.

## 1. References

- Material Web icon button tokens, revision `47adb65`
  ([`_md-comp-icon-button-*.scss`](https://github.com/material-components/material-web/tree/47adb65/tokens/versions/latest/sass)).
  These give sizes, icon sizes, corner shapes, colour roles, state layers and disabled values.
- shadcn-m3e motion tokens, revision `8f1b3fb`
  ([`m3e.generated.css`](https://github.com/Crysta1221/shadcn-m3e/blob/8f1b3fb/packages/m3e/src/styles/m3e.generated.css)).
  The expressive fast spatial spring is 360ms, with overshoot. The icon button uses it, following its official token (`spring-fast-spatial`).
- shadcn-m3e `button.tsx`, revision `8f1b3fb`, for the press-shape approach.
- **Not read:** the official Material icon button pages at `m3.material.io`. The egress proxy blocks that host.

## 2. Values

| Size | Container | Icon | Round corner | Square corner | Pressed corner | Selected round | Selected square |
| --- | --- | --- | --- | --- | --- | --- | --- |
| XS | 32px | 20px | 16px | 12px | 8px | 12px | circle |
| S | 40px | 24px | 20px | 12px | 8px | 12px | circle |
| M | 56px | 24px | 28px | 16px | 12px | 16px | circle |
| L | 96px | 32px | 48px | 28px | 16px | 28px | circle |
| XL | 136px | 40px | 68px | 28px | 16px | 28px | circle |

Round is a circle (half the container). The selected toggle swaps round and
square: a selected round becomes the selected corner, and a selected square
becomes a circle. The pressed corner applies in every state.

| Variant | Container | Icon | Selected container | Selected icon | Outline |
| --- | --- | --- | --- | --- | --- |
| Standard | transparent | on-surface-variant | transparent | primary | none |
| Filled | primary | on-primary | primary | on-primary | none |
| Tonal | secondary-container | on-secondary-container | secondary | on-secondary | none |
| Outlined | transparent | on-surface-variant | inverse-surface | inverse-on-surface | outline-variant |

State layers: hover 0.08, focus 0.10, pressed 0.10. Disabled: 10% container, 38% icon.

## 3. Input, state and platform matrix

Raw data: [`icon-button-samples.json`](icon-button-samples.json).

| Check | Result |
| --- | --- |
| Sizes XS, S, M, L, XL | Measured 32, 40, 56, 96, 136px, with icons 20, 24, 24, 32, 40px. Round radius is half the container. |
| Filled and tonal colours | Match the theme tokens exactly, as hex |
| Press (pointer held 500ms) | Corner moves from 20px to a minimum of 6.9px and a maximum of 21.1px, then returns to 20px. The overshoot is the expressive spring. |
| Square toggle, unselected | 12px corner |
| Square toggle, selected | Circle (20px for S), `aria-pressed` true |
| Standard toggle | `aria-pressed` false, then true after click |
| Disabled press | No pressed class |
| Page errors | 0 |
| Keyboard Space | Pressed state from the same code path as pointer. Not measured separately. |
| Enter key | Native click. Not measured. |
| Focus via Tab | Focus ring is set in CSS. Not measured. |

Visual check: [`icon-button-section.png`](icon-button-section.png). This is a resting state,
not a mid-animation capture.

## 4. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed |
| Icon button usage snippet compiled against the public API | Passed |
| Browser load | 0 page errors; all sections rendered |

## 5. Unresolved differences

- **No ripple.** The official icon button tokens define a state layer and a shape
  change. The Button pilot has a ripple, and the kit's FABs use one.
- **Spring choice.** The icon button uses the 360ms fast spatial spring, from its
  token. The Button pilot uses the 240ms effects spring. Both come from the same
  expressive motion scheme.
- **Official Material pages** not read.
- **Keyboard and focus, Android touch and reduced motion** not measured in this round.
- **Focus ring colour** is secondary, as with the other new components. The Button pilot uses primary.

## 6. Steps to reproduce

1. Start the preview as in the [switch report](switch-verification.md#8-steps-to-reproduce).
2. In section 06, press and hold a filled icon button, and read `border-radius` while held and after release.
3. Click a square toggle, and read its `border-radius` and `aria-pressed`.
4. Compare with the table in section 2.
