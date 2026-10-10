# Select verification — 2026-10-10

## Scope and sources

`Select`, `SelectOption` and `SelectSize` in `select.rs`, shared `selection.js`/`anchored.js`, `assets/select.css` and `assets/help.css`.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

Text fields: [overview](https://m3.material.io/components/text-fields/overview) · [specs](https://m3.material.io/components/text-fields/specs) · [guidelines](https://m3.material.io/components/text-fields/guidelines) · [accessibility](https://m3.material.io/components/text-fields/accessibility)

Menus: [overview](https://m3.material.io/components/menus/overview) · [specs](https://m3.material.io/components/menus/specs) · [guidelines](https://m3.material.io/components/menus/guidelines) · [accessibility](https://m3.material.io/components/menus/accessibility)

[Actual upstream Select source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/select.tsx) · [APG combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/)

## Reference contract and implementation

**Filled arrow correction:** trailing glyphs now center in the complete field,
independently of filled value padding. Default 24px glyph, 12px logical end inset,
28px center in a 56px field; compact remains centered at 20px. See the
[measured follow-up](multi-select-filled-alignment-verification.md).

| Property | Implemented value |
| --- | --- |
| Field | Outlined/filled, default 56px, optional compact 40px |
| Shape/type | 4px field corners, 16/24px value, 12/16px label |
| Outlined label | Real fieldset/legend notch; 1px rest / 2px focus border |
| Filled field | Surface-container-highest; 4px top corners; bottom indicator |
| List surface | Field width, max 288px height and viewport space; low container, elevation 2, 16px corners |
| Rows | At least **48px**, rather than upstream 44px minimum; disabled .38 opacity |
| Selected | Tertiary-container/on-tertiary-container, 12px shape and check icon |
| Keyboard focus | Stays on native button with combobox role; active option via aria-activedescendant |

Arrow keys preview, Home/End jump, type-ahead searches, Enter/Space commit,
Escape cancels and Tab closes/leaves. Disabled listbox options are skipped;
menus have different disabled-focus behavior. Required/error/disabled props
and supporting text are exposed. Error adds a non-color icon cue.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Pointer opening | Current selection highlighted; native trigger retains focus |
| Down twice from Europe | Disabled Moon skipped; active Asia Pacific |
| Escape | Original Europe retained |
| Closed type-ahead “a”, then Enter | Americas active and committed |
| Pointer disabled option | Selection unchanged, list stayed open |
| Pointer Asia Pacific | Committed and closed |
| Home/End | Active option ends at Africa |
| Tab | Closed and focused Billing region |
| Required selection | Error state cleared after controlled selection |
| Disabled trigger | Forced pointer activation ignored |
| Rapid Enter/Escape/Enter | Latest open generation remained visible |
| RTL | Popup/field width 463.5px, direction RTL and logical start aligned |
| Compact | 40px field, rows remained 48px |
| 390×844 real scroll | Popup flipped from bottom to top; remained within bounds |
| Dark | Semantic low-container/on-surface colors measured; selected role retained |
| Motion | Final aligned origin `0px -4px`; 463.5px width constant; scaleY .6→1 with small spring overshoot; exit sampled |
| OS Reduce Motion ON | First visible sampled frame settled (early stream affected by RPC latency) |


## Differences and remaining scope

The compact 40px field is a deliberate opt-in density exception; default 56px
should be used for touch-oriented applications. The field is full width instead
of upstream's w-fit/min-width setup. Persistent native scrollbar replaces
upstream scroll-arrow helpers. Popups do not align an item directly over the
trigger, offer custom boundaries, or switch to mobile sheets.

`required` on this custom button supplies ARIA state; the consuming form must
validate the committed value. `name` supplies hidden values. NativeSelect offers
native constraint validation. No claim of complete Base UI keyboard/API parity.

Screenshots: [selection fields](help-selection-desktop.png), [dark scheme](help-selection-dark.png).

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
