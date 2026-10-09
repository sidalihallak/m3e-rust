use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

/// A checkbox built on a native `<button role="checkbox">`.
///
/// The visible box is 18×18dp with a 2dp outline and 2dp corners. The
/// touch target and state layer are 40dp.
#[component]
pub fn Checkbox(
    #[props(default)] checked: bool,
    #[props(default)] indeterminate: bool,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onchange: EventHandler<bool>,
) -> Element {
    let mut pressed = use_signal(|| false);
    let checked_class = if checked || indeterminate {
        " m3-checkbox--checked"
    } else {
        ""
    };
    let indeterminate_class = if indeterminate {
        " m3-checkbox--indeterminate"
    } else {
        ""
    };
    let pressed_class = if pressed() && !disabled {
        " m3-checkbox--pressed"
    } else {
        ""
    };
    let disabled_class = if disabled { " m3-checkbox--disabled" } else { "" };
    let error_class = if error { " m3-checkbox--error" } else { "" };
    let class = format!(
        "m3-checkbox{checked_class}{indeterminate_class}{pressed_class}{disabled_class}{error_class} {class}"
    );
    // A mixed checkbox resolves to checked when activated, as in the reference.
    let aria_checked = if indeterminate {
        "mixed".to_string()
    } else {
        checked.to_string()
    };

    rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            class,
            disabled,
            aria_checked,
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
            onclick: move |_| onchange.call(indeterminate || !checked),
            super::ripple::Ripple {}
            span { class: "m3-checkbox__state", aria_hidden: "true" }
            span { class: "m3-checkbox__box", aria_hidden: "true",
                svg { class: "m3-checkbox__mark", view_box: "0 0 18 18",
                    path {
                        class: "m3-checkbox__check",
                        d: "M3.5 9.2 7.2 12.8 14.6 5.4",
                        "pathLength": "1",
                    }
                    path {
                        class: "m3-checkbox__dash",
                        d: "M4 9h10",
                    }
                }
            }
        }
    }
}
