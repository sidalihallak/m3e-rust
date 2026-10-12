# Navigation menu verification — 2026-10-12

## Scope / sources

`navigation_menu.rs`, `navigation-extensions.css`, gallery; depends on Popover
and its owned anchored runtime. Inspected full [upstream navigation menu](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/navigation-menu.tsx).
Read the rendered Material catalog (no standalone navigation-menu page) and
[WAI disclosure navigation](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/examples/disclosure-navigation/).
Current Material menus guidance was also read; destinations are native links,
not application menuitems.

## Contract

Named nav/ul hierarchy with native direct links and button-controlled destination
popups. Enabled links retain href/new-tab/context-menu behavior; current-page
aria-current comes from caller state. Disabled groups use native disabled buttons.
Popup is an explicitly labelled nonmodal dialog with native link lists, matching
our existing Popover semantics; no menu/menubar role assigned to destination links.
Tab moves through links and leaves, Escape returns to opener, outside closes.
The controlled single active index prevents simultaneously open groups.

Source label-large14px, round triggers, secondary-container open state,12px down
icon,8px popup offset,8px popup shape, surface-container fill,12px link padding.
48px trigger/link targets replace source40px visual triggers. Descriptions are
body-medium14px. Popup uses existing anchored fast-spatial/default-effects motion
rather than source's shared viewport and350ms lateral content translations.
Visible group titles differ from upstream content-only popup.

## Verification

Runtime results are recorded below. Android remains paused.

## Gaps

Base UI hover activation, shared viewport/indicator, lateral content transitions,
rich custom render slots/router adapters and direct-link active API are extensions.
RTL, screen-reader speech, forced colors, zoom and mobile touch remain unverified.

## Runtime checks and repairs

- Explore initially focuses Getting started. Tab moves to Components; the next
  Tab closes the popup and focuses Resources. Escape returns to Explore.
- Clicking Resources while Explore is open switches correctly. Components
  native Enter navigation reaches `/#buttons`. Upcoming is native disabled.
- Existing help styles initially overrode intended popup width/radius. Scoped
  specificity now yields360px/8px without altering shared Popover defaults.
- At390px, popup width359px at x8 ends x367; body/client375px, no overflow.
- Dark popup rgb(33,31,36), text rgb(230,224,233); current link background
  rgb(74,68,88), text rgb(232,222,248). Light restored after inspection.
- Actual popup frames record opacity/scale and settle at the final rectangle.
  This verifies reused anchored entry, not upstream shared viewport parity.

[Desktop](evidence/navigation-extensions/navigation-menu-desktop.png),
[390px](evidence/navigation-extensions/navigation-menu-390.png).
Use `/#navigation-menu`; open Explore, Tab through links, Escape, switch groups
and activate a native link. The diagnostic `geometry` field precedes the CSS
repair; `finalGeometry`/`phone` are current measurements.

## Build, export and evidence

Final preview built successfully in13.47s after the last gallery update (earlier
successful31.18s/20.10s builds include runtime repairs). Reloaded before final
checks; one `.app-shell` confirmed. The final refreshed independent Rust/Wasm
library and all **82** usage examples compile in **7.26s**, with one existing
unused ChipSize warning. No dependency was added. Preview remains running.

[Raw desktop/responsive/motion/copy samples](navigation-extensions-samples.json).
Three new copy controls exactly match displayed Rust text: Menubar568,
NavigationMenu427 and Sidebar781 characters. The initial navigation-menu read
preceded clipboard completion; the completed recheck passed. Viewport and light
mode restored. Android remains paused;390px results are desktop responsiveness,
not Android touch evidence. Audible screen-reader, actual OS Reduce Motion,
RTL, forced colors, zoom and held/touch cancellation checks remain open for this
family. Existing shared runtimes support reduced motion but were not newly
verified under the OS setting here.
