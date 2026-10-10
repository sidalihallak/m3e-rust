use super::{
    anchored::{use_popup_runtime, PopupAlign, PopupSide},
    motion::next_id,
};
use dioxus::prelude::*;

/// Supplementary plain tooltip for an existing mounted control. Give the control
/// its own accessible name; `anchor_id` must be unique and stable. No actions here.
/// Hover delay matches upstream (500ms); the 1500ms leave delay follows Material.
#[component]
pub fn Tooltip(
    anchor_id: String,
    text: String,
    #[props(default = PopupSide::Top)] side: PopupSide,
    #[props(default)] align: PopupAlign,
    #[props(default = 4.0)] offset: f64,
    #[props(default = 500)] delay: u32,
    #[props(default = 1500)] close_delay: u32,
    #[props(default)] disabled: bool,
) -> Element {
    let id = use_hook(|| next_id("m3-tooltip"));
    use_popup_runtime(id.clone(), "", EventHandler::new(|_| {}));
    rsx! { div { id: "{id}", class: "m3-anchor m3-tooltip", role: "tooltip", "popover": "manual",
        "data-kind": "tooltip", "data-anchor": anchor_id, "data-side": side.class(), "data-align": align.class(),
        "data-offset": offset, "data-delay": delay, "data-close-delay": close_delay, "data-disabled": disabled,
        "{text}"
    } }
}

/// Persistent rich tooltip opened by an explicit labeled action. It is a named
/// nonmodal dialog so links/actions remain accessible; focus is never trapped.
/// Keep essential information on the page. Material allows up to two text actions.
#[component]
pub fn RichTooltip(
    anchor_id: String,
    title: String,
    open: bool,
    #[props(default)] side: PopupSide,
    #[props(default)] align: PopupAlign,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default = rsx! {})] actions: Element,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-rich-tooltip"));
    let title_id = format!("{id}-title");
    use_popup_runtime(
        id.clone(),
        "",
        EventHandler::new(move |m: Vec<String>| {
            if m.first().is_some_and(|s| s == "open") {
                onopenchange.call(m.get(1).is_some_and(|s| s == "true"));
            }
        }),
    );
    rsx! { div { id:"{id}", class:"m3-anchor m3-rich-tooltip", role:"dialog", aria_labelledby:title_id.clone(), tabindex:"-1", "popover":"manual",
        "data-kind":"rich", "data-anchor":anchor_id, "data-open":open, "data-side":side.class(), "data-align":align.class(), "data-offset":"4",
        h3 { id:title_id, class:"m3-help-title", "{title}" }
        div { class:"m3-help-body", {children} }
        div { class:"m3-rich-tooltip__actions", {actions} }
    }}
}
