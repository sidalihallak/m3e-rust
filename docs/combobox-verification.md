# Combobox verification — 2026-10-10

## Scope and sources

Editable single/multiple `Combobox` in `combobox.rs`, using the shared ChoiceField and listbox runtime. Committed IDs and draft input are separately controlled.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

Text fields: [overview](https://m3.material.io/components/text-fields/overview) · [specs](https://m3.material.io/components/text-fields/specs) · [guidelines](https://m3.material.io/components/text-fields/guidelines) · [accessibility](https://m3.material.io/components/text-fields/accessibility)

Menus: [overview](https://m3.material.io/components/menus/overview) · [specs](https://m3.material.io/components/menus/specs) · [guidelines](https://m3.material.io/components/menus/guidelines) · [accessibility](https://m3.material.io/components/menus/accessibility)

[Actual upstream Combobox source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/combobox.tsx) · [InputGroup source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/input-group.tsx) · [APG combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/) · [Base UI Combobox behavior](https://base-ui.com/react/components/combobox)

## Reference contract and implementation

**Trailing-action correction:** single-value clear replaces dropdown when populated,
matching upstream; 24px glyph, 12px outer inset, 16px input gap and a 48×48px
target. The previous dual-action layout was corrected. See the
[follow-up measurements and input checks](selection-trailing-action-verification.md).

**Latest multi-selection correction:** input chips keep their per-value remove
actions, and the field retains dropdown instead of clear-all. `show_clear`
applies to single-value fields. See the
[official-reference follow-up](multi-select-filled-alignment-verification.md).

Outlined and filled fields share Select's Material label, colors, focus/error
states and 56px minimum. Native text editing is retained. Case-insensitive
substring filtering, grouped options, empty result status, selected checks,
clear/toggle actions, and multiple selected chips are implemented. Pointer clicks
on the input or field padding open the list. Editable arrow navigation loops
through an unhighlighted input state, matching Base UI's default behavior.

The list uses the upstream menu spring (scaleY .6→1), 6px bottom gap, low surface,
16px corners and elevation 2. Arrow keys establish an active option while DOM
focus stays in the input; Enter commits, Escape cancels the restricted draft,
Tab leaves. Values are option IDs; `input_value` is the draft query.

With `allow_custom`, Enter commits an unmatched draft without an active option.
Multiple custom values append and deduplicate. Selected chips have 32px visible
outlines inside **48px remove targets**; each remove action is named and is in
the Tab order. The native input also supports selecting/deleting all text.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Actual input/padding pointer clicks | Opened, native input focused |
| Editable arrow boundary | After seven items, active option cleared; next Down returned to first, Up from input reached last |
| Type “py” | Only Data/Python remained; input kept DOM focus |
| ArrowDown/Enter | Python ID committed, draft cleared and label displayed |
| Unmatched filter | “No matching options” status rendered |
| Escape after unmatched draft | Prior Python selection preserved |
| Toggle/clear pointer | Opens; clear clears values and keeps popup open after boundary fix |
| Multiple “go” + Down/Enter | Go appended, list stayed open |
| Remove TypeScript pointer | Chip removed, popup stayed open after fix |
| 390px layout | Chips wrapped to 112px field height, no local overflow; all remove/clear/toggle targets 48×48 |
| Named groups | AX retained Systems/Web/Data group containers and their option children |
| Filled editable filter | Africa selected; 56px field, 24px text input, 48px trailing targets |
| Disabled editable pointer | Retained Rust, no popup |
| Custom multiple values | Art then Writing appended; duplicate Art did not add another chip |
| Required custom choices | Error/invalid cleared when first value committed |
| Copy | Exact 630-character snippet; independently compiled |


## Differences and remaining scope

Deliberate sizing choices: field-width popup rather than the upstream single
input anchor +28px expansion; 48px options rather than 44px; 288px popup cap
rather than a 252px list inside a separate popup wrapper. Persistent scrollbar
is kept instead of hiding it. The visible chip shape remains compact, but
48px targets can increase field row height.

No automatic first-match selection, inline completion, async fetching/virtual
list, custom render-option API, multi-chip Backspace navigation or clear-button
Tab stop is claimed. Disabled/error/filled combinations are implemented;
exhaustive state permutations and screen-reader selection announcements remain
additional coverage. Required custom choices need consuming-form validation.


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
