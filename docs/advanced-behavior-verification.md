# Sidebar extensions and advanced behavior — 2026-10-13

## Scope

This family adds seven gallery routes and copyable Dioxus examples:
sidebar provider, async selection, scrollable tabs, drag/reorder, advanced lists,
editable tables and streaming messages. Existing component APIs keep their
prior defaults; these behaviors are opt-in. New runtime/CSS is part of the
editable source export.

Notification work preceding this family was pushed to main as `5e564d4`.
This family is local pending user review/push.

## Reference audit

Current official rendered overview/specs/guidelines/accessibility pages were
read on Oct12–13 for [navigation rail](https://m3.material.io/components/navigation-rail/overview),
[text fields](https://m3.material.io/components/text-fields/overview),
[tabs](https://m3.material.io/components/tabs/overview),
[chips](https://m3.material.io/components/chips/overview),
[switch](https://m3.material.io/components/switch/overview),
[cards](https://m3.material.io/components/cards/overview) and
[lists](https://m3.material.io/components/lists/overview).
Per-component reports link all four pages and record values/differences.
Tables and message scrolling do not have dedicated current M3 contracts:
[archived M2 data tables](https://m2.material.io/components/data-tables) and
[WAI native table pattern](https://www.w3.org/WAI/ARIA/apg/patterns/table/) were read,
and that boundary is preserved in the reports.

Actual upstream sources inspected at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`:
[sidebar](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sidebar.tsx),
combobox, tabs, chip, switch, card, item, table, pagination, message-scroller and
shared m3e.css. Upstream autocomplete/DataTable standalone source paths do not
exist; this kit composes their behavior. The underlying message-scroller
`@shadcn/react` 0.3.1 implementation/types were inspected as well.

Dragged card/input-chip level4 values were checked against Material Web's
[v0_192 elevated-card](https://github.com/material-components/material-web/blob/main/tokens/v0_192/_md-comp-elevated-card.scss)
and [input-chip](https://github.com/material-components/material-web/blob/main/tokens/v0_192/_md-comp-input-chip.scss)
token files. Motion uses existing DefaultSpatial440ms and FastSpatial360ms
upstream mappings; these are source values, not a blanket duration for every
Material interaction.

## Fresh desktop input results

Evidence: [raw samples](advanced-behavior-samples.json),
[desktop](advanced-behavior-desktop.png), [compact sidebar](sidebar-provider-compact.png).
Browser: Codex in-app browser on macOS, mouse and keyboard, desktop/default
viewport and a temporary390×844 override. Exactly one `.app-shell` was observed.

| Feature | Input and observed result |
| --- | --- |
| Sidebar collapse | Pointer click; width288→80px, spring interpolation recorded through settle; focus remains on Expand |
| State ownership | Parent rejecting expansion leaves false; accepting changes true |
| Shortcut | Ctrl+B in destination input does not collapse |
| Compact sidebar | Surface358×844, header/filter/menu/footer visible; Shift+Tab wraps Close→Collections |
| Modal dismissal | Escape; saved exit samples reach closed dialog and focus Expand workspace sidebar |
| Async selection | Slow then fast request;2000 logical options,24 mounted remote rows, active option35 exists in DOM; Enter commits p35 |
| Async error/cancel | Error alert and selected p35 retained; cancel sets loading false |
| Input chips | Empty Backspace focuses last chip; Left→Bob; Delete removes; F2 Alice moves value into draft/focus input |
| Scrollable tabs |48px height,52px initial leading offset; end clamps; disabled item skipped; RTL keys and actual RTL panel swipe checked |
| Switch | Space/Enter round trip; pointer drag false→true with exactly one added callback |
| Reorder | Pointer crossing row centre and keyboard commit match; Escape cancels; Move alternative restores boundary focus and announces position |
| Reorder motion |1.538s sample interval covers full interpolation; neighbor transform−108→positive spring overshoot→none; intended FastSpatial360ms |
| List | Prefix moves Design→Themes; Enter expands; action click, Archive Space and real swipe work; Undo restores with no extra primary activation |
| Table | NaN alert; Enter saves42, restores cell focus; Escape keeps Ada; Shift sorting retains primary aria-sort; range Right180→190px |
| CSV | Filter Project2 yields all five matching sorted rows; payload inspected, download file not separately checked |
| Message scrolling | Pause leaves top160 while growing; long response follows true end236 =632−396; peek pauses |
| Stable prepend | Runtime defect corrected; paused reader top160→216 after56px prepend; existing h0 screen Y stays467.992px |
| Copy controls | All seven clipboard strings exactly match displayed snippets |

### Reduced motion

Using prior user permission, macOS Reduce Motion was toggled off→on→off through
System Settings. Actual sidebar immediately reaches288/80px with computed
transition0.00001s (shared override); reordering shows no neighbor interpolation
and commits. This does not establish reduced-motion coverage for every new
selection popup, scrolling or gesture permutation. VoiceOver was not toggled
in this family; audible screen-reader coverage remains open.

### Historical and initial-failure evidence

Oct12 checks additionally observed persistence across reload, disabled skipping,
end/floating/offcanvas, submenu/action and compact slots. Transient arrays were
lost on app restart; these observations are not represented as saved motion.
Raw fresh evidence retains early incomplete/failed checks as well as corrected
checks. In particular the first prepend test used DOM node identity and failed;
`corrected prepend stable IDs` records the passing stable-ID implementation.
The earlier compact focus read occurred before exit finished; the final saved
exit trace establishes restoration. Early virtual option counts include People
options; the remote window itself is24 rows. Early reorder samples were short;
`reorder complete motion samples` covers the full transition.

## Build/export

Successful live Wasm builds confirmed after source corrections (13.78s initial
restart;35.27s and48.62s later updates). Browser reloaded before final checks.
A fresh export to `/private/tmp/m3e-advanced-final-1013` compiles its library plus
123 independent usage examples offline for `wasm32-unknown-unknown`. Initial
check11.99s passed; final repeated check passed in30.41s after the stable-ID anchoring and native move
announcement fixes. `git diff --check` also passed. Existing unused
ChipSize example import warning is unrelated to this family. No new dependency.

## Remaining differences and platform gaps

- Android web touch and native Dioxus Android remain paused; desktop drag and
  compact viewport checks do not close touch scrolling/cancel/keyboard behavior.
- Audible screen readers, forced colors, custom font/zoom and live prop changes
  need expanded coverage. New gestures do have native keyboard/button alternatives.
- Reorder lacks cross-group dragging, edge autoscroll and virtualization.
- SwipeAction commits an action rather than animating the official Android
  multi-button reveal. TabsViewport commits selection without interactive panel
  translation. List expansion is measured height, not a full-screen transform.
- Fixed48px option virtualization does not support variable-height rows; grouped
  virtual labels are flattened accessibly. Native transport cancellation and
  query debouncing are application-owned. Select/Autocomplete-specific async
  permutations need their own checks beyond shared Combobox evidence.
- Width ranges are the implemented accessible sizing UI; boundary drag is open.
  Server-backed sorting/edit persistence and virtual tables are separate work.
- Message scrolling supplies local visible-item tracking and stable anchors;
  full upstream engine/virtualization parity is not claimed.
- Sidebar rail edge handle, every menu size/outline/tooltip variant, cookie
  adapter and multi-provider shortcut arbitration remain optional extensions.

## Reproduction

Run `./scripts/dev.sh`, confirm successful build, reload the preview.
Open `#sidebar-provider`, `#advanced-selection`, `#advanced-tabs`,
`#advanced-gestures`, `#advanced-lists`, `#advanced-tables`, `#advanced-messages`.
Repeat the inputs in the table; inspect the live states and clipboard, not only
CSS declarations. Keep unique item/message IDs and apply controlled callbacks.
