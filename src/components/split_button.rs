use super::action_control::ActionControl;
use super::motion::next_id;
use super::{ButtonSize, ButtonVariant, IconData, Menu, MenuColor, MenuEntry, MenuSelection};
use crate::icons;
use dioxus::prelude::*;

/// M3 Expressive split action and related menu. Both halves share emphasis and
/// size; the trailing half exposes expanded state and controls its Menu.
/// Opening the menu changes only shape/state layer, never the container color.
#[component]
pub fn SplitButton(
    label: String,
    entries: Vec<MenuEntry>,
    open: bool,
    #[props(default)] leading_icon: Option<IconData>,
    #[props(default)] icon_only: bool,
    #[props(default)] size: ButtonSize,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    #[props(default)] trailing_disabled: bool,
    #[props(default)] trailing_label: Option<String>,
    #[props(default)] menu_color: MenuColor,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onselect: EventHandler<MenuSelection>,
) -> Element {
    let id = use_hook(|| next_id("m3-split"));
    let trigger = format!("{id}-trigger");
    let menu_id = format!("{id}-menu");
    let size_class = match size {
        ButtonSize::ExtraSmall => "xs",
        ButtonSize::Small => "s",
        ButtonSize::Medium => "m",
        ButtonSize::Large => "l",
        ButtonSize::ExtraLarge => "xl",
    };
    let trailing_label = trailing_label.unwrap_or_else(|| format!("More {label} options"));
    let mut last = use_signal(|| false);
    rsx! {
        div { class: "m3-split m3-split--{size_class} {class}", role: "group", aria_label: label.clone(),
            ActionControl { label: label.clone(), icon: leading_icon, icon_only, variant, size, disabled,
                class: "m3-split__leading", onclick,
            }
            ActionControl { id: trigger.clone(), label: trailing_label.clone(), icon: Some(icons::KEYBOARD_ARROW_DOWN),
                icon_only: true, variant, size, disabled: disabled || trailing_disabled,
                class: "m3-split__trailing", expanded: Some(open), controls: Some(menu_id.clone()),
                onclick: move |_| { last.set(false); onopenchange.call(!open); },
                onkeydown: move |e: KeyboardEvent| {
                    if !disabled && !trailing_disabled && matches!(e.key(), Key::ArrowDown | Key::ArrowUp) {
                        e.prevent_default(); last.set(e.key() == Key::ArrowUp); onopenchange.call(true);
                    }
                },
            }
            Menu { id: Some(menu_id.clone()), anchor_id: trigger.clone(), entries, open, aria_label: Some(trailing_label.clone()),
                color: menu_color, initial_focus_last: last(), onopenchange, onselect,
            }
        }
    }
}
