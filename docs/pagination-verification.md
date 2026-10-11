# Pagination verification — 2026-10-11

## Component scope

Controlled one-based `Pagination` and `TablePagination`. Files:
`src/components/pagination.rs`, `assets/pagination.css`; native table-page-size
selector uses existing Select primitives/styles.

## References and scope

Retrieved 2026-10-11. The rendered [current Material 3 catalog](https://m3.material.io/components)
has no dedicated accordion, collapsible, table, pagination or data-table
component pages. There are consequently no current four-page M3 contracts to
claim for this family. These are library compositions using Material color,
typography, shape and motion roles. Table guidance below is explicitly
[archived Material 2](https://m2.material.io/components/data-tables), not a new M3 spec.

Upstream revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Reference contract and implementation

[Actual upstream pagination](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/pagination.tsx).

Upstream uses nav/list structure, outlined active link, ghost inactive links,
4px gap, decorative ellipsis and previous/next links. This port's controlled
local pagination uses native **buttons**, not fake hrefs. For URL navigation,
consumers must compose real links; a link pagination primitive is a remaining
extension. Previous/next are compact arrows with full accessible names at all
widths, rather than upstream's desktop visible text labels.

48px targets, finite 24px resting corners/8px press corners and DefaultEffects
240ms follow the kit's accepted control contract. Native button semantics,
`aria-current=page` and disabled bounds are preserved; ellipses are not focusable.
First/last and adjacent page numbers are shown; the sequence wraps at compact
width rather than shrinking targets. Zero page count normalizes to one.

TablePagination follows archived official table pagination: page-size choice,
visible row range/count and previous/next arrows below the table. NativeSelect
supplies correct Material field geometry; its opened picker belongs to the OS.

## Runtime verification

First/middle/last page cases pass with exactly one current page. Enter on page4
preserves focus on Page4. Page20 disables Next; Previous Space moves to19.
Disabled pagination exposes disabled native controls. Table page size25 plus
Next shows26–50 of137. DataTable page size/filter behavior is separately checked.
Measured page targets48×48 (final arrow sizing also48). Click press morph shows
24→8.04→24px without width/height changes; pressed state sampled50–252ms,
return completes520ms. Source token240ms plus225ms visible press is retained.
At390px, numbered pagination wraps to two rows, all targets48px, no page overflow.
Both copy snippets exactly match. URL links, broader RTL, very large page counts,
held/repeated pointer and page-size OS popup behavior remain unverified.

## Build and evidence

Dioxus web preview built successfully (13.74s, 14.39s, 14.53s and final
13.68s). Reloaded after the successful build; one `.app-shell` mounted.
The independently exported Rust/Wasm library and all 66 real usage examples
pass `cargo check --locked --offline --target wasm32-unknown-unknown`
with the existing `wasm-dev` profile/cache. The pre-existing ChipSize import
warning in the chip example is unrelated.

Raw browser evidence: [data-family-samples.json](data-family-samples.json).
Desktop screenshots and responsive evidence: [evidence/data](evidence/data).
390×844 is desktop responsive emulation, not Android touch verification.
Android remains paused. OS Reduce Motion was temporarily enabled with the
user's existing authorization and restored off. Audible screen readers,
forced colors, RTL, text zoom and all custom seed contrasts remain untested
for these new components. At 320px the existing showcase body's 360px
minimum causes page overflow; 390px has no page overflow. Do not call the
320px showcase responsive behavior verified.

## Reproduce

Open `/#pagination`. Use Enter/Space, page20 and Previous. Change table footer
size to25 and Next. Inspect current/disabled attributes and sample press corners.
Screenshot: `evidence/data/pagination-phone.png`.
