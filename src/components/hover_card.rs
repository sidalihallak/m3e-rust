use super::{
    anchored::{use_popup_runtime, PopupAlign, PopupSide},
    motion::next_id,
};
use dioxus::prelude::*;

/// Supplementary preview for an existing link. Following Base UI PreviewCard,
/// the preview is hidden from assistive technology and has no focusable actions.
/// All information must also exist at the linked destination. Use Popover for
/// interactive or essential content. Keyboard focus may show the visual preview.
#[component]
pub fn HoverCard(
    anchor_id: String,
    #[props(default)] side: PopupSide,
    #[props(default)] align: PopupAlign,
    #[props(default = 600)] delay: u32,
    #[props(default = 300)] close_delay: u32,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-hover-card"));
    use_popup_runtime(id.clone(), "", EventHandler::new(|_| {}));
    rsx! { div { id:"{id}", class:"m3-anchor m3-hover-card", "popover":"manual", aria_hidden:"true",
        "data-kind":"hover", "data-anchor":anchor_id, "data-side":side.class(), "data-align":align.class(), "data-offset":"4", "data-align-offset":"4",
        "data-delay":delay, "data-close-delay":close_delay,
        div { "inert":"", {children} }
    }}
}
