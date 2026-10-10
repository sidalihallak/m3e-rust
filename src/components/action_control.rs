use super::{ButtonShape, ButtonSize, ButtonVariant, Icon, IconData};
use dioxus::prelude::*;

/// Shared native control for composite buttons. The accepted standalone Button
/// keeps its original lifecycle. Copy this helper with action-control.css.
#[component]
pub(crate) fn ActionControl(
    label: String,
    #[props(default)] id: Option<String>,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] icon_only: bool,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] shape: ButtonShape,
    #[props(default)] pressed: Option<bool>,
    #[props(default)] expanded: Option<bool>,
    #[props(default)] controls: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
) -> Element {
    let variant = match variant {
        ButtonVariant::Filled => "filled",
        ButtonVariant::Tonal => "tonal",
        ButtonVariant::Outlined => "outlined",
        ButtonVariant::Elevated => "elevated",
        ButtonVariant::Text => "text",
    };
    let size = match size {
        ButtonSize::ExtraSmall => "xs",
        ButtonSize::Small => "s",
        ButtonSize::Medium => "m",
        ButtonSize::Large => "l",
        ButtonSize::ExtraLarge => "xl",
    };
    let shape = if shape == ButtonShape::Round {
        "round"
    } else {
        "square"
    };
    let toggle = if pressed.is_some() {
        " m3-action--toggle"
    } else {
        ""
    };
    let icon_class = if icon_only && icon.is_some() {
        " m3-action--icon-only"
    } else {
        ""
    };
    rsx! {
        button { id, class: "m3-action m3-action--{variant} m3-action--{size} m3-action--{shape}{toggle}{icon_class} {class}",
            r#type: "button", disabled, aria_label: label.clone(), aria_pressed: pressed,
            aria_expanded: expanded, aria_controls: controls,
            aria_haspopup: expanded.map(|_| "menu"), onclick, onkeydown,
            super::ripple::Ripple {}
            span { class: "m3-action__content",
                if let Some(icon) = icon { Icon { icon, class: "m3-action__icon" } }
                if !icon_only || icon.is_none() { span { "{label}" } }
            }
        }
    }
}
