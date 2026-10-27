# Changelog

All notable changes to `m3e-rust-ui` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows
[Semantic Versioning](https://semver.org/) as described in [docs/versioning.md](docs/versioning.md).

## [Unreleased]

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
