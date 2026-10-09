use dioxus::prelude::*;

use super::{Icon, IconData};

/// Icon button style. Colours follow Material Web's icon button tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    #[default]
    Standard,
    Filled,
    Tonal,
    Outlined,
}

impl IconButtonVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Filled => "filled",
            Self::Tonal => "tonal",
            Self::Outlined => "outlined",
        }
    }
}

/// Container sizes: 32, 40, 56, 96 and 136dp, with 20, 24, 24, 32 and 40dp icons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonSize {
    ExtraSmall,
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl IconButtonSize {
    const fn class(self) -> &'static str {
        match self {
            Self::ExtraSmall => "xs",
            Self::Small => "s",
            Self::Medium => "m",
            Self::Large => "l",
            Self::ExtraLarge => "xl",
        }
    }
}

/// Round is a circle; square uses the medium corner for the size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonShape {
    #[default]
    Round,
    Square,
}

impl IconButtonShape {
    const fn class(self) -> &'static str {
        match self {
            Self::Round => "round",
            Self::Square => "square",
        }
    }
}

/// Container width at each Expressive size, from current Material Web tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonWidth { Narrow, #[default] Default, Wide }
impl IconButtonWidth { const fn class(self) -> &'static str { match self { Self::Narrow => "narrow", Self::Default => "default", Self::Wide => "wide" } } }

/// An icon button built on a native `<button>`.
///
/// The button needs an `aria_label` because the icon is hidden from assistive
/// technology. Set `toggle` to make it a toggle button; `selected` then drives
/// `aria-pressed`, the selected colours and the selected corner. Pressing morphs
/// the corner to a smaller radius, as the expressive press shape does.
#[component]
pub fn IconButton(
    icon: IconData,
    #[props(default)] variant: IconButtonVariant,
    #[props(default)] size: IconButtonSize,
    #[props(default)] shape: IconButtonShape,
    #[props(default)] width: IconButtonWidth,
    #[props(default)] selected_icon: Option<IconData>,
    #[props(default)] toggle: bool,
    #[props(default)] selected: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let icon = if toggle && selected { selected_icon.unwrap_or(icon) } else { icon };
    let toggle_class = if toggle { " m3-icon-button--toggle" } else { "" };
    let selected_class = if toggle && selected {
        " m3-icon-button--selected"
    } else {
        ""
    };
    let disabled_class = if disabled { " m3-icon-button--disabled" } else { "" };
    let class = format!(
        "m3-icon-button m3-icon-button--{} m3-icon-button--{} m3-icon-button--{} m3-icon-button--width-{}{toggle_class}{selected_class}{disabled_class} {class}",
        variant.class(),
        size.class(),
        shape.class(),
        width.class(),
    );
    let aria_pressed = if toggle {
        Some(if selected { "true" } else { "false" })
    } else {
        None
    };

    rsx! {
        button {
            r#type: "button",
            class,
            disabled,
            "aria-pressed": aria_pressed,
            aria_label,
            onclick: move |event| onclick.call(event),
            super::ripple::Ripple {}
            Icon { icon, class: "m3-icon-button__icon" }
        }
    }
}
