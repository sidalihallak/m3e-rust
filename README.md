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

The floating action button is verified in
[`docs/fab-verification.md`](docs/fab-verification.md), with raw samples in
[`docs/fab-motion-samples.json`](docs/fab-motion-samples.json).

The FAB family (branded, toolbar and menu) is verified in
[`docs/fab-family-verification.md`](docs/fab-family-verification.md).

The icon button is verified in [`docs/icon-button-verification.md`](docs/icon-button-verification.md).

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
