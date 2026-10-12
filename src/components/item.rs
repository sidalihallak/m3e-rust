use super::{Divider, ListSelection, motion::next_id};
#[derive(Clone)]
struct ItemSelection(Memo<ListSelection>);
use dioxus::prelude::*;
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outlined,
    Muted,
    Segmented,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}
/// Composable list content. Interactive rows use a native button or link inside
/// a listitem wrapper. Put nested actions in a noninteractive Item; never nest
/// controls inside an interactive row. `selected` makes an action a toggle.
#[component]
pub fn Item(
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    #[props(default)] interactive: bool,
    #[props(default)] href: Option<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] selected: Option<bool>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let selection = try_use_context::<ItemSelection>()
        .map(|v| (v.0)())
        .unwrap_or(ListSelection::None);
    let variant = match variant {
        ItemVariant::Default => "default",
        ItemVariant::Outlined => "outlined",
        ItemVariant::Muted => "muted",
        ItemVariant::Segmented => "segmented",
    };
    let size = match size {
        ItemSize::Default => "default",
        ItemSize::Small => "small",
        ItemSize::ExtraSmall => "xs",
    };
    let class = format!("m3-item m3-item--{variant} m3-item--{size} {class}");
    rsx! {div{role:if selection==ListSelection::None{"listitem"}else{"presentation"},class:"m3-item-entry","data-size":size,"data-variant":variant,
     if let Some(href)=href {a{class,href:if disabled{None}else{Some(href)},role:"link",aria_label:label,aria_disabled:disabled,tabindex:if disabled{"-1"}else{"0"},"data-disabled":disabled,"data-selected":selected==Some(true),onclick:move|e|{if disabled{e.prevent_default();}else{onclick.call(e);}},super::ripple::Ripple{},{children}}}
     else if interactive||selected.is_some() {button{class,r#type:"button",disabled,aria_label:label,role:if selection==ListSelection::None{None}else{Some("option")},aria_pressed:if selection==ListSelection::None{selected}else{None},aria_selected:if selection==ListSelection::None{None}else{selected},"data-selected":selected==Some(true),onclick:move|e|{if !disabled{onclick.call(e);}},super::ripple::Ripple{},{children}}}
     else {div{class,"data-disabled":disabled,{children}}}
    }}
}
#[component]
pub fn ItemGroup(
    #[props(default)] label: Option<String>,
    #[props(default)] selection: ListSelection,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-items"));
    let selection_state = use_memo(use_reactive((&selection,), |(value,)| value));
    use_context_provider(|| ItemSelection(selection_state));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("item.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3ItemGroups?.[{id:?}]?.();")).await;
            });
        }
    });
    rsx! {div{id,role:if selection==ListSelection::None{"list"}else{"listbox"},aria_multiselectable:if selection==ListSelection::Multiple{Some("true")}else{None},aria_label:label,class:"m3-item-group {class}",{children}}}
}
#[component]
pub fn ItemSeparator(#[props(default)] class: String) -> Element {
    rsx! {Divider{class:format!("m3-item-separator {class}")}}
}
#[component]
pub fn ItemMedia(
    #[props(default)] variant: ItemMediaVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let variant = match variant {
        ItemMediaVariant::Default => "default",
        ItemMediaVariant::Icon => "icon",
        ItemMediaVariant::Image => "image",
    };
    rsx! {span{class:"m3-item-media m3-item-media--{variant} {class}",{children}}}
}
#[component]
pub fn ItemContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-item-content {class}",{children}}}
}
#[component]
pub fn ItemTitle(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-item-title {class}",{children}}}
}
#[component]
pub fn ItemDescription(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-item-description {class}",{children}}}
}
#[component]
pub fn ItemActions(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-item-actions {class}",{children}}}
}
#[component]
pub fn ItemHeader(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-item-header {class}",{children}}}
}
#[component]
pub fn ItemFooter(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-item-footer {class}",{children}}}
}
