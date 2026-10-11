use super::{
    Icon, Tooltip,
    date::{CivilDate, DateConstraints, DateLocale, DateSelection},
    motion::next_id,
};
use crate::icons;
use dioxus::prelude::*;

pub(crate) fn use_today(override_date: Option<CivilDate>) -> Signal<Option<CivilDate>> {
    let mut today = use_signal(|| override_date);
    use_effect(use_reactive((&override_date,), move |(value,)| {
        if let Some(date) = value {
            today.set(Some(date));
        } else {
            spawn(async move {
                let mut eval = document::eval(
                    "const d=new Date();dioxus.send([d.getFullYear(),d.getMonth()+1,d.getDate()]);",
                );
                if let Ok((y, m, d)) = eval.recv::<(u16, u8, u8)>().await {
                    today.set(CivilDate::new(y, m, d));
                }
            });
        }
    }));
    today
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum CalendarView {
    #[default]
    Days,
    Months,
    Years,
}
impl CalendarView {
    fn name(self) -> &'static str {
        match self {
            Self::Days => "days",
            Self::Months => "months",
            Self::Years => "years",
        }
    }
}
fn month_allowed(rules: &DateConstraints, date: CivilDate) -> bool {
    let (first, last) = rules.bounds();
    date.year() >= first
        && date.year() <= last
        && rules.min.is_none_or(|min| date.month_end() >= min)
        && rules.max.is_none_or(|max| date.month_start() <= max)
}
pub(crate) fn clamp_month(rules: &DateConstraints, date: CivilDate) -> CivilDate {
    let (first, last) = rules.bounds();
    if first > last {
        return date.month_start();
    }
    let lower = rules
        .min
        .unwrap_or(CivilDate::new(first, 1, 1).unwrap())
        .month_start()
        .max(CivilDate::new(first, 1, 1).unwrap());
    let upper = rules
        .max
        .unwrap_or(CivilDate::new(last, 12, 31).unwrap())
        .month_start()
        .min(CivilDate::new(last, 12, 1).unwrap());
    date.month_start().max(lower).min(upper)
}

/// Controlled Gregorian calendar. Keep month/onmonthchange and value/onselect in
/// application state. Date cells have 40px visuals inside native 48px targets.
#[component]
pub fn Calendar(
    month: CivilDate,
    #[props(default)] value: DateSelection,
    #[props(default)] constraints: DateConstraints,
    #[props(default)] locale: DateLocale,
    #[props(default)] today: Option<CivilDate>,
    #[props(default)] id: Option<String>,
    #[props(default = true)] active: bool,
    #[props(default)] auto_focus: bool,
    #[props(default)] class: String,
    #[props(default)] onmonthchange: EventHandler<CivilDate>,
    #[props(default)] onselect: EventHandler<CivilDate>,
) -> Element {
    let generated = use_hook(|| next_id("m3-calendar"));
    let id = id.unwrap_or(generated);
    let current_today = use_today(today);
    let seed = value
        .start()
        .filter(|d| d.month() == month.month() && d.year() == month.year())
        .or_else(|| today.filter(|d| d.month() == month.month() && d.year() == month.year()))
        .unwrap_or(month.month_start());
    let mut focused = use_signal(|| seed);
    let mut view = use_signal(CalendarView::default);
    let mut revision = use_signal(|| 0u64);
    use_effect(use_reactive((&month,), move |(month,)| {
        let date = *focused.peek();
        let new = CivilDate::new(
            month.year(),
            month.month(),
            date.day()
                .min(CivilDate::days_in_month(month.year(), month.month())),
        )
        .unwrap();
        if new != date {
            focused.set(new);
        }
    }));
    let rules = constraints.clone();
    let event = use_callback(move |message: Vec<String>| {
        let action = message.first().map(String::as_str).unwrap_or("");
        let amount = message
            .get(1)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);
        if action == "view" {
            view.set(if amount == 1 {
                CalendarView::Months
            } else {
                CalendarView::Years
            });
            revision += 1;
            return;
        }
        let date = focused();
        let target = match action {
            "days" => date.add_days(amount),
            "months" => date.add_months(amount),
            "home" => date.month_start(),
            "end" => date.month_end(),
            _ => return,
        };
        let direction = if target < date { -1 } else { 1 };
        let mut next = target;
        let mut found = None;
        for _ in 0..367 {
            if rules.allows(next) {
                found = Some(next);
                break;
            }
            let candidate = next.add_days(direction);
            if candidate == next {
                break;
            }
            next = candidate;
        }
        if let Some(next) = found {
            focused.set(next);
            if next.month() != month.month() || next.year() != month.year() {
                onmonthchange.call(next.month_start());
            }
            revision += 1;
        }
    });
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("calendar.js").replace("__ID__", &format!("{id:?}"));
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
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Calendars?.[{id:?}]?.();")).await;
            });
        }
    });
    let first_day = month.month_start();
    let offset = (first_day.weekday() + 7 - locale.first_weekday % 7) % 7;
    let cells = (usize::from(offset)
        + usize::from(CivilDate::days_in_month(month.year(), month.month())))
    .div_ceil(7)
        * 7;
    let focus_date = if focused().month() == month.month()
        && focused().year() == month.year()
        && constraints.allows(focused())
    {
        Some(focused())
    } else {
        (1..=CivilDate::days_in_month(month.year(), month.month()))
            .filter_map(|d| CivilDate::new(month.year(), month.month(), d))
            .find(|d| constraints.allows(*d))
    };
    let (first_year, last_year) = constraints.bounds();
    let previous = month.add_months(-1);
    let next = month.add_months(1);
    let selected_start = value.start();
    let selected_end = value.end();
    let current = current_today();
    let caption = format!(
        "{} {}",
        locale.months[month.month() as usize - 1],
        month.year()
    );
    let month_name = locale.months[month.month() as usize - 1].clone();
    rsx! {div {id:"{id}",class:"m3-calendar {class}","data-view":view().name(),"data-active":active,"data-auto-focus":auto_focus,"data-focus":focus_date.map(CivilDate::iso).unwrap_or_default(),"data-focus-revision":revision(),
        div {class:"m3-calendar__navigation",
            button {id:"{id}-month",r#type:"button",class:"m3-calendar__caption",aria_label:"Choose month, {month_name}",aria_expanded:view()==CalendarView::Months,aria_haspopup:"listbox",aria_controls:"{id}-months",aria_keyshortcuts:"Shift+M",disabled:view()==CalendarView::Years,onclick:move |_|view.set(if view()==CalendarView::Months{CalendarView::Days}else{CalendarView::Months}),super::ripple::Ripple {}"{month_name}" Icon {icon:icons::ARROW_DROP_DOWN}}
            button {id:"{id}-year",r#type:"button",class:"m3-calendar__caption",aria_label:"Choose year, {month.year()}",aria_expanded:view()==CalendarView::Years,aria_haspopup:"listbox",aria_controls:"{id}-years",aria_keyshortcuts:"Shift+Y",disabled:view()==CalendarView::Months,onclick:move |_|view.set(if view()==CalendarView::Years{CalendarView::Days}else{CalendarView::Years}),super::ripple::Ripple {}"{month.year()}" Icon {icon:icons::ARROW_DROP_DOWN}}
            if view()==CalendarView::Days {
                button {id:"{id}-previous",r#type:"button",class:"m3-calendar__icon",aria_label:"Previous month",aria_keyshortcuts:"PageUp",disabled:previous==month||!month_allowed(&constraints,previous),onclick:move |_|onmonthchange.call(previous.month_start()),super::ripple::Ripple {}Icon {icon:icons::CHEVRON_LEFT}}
                button {id:"{id}-next",r#type:"button",class:"m3-calendar__icon",aria_label:"Next month",aria_keyshortcuts:"PageDown",disabled:next==month||!month_allowed(&constraints,next),onclick:move |_|onmonthchange.call(next.month_start()),super::ripple::Ripple {}Icon {icon:icons::CHEVRON_RIGHT}}
                Tooltip {anchor_id:format!("{id}-previous"),text:"Previous month (Page Up); previous year (Shift + Page Up)",disabled:!active}
                Tooltip {anchor_id:format!("{id}-next"),text:"Next month (Page Down); next year (Shift + Page Down)",disabled:!active}
            }
        }
        span {class:"m3-date__sr",role:"status",aria_live:"polite",aria_atomic:"true","{caption}"}
        span {id:"{id}-keys",class:"m3-date__sr","Use arrows to move dates, Home and End for the first and last date of the month, Page Up and Down for months, Shift plus Page Up and Down for years. Shift M opens months; Shift Y opens years."}
        if view()==CalendarView::Days {table {class:"m3-calendar__grid",role:"grid",aria_label:caption,aria_describedby:"{id}-keys",aria_multiselectable:if value.is_range(){Some("true")}else{None},
            thead {tr {role:"row",for col in 0..7 {th {role:"columnheader",scope:"col",abbr {title:locale.weekdays[(col+locale.first_weekday as usize)%7].clone(),aria_label:locale.weekdays[(col+locale.first_weekday as usize)%7].clone(),"{locale.weekdays[(col+locale.first_weekday as usize)%7].chars().next().unwrap_or(' ')}"}}}}}
            tbody {for row in 0..cells/7 {tr {role:"row",for col in 0..7 { {
                let index=row*7+col;let number=index as i32-i32::from(offset)+1;
                if number<1||number>i32::from(CivilDate::days_in_month(month.year(),month.month())){rsx!{td {role:"gridcell",class:"m3-calendar__blank"}}}
                else {let date=CivilDate::new(month.year(),month.month(),number as u8).unwrap();let start=selected_start==Some(date);let end=selected_end==Some(date);let middle=selected_start.zip(selected_end).is_some_and(|(a,b)|date>a&&date<b);let selected=start||end||middle;let range=value.is_range()&&selected_end.is_some();let disabled=!constraints.allows(date);
                    rsx!{td {role:"gridcell",aria_selected:selected,class:"m3-calendar__cell","data-range":range,"data-start":start,"data-end":end,"data-middle":middle,"data-single":start&&!range,
                        button {id:"{id}-day-{date.iso()}",r#type:"button",class:"m3-calendar__day",aria_label:locale.full_date(date),aria_current:if current==Some(date){Some("date")}else{None},tabindex:if focus_date==Some(date){"0"}else{"-1"},disabled,"data-date":date.iso(),"data-selected":start||end,"data-today":current==Some(date),"data-middle":middle,
                            onfocus:move |_|{if *focused.peek()!=date{focused.set(date);}},onclick:move |_|onselect.call(date),super::ripple::Ripple {}span {"{number}"}
                        }
                    }}
                }
            } }}}}
        }} else if view()==CalendarView::Months {div {id:"{id}-months",class:"m3-calendar__months",role:"listbox",aria_label:"Month",for (index,name) in locale.months.iter().enumerate(){ {let date=CivilDate::new(month.year(),index as u8+1,1).unwrap();rsx!{button {r#type:"button",role:"option",class:"m3-calendar__month-option",aria_selected:date.month()==month.month(),disabled:!month_allowed(&constraints,date),tabindex:if date.month()==month.month(){"0"}else{"-1"},onclick:move |_|{onmonthchange.call(date);view.set(CalendarView::Days);revision+=1;},super::ripple::Ripple {}"{name}"}}} }}}
        else {div {id:"{id}-years",class:"m3-calendar__years",role:"listbox",aria_label:"Year",for year in first_year..=last_year { {let date=clamp_month(&constraints,CivilDate::new(year,month.month(),1).unwrap());rsx!{button {r#type:"button",role:"option",class:"m3-calendar__year-option",aria_selected:year==month.year(),tabindex:if year==month.year(){"0"}else{"-1"},onclick:move |_|{onmonthchange.call(date);view.set(CalendarView::Days);revision+=1;},super::ripple::Ripple {}span {"{year}"}}} }}}}
    }}
}
