# Copy components into a Dioxus project

The gallery's **Copy Rust code** buttons copy real usage examples. Component
implementations remain editable Rust and CSS files in this repository.

## Export the complete source kit

Run from this repository, choosing a new destination:

```sh
python3 scripts/export-kit.py /path/to/my-project/ui-kit
```

The export is an ordinary local Rust library, with all component sources,
semantic theme generation, icon data, loading shape data, motion helpers and
component CSS. It excludes the gallery. Add it to your application's Cargo.toml:

```toml
[dependencies]
m3e-rust-ui = { path = "ui-kit" }
```

Copy the exported CSS into your application's assets directory. Include the
stylesheets for the components you use with Dioxus `document::Stylesheet` and
`asset!`. Interactive components below also require **ripple.css**. For example:

```rust
rsx! {
    document::Stylesheet { href: asset!("/assets/icon.css") }
    document::Stylesheet { href: asset!("/assets/ripple.css") }
    document::Stylesheet { href: asset!("/assets/icon-button.css") }
    IconButton { icon: icons::FAVORITE, aria_label: "Save" }
}
```

Components consume `--md-sys-color-*` properties. Generate them through
`m3e_rust_ui::theme::{ThemePreview, css_variables}` and apply the resulting inline
style on an ancestor, as the gallery does. The exported theme module uses
`material-colors`; the components themselves do not depend on its API.

## Dependencies for a smaller source copy

Keep the module layout shown in `src/components/mod.rs` and exports in
`src/lib.rs`; component imports use `super` and generated data uses `crate`.
If copying only selected files, retain the corresponding module declarations.

| Component Rust file | CSS | Additional Rust source |
| --- | --- | --- |
| `button.rs` | `button.css` | None; leading icons may need Icon |
| `icon.rs` | `icon.css` | `icons.rs` for supplied generated glyphs |
| `icon_button.rs`, `fab.rs` | matching CSS, `icon.css`, `ripple.css` | `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `fab_menu.rs`, `chip.rs`, `segmented_button.rs`, `tabs.rs` | matching CSS, `icon.css`, `ripple.css` | `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `checkbox.rs`, `switch.rs`, `radio.rs`, `card.rs` | matching CSS, `ripple.css` | `ripple.rs`, `motion.rs`; card actions may use Button |
| `progress.rs` | `progress.css` | `motion.rs` |
| `loading_indicator.rs` | `loading-indicator.css` | `motion.rs`, `loading_shapes.rs` |
| `text_field.rs` | `text-field.css`, `icon.css` | `motion.rs`, `icon.rs`, `icons.rs` |
| `divider.rs`, `badge.rs` | matching CSS | Badge anchor icons may use Icon |

Keep the existing notices on generated Material Symbols and shape data.

## Verification scope

The exported library and all 19 gallery usage examples were compiled for
`wasm32-unknown-unknown` in a separate directory on 2026-10-09. This establishes
that the source bundle includes its Rust dependencies and that the examples
use the current API. Runtime styling and interaction checks were performed in
the original live gallery. A consumer still needs to include its chosen CSS,
provide semantic color roles, and supply its own media assets and application
state. See [the fidelity fix report](component-fidelity-fixes.md).
