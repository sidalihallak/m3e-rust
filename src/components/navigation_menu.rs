use super::{Icon, Popover, PopupAlign, motion::next_id};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationLink {
    pub label: String,
    pub href: String,
    pub description: Option<String>,
    pub active: bool,
}
impl NavigationLink {
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            description: None,
            active: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationMenuEntry {
    pub label: String,
    pub href: Option<String>,
    pub links: Vec<NavigationLink>,
    pub disabled: bool,
}
impl NavigationMenuEntry {
    pub fn group(label: impl Into<String>, links: Vec<NavigationLink>) -> Self {
        Self {
            label: label.into(),
            href: None,
            links,
            disabled: false,
        }
    }
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            links: vec![],
            disabled: false,
        }
    }
}
/// Native destination links with button-controlled nonmodal link popups. No
/// application-menu roles are assigned to navigation destinations.
#[component]
pub fn NavigationMenu(
    label: String,
    entries: Vec<NavigationMenuEntry>,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| next_id("m3-navigation-menu"));
    let mut open = use_signal(|| None::<usize>);
    rsx! {nav {id:"{id}",aria_label:label,class:"m3-navigation-menu {class}",ul {class:"m3-navigation-menu__list",
     for (index,entry) in entries.into_iter().enumerate(){li {
      if let Some(href)=entry.href.filter(|_|!entry.disabled){a {href,class:"m3-navigation-menu__trigger","{entry.label}"}}
      else {button {id:"{id}-trigger-{index}",r#type:"button",class:"m3-navigation-menu__trigger",disabled:entry.disabled,aria_expanded:open()==Some(index),onclick:move |_|open.set(if open()==Some(index){None}else{Some(index)}),super::ripple::Ripple{},span{"{entry.label}"}Icon {icon:icons::KEYBOARD_ARROW_DOWN}}
       Popover {anchor_id:format!("{id}-trigger-{index}"),title:entry.label,open:open()==Some(index),align:PopupAlign::Start,offset:8.0,class:"m3-navigation-menu__popup",onopenchange:move |value:bool|{if !value&&open()==Some(index){open.set(None);}},
        ul {class:"m3-navigation-menu__links",for link in entry.links {li {a {href:link.href,class:"m3-navigation-menu__link",aria_current:if link.active{Some("page")}else{None},onclick:move |_|open.set(None),span{"{link.label}"}if let Some(description)=link.description{small{"{description}"}}}}}}
       }
      }
     }}
    }}}
}
