# Badge alignment correction — 2026-10-09

## Scope and sources

Corrected NotificationBadge / BadgeAnchor in `src/components/badge.rs` and
`assets/badge.css`, with LTR, RTL and standalone gallery examples. Read the
current official [overview](https://m3.material.io/components/badges/overview),
[specs](https://m3.material.io/components/badges/specs),
[guidelines](https://m3.material.io/components/badges/guidelines), and
[accessibility](https://m3.material.io/components/badges/accessibility) in the
browser on this date, including the loaded Measurements diagram. Inspected
[actual upstream badge.tsx at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/badge.tsx).
Upstream defines size/colors/text and leaves positioning to its consumer.

## Cause and corrected contract

The previous percentage translation centered each count pill on the icon
corner: as the text widened, its leading edge moved left. It also raised
counts 12px above the icon. Those offsets were arbitrary. Current official
specs place badges inside the upper trailing part of the icon bounds, with
fixed offsets to the badge's bottom-leading corner.

| Property | Current value / measured result |
| --- | --- |
| Icon anchor | 24×24px in these examples |
| Small dot | 6×6px; x=18, y=0 relative to LTR anchor |
| Small bottom-leading offset from icon top-trailing | 6px inward, 6px down |
| Large count | 16px high; x=12, y=−2 for **all** 3/99/999+ examples |
| Large bottom-leading offset | 12px inward, 14px down |
| Count widths | 16 / 22.859 / 38.133px with current Inter typography |
| RTL | Dot x=0; count's right edge remains at x=12; growth extends left |
| Numeric text in RTL | `bdi dir=ltr` isolates 999+ without changing badge placement |
| Standalone badges | Relative/in-flow, separated; no absolute overlap |
| Semantics | Dot img/New notification; counts status with their count label |
| Colors / motion | error/on-error; no badge animation added |

The declared fixed anchor replaces the earlier choice below. Count width
adapts to the existing kit font; the official maximum-count illustration is
34px wide, whereas Inter renders 999+ at 38.133px. This fix establishes
placement rather than claiming font/pixel parity.

## Verification and build

- Actual desktop DOM measurements before/after, default and dark schemes;
  one-digit, two-digit and capped counts; RTL mirroring; standalone flow.
- Dioxus reported a successful 9.44s live build; preview reloaded and current
  gallery confirmed. Locked offline Wasm check passed.
- Screenshot: [badge alignment](badge-alignment.png).
- Raw data: [badge/segment alignment samples](badge-segmented-alignment-samples.json).
- Copy sources/CSS remain listed in [copy-components.md](copy-components.md).
- Not tested: Android/touch, screen-reader announcements, navigation-host
  dismissal behavior. OS reduced-motion true was not exercised; this component
  has no animation. Android remains paused.

## Reproduce

Run ./scripts/dev.sh, confirm build success and reload. In Badges, compare
3, 99 and 999+: their leading badge edges must share the same fixed position
relative to their icons. The RTL card mirrors placement and preserves 999+
text order. Inline badges must occupy separate flow positions. Repeat in dark
mode and compare the raw measurements.

## Historical report before this correction

# Notification badge verification

Scope: `NotificationBadge` and `BadgeAnchor` in
[`src/components/badge.rs`](../src/components/badge.rs) and
[`assets/badge.css`](../assets/badge.css). Verified in the Dioxus preview on 2026-10-09 with
headless Chromium 1194 at device scale 2. Desktop only.

Scope note: this is the notification badge (a dot or a count). The compact label badge in the
shadcn-m3e source is not implemented in this round.

## 1. References

- Material Web badge tokens, revision `47adb65`
  ([`_md-comp-badge.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-badge.scss)):
  6dp dot, 16dp large badge (min width 16dp), error and on-error colours, full corner shape,
  label-small text for counts.
- shadcn-m3e `badge.tsx`, revision `8f1b3fb`: the `NotificationBadge` with the same sizes, the
  pill with 4dp horizontal padding, and the `count` / `max` (999+) behaviour.
- **Not read:** the official `m3.material.io` badge page. The egress proxy blocks that host.
  The token file and upstream source are the sources used.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Dot | 6×6px, error background | Tokens |
| Count badge | 16px high, min width 16px, 4px horizontal padding, grows with the text | Tokens and upstream |
| Count text | label-small, 11/16, weight 500, 0.5px tracking, on-error | Tokens |
| Shape | Full (999px radius) | Tokens |
| Cap | Counts above `max` (default 999) show as `999+` | Upstream |
| Placement | Top-right of a `BadgeAnchor`: the dot is centred on the corner; a count's right edge sits on the corner and its bottom edge reaches 4px into the icon | Choice, not in the tokens |
| Accessibility | Count: `role="status"` with an aria-label such as "3 notifications". Dot: `role="img"` labelled "New notification" | ARIA |
| Motion | None in the spec | Tokens |

## 3. Input, state and platform matrix

Raw data: [`badge-samples.json`](badge-samples.json). Screenshot:
[`badge-section.png`](badge-section.png).

| Check | Result |
| --- | --- |
| Dot size | 6×6px, background rgb(186, 26, 26) (error), text colour white (on-error) |
| Count "3" | 16×16px, label-small 11px/16px weight 500 |
| Count "99" | 22.9px wide, 16px high |
| Count "1000" with default max | Shows "999+", 38.1px wide, aria-label "999+ notifications" |
| Count role | `status` (live region) |
| Dot role | `img` with aria-label "New notification" |
| Placement, dot | Centred on the icon's top-right corner (3px right, 3px above the icon box) |
| Placement, count | Raised 12px above the icon top; right edge 8px / 11px / 19px beyond the icon (for 3 / 99 / 999+). Icon area covered: 5.6% / 7.9% / 13.2% |
| Page errors | 0 |

## 4. Diagnosed defects

- **999+ covered the icon.** The first placement put the badge's right edge 8px past the icon and
  its top 6px above the icon's top. The 38px pill then covered most of the 24px icon, and the
  "+" was hidden. Measured coverage was 40%. The count is now raised 12px above the icon, with
  its right edge on the corner. Coverage is 13.2% for 999+, and the icon is fully visible.
  Raw data: [`badge-cover.json`](badge-cover.json); screenshot: [`badge-section.png`](badge-section.png).

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Preview served from a fresh build | Passed |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's count card, which compiles |

## 6. Unresolved differences and untested behaviour

- The placement is a choice. The token file does not give the offset of the badge from its
  anchor. A 999+ pill is wider than a 24px icon, so some overlap with the icon is unavoidable. It
  is kept to the pill's bottom 4px. Check this against the Material badge spec image when it is available.
- A count of zero is shown as "0". The spec does not define hiding the badge at zero; the consumer
  should omit the badge instead.
- The compact label badge from shadcn-m3e is not implemented.
- Touch, Android and reduced motion are not measured. The spec has no motion.
- Screen-reader announcements of count changes are not measured.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script against the badge section, reading computed sizes, colours, fonts and
   positions from the `.m3-badge` elements inside `.m3-badge-anchor`.
3. Compare the output with [`badge-samples.json`](badge-samples.json).
