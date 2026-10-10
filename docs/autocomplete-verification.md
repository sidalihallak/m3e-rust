# Autocomplete verification — 2026-10-10

## Scope and sources

`Autocomplete` in `combobox.rs`: free text with manual list suggestions; shares the editable ChoiceField runtime.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

Text fields: [overview](https://m3.material.io/components/text-fields/overview) · [specs](https://m3.material.io/components/text-fields/specs) · [guidelines](https://m3.material.io/components/text-fields/guidelines) · [accessibility](https://m3.material.io/components/text-fields/accessibility)

Menus: [overview](https://m3.material.io/components/menus/overview) · [specs](https://m3.material.io/components/menus/specs) · [guidelines](https://m3.material.io/components/menus/guidelines) · [accessibility](https://m3.material.io/components/menus/accessibility)

[Actual upstream Combobox source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/combobox.tsx) · [APG combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/)

## Reference contract and implementation

**Shared field spacing correction:** the dropdown is now a 24px glyph with
12px outer inset, 16px input gap and a 48×48px target. This free-text API keeps
`show_clear: false`; clearable single-value Combobox replaces dropdown with clear. See the
[follow-up report](selection-trailing-action-verification.md).

This is a Rust convenience API over the upstream editable-combobox pattern,
not a distinct official Material component. Field and popup appearance match
Combobox. Arbitrary text is a valid value, and a manually chosen suggestion
writes its **label** into the native input. `onselect` reports its option ID;
Enter with no active option can report custom text. There is no automatic
first-result selection or inline completion. Optional `name`, placeholder and class
props support native form integration; `required` applies to its native free-text input.

The native input has combobox/list autocomplete semantics, with active-descendant
navigation. Standard Home/End/Left/Right, selection and IME editing are preserved.
Escape dismisses suggestions without discarding the free text.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Type “de”, Down/Enter | Design label displayed, design ID reported |
| Type “My custom tag”, Enter | Custom text remained and popup closed |
| Native Home then Right | Selection start/end both 1; native editing retained |
| Copy control | Exact 572-character Rust snippet copied |
| Actual clipboard paste into single-line input | 557 characters; exact after native line-break removal (15 newlines) |
| Compilation | Independent usage example passes Wasm source-copy check |


## Differences and remaining scope

This manual suggestion mode does not perform async fetching or inline/automatic
completion. Dense multi-chip editing belongs to Combobox. Native text validation
and application rules must be supplied by the caller. The same runtime was
checked under reduced motion for Select, but this field did not receive a
separate reduced-motion stream.


## Builds, evidence and reproduction

The live Dioxus Wasm build succeeds; source export and all **35** independent
usage examples compile for `wasm32-unknown-unknown`. The existing ChipSize
unused-import warning remains. See [family build and motion details](help-selection-verification.md).

[Raw browser samples](help-selection-samples.json) include actual input, focus,
expanded/selected states, computed geometry and motion. Each sample stream uses
browser round trips; its clock is relative to the sampling request, **not** the
DOM input event. Do not interpret RPC latency as a component animation duration.

Run `./scripts/dev.sh`, wait for successful completion, reload and check one
`.app-shell`. Open the family anchors, perform the matrix above through real
pointer/keyboard input, and sample computed styles concurrently. Read the
[copy guide](copy-components.md) for dependencies and controlled state.

Desktop IAB and a desktop browser at **390×844** are the verified platforms.
Android touch/on-screen keyboard and native Dioxus Android remain paused.
Audible screen-reader announcements, forced colors, text enlargement,
transformed/scrolled ancestors, dynamic anchor replacement and unmount stress
remain unverified. A separate consuming-app runtime is also still open.
