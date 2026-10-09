# Remaining work — 2026-10-09

The desktop defects identified in the fidelity review have implementation fixes,
including the subsequent badge, segmented-content and primary-indicator
corrections. See [the fix report](component-fidelity-fixes.md) and the latest
sections of each component report. Historical sections describe older versions;
they are not the current defect list.

## 1. Verify the existing components more broadly

- Exercise actual OS reduced-motion preference, including toggling it while
  progress/loading indicators and press feedback are running.
- Run screen-reader checks: names, selection announcements, live badge counts,
  tab/panel association, menu focus and keyboard interaction.
- Check the full seed/light/dark contrast matrix and RTL navigation. Current
  RTL geometry evidence covers badges and divider insets, not the entire kit.
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

1. Connected button groups and split buttons. The official segmented-button
   pages now recommend connected groups for new M3 Expressive designs; the
   existing classic segmented control remains available.
2. Dialog/alert dialog, menus/select/combobox, popover and tooltip.
3. Sheet/drawer/side sheet, app bar/toolbar and navigation components.
4. Toast/snackbar behavior, search, lists/items and avatar.
5. Date/time pickers and calendar, then tables and other larger compositions.

This is a planning order, not a statement that every upstream extension is an
M3 component. Each port must independently read its official contract and actual
upstream implementation using [the component protocol](component-development.md).

## 4. Finish the Rust/UI distribution workflow

The current [source export](copy-components.md) and all 19 usage examples compile
as an independent library. Still needed:

- Launch a real consuming app with copied CSS, color roles and assets, and
  verify runtime styling, motion and interactions there.
- Verify the end-to-end copy/paste experience in a normal browser. The gallery's
  clipboard API resolved successfully; the automation's separate virtual
  clipboard did not expose the write for a paste comparison.
- Add a per-component registry/installer workflow if that is the desired
  distribution model. The current exporter copies the complete editable kit;
  it is not yet a Rust/UI registry installer.

## Suggested next step

While Android is paused, finish reduced-motion/accessibility checks on the
current controls, then pilot **connected button groups**. Dialog and select
components are the next useful application-building additions.
