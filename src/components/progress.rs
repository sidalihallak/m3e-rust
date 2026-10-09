use dioxus::prelude::*;

/// Linear progress: `value` is 0.0 to 1.0, or `None` for indeterminate.
///
/// Wavy tracks use a sine wave, 3dp amplitude and 40dp wavelength (20dp while
/// indeterminate). Thick tracks are 8dp, and flat tracks 4dp. Determinate
/// tracks show a 4dp stop dot at the end of the track.
#[component]
pub fn LinearProgress(
    #[props(default)] value: Option<f64>,
    #[props(default)] wavy: bool,
    #[props(default)] thick: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let wavy_class = if wavy { " m3-linear--wavy" } else { "" };
    let thick_class = if thick { " m3-linear--thick" } else { "" };
    let class = format!("m3-linear{wavy_class}{thick_class} {class}");
    let value = value.map(|v| v.clamp(0.0, 1.0));

    match value {
        Some(v) => {
            let percent = format!("{:.2}%", v * 100.0);
            let now = format!("{:.2}", v);
            rsx! {
                div {
                    class: "{class} m3-linear--determinate",
                    role: "progressbar",
                    "aria-label": aria_label,
                    "aria-valuemin": "0",
                    "aria-valuemax": "1",
                    "aria-valuenow": now,
                    div { class: "m3-linear__active", style: "width: {percent};",
                        div { class: "m3-linear__wave" }
                    }
                    div { class: "m3-linear__track" }
                    if v < 1.0 {
                        span { class: "m3-linear__stop", aria_hidden: "true" }
                    }
                }
            }
        }
        None => rsx! {
            div {
                class: "{class} m3-linear--indeterminate",
                role: "progressbar",
                "aria-label": aria_label,
                div { class: "m3-linear__bar m3-linear__bar--1",
                    div { class: "m3-linear__wave" }
                }
                div { class: "m3-linear__bar m3-linear__bar--2",
                    div { class: "m3-linear__wave" }
                }
            }
        },
    }
}

/// Circular progress: a 40dp ring with a 4dp stroke (8dp when thick).
/// `value` is 0.0 to 1.0, or `None` for indeterminate, which rotates the arc.
///
/// The wavy circular form is not implemented in this version.
#[component]
pub fn CircularProgress(
    #[props(default)] value: Option<f64>,
    #[props(default)] thick: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let thick_class = if thick { " m3-circular--thick" } else { "" };
    let indeterminate_class = if value.is_none() { " m3-circular--indeterminate" } else { "" };
    let class = format!("m3-circular{thick_class}{indeterminate_class} {class}");
    let v = value.map(|v| v.clamp(0.0, 1.0));
    // Circumference on a 40-unit viewBox with the ring at radius 18.
    let circumference = 2.0 * std::f64::consts::PI * 18.0;
    let dash_offset = format!("{:.2}", circumference * (1.0 - v.unwrap_or(0.0)));
    let dash_array = format!("{:.2}", circumference);
    let now = v.map(|v| format!("{:.2}", v));

    rsx! {
        div {
            class,
            role: "progressbar",
            "aria-label": aria_label,
            "aria-valuemin": "0",
            "aria-valuemax": "1",
            "aria-valuenow": now,
            svg { class: "m3-circular__svg", view_box: "0 0 40 40",
                circle { class: "m3-circular__track", cx: "20", cy: "20", r: "18" }
                circle {
                    class: "m3-circular__active",
                    cx: "20",
                    cy: "20",
                    r: "18",
                    "stroke-dasharray": dash_array,
                    "stroke-dashoffset": dash_offset,
                }
            }
        }
    }
}
