# Divider verification

Scope: `Divider`, `DividerOrientation` and `DividerInset` in
[`src/components/divider.rs`](../src/components/divider.rs) and
[`assets/divider.css`](../assets/divider.css). Verified in the Dioxus preview on 2026-10-09 with
headless Chromium 1194, at device scale 2. Desktop only.

## 1. References

- Material Web divider tokens, revision `47adb65`
  ([`_md-comp-divider.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-divider.scss)):
  1px thickness, outline-variant colour.
- shadcn-m3e `separator.tsx`, revision `8f1b3fb`: `bg-outline-variant`, `h-px` horizontal and
  `w-px` vertical, `self-stretch` vertical.
- The 16dp inset is the usual list inset. It is a choice, not a token value.
- **Not read:** the official `m3.material.io` divider page. The egress proxy blocks that host.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Thickness | 1px | Tokens |
| Colour | outline-variant | Tokens |
| Horizontal width | Full container width | Upstream |
| Vertical | 1px wide, stretched to the row height | Upstream |
| Inset start | 16px on the start side | Choice (list inset) |
| Inset middle | 16px on both sides | Choice (list inset) |
| Semantics | `role="separator"` with `aria-orientation` | ARIA |

## 3. Input, state and platform matrix

Raw data: [`divider-samples.json`](divider-samples.json). Screenshot: [`divider-section.png`](divider-section.png).

| Check | Result |
| --- | --- |
| Count on the page | 4 (3 horizontal, 1 vertical) |
| Thickness | 1px horizontal; 1px vertical |
| Colour | rgb(202, 196, 207), outline-variant |
| Full-width divider | 504px in the 504px container; 8px container padding on each side |
| Inset start | Left edge 24px (8px padding + 16px inset) |
| Inset middle | 24px on each side (8px padding + 16px inset) |
| Vertical divider | Stretched to 88px, the row height |
| Roles | `separator` on all four; `aria-orientation` matches the orientation |
| Page errors | 0 |

## 4. Diagnosed defects

None recorded in this round.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Preview served from a fresh build | Passed |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's horizontal card, which compiles |

## 6. Unresolved differences and untested behaviour

- The 16dp inset is a choice, not a token value. The Material list specs were not read (blocked host).
- Dividers are static; there is no interaction or motion to test.
- Touch, Android and reduced motion are not applicable or not measured.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script against the divider section, reading each `.m3-divider`'s computed
   colour, height and width, and its position relative to its container.
3. Compare the output with [`divider-samples.json`](divider-samples.json).
