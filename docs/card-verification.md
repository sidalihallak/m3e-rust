# Card — current fidelity update

## Current desktop verification — 2026-10-10

Browser access is restored. This section supersedes the blocked source-only audit
and older 24px padding measurements below. Scope: `card.rs`, `card.css`, and the
card gallery in `main.rs`/`main.css`; macOS Codex in-app browser, pointer/keyboard.

Freshly read all four rendered official sections: [overview](https://m3.material.io/components/cards/overview),
[specs](https://m3.material.io/components/cards/specs), [guidelines](https://m3.material.io/components/cards/guidelines),
[accessibility](https://m3.material.io/components/cards/accessibility).
The specs show 12dp corners, 16dp left/right content padding, start-aligned text,
and an 8dp maximum gap between cards. Corrected CardBody/CardActions from 24px
to **16px**, and the showcase's stacked-card gap from 18px to **8px**. Structured
cards keep root padding 0; body slots own the inset. Media remains edge-to-edge.
The pinned upstream/token references in the audit below remain the source contract.
Upstream also offers 12/16/24px size choices; this kit defaults to the official 16px inset.

| Check | Actual result |
| --- | --- |
| Variants | All seven examples: 12px corners; filled/elevated border 0; outlined border 1px outline-variant; semantic filled/highest, elevated/low, outlined/surface roles |
| Slot/media geometry | CardBody 16px, root 0; media 360×202.5 (16:9), inset 0; headline inset 16px, action padding 16px and gap 8px |
| Pointer ripple | 28 computed-style frames: opacity 0→.0314→.0636→.0874→.10, expanding wave; card stayed 451.5×100px. Pressed elevation 1, released hover elevation 2 |
| Keyboard | Enter count 1→2, Space 2→3; outlined focus layer .10, secondary 3px outline/2px offset, on-surface border, resting elevation |
| Repeated/exit input | Double click 3→5; drag released inside 5→6, released outside stayed 6 |
| Disabled | Native disabled attributes present; outlined click did not increment or focus. Outlined border outline at .12, content .38, opaque surface; filled root .38 |
| Passive actions | Native div container, no nested actionable elements in interactive buttons; Tab visits two elevated-card actions then filled-card Buy tickets, skipping disabled cards |
| Reduced motion | Actual macOS preference enabled with prior approval, media query true; first visible ripple already final scale/opacity .10, activation retained; preference restored OFF |
| Dark mode | Roles switch to highest rgb(54,52,58), low rgb(29,27,32), surface rgb(20,18,24); divider/outline-variant rgb(73,69,78); disabled treatments retained. Light mode restored |

Shared ripple reference values remain 225ms minimum press, 450ms growth,
105ms fade-in, 375ms fade-out; the actual frames show progressive feedback and
stable dimensions, but do not establish exact timing parity. The kit's 200ms
state/elevation transition still differs from upstream FastEffects spring.
[Raw desktop frames](resumed-browser-verification.json),
[variant screenshot](cards-verified-preview.png),
[media/action screenshot](card-media-verified-preview.png).
Final live build and exported-example checks are in [the composite index](composite-verification.md).

Gaps: the native drag action was too short to establish extended held-pointer
behavior; release outside is not a touch pointer-cancel test. Android stays paused.
Audible screen readers, forced colors, text enlargement, custom-seed state contrast,
selectable/dragged cards and the complete upstream CardHeader/size API remain open.
No full platform/pixel parity claim is made. Text/image content and image alt text
remain consumer responsibilities; do not nest buttons/links in an interactive card.

Reproduce: after successful `./scripts/dev.sh`, reload Cards; inspect 12/16/8px
geometry, Tab/Enter/Space, disabled outlined click, inside/outside release and
passive action order. Sample computed styles during real pointer input. Toggle
preview dark mode and restore it. OS preferences require user authorization.

## Historical source-only audit and earlier runtime evidence

## Card source audit and follow-up — 2026-10-10

Scope: filled, elevated, outlined, passive/interactive, disabled, text/media,
CardBody/CardActions and focus/press styles. Re-read pinned upstream
[card.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/card.tsx)
and [state-layer rules](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css),
plus all three [official card token files](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)
and `_md-sys-state.scss` at `47adb655bd7a88c4d62e8faac2873084eed555dc`
(version 34.0.21). All eight official card/divider overview/specs/guideline/
accessibility URLs were requested, but returned JavaScript-only shells. Earlier
rendered readings below remain the guideline evidence; no fresh browser reading
or runtime certification is claimed.

| Contract from source/tokens | Audit result and correction |
| --- | --- |
| 12px corners and three semantic surface roles | Retained |
| Filled/elevated have no outline; outlined has 1px outline-variant | Removed the 1px transparent border from all filled/elevated cards, rather than only media cards |
| Slot content should not accumulate parent padding | CardBody now owns its 24px inset; root padding becomes 0 when that slot is present, fixing the previous 16+24=40px inset |
| Hover elevation filled/outlined 1, elevated 2; press returns to rest | Retained; quick clicks now retain pressed elevation for the existing Ripple press lifecycle |
| Hover belongs to devices that support hovering | Wrapped hover styling in the same hover-capability media query used upstream; touch behavior remains untested |
| Keyboard focus layer .10 and resting elevation | Added missing focus layer/rest elevation; outlined focus border uses on-surface as its official token specifies |
| Pressed feedback via ripple when a Ripple is present | Removed the additional static .10 pressed overlay, matching upstream's explicit exclusion of ripple hosts |
| Outlined disabled border: outline at .12 | Corrected former whole-card .38 fade: border now .12, content independently .38, surface remains opaque |
| Filled/elevated disabled container roles | Filled uses surface-variant; elevated uses surface, with existing .38 fade and resting elevation |
| Direction-aware text | Changed left alignment to logical start |
| Native activation and semantics | Native button, disabled attribute, Enter/Space and Ripple code retained; passive cards permit separate actions |
| Interactive target minimum | Added a 48px minimum height for unusually short content; ordinary card heights remain content-driven |

The filled gallery example now exercises CardBody without media, and an outlined
disabled example displays the corrected border/content treatment. Internal action
controls belong in passive cards; do not nest buttons/links inside an interactive
Card button.

### Reference choices and remaining differences

- The kit's structured body/action layout uses 24px padding, headline-small 24/32
  and a 16:9 media frame from the previously inspected Material composition.
  Upstream also offers 12/16/24px card sizes and uses title-large 22/28. These are
  layout/type choices, not universal mandatory values for every Material card;
  upstream's complete CardHeader/Action/size API is not ported here.
- The existing 200ms elevation/state transition is a kit choice; upstream uses
  its FastEffects spring for state layers. This follow-up does not retune motion.
- Drag/dragged states and selectable-card composition remain unimplemented.
- Browser inspection is blocked by the existing URL security policy. Post-fix
  geometry, hover/focus precedence, quick/held/repeated/cancelled input, disabled
  rendering, media corners and reduced-motion behavior are **pending runtime
  verification**. Earlier samples below predate these corrections. A screenshot
  was requested; none is inferred from source declarations.
- Android is paused; audible screen-reader checks, forced colors, text enlargement,
  custom-seed state contrast and native packaging remain unverified.

Reproduce after a successful preview build: reload Cards, compare root/slot insets
with and without media, verify the disabled outlined border, use Tab/Enter/Space
and pointer presses to sample state/elevation, check light/dark and RTL, and confirm
that passive card actions remain separate focus stops. Record actual samples
before marking this follow-up verified.

### Build and copy checks

Locked offline Wasm check passed in **3.89s**. Fresh `./scripts/dev.sh` build
succeeded in **6.55s**; the subsequent hover media-query stylesheet hot reload
does not change Rust behavior. The separately exported Card and updated Divider
examples compile for Wasm in **0.88s**. `git diff --check` passes. The preview is
left running. [Source-audit notes](card-divider-source-audit.json) retain reference
hashes and distinguish expected values from runtime measurements.

Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/cards/overview)
- [Material specs](https://m3.material.io/components/cards/specs)
- [Material guidelines](https://m3.material.io/components/cards/guidelines)
- [Material accessibility](https://m3.material.io/components/cards/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/card.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R4; interactive card now has expanding ripple while retaining native button semantics, elevation and media/body/action slots. Enter increments the elevated example and reaches ripple .10 at unchanged 337.5×102px. Drag state remains outside implemented scope.

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

## 3e. Visible corners and the elevated margin (follow-up)

Two things were reported as still wrong: the image corners did not look rounded, and the
elevated image card looked padded.

- **Corners.** Pixel sampling at 3x (`card-E-3x.png`) shows the bottom-left corner of the image
  curving to the card background, so the 12px radius is applied. The corners were hard to see
  because the demo card was 520px wide. The Material reference card is about 360dp wide, so the
  12dp radius reads smaller there. The image cards are now shown at 360px wide, and the corners
  read at the reference's proportion: `docs/card-media-elevated-360.png`.
- **Elevated margin.** Inside the card the image is flush (0px on the left and top edges, as
  measured in 3c). The thin light band above the image is the elevated card's level-1 shadow,
  drawn outside the card edge, together with a half-pixel offset from the card's position.
  It is not padding. If the band should not show, the media example can use a card without
  elevation. This is not changed.

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
- Media, body and action slots and headline/subhead/supporting classes are implemented; current ripple behavior is recorded above.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script against the card section with the Chromium at
   `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`. It reads computed styles, holds the pointer
   down, presses Enter and Space on the focused outlined card, and clicks the disabled card.
3. Compare the output with [`card-samples.json`](card-samples.json).
