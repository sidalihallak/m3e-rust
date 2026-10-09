use dioxus::prelude::*;

/// A slider built on a native `<input type="range">`, so arrow keys, Home, End and
/// page keys work as they do for any range input. The visuals are drawn over it.
///
/// The track is 16dp, with an 8dp gap either side of a 4×44dp handle (2dp wide
/// while pressed). A 4dp stop indicator sits at the end of the inactive track
/// until the value reaches the maximum.
#[component]
pub fn Slider(
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default)] value: f64,
    #[props(default)] step: Option<f64>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onchange: EventHandler<f64>,
) -> Element {
    let span = (max - min).max(f64::MIN_POSITIVE);
    let fraction = ((value - min) / span).clamp(0.0, 1.0);
    let at_max = fraction >= 1.0;
    let at_max_class = if at_max { " m3-slider--max" } else { "" };
    let disabled_class = if disabled { " m3-slider--disabled" } else { "" };
    let class = format!("m3-slider{at_max_class}{disabled_class} {class}");
    let step_attr = step.map(|s| s.to_string()).unwrap_or_else(|| "any".to_string());
    let now = format!("{value}");

    rsx! {
        div { class, style: "--p: {fraction};",
            div { class: "m3-slider__track",
                span { class: "m3-slider__active" }
                span { class: "m3-slider__inactive",
                    span { class: "m3-slider__stop" }
                }
            }
            input {
                class: "m3-slider__input",
                r#type: "range",
                min: "{min}",
                max: "{max}",
                step: "{step_attr}",
                value: "{now}",
                disabled,
                "aria-label": aria_label,
                oninput: move |event| {
                    if let Ok(v) = event.value().parse::<f64>() {
                        onchange.call(v);
                    }
                },
            }
            span { class: "m3-slider__state", aria_hidden: "true" }
            span { class: "m3-slider__handle", aria_hidden: "true" }
        }
    }
}
