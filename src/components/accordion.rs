use super::{Icon, motion::next_id};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AccordionEntry {
    pub value: String,
    pub title: String,
    pub content: Element,
    pub disabled: bool,
}
impl AccordionEntry {
    pub fn new(value: impl Into<String>, title: impl Into<String>, content: Element) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            content,
            disabled: false,
        }
    }
}

pub(crate) fn use_disclosure_runtime(id: String) {
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("disclosure.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop(move || {
        spawn(async move {
            let _ = document::eval(&format!("window.__m3Disclosure?.[{id:?}]?.();")).await;
        });
    });
}

/// Controlled, vertical accordion with measured, interruptible panel height. Values must be unique. All enabled headings
/// remain in the Tab sequence; native Enter and Space activate the button.
#[component]
pub fn Accordion(
    items: Vec<AccordionEntry>,
    #[props(default)] value: Vec<String>,
    #[props(default)] multiple: bool,
    #[props(default)] disabled: bool,
    #[props(default = 3)] heading_level: u8,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<Vec<String>>,
) -> Element {
    let id = use_hook(|| next_id("m3-accordion"));
    use_disclosure_runtime(id.clone());
    rsx! {div {id:"{id}",class:"m3-accordion {class}",
        for (index,item) in items.into_iter().enumerate() {
            { let open=if multiple {value.contains(&item.value)}else{value.first()==Some(&item.value)};
              let trigger=format!("{id}-trigger-{index}");let panel=format!("{id}-panel-{index}");
              let mut next=if multiple {value.clone()}else{Vec::new()};
              if open {next.retain(|v|v!=&item.value);}else{next.push(item.value.clone());}
              rsx! {div {key:"{item.value}",class:"m3-accordion__item", "data-open":open,
                div {role:"heading",aria_level:heading_level.clamp(1,6),class:"m3-accordion__heading",
                    button {id:"{trigger}",r#type:"button",class:"m3-accordion__trigger",disabled:disabled||item.disabled,aria_expanded:open,aria_controls:"{panel}",onclick:move |_|onchange.call(next.clone()),
                        super::ripple::Ripple {},span {"{item.title}"}span {class:if open{"m3-accordion__icon m3-accordion__icon--open"}else{"m3-accordion__icon"},aria_hidden:"true",Icon {icon:icons::KEYBOARD_ARROW_DOWN}}
                    }
                }
                div {id:panel,class:"m3-disclosure__panel", "data-open":open,"data-animate":"true",aria_labelledby:trigger,aria_hidden:!open,inert:if !open{Some("")}else{None},
                    div {class:"m3-disclosure__inner m3-accordion__content",{item.content}}
                }
              }}
            }
        }
    }}
}
