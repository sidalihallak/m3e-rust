use dioxus::prelude::*;

use super::{Icon, IconData};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderSize { #[default] ExtraSmall, Small, Medium, Large, ExtraLarge }
impl SliderSize {
    const fn geometry(self) -> (f64, f64, f64, f64, f64) {
        match self {
            Self::ExtraSmall => (16.0, 44.0, 8.0, 0.0, 0.0),
            Self::Small => (24.0, 44.0, 8.0, 0.0, 0.0),
            Self::Medium => (40.0, 44.0, 12.0, 24.0, 6.0),
            Self::Large => (56.0, 68.0, 16.0, 24.0, 6.0),
            Self::ExtraLarge => (96.0, 108.0, 28.0, 32.0, 8.0),
        }
    }
}

/// A slider built on native `<input type="range">` elements, so arrow keys, Home,
/// End and page keys work. The visuals are drawn over them.
///
/// - Track: 16dp, with an 8dp gap either side of each 4×44dp handle (2dp wide while pressed).
/// - Stop indicator: 4dp at the end of the trailing inactive track, hidden at the maximum.
/// - `value_end` makes a range slider with two handles. `onchange_range` reports both values.
/// - `ticks` draws a tick at each `step` (needs `step`).
/// - `label` shows a value indicator above a handle while it is pressed or focused.
/// - `vertical` rotates the control. `value` increases upward, as the reference does.
#[component]
pub fn Slider(
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default)] value: f64,
    #[props(default)] value_end: Option<f64>,
    #[props(default)] step: Option<f64>,
    #[props(default)] ticks: bool,
    #[props(default)] label: bool,
    #[props(default)] size: SliderSize,
    #[props(default)] centered: bool,
    #[props(default)] leading_icon: Option<IconData>,
    #[props(default)] trailing_icon: Option<IconData>,
    #[props(default)] vertical: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onchange: EventHandler<f64>,
    #[props(default)] onchange_range: EventHandler<(f64, f64)>,
) -> Element {
    let span = (max - min).max(f64::MIN_POSITIVE);
    let fraction = |v: f64| ((v - min) / span).clamp(0.0, 1.0);
    let range = value_end.is_some();
    let mid = 0.5;
    let normalize = |v: f64| {
        let v = v.clamp(min, max);
        if let Some(s) = step.filter(|s| s.is_finite() && *s > 0.0) {
            min + ((v - min) / s).round().min(((max - min) / s).floor()) * s
        } else { v }
    };
    let hi = normalize(value_end.unwrap_or(value));
    let lo = normalize(value).min(if range { hi } else { max });
    let start = fraction(lo);
    let end = if range { fraction(hi) } else { start };
    let at_max = (if range { end } else { start }) >= 1.0;
    let step_attr = step.map(|s| s.to_string()).unwrap_or_else(|| "any".to_string());

    let mut classes = String::from("m3-slider");
    if range {
        classes.push_str(" m3-slider--range");
    } else {
        classes.push_str(" m3-slider--single");
    }
    if centered && !range { classes.push_str(" m3-slider--centered"); }
    if vertical {
        classes.push_str(" m3-slider--vertical");
    }
    if at_max {
        classes.push_str(" m3-slider--max");
    }
    if disabled {
        classes.push_str(" m3-slider--disabled");
    }
    if label {
        classes.push_str(" m3-slider--labelled");
    }
    let class = format!("{classes} {class}");

    // Tick marks: one per step, active when inside the active range.
    let tick_marks: Vec<(f64, bool)> = match (ticks, step) {
        (true, Some(s)) if s > 0.0 => {
            let n = ((max - min) / s).floor().max(0.0).min(10000.0) as usize;
            (0..=n)
                .map(|i| {
                    let f = fraction(min + i as f64 * s);
                    let active = if range { f >= fraction(lo) - 1e-9 && f <= fraction(hi) + 1e-9 } else if centered { f >= start.min(mid) - 1e-9 && f <= start.max(mid) + 1e-9 } else { f <= start + 1e-9 };
                    (f, active)
                })
                .collect()
        }
        _ => Vec::new(),
    };

    let (track_h, handle_h, corner, icon_size, icon_padding) = size.geometry();
    let hit_h = handle_h.max(48.0);
    let start_gap = if start < mid { 8.0 } else { 0.0 };
    let end_gap = if start >= mid { 8.0 } else { 0.0 };
    let center_lo = start.min(mid);
    let center_hi = start.max(mid);
    let style = format!("--p: {start}; --q: {end}; --track-h: {track_h}px; --handle-h: {handle_h}px; --hit-h: {hit_h}px; --track-corner: {corner}px; --slider-icon: {icon_size}px; --icon-pad: {icon_padding}px; --center-lo: {center_lo}; --center-hi: {center_hi}; --center-start-gap: {start_gap}px; --center-end-gap: {end_gap}px;");
    let fmt = |v: f64| {
        let r = (v * 100.0).round() / 100.0;
        format!("{r}")
    };
    let start_label = fmt(value);
    let end_label = fmt(hi);

    rsx! {
        div { class, style,
            div { class: "m3-slider__frame",
                div { class: "m3-slider__track",
                    if range {
                        span { class: "m3-slider__inactive m3-slider__inactive--lead",
                            if icon_size > 0.0 { if let Some(icon) = leading_icon { Icon { icon, class: "m3-slider__icon m3-slider__icon--leading" } } }
                        }
                        span { class: "m3-slider__active",
                            if icon_size > 0.0 { if let Some(icon) = leading_icon { Icon { icon, class: "m3-slider__icon m3-slider__icon--leading" } } }
                        }
                        span { class: "m3-slider__inactive m3-slider__inactive--trail",
                            span { class: "m3-slider__stop" }
                            if icon_size > 0.0 { if let Some(icon) = trailing_icon { Icon { icon, class: "m3-slider__icon m3-slider__icon--trailing" } } }
                        }
                    } else {
                        if centered { span { class: "m3-slider__inactive m3-slider__inactive--lead" } }
                        span { class: "m3-slider__active",
                            if icon_size > 0.0 { if let Some(icon) = leading_icon { Icon { icon, class: "m3-slider__icon m3-slider__icon--leading" } } }
                        }
                        span { class: "m3-slider__inactive m3-slider__inactive--trail",
                            span { class: "m3-slider__stop" }
                            if icon_size > 0.0 { if let Some(icon) = trailing_icon { Icon { icon, class: "m3-slider__icon m3-slider__icon--trailing" } } }
                        }
                    }
                }
                for (f, active) in tick_marks.iter().copied() {
                    span {
                        class: if active { "m3-slider__tick m3-slider__tick--active" } else { "m3-slider__tick" },
                        style: "--f: {f};",
                        aria_hidden: "true",
                    }
                }
                input {
                    class: "m3-slider__input m3-slider__input--start",
                    r#type: "range",
                    min: "{min}",
                    max: if range { fmt(hi) } else { fmt(max) },
                    step: "{step_attr}",
                    value: fmt(lo),
                    disabled,
                    "aria-orientation": if vertical { "vertical" } else { "horizontal" },
                    "aria-label": aria_label.clone().map(|l| if range { format!("{l} start") } else { l }),
                    oninput: move |event| {
                        if let Ok(v) = event.value().parse::<f64>() {
                            if range {
                                onchange_range.call((v.min(hi), hi));
                            } else {
                                onchange.call(v);
                            }
                        }
                    },
                }
                if range {
                    input {
                        class: "m3-slider__input m3-slider__input--end",
                        r#type: "range",
                        min: "{lo}",
                        max: "{max}",
                        step: "{step_attr}",
                        value: fmt(hi),
                        disabled,
                        "aria-orientation": if vertical { "vertical" } else { "horizontal" },
                        "aria-label": aria_label.clone().map(|l| format!("{l} end")),
                        oninput: move |event| {
                            if let Ok(v) = event.value().parse::<f64>() {
                                onchange_range.call((lo, v.max(lo)));
                            }
                        },
                    }
                }
                span { class: "m3-slider__handle m3-slider__handle--start", aria_hidden: "true" }
                if range {
                    span { class: "m3-slider__handle m3-slider__handle--end", aria_hidden: "true" }
                }
            }
            // Labels stay outside the frame, so they are not rotated in vertical mode.
                if label {
                    span { class: "m3-slider__label m3-slider__label--start", aria_hidden: "true", "{start_label}" }
                    if range {
                        span { class: "m3-slider__label m3-slider__label--end", aria_hidden: "true", "{end_label}" }
                    }
                }
        }
    }
}
