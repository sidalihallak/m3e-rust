use dioxus::prelude::*;
use super::{ButtonShape, ButtonSize, Icon, IconData};
use super::motion::next_id;

/// An option in a connected group. Labels also name icon-only buttons.
#[derive(Clone, Debug, PartialEq)]
pub struct ConnectedButtonItem {
    pub label: String,
    pub icon: Option<IconData>,
    pub icon_only: bool,
    pub disabled: bool,
}
impl ConnectedButtonItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), icon: None, icon_only: false, disabled: false }
    }
    pub fn icon(label: impl Into<String>, icon: IconData) -> Self {
        Self { icon: Some(icon), icon_only: true, ..Self::new(label) }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupSelection { #[default] Single, Multiple }

/// Container styles appropriate for connected toggle buttons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConnectedButtonVariant { #[default] Filled, Tonal, Outlined, Elevated }

/// Controlled Material 3 Expressive connected toggle buttons.
///
/// `selected` contains item indices. With `selection_required`, activation cannot
/// remove the final selection; initialize the caller's state with a selection.
/// Arrow keys move focus (RTL-aware); Space/Enter activate the focused option.
/// Tab reaches every enabled button, and the container is never a tab stop.
/// All buttons share size, shape and variant. Groups stay on one line; use a
/// suitable maximum width and concise labels in the consuming layout.
/// Copy with button-group.css, ripple.css, icon.css, ripple.rs and motion.rs.
#[component]
pub fn ConnectedButtonGroup(
    items: Vec<ConnectedButtonItem>,
    #[props(default)] selected: Vec<usize>,
    #[props(default)] selection: ButtonGroupSelection,
    #[props(default)] selection_required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] size: ButtonSize,
    #[props(default)] shape: ButtonShape,
    #[props(default)] variant: ConnectedButtonVariant,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<Vec<usize>>,
) -> Element {
    let id = use_hook(|| next_id("m3-connected"));
    let mut selected: Vec<usize> = selected.into_iter().filter(|i| *i < items.len()).collect();
    selected.sort_unstable(); selected.dedup();
    if selection == ButtonGroupSelection::Single { selected.truncate(1); }
    let size = match size { ButtonSize::ExtraSmall => "xs", ButtonSize::Small => "s", ButtonSize::Medium => "m", ButtonSize::Large => "l", ButtonSize::ExtraLarge => "xl" };
    let shape = if shape == ButtonShape::Round { "round" } else { "square" };
    let variant = match variant { ConnectedButtonVariant::Filled => "filled", ConnectedButtonVariant::Tonal => "tonal", ConnectedButtonVariant::Outlined => "outlined", ConnectedButtonVariant::Elevated => "elevated" };
    rsx! {
        div { id: "{id}", class: "m3-connected m3-connected--{size} m3-connected--{shape} m3-connected--{variant} {class}", role: "group", aria_label,
            onkeydown: { let id = id.clone(); move |event| {
                let key = event.key().to_string();
                if matches!(key.as_str(), "ArrowLeft" | "ArrowRight" | "Home" | "End") {
                    event.prevent_default();
                    let script = format!(r#"
const root = document.getElementById({id:?});
const buttons = [...root.querySelectorAll('button:not(:disabled)')];
const n = buttons.length;
const pos = buttons.indexOf(document.activeElement);
const key = {key:?};
const delta = (key === 'ArrowRight' ? 1 : -1) * (getComputedStyle(root).direction === 'rtl' ? -1 : 1);
if (n) buttons[key === 'Home' ? 0 : key === 'End' ? n-1 : (pos + delta + n) % n]?.focus();
"#);
                    spawn(async move { let _ = document::eval(&script).await; });
                }
            }},
            for (index, item) in items.into_iter().enumerate() {
                { let current = selected.clone(); let is_selected = current.contains(&index);
                  let item_disabled = disabled || item.disabled;
                  let icon_class = if item.icon_only && item.icon.is_some() { " m3-connected__button--icon-only" } else { "" };
                  rsx! {
                    button { key: "{index}", r#type: "button", class: "m3-connected__button{icon_class}", disabled: item_disabled,
                        aria_label: item.label.clone(), aria_pressed: is_selected,
                        onclick: move |_| {
                            if item_disabled { return; }
                            let mut next = current.clone();
                            if is_selected {
                                if selection_required && next.len() == 1 { return; }
                                next.retain(|i| *i != index);
                            } else if selection == ButtonGroupSelection::Single { next = vec![index]; }
                            else { next.push(index); next.sort_unstable(); }
                            onchange.call(next);
                        },
                        super::ripple::Ripple {}
                        span { class: "m3-connected__content",
                            if let Some(icon) = item.icon { Icon { icon, class: "m3-connected__icon" } }
                            if !item.icon_only || item.icon.is_none() { span { "{item.label}" } }
                        }
                    }
                  }
                }
            }
        }
    }
}
