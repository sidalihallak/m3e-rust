# Multi-select removal and filled Select alignment — 2026-10-10

## Scope and references

Follow-up corrections in `select.rs`, `combobox.rs` and `assets/select.css`.
Official Material is the authority for these layout and interaction choices.

- Text fields: [overview](https://m3.material.io/components/text-fields/overview), [specs](https://m3.material.io/components/text-fields/specs), [guidelines](https://m3.material.io/components/text-fields/guidelines), [accessibility](https://m3.material.io/components/text-fields/accessibility).
- Chips: [overview](https://m3.material.io/components/chips/overview), [specs](https://m3.material.io/components/chips/specs), [guidelines](https://m3.material.io/components/chips/guidelines), [accessibility](https://m3.material.io/components/chips/accessibility).
- Actual upstream [Select](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/select.tsx) and [Combobox](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/combobox.tsx), revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

These official pages were read rendered on 2026-10-10 across this follow-up and
the preceding field audit. Chips were newly inspected, and the text-field specs
and accessibility guidance were revisited. Relevant actual upstream source was
read again.

## Contract and diagnosis

Material input chips represent entered values. Each removable chip exposes its
own remove icon. Material does **not** require a bulk clear button beside those
chips, nor explicitly prohibit such an application action. Our design choice
is per-chip removal with a persistent dropdown affordance; single-value fields
retain their clear behavior. `show_clear` consequently applies to single-value
Combobox. No new public API or usage snippet is required.

The previous correction applied the upstream single-input clear/trigger
replacement rule to multi-selection too broadly. Upstream separately exposes
ComboboxChips/ComboboxChip with per-value removal. It should not be treated as
an official requirement for a clear-all action on the chip field.

Material text-field specs require vertically centered icons. Before this fix,
Billing region's arrow center was at **36px** within its **56px** field, 8px
below center. Filled text padding (24px top / 8px bottom) moved the arrow along
with the value. The upstream filled trigger uses this same asymmetric flex
padding; the port now follows Material's centering contract instead.

The decorative Select arrow is positioned independently at half the field
height. Default glyph: 24px, 12px logical end inset, 52px reserved trailing space.
The existing opt-in 40px compact field keeps a 20px glyph centered at 20px.
The complete native trigger remains clickable. Motion tokens are unchanged.

## Actual desktop checks

| Input/state | Result |
| --- | --- |
| Default outlined, filled, disabled, error and RTL Select | Arrow 24×24px, center at 28px in 56px field; logical end inset 12px |
| Compact Select | Arrow 20×20px, center at 20px in 40px field |
| Pointer open filled Select, Escape close | Arrow rotated 180° and back; resting and open center both 28px |
| Concurrent closing style samples | Rotation interpolation and spring overshoot observed, then settled to none; no motion-token changes |
| Filled Down, End, Enter | Africa committed; popup closed and arrow stayed centered |
| Populated Project languages | One dropdown; zero clear-all buttons; two named chip remove buttons with 48×48px targets |
| Pointer Remove Rust | Only Rust removed, TypeScript retained |
| Space on Remove TypeScript | Remaining chip removed; dropdown retained |
| Pointer choose Rust and TypeScript again | Both restored; no clear-all icon appeared |
| Desktop viewport 390×844 | Chips wrapped to 112px field height; both remove targets remained 48×48; Select arrows retained their centers |

Final live Dioxus Wasm build succeeded in **17.85s**, followed by a reload and
one `.app-shell`. `cargo check --locked --offline --target wasm32-unknown-unknown`
succeeded in **18.09s**. The preview remains running.

## Evidence, gaps and reproduction

[Raw source readings, geometry and motion samples](multi-select-filled-alignment-samples.json),
[desktop screenshot](multi-select-filled-alignment-desktop.png),
[compact desktop screenshot](multi-select-filled-alignment-compact.png).
Sample clocks include browser/input round-trip latency; they do not establish
exact event-relative animation duration. Opening sampling captured endpoints;
closing captured intermediate rotation matrices.

These checks establish the two requested corrections, not complete chip parity.
Existing input-chip editing/reordering, whole-chip remove-only focus treatment,
focused-chip Backspace/Delete and chip-list navigation gaps remain. The current
remove buttons support native Enter/Space, which was checked here. No claim is
made that every official input-chip interaction is implemented.

Android remains paused. The compact evidence is a resized desktop browser.
Audible screen-reader and broader consuming-app verification remain open.

Run `./scripts/dev.sh`, wait for a successful build, reload, and open selection
fields. Compare filled arrow center with half the field height; open/close it,
then remove and restore multiple values using pointer and keyboard. Repeat at
390×844 and reset the viewport afterward.
