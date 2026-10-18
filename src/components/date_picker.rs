use super::{
    Button, ButtonVariant, Calendar, Dialog, DialogVariant, Icon, TextField,
    anchored::use_popup_runtime,
    calendar::{clamp_month, use_today},
    date::{CivilDate, DateConstraints, DateLocale, DateRange, DateSelection},
    motion::next_id,
};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatePickerView {
    #[default]
    Calendar,
    Input,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DatePickerLayout {
    #[default]
    Adaptive,
    Dialog,
    FullScreen,
}

/// Controlled date picker with a separate native calendar action. Text is
/// normalized only on Enter/blur. Compact opening uses the modal picker.
#[component]
pub fn DatePicker(
    #[props(default="Date".to_string())] label: String,
    #[props(default)] value: Option<CivilDate>,
    #[props(default)] constraints: DateConstraints,
    #[props(default)] locale: DateLocale,
    #[props(default)] today: Option<CivilDate>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<Option<CivilDate>>,
) -> Element {
    let id = use_hook(|| next_id("m3-date-field"));
    let anchor = format!("{id}-calendar");
    let mut open = use_signal(|| false);
    let mut text = use_signal(|| value.map(|d| d.format(locale.format)).unwrap_or_default());
    let compact = super::compact::use_compact();
    let local_today = use_today(today);
    let mut month = use_signal(|| value.or(today));
    use_effect(use_reactive(
        (&value, &locale.format),
        move |(value, format)| {
            text.set(value.map(|d| d.format(format)).unwrap_or_default());
            if let Some(date) = value {
                month.set(Some(date.month_start()));
            }
        },
    ));
    use_effect(use_reactive((&disabled,), move |(disabled,)| {
        if disabled {
            open.set(false);
        }
    }));
    let rules = constraints.clone();
    let commit = use_callback(move |raw: String| {
        if raw.trim().is_empty() {
            text.set(String::new());
            if value.is_some() {
                onchange.call(None);
            }
        } else if let Some(date) =
            CivilDate::parse(&raw, locale.format).filter(|d| rules.allows(*d))
        {
            text.set(date.format(locale.format));
            if value != Some(date) {
                onchange.call(Some(date));
            }
        }
    });
    let invalid = !text().trim().is_empty()
        && CivilDate::parse(&text(), locale.format).is_none_or(|d| !constraints.allows(d));
    let rules = constraints.clone();
    let open_calendar = move |_| {
        if !disabled {
            let date = value.or(*local_today.peek());
            if let Some(date) = date {
                month.set(Some(clamp_month(&rules, date)));
            }
            open.set(!open());
        }
    };
    let choose = move |date: CivilDate| {
        text.set(date.format(locale.format));
        onchange.call(Some(date));
        open.set(false);
    };
    let locale_for_confirm = locale.clone();
    rsx! {div {class:"m3-date-picker {class}",
        DateTextInput {id:id.clone(),label:label.clone(),text:text(),format:locale.format,error:invalid,disabled,oninput:move|v|text.set(v),oncommit:move|v|commit.call(v),
            button {id:anchor.clone(),r#type:"button",class:"m3-date-field__calendar m3-calendar__icon",aria_label:"Choose {label}",disabled,onclick:open_calendar,super::ripple::Ripple {}Icon {icon:icons::CALENDAR_TODAY}}
        }
        if !compact(){
            DockedDatePopup {anchor_id:anchor,position_anchor:id.clone(),title:format!("Choose {label}"),open:open()&&!disabled,onopenchange:move|v|open.set(v),
                if let Some(date)=month().or(local_today()) {Calendar {month:clamp_month(&constraints,date),value:DateSelection::Single(value),constraints:constraints.clone(),locale:locale.clone(),today:local_today(),active:open(),auto_focus:true,onmonthchange:move|v|month.set(Some(v)),onselect:choose}}
            }
        }else {
            DatePickerModal {open:open()&&!disabled,title:Some(format!("Choose {label}")),value:DateSelection::Single(value),constraints:constraints.clone(),locale:locale.clone(),today:local_today(),onopenchange:move|v|open.set(v),onconfirm:move|selection:DateSelection|{if let DateSelection::Single(Some(date))=selection {text.set(date.format(locale_for_confirm.format));onchange.call(Some(date));}}}
        }
    }}
}
#[component]
fn DockedDatePopup(
    anchor_id: String,
    position_anchor: String,
    title: String,
    open: bool,
    onopenchange: EventHandler<bool>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-date-popup"));
    use_popup_runtime(
        id.clone(),
        "",
        EventHandler::new(move |message: Vec<String>| {
            if message.first().is_some_and(|m| m == "open") {
                onopenchange.call(message.get(1).is_some_and(|v| v == "true"));
            }
        }),
    );
    rsx! {div {id,class:"m3-anchor m3-date-popup",role:"dialog",aria_label:title,tabindex:"-1","popover":"manual","data-kind":"popover","data-anchor":anchor_id,"data-position-anchor":position_anchor,"data-open":open,"data-side":"bottom","data-align":"start","data-offset":"4",{children}}}
}

/// Modal draft state is discarded by Cancel/Escape/outside; only OK/Save confirms.
/// The adaptive layout uses a full-screen native dialog below 600px.
#[component]
pub fn DatePickerModal(
    open: bool,
    #[props(default)] value: DateSelection,
    #[props(default)] title: Option<String>,
    #[props(default)] default_view: DatePickerView,
    #[props(default)] layout: DatePickerLayout,
    #[props(default)] constraints: DateConstraints,
    #[props(default)] locale: DateLocale,
    #[props(default)] today: Option<CivilDate>,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onconfirm: EventHandler<DateSelection>,
) -> Element {
    let now = use_today(today);
    if let Some(today) = now() {
        rsx! {DatePickerBody {open,value,title,default_view,layout,constraints,locale,today,class,onopenchange,onconfirm}}
    } else {
        rsx! {}
    }
}
#[component]
fn DatePickerBody(
    open: bool,
    value: DateSelection,
    title: Option<String>,
    default_view: DatePickerView,
    layout: DatePickerLayout,
    constraints: DateConstraints,
    locale: DateLocale,
    today: CivilDate,
    class: String,
    onopenchange: EventHandler<bool>,
    onconfirm: EventHandler<DateSelection>,
) -> Element {
    let id = use_hook(|| next_id("m3-date-modal-body"));
    let mut draft = use_signal(|| value);
    let mut view = use_signal(|| default_view);
    let mut month = use_signal(|| clamp_month(&constraints, value.start().unwrap_or(today)));
    let mut texts = use_signal(|| {
        [
            value
                .start()
                .map(|d| d.format(locale.format))
                .unwrap_or_default(),
            value
                .end()
                .map(|d| d.format(locale.format))
                .unwrap_or_default(),
        ]
    });
    let compact = super::compact::use_compact();
    let fullscreen =
        layout == DatePickerLayout::FullScreen || layout == DatePickerLayout::Adaptive && compact();
    let rules = constraints.clone();
    use_effect(use_reactive((&open,), move |(open,)| {
        if open {
            draft.set(value);
            view.set(default_view);
            month.set(clamp_month(&rules, value.start().unwrap_or(today)));
            texts.set([
                value
                    .start()
                    .map(|d| d.format(locale.format))
                    .unwrap_or_default(),
                value
                    .end()
                    .map(|d| d.format(locale.format))
                    .unwrap_or_default(),
            ]);
        }
    }));
    let range = draft().is_range();
    let title =
        title.unwrap_or_else(|| if range { "Select dates" } else { "Select date" }.to_string());
    let rules = constraints.clone();
    let parse = use_callback(move |(index, raw): (usize, String)| {
        let mut next = texts();
        next[index] = raw;
        texts.set(next.clone());
        let a = CivilDate::parse(&next[0], locale.format).filter(|d| rules.allows(*d));
        let b = CivilDate::parse(&next[1], locale.format).filter(|d| rules.allows(*d));
        draft.set(if range {
            DateSelection::Range(DateRange { start: a, end: b })
        } else {
            DateSelection::Single(a)
        });
    });
    let locale_format = locale.format;
    let normalize = use_callback(move |(index, raw): (usize, String)| {
        let text = CivilDate::parse(&raw, locale_format)
            .map(|d| d.format(locale_format))
            .unwrap_or(raw);
        parse.call((index, text));
    });
    let start = draft().start();
    let end = draft().end();
    let valid = draft().valid(&constraints);
    let backwards = start.zip(end).is_some_and(|(a, b)| a > b);
    let header = if range {
        format!(
            "{} – {}",
            start
                .map(|d| format!(
                    "{} {}",
                    locale.short_months[d.month() as usize - 1],
                    d.day()
                ))
                .unwrap_or("Start date".into()),
            end.map(|d| format!(
                "{} {}",
                locale.short_months[d.month() as usize - 1],
                d.day()
            ))
            .unwrap_or("End date".into())
        )
    } else {
        start
            .map(|d| locale.headline(d))
            .unwrap_or("Enter date".into())
    };
    let format = locale.format;
    let confirm_rules = constraints.clone();
    let confirm = use_callback(move |_: MouseEvent| {
        if draft().valid(&confirm_rules) {
            onconfirm.call(draft());
            onopenchange.call(false);
        }
    });
    let toggle = move |_| {
        if view() == DatePickerView::Calendar {
            let value = draft();
            texts.set([
                value.start().map(|d| d.format(format)).unwrap_or_default(),
                value.end().map(|d| d.format(format)).unwrap_or_default(),
            ]);
            view.set(DatePickerView::Input);
        } else {
            if let Some(date) = draft().start() {
                month.set(date.month_start());
            }
            view.set(DatePickerView::Calendar);
        }
    };
    let rules_for_text = constraints.clone();
    let second_error = backwards
        || start
            .zip(end)
            .is_some_and(|(a, b)| !rules_for_text.allows_range(a, b));
    rsx! {Dialog {title:title.clone(),open,variant:if fullscreen{DialogVariant::FullScreen}else{DialogVariant::Basic},initial_focus_id:Some(format!("{id}-switch")),class:"m3-date-modal {class}",onopenchange,
        header_action:if fullscreen{Some(rsx!{Button {variant:ButtonVariant::Text,disabled:!valid,onclick:move |e|confirm.call(e),"Save"}})}else{None},
        actions:if !fullscreen{Some(rsx!{Button {variant:ButtonVariant::Text,onclick:move |_|onopenchange.call(false),"Cancel"}Button {variant:ButtonVariant::Text,disabled:!valid,onclick:move |e|confirm.call(e),"OK"}})}else{None},
        div {class:"m3-date-modal__header","data-range":range,
            if !fullscreen {p {class:"m3-date-modal__label","{title}"}}
            div {class:"m3-date-modal__headline-row",p {class:"m3-date-modal__headline",role:"status",aria_live:"polite","{header}"}
                button {id:"{id}-switch",r#type:"button",class:"m3-calendar__icon",aria_label:if view()==DatePickerView::Calendar{"Switch to text input"}else{"Switch to calendar"},onclick:toggle,super::ripple::Ripple {}Icon {icon:if view()==DatePickerView::Calendar{icons::EDIT}else{icons::CALENDAR_TODAY}}}
            }
        }
        if view()==DatePickerView::Calendar {
            div {class:"m3-date-modal__calendars",Calendar {id:Some(format!("{id}-calendar")),month:month(),value:draft(),constraints:constraints.clone(),locale:locale.clone(),today:Some(today),active:open,auto_focus:true,onmonthchange:move|v|month.set(v),onselect:move|v|draft.set(draft().choose(v))}
                if range&&fullscreen&&month().add_months(1)!=month() {Calendar {id:Some(format!("{id}-calendar-next")),month:month().add_months(1),value:draft(),constraints:constraints.clone(),locale:locale.clone(),today:Some(today),active:open,auto_focus:false,onmonthchange:move|v:CivilDate|month.set(v.add_months(-1)),onselect:move|v|draft.set(draft().choose(v))}}
            }
        }else {div {class:"m3-date-modal__inputs",
            DateTextInput {id:format!("{id}-input-start"),label:if range{"Start date"}else{"Date"},text:texts()[0].clone(),format:locale.format,error:!texts()[0].is_empty()&&start.is_none(),auto_focus:true,oninput:move|v|parse.call((0,v)),oncommit:move|v|normalize.call((0,v))}
            if range {DateTextInput {id:format!("{id}-input-end"),label:"End date",text:texts()[1].clone(),format:locale.format,error:!texts()[1].is_empty()&&(end.is_none()||second_error),error_message:if backwards{"End date is before the start."}else if second_error{"Range includes an unavailable date."}else{"Enter a valid date."},oninput:move|v|parse.call((1,v)),oncommit:move|v|normalize.call((1,v))}}
        }}
    }}
}
#[component]
fn DateTextInput(
    id: String,
    label: String,
    text: String,
    format: super::date::DateInputFormat,
    #[props(default)] error: bool,
    #[props(default="Enter a valid available date.".to_string())] error_message: String,
    #[props(default)] auto_focus: bool,
    #[props(default)] disabled: bool,
    #[props(default)] oninput: EventHandler<String>,
    #[props(default)] oncommit: EventHandler<String>,
    #[props(default)] children: Element,
) -> Element {
    let mut touched = use_signal(|| false);
    let commit = use_callback(move |raw: String| {
        touched.set(true);
        oncommit.call(raw);
    });
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("date_input.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(raw) = eval.recv::<String>().await {
                    commit.call(raw);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3DateInputs?.[{id:?}]?.();"));
        }
    });
    let show_error = error && touched();
    let help = if show_error {
        error_message
    } else {
        format.hint().to_string()
    };
    rsx! {div {id,class:"m3-date-field","data-auto-focus":auto_focus,
        TextField {label,value:text,input_type:"text",disabled,error:show_error,supporting:Some(help),oninput:move|v|{touched.set(false);oninput.call(v);}}
        {children}
    }}
}
