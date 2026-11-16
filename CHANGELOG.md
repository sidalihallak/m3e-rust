# Changelog

All notable changes to `m3e-rust-ui` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows
[Semantic Versioning](https://semver.org/) as described in [docs/versioning.md](docs/versioning.md).

## [Unreleased]

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
