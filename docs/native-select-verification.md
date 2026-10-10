# Native select verification — 2026-10-10

## Scope and sources

`NativeSelect` in `select.rs`: actual HTML select/options/optgroups with Material field styling.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

Text fields: [overview](https://m3.material.io/components/text-fields/overview) · [specs](https://m3.material.io/components/text-fields/specs) · [guidelines](https://m3.material.io/components/text-fields/guidelines) · [accessibility](https://m3.material.io/components/text-fields/accessibility)

Menus: [overview](https://m3.material.io/components/menus/overview) · [specs](https://m3.material.io/components/menus/specs) · [guidelines](https://m3.material.io/components/menus/guidelines) · [accessibility](https://m3.material.io/components/menus/accessibility)

[Actual upstream NativeSelect source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/native-select.tsx)

## Reference contract and implementation

Outlined/filled 56px fields, opt-in 40px density, 4px corners, 1px rest/2px focus
indicator, persistent 12/16px label, body 16/24px and decorative 20px arrow.
The real select provides name/value, disabled/required native constraints and
keyboard/browser/OS picker behavior. `onchange` reports a String option value.
Native group labels and disabled options are retained.

The added filled variant and persistent label reuse the same Material field
contract as Select. Disabled text uses Material .38 state opacity rather than
the upstream wrapper's .50 opacity. Error/supporting text and its icon offer
additional context.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Browser selectOption action | Actual change event committed Africa, visible native value changed |
| Native structure | Named native combobox, disabled Moon option retained |
| Keyboard ArrowDown/Enter | Native control remained usable; value eu observed (no value-change claim) |
| Compact/layout | Native field stays within the family container |
| Copy | Exact 437-character usage copied; source export compiles |


## Differences and remaining scope

The expanded picker is rendered by the browser/OS and has no kit-controlled
spring or Material popup shape. Its expanded visual appearance and keyboard
selection sequences were not exhaustively verified. Supply an explicit empty
option if an unselected native value is needed; the browser otherwise presents
the first enabled option. Do not confuse this fallback with the custom exposed
listbox's visual parity.


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
