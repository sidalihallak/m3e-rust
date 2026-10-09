use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::{Icon, IconData};
use crate::icons;

/// Chip kind. Colours and behaviour follow Material Web's chip tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChipVariant {
    /// An action, such as "Add to calendar".
    #[default]
    Assist,
    /// A toggle. When selected it shows a check unless a leading icon is given.
    Filter,
    /// A value the user entered. It has a trailing remove action.
    Input,
    /// A suggested action or reply.
    Suggestion,
}

impl ChipVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Assist => "assist",
            Self::Filter => "filter",
            Self::Input => "input",
            Self::Suggestion => "suggestion",
        }
    }
}

/// A chip built on a native `<button>`. Input chips add a second button for
/// removal, because one control cannot hold two actions.
///
/// Set `selected` for filter chips, `elevated` for the elevated style, and
/// `onremove` for input chips. Filter chips report toggles through `onclick`.
#[component]
pub fn Chip(
    label: String,
    #[props(default)] variant: ChipVariant,
    #[props(default)] elevated: bool,
    #[props(default)] selected: bool,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onremove: EventHandler<()>,
) -> Element {
    let mut pressed = use_signal(|| false);
    let elevated_class = if elevated { " m3-chip--elevated" } else { "" };
    let selected_class = if variant == ChipVariant::Filter && selected {
        " m3-chip--selected"
    } else {
        ""
    };
    let pressed_class = if pressed() && !disabled {
        " m3-chip--pressed"
    } else {
        ""
    };
    let disabled_class = if disabled { " m3-chip--disabled" } else { "" };
    let class = format!(
        "m3-chip m3-chip--{}{elevated_class}{selected_class}{pressed_class}{disabled_class} {class}",
        variant.class(),
    );
    // A filter chip shows a check when selected, unless it has its own icon.
    let leading = match (variant, selected, icon) {
        (_, _, Some(icon)) => Some(icon),
        (ChipVariant::Filter, true, None) => Some(icons::CHECK),
        _ => None,
    };
    let aria_pressed = if variant == ChipVariant::Filter {
        Some(if selected { "true" } else { "false" })
    } else {
        None
    };
    let remove_label = format!("Remove {label}");

    rsx! {
        span { class,
            button {
                r#type: "button",
                class: "m3-chip__main",
                disabled,
                "aria-pressed": aria_pressed,
                onpointerdown: move |event| {
                    if !disabled && event.is_primary() && event.trigger_button() == Some(MouseButton::Primary) {
                        pressed.set(true);
                    }
                },
                onpointerup: move |_| pressed.set(false),
                onpointercancel: move |_| pressed.set(false),
                onpointerleave: move |_| pressed.set(false),
                onkeydown: move |event| {
                    if !disabled && event.code() == Code::Space && !event.is_auto_repeating() {
                        pressed.set(true);
                    }
                },
                onkeyup: move |event| {
                    if event.code() == Code::Space {
                        pressed.set(false);
                    }
                },
                onblur: move |_| pressed.set(false),
                onclick: move |event| onclick.call(event),
                if let Some(icon) = leading {
                    Icon { icon, class: "m3-chip__icon" }
                }
                span { class: "m3-chip__label", "{label}" }
            }
            if variant == ChipVariant::Input {
                button {
                    r#type: "button",
                    class: "m3-chip__remove",
                    disabled,
                    "aria-label": remove_label,
                    onclick: move |_| onremove.call(()),
                    Icon { icon: icons::CLOSE, class: "m3-chip__remove-icon" }
                }
            }
        }
    }
}
