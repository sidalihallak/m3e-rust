use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

/// A two-state switch built on a native `<button role="switch">`.
///
/// The track is 52×32dp. The handle is 16dp unselected, 24dp selected, 28dp
/// while pressed, and 24dp unselected when an unchecked icon is supplied.
#[component]
pub fn Switch(
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] checked_icon: Option<Element>,
    #[props(default)] unchecked_icon: Option<Element>,
    #[props(default)] onchange: EventHandler<bool>,
) -> Element {
    let mut pressed = use_signal(|| false);
    let checked_class = if checked { " m3-switch--checked" } else { "" };
    let pressed_class = if pressed() && !disabled {
        " m3-switch--pressed"
    } else {
        ""
    };
    let unchecked_icon_class = if !checked && unchecked_icon.is_some() {
        " m3-switch--unchecked-icon"
    } else {
        ""
    };
    let disabled_class = if disabled { " m3-switch--disabled" } else { "" };
    let error_class = if error { " m3-switch--error" } else { "" };
    let class = format!(
        "m3-switch{checked_class}{pressed_class}{unchecked_icon_class}{disabled_class}{error_class} {class}"
    );

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            class,
            disabled,
            aria_checked: Some(checked),
            aria_invalid: if error { Some("true") } else { None },
            aria_label,
            onpointerdown: move |event| {
                if !disabled && event.is_primary() && event.trigger_button() == Some(MouseButton::Primary) {
                    pressed.set(true);
                }
            },
            onpointerup: move |_| pressed.set(false),
            onpointercancel: move |_| pressed.set(false),
            onpointerleave: move |_| pressed.set(false),
            onkeydown: move |event| {
                if !disabled && event.code() == Code::Space && !event.is_auto_repeating() {
                    pressed.set(true);
                }
            },
            onkeyup: move |event| {
                if event.code() == Code::Space {
                    pressed.set(false);
                }
            },
            onblur: move |_| pressed.set(false),
            onclick: move |_| onchange.call(!checked),
            span { class: "m3-switch__handle",
                span { class: "m3-switch__icon m3-switch__icon--checked", aria_hidden: "true",
                    if let Some(icon) = checked_icon.clone() {
                        {icon}
                    }
                }
                span { class: "m3-switch__icon m3-switch__icon--unchecked", aria_hidden: "true",
                    if let Some(icon) = unchecked_icon.clone() {
                        {icon}
                    }
                }
            }
        }
    }
}
