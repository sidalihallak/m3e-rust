# Consumer example

A complete Dioxus app that imports `m3e-rust-ui` as an ordinary Cargo dependency. It is the
example for [the quick start](../../docs/quick-start.md) with more components: buttons, icons,
text fields, a dialog, tabs, a sidebar, a data table and a message list.

It depends on the crate by path so it always uses the checkout it sits in:

```toml
[dependencies]
m3e-rust-ui = { path = "../.." }
```

To run it from a checkout of this repository:

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.7.10 --locked
cd examples/consumer
dx serve --platform web --addr 127.0.0.1 --port 8081 --open false
```

Open `http://127.0.0.1:8081/`. The "Toggle consumer theme" button switches between light and dark
schemes. The seed is set in `src/main.rs` in `App`.

Outside this repository, copy the app and change the dependency to the released tag:

```toml
m3e-rust-ui = { git = "https://github.com/sidalihallak/m3e-rust", tag = "v0.1.0" }
```
