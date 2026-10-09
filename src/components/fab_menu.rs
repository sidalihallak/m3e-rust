use dioxus::prelude::*;

use super::{Icon, IconData};
use crate::icons;

/// Colour role for the FAB menu. The open trigger and the items use the
/// matching role: primary, secondary or tertiary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabMenuColor {
    #[default]
    Primary,
    Secondary,
    Tertiary,
}

impl FabMenuColor {
    const fn class(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
        }
    }
}

/// One action in the FAB menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FabMenuItem {
    pub icon: IconData,
    pub label: &'static str,
}

/// A FAB that opens a stack of up to six action items.
///
/// The trigger is a 56dp button that becomes a 20dp close icon on a circle while
/// open. Items are 56dp pills, 4dp apart, and spring in nearest the trigger
/// first with a 35ms stagger. Escape closes the menu, and so does choosing an
/// item. The consumer owns `open`; `onchange` reports the requested state.
#[component]
pub fn FabMenu(
    items: Vec<FabMenuItem>,
    #[props(default)] color: FabMenuColor,
    #[props(default)] open: bool,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onchange: EventHandler<bool>,
    #[props(default)] onselect: EventHandler<usize>,
) -> Element {
    let open_class = if open { " m3-fab-menu--open" } else { "" };
    let class = format!(
        "m3-fab-menu m3-fab-menu--{}{open_class} {class}",
        color.class()
    );
    let trigger_icon = if open {
        icons::CLOSE
    } else {
        icon.unwrap_or(icons::ADD)
    };
    let trigger_glyph_class = if open {
        "m3-fab-menu__glyph m3-fab-menu__glyph--close"
    } else {
        "m3-fab-menu__glyph"
    };
    let count = items.len().min(6);
    let label = aria_label.unwrap_or_else(|| "Actions".to_string());
    let expanded = if open { "true" } else { "false" };

    rsx! {
        div {
            class,
            onkeydown: move |event| {
                if open && event.key() == Key::Escape {
                    onchange.call(false);
                }
            },
            div {
                class: "m3-fab-menu__items",
                role: "menu",
                "aria-hidden": if open { None } else { Some("true") },
                for (index, item) in items.iter().take(count).enumerate() {
                    button {
                        key: "{index}",
                        r#type: "button",
                        role: "menuitem",
                        class: "m3-fab-menu__item",
                        tabindex: if open { "0" } else { "-1" },
                        style: "--i: {count - 1 - index};",
                        onclick: move |_| {
                            onselect.call(index);
                            onchange.call(false);
                        },
                        Icon { icon: item.icon, class: "m3-fab-menu__item-icon" }
                        span { class: "m3-fab-menu__item-label", "{item.label}" }
                    }
                }
            }
            button {
                r#type: "button",
                class: "m3-fab-menu__trigger",
                "aria-haspopup": "menu",
                "aria-expanded": expanded,
                aria_label: label,
                onclick: move |_| onchange.call(!open),
                span { class: trigger_glyph_class,
                    Icon { icon: trigger_icon, class: "m3-fab-menu__trigger-icon" }
                }
            }
        }
    }
}
