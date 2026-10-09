use dioxus::prelude::*;

/// A radio button built on a native `<input type="radio">`, so arrow-key
/// navigation and single selection within a `name` group come from the browser.
///
/// The visible ring is 20dp with a 2dp outline; the selected dot is 10dp and
/// scales in on the expressive fast spatial spring. The state layer is 40dp.
#[component]
pub fn Radio(
    name: String,
    value: String,
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onchange: EventHandler<()>,
) -> Element {
    let checked_class = if checked { " m3-radio--checked" } else { "" };
    let disabled_class = if disabled { " m3-radio--disabled" } else { "" };
    let class = format!("m3-radio{checked_class}{disabled_class} {class}");

    rsx! {
        label { class,
            input {
                class: "m3-radio__input",
                r#type: "radio",
                name,
                value,
                checked,
                disabled,
                "aria-label": aria_label,
                onchange: move |_| onchange.call(()),
            }
            super::ripple::Ripple {}
            span { class: "m3-radio__state", aria_hidden: "true" }
            // Ring and dot are SVG circles on a 20-unit grid: the centre stays exact
            // at any browser zoom, unlike boxes whose edges round to device pixels.
            svg { class: "m3-radio__ring", view_box: "0 0 20 20", "aria-hidden": "true",
                circle { class: "m3-radio__outline", cx: "10", cy: "10", r: "9" }
                circle { class: "m3-radio__dot", cx: "10", cy: "10", r: "5" }
            }
        }
    }
}

/// A group of radio buttons that share a name, with a group label for assistive technology.
#[component]
pub fn RadioGroup(
    #[props(default)] aria_label: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "m3-radio-group {class}", role: "radiogroup", "aria-label": aria_label,
            {children}
        }
    }
}
