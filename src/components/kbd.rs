use dioxus::prelude::*;
/// Static keyboard hint. The consuming application owns shortcut bindings.
/// `key_label` gives glyphs such as⌘ a readable name without adding a focus stop.
#[component]
pub fn Kbd(
    #[props(default)] key_label: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! { kbd {class:"m3-kbd {class}",
        if let Some(label)=key_label {span{aria_hidden:"true",{children}}span{class:"m3-helper-sr-only","{label}"}}
        else {{children}}
    } }
}
#[component]
pub fn KbdGroup(#[props(default)] class: String, children: Element) -> Element {
    rsx! { kbd {class:"m3-kbd-group {class}",{children}} }
}
