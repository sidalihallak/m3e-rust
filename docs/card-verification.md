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
| Media | Full-width 16:9 image at the top, 12px top corners; card padding removed | Material card reference layout |
| Actions | Row at the bottom, end-aligned by default, `start` option | Material card reference layout |
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

- **Image cards did not match the Material layout.** The first media card had a single title
  and text under the image, with no actions and smaller text. It now uses the reference anatomy
  (section 3b).
- **Title and text ran together in image cards.** `CardBody` is a block, but the title and
  text were inline spans, so "Media card" and its supporting text sat on one line. Both are now
  blocks. Measured: the text starts 4px below the title, as the margin sets.

## 3d. Image corners

The Material reference with a headline, subhead and supporting text (fourth reference) rounds
all four corners of the image, not only the top two. The image now has a 12px radius on each
corner. Measured (`card-media-corners.json`): 12px on all four corners, flush with the card's
top, left and right edges, and the text body starts directly below the image, with its own 24px inset.

Action alignment: the references differ. One shows the actions at the end of the row, and the
fourth shows them at the start, on the filled and outlined cards. Both are supported
(`CardActions`, with `start: true` for start alignment). The default is end, as before; this
has not been changed for the references that show start alignment.

## 3c. Image flush to the card edge

The reference images show the image flush with the card's top and sides, with no padding, and
with its top corners rounded. The first version had a 1px gap: the card's 1px transparent
border pushed the image in from the top and sides. Filled and elevated cards with media now drop
the border, and the card clips the image to its 12px corners.

Measured (`card-flush.json`): media inset 0px left, top and right for elevated and filled cards;
card and image top radius both 12px; card `overflow: hidden`.

Outlined cards keep their 1dp outline, so an outlined card with media has the image inside the
outline. The reference shows no outlined media card, so this is not verified against it.

Raw data: [`card-flush.json`](card-flush.json); screenshot: [`card-flush-section.png`](card-flush-section.png).

## 3b. Card anatomy (Material layout, revised)

The earlier media example put a title and one line of text under the image, and the Material
reference does not. The reference layout is: media at the top, then a headline, a subhead,
supporting text, and an action row at the bottom right (or a filled action at the start, as in
the "Glass Souls' World Tour" reference). The card now follows that anatomy.

| Element | Value | Measured |
| --- | --- | --- |
| Text inset | 24dp from the card edge | 25px (1px border + 24px) |
| Headline | 24/32, weight 400 (headline-small) | 24px / 32px / 400 |
| Subhead | 16/24, weight 500 (title-medium), 4px above it | 16px / 24px / 500 |
| Supporting text | 14/20, weight 400 (body-medium), 16px above it | 14px / 20px / 400 |
| Action row | 24px inset on all sides, buttons aligned to the end, 8dp gap | Right inset 25px; 8px gap; row height 88px (24 + 40 + 24) |
| Start-aligned actions | Buttons aligned to the start | Left inset 25px |
| Media | Flush with the card's top, left and right edges; 16:9; 12px top corners, clipped by the card | Insets 0px / 0px / 0px; top radius 12px = card radius |

The 4px subhead margin and the 16px supporting margin are read from the reference screenshot
(distances between the text lines). They are not token values.

Raw data: [`card-anatomy-samples.json`](card-anatomy-samples.json); screenshot:
[`card-anatomy-section.png`](card-anatomy-section.png).

## 3a. Cards with media

Added: `CardMedia` (an image at the top, edge to edge, 16:9, 12px top corners) and `CardBody`
(16px text inset). A card that starts with `CardMedia` drops its padding.

| Check | Result |
| --- | --- |
| Card padding with media | 0px |
| Image width / card width (inner, excluding 1px border) | 518px / 520px |
| Image aspect ratio | 1.7778 (16:9) |
| Image top corners / bottom corners | 12px / 0px |
| Text inset from card edge (1px border + 16px padding) | 17px |
| Title and text order | Stacked; text 4px below title |
| Interactive media card: tap | Counter increments (0 → 1) |

Raw data: [`card-media-samples.json`](card-media-samples.json); screenshot:
[`card-media-section.png`](card-media-section.png). The image is the bundled
`assets/card-image.svg`, a generated illustration, not a photo.

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
