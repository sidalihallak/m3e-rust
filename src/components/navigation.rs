use super::motion::next_id;
use super::{BadgeAnchor, Icon, IconButton, IconData, NotificationBadge};
use crate::icons;
const MODAL_SCRIPT: &str = include_str!("modal_navigation.js");
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavigationLayout {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NavigationDestination {
    pub value: String,
    pub label: String,
    pub icon: IconData,
    pub active_icon: IconData,
    pub disabled: bool,
    pub badge: Option<u32>,
}
impl NavigationDestination {
    pub fn new(
        value: impl Into<String>,
        label: impl Into<String>,
        icon: IconData,
        active_icon: IconData,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon,
            active_icon,
            disabled: false,
            badge: None,
        }
    }
}

pub(crate) fn use_navigation_keyboard(id: String) {
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("navigation.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop(move || {
        let script = format!("window.__m3Navigation?.[{id:?}]?.();");
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    });
}

#[component]
fn Destination(
    item: NavigationDestination,
    active: bool,
    horizontal: bool,
    onchange: EventHandler<String>,
) -> Element {
    let label = if let Some(count) = item.badge {
        format!("{}, {} notifications", item.label, count)
    } else {
        item.label.clone()
    };
    let glyph = if active { item.active_icon } else { item.icon };
    rsx! {button {r#type:"button",class:"m3-nav__item","data-nav-destination":"true","data-active":active,
        "data-horizontal":horizontal,disabled:item.disabled,aria_label:label,
        aria_current:if active {Some("page")}else{None},onclick:move |_|onchange.call(item.value.clone()),
        super::ripple::Ripple {}
        span {class:"m3-nav__icon",aria_hidden:"true",if !horizontal {span {class:"m3-nav__indicator"}}
            BadgeAnchor {Icon {icon:glyph} if let Some(count)=item.badge {NotificationBadge {count}}}}
        if horizontal {span {class:"m3-nav__indicator",aria_hidden:"true"}}
        span {class:"m3-nav__label","{item.label}"}
    }}
}

/// Three to five top-level destinations on compact/medium windows. Native
/// buttons retain Tab and Enter/Space; arrows move focus without selecting.
#[component]
pub fn NavigationBar(
    items: Vec<NavigationDestination>,
    selected: String,
    #[props(default)] layout: NavigationLayout,
    #[props(default)] tall: bool,
    #[props(default)] elevated: bool,
    #[props(default = "Main navigation".to_string())] aria_label: String,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-nav-bar"));
    use_navigation_keyboard(id.clone());
    rsx! {nav {id,class:"m3-nav-bar {class}",aria_label,"data-nav-axis":"horizontal","data-tall":tall,"data-elevated":elevated,
        for item in items {Destination {active:item.value==selected,item,horizontal:layout==NavigationLayout::Horizontal,onchange}}
    }}
}

/// Expressive collapsed/expanded rail. Expanded destination targets span the
/// container; this port uses the officially permitted full-width pill style.
#[component]
pub fn NavigationRail(
    items: Vec<NavigationDestination>,
    selected: String,
    #[props(default)] expanded: bool,
    #[props(default)] narrow: bool,
    #[props(default = 220)] expanded_width: u16,
    #[props(default)] header: Option<Element>,
    #[props(default = "Main navigation".to_string())] aria_label: String,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-nav-rail"));
    use_navigation_keyboard(id.clone());
    let width = expanded_width.clamp(220, 360);
    rsx! {nav {id,class:"m3-nav-rail {class}",style:"--m3-rail-width:{width}px",aria_label,
        "data-nav-axis":"vertical","data-expanded":expanded,"data-narrow":narrow,
        if let Some(header)=header {div {class:"m3-nav-rail__header",{header}}}
        for item in items {Destination {active:item.value==selected,item,horizontal:expanded,onchange}}
    }}
}

/// Controlled modal expanded rail with boundary Tab wrapping. Native dialog supplies focus containment
/// and background inertness; Escape/outside dismiss through `onopenchange`.
#[component]
pub fn ModalNavigationRail(
    open: bool,
    items: Vec<NavigationDestination>,
    selected: String,
    #[props(default = "Navigation".to_string())] title: String,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onchange: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-modal-rail"));
    use_effect({
        let id = id.clone();
        move || {
            let script = MODAL_SCRIPT.replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while eval.recv::<bool>().await.is_ok() {
                    onopenchange.call(false);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let script = format!("window.__m3ModalRails?.[{id:?}]?.();");
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    rsx! {dialog {id:"{id}",class:"m3-modal-rail {class}",aria_modal:"true",aria_labelledby:"{id}-title","data-open":open,
        div {class:"m3-modal-rail__heading",IconButton {icon:icons::MENU_OPEN,aria_label:"Close navigation",onclick:move |_|onopenchange.call(false)}h2 {id:"{id}-title","{title}"}}
        NavigationRail {items,selected,expanded:true,expanded_width:320,aria_label:title,
            onchange:move |value|{onchange.call(value);onopenchange.call(false);}}
    }}
}
