# Quick start

This guide creates a Dioxus web app that uses `m3e-rust-ui` as an ordinary Cargo dependency.
The app in step 2 was built with `dx build` and checked in a browser. The install commands in step 1 were not run again from scratch for this guide (see [Verify the crate](#verify-the-crate)).

## 1. Install the tools

You need Rust 1.97 or newer, the WebAssembly target, and the Dioxus CLI. The CLI version must
match the Dioxus version the crate uses, which is 0.7.10.

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked
```

Check the CLI with `dx --version`. It should print `dioxus 0.7.10`.

## 2. Create the app

```sh
cargo new my-app
cd my-app
```

Replace `Cargo.toml` with:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { version = "0.7", features = ["web"] }
m3e-rust-ui = { git = "https://github.com/sidalihallak/m3e-rust", tag = "v0.1.0" }
```

Create `Dioxus.toml`:

```toml
[application]
name = "my-app"
default_platform = "web"
```

Replace `src/main.rs` with:

```rust
use dioxus::prelude::*;
use m3e_rust_ui::{Button, ButtonVariant, ColorMode, ThemeProvider, ThemeSettings};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut clicks = use_signal(|| 0);
    rsx! {
        ThemeProvider {
            default_theme: ThemeSettings { seed: 0xff6750a4, mode: ColorMode::System, ..Default::default() },
            main { style: "padding: 24px; display: grid; gap: 16px; justify-items: start;",
                h1 { "Hello, Material 3" }
                Button { variant: ButtonVariant::Filled, onclick: move |_| clicks += 1, "Clicked {clicks} times" }
            }
        }
    }
}
```

## 3. Run it

```sh
dx serve --platform web
```

Open `http://127.0.0.1:8080/`. The button is the Material filled style in the color generated from
the seed `0xff6750a4`. Change the seed, the mode or the variant in `ThemeSettings` to change the theme.

## 4. Build for release

```sh
dx build --platform web --release
```

The output is in `target/dx/my-app/release/web/public`.

## Choose a theme

| Setting | Values | Effect |
| --- | --- | --- |
| `seed` | an ARGB color, such as `0xff006a60` | the source color for all Material roles |
| `variant` | `TonalSpot` (default), and the other Material variants | the palette style |
| `contrast` | `ThemeContrast::Standard` (default) and the others | contrast level |
| `mode` | `ColorMode::Light`, `Dark`, `System` (default) | which scheme is shown |

To change the theme at runtime, read the settings with `use_theme()` and update them:

```rust
let mut theme = use_theme();
theme.with_mut(|t| t.mode = ColorMode::Dark);
```

Put a component inside `ThemeProvider`. The provider writes the roles on its own element, so a
`--md-sys-color-*` override set above it is replaced. Set overrides on an element inside it.

## Use the components

Every component is exported from the crate root:

```rust
use m3e_rust_ui::{Card, CardActions, CardBody, Button, ButtonVariant, TextField};
```

The [component list and verification notes](../README.md) show each component and its usage.
[`llms.txt`](../llms.txt) lists the same API for coding agents.

## Verify the crate

The checks used for this release:

```sh
cargo check --locked --offline --target wasm32-unknown-unknown
```

From a checkout of this repository, the consumer example is the same app as above, with more
components: [`examples/consumer`](../examples/consumer/README.md).
