# M3E Rust UI

An M3 Expressive component kit for Dioxus, built on the Rust/UI registry model.

[Live preview](https://sidalihallak.github.io/m3e-rust/) ·
[Source](https://github.com/sidalihallak/m3e-rust)

Agent contributors must follow [AGENTS.md](AGENTS.md) and the
[component development protocol](docs/component-development.md). Every
component gets a reference comparison and a recorded verification report.

This is a fresh project. The existing `rskit-ui` implementation is intentionally out of scope and has not been modified.

## Color foundation pilot

The preview generates light and dark Material 3 Tonal Spot schemes from a seed color with [`material-colors`](https://crates.io/crates/material-colors), an independent Rust port of Material Color Utilities. Its algorithms stay behind [`src/theme.rs`](src/theme.rs), which adapts the generated roles to CSS custom properties. Tailwind v4 maps those properties to semantic utilities such as `bg-primary` and `text-on-primary` in [`tailwind.css`](tailwind.css).

This keeps the component layer independent from the palette-generation crate. The current preview covers seed switching, light/dark schemes, tonal palettes, and a small set of components using semantic roles. The selected crate currently requires Rust 1.97 or newer.

## Button component pilot

[`src/components/button.rs`](src/components/button.rs) exposes `Button` from the crate root with `ButtonVariant`, `ButtonSize`, `ButtonShape`, and `ButtonType` props, leading-icon support, custom classes, disabled state, and toggle selection. Include [`assets/button.css`](assets/button.css) in Dioxus apps that use it. The implementation follows the [M3 button overview](https://m3.material.io/components/buttons/overview), [specs](https://m3.material.io/components/buttons/specs), [guidelines](https://m3.material.io/components/buttons/guidelines), and [accessibility](https://m3.material.io/components/buttons/accessibility) pages.

## Prerequisites

- Rust toolchain
- Dioxus CLI (`dx`)
- `wasm32-unknown-unknown` target for web builds

## Run

The pilot already has an isolated Rust toolchain and Dioxus CLI under
`/private/tmp/m3e-*`. The development script selects them when present:

```sh
./scripts/dev.sh
```

It serves the app at `http://127.0.0.1:8080/` with live rebuilds. After a system
restart clears the temporary tools, install the prerequisites normally:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli
dx serve
```

Switch and checkbox interaction and motion checks are recorded in
[`docs/switch-verification.md`](docs/switch-verification.md) and
[`docs/checkbox-verification.md`](docs/checkbox-verification.md), with raw
samples in [`docs/control-motion-samples.json`](docs/control-motion-samples.json).

The floating action button evidence and scope are recorded in
[`docs/fab-verification.md`](docs/fab-verification.md), with raw samples in
[`docs/fab-motion-samples.json`](docs/fab-motion-samples.json).

The FAB family (branded, toolbar and menu) evidence and scope are recorded in
[`docs/fab-family-verification.md`](docs/fab-family-verification.md).

The icon button evidence and scope are recorded in [`docs/icon-button-verification.md`](docs/icon-button-verification.md).

The chip evidence and scope are recorded in [`docs/chip-verification.md`](docs/chip-verification.md).

Progress indicators evidence and scope are recorded in [`docs/progress-verification.md`](docs/progress-verification.md).

Radio buttons evidence and scope are recorded in [`docs/radio-verification.md`](docs/radio-verification.md).

The slider evidence and scope are recorded in [`docs/slider-verification.md`](docs/slider-verification.md).

Segmented buttons evidence and scope are recorded in [`docs/segmented-button-verification.md`](docs/segmented-button-verification.md).

Tabs evidence and scope are recorded in [`docs/tabs-verification.md`](docs/tabs-verification.md).

Notification badges evidence and scope are recorded in [`docs/badge-verification.md`](docs/badge-verification.md).

Cards evidence and scope are recorded in [`docs/card-verification.md`](docs/card-verification.md).

Dividers evidence and scope are recorded in [`docs/divider-verification.md`](docs/divider-verification.md).

Text fields evidence and scope are recorded in [`docs/text-field-verification.md`](docs/text-field-verification.md).

Button motion measurements and reference comparisons are recorded in
[`docs/button-motion-verification.md`](docs/button-motion-verification.md).

Android emulator setup and mobile verification status are recorded in
[`docs/android-preview.md`](docs/android-preview.md).

The project is configured for the web platform and includes Tailwind CSS v4 input at the repository root. Dioxus CLI manages the Tailwind watcher when serving the app.

## GitHub Pages

Pushes to `main` build and deploy the preview through
[the Pages workflow](.github/workflows/pages.yml). The workflow uses Rust 1.99.0,
Dioxus CLI 0.7.10 and the repository's `Cargo.lock`. It passes the Pages base
path at build time, so local development continues to use `/`.

To produce the same web build locally:

```sh
dx build --platform web --release --locked --base-path /m3e-rust --debug-symbols false
```

Static files are emitted to `target/dx/m3e-rust-ui/release/web/public` and are
uploaded as a Pages artifact; generated build files are excluded from Git.

## Fidelity fixes and source copying

Current desktop fixes and platform scope are in
[the fidelity fix report](docs/component-fidelity-fixes.md).
[Loading indicator evidence](docs/loading-indicator-verification.md) is recorded
separately. Follow [the copy guide](docs/copy-components.md) for complete Rust,
CSS, icon, shape and motion dependencies, or export an editable local library:

```sh
python3 scripts/export-kit.py /path/to/my-project/ui-kit
```

Current verification gaps, optional behaviors and the suggested port order are
listed in [remaining work](docs/remaining-work.md).

## Accessibility and connected groups

[Accessibility audit](docs/accessibility-verification.md) records keyboard, semantic
contrast and actual macOS Reduce Motion results, fixes, and screen-reader gaps.
[Connected button groups](docs/button-group-verification.md) now support single
and multiple selection, required selection, disabled options, five sizes, two
shapes, four container styles, icons and RTL-aware keyboard focus. The live gallery
includes a copyable usage example. Android verification remains paused.

## Groups, split buttons, dialogs and menus

[Standard groups](docs/standard-button-group-verification.md),
[split buttons](docs/split-button-verification.md),
[dialogs and alerts](docs/dialog-verification.md), and
[dropdown/context menus](docs/menu-verification.md) now have copyable Dioxus APIs
and live examples. Their [verification index](docs/composite-verification.md)
records reference choices, desktop motion and keyboard checks, build/export
results and remaining platform gaps. All 27 usage examples compile independently.

Current desktop follow-up: [card](docs/card-verification.md),
[divider](docs/divider-verification.md), [dialog](docs/dialog-verification.md),
[field](docs/text-field-verification.md) and [raw browser frames](docs/resumed-browser-verification.json).

## Help and selection

[Family verification](docs/help-selection-verification.md) covers plain/rich
tooltips, popover, hover card, outlined/filled select, native select, editable
single/multiple combobox and free text autocomplete. Eight copyable examples
are in the live gallery; all **35** source-export examples compile. Desktop
keyboard/pointer, popup motion, actual Reduce Motion and 390px responsive checks
are recorded separately from the paused Android work.

## Navigation and showcase

[Navigation verification](docs/navigation-verification.md) covers
[app bars](docs/app-bar-verification.md), [navigation bars](docs/navigation-bar-verification.md),
[rails](docs/navigation-rail-verification.md), [modal rails](docs/modal-navigation-rail-verification.md)
and [toolbars](docs/toolbar-verification.md). The showcase now has six families,
28 component pages, a searchable desktop catalogue and compact navigation.
Five new copyable examples bring the independent source-export total to **40**.
The report distinguishes earlier desktop observations from final untested edits.
