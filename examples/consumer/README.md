# Consumer example

An ordinary Dioxus app that uses `m3e-rust-ui` as a normal Cargo dependency:

```toml
[dependencies]
m3e-rust-ui = { path = "../.." }   # or the crates.io / git version
```

```rust
use m3e_rust_ui::{ThemeProvider, ThemeSettings};

rsx! {
    ThemeProvider {
        default_theme: ThemeSettings { seed: 0xff006a60, ..Default::default() },
        App {}
    }
}
```

`ThemeProvider` injects the component styles and builds the Material scheme from the
seed colour. Components are used as they are; no source is copied into the app.

```sh
cd examples/consumer
dx serve --platform web --addr 127.0.0.1 --port 8081 --open false
```
