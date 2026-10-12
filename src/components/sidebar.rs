use super::{ModalNavigationRail, NavigationDestination, NavigationRail, compact::use_compact};
use crate::icons;
use dioxus::prelude::*;
/// Controlled application navigation helper. Desktop uses an expanded/collapsed
/// expressive rail; compact windows use its existing modal alternative. Cookie
/// persistence and global shortcuts belong to the consuming app.
#[component]
pub fn Sidebar(
    items: Vec<NavigationDestination>,
    selected: String,
    expanded: bool,
    mobile_open: bool,
    onexpandedchange: EventHandler<bool>,
    onmobileopenchange: EventHandler<bool>,
    onchange: EventHandler<String>,
    #[props(default="Workspace navigation".to_string())] label: String,
    #[props(default)] header: Option<Element>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] class: String,
) -> Element {
    let compact = use_compact();
    rsx! {div {class:"m3-sidebar {class}","data-expanded":expanded,"data-compact":compact(),
        SidebarTrigger {expanded:if compact(){mobile_open}else{expanded},onclick:move |_|{if compact(){onmobileopenchange.call(!mobile_open);}else{onexpandedchange.call(!expanded);}}}
        if !compact(){
            if expanded {if let Some(header)=header{SidebarHeader {{header}}}}
            NavigationRail {items:items.clone(),selected:selected.clone(),expanded,narrow:true,expanded_width:288,aria_label:label.clone(),onchange}
            if expanded {if let Some(footer)=footer{SidebarFooter {{footer}}}}
        }
        ModalNavigationRail {title:label,items,selected,open:compact()&&mobile_open,onopenchange:onmobileopenchange,onchange}
    }}
}
#[component]
pub fn SidebarTrigger(
    expanded: bool,
    onclick: EventHandler<MouseEvent>,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {button {r#type:"button",class:"m3-sidebar-trigger",disabled,aria_expanded:expanded,aria_label:if expanded{"Collapse sidebar"}else{"Expand sidebar"},onclick,super::ripple::Ripple{},super::Icon {icon:if expanded{icons::MENU_OPEN}else{icons::MENU}}}}
}
#[component]
pub fn SidebarLayout(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div {class:"m3-sidebar-layout {class}",{children}}}
}
#[component]
pub fn SidebarInset(#[props(default)] class: String, children: Element) -> Element {
    rsx! {section {class:"m3-sidebar-inset {class}",{children}}}
}
#[component]
pub fn SidebarHeader(children: Element) -> Element {
    rsx! {div {class:"m3-sidebar__header",{children}}}
}
#[component]
pub fn SidebarFooter(children: Element) -> Element {
    rsx! {div {class:"m3-sidebar__footer",{children}}}
}
#[component]
pub fn SidebarGroup(label: String, children: Element) -> Element {
    rsx! {section {class:"m3-sidebar__group",aria_label:label.clone(),h3 {"{label}"}{children}}}
}
#[component]
pub fn SidebarContent(children: Element) -> Element {
    rsx! {div {class:"m3-sidebar__content",{children}}}
}
