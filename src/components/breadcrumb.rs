use super::Icon;
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbEntry {
    pub label: String,
    pub href: Option<String>,
}
impl BreadcrumbEntry {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
        }
    }
    pub fn page(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }
}
#[component]
pub fn Breadcrumb(
    #[props(default="Breadcrumb".to_string())] label: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {nav {aria_label:label,class:"m3-breadcrumb {class}",{children}}}
}
#[component]
pub fn BreadcrumbList(#[props(default)] class: String, children: Element) -> Element {
    rsx! {ol {class:"m3-breadcrumb__list {class}",{children}}}
}
#[component]
pub fn BreadcrumbItem(#[props(default)] class: String, children: Element) -> Element {
    rsx! {li {class:"m3-breadcrumb__item {class}",{children}}}
}
#[component]
pub fn BreadcrumbLink(href: String, #[props(default)] class: String, children: Element) -> Element {
    rsx! {a {href,class:"m3-breadcrumb__link {class}",{children}}}
}
#[component]
pub fn BreadcrumbPage(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span {aria_current:"page",class:"m3-breadcrumb__page {class}",{children}}}
}
#[component]
pub fn BreadcrumbSeparator(#[props(default)] children: Option<Element>) -> Element {
    rsx! {li {class:"m3-breadcrumb__separator",role:"presentation",aria_hidden:"true",if let Some(children)=children{{children}}else{Icon {icon:icons::CHEVRON_RIGHT}}}}
}
/// Decorative ellipsis. Wrap in a named real button with an accessible Menu or
/// Popover when it represents hidden ancestors; this span is not a control.
#[component]
pub fn BreadcrumbEllipsis() -> Element {
    rsx! {span {class:"m3-breadcrumb__ellipsis",aria_hidden:"true","…"}}
}
/// Data convenience API. Final entry is always the current page; parent links
/// retain native URL/context-menu/new-tab behavior. Missing hrefs are text.
#[component]
pub fn BreadcrumbTrail(
    entries: Vec<BreadcrumbEntry>,
    #[props(default="Breadcrumb".to_string())] label: String,
    #[props(default)] class: String,
) -> Element {
    let last = entries.len().saturating_sub(1);
    rsx! {Breadcrumb {label,class,BreadcrumbList {for (index,entry) in entries.into_iter().enumerate(){
        if index>0 {BreadcrumbSeparator {}}
        BreadcrumbItem {if index==last {BreadcrumbPage {"{entry.label}"}}else if let Some(href)=entry.href{BreadcrumbLink {href,"{entry.label}"}}else{span {"{entry.label}"}}}
    }}}}
}
