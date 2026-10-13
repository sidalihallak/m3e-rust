# Provider and composable sidebar — 2026-10-13

## Scope and references

Preview `#sidebar-provider`. Current official rendered pages read on Oct12–13:
[overview](https://m3.material.io/components/navigation-rail/overview), [specs](https://m3.material.io/components/navigation-rail/specs), [guidelines](https://m3.material.io/components/navigation-rail/guidelines), [accessibility](https://m3.material.io/components/navigation-rail/accessibility).
Actual [upstream source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sidebar.tsx), pinned `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`;
also chip, switch, tabs, item, combobox, table, pagination and message-scroller.

## Contract, checks and limits

`sidebar_provider.rs`/`.js`, `sidebar-provider.css`: optional controlled state,
opt-in localStorage, Ctrl/Cmd+B (editing excluded), Start/End placement,
Standard/Floating/Inset, Icon/Offcanvas/None collapse, native menu actions,
submenus, filter input, separator and skeleton. Header/footer/content/group
continue to use the existing copyable helpers.

Expanded/icon widths 288/80px; offcanvas 0px. Expanded target 56px, collapsed
80px target with visible label and 56×32px indicator; independent actions 48px.
DefaultSpatial 440ms spring comes from shared upstream motion tokens. Source
upstream cookie persistence (7 days) becomes opt-in localStorage; compact state
is not persisted. Upstream collapsed labels are hidden; this kit keeps them
visible per official rail guidance. End placement is an application extension:
primary M3 navigation belongs on the leading side. Offcanvas is for immersive
layouts; do not hide a collapsed primary rail by default.

Fresh collapse samples cover 288→80px including spring overshoot (76.85px),
ending at 80px within the recorded 0.6s interval. One `.app-shell`. Parent
rejection keeps expansion false; parent acceptance changes it true. Input
Ctrl+B leaves expansion true. Actual macOS Reduce Motion: 288/80px immediately,
computed transition 0.00001s from the shared global reduced-motion rule.
Original OS preference off was restored. Compact Shift+Tab wraps Close→Collections.
[Compact screenshot](sidebar-provider-compact.png). Final compact exit samples establish closed state and focus on Expand workspace
sidebar. Surface measured358×844px inside the390px modal viewport.

Earlier Oct12 observations: persisted reload, disabled Reports skip, Collections
submenu/action activation, end/floating and inert offcanvas, compact full slots.
Those transient arrays were lost on restart; they are observations, not saved
raw evidence. Multi-provider shortcut arbitration, native Android, source rail
edge handle and all size/outline/tooltip variants remain outside this pilot.

### Verification and reproduction

Run `./scripts/dev.sh`, wait for a successful Wasm build, reload, then open the
listed hash route. Use actual native pointer and keyboard input. Fresh desktop
results and sample arrays are in [advanced-behavior-samples.json](advanced-behavior-samples.json).
All seven new copy controls matched their displayed Rust snippets exactly.
The exported editable library and **123 usage examples** passed an offline
`cargo check` for `wasm32-unknown-unknown` (2026-10-13, 11.99s); an existing
unused `ChipSize` import warning remains. Final anchoring/announcement fixes were rebuilt and rechecked; see the family report.

Android remains paused. Compact browser geometry is desktop responsive evidence,
not Android touch evidence. Audible screen-reader, forced-colors, large-font,
live-prop interruption and exhaustive variant coverage remain open. Source
settings alone do not establish runtime behavior.
