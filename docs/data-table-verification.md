# Data Table verification — 2026-10-11

## Component scope

Reusable client-side `DataTable`, `DataColumn`, `DataRow`, `DataCell` on the
Table/Pagination/TextField/NativeSelect/Checkbox primitives. Files:
`src/components/data_table.rs`, `assets/data-table.css`.

## References and scope

Retrieved 2026-10-11. The rendered [current Material 3 catalog](https://m3.material.io/components)
has no dedicated accordion, collapsible, table, pagination or data-table
component pages. There are consequently no current four-page M3 contracts to
claim for this family. These are library compositions using Material color,
typography, shape and motion roles. Table guidance below is explicitly
[archived Material 2](https://m2.material.io/components/data-tables), not a new M3 spec.

Upstream revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Reference contract and implementation

[Upstream component inventory](https://github.com/Crysta1221/shadcn-m3e/tree/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components)
has no standalone data-table module. This is a Rust composition of its table and
pagination styling, with behavior inspired by the [shadcn data-table composition](https://ui.shadcn.com/docs/components/base/data-table),
not a TanStack API port. [WAI sortable table](https://www.w3.org/WAI/ARIA/apg/patterns/table/examples/sortable-table/)
guides native headings/buttons and aria-sort.

- Stable unique row IDs and column keys required; cells follow column order.
- Controlled row selection; local query, sort, visibility and page/page size.
- Text filtering across all columns; numeric sort keys independent of display
  strings; ascending→descending→original cycle, stable ties.
- Optional custom cell Element retains plain searchable text and sort key.
- Select-page checkbox has mixed state and excludes disabled rows, preserving
  selections on other pages. Clear selection clears all current IDs.
- Column disclosure retains at least one visible column and resets hidden sort.
  It uses native details semantics, not a menu with an implied dismissal contract.
- Native table counts/indices retain the original column position and full
  filtered row count across pagination; one active aria-sort heading.
- Loading disables manipulation, sets aria-busy and displays status; error uses
  alert; empty result displays status and 0–0 range.
- 56/52px geometry, 48px checkbox/sort targets, correct selected color roles.
  Outer16px border radius is the kit composition choice; not an M3 table mandate.
- Shared TextField/NativeSelect supply filter/page-size fields; no custom fake
  date/number controls or new shared Button behavior.

## Runtime verification

| Behavior | Observed result |
| --- | --- |
| Numeric ascending | 0,80,250,300,450 (formatted dollars), not lexical order |
| Numeric descending | 10000,9000,2200,1500,1200 |
| Reset | Returns source order; aria-sort removed |
| Selection | Atlas→mixed header; select page→5; select Field notes on next page→6 preserved |
| Disabled row | Legacy archive checkbox disabled |
| Filter | Complete finds3 rows; no-match shows status and0–0; reset returns page1 |
| Page size |10 renders10 rows, 1–10of12 |
| Columns | Hidden Status removes cells; index4 Budget retained; last visible column disabled |
| Hidden sort | Hiding Budget resets sort/source order |
| Loading/error | Busy/filter/sort disabled; Loading rows status; error alert observed |
| Colors | Light selected232,222,248/text74,68,88; dark swaps matching semantic roles |
| Geometry |56px header,52px row,48px checkbox,56px filter;16px composition corner |
| Color motion | Deselection interpolates selected fill to8% hover through150ms FastEffects |
| Copy | Exact clipboard match |

Server-side/manual state APIs, column resizing/reordering/pinning, multi-sort,
virtualization, editing, CSV export, range selection and drag/drop are not
implemented. Live row/column replacement, numeric missing keys/custom cell
widgets, every disabled/loading combination and audible announcements are not
independently verified. This is a working local data table, not a claim of a
complete enterprise data grid or exact upstream data-table parity.

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

Open `/#data-tables`. Enter/Space on Budget cycles sorting. Select Atlas then
select-page, Next, Field notes and Next; inspect selection count and disabled
archive. Filter Complete/no-match; change page size. Open Columns and hide/show
columns including the sorted column. Toggle Loading/Error. Repeat at390px and
in dark mode. Screenshots: `data-table-desktop.png`, `data-table-dark.png`,
`data-table-phone.png` under `evidence/data`.
