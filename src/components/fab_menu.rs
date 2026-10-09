use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{Icon, IconData};
use crate::icons;

static NEXT_MENU_ID: AtomicUsize = AtomicUsize::new(0);

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
    let count = items.len().min(6);
    let label = aria_label.unwrap_or_else(|| "Actions".to_string());
    let expanded = if open { "true" } else { "false" };

    // Each menu gets an id so a document-level pointer listener can tell whether a
    // press landed outside it. The listener only reports; the consumer decides.
    let menu_id = use_hook(|| format!("m3-fab-menu-{}", NEXT_MENU_ID.fetch_add(1, Ordering::Relaxed)));
    let listener_id = menu_id.clone();
    use_hook(move || {
        spawn(async move {
            let mut events = document::eval(&format!(
                r#"
                const root = document.getElementById("{listener_id}");
                const handler = (e) => {{
                    if (root && root.classList.contains("m3-fab-menu--open") && !root.contains(e.target)) {{
                        dioxus.send(true);
                    }}
                }};
                window.__m3FabMenuListeners = window.__m3FabMenuListeners || {{}};
                window.__m3FabMenuListeners["{listener_id}"] = handler;
                document.addEventListener("pointerdown", handler, true);
                "#
            ));
            while events.recv::<bool>().await.is_ok() {
                onchange.call(false);
            }
        });
    });
    let drop_id = menu_id.clone();
    use_drop(move || {
        spawn(async move {
            let _ = document::eval(&format!(
                r#"
                const handler = window.__m3FabMenuListeners && window.__m3FabMenuListeners["{drop_id}"];
                if (handler) document.removeEventListener("pointerdown", handler, true);
                if (window.__m3FabMenuListeners) delete window.__m3FabMenuListeners["{drop_id}"];
                "#
            ))
            .await;
        });
    });

    rsx! {
        div {
            id: "{menu_id}",
            class,
            onkeydown: { let menu_id = menu_id.clone(); move |event| {
                let key = event.key().to_string();
                if key == "Escape" && open {
                    event.prevent_default(); onchange.call(false);
                    focus_trigger(&menu_id);
                } else if matches!(key.as_str(), "ArrowDown" | "ArrowUp" | "Home" | "End") {
                    event.prevent_default();
                    if !open { onchange.call(true); }
                    let script = format!(r#"await new Promise(requestAnimationFrame);
const root = document.getElementById({menu_id:?});
const items = [...root.querySelectorAll('[role="menuitem"]')];
const pos = items.indexOf(document.activeElement);
const key = {key:?};
const n = items.length;
if (n) {{ const next = key === 'Home' ? 0 : key === 'End' ? n-1 : key === 'ArrowDown' ? (pos+1)%n : (pos <= 0 ? n-1 : pos-1); items[next].focus(); }}"#);
                    spawn(async move { let _ = document::eval(&script).await; });
                }
            }},
            button {
                r#type: "button",
                class: "m3-fab-menu__trigger",
                "aria-haspopup": "menu",
                "aria-controls": "{menu_id}-items",
                "aria-expanded": expanded,
                aria_label: label,
                onclick: move |_| onchange.call(!open),
                super::ripple::Ripple {}
                span { class: "m3-fab-menu__glyph m3-fab-menu__glyph--original",
                    Icon { icon: icon.unwrap_or(icons::ADD), class: "m3-fab-menu__trigger-icon" }
                }
                span { class: "m3-fab-menu__glyph m3-fab-menu__glyph--close",
                    Icon { icon: icons::CLOSE, class: "m3-fab-menu__trigger-icon" }
                }
            }
            div {
                id: "{menu_id}-items",
                class: "m3-fab-menu__items",
                aria_label: "Actions",
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
                        onclick: { let menu_id = menu_id.clone(); move |_| {
                            onselect.call(index);
                            onchange.call(false);
                            focus_trigger(&menu_id);
                        }},
                        super::ripple::Ripple {}
                        Icon { icon: item.icon, class: "m3-fab-menu__item-icon" }
                        span { class: "m3-fab-menu__item-label", "{item.label}" }
                    }
                }
            }

        }
    }
}

fn focus_trigger(id: &str) {
    let script = format!("document.getElementById({id:?})?.querySelector('.m3-fab-menu__trigger')?.focus();");
    spawn(async move { let _ = document::eval(&script).await; });
}
