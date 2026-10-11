use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;

const DATE_PICKER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{CivilDate, DatePicker};
#[component]
fn AppointmentDate() -> Element {
    let mut date = use_signal(|| None::<CivilDate>);
    rsx! { DatePicker { label: "Appointment date", value: date(),
        onchange: move |value| date.set(value) } }
}"#;
const DATE_MODAL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, DatePickerModal, DateSelection};
#[component]
fn ChooseDate() -> Element {
    let mut open = use_signal(|| false);
    let mut selected = use_signal(DateSelection::default);
    rsx! {
        Button { onclick: move |_| open.set(true), "Choose date" }
        DatePickerModal { open: open(), value: selected(),
            onopenchange: move |value| open.set(value),
            onconfirm: move |value| selected.set(value) }
    }
}"#;
const RANGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, DatePickerModal, DateRange, DateSelection, DatePickerView};
#[component]
fn TravelDates() -> Element {
    let mut open = use_signal(|| false);
    let mut selected = use_signal(|| DateSelection::Range(DateRange::default()));
    rsx! {
        Button { onclick: move |_| open.set(true), "Choose travel dates" }
        DatePickerModal { open: open(), value: selected(), default_view: DatePickerView::Input,
            onopenchange: move |value| open.set(value),
            onconfirm: move |value| selected.set(value) }
    }
}"#;
const CALENDAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Calendar, CivilDate, DateSelection};
#[component]
fn InlineCalendar() -> Element {
    let mut month = use_signal(|| CivilDate::new(2026, 10, 1).unwrap());
    let mut selected = use_signal(DateSelection::default);
    rsx! { Calendar { month: month(), value: selected(),
        onmonthchange: move |value| month.set(value),
        onselect: move |value| selected.set(DateSelection::Single(Some(value))) } }
}"#;

const TIME_PICKER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, TimePicker, TimeValue};
#[component]
fn MeetingTime() -> Element {
    let mut open = use_signal(|| false);
    let mut time = use_signal(TimeValue::default);
    rsx! {
        Button { onclick: move |_| open.set(true), "Choose meeting time" }
        TimePicker { open: open(), value: time(),
            onopenchange: move |value| open.set(value),
            onconfirm: move |value| time.set(value) }
    }
}"#;
const TIME_INPUT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, TimePicker, TimePickerMode, TimeValue};
#[component]
fn AlarmTime() -> Element {
    let mut open = use_signal(|| false);
    let mut time = use_signal(|| TimeValue::new(23, 45).unwrap());
    rsx! {
        Button { onclick: move |_| open.set(true), "Set alarm" }
        TimePicker { open: open(), value: time(), hour24: true,
            default_mode: TimePickerMode::Input,
            onopenchange: move |value| open.set(value),
            onconfirm: move |value| time.set(value) }
    }
}"#;

#[component]
pub fn SchedulingGallery(active: String) -> Element {
    let gallery_class = "roles-section wrap feedback-gallery";
    let initial = CivilDate::new(2026, 10, 11).unwrap();
    let mut date = use_signal(|| Some(initial));
    let mut modal = use_signal(|| false);
    let mut input = use_signal(|| false);
    let mut range_open = use_signal(|| false);
    let mut selected = use_signal(|| DateSelection::Single(Some(initial)));
    let mut range = use_signal(|| DateSelection::Range(DateRange::default()));
    let mut month = use_signal(|| initial.month_start());
    let mut calendar_value = use_signal(DateSelection::default);
    let mut rtl_month = use_signal(|| initial.month_start());
    let mut rtl_value = use_signal(DateSelection::default);
    let mut time_open = use_signal(|| false);
    let mut time = use_signal(TimeValue::default);
    let mut hour24 = use_signal(|| false);
    let mut time_input = use_signal(|| false);
    let mut disabled_time = use_signal(|| false);
    let appointment = date()
        .map(CivilDate::iso)
        .unwrap_or_else(|| String::from("None"));
    rsx! {
        section { class: gallery_class, hidden: active != "time-pickers", id: "time-pickers",
            p { "Choose hours, then minutes. Use arrows on the clock or switch to keyboard input. Changes are saved only with OK." }
            div { class: "feedback-row",
                Button { onclick: move |_| {hour24.set(false);time_input.set(false);time_open.set(true);}, "Open time picker" }
                Button { variant: ButtonVariant::Outlined, onclick: move |_| {hour24.set(true);time_input.set(false);time_open.set(true);}, "Open 24-hour picker" }
                Button { variant: ButtonVariant::Outlined, onclick: move |_| {hour24.set(true);time_input.set(true);time_open.set(true);}, "Enter a time" }
                Button { disabled: true, onclick: move |_| disabled_time.set(true), "Unavailable time" }
            }
            p { role: "status", "Confirmed time: {time().format(true)}" }
            TimePicker { open: time_open(), value: time(), hour24: hour24(), default_mode: if time_input() {TimePickerMode::Input} else {TimePickerMode::Dial},
                onopenchange: move |v| time_open.set(v), onconfirm: move |v| time.set(v) }
            TimePicker { open: disabled_time(), disabled: true }
            CodeCard { eyebrow: "SCHEDULING", title: "Dial time picker", code: TIME_PICKER_USAGE }
            CodeCard { eyebrow: "SCHEDULING", title: "24-hour time input", code: TIME_INPUT_USAGE }
        }
        section { class: gallery_class, hidden: active != "date-pickers", id: "date-pickers",
            p { "Use the calendar action or type a date. Enter or blur applies formatting; modal changes are saved only with OK or Save." }
            div { class: "feedback-row",
                DatePicker { label: "Appointment date", value: date(), onchange: move |v| date.set(v) }
                DatePicker { label: "Unavailable date", disabled: true, value: Some(initial) }
            }
            p { role: "status", "Appointment: {appointment}" }
            div { class: "feedback-row",
                Button { onclick: move |_| { input.set(false); modal.set(true); }, "Open date picker" }
                Button { variant: ButtonVariant::Outlined, onclick: move |_| { input.set(true); modal.set(true); }, "Enter a date" }
                Button { variant: ButtonVariant::Outlined, onclick: move |_| range_open.set(true), "Choose date range" }
            }
            p { role: "status", "Confirmed date: {selected().start().map(CivilDate::iso).unwrap_or_default()}" }
            p { role: "status", "Confirmed range: {range().start().map(CivilDate::iso).unwrap_or_default()} – {range().end().map(CivilDate::iso).unwrap_or_default()}" }
            DatePickerModal { open: modal(), value: selected(), default_view: if input() {DatePickerView::Input} else {DatePickerView::Calendar},
                onopenchange: move |v| modal.set(v), onconfirm: move |v| selected.set(v) }
            DatePickerModal { open: range_open(), value: range(), onopenchange: move |v| range_open.set(v), onconfirm: move |v| range.set(v) }
            CodeCard { eyebrow: "SCHEDULING", title: "Docked date picker", code: DATE_PICKER_USAGE }
            CodeCard { eyebrow: "SCHEDULING", title: "Modal date picker", code: DATE_MODAL_USAGE }
            CodeCard { eyebrow: "SCHEDULING", title: "Date range with text entry", code: RANGE_USAGE }
        }
        section { class: gallery_class, hidden: active != "calendars", id: "calendars",
            p { "48px date targets, a 40px selected circle, separate month and year navigation, and arrow-key navigation." }
            div { class: "feedback-row",
                Calendar { month: month(), value: calendar_value(), onmonthchange: move |v| month.set(v),
                    onselect: move |v| calendar_value.set(DateSelection::Single(Some(v))),
                    constraints: DateConstraints { disabled_weekdays: vec![0,6], ..Default::default() } }
                div { dir: "rtl", Calendar { month: rtl_month(), value: rtl_value(), onmonthchange: move |v| rtl_month.set(v), onselect: move |v| rtl_value.set(DateSelection::Single(Some(v))) } }
            }
            CodeCard { eyebrow: "SCHEDULING", title: "Controlled calendar", code: CALENDAR_USAGE }
        }
    }
}
