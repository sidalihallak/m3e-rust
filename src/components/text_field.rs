use dioxus::prelude::*;

use super::motion::next_id;

/// Text field style. `Outlined` has a 1dp outline. `Filled` has a
/// surface-container-highest container and a bottom active indicator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextFieldVariant {
    #[default]
    Outlined,
    Filled,
}

impl TextFieldVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Filled => "filled",
        }
    }
}

/// A 56dp text field on a native `<input>`, with a label that floats above the
/// text once the field is focused or has a value.
///
/// `supporting` is helper text below the field. When `error` is set, the field
/// shows the error colours, sets `aria-invalid`, and the supporting text is the
/// error message. The label is linked with `for`, and the supporting text with
/// `aria-describedby`.
#[component]
pub fn TextField(
    label: String,
    #[props(default)] variant: TextFieldVariant,
    #[props(default)] value: String,
    #[props(default)] input_type: String,
    #[props(default)] supporting: Option<String>,
    #[props(default)] error: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-field"));
    let support_id = format!("{id}-support");
    let input_type = if input_type.is_empty() { "text".to_string() } else { input_type };
    let error_class = if error { " m3-field--error" } else { "" };
    let disabled_class = if disabled { " m3-field--disabled" } else { "" };
    let class = format!("m3-field m3-field--{}{error_class}{disabled_class} {class}", variant.class());
    let has_support = supporting.is_some();

    rsx! {
        div { class,
            div { class: "m3-field__container",
                span { class: "m3-field__surface", aria_hidden: "true" }
                // A single space placeholder lets CSS detect an empty field with :placeholder-shown.
                input {
                    id: "{id}",
                    class: "m3-field__input",
                    r#type: input_type,
                    placeholder: " ",
                    value,
                    disabled,
                    "aria-invalid": if error { "true" } else { "false" },
                    "aria-describedby": if has_support { support_id.clone() } else { String::new() },
                    oninput: move |event| oninput.call(event.value()),
                }
                // The outline follows the input so that `input:focus ~ .m3-field__outline` matches.
                span { class: "m3-field__outline", aria_hidden: "true" }
                label { class: "m3-field__label", r#for: "{id}", "{label}" }
                span { class: "m3-field__indicator", aria_hidden: "true" }
            }
            if let Some(text) = supporting {
                p { id: "{support_id}", class: "m3-field__supporting", "{text}" }
            }
        }
    }
}
