# Showcase navigation follow-up — 2026-10-11

## Reference and scope

Inspected the rendered [Material website](https://m3.material.io/) and its component
navigation on 2026-10-11. Observed a compact persistent primary rail, a secondary
component drawer, generous article headings, flat tonal backgrounds, rounded
surfaces and pill-shaped selected destinations. This is an application layout
inspired by the website, not a claim that its editorial shell is itself a reusable
Material component or that its artwork/font is copied.

Files: `src/main.rs`, `src/showcase.rs`, `assets/showcase.css`.
Reusable controls still use the existing theme roles and component APIs.

## Changes

- Fixed 96px primary rail; menu button shows/hides the secondary 240px catalogue.
  Selecting a family opens the catalogue and changes the active route.
- Catalogue has a fixed-width inner surface while its grid track interpolates.
  This prevents label reflow during the reveal. DefaultSpatial 440ms controls
  width/translation; DefaultEffects 240ms controls content opacity. These are
  existing upstream-derived approximations, not prescribed universal durations.
- Closed catalogue is inert and hidden from assistive technology while closing
  visually; it does not retain invisible keyboard targets. Toggle stays in rail.
- Compact layout retains a named menu opener and controlled modal rail, plus
  the existing NativeSelect component picker. Six families and hash routes remain.
- Large page title/description and examples/source links; flat semantic surfaces,
  24–32px showcase card corners, larger spacing and readable metadata.
  Core component geometry and the accepted button press animation are untouched.
- Foundation hero introduces the Rust kit and retains live seed selection.
- Unknown in-page hashes preserve the active page; the examples link targets
  `#showcase-examples`. Source export continues to exclude application-only CSS.

## Checks and gaps

Live Dioxus preview is running at `http://127.0.0.1:8080/` and reported successful
builds. Locked offline Wasm check passed (8.96s); JS syntax and whitespace checks
passed. All 40 standalone exported examples passed with updated sources (4.19s; one existing unused ChipSize import warning).

The browser URL policy rejected local preview inspection. There is no new local
screenshot or measured-motion dataset for this follow-up. The implementation's
CSS values do not establish a smooth runtime result. Required fresh checks:
open/close/reverse the catalogue, inspect stable label geometry and focus, search
across families, native Back, direct hashes, skip/examples links, seed/mode controls,
compact picker/menu and long-label layouts. Verify reduced motion and save actual
width/opacity samples. No Android touch or audible screen-reader result is claimed.

## Reproduce

Wait for `./scripts/dev.sh` to report success, reload the preview, and toggle the
component menu on desktop. Sample `.showcase-catalog` width and `.catalog-inner`
opacity during pointer/keyboard input. Resize to 390px and test the modal panel
and scrim separately. Capture full-page desktop/compact screenshots, restore any
viewport/system settings, and append measured results here when access resumes.

## In-app browser recheck — 2026-10-11

Opening a fresh `http://127.0.0.1:8080/` tab directly in Codex's in-app browser
succeeded. This supersedes the earlier access-block status. One app shell and
`prefers-reduced-motion: false` were observed.

The actual served duration tokens were `.44s` and `.24s`. The modal runtime's
`parseFloat` treated those as 0.44/0.24 milliseconds, causing the sudden motion.
It now converts seconds to milliseconds and also accepts explicit ms values.
The live rebuilt version passed (8.57s). Actual pointer-open and keyboard-Escape
samples show panel movement through -320, -291.5, -232.2, -186.6 and -126.3px,
then a small spring overshoot before resting; the scrim independently fades
0→0.255→0.586→0.743→0.880→1. Exit has multiple interpolated frames, followed by
native close, scroll restoration and focus back on Open navigation.

Desktop catalogue width interpolated 0→23.5→69.0→117.5→171.5→240px, with a slight
spring overshoot. Inner content remained 240px wide throughout. Closing also
interpolated and removed its focus targets. A newly found `inert="false"` bug
made the open catalogue inaccessible; the attribute is now omitted while open.
The open catalogue/search/links are visible in the accessibility tree.

Both modal Tab boundaries were checked. A 390×844 resized desktop check exercised
family Selection and native picker navigation to `#selection-fields`. This is
responsive desktop evidence, not Android touch coverage. The viewport was reset.
Raw observations: [navigation-followup-samples.json](navigation-followup-samples.json).
Times are external sampler elapsed times, not DOM event timestamps. Historical
`modalOpen`/`modalClose` capture the unit bug; `modalOpenFixed`/`modalCloseFixed`
capture the corrected version. [Desktop screenshot](showcase-navigation-desktop.png).

Still untested after the correction: reduced-motion preference changes, interrupted
entry/exit, RTL modal motion, enlarged text and Android. Earlier reduced-motion
results must not be treated as verification of this changed runtime.
