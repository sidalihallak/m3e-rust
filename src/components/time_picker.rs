use super::{Button, ButtonVariant, Dialog, Icon, motion::next_id, time::TimeValue};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimePickerMode {
    #[default]
    Dial,
    Input,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimePickerLayout {
    #[default]
    Adaptive,
    Vertical,
    Horizontal,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum TimePart {
    #[default]
    Hours,
    Minutes,
}

/// Native modal time picker. Edits are a draft; only OK calls onconfirm.
/// Keyboard entry is always available. Value uses validated 24-hour storage.
/// Copy with time.rs, time_picker.js and time-picker.css plus shared Dialog assets.
#[component]
pub fn TimePicker(
    open: bool,
    #[props(default)] value: TimeValue,
    #[props(default = "Select time".to_string())] title: String,
    #[props(default)] hour24: bool,
    #[props(default)] default_mode: TimePickerMode,
    #[props(default)] layout: TimePickerLayout,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onconfirm: EventHandler<TimeValue>,
) -> Element {
    let picker_class = "m3-time-picker";
    let id = use_hook(|| next_id("m3-time"));
    let mut draft = use_signal(|| value);
    let mut part = use_signal(TimePart::default);
    let mut mode = use_signal(|| default_mode);
    let mut texts = use_signal(|| {
        [
            format!("{:02}", value.shown_hour(hour24)),
            format!("{:02}", value.minutes()),
        ]
    });
    let mut viewport = use_signal(|| (false, false, false));
    use_effect(use_reactive((&open,), move |(open,)| {
        if open {
            draft.set(value);
            part.set(TimePart::Hours);
            mode.set(default_mode);
            texts.set([
                format!("{:02}", value.shown_hour(hour24)),
                format!("{:02}", value.minutes()),
            ]);
        }
    }));
    let forced_input = viewport().1 || layout == TimePickerLayout::Vertical && viewport().0;
    let effective_input = mode() == TimePickerMode::Input || forced_input;
    let horizontal = !effective_input
        && viewport().2
        && (layout == TimePickerLayout::Horizontal
            || layout == TimePickerLayout::Adaptive && viewport().0);
    let event = use_callback(move |message: Vec<String>| {
        let kind = message.first().map(String::as_str).unwrap_or("");
        let n = message
            .get(1)
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(0);
        match kind {
            "viewport" => viewport.set((
                n == 1,
                message.get(2).is_some_and(|v| v == "1"),
                message.get(3).is_some_and(|v| v == "1"),
            )),
            "dial" | "finish" => {
                let next = if part() == TimePart::Hours {
                    draft().set_hour(n, hour24)
                } else {
                    draft().set_minute(n)
                };
                if let Some(next) = next {
                    draft.set(next);
                }
                if kind == "finish" && part() == TimePart::Hours {
                    part.set(TimePart::Minutes);
                }
            }
            "cancel" => {
                if let Some(v) = message
                    .get(2)
                    .and_then(|s| s.parse().ok())
                    .and_then(|m| TimeValue::new(n, m))
                {
                    draft.set(v);
                }
            }
            "commit" => {
                let index = usize::from(n.min(1));
                let raw = texts()[index].clone();
                let parsed = TimeValue::parse_field(
                    &raw,
                    if index == 0 {
                        if hour24 { 23 } else { 12 }
                    } else {
                        59
                    },
                    if index == 0 && !hour24 { 1 } else { 0 },
                );
                if let Some(v) = parsed {
                    let mut t = texts();
                    t[index] = format!("{v:02}");
                    texts.set(t);
                }
            }
            _ => {}
        }
    });
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("time_picker.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<Vec<String>>().await {
                    event.call(message);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Times?.[{id:?}]?.();"));
        }
    });
    let edit = use_callback(move |(index, raw): (usize, String)| {
        let mut t = texts();
        t[index] = raw.clone();
        texts.set(t);
        if let Some(n) = TimeValue::parse_field(
            &raw,
            if index == 0 {
                if hour24 { 23 } else { 12 }
            } else {
                59
            },
            if index == 0 && !hour24 { 1 } else { 0 },
        ) {
            if let Some(v) = if index == 0 {
                draft().set_hour(n, hour24)
            } else {
                draft().set_minute(n)
            } {
                draft.set(v);
            }
        }
    });
    let valid = !effective_input
        || TimeValue::parse_field(
            &texts()[0],
            if hour24 { 23 } else { 12 },
            if hour24 { 0 } else { 1 },
        )
        .is_some()
            && TimeValue::parse_field(&texts()[1], 59, 0).is_some();
    let toggle = move |_| {
        if !effective_input {
            texts.set([
                format!("{:02}", draft().shown_hour(hour24)),
                format!("{:02}", draft().minutes()),
            ]);
            mode.set(TimePickerMode::Input);
        } else if valid {
            mode.set(TimePickerMode::Dial);
        }
    };
    let shown = draft().shown_hour(hour24);
    let minute = draft().minutes();
    let selected = if part() == TimePart::Hours {
        shown
    } else {
        minute
    };
    let principal = if part() == TimePart::Hours {
        f64::from(shown % 12) * 30.0
    } else {
        f64::from(minute) * 6.0
    };
    // Initialize at the selected angle and unwrap subsequent turns.
    let angle_state = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(principal)));
    let angle =
        angle_state.get() + (principal - angle_state.get() + 180.0).rem_euclid(360.0) - 180.0;
    angle_state.set(angle);
    let radius =
        if part() == TimePart::Hours && hour24 && (draft().hours() == 0 || draft().hours() > 12) {
            64
        } else {
            100
        };
    let labels: Vec<(u8, String, f64, i32)> = if part() == TimePart::Minutes {
        (0..12)
            .map(|i| (i * 5, format!("{:02}", i * 5), f64::from(i) * 30.0, 100))
            .collect()
    } else {
        let mut labels: Vec<_> = (0..12)
            .map(|i| {
                let v = if i == 0 { 12 } else { i };
                (v, v.to_string(), f64::from(i) * 30.0, 100)
            })
            .collect();
        if hour24 {
            labels.extend((0..12).map(|i| {
                let v = if i == 0 { 0 } else { i + 12 };
                (v, format!("{v:02}"), f64::from(i) * 30.0, 64)
            }));
        }
        labels
    };
    let off_tick = part() == TimePart::Minutes && minute % 5 != 0;
    let hour_bad = TimeValue::parse_field(
        &texts()[0],
        if hour24 { 23 } else { 12 },
        if hour24 { 0 } else { 1 },
    )
    .is_none();
    let minute_bad = TimeValue::parse_field(&texts()[1], 59, 0).is_none();
    rsx! {Dialog {title:title.clone(),open:open&&!disabled,class:"m3-time-dialog {class}",initial_focus_id:Some(format!("{id}-hour")),onopenchange,
        div {id:id.clone(),class:picker_class,"data-open":open&&!disabled,"data-mode":if effective_input{"input"}else{"dial"},"data-horizontal":horizontal,"data-field":if part()==TimePart::Hours{"hours"}else{"minutes"},"data-hours":draft().hours(),"data-minutes":minute,"data-hour24":hour24,
            p {class:"m3-time-picker__title","{title}"}
            div {class:"m3-time-picker__layout",
                div {class:"m3-time-picker__selectors",
                    div {class:"m3-time-picker__numbers","data-hour24":hour24,
                        if effective_input {
                            label {class:"m3-time-picker__input-label",
                                input {id:"{id}-hour",class:"m3-time-picker__number",r#type:"text",inputmode:"numeric",autocomplete:"off",maxlength:"2",aria_label:"Hour",aria_invalid:hour_bad,"aria-describedby":"{id}-hour-help","data-part":"0","data-active":part()==TimePart::Hours,value:texts()[0].clone(),
                                    onfocus:move |_|part.set(TimePart::Hours),oninput:move|e|edit.call((0,e.value()))}
                                span {id:"{id}-hour-help","Hour"}
                            }
                        }else {button {id:"{id}-hour",r#type:"button",class:"m3-time-picker__number",aria_label:"Select hours, {shown}",aria_pressed:part()==TimePart::Hours,"data-active":part()==TimePart::Hours,onclick:move |_|part.set(TimePart::Hours),super::ripple::Ripple {}"{shown:02}"}}
                        span {class:"m3-time-picker__separator",aria_hidden:"true",":"}
                        if effective_input {
                            label {class:"m3-time-picker__input-label",
                                input {id:"{id}-minute",class:"m3-time-picker__number",r#type:"text",inputmode:"numeric",autocomplete:"off",maxlength:"2",aria_label:"Minute",aria_invalid:minute_bad,"aria-describedby":"{id}-minute-help","data-part":"1","data-active":part()==TimePart::Minutes,value:texts()[1].clone(),
                                    onfocus:move |_|part.set(TimePart::Minutes),oninput:move|e|edit.call((1,e.value()))}
                                span {id:"{id}-minute-help","Minute"}
                            }
                        }else {button {id:"{id}-minute",r#type:"button",class:"m3-time-picker__number",aria_label:"Select minutes, {minute}",aria_pressed:part()==TimePart::Minutes,"data-active":part()==TimePart::Minutes,onclick:move |_|part.set(TimePart::Minutes),super::ripple::Ripple {}"{minute:02}"}}
                    }
                    if !hour24 {div {class:"m3-time-picker__period",role:"radiogroup",aria_label:"AM or PM",
                        button {r#type:"button",role:"radio",class:"m3-time-picker__period-button",aria_checked:!draft().is_pm(),tabindex:if draft().is_pm(){"-1"}else{"0"},"data-period":"am",onclick:move |_|draft.set(draft().set_period(false)),super::ripple::Ripple {}"AM"}
                        button {r#type:"button",role:"radio",class:"m3-time-picker__period-button",aria_checked:draft().is_pm(),tabindex:if draft().is_pm(){"0"}else{"-1"},"data-period":"pm",onclick:move |_|draft.set(draft().set_period(true)),super::ripple::Ripple {}"PM"}
                    }}
                }
                if !effective_input {div {class:"m3-time-picker__dial",role:"group",aria_label:if part()==TimePart::Hours{"Choose hours"}else{"Choose minutes"},
                    div {class:"m3-time-picker__hand",aria_hidden:"true",style:"transform:rotate({angle}deg)","data-angle":principal,"data-radius":radius,
                        span {class:"m3-time-picker__track",style:"height:{radius}px"}
                        span {class:"m3-time-picker__handle",style:"transform:translate(-50%, {-radius-24}px)",
                            if off_tick {span {class:"m3-time-picker__dot"}}
                        }
                    }
                    span {class:"m3-time-picker__center",aria_hidden:"true"}
                    div {class:"m3-time-picker__labels",
                    for (n,text,angle,r) in labels { {
                        let a=angle.to_radians();let x=128.0+f64::from(r)*a.sin();let y=128.0-f64::from(r)*a.cos();
                        let name=if part()==TimePart::Hours{format!("Hour {n} of {}",if hour24{24}else{12})}else{format!("Minute {n} of 60")};
                        rsx! {button {r#type:"button",class:"m3-time-picker__tick",aria_label:name,aria_pressed:selected==n,tabindex:if selected==n{"0"}else{"-1"},"data-value":n,"data-selected":selected==n,style:"left:{x}px;top:{y}px", "{text}"}}
                    }}
                    if off_tick { {
                        let a=principal.to_radians();let x=128.0+100.0*a.sin();let y=128.0-100.0*a.cos();
                        rsx!{button {r#type:"button",class:"m3-time-picker__tick m3-time-picker__tick--off",aria_label:"Minute {minute} of 60",aria_pressed:true,tabindex:"0","data-value":minute,"data-selected":true,style:"left:{x}px;top:{y}px"}}
                    }}
                    }
                }}
            }
            if effective_input&&!valid {p {class:"m3-time-picker__error",role:"status","Enter an hour from " if hour24{"0 to 23"}else{"1 to 12"} " and a minute from 0 to 59."}}
            div {class:"m3-time-picker__actions",
                button {r#type:"button",class:"m3-time-picker__switch",aria_label:if effective_input{"Switch to dial"}else{"Switch to text input"},disabled:forced_input||effective_input&&!valid,onclick:toggle,super::ripple::Ripple {}Icon {icon:if effective_input{icons::SCHEDULE}else{icons::KEYBOARD}}}
                span {class:"m3-time-picker__spacer"}
                Button {variant:ButtonVariant::Text,onclick:move |_|onopenchange.call(false),"Cancel"}
                Button {variant:ButtonVariant::Text,disabled:!valid,onclick:move |_|{onconfirm.call(draft());onopenchange.call(false);},"OK"}
            }
        }
    }}
}
