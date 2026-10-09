use dioxus::prelude::*;

/// One Material Symbols glyph: its name and the path data on the 960-unit grid.
///
/// Glyphs come from [`crate::icons`], generated from the official
/// `@material-symbols/svg-400` package by `scripts/gen-icons.mjs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IconData {
    pub name: &'static str,
    pub path: &'static str,
}

/// Renders a Material Symbols glyph as an inline `<svg>` filled with `currentColor`.
///
/// Size comes from `--m3-icon-size` (24px by default) so a parent can set it.
/// Pass `aria_label` when the icon carries meaning on its own; otherwise it is
/// hidden from assistive technology.
#[component]
pub fn Icon(
    icon: IconData,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let class = format!("m3-icon {class}");
    let role = aria_label.as_ref().map(|_| "img");
    let hidden = if aria_label.is_some() { None } else { Some("true") };
    rsx! {
        svg {
            class,
            view_box: "0 -960 960 960",
            role,
            "aria-label": aria_label,
            "aria-hidden": hidden,
            "data-icon": icon.name,
            path { d: icon.path }
        }
    }
}
