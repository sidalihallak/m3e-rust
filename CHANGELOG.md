# Changelog

All notable changes to `m3e-rust-ui` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows
[Semantic Versioning](https://semver.org/) as described in [docs/versioning.md](docs/versioning.md).

## [Unreleased]

## [0.2.1] - 2026-11-20

### Fixed

- Web showcase stylesheet cascade order: render `M3eStyles` before `main.css` and `showcase.css` in
  `src/main.rs`, and qualify `.m3-nav-bar.showcase-mobile-nav` in `assets/showcase.css` so
  `.m3-nav-bar { display: flex; position: relative; }` no longer overrides `display: none` on
  desktop viewports or `position: fixed` on mobile viewports.

## [0.2.0] - 2026-11-20

### Changed

- Upgraded `dioxus` dependency from `0.7` to `0.8.0-alpha.3` (including `Key::Named(NamedKey::*)`
  keyboard handling across `Chart`, `Chip`, `DataTable`, `Menu`, `SplitButton`, and `Switch`, and
  `current_scope_id()` formatting in `motion.rs`) to support both Web and Blitz 0.3-beta native
  renderers (iOS, Android, Desktop).

### Fixed

- **Native Blitz SVG rendering**: Added explicit inline `fill="currentColor"` / `stroke="currentColor"`
  presentation attributes to `Icon`, `MaterialSymbol`, `Checkbox`, `Radio`, and `CircularProgress`
  (rendering the circular track arc in a sibling `<svg>` with `--md-sys-color-secondary-container`)
  so glyphs, checkmarks, radio rings/dots, and progress tracks tint accurately without CSS SVG
  cascade.
- **Wavy `LinearProgress` on native**: Added an inline `<svg>` sine-wave fallback clipped by
  `.m3-linear__active--svg-wave` for non-`wasm32` targets where CSS `mask-repeat: repeat-x` on SVG
  data URLs is not tiled, while preserving animated CSS masks on Web.
- **Outlined `TextField` notch on native**: Added `.m3-field--native-notch` surface masking and
  gated the JS `ResizeObserver` effect to `wasm32` so the top outline no longer strikes through the
  floated label on Blitz.
- **Native overlays & dropdown menus**:
  - `Dialog`, `AlertDialog`, `Sheet`, `BottomSheet`, and `SideSheet` now set `<dialog open>` on
    non-`wasm32` targets, include `z-index: 100`, and wire Rust `onclick` scrim dismissal.
  - `Menu`, `DropdownMenu`, and `SplitButton` now render as anchored Material 3 dropdowns on
    non-`wasm32` targets (`position: absolute; top: 100%; margin-top: 4px;` with a transparent
    click-outside backdrop, `data-open` z-index elevation, and Rust-side close on selection).
  - `Snackbar` sets `data-ready="true"` and `hidden` directly in Rust on non-`wasm32` targets.
- **AppBar / NavigationBar stacking & compact SegmentedButton**: Added `position: relative; z-index: 10;`
  to `.m3-app-bar` and `.m3-nav-bar` so scrolled sibling content cannot steal pointer events, and
  added `overflow: hidden` to `.m3-segmented__item` to prevent label bleed on narrow viewports.

## [0.1.1] - 2026-11-16

### Added

- Page templates for the Material canonical layouts, in `src/components/templates.rs`:
  `PageLayout` (with `PageWidth`), `ListDetailLayout`, `SupportingPaneLayout`, `FeedLayout`,
  `FormLayout`, `SettingsSection` and `SettingsItem`. They are bundled in `assets/templates.css`,
  and the showcase lists them under Content (page layouts, list-detail and supporting pane,
  feed and dashboards).
- Global `@media (forced-colors: active)` baseline rules in `assets/base.css` and `48px`
  `@media (pointer: coarse)` minimum height on `XS` and `Small` `Button`s in `assets/button.css`.

### Fixed

- `ThemeProvider` injects `navigation.css`: `NavigationBar`, `NavigationRail` and
  `ModalNavigationRail` were unstyled in apps (only the preview linked the file).
- Element ids no longer come from a process-wide counter but from the component's scope, so a
  server-rendered page (Dioxus fullstack) and its hydration agree on them. Before, the ids in
  the server's HTML grew with every request and differed from the browser's, so the scripts
  of server-rendered components (a sheet, a dialog, navigation keys) found no element.
- `AppBar` scrolled state (`data-scrolled="true"`) uses `surface-container` tonal elevation with
  `box-shadow: none`, and `NavigationBar` (`data-elevated="true"`) casts its shadow upward.
- Outlined `Select` / `Combobox` (`.m3-choice:not(.m3-choice--filled)`) reserves `8px` top
  padding for its floating-label outline notch, and `prefers-reduced-motion: reduce` collapses
  keyframe animations as well as transitions.

## [0.1.0] - 2026-10-27

First versioned release, distributed as a Cargo crate.

### Added

- Material 3 Expressive components for Dioxus, exported from the crate root.
- `ThemeProvider` and `ThemeSettings`: Material light and dark schemes generated from a seed
  color with `material-colors` (variant, contrast and color mode are settings).
- `M3eStyles` and `component_css()`: every component stylesheet is bundled in the crate, so apps
  link no CSS of their own.
- Consumer example in `examples/consumer`, a standalone app that depends on the crate.
- `llms.txt` at the repository root for agents, and a quick start in `docs/quick-start.md`.

### Changed

- Components are library exports, not copied source. The copy-install registry, the
  `scripts/m3e.py` installer and the registry release workflow were removed.
