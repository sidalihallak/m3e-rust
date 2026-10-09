use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use super::{Icon, IconData};

/// Floating action button size. Icon-only containers follow Material Web's FAB
/// tokens: 40/56/80/96dp with 24/24/28/36dp icons. An extended FAB (a label is
/// given) uses 56/56/80/96dp heights, so its small size differs from the icon-only one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabSize {
    Small,
    #[default]
    Standard,
    Medium,
    Large,
}

impl FabSize {
    const fn class(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Standard => "standard",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Container colour role. The icon takes the paired `on-` role.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabColor {
    #[default]
    PrimaryContainer,
    Primary,
    SecondaryContainer,
    Secondary,
    TertiaryContainer,
    Tertiary,
    Surface,
}

impl FabColor {
    const fn class(self) -> &'static str {
        match self {
            Self::PrimaryContainer => "primary-container",
            Self::Primary => "primary",
            Self::SecondaryContainer => "secondary-container",
            Self::Secondary => "secondary",
            Self::TertiaryContainer => "tertiary-container",
            Self::Tertiary => "tertiary",
            Self::Surface => "surface",
        }
    }
}

/// A floating action button built on a native `<button>`.
///
/// Pass `label` to make it an extended FAB. Icon-only buttons need `aria_label`,
/// because the icon is hidden from assistive technology.
#[component]
pub fn Fab(
    icon: IconData,
    #[props(default)] size: FabSize,
    #[props(default)] color: FabColor,
    #[props(default)] lowered: bool,
    /// Branded FAB: surface container with a larger icon (36dp standard, 48dp large).
    #[props(default)] branded: bool,
    /// Toolbar floating FAB: elevation 1 at rest (2 when medium). Use the
    /// secondary-container or tertiary-container colour.
    #[props(default)] toolbar: bool,
    #[props(default)] disabled: bool,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let mut pressed = use_signal(|| false);
    let extended_class = if label.is_some() { " m3-fab--extended" } else { "" };
    let lowered_class = if lowered { " m3-fab--lowered" } else { "" };
    let branded_class = if branded { " m3-fab--branded" } else { "" };
    let toolbar_class = if toolbar { " m3-fab--toolbar" } else { "" };
    let pressed_class = if pressed() && !disabled {
        " m3-fab--pressed"
    } else {
        ""
    };
    let class = format!(
        "m3-fab m3-fab--{} m3-fab--{}{extended_class}{lowered_class}{branded_class}{toolbar_class}{pressed_class} {class}",
        size.class(),
        color.class(),
    );

    rsx! {
        button {
            r#type: "button",
            class,
            disabled,
            aria_label,
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
            Icon { icon, class: "m3-fab__icon" }
            if let Some(text) = label {
                span { class: "m3-fab__label", "{text}" }
            }
        }
    }
}
