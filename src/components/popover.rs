use super::{
    action_control::ActionControl,
    anchored::{use_popup_runtime, PopupAlign, PopupSide},
    motion::next_id,
    ButtonVariant, IconData,
};
use dioxus::prelude::*;

/// Native action with a caller-supplied stable ID for help popup anchoring.
#[component]
pub fn HelpTrigger(
    id: String,
    label: String,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] icon_only: bool,
    #[props(default = ButtonVariant::Tonal)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let content = rsx! {ActionControl {id,label,icon,icon_only,variant,disabled,onclick}};
    content
}

/// Controlled nonmodal anchored dialog. Put it immediately after its trigger in
/// DOM order. The runtime adds expanded/controls/haspopup semantics to the anchor.
/// Escape restores focus; outside activation and Tab leave without trapping.
#[component]
pub fn Popover(
    anchor_id: String,
    title: String,
    open: bool,
    #[props(default)] description: Option<String>,
    #[props(default)] side: PopupSide,
    #[props(default)] align: PopupAlign,
    #[props(default = 4.0)] offset: f64,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-popover"));
    let title_id = format!("{id}-title");
    let description_id = format!("{id}-description");
    use_popup_runtime(
        id.clone(),
        "",
        EventHandler::new(move |m: Vec<String>| {
            if m.first().is_some_and(|s| s == "open") {
                onopenchange.call(m.get(1).is_some_and(|s| s == "true"));
            }
        }),
    );
    rsx! { div { id:"{id}", class:"m3-anchor m3-popover {class}", role:"dialog", tabindex:"-1", aria_labelledby:title_id.clone(),
        aria_describedby:description.as_ref().map(|_|description_id.clone()), "popover":"manual",
        "data-kind":"popover", "data-anchor":anchor_id, "data-open":open, "data-side":side.class(), "data-align":align.class(), "data-offset":offset,
        div { class:"m3-help-header", h3 { id:title_id, class:"m3-help-title", "{title}" }
            if let Some(text)=&description { p { id:description_id.clone(), "{text}" } }
        }
        {children}
    }}
}
