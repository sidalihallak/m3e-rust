# Copy components into a Dioxus project

Desktop copy/paste checked on 2026-10-10: nine composite/card/divider controls
showed success, and native paste exactly matched the 509-character Divider usage.
The updated export includes 16px card slot padding and the dialog focus/label
paint gutter. API/source completeness is checked separately by compiling all
27 exported examples; runtime styling in a separate consuming app remains open.
See [current browser evidence](resumed-browser-verification.json).

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
| `standard_button_group.rs` | `standard-button-group.css`, `action-control.css`, `composite-motion.css`, `ripple.css`, `icon.css` | `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `split_button.rs` | `split-button.css` and all menu/shared action CSS below | `menu.rs` + `menu.js`, `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `menu.rs` | `menu.css`, `action-control.css`, `composite-motion.css`, `ripple.css`, `icon.css` | **`menu.js` in the same directory**, `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `dialog.rs` | `dialog.css`, `action-control.css`, `composite-motion.css`, `ripple.css`, `icon.css` | **`dialog.js` in the same directory**, `compact.rs`, `action_control.rs`, `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
| `button_group.rs` | `button-group.css`, `ripple.css`, `icon.css` | `button.rs` enums, `icon.rs`, `icons.rs`, `ripple.rs`, `motion.rs` |
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

The exported library and all 27 gallery usage examples were compiled for
`wasm32-unknown-unknown` in a separate directory on 2026-10-10. This establishes
that the source bundle includes its Rust dependencies and that the examples
use the current API. Runtime styling and interaction checks were performed in
the original live gallery. A consumer still needs to include its chosen CSS,
provide semantic color roles, and supply its own media assets and application
state. See [the fidelity fix report](component-fidelity-fixes.md).

`TextField` fills its parent width. Constrain the containing layout for narrower
fields; dialog content automatically receives the full available field width.

Menus and dialogs include their JavaScript runtime through Rust `include_str!`;
retain those files when making a smaller copy. The complete exporter includes
them automatically. The shared action styles use finite shape radii, semantic
roles and native button semantics. Their motion stylesheet supplies the actual
spring curves used by the popup runtime. See [composite evidence](composite-verification.md).

`StandardButtonGroup`, `SplitButton`, `DropdownMenu`, `ContextMenu`, `Dialog` and
`AlertDialog` expose controlled state: update it in `onchange`/`onopenchange`.
For a bare `Menu`, supply a stable existing `anchor_id`. Menu selection reports a
path through `MenuEntry` indices; checkbox selection stays open. Your app owns
radio exclusivity, validation and side effects. Use `DialogVariant::Adaptive` for
compact full-screen/desktop basic presentation. Intercept its `onopenchange`
before discarding an unsaved draft. Include additional component CSS (for example
TextField) when using those controls in dialog content.

For text-only tab panels, include `tabindex: "0"` and a visible focus style.
Give range sliders an `aria_label`. Badge `aria_label` supplies full announcement
context; copy `badge.css` too so its announcement text is visually hidden.
