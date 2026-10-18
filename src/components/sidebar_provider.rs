use super::{Icon, IconData, Sheet, SheetSide, compact::use_compact, motion::next_id};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct SidebarState {
    expanded: Signal<bool>,
    mobile_open: Signal<bool>,
    compact: Signal<bool>,
    controlled: Signal<bool>,
    onexpandedchange: EventHandler<bool>,
}
impl SidebarState {
    pub fn expanded(self) -> bool {
        (self.expanded)()
    }
    pub fn mobile_open(self) -> bool {
        (self.mobile_open)()
    }
    pub fn compact(self) -> bool {
        (self.compact)()
    }
    pub fn set_expanded(mut self, value: bool) {
        if !(self.controlled)() {
            self.expanded.set(value);
        }
        self.onexpandedchange.call(value);
    }
    pub fn set_mobile_open(mut self, value: bool) {
        self.mobile_open.set(value);
    }
    pub fn toggle(self) {
        if self.compact() {
            self.set_mobile_open(!self.mobile_open());
        } else {
            self.set_expanded(!self.expanded());
        }
    }
}
pub fn use_sidebar() -> SidebarState {
    use_context::<SidebarState>()
}
/// Scope exactly one shortcut owner. Persistence is opt-in and stores only
/// desktop expansion; compact/modal open state is never restored.
#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_expanded: bool,
    #[props(default)] expanded: Option<bool>,
    #[props(default)] persistence_key: Option<String>,
    #[props(default = true)] keyboard_shortcut: bool,
    #[props(default)] onexpandedchange: EventHandler<bool>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut state_expanded = use_signal(|| expanded.unwrap_or(default_expanded));
    let mut mobile_open = use_signal(|| false);
    let compact = use_compact();
    let mut controlled = use_signal(|| expanded.is_some());
    let change = use_callback(move |value| onexpandedchange.call(value));
    let id = use_hook(|| next_id("m3-sidebar-provider"));
    let state = SidebarState {
        expanded: state_expanded,
        mobile_open,
        compact,
        controlled,
        onexpandedchange: change,
    };
    use_context_provider(|| state);
    use_effect(use_reactive((&expanded,), move |(expanded,)| {
        if *controlled.peek() != expanded.is_some() {
            controlled.set(expanded.is_some());
        }
        if let Some(value) = expanded {
            if *state_expanded.peek() != value {
                state_expanded.set(value);
            }
        }
    }));
    use_effect(move || {
        if !compact() && mobile_open() {
            mobile_open.set(false);
        }
    });
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("sidebar_provider.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<String>().await {
                    match message.as_str() {
                        "toggle" => state.toggle(),
                        "restore:true" => state.set_expanded(true),
                        "restore:false" => state.set_expanded(false),
                        _ => {}
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3SidebarProviders?.[{id:?}]?.();"));
        }
    });
    rsx! {div{id,class:"m3-sidebar-provider {class}","data-expanded":state_expanded(),"data-compact":compact(),"data-key":persistence_key.unwrap_or_default(),"data-shortcut":keyboard_shortcut,"data-controlled":expanded.is_some(),{children}}}
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Start,
    End,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Standard,
    Floating,
    Inset,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarCollapse {
    #[default]
    Icon,
    Offcanvas,
    None,
}
#[component]
pub fn SidebarToggle(#[props(default)] disabled: bool) -> Element {
    let state = use_sidebar();
    let expanded = if state.compact() {
        state.mobile_open()
    } else {
        state.expanded()
    };
    rsx! {button{r#type:"button",class:"m3-sidebar-trigger",disabled,aria_expanded:expanded,aria_label:if expanded{"Collapse workspace sidebar"}else{"Expand workspace sidebar"},aria_keyshortcuts:"Control+B Meta+B",onclick:move |_|state.toggle(),super::ripple::Ripple{},Icon{icon:if expanded{icons::MENU_OPEN}else{icons::MENU}}}}
}
/// Compose with SidebarInset beside this panel. End-side is an application
/// option; use the leading edge for primary Material navigation.
#[component]
pub fn SidebarPanel(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapse,
    #[props(default="Workspace navigation".into())] label: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let state = use_sidebar();
    let expanded = state.expanded() || collapsible == SidebarCollapse::None;
    let hidden = !expanded && collapsible == SidebarCollapse::Offcanvas;
    let width = if hidden {
        0
    } else if expanded {
        288
    } else {
        80
    };
    let side_name = if side == SidebarSide::Start {
        "start"
    } else {
        "end"
    };
    let variant_name = match variant {
        SidebarVariant::Standard => "standard",
        SidebarVariant::Floating => "floating",
        SidebarVariant::Inset => "inset",
    };
    let body = rsx! {div{class:"m3-sidebar-panel__body","data-icon-only":!expanded&&!state.compact(),{children}}};
    rsx! {
        if !state.compact(){aside{class:"m3-sidebar-panel {class}",aria_label:label.clone(),"data-side":side_name,"data-variant":variant_name,"data-hidden":hidden,style:"--m3-sidebar-panel-width:{width}px",inert:if hidden{Some("")}else{None},aria_hidden:if hidden{Some("true")}else{None},{body}}}
        else {Sheet{open:state.mobile_open(),title:label,side:if side==SidebarSide::Start{SheetSide::Start}else{SheetSide::End},width:360,class:"m3-sidebar-mobile {class}",onopenchange:move |v|state.set_mobile_open(v),{body}}}
    }
}
#[component]
pub fn SidebarMenu(
    #[props(default="Workspace destinations".into())] label: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-sidebar-menu"));
    super::navigation::use_navigation_keyboard(id.clone());
    rsx! {nav{id,aria_label:label,"data-nav-axis":"vertical",ul{class:"m3-sidebar-menu",{children}}}}
}
#[component]
pub fn SidebarMenuItem(children: Element) -> Element {
    rsx! {li{class:"m3-sidebar-menu-item",{children}}}
}
#[component]
pub fn SidebarMenuButton(
    label: String,
    icon: IconData,
    #[props(default)] active_icon: Option<IconData>,
    #[props(default)] active: bool,
    #[props(default)] expanded: Option<bool>,
    #[props(default)] disabled: bool,
    #[props(default)] badge: Option<u32>,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let name = badge
        .map(|n| format!("{label}, {n} notifications"))
        .unwrap_or(label.clone());
    rsx! {button{r#type:"button",class:"m3-sidebar-menu-button",disabled,"data-nav-destination":"true","data-active":active,aria_label:name,aria_expanded:expanded,aria_current:if active{Some("page")}else{None},onclick,super::ripple::Ripple{},span{class:"m3-sidebar-menu-icon",aria_hidden:"true",Icon{icon:if active{active_icon.unwrap_or(icon)}else{icon}} if let Some(count)=badge{span{class:"m3-sidebar-icon-badge",super::NotificationBadge{count}}}}span{class:"m3-sidebar-menu-label","{label}"}if let Some(count)=badge{span{class:"m3-sidebar-label-badge",aria_hidden:"true",super::NotificationBadge{count}}}}}
}
#[component]
pub fn SidebarMenuAction(
    label: String,
    icon: IconData,
    #[props(default)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {button{r#type:"button",class:"m3-sidebar-menu-action",aria_label:label,disabled,onclick,super::ripple::Ripple{},Icon{icon}}}
}
#[component]
pub fn SidebarMenuSub(children: Element) -> Element {
    rsx! {ul{class:"m3-sidebar-menu-sub",{children}}}
}
#[component]
pub fn SidebarSeparator() -> Element {
    rsx! {super::Divider{}}
}
#[component]
pub fn SidebarInput(
    label: String,
    #[props(default)] value: String,
    #[props(default)] disabled: bool,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-sidebar-input"));
    rsx! {div{class:"m3-sidebar-extra",super::Label{html_for:id.clone(),"{label}"}super::Input{id,aria_label:label,value,disabled,oninput}}}
}
#[component]
pub fn SidebarMenuSkeleton(#[props(default = 3)] rows: usize) -> Element {
    rsx! {div{class:"m3-sidebar-extra",aria_busy:"true",aria_label:"Loading navigation",for _ in 0..rows.min(10){super::Skeleton{width:"100%",height:48}}}}
}
