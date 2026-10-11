# Table verification — 2026-10-11

## Component scope

Native composable Table, Header, Body, Footer, Row, Head, Cell and Caption,
plus `TableSort` and optional row/column indices/counts. Files:
`src/components/table.rs`, `assets/table.css`.

## References and scope

Retrieved 2026-10-11. The rendered [current Material 3 catalog](https://m3.material.io/components)
has no dedicated accordion, collapsible, table, pagination or data-table
component pages. There are consequently no current four-page M3 contracts to
claim for this family. These are library compositions using Material color,
typography, shape and motion roles. Table guidance below is explicitly
[archived Material 2](https://m2.material.io/components/data-tables), not a new M3 spec.

Upstream revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Reference contract and implementation

[Actual upstream table](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/table.tsx),
[WAI table pattern](https://www.w3.org/WAI/ARIA/apg/patterns/table/).

Matches upstream 56px column headers, 52px data rows, 16px horizontal cell
padding, 14/20 body, title-small headers, outline-variant dividers,
surface-container-low footer, bottom caption and horizontally scrollable width.
Native row headings are 52px instead of accidentally inheriting header height.
Numbers align to the logical end with tabular numerals. Selected rows use
secondary-container/on-secondary-container. Row hover uses on-surface at 8%.
Row color transition uses kit FastEffects 150ms; this is a kit motion choice,
not a separately mandated M3 table timing. Consumer container shape is unstyled.
Native table semantics are retained; interactive cells remain separate Tab stops.
The scroll region is named/focusable for keyboard horizontal scrolling.

## Runtime verification

Measured invoice header 56px, all three body rows 52px, 16px padding,
logical-end number alignment, bottom caption and `scope=row` invoice headings.
At 390px the invoice scroll viewport is 295px around a 385px table; data-table
scroll viewport is 325px around 382px. Actual ArrowRight scroll changes scrollLeft
(the early sample records 0→1.5px during native scrolling). Page width stays
within the client width. Table code copy exactly matches. Selected color role
and transition evidence is exercised by DataTable below. Footers are visible.
Nested widget/table screen-reader navigation remains an audible verification gap.

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

Open `/#tables`; inspect the caption, totals and headings. At 390px focus the
named table scroll region and press ArrowRight/Left; compare scrollLeft. Copy
Table usage. See `tableGeometry` and `tableKeyboardScroll` in the raw samples.
