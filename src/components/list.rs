use super::{Icon, motion::next_id};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListStyle {
    Standard,
    #[default]
    Segmented,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListSelection {
    #[default]
    None,
    Single,
    Multiple,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListLines {
    #[default]
    One,
    Two,
    Three,
}
#[component]
pub fn List(
    aria_label: String,
    #[props(default)] style: ListStyle,
    #[props(default)] selection: ListSelection,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-list"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("list.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Lists?.[{id:?}]?.();"));
        }
    });
    rsx! {div {id,class:"m3-list {class}",role:if selection==ListSelection::None{"list"}else{"listbox"},aria_label,aria_multiselectable:if selection==ListSelection::Multiple{Some("true")}else{None},"data-style":if style==ListStyle::Segmented{"segmented"}else{"standard"},{children}}}
}
/// Decorative slots belong inside the primary action. Secondary controls are
/// siblings, never nested buttons, and are only supported for nonselection rows.
#[component]
pub fn ListItem(
    label: String,
    #[props(default)] supporting: Option<String>,
    #[props(default)] expanded: Option<bool>,
    #[props(default)] expanded_content: Option<Element>,
    #[props(default)] onexpandedchange: EventHandler<bool>,
    #[props(default)] overline: Option<String>,
    #[props(default)] trailing_text: Option<String>,
    #[props(default)] leading: Option<Element>,
    #[props(default)] trailing: Option<Element>,
    #[props(default)] secondary: Option<Element>,
    #[props(default)] lines: ListLines,
    #[props(default)] interactive: bool,
    #[props(default)] selected: Option<bool>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onactivate: EventHandler<()>,
) -> Element {
    let id=use_hook(||next_id("m3-list-row"));
    super::accordion::use_disclosure_runtime(id.clone());
    let expandable=selected.is_none()&&expanded_content.is_some()&&expanded.is_some();
    let actionable = interactive || selected.is_some() || expandable;
    let line = if lines == ListLines::Three || overline.is_some() {
        3
    } else if lines == ListLines::Two || supporting.is_some() {
        2
    } else {
        1
    };
    let content = rsx! {
        if let Some(leading)=leading {span {class:"m3-list__leading",{leading}}}
        span {class:"m3-list__text",if let Some(overline)=overline {span {class:"m3-list__overline","{overline}"}}span {class:"m3-list__label","{label}"}if let Some(text)=supporting {span {class:"m3-list__supporting","{text}"}}}
        if let Some(text)=trailing_text {span {class:"m3-list__trailing-text","{text}"}}
        if selected.is_some(){span {class:"m3-list__selection",aria_hidden:"true",if selected==Some(true){Icon {icon:icons::CHECK}}}}
        else if let Some(trailing)=trailing {span {class:"m3-list__trailing",{trailing}}}
    };
    rsx! {div {id:id.clone(),class:"m3-list__row {class}",role:if selected.is_some(){"presentation"}else{"listitem"},"data-lines":line,"data-selected":selected==Some(true),"data-disabled":disabled,
        if actionable {button {r#type:"button",class:"m3-list__primary",role:if selected.is_some(){Some("option")}else{None},aria_selected:selected,disabled,tabindex:"0",aria_expanded:if expandable{expanded}else{None},aria_controls:if expandable{Some(format!("{id}-panel"))}else{None},onclick:move |_|{if expandable{onexpandedchange.call(!expanded.unwrap_or(false));}onactivate.call(());},super::ripple::Ripple {},{content}}}
        else {div {class:"m3-list__primary",{content}}}
        if expandable{div{id:"{id}-panel",class:"m3-disclosure__panel m3-list__expanded","data-open":expanded.unwrap_or(false),"data-animate":"true",aria_hidden:!expanded.unwrap_or(false),inert:if !expanded.unwrap_or(false){Some("")}else{None},div{class:"m3-disclosure__inner",{expanded_content}}}}
        if selected.is_none(){if let Some(secondary)=secondary {div {class:"m3-list__secondary",{secondary}}}}
    }}
}

/// A swipe action always has an equivalent native button. Provide reversible
/// application behavior (e.g. Undo) when the action removes information.
#[component]
pub fn SwipeAction(label:String,action_label:String,#[props(default)] disabled:bool,onaction:EventHandler<()>,children:Element)->Element{
 let id=use_hook(||next_id("m3-swipe-action"));super::swipe::use_swipe(id.clone(),EventHandler::new(move |_|{if !disabled{onaction.call(());}}));
 rsx!{div{id,class:"m3-swipe-action",role:"group",aria_label:label,"data-allow-buttons":"true","data-disabled":disabled,
  div{class:"m3-swipe-action-content",{children}}button{r#type:"button",class:"m3-swipe-action-button",disabled,"data-swipe-action":"true",onclick:move |_|onaction.call(()),"{action_label}"}
 }}
}
