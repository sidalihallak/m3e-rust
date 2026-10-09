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
