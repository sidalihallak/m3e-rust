use super::{Icon, IconData, motion::next_id};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SearchLayout {
    #[default]
    Adaptive,
    Docked,
    FullScreen,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    pub value: String,
    pub label: String,
    pub supporting: Option<String>,
    pub icon: Option<IconData>,
    pub disabled: bool,
}
impl SearchResult {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            supporting: None,
            icon: None,
            disabled: false,
        }
    }
}
/// Native primary opener and independent trailing controls; never nest buttons.
#[component]
pub fn SearchBar(
    id: String,
    label: String,
    #[props(default)] value: String,
    #[props(default)] elevated: bool,
    #[props(default)] outlined: bool,
    #[props(default)] trailing: Option<Element>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {div {id,class:"m3-search-bar {class}","data-elevated":elevated,"data-outlined":outlined,
        button {r#type:"button",class:"m3-search-bar__primary",aria_label:label.clone(),disabled,onclick,super::ripple::Ripple {},span {class:"m3-search-bar__leading",Icon {icon:icons::SEARCH}}span {class:"m3-search-bar__text",if value.is_empty(){"{label}"}else{"{value}"}}}
        if let Some(trailing)=trailing {div {class:"m3-search-bar__trailing",{trailing}}}
    }}
}
/// Controlled contained search with native modal focus restoration and 24→12px margins. Provide an existing, stable SearchBar anchor ID.
/// Disabled results are skipped during arrow navigation; reopening clears preview selection.
/// Submitted results retain keyboard navigation when input focus is released.
/// Results are supplied by the app; keyboard preview keeps native input focus.
#[component]
pub fn SearchView(
    anchor_id: String,
    open: bool,
    value: String,
    label: String,
    results: Vec<SearchResult>,
    #[props(default)] layout: SearchLayout,
    #[props(default)] class: String,
    #[props(default)] oninput: EventHandler<String>,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onselect: EventHandler<String>,
    #[props(default)] onsubmit: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-search-view"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("search.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok((action, text)) = eval.recv::<(String, String)>().await {
                    match action.as_str() {
                        "close" => onopenchange.call(false),
                        "submit" => onsubmit.call(text),
                        _ => {}
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Search?.[{id:?}]?.();"));
        }
    });
    let kind = match layout {
        SearchLayout::Adaptive => "adaptive",
        SearchLayout::Docked => "docked",
        SearchLayout::FullScreen => "full",
    };
    let count = results.len();
    rsx! {dialog {id:"{id}",class:"m3-search-view {class}",aria_label:label.clone(),aria_modal:"true","data-open":open,"data-layout":kind,"data-anchor":anchor_id,"data-value":value.clone(),
        div {class:"m3-search-view__scrim",aria_hidden:"true"}
        div {class:"m3-search-view__surface",
            div {class:"m3-search-view__header",
                button {r#type:"button",class:"m3-feedback__icon",aria_label:"Close {label}",onclick:move |_|onopenchange.call(false),super::ripple::Ripple {} Icon {icon:icons::ARROW_BACK}}
                input {id:"{id}-input",r#type:"search",role:"combobox",aria_label:label.clone(),placeholder:label,value:value.clone(),aria_expanded:"true",aria_autocomplete:"list",aria_controls:"{id}-results",oninput:move|e|oninput.call(e.value())}
                if !value.is_empty(){button {r#type:"button",class:"m3-feedback__icon","data-search-clear":"true",aria_label:"Clear search query",onclick:move |_|oninput.call(String::new()),super::ripple::Ripple {} Icon {icon:icons::CLOSE}}}
            }
            p {class:"m3-search-view__status",role:"status",aria_live:"polite",if count==0 {"No results"}else{"{count} results"}}
            div {id:"{id}-results",class:"m3-search-view__results",role:"listbox",aria_label:"Search results",tabindex:"-1",
                for (index,item) in results.into_iter().enumerate(){button {id:"{id}-result-{index}",key:"{item.value}",r#type:"button",class:"m3-search-result",role:"option",aria_selected:"false",disabled:item.disabled,tabindex:"-1",
                    onclick:move |_|{onselect.call(item.value.clone());onopenchange.call(false);},super::ripple::Ripple {},
                    if let Some(icon)=item.icon {Icon {icon}}
                    span {class:"m3-search-result__text",span {"{item.label}"}if let Some(text)=item.supporting {small {"{text}"}}}
                }}
            }
        }
    }}
}
