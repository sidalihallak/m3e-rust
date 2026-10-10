use super::{Icon, IconData, TextFieldVariant, anchored::use_popup_runtime, motion::next_id};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
    pub group: Option<String>,
    pub icon: Option<IconData>,
}
impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
            group: None,
            icon: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectSize {
    #[default]
    Default,
    Small,
}

/// Controlled select-only combobox. Navigation previews an option; Enter/Space
/// commits it; Escape cancels. `name` supplies a hidden form value. Disabled
/// options are visible but skipped, following listbox (not menu) semantics.
#[component]
pub fn Select(
    label: String,
    options: Vec<SelectOption>,
    #[props(default)] value: String,
    #[props(default)] open: bool,
    #[props(default)] variant: TextFieldVariant,
    #[props(default)] size: SelectSize,
    #[props(default)] placeholder: String,
    #[props(default)] supporting: Option<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(default)] error: bool,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onchange: EventHandler<String>,
) -> Element {
    rsx! { ChoiceField { label,options,values:if value.is_empty(){vec![]}else{vec![value]},open,variant,size,placeholder,supporting,name,disabled,required,error,class,
        onopenchange,onchange:move|values:Vec<String>|onchange.call(values.into_iter().next().unwrap_or_default()),
    }}
}

/// Native picker fallback: Material field styling around a real HTML select.
/// Its opened picker is supplied by the browser/OS, rather than the kit.
#[component]
pub fn NativeSelect(
    label: String,
    options: Vec<SelectOption>,
    #[props(default)] value: String,
    #[props(default)] variant: TextFieldVariant,
    #[props(default)] size: SelectSize,
    #[props(default)] supporting: Option<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(default)] error: bool,
    #[props(default)] onchange: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-native-select"));
    let support_id = format!("{id}-support");
    let style = if variant == TextFieldVariant::Filled {
        "filled"
    } else {
        "outlined"
    };
    let size = if size == SelectSize::Small {
        "small"
    } else {
        "default"
    };
    rsx! { div {class:"m3-choice m3-choice--{style} m3-choice--{size}", "data-disabled":disabled,"data-error":error,
        div {class:"m3-choice__field",
            ChoiceLabel { label:label.clone(),required,variant }
            label { r#for:"{id}",class:"m3-choice__sr", "{label}" if required {" *"} }
            select { id:"{id}",class:"m3-choice__native",name,value,disabled,required,aria_invalid:error,
                aria_describedby:supporting.as_ref().map(|_|support_id.clone()),
                onchange:move|e|onchange.call(e.value()),
                for (index,option) in options.iter().enumerate() {
                    if let Some(group)=&option.group {
                        if index==0||options[index-1].group.as_ref()!=Some(group) {
                            optgroup {label:group.clone(),for o in options.iter().skip(index).take_while(|o|o.group.as_ref()==Some(group)) {option {value:o.value.clone(),disabled:o.disabled,"{o.label}"}}}
                        }
                    }else {option {value:option.value.clone(),disabled:option.disabled,"{option.label}"}}
                }
            }
            if error {Icon {icon:icons::CANCEL,class:"m3-choice__native-error"}}
            Icon {icon:icons::ARROW_DROP_DOWN,class:"m3-choice__native-arrow"}
        }
        if let Some(text)=&supporting {div {id:support_id.clone(),class:"m3-choice__support",role:if error {Some("alert")}else{None},"{text}"}}
    }}
}

#[component]
fn ChoiceLabel(label: String, required: bool, variant: TextFieldVariant) -> Element {
    rsx! {
        if variant==TextFieldVariant::Outlined {fieldset {class:"m3-choice__outline",aria_hidden:"true",
            legend {span {"{label}" if required {" *"}}}
        }}else{span {class:"m3-choice__filled-label",aria_hidden:"true","{label}" if required {" *"}}}
    }
}

#[component]
pub(crate) fn ChoiceField(
    label: String,
    options: Vec<SelectOption>,
    values: Vec<String>,
    open: bool,
    #[props(default)] editable: bool,
    #[props(default)] input_value: String,
    #[props(default)] multiple: bool,
    #[props(default)] allow_custom: bool,
    #[props(default = true)] clear_on_select: bool,
    #[props(default = true)] show_clear: bool,
    #[props(default)] variant: TextFieldVariant,
    #[props(default)] size: SelectSize,
    #[props(default)] placeholder: String,
    #[props(default)] supporting: Option<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(default)] error: bool,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] oninput: EventHandler<String>,
    #[props(default)] onchange: EventHandler<Vec<String>>,
) -> Element {
    let id = use_hook(|| next_id("m3-choice"));
    let input_id = format!("{id}-input");
    let panel_id = format!("{id}-list");
    let support_id = format!("{id}-support");
    let label = if required {
        format!("{label} *")
    } else {
        label
    };
    let style = if variant == TextFieldVariant::Filled {
        "filled"
    } else {
        "outlined"
    };
    let size = if size == SelectSize::Small {
        "small"
    } else {
        "default"
    };
    let selected = values
        .first()
        .and_then(|v| options.iter().find(|o| &o.value == v))
        .map(|o| o.label.clone())
        .unwrap_or_else(|| {
            if allow_custom {
                values
                    .first()
                    .cloned()
                    .unwrap_or_else(|| placeholder.clone())
            } else {
                placeholder.clone()
            }
        });
    let all = options.clone();
    let current = values.clone();
    let custom_values = values.clone();
    use_popup_runtime(
        panel_id.clone(),
        include_str!("selection.js"),
        EventHandler::new(move |message: Vec<String>| {
            if message.first().is_some_and(|m| m == "open") {
                let next = message.get(1).is_some_and(|v| v == "true");
                if !next && editable && clear_on_select {
                    oninput.call(String::new());
                }
                onopenchange.call(next);
            } else if message.first().is_some_and(|m| m == "custom") && allow_custom {
                if let Some(value) = message.get(1) {
                    let value = value.trim().to_string();
                    if !value.is_empty() {
                        let mut next = if multiple {
                            custom_values.clone()
                        } else {
                            vec![]
                        };
                        if !next.contains(&value) {
                            next.push(value.clone());
                        }
                        onchange.call(next);
                        oninput.call(if clear_on_select {
                            String::new()
                        } else {
                            value
                        });
                        onopenchange.call(false);
                    }
                }
            }
        }),
    );
    // The event closure is rebuilt with current controlled values each render.
    let choose = EventHandler::new(move |option: SelectOption| {
        if disabled || option.disabled {
            return;
        }
        if multiple {
            let mut next = current.clone();
            if let Some(at) = next.iter().position(|v| v == &option.value) {
                next.remove(at);
            } else {
                next.push(option.value);
            }
            onchange.call(next);
            oninput.call(String::new());
        } else {
            onchange.call(vec![option.value]);
            if editable {
                oninput.call(if clear_on_select {
                    String::new()
                } else {
                    option.label
                });
            }
            onopenchange.call(false);
        }
    });
    let visible: Vec<_> = options
        .iter()
        .enumerate()
        .filter(|(_, o)| {
            !editable
                || input_value.trim().is_empty()
                || o.label
                    .to_lowercase()
                    .contains(&input_value.trim().to_lowercase())
        })
        .map(|(i, o)| (i, o.clone()))
        .collect();
    let groups: Vec<_> = visible
        .chunk_by(|a, b| a.1.group == b.1.group)
        .map(|items| (items[0].1.group.clone(), items.to_vec()))
        .collect();
    rsx! { div { id:"{id}",class:"m3-choice m3-choice--{style} m3-choice--{size} {class}","data-disabled":disabled,"data-error":error,"data-open":open,
        div { id:"{id}-field",class:"m3-choice__field", "data-multiple":multiple,
            ChoiceLabel {label:label.clone(),required:false,variant}
            if editable {
                div {class:"m3-choice__edit-row",
                    if multiple {for value in values.iter() {
                        {let text=all.iter().find(|o|&o.value==value).map(|o|o.label.clone()).unwrap_or_else(||value.clone());let remove=value.clone();let selected_values=values.clone();
                        rsx!{span {class:"m3-choice-chip",span {"{text}"}
                            button {r#type:"button",class:"m3-choice-chip__remove",disabled,aria_label:"Remove {text}",
                                onclick:move |_|{let next=selected_values.iter().filter(|v|v!=&&remove).cloned().collect();onchange.call(next);},
                                Icon {icon:icons::CLOSE}
                            }
                        }} }
                    }}
                    input { id:"{input_id}", class:"m3-choice__input", r#type:"text",role:"combobox",autocomplete:"off",disabled,
                        name:if !clear_on_select&&!multiple {name.clone()}else{None},required:required&&!clear_on_select&&!multiple,
                        value:if input_value.is_empty()&&!multiple&&values.len()==1{selected.clone()}else{input_value.clone()},
                        placeholder,aria_label:label.clone(),aria_haspopup:"listbox",aria_autocomplete:"list",aria_expanded:open,aria_controls:panel_id.clone(),aria_required:required,aria_invalid:error,
                        aria_describedby:supporting.as_ref().map(|_|support_id.clone()),
                        onclick:move |_|onopenchange.call(true),
                        oninput:move|e|{let next=e.value();if next.is_empty()&&!multiple{onchange.call(vec![]);}oninput.call(next);onopenchange.call(true);},
                    }
                    if error {Icon {icon:icons::CANCEL,class:"m3-choice__error-icon"}}
                    // Input chips already expose per-value removal. Keep the
                    // dropdown affordance instead of a redundant clear-all.
                    if !multiple&&show_clear&&(!values.is_empty()||!input_value.is_empty()) {button {r#type:"button",class:"m3-choice__clear",tabindex:"-1",aria_label:"Clear {label}",disabled,
                        onmousedown:move|e|e.prevent_default(),onclick:move |_|{onchange.call(vec![]);oninput.call(String::new());},Icon {icon:icons::CLOSE}
                    }}else{button {r#type:"button",class:"m3-choice__toggle",tabindex:"-1",aria_label:"Show {label} options",disabled,
                        onmousedown:move|e|e.prevent_default(),onclick:move |_|onopenchange.call(!open),Icon {icon:icons::ARROW_DROP_DOWN}
                    }}
                }
            }else{
                button { id:"{input_id}",class:"m3-choice__trigger",r#type:"button",role:"combobox",disabled,
                    aria_label:label.clone(),aria_required:required,aria_invalid:error,aria_expanded:open,aria_controls:panel_id.clone(),aria_haspopup:"listbox",
                    aria_describedby:supporting.as_ref().map(|_|support_id.clone()),
                    onclick:move |_|onopenchange.call(!open),
                    span {class:"m3-choice__value", "data-placeholder":values.is_empty(),"{selected}"}
                    if error { Icon {icon:icons::CANCEL,class:"m3-choice__error-icon"} }
                    Icon {icon:icons::ARROW_DROP_DOWN,class:"m3-choice__arrow"}
                }
            }
        }
        if let Some(text)=supporting {div {id:support_id,class:"m3-choice__support",role:if error{Some("alert")}else{None},"{text}"}}
        if let Some(name)=name { for value in values.iter() {input {r#type:"hidden",name:name.clone(),value:value.clone(),disabled}} }
        div {id:"{panel_id}",class:"m3-anchor m3-choice-list",role:"listbox",aria_label:label.clone(),aria_multiselectable:multiple,
            "popover":"manual","data-kind":if editable{"combobox"}else{"select"},"data-anchor":input_id,"data-open":open&&!disabled,
            "data-position-anchor":"{id}-field","data-query":input_value,"data-multiple":multiple,"data-custom":allow_custom,"data-side":"bottom","data-align":"start","data-offset":if editable{6}else{4},
            if visible.is_empty() {div {class:"m3-choice-empty",role:"status","No matching options"}}
            for (group, items) in groups.iter() {
                div {class:"m3-choice-options",role:if group.is_some(){"group"}else{"presentation"},aria_label:group.clone(),
                    if let Some(group)=group {div {class:"m3-choice-group",aria_hidden:"true","{group}"}}
                    for (index,option) in items.iter() {
                {let option=option.clone();let select=option.clone();let selected=values.contains(&option.value);
                rsx!{button {id:"{panel_id}-option-{index}",key:"{option.value}",class:"m3-choice-option",r#type:"button",role:"option",tabindex:"-1",
                    aria_selected:selected,aria_disabled:option.disabled,"data-label":option.label.clone(),"data-value":option.value.clone(),
                    onmousedown:move|e|e.prevent_default(),onclick:move |_|choose.call(select.clone()),
                    super::ripple::Ripple {}
                    if let Some(icon)=option.icon {Icon {icon,class:"m3-choice-option__icon"}}
                    span {"{option.label}"}
                    if selected {Icon {icon:icons::CHECK,class:"m3-choice-option__check"}}
                }} }
            }
                }
            }
        }
    }}
}
