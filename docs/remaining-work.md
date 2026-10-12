# Remaining work — 2026-10-11

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

## 2. Finish optional behavior in existing components

- Switch drag gestures and card dragged/elevation states.
- Chip avatars and drag behavior.
- Finish input-chip keyboard editing/navigation and the whole-chip focus target
  for remove-only chips; the embedded Combobox currently exposes named native
  remove buttons with Enter/Space support. See the
  [official-reference audit](multi-select-filled-alignment-verification.md).
- Scrollable tabs, inline badges in tabs and content-swipe integration; vertical
  tabs are an additional upstream feature.
- Audit all variants and disabled/focus combinations rather than treating the
  gallery's sampled states as exhaustive coverage.

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
not by source-file count. Basic inputs/textarea already exist through TextField,
and button toggles through the current button APIs; separate upstream wrappers
and richer compositions are not yet equivalent ports.

| Priority/family | Remaining ports |
| --- | --- |
| Help and selection extensions | Transient rich help, custom/virtual popup anchors and boundaries, async/virtual suggestions, chip Backspace navigation; core family is implemented |
| Surfaces extensions | Additional snap points, nested drawer stacking, velocity fling and whole-surface touch gestures; core bottom/side/generic sheets implemented |
| Navigation extensions | Sidebar provider/persistence, right-edge/offcanvas variants and auxiliary item wrappers; menubar/navigation menu/core sidebar helpers and app bar/toolbar/bar/rail are implemented |
| Feedback and content extensions | Imperative toast queue/promise host, source-to-full-screen search morph, search app bar, async/virtual suggestions, list expansion/swipe/typeahead; core family is implemented |
| Scheduling extensions | Date/calendar touch swipes, infinite range scrolling; time pointer-cancel/held/repeated input evidence, broader locale/accessibility coverage |
| Larger upstream compositions | Command nested/fuzzy/virtual extensions and breadcrumb overflow/router adapters; charts, carousels, command palette and breadcrumbs are implemented; accordion/collapsible, table, pagination and client-side DataTable are implemented |
| Input extensions | Field/input-group wrappers, OTP input, standalone input/textarea/label/toggle-group wrappers |
| Upstream helpers and application blocks | Aspect ratio, scroll area, resizable panels, keyboard hints, shapes/markers, image-derived theme, attachment/bubble/message/message scroller/questionnaire |

Provider/context helpers (M3 theme scope, color mode, portal container) need an
idiomatic Dioxus integration design rather than a literal React API translation.
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
Next new family: input groups and OTP input.

## Navigation extensions — 2026-10-12

Menubar, native-link NavigationMenu and controlled Sidebar helpers are implemented
locally with three new usages. All **82** exported examples compile; three copy
controls match exact clipboard text. Desktop keyboard/pointer/repeated toggle,
390px bounds and compact modal focus have [saved evidence](navigation-extensions-samples.json).
Full upstream sidebar provider/auxiliary wrappers, navigation shared viewport,
advanced adaptation and broader platform checks remain in the component reports.
Next new family: input groups and OTP input.
