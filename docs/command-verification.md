# Command / command dialog verification — 2026-10-12

## Scope and references

`command.rs`, `command.js`, `command.css`, `wayfinding_gallery.rs`.
Inspected the full pinned upstream [command source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/command.tsx).
Upstream delegates state/filtering/keyboard behavior to cmdk and uses InputGroup.

Read the current rendered [Material catalog](https://m3.material.io/components):
there is no standalone command-palette specification. Read current dialog
[overview](https://m3.material.io/components/dialogs/overview),
[specs](https://m3.material.io/components/dialogs/specs),
[guidelines](https://m3.material.io/components/dialogs/guidelines), and
[accessibility](https://m3.material.io/components/dialogs/accessibility) for the
modal composition. Also read [WAI combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/).
These are library compositions with Material roles, not new official M3 components.

## Contract

- Root16px corners/padding4/surface-container-high; input56px pill with
  surface-container-highest/20px search icon/label-large14px. Source list max288px;
  group padding4, heading12px with8px/12px padding; divider1px/4px margins.
- Results min48px,8px corners,12px gap/horizontal padding,16px/24px body-large;
  selected preview uses on-surface10%; disabled opacity.38. Check indicator
  carries an enabled annotation in the accessible label. Shortcuts are display
  hints only, hidden from option names; apps own any global hotkeys.
- Controlled Rust query/actions, stable unique entry values; case-insensitive
  all-word substring match over label/group/keywords. This is not cmdk fuzzy
  ranking. Groups preserve first occurrence and original item order.
- Native combobox input keeps focus; aria-activedescendant identifies the active
  option. Arrow navigation is bounded and skips disabled results; Enter runs it.
  Ordinary text editing/Home/End remain native. IME composition does not submit.
  Pointer preview/click retains input focus. No-match state clears active target.
- Inline Escape clears query. Modal Escape dismisses through existing Dialog;
  activation closes it, returning focus. Query persists until app resets it.
- Dialog has explicit title/description/close icon and inherits native
  modality/inertness, focus loop, restoration and existing motion runtime.
  Library16px shape/compact padding differ from standard28px/24px basic-dialog
  geometry; visible title differs from upstream sr-only header. The surface
  width caps560px and viewport margins are retained. No baseline Dialog edits.

## Verification

Independent export and all79 usage examples compile (8.01s); existing unused
ChipSize import warning only. Browser results are recorded below after
preview rebuild/reload. Android remains paused; responsive checks are desktop.

## Gaps

cmdk fuzzy score/ranking, nested pages, async/virtual results, custom rendering,
global shortcut registration and command-specific loading states are extensions.
Audible screen readers, RTL, forced colors, text enlargement and actual mobile
keyboard/IME/pointer cancellation remain unverified unless recorded below.

## Desktop and responsive evidence

The preview built successfully in35.66s, then25.54s after removing a redundant
close action. Four new clipboard controls (two commands/two breadcrumbs) matched
the visible code exactly. Refreshed independent library/all79 examples passed
in7.76s. Final runtime refresh is recorded below if further repairs are needed.

Actual input checks:

| Check | Outcome |
| --- | --- |
| Arrow navigation | Create→Open→Settings, skipping disabled Archive |
| Bounded/repeated arrows | Final Notifications option remains active |
| Enter | Selected settings / create; native input retains focus inline |
| Keyword match | `new workspace` matches Create project |
| Empty query results | No-match message,0 commands, active descendant removed |
| Inline Escape | Query cleared, first enabled result restored |
| Pointer option | Open project activated, input focus retained |
| Native text editing | Home at0, End at6 for `Create` |
| Disabled surface / result | Input disabled; disabled-only match has no active item; Enter does not close |
| Modal focus | Native :modal true, labelled title, requested input initially focused |
| Dismissal | Selection and Escape return focus to Open command palette |
| Native Tab loop | Input→Close dialog→input at390px |
| Compact geometry |342×454px surface at x24,56px input,48px results,14px text; no page overflow |
| Colors | Dark surface rgb(43,41,47), input rgb(54,52,58), text rgb(230,224,233); light restored |

A scrollable result list initially entered the Tab order. Added explicit
`tabindex=-1`; native keyboard input confirms the repaired focus loop. Some
locator-based Tab samples still reported the list and remain in diagnostic raw
data; explicit focus followed by native Tab is the final evidence.
Entry/exit19/20 frames record actual opacity and transform through inherited
Dialog motion. Entry settles at opacity1/transform none; exit closes the native
modal and restores focus. No new animation timing was added. Actual OS Reduce
Motion was not toggled for this composition, and inherited source support is
not a new platform verification.

[Raw samples](command-samples.json), [desktop](evidence/wayfinding/command-desktop.png),
[390px](evidence/wayfinding/command-390.png), [dark](evidence/wayfinding/command-dark.png).
One app shell confirmed. Android remains paused; virtual keyboard/touch and
screen-reader speech are open. See earlier gap list. Use `/#command-palette`,
filter, use arrows/Enter/Escape, then open modal and repeat Tab/activation checks.

Final result-highlight repair: the owned JS runtime now owns `aria-selected`
without a competing static Rust attribute. It reapplies the active state after
result replacement. Final22.42s preview build/reload confirms Create initially
active and Open active after pointer selection/reopen. Final screenshots omit
the earlier hot-patch toast. The existing query/active choice is preserved when
still valid. Inline/native Dialog baseline components were not edited.

Final refreshed library/all79 example Wasm check passed in7.40s; only the
existing unused ChipSize warning remains.
