<!-- Later implementation notes override historical planning entries below. -->
# Remaining work — 2026-10-13

The desktop defects identified in the fidelity review have implementation fixes,
including the subsequent badge, segmented-content and primary-indicator
corrections. See [the fix report](component-fidelity-fixes.md) and the latest
sections of each component report. Historical sections describe older versions;
they are not the current defect list.

## 1. Verify the existing components more broadly

- Expand actual reduced-motion coverage beyond the sampled buttons and indicators.
  The 2026-10-10 audits verified the OS preference, indicator pause/resume, card
  activation/ripple and full-screen dialog. Other component/state coverage remains.
- Run screen-reader checks: names, selection announcements, live badge counts,
  tab/panel association, menu focus and keyboard interaction.
- Expand rendered state/custom-seed contrast and RTL coverage. The six default
  seed/mode semantic text matrices now pass; RTL tab and connected-group arrows
  were exercised. Hover overlays, forced colors and text enlargement remain.
- Check dynamic labels/fonts, transformed ancestors and live prop changes.
  The tab observer handles these in source, but those permutations are not all
  measured. Loading-indicator shape/frame parity also needs a deeper comparison.
- Check font metrics against the visual reference. For example, Inter renders
  the 999+ badge at about 38.13px; the official illustration is 34px. Its fixed
  anchor was corrected, but full typography/pixel parity is not claimed.
- **Android remains paused at the user's request.** When resumed, test touch
  targets, quick taps, held/repeated presses, scrolling that starts on a control,
  touch cancellation and on-screen keyboards. Native Dioxus Android requires a
  separate package/device check from the web preview in Android Chrome.

## 2. Extend and verify advanced behavior

[Sidebar/advanced implementation and desktop evidence](advanced-behavior-verification.md)
now covers provider persistence/context, menu helpers, async/virtual selection,
chip keyboard editing, scrollable/badged tabs and panel swipe, switch drag,
card/chip reorder with native alternatives, list expansion/typeahead/swipe,
multisort/edit/width/CSV and streaming-message anchors/status.

Remaining extensions include chip avatars, animated swipe-to-reveal and tab
panel translation, cross-group reorder/autoscroll, variable-height options,
standalone Select/Autocomplete async permutations, boundary-drag widths,
server/virtual data tables and full message-scroller engine parity. Sidebar
edge rail/every item variant/multi-provider arbitration remain optional.
Broader reduced-motion, audible reader, RTL drag and Android checks remain.

## 3. Extend the Material 3 Expressive component kit

The upstream source inventory at c37c0d2 contains many components not yet ported.
Recommended order for reusable application building blocks:

1. Help and selection (tooltip/rich tooltip, popover, hover card, select/native
   select, combobox/autocomplete) is implemented with [desktop evidence](help-selection-verification.md). Standard/connected
   groups, split buttons, dialogs/alerts and dropdown/context menus are also
   implemented; see [composite scope and checks](composite-verification.md) and
   [connected-group report](button-group-verification.md). The official segmented-button
   pages now recommend connected groups for new M3 Expressive designs; the
   existing classic segmented control remains available.
2. Menu extensions: badges/custom content, compact bottom-sheet adaptation and
   deeper dynamic-content checks. Dialog unsaved-data confirmation belongs to app state.
3. Core navigation and surfaces are implemented; see [navigation evidence](navigation-verification.md) and [surfaces evidence](surfaces-verification.md). Current Expressive expanded rails replace legacy navigation drawers.
4. Core feedback and content is implemented; see [family evidence](feedback-content-verification.md). Advanced toast queues/promise APIs, search morph/virtual suggestions and list expansion/swipe remain.
5. Core date picker/calendar is implemented; see [date picker](date-picker-verification.md) and [calendar](calendar-verification.md). Time picker is also implemented; see [time picker](time-picker-verification.md). Accordion/collapsible, table, pagination and local DataTable are implemented; charts and carousels are implemented.

### Remaining component inventory

Compared the current local public modules with the upstream
[component directory](https://github.com/Crysta1221/shadcn-m3e/tree/main/packages/m3e/src/components)
through GitHub's contents API on 2026-10-10. The directory contains both Material
components and library-specific additions; this list groups ports by family,
not by source-file count. Core Input/Textarea/Label/Field/Toggle/ToggleGroup wrappers now exist alongside
TextField and the existing button APIs. Remaining items below are extensions.

| Priority/family | Remaining ports |
| --- | --- |
| Help and selection extensions | Transient rich help, custom/virtual popup anchors and boundaries, async/virtual suggestions, chip Backspace navigation; core family is implemented |
| Surfaces extensions | Additional snap points, nested drawer stacking, velocity fling and whole-surface touch gestures; core bottom/side/generic sheets implemented |
| Navigation extensions | Sidebar provider/persistence, right-edge/offcanvas variants and auxiliary item wrappers; menubar/navigation menu/core sidebar helpers and app bar/toolbar/bar/rail are implemented |
| Feedback and content extensions | Imperative toast queue/promise host, source-to-full-screen search morph, search app bar, async/virtual suggestions, list expansion/swipe/typeahead; core family is implemented |
| Scheduling extensions | Date/calendar touch swipes, infinite range scrolling; time pointer-cancel/held/repeated input evidence, broader locale/accessibility coverage |
| Larger upstream compositions | Command nested/fuzzy/virtual extensions and breadcrumb overflow/router adapters; charts, carousels, command palette and breadcrumbs are implemented; accordion/collapsible, table, pagination and client-side DataTable are implemented |
| Input extensions | Arbitrary field slots/native constraints and form integrations; vertical/custom-spacing toggle groups; core Input/Textarea/Label/Field/Toggle/ToggleGroup, input groups and OTP are implemented |
| Upstream helpers and application blocks | Theme/provider/portal extensions; core application blocks implemented |

Theme scope/color mode and Dioxus context are now implemented. Portal-container
adapters and upstream provider extensions still need consuming-project design.
The existing Rust theme generation and native popup behavior cover part of their
purpose. Review each optional helper against actual consuming-project needs.

This is a planning order, not a statement that every upstream extension is an
M3 component. Each port must independently read its official contract and actual
upstream implementation using [the component protocol](component-development.md).

## 4. Finish the Rust/UI distribution workflow

The current [source export](copy-components.md) and all 79 usage examples compile
as an independent library. Still needed:

- Launch a real consuming app with copied CSS, color roles and assets, and
  verify runtime styling, motion and interactions there.
- Broaden source-copy verification beyond the shared clipboard workflow. Nine
  gallery copy controls succeeded and a native paste exactly matched the Divider
  snippet (509 characters); all 79 examples compile. Other consumer runtimes remain.
- Add a per-component registry/installer workflow if that is the desired
  distribution model. The current exporter copies the complete editable kit;
  it is not yet a Rust/UI registry installer.

## Suggested next step

The [accessibility audit](accessibility-verification.md) and
[composite pilot](composite-verification.md) have desktop evidence. The post-restart browser/copy-control recheck is complete; next expand audible
screen-reader verification and held-pointer/compact regression coverage. Core Scheduling is implemented with date picker, time picker and calendar evidence. Accordion/collapsible, table, pagination and local DataTable are implemented; charts and carousels are implemented. Core feedback/content has desktop evidence and eight copyable examples.
Help and selection now has desktop and compact-layout evidence. Android remains paused.

Navigation follow-up: skip link, tall navigation bar and vertical toolbar interactions still need broader browser checks. Compact routing and modal motion now have saved samples; the original navigation session arrays were not persisted.

The 2026-10-11 [drawer/showcase follow-up](showcase-navigation-verification.md) has saved desktop motion, focus and compact-routing evidence. Additional interruption, RTL and reduced-motion combinations remain; use current surface/scrim measurements rather than historical modal geometry.

Local browser access was restored with a fresh Codex in-app browser tab. Desktop drawer entry/exit, scrim fade, inert targets and compact routing now have [saved runtime evidence](showcase-navigation-verification.md); interrupted/reduced-motion/RTL and Android follow-ups remain.

Core surfaces now have desktop,390px responsive and actual bottom-sheet reduced-motion evidence. Android touch and native packages remain paused. See the family report for unported drawer extensions.


Feedback/content is implemented in the local preview. Latest evidence includes
focus-pause timing, native selection, search keyboard commitment, actual reduced
motion and 390px search/list layouts. Audible screen-reader checks, broader RTL,
text zoom and Android remain open. Future Scheduling extensions should inspect current Material date/time guidance
and upstream sources before adding APIs.


Disclosure/data compositions now have [accordion](accordion-verification.md),
[collapsible](collapsible-verification.md), [table](table-verification.md),
[pagination](pagination-verification.md) and [data-table](data-table-verification.md)
evidence. Advanced table server state, virtualization, editing, column sizing,
multisort and export remain extensions. The showcase still has a 360px body
minimum; 390px responsive checks passed, while 320px page overflow is recorded.


## Visual family — 2026-10-11

Charts and standard/expressive carousels are implemented locally. The refreshed
independent library plus **75** snippets compile, with exact clipboard evidence
for all nine new visual examples. Desktop keyboard/pointer/drag, chart dark mode,
390px chart/expressive layouts and actual expressive Reduce Motion checks have
[saved samples](visual-family-samples.json). Android, audible screen readers,
RTL, forced colors, zoom and broader interruption/dynamic-state checks remain.
Current scope: bar/line/area/donut; horizontal/vertical standard carousel;
multi-browse, hero, uncontained and vertical full-screen expressive carousel.
Centered hero, multi-aspect ratio, complete Compose keyline parity, richer chart
engines and Embla/Recharts API parity remain extensions. See component reports.
Command palette and breadcrumbs are implemented locally; navigation menu/menubar and controlled sidebar helpers are implemented locally.

The excessive expressive terminal gap is fixed using Material-style shifted size keylines;
[desktop and390px end evidence](expressive-carousel-verification.md#terminal-layout-correction--2026-10-11) records trailing error below1px.

## Commands and wayfinding — 2026-10-12

Inline Command, CommandDialog and Breadcrumb primitives/Trail are implemented
locally with four copyable examples. All79 exported usages compile. Native
keyboard, filtering, disabled actions, modal focus restoration and390px layouts
have [command](command-verification.md) and [breadcrumb](breadcrumb-verification.md)
evidence. Four copy controls match clipboard text exactly. Automatic breadcrumb
overflow, cmdk fuzzy ranking/nested pages and broader platform coverage remain.
Next new family: theme scope/color mode helpers and image-derived seed.

## Navigation extensions — 2026-10-12

Menubar, native-link NavigationMenu and controlled Sidebar helpers are implemented
locally with three new usages. All **82** exported examples compile; three copy
controls match exact clipboard text. Desktop keyboard/pointer/repeated toggle,
390px bounds and compact modal focus have [saved evidence](navigation-extensions-samples.json).
Full upstream sidebar provider/auxiliary wrappers, navigation shared viewport,
advanced adaptation and broader platform checks remain in the component reports.
Next new family: theme scope/color mode helpers and image-derived seed.

## Input extensions — 2026-10-12

Input groups (inline/block addons, actions and textarea) and single-native-input
OTP codes are implemented locally. Four copy controls match exactly and all86
exported usages compile. Desktop editing/selection/paste/disabled/read-only and
390px bounds have [saved evidence](input-extensions-samples.json).
Android, actual Reduce Motion, audible screen readers, native SMS autofill,
password-manager adaptations and broader input/browser permutations remain.
Standalone Input/Textarea/Label, Field compositions and ToggleGroup wrappers
remain optional additions; existing TextField already handles ordinary inputs.
Navigation extensions were pushed at378dae2; input extensions are on main at3bf4d66.

## Layout helpers — 2026-10-12

Aspect ratio, native ScrollArea with custom bars and ResizablePanelGroup/Panel/Handle
are implemented locally. Five usages and all91 exported examples compile.
Keyboard/native pointer, scrollbar entry/exit, resize cancellation/collapse and
390px adaptation have [saved evidence](layout-helpers-samples.json).
Android, audible screen readers, actual Reduce Motion, touch cancellation and
broader dynamic/nested cases remain. Full Base UI/react-resizable-panels parity
is outside the current core port. Layout family is published on main at71ac627.
Keyboard hints and shape/marker helpers are implemented; next consider theme
providers/image-derived seed and application blocks. Standalone field/toggle
wrappers and distribution/consumer-runtime work remain in the inventory above.

## Expression helpers —2026-10-12

Kbd/KbdGroup, Shape with all35 source polygons, and Marker/Icon/Content are
implemented locally with four usages; all95 exports compile. Exact polygon/source
comparison, pointer/keyboard/interruption, actual OS Reduce Motion off→on→off,
markers' native-link behavior,390px compact widths and exact clipboard checks
have [saved evidence](expression-helpers-samples.json).
Android, audible readers, all shape-pair/permutation and broader accessibility
checks remain in the reports. Layout and expression families are published on main at71ac627.
Next family: theme scope/color mode helpers and image-derived seed, followed by
optional application/message blocks. Field/toggle wrappers and source installer/
consumer runtime work remain in the inventory.


## Theme helpers —2026-10-12

ThemeScope, light/dark/system ColorMode, ThemeProvider/use_theme context and
ImageThemePicker with MCU seed ranking are implemented locally. The complete
color-role output adds fixed roles, lowest/bright/dim surfaces and error containers.
All98 exported usage examples compile. Desktop keyboard, six mode/contrast states,
image seed/error behavior, exact copies and390px bounds have saved verification
reports and samples. Root persistence, arbitrary image
sources, Vibrant multipalette parity, variant/spec/motion/shape provider extensions
and broader platform coverage remain. Core application blocks are now implemented locally. Next new ports: optional
form/field/input/toggle wrappers.


## Application blocks —2026-10-12

Attachment slots/states, seven Bubble variants/reactions, Message compositions,
MessageScroller and a data-driven Questionnaire are implemented locally. Five
exact clipboard usages join103 compiling exports. Native pointer/keyboard,
question validation/retention/skip/submit/disabled focus, scroll following/pause/
prepend/interruption, desktop and390px geometry, and actual OS Reduce Motion
off→on→off have saved reports/raw samples.

Remaining application extensions: complete scroller streaming-turn spacer/peek
and visibility APIs, virtualization, questionnaire slots/controlled/reset/default
answers/advanced constraints, richer uploader/composer integrations and broader
platform coverage. They remain library application patterns, not separately
specified official M3 components. Next: forms/field/Input/Textarea/Label/ToggleGroup
wrappers, or extend existing components and consuming-app distribution.


## Form primitives —2026-10-12

Input, Textarea, Label, FieldSet/Legend/Group/Field/Content/Title/Label/Description/
Error/Separator, standalone Toggle and string-valued ToggleGroup are implemented
locally with six exact copy examples; all109 exports compile. Pointer/keyboard,
validation/helper replacement, disabled navigation, required and multiple
selection, sampled shape/width motion and390px layout have saved reports.

The core forms wrapper family is covered. Remaining work is primarily deeper
upstream API parity (vertical/custom-spacing toggle groups, arbitrary field slots/
form integration, provider and message-engine extensions), distribution/installer
and consuming-app runtime verification, Android touch, RTL and broader accessibility.
Accepted theme helpers and application blocks are published at0362828.

The compact showcase selector now remounts per route; actual native selected
values matched form-fields and toggles after switching families.


## Content and loading compositions —2026-10-12

Item/Group/Media/Content/Title/Description/Actions/Header/Footer/Separator and
compact Spinner now have explicit copyable wrappers. Three examples join112
compiling exports. Native selection/focus reentry, exact copies, measured shape/
arc/rotation,390px layout and actual OS Reduce Motion have saved evidence.
Form primitives are published at8299184.

Next priorities: toast host/queue/promise notifications, richer sidebar/provider
APIs, source registry/installer and consuming-app runtime. Full upstream slot/
engine parity, Android touch, RTL and audible assistive checks remain extensions.


## Notification queue —2026-10-12

Item and Spinner are published on main at0ec375b. ToastProvider, Toaster, scoped
manager, FIFO queue, ID updates/dismissal and Rust future loading→success/error
are implemented locally for review. Four exact copy examples join116 compiling
exports. Desktop queue/update/callback/keyboard/pause timing, stale timer/result
guards, inverse colors, measured motion, actual Reduce Motion and390px layout
have saved evidence in [the report](toaster-verification.md).

Next new family: richer Sidebar/provider compositions. Remaining extensions:
source registry/installer, consuming-app runtime, deeper upstream slot/engine
parity (including Base UI toast stacks/swipe/portal adapter), result-message
mappers, Android touch, RTL, audible assistive checks and broader accessibility.

## Latest advanced follow-up — 2026-10-13

Sidebar extensions and the advanced behavior pilot are implemented locally with
seven copy examples and123 independently compiling usages. The family report
lists actual verified states and limitations. This supersedes earlier entries
listing switch drag, chip keyboard editing, scrollable tabs, async suggestions,
list gestures, data editing and message anchors as wholly unimplemented.
Remaining families are extensions/hardening: scheduling gestures, command
nested/fuzzy/virtual behavior, breadcrumb router/overflow adapters, input/form
slots, provider/portal integrations and consuming-app distribution checks.

## Icons and distribution pilot — 2026-10-13

Complete pinned Material Symbols catalogue, local selective registry installer
and independent consuming-app desktop verification are implemented locally.
76 selective entries and 126 exported usages compile. This supersedes the
earlier entries marking distribution/consumer checks wholly unimplemented.

Remaining: remote registry/release/CI, full upstream engine/slot parity,
scheduling gestures, command nested/fuzzy/virtual behavior, breadcrumb router/
overflow adapters, provider/portal integrations, Android touch, native Dioxus
targets, RTL and audible accessibility checks. See the three new reports.

## Registry release preparation —2026-10-13

GitHub-only versioned source distribution, deterministic packaging/checksums,
installer roundtrip tests and CI/release workflows are prepared locally.
Conclusive icon source and rendered/live-change comparisons are recorded.
Remaining distribution steps: commit/push, hosted CI execution, create matching
version tag, review the resulting draft and publish. A standalone public registry
service is unnecessary. Private access can use authenticated GitHub downloads
or a pinned clone. Android/native/RTL/audible/platform and API extensions above
remain open.
