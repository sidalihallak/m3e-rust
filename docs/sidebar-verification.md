# Sidebar helpers verification — 2026-10-12

## Scope / sources

`sidebar.rs`, `navigation-extensions.css`, gallery. Sidebar/Trigger/Layout/Inset/
Header/Footer/Group/Content helpers compose existing NavigationRail and
ModalNavigationRail; controlled state stays with the app.
Inspected full [upstream sidebar](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sidebar.tsx).
Read current official rail [overview](https://m3.material.io/components/navigation-rail/overview),
[specs](https://m3.material.io/components/navigation-rail/specs),
[guidelines](https://m3.material.io/components/navigation-rail/guidelines), and
[accessibility](https://m3.material.io/components/navigation-rail/accessibility).
Material expanded rails replace legacy drawers; generic Sidebar has no separate
four-page official component contract.

## Contract

Source288px desktop and80px collapsed widths,12px header/footer padding,16px
shell/inset corners, semantic surface-container colors. Menu icon changes with
expansion. Destinations retain the existing rail's filled active icon, indicator,
native keyboard and accessible names. Collapsed labels remain visible per current
Material guidance, unlike upstream's icon-only hidden text. Item target geometry
is inherited from the rail, rather than source32px collapsed visual buttons.

Desktop content shares a flex layout and adjusts with rail width. Header/footer
are desktop-expanded slots. Compact599px breakpoint replaces the standard rail
with a button that opens the accepted modal expanded rail. Desktop expansion and
mobile open are independent controlled props; resizing does not automatically
open a modal. Entry/exit, focus/inertness and reduced motion use existing runtimes.
Native app shells must place navigation outside content panes and provide a
compact NavigationBar when appropriate; the gallery is a bounded preview, not
a claim of complete primary-navigation adaptation. Mobile header/footer slots
are not ported into the rail's destination-only modal.

## Verification

Runtime results are recorded below. Android remains paused.

## Gaps

React provider/context, cookie persistence, global Ctrl/Cmd+B, right-edge,
floating/inset/offcanvas variants, secondary action/badge/submenu/skeleton/input
wrappers, predictive back and velocity gestures are extensions. Audible screen
readers, RTL, zoom, forced colors and Android remain unverified.

## Runtime checks

- Desktop expanded288px and collapsed80px. Actual width samples through both
  transitions settle at those values; labels remain visible in both states.
  Expanded destinations are56px tall. Projects Enter updates aria-current=page.
- Trigger uses native aria-expanded,48px target and shared Ripple with finite
 24→8px corners/DefaultEffects. It has an optional native disabled prop. The
  modal rail, standalone IconButton and original Ripple were not edited.
- At390px the standard rail is absent and modal initially closed; body/client
 375px. Desktop expansion does not automatically open compact navigation.
- Compact entry26 frames/exit25 frames:320px modal ends at x0 with transform
  none, then closes and returns focus to Expand sidebar. No trailing surface gap.
- Native Tab starts at Close navigation→Home, cycles through enabled destinations
  and returns to Close navigation, skipping disabled Reports.
- Settings activation updates consuming content heading to Settings, closes the
  modal and returns focus to Expand sidebar. Escape also restores the opener.

[Desktop](evidence/navigation-extensions/sidebar-desktop.png),
[390px](evidence/navigation-extensions/sidebar-390.png).
Use `/#sidebar-helpers`, collapse/expand/select, then resize390px and open the
modal. Verify Tab, disabled Reports, selection and Escape; reset viewport.

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
