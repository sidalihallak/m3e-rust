use dioxus::prelude::*;

use super::motion::next_id;
use super::{Icon, IconData};

/// One tab. Set `icon` for an icon-and-label tab; `disabled` removes it from input.
#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub label: String,
    pub icon: Option<IconData>,
    pub disabled: bool,
}

impl TabItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), icon: None, disabled: false }
    }
}

/// Tab style. `Primary` has a 3dp indicator and a primary label. `Secondary` has
/// a 2dp indicator across the full tab width and an on-surface label.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    #[default]
    Primary,
    Secondary,
}

impl TabsVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

/// A tab bar built on native buttons with `role="tab"`. Arrow keys, Home and End
/// move focus and select the tab (the automatic activation pattern). Only the
/// selected tab is in the tab order.
///
/// The indicator slides to the selected tab. Render the panel for `selected`
/// outside this component.
#[component]
pub fn Tabs(
    items: Vec<TabItem>,
    selected: usize,
    #[props(default)] variant: TabsVariant,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] panel_ids: Vec<String>,
    #[props(default)] tab_ids: Vec<String>,
    #[props(default)] onchange: EventHandler<usize>,
) -> Element {
    let id = use_hook(|| next_id("m3-tabs"));
    let has_icon = items.iter().any(|item| item.icon.is_some());
    let icons_class = if has_icon { " m3-tabs--icons" } else { "" };
    let class = format!("m3-tabs m3-tabs--{}{icons_class} {class}", variant.class());
    let enabled: Vec<usize> = items.iter().enumerate().filter(|(_, item)| !item.disabled).map(|(i, _)| i).collect();
    let selected = enabled.iter().copied().find(|&i| i == selected).or_else(|| enabled.first().copied());

    // The indicator's position follows the selected tab's `aria-selected`. The
    // script is installed once per tab bar, so it survives every re-render.
    use_effect({
        let id = id.clone();
        move || {
            let script = indicator_script(&id);
            spawn(async move {
                if let Err(err) = document::eval(&script).join::<bool>().await {
                    let _ = document::eval(&format!("console.error({:?})", format!("tabs: {err}"))).await;
                }
            });
        }
    });

    use_drop({
        let id = id.clone();
        move || {
            let script = format!("window.__m3TabIndicators?.[{id:?}]?.(); if (window.__m3TabIndicators) delete window.__m3TabIndicators[{id:?}];");
            spawn(async move { let _ = document::eval(&script).await; });
        }
    });

    rsx! {
        div { id: "{id}", class, role: "tablist", "aria-label": aria_label,
            for (index, item) in items.into_iter().enumerate() {
                {
                    let is_selected = Some(index) == selected;
                    let available = enabled.clone();
                    let panel_id = panel_ids.get(index).cloned();
                    let tab_id = tab_ids.get(index).cloned().unwrap_or_else(|| format!("{id}-{index}"));
                    let tab_class = if is_selected { "m3-tabs__tab m3-tabs__tab--selected" } else { "m3-tabs__tab" };
                    let focus_id = id.clone();
                    rsx! {
                        button {
                            id: "{tab_id}",
                            r#type: "button",
                            role: "tab",
                            class: tab_class,
                            "aria-selected": if is_selected { "true" } else { "false" },
                            tabindex: if is_selected { "0" } else { "-1" },
                            disabled: item.disabled,
                            aria_controls: panel_id,
                            onclick: move |_| onchange.call(index),
                            onkeydown: move |event| {
                                let pos = available.iter().position(|&i| i == index).unwrap_or(0);
                                let n = available.len();
                                let target = if n == 0 { None } else { match event.key().to_string().as_str() {
                                    "ArrowRight" => Some(available[(pos + 1) % n]),
                                    "ArrowLeft" => Some(available[(pos + n - 1) % n]),
                                    "Home" => available.first().copied(),
                                    "End" => available.last().copied(),
                                    _ => None,
                                }};
                                if let Some(target) = target {
                                    event.prevent_default();
                                    onchange.call(target);
                                    let script = format!(
                                        "document.getElementById('{focus_id}')?.querySelectorAll('[role=tab]')[{target}]?.focus();"
                                    );
                                    spawn(async move {
                                        let _ = document::eval(&script).await;
                                    });
                                }
                            },
                            super::ripple::Ripple {}
                            if let Some(icon) = item.icon {
                                Icon { icon, class: "m3-tabs__icon" }
                            }
                            span { class: "m3-tabs__label", "{item.label}" }
                        }
                    }
                }
            }
            span { class: "m3-tabs__indicator", aria_hidden: "true" }
        }
    }
}

/// Places the indicator under the selected label, then keeps it in place. The
/// list's `data-ready` attribute reveals the indicator after the first placement,
/// so it does not slide in from the left edge on mount.
fn indicator_script(id: &str) -> String {
    format!(
        r#"
const list = document.getElementById("{id}");
if (!list) return false;
window.__m3TabIndicators = window.__m3TabIndicators || {{}};
window.__m3TabIndicators["{id}"]?.();
let motionFrame = 0;
const place = () => {{
  if (!list.isConnected) return;
  const tab = list.querySelector('[aria-selected="true"]');
  if (!tab) {{ list.removeAttribute("data-ready"); return; }}
  const label = tab.querySelector(".m3-tabs__label");
  if (!label) return;
  const rootRect = list.getBoundingClientRect();
  const tabRect = tab.getBoundingClientRect();
  const labelRect = label.getBoundingClientRect();
  // Keep fractional CSS pixels: offsetLeft/offsetWidth round the label geometry.
  const origin = rootRect.left + list.clientLeft - list.scrollLeft;
  list.style.setProperty("--tabs-left", (tabRect.left - origin) + "px");
  list.style.setProperty("--tabs-width", tabRect.width + "px");
  list.style.setProperty("--tabs-label-left", (labelRect.left - origin) + "px");
  list.style.setProperty("--tabs-label-width", labelRect.width + "px");
  list.setAttribute("data-ready", "");
}};
const resize = new ResizeObserver(place);
const observed = new WeakSet();
const observeTargets = () => {{
  for (const target of [list, ...list.querySelectorAll('.m3-tabs__tab, .m3-tabs__label')]) {{
    if (!observed.has(target)) {{ resize.observe(target); observed.add(target); }}
  }}
}};
const changes = new MutationObserver(() => {{ observeTargets(); place(); }});
changes.observe(list, {{ attributes: true, subtree: true, childList: true,
  characterData: true, attributeFilter: ["aria-selected"] }});
observeTargets();
place();
const enableMotion = () => {{
  place();
  // Paint initial geometry before enabling transitions, so mount never slides from x=0.
  motionFrame = requestAnimationFrame(() => {{
    motionFrame = requestAnimationFrame(() => {{
      if (list.isConnected) list.setAttribute("data-motion", "");
    }});
  }});
}};
if (document.fonts) document.fonts.ready.then(enableMotion); else enableMotion();
window.__m3TabIndicators["{id}"] = () => {{
  resize.disconnect(); changes.disconnect(); cancelAnimationFrame(motionFrame);
}};
return true;
"#
    )
}
