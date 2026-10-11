use super::{Icon, IconData, motion::next_id};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Neutral,
    Info,
    Tertiary,
    Error,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertRole {
    #[default]
    Status,
    Alert,
    Note,
}
#[component]
pub fn Alert(
    title: String,
    description: String,
    #[props(default)] variant: AlertVariant,
    #[props(default)] role: AlertRole,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] action: Option<Element>,
    #[props(default)] dismissible: bool,
    #[props(default)] class: String,
    #[props(default)] onclose: EventHandler<()>,
) -> Element {
    let tone = match variant {
        AlertVariant::Neutral => "neutral",
        AlertVariant::Info => "info",
        AlertVariant::Tertiary => "tertiary",
        AlertVariant::Error => "error",
    };
    rsx! {div {class:"m3-alert {class}",role:match role{AlertRole::Status=>"status",AlertRole::Alert=>"alert",AlertRole::Note=>"note"},"data-tone":tone,
        if let Some(icon)=icon {span {class:"m3-alert__icon",Icon {icon}}}
        div {class:"m3-alert__text",h3 {"{title}"}p {"{description}"}}
        if let Some(action)=action {div {class:"m3-alert__action",{action}}}
        if dismissible {button {r#type:"button",class:"m3-feedback__icon",aria_label:"Dismiss {title}",onclick:move |_|onclose.call(()),super::ripple::Ripple {} Icon {icon:icons::CLOSE}}}
    }}
}
/// One controlled Material snackbar, with upstream fast scale/fade motion. Actionable messages never auto-dismiss.
/// Focused dismissal returns to the triggering action; overlapping pause releases keep one timer.
/// Auto-dismiss is opt-in (duration_ms > 0) and requires equivalent inline feedback.
#[component]
pub fn Snackbar(
    open: bool,
    message: String,
    #[props(default)] action_label: Option<String>,
    #[props(default = true)] dismissible: bool,
    #[props(default)] duration_ms: u32,
    #[props(default)] revision: u64,
    #[props(default)] action_new_line: bool,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onaction: EventHandler<()>,
) -> Element {
    let id = use_hook(|| next_id("m3-snackbar"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("snackbar.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while eval.recv::<bool>().await.is_ok() {
                    onopenchange.call(false);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Snackbars?.[{id:?}]?.();")).await;
            });
        }
    });
    let duration = if action_label.is_some() || duration_ms == 0 {
        0
    } else {
        duration_ms.clamp(4000, 10000)
    };
    rsx! {div {id:"{id}",class:"m3-snackbar {class}",role:"region",aria_label:"Notifications",aria_keyshortcuts:"Alt+G",tabindex:"-1",aria_hidden:if !open{Some("true")}else{None},inert:if !open{Some("")}else{None},
        "data-open":open,"data-message":message.clone(),"data-revision":revision,"data-duration":duration,"data-new-line":action_new_line,
        span {class:"m3-feedback__sr",role:"status",aria_live:"polite",aria_atomic:"true","data-snackbar-announcement":"true"}
        p {class:"m3-snackbar__message","{message}"}
        if let Some(label)=action_label {button {r#type:"button",class:"m3-snackbar__action",onclick:move |_|{onaction.call(());onopenchange.call(false);},super::ripple::Ripple {}"{label}"}}
        if dismissible {button {r#type:"button",class:"m3-feedback__icon",aria_label:"Dismiss notification",onclick:move |_|onopenchange.call(false),super::ripple::Ripple {}Icon {icon:icons::CLOSE}}}
    }}
}
/// Toast naming compatibility with the single-message Material snackbar contract.
#[component]
pub fn Toast(
    open: bool,
    message: String,
    #[props(default)] duration_ms: u32,
    #[props(default)] revision: u64,
    #[props(default)] action_label: Option<String>,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onaction: EventHandler<()>,
) -> Element {
    rsx! {Snackbar {open,message,duration_ms,revision,action_label,onopenchange,onaction}}
}
