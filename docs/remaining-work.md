# Remaining work — 2026-10-10

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
- Scrollable tabs, inline badges in tabs and content-swipe integration; vertical
  tabs are an additional upstream feature.
- Audit all variants and disabled/focus combinations rather than treating the
  gallery's sampled states as exhaustive coverage.

## 3. Extend the Material 3 Expressive component kit

The upstream source inventory at c37c0d2 contains many components not yet ported.
Recommended order for reusable application building blocks:

1. Popover and tooltip, then select/combobox/autocomplete. Standard/connected
   groups, split buttons, dialogs/alerts and dropdown/context menus are now
   implemented; see [composite scope and checks](composite-verification.md) and
   [connected-group report](button-group-verification.md). The official segmented-button
   pages now recommend connected groups for new M3 Expressive designs; the
   existing classic segmented control remains available.
2. Menu extensions: badges/custom content, compact bottom-sheet adaptation and
   deeper dynamic-content checks. Dialog unsaved-data confirmation belongs to app state.
3. Sheet/drawer/side sheet, app bar/toolbar and navigation components.
4. Toast/snackbar behavior, search, lists/items and avatar.
5. Date/time pickers and calendar, then tables and other larger compositions.

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
| Anchored help and selection | Tooltip, popover, hover card; select/native-select, combobox/autocomplete |
| Surfaces | Bottom sheet/drawer, sheet and side sheet |
| Navigation | App bar, toolbar, navigation bar/rail/drawer, sidebar, navigation menu/menubar |
| Feedback and content | Snackbar/toast, alert banner, search, list/item, avatar, skeleton, empty state |
| Scheduling | Date picker, time picker, calendar |
| Larger upstream compositions | Accordion/collapsible, table, chart, carousel/expressive carousel, command palette, breadcrumb, pagination |
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

The current [source export](copy-components.md) and all 27 usage examples compile
as an independent library. Still needed:

- Launch a real consuming app with copied CSS, color roles and assets, and
  verify runtime styling, motion and interactions there.
- Broaden source-copy verification beyond the shared clipboard workflow. Nine
  gallery copy controls succeeded and a native paste exactly matched the Divider
  snippet (509 characters); all 27 examples compile. Other consumer runtimes remain.
- Add a per-component registry/installer workflow if that is the desired
  distribution model. The current exporter copies the complete editable kit;
  it is not yet a Rust/UI registry installer.

## Suggested next step

The [accessibility audit](accessibility-verification.md) and
[composite pilot](composite-verification.md) have desktop evidence. The post-restart browser/copy-control recheck is complete; next expand audible
screen-reader verification and held-pointer/compact regression coverage. The next components are popover/tooltip and
select/combobox. Android remains paused.
