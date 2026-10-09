# Card verification

Scope: `Card` and `CardVariant` in [`src/components/card.rs`](../src/components/card.rs) and
[`assets/card.css`](../assets/card.css). Verified in the Dioxus preview on 2026-10-09 with
headless Chromium 1194, using real pointer and keyboard input, at device scale 2. Desktop only.

## 1. References

- Material Web card tokens, revision `47adb65`: elevated
  ([`_md-comp-elevated-card.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-elevated-card.scss)),
  filled (`_md-comp-filled-card.scss`) and outlined (`_md-comp-outlined-card.scss`). Used:
  12dp corner, surface-container-low / surface-container-highest / surface containers,
  1dp outline-variant (outlined), elevation levels at rest, hover and pressed, state layers
  0.08 hover and 0.10 pressed, 3dp secondary focus indicator, 38% disabled container opacity.
- Material Web elevation mixins, revision `47adb65`
  ([`internal/_elevation.scss`](https://github.com/material-components/material-web/blob/47adb65/elevation/internal/_elevation.scss)):
  the key shadow (0.3 opacity) and ambient shadow (0.15 opacity) offsets and blurs for each level.
- shadcn-m3e `card.tsx`, revision `8f1b3fb`: the three variants and their hover and press
  elevation (`elevated`: low container with shadow-elevation-1, hover level 2, press level 1).
- **Not read:** the official `m3.material.io` card page. The egress proxy blocks that host.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Corner | 12px | Tokens (corner-medium) |
| Filled | surface-container-highest, level 0 at rest, level 1 on hover | Tokens |
| Elevated | surface-container-low, level 1 at rest, level 2 on hover, level 1 pressed | Tokens and upstream |
| Outlined | surface, 1dp outline-variant, level 0 at rest, level 1 on hover (outline unchanged) | Tokens |
| State layer (interactive) | Hover 0.08; pressed and focus 0.10; on-surface | Tokens |
| Focus | 3px secondary outline, 2px outside the card | Tokens (outer offset) |
| Disabled | 38% opacity for the whole card; no state layer, no focus | Tokens |
| Interactive semantics | A native `<button>`; Enter and Space activate it | Native |
| Non-interactive | A `<div>` with any content | — |
| Motion | Box-shadow and background transitions, 200ms standard easing | Choice; the token file gives no duration |

## 3. Input, state and platform matrix

Raw data: [`card-samples.json`](card-samples.json). Screenshot: [`card-section.png`](card-section.png).

| Check | Result |
| --- | --- |
| Corner radius, all variants | 12px |
| Filled rest background | rgb(230, 224, 233) (surface-container-highest); no shadow |
| Elevated rest shadow | Key 0 1px 2px 0.3, ambient 0 1px 3px 1px 0.15 (level 1) |
| Elevated hover | Level 2: 0 1px 2px 0.3, 0 2px 6px 2px 0.15; state layer 0.08 |
| Elevated pressed (mouse held 350ms) | Level 1 shadow; state layer 0.10 |
| Elevated after release on the card | Level 2 again (hover); the click counter increments |
| Outlined rest | Background rgb(253, 247, 255) (surface); border 1px rgb(202, 196, 207) (outline-variant) |
| Outlined hover | Level 1 shadow; state layer 0.08 |
| Keyboard Enter on the focused outlined card | Counter increments |
| Keyboard Space on the focused outlined card | Counter increments |
| Focus-visible outline | solid 3px |
| Disabled card: opacity | 0.38 |
| Disabled card: pointer click | Counter unchanged; `disabled` is set |
| Non-interactive card on hover | No shadow, no state layer |
| Page errors | 0 |

## 4. Diagnosed defects

None recorded in this round.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Preview served from a fresh build | Passed |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's cards, which compile |

## 6. Unresolved differences and untested behaviour

- The shadows use the Material Web mixin values for levels 0 to 2. They were measured as computed
  styles, not compared with a Material screenshot. The official card page was not read.
- The transition duration (200ms) is a choice. The token file does not define it.
- Touch, Android, the on-screen keyboard and reduced motion are not measured. The reduced-motion rule
  is in place and is not verified.
- Screen-reader announcements of interactive cards are not measured.
- The Material card's drag and dragged elevation states are not implemented.
- The Material card's media and headline slots are not implemented; the content is free-form.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script against the card section with the Chromium at
   `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`. It reads computed styles, holds the pointer
   down, presses Enter and Space on the focused outlined card, and clicks the disabled card.
3. Compare the output with [`card-samples.json`](card-samples.json).
