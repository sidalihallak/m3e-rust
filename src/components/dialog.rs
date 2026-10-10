use super::action_control::ActionControl;
use super::motion::next_id;
use super::{ButtonVariant, Icon, IconData};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogVariant {
    #[default]
    Basic,
    FullScreen,
    Adaptive,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogRole {
    #[default]
    Dialog,
    Alert,
}

/// Controlled native modal dialog. Native showModal supplies background
/// inertness; the runtime manages focus, scroll locking and interruptible motion.
/// Use AlertDialog for urgent confirmations. `actions` stays outside the
/// scrollable body. FullScreen supplies a 56px header and close affordance.
#[component]
pub fn Dialog(
    title: String,
    open: bool,
    #[props(default)] description: Option<String>,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] variant: DialogVariant,
    #[props(default)] role: DialogRole,
    #[props(default)] show_close: bool,
    #[props(default = true)] dismiss_on_outside: bool,
    #[props(default = true)] dismiss_on_escape: bool,
    #[props(default)] initial_focus_id: Option<String>,
    #[props(default)] header_action: Option<Element>,
    #[props(default)] actions: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-dialog"));
    let compact = super::compact::use_compact();
    let fullscreen =
        variant == DialogVariant::FullScreen || (variant == DialogVariant::Adaptive && compact());
    let footer = if variant == DialogVariant::Adaptive && !fullscreen {
        actions.or_else(|| header_action.clone())
    } else {
        actions
    };
    let variant_class = if fullscreen { "fullscreen" } else { "basic" };
    let role = if role == DialogRole::Alert {
        "alertdialog"
    } else {
        "dialog"
    };
    let has_description = description.is_some();
    use_effect({
        let id = id.clone();
        move || {
            let script = DIALOG_SCRIPT.replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(_) = eval.recv::<bool>().await {
                    onopenchange.call(false);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Dialogs?.[{id:?}]?.();")).await;
            });
        }
    });
    rsx! {
        dialog { id: "{id}", class: "m3-dialog m3-dialog--{variant_class} {class}", role,
            aria_modal: "true", aria_labelledby: "{id}-title",
            aria_describedby: if has_description { Some(format!("{id}-description")) } else { None },
            "data-open": open, "data-dismiss-outside": dismiss_on_outside,
            "data-dismiss-escape": dismiss_on_escape, "data-initial-focus": initial_focus_id,
            div { class: "m3-dialog__scrim", aria_hidden: "true" }
            div { class: "m3-dialog__surface",
                div { class: "m3-dialog__header",
                    if fullscreen {
                        ActionControl { label: "Close dialog", icon: Some(icons::CLOSE), icon_only: true,
                            variant: ButtonVariant::Text, class: "m3-dialog__close",
                            onclick: move |_| onopenchange.call(false),
                        }
                    }
                    if !fullscreen { if let Some(icon) = icon { Icon { icon, class: "m3-dialog__icon" } } }
                    h2 { id: "{id}-title", class: "m3-dialog__title", "{title}" }
                    if fullscreen { if let Some(action) = header_action { {action} } }
                }
                div { class: "m3-dialog__body", tabindex: "-1",
                    if let Some(description) = description { p { id: "{id}-description", class: "m3-dialog__description", "{description}" } }
                    {children}
                }
                if let Some(actions) = footer {
                    div { class: "m3-dialog__actions", {actions} }
                }
                if !fullscreen && show_close {
                    ActionControl { label: "Close dialog", icon: Some(icons::CLOSE), icon_only: true,
                        variant: ButtonVariant::Text, class: "m3-dialog__close m3-dialog__close--corner",
                        onclick: move |_| onopenchange.call(false),
                    }
                }
            }
        }
    }
}

/// Urgent basic confirmation. Scrim activation is ignored by default; Escape
/// and explicit actions remain available. Put the cancel action first for a
/// safe initial focus; the consumer owns its confirmation logic and open state.
#[component]
pub fn AlertDialog(
    title: String,
    open: bool,
    #[props(default)] description: Option<String>,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] actions: Option<Element>,
    #[props(default)] initial_focus_id: Option<String>,
    #[props(default)] onopenchange: EventHandler<bool>,
    children: Element,
) -> Element {
    rsx! { Dialog { title, open, description, icon, actions, initial_focus_id,
        role: DialogRole::Alert, dismiss_on_outside: false, onopenchange, {children}
    }}
}

/// An accessible 48px target around the standard 40px dialog text action.
#[component]
pub fn DialogAction(
    label: String,
    #[props(default)] disabled: bool,
    #[props(default)] id: Option<String>,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! { ActionControl { label, id, variant: ButtonVariant::Text, disabled,
        class: "m3-dialog-action", onclick,
    }}
}
const DIALOG_SCRIPT: &str = include_str!("dialog.js");
