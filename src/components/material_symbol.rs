use dioxus::prelude::*;

/// Material Symbols style. Static SVGs use weight 400 and the official 960 grid.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum SymbolStyle {
    #[default]
    Rounded,
    Outlined,
    Sharp,
}
impl SymbolStyle {
    pub fn module(self) -> &'static str {
        match self {
            Self::Rounded => "rounded",
            Self::Outlined => "outlined",
            Self::Sharp => "sharp",
        }
    }
}

/// Dynamic name renderer using local SVG sprites, without an external font/CDN.
/// For a statically known icon, prefer Icon with icons::rounded::NAME, which
/// embeds only the referenced path. This renderer loads a complete style sprite.
#[component]
pub fn MaterialSymbol(
    name: String,
    #[props(default)] style: SymbolStyle,
    #[props(default)] filled: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let valid = crate::icons::SYMBOL_NAMES
        .binary_search(&name.as_str())
        .is_ok();
    let sprite = match style {
        SymbolStyle::Rounded => asset!("/assets/material-symbols/rounded.svg"),
        SymbolStyle::Outlined => asset!("/assets/material-symbols/outlined.svg"),
        SymbolStyle::Sharp => asset!("/assets/material-symbols/sharp.svg"),
    };
    let href = format!(
        "{sprite}#{}-{name}{}",
        style.module(),
        if filled { "-fill" } else { "" }
    );
    rsx! {for reference in [href] {svg {
        key:"{reference}",
        class:"m3-icon {class}",view_box:"0 -960 960 960",
        role:aria_label.as_ref().map(|_|"img"),"aria-hidden":if aria_label.is_none(){Some("true")}else{None},"aria-label":aria_label.clone(),
        "data-icon":name.clone(),"data-symbol-style":style.module(),"data-symbol-filled":filled,"data-symbol-valid":valid,
        // Key the entire SVG in a dynamic list to recreate its external use tree.
        if valid {r#use {href:reference,x:"0",y:"-960",width:"960",height:"960"}} else {path {d:crate::icons::HELP.path}}
    }}}
}
