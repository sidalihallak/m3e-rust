# Editable field trailing action — 2026-10-10 follow-up

**Subsequent correction:** the single-value measurements below remain valid;
multi-selection now retains dropdown and per-chip removal rather than a
clear-all action. See the [latest official-reference audit](multi-select-filled-alignment-verification.md).

## Scope and sources

Spacing correction for editable ChoiceField (`src/components/select.rs`,
`assets/select.css`), shared by Combobox and Autocomplete. The filled gallery
adapter in `src/help_gallery.rs` now passes an empty values vector after clear,
rather than a vector containing an empty string. Select and NativeSelect were
not changed by this correction.

Re-read rendered Material text-field [overview](https://m3.material.io/components/text-fields/overview),
[specs](https://m3.material.io/components/text-fields/specs),
[guidelines](https://m3.material.io/components/text-fields/guidelines) and
[accessibility](https://m3.material.io/components/text-fields/accessibility)
on 2026-10-10. Reviewed actual upstream
[Combobox](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/combobox.tsx)
and [InputGroup](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/input-group.tsx)
at revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Diagnosis and reference contract

The previous implementation rendered clear and dropdown together. Its CSS
placed their centers 56px apart in outlined fields and 48px apart in filled
fields (derived from declarations, not a saved pre-fix browser measurement).
Upstream ComboboxInput explicitly hides its trigger with
`group-has-data-[slot=combobox-clear]/input-group:hidden` while clear is present.
Therefore two adjacent trailing actions were a porting error; there is no
Material-defined clear-to-arrow gap to reproduce from this upstream pattern.

Follow the upstream replacement behavior: populated, clearable Combobox shows
clear; empty Combobox shows dropdown. Clicking the input/padding and arrow-key
navigation still open suggestions when the dropdown is replaced. Autocomplete
continues to show dropdown because its API uses `show_clear: false`.

Material specifies 12dp outer padding with icons, 16dp between icon and text,
vertical icon centering, and a 56dp field. Its guidelines recommend 24dp icons,
clear only when text is present, and named interactive trailing buttons. Use
a 24px icon centered inside one 48×48px button, aligned to the field's logical
end. This yields the 12px outer inset. A 4px flex gap plus the button's 12px
inner inset yields 16px from the input box to the icon; filled fields reserve
52px at the logical end for the same measured gap. Chip-to-chip gaps remain
8px. Multi-chip rows may grow when wrapping.

The 24px glyph and 48px action target deliberately follow Material rather than
upstream's 32px icon-xs button and 16px dropdown glyph. No target shrinking or
overlapping hit regions was introduced. Popup motion tokens are unchanged.

## Actual browser checks

| Input/state | Observed result |
| --- | --- |
| Initial single, multiple, filled, disabled and free autocomplete | Exactly one trailing action per field; 24×24 icon, 12px outer inset, 48×48 target |
| Single outlined and filled | 56px height, icon centered at 28px; input-to-icon gap 16px |
| Pointer clear Language | Empty input; dropdown replaced clear at exactly the same coordinates |
| Pointer dropdown, select Rust | Options opened; Rust committed; clear replaced dropdown |
| Click populated native input | List opened with clear still present |
| Clear while popup open, Down/Enter | List remained open through clear; keyboard recommitted Rust and closed list |
| Filled clear, dropdown, Americas | Empty vector returned dropdown; selecting Americas restored clear; height remained 56px |
| Free autocomplete “des”, Down, inspect active option, Enter | Design committed; one dropdown action retained |
| Disabled clear pointer attempt | Rust retained, no popup |
| Multiple clear | Both chips removed; dropdown replaced clear |
| 390×844 desktop viewport, Rust/TypeScript selected | Field wrapped to 112px; single clear target remained 48×48, icon inset 12px, text gap 16px |
| Actual click/Escape with concurrent style samples | Opening scaleY .605041 / opacity .091418 observed; closing scaleY .995596→.910675 / opacity .955963→.10675 observed |

All six editable gallery fields retained the 12px inset and 48px target at
390px width. Error decoration is separate from the trailing action; its
presence adds space before that action. Motion sample clocks include browser
round-trip latency and must not be interpreted as exact event-relative timing.

## Build and evidence

Final live Dioxus Wasm build completed successfully in 13.49s. Reloaded that
build and confirmed one `.app-shell`. Final `cargo check --locked --offline
--target wasm32-unknown-unknown` passed in 6.76s after the gallery adapter fix.
`git diff --check` passed. The preview remains running.

- [Raw geometry, state, motion and rendered source readings](selection-trailing-action-samples.json)
- [Desktop appearance](selection-trailing-action-desktop.png)
- [Compact desktop appearance](selection-trailing-action-compact.png)

## Gaps and reproduction

Android remains paused. The 390px result is desktop responsive evidence, not
Android touch or on-screen keyboard evidence. RTL editable fields use logical
padding but were not separately exercised in this follow-up. Existing reduced
motion evidence remains in the family report; no new OS preference change was
needed for this layout-only correction. Existing audible screen-reader and
consuming-app runtime gaps remain open.

Run `./scripts/dev.sh`, wait for build success, reload, open selection fields,
and follow the input matrix. Inspect `.m3-choice__clear` or
`.m3-choice__toggle` and its `.m3-icon` while switching empty/populated values.
Set a 390×844 desktop viewport for the wrapping check, then reset it.
