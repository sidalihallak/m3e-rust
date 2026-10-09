use dioxus::prelude::*;

use super::motion::next_id;
use super::{Icon, IconData};

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

/// A 56dp text field on a native `<input>` (or `<textarea>` when `multiline`),
/// with a label that floats above the text once the field is focused or has a value.
///
/// - `leading_icon` and `trailing_icon` are 24dp icons at the field's edges.
/// - `prefix` and `suffix` are text inside the field. As in Material, they show only
///   while the label is floated, so the resting label stays centred.
/// - `max_length` shows a counter (`count/max`) under the field. Going over the
///   limit shows the error state.
/// - `multiline` uses a textarea of three lines that grows with its content.
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
    #[props(default)] read_only: bool,
    #[props(default)] leading_icon: Option<IconData>,
    #[props(default)] trailing_icon: Option<IconData>,
    #[props(default)] prefix: Option<String>,
    #[props(default)] suffix: Option<String>,
    #[props(default)] max_length: Option<usize>,
    #[props(default)] multiline: bool,
    #[props(default)] class: String,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let id = use_hook(|| next_id("m3-field"));
    let support_id = format!("{id}-support");
    let input_type = if input_type.is_empty() { "text".to_string() } else { input_type };
    let length = value.chars().count();
    let over_limit = max_length.is_some_and(|max| length > max);
    let invalid = error || over_limit;
    let error_class = if invalid { " m3-field--error" } else { "" };
    let disabled_class = if disabled { " m3-field--disabled" } else { "" };
    let leading_class = if leading_icon.is_some() { " m3-field--leading" } else { "" };
    let trailing_class = if trailing_icon.is_some() { " m3-field--trailing" } else { "" };
    let multiline_class = if multiline { " m3-field--multiline" } else { "" };
    let class = format!(
        "m3-field m3-field--{}{error_class}{disabled_class}{leading_class}{trailing_class}{multiline_class} {class}",
        variant.class()
    );
    let has_support = supporting.is_some() || max_length.is_some();
    let counter = max_length.map(|max| format!("{length}/{max}"));

    // The notch is cut to the width of the floated label. A ResizeObserver tracks label and font changes.
    use_effect({
        let id = id.clone();
        move || {
            let script = notch_script(&id);
            spawn(async move {
                if let Err(err) = document::eval(&script).join::<bool>().await {
                    let _ = document::eval(&format!("console.error({:?})", format!("text field: {err}"))).await;
                }
            });
        }
    });

    rsx! {
        div { id: "{id}-root", class,
            div { class: "m3-field__container",
                onclick: { let id = id.clone(); move |_| {
                    if !disabled { let script = format!("document.getElementById({id:?})?.focus();"); spawn(async move { let _ = document::eval(&script).await; }); }
                }},
                span { class: "m3-field__surface", aria_hidden: "true" }
                div { class: "m3-field__row",
                    if let Some(icon) = leading_icon {
                        Icon { icon, class: "m3-field__icon m3-field__icon--leading" }
                    }
                    if let Some(text) = prefix {
                        span { class: "m3-field__affix m3-field__prefix", "{text}" }
                    }
                    if multiline {
                        textarea {
                            id: "{id}",
                            class: "m3-field__input",
                            rows: "3",
                            placeholder: " ",
                            value: "{value}",
                            disabled,
                            readonly: read_only,
                            "aria-invalid": if invalid { "true" } else { "false" },
                            "aria-describedby": if has_support { support_id.clone() } else { String::new() },
                            oninput: move |event| oninput.call(event.value()),
                        }
                    } else {
                        input {
                            id: "{id}",
                            class: "m3-field__input",
                            r#type: input_type,
                            placeholder: " ",
                            value,
                            disabled,
                            readonly: read_only,
                            "aria-invalid": if invalid { "true" } else { "false" },
                            "aria-describedby": if has_support { support_id.clone() } else { String::new() },
                            oninput: move |event| oninput.call(event.value()),
                        }
                    }
                    // These follow the input inside the row, so `input:focus ~ …` matches.
                    span { class: "m3-field__outline", aria_hidden: "true" }
                    label { class: "m3-field__label", r#for: "{id}", "{label}" }
                    span { class: "m3-field__indicator", aria_hidden: "true" }
                    if let Some(text) = suffix {
                        span { class: "m3-field__affix m3-field__suffix", "{text}" }
                    }
                    if let Some(icon) = trailing_icon {
                        Icon { icon, class: "m3-field__icon m3-field__icon--trailing" }
                    }
                }
            }
            // Hidden copy of the label at its floated size, used only to measure the notch width.
            span { class: "m3-field__measure", aria_hidden: "true", "{label}" }
            if has_support {
                div { class: "m3-field__support-row", id: "{support_id}",
                    if let Some(text) = supporting {
                        p { class: "m3-field__supporting", "{text}" }
                    }
                    if let Some(text) = counter {
                        span { class: "m3-field__counter", "{text}" }
                    }
                }
            }
        }
    }
}

/// Sets `--notch-text-w` on the field to the floated label's width, after font loading and whenever its size changes.
fn notch_script(id: &str) -> String {
    format!(
        r#"
const root = document.getElementById("{id}-root");
if (!root) return false;
const measure = () => {{
  const m = root.querySelector(".m3-field__measure");
  if (m) root.style.setProperty("--notch-text-w", m.getBoundingClientRect().width + "px");
}};
measure();
const m = root.querySelector(".m3-field__measure");
if (m) new ResizeObserver(measure).observe(m);
if (document.fonts) document.fonts.ready.then(measure);
return true;
"#
    )
}
