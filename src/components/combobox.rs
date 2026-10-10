use super::{
    TextFieldVariant,
    select::{ChoiceField, SelectOption},
};
use dioxus::prelude::*;

/// Editable list autocomplete, single or multiple values. Values are option IDs;
/// input_value is the draft filter. Selecting an item clears the filter and shows
/// its label (or chips). Escape preserves committed values. With allow_custom,
/// Enter may commit a draft when no option is active. Keep standard editing keys.
/// `show_clear` controls the single-value clear action. Multiple values use
/// each input chip's remove action and retain the field's dropdown affordance.
#[component]
pub fn Combobox(
    label: String,
    options: Vec<SelectOption>,
    #[props(default)] values: Vec<String>,
    #[props(default)] input_value: String,
    #[props(default)] open: bool,
    #[props(default)] multiple: bool,
    #[props(default)] allow_custom: bool,
    #[props(default = true)] show_clear: bool,
    #[props(default)] variant: TextFieldVariant,
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
    rsx! {ChoiceField {label,options,values,input_value,open,multiple,allow_custom,show_clear,variant,placeholder,supporting,name,disabled,required,error,class,editable:true,onopenchange,oninput,onchange}}
}

/// Free text with manual list suggestions. The native input remains editable;
/// oninput owns the text, onselect reports a chosen option ID (or custom text on
/// Enter). Unlike a restricted Combobox, arbitrary typed text is a valid value.
#[component]
pub fn Autocomplete(
    label: String,
    options: Vec<SelectOption>,
    #[props(default)] value: String,
    #[props(default)] open: bool,
    #[props(default)] variant: TextFieldVariant,
    #[props(default)] supporting: Option<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] placeholder: String,
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(default)] error: bool,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] oninput: EventHandler<String>,
    #[props(default)] onselect: EventHandler<String>,
) -> Element {
    rsx! {ChoiceField {label,options,input_value:value,values:vec![],open,variant,supporting,name,placeholder,class,disabled,required,error,editable:true,allow_custom:true,clear_on_select:false,show_clear:false,onopenchange,oninput,
        onchange:move|values:Vec<String>|{if let Some(value)=values.first(){onselect.call(value.clone());}}
    }}
}
