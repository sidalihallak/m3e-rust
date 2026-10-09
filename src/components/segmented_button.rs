use dioxus::prelude::*;

use super::{Icon, IconData};
use crate::icons;

/// A group of segmented buttons built on native inputs. Single select uses
/// `<input type="radio">` (arrow keys move the selection within `name`); multi
/// select uses `<input type="checkbox">` (Space toggles each button).
///
/// Visuals follow the Material 3 segmented button: one 40dp pill with a 1dp
/// outline, 1dp dividers between items and no gaps. A selected item is filled
/// with secondary-container and shows a check unless it has its own icon.
#[component]
pub fn SegmentedButtonSet(
    #[props(default)] aria_label: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "m3-segmented {class}", role: "group", "aria-label": aria_label,
            {children}
        }
    }
}

/// One button in a [`SegmentedButtonSet`].
///
/// Give every button in a single-select set the same `name`. A selected button
/// shows a check unless it has its own `icon`, as filter chips do. `onchange`
/// reports the new checked state.
#[component]
pub fn SegmentedButton(
    label: String,
    name: String,
    value: String,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] multiple: bool,
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<bool>,
) -> Element {
    let checked_class = if checked { " m3-segmented__item--checked" } else { "" };
    let disabled_class = if disabled { " m3-segmented__item--disabled" } else { "" };
    let class = format!("m3-segmented__item{checked_class}{disabled_class} {class}");
    let input_type = if multiple { "checkbox" } else { "radio" };
    // A selected button shows a check, unless the caller gave it an icon.
    let leading = match (icon, checked) {
        (Some(icon), _) => Some(icon),
        (None, true) => Some(icons::CHECK),
        (None, false) => None,
    };

    rsx! {
        label { class,
            input {
                class: "m3-segmented__input",
                r#type: input_type,
                name,
                value,
                checked,
                disabled,
                // A radio's change event only fires when it becomes checked, so report true.
                onchange: move |event| onchange.call(!multiple || event.checked()),
            }
            span { class: "m3-segmented__state", aria_hidden: "true" }
            span { class: "m3-segmented__content",
                if let Some(icon) = leading {
                    Icon { icon, class: "m3-segmented__icon" }
                }
                span { class: "m3-segmented__label", "{label}" }
            }
        }
    }
}
