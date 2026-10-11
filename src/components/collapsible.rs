use super::{accordion::use_disclosure_runtime, motion::next_id};
use dioxus::prelude::*;
/// Unstyled disclosure primitive, matching upstream's unstyled Collapsible.
/// Add app-specific trigger/content classes without nesting interactive children.
#[component]
pub fn Collapsible(
    label: String,
    open: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] trigger_class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-collapsible"));
    use_disclosure_runtime(id.clone());
    let panel = format!("{id}-panel");
    let trigger = format!("{id}-trigger");
    rsx! {div {id,class:"m3-collapsible {class}",
        button {id:trigger.clone(),r#type:"button",class:"m3-collapsible__trigger {trigger_class}",disabled,aria_expanded:open,aria_controls:panel.clone(),onclick:move |_|onopenchange.call(!open),"{label}"}
        div {id:panel,class:"m3-disclosure__panel","data-open":open,"data-animate":"false",aria_labelledby:trigger,aria_hidden:!open,inert:if !open{Some("")}else{None},div {class:"m3-disclosure__inner",{children}}}
    }}
}
