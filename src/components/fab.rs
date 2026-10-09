use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use std::rc::Rc;

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

/// Starts a ripple at the pointer (or the centre for keyboard input). Mirrors
/// the Button pilot: the ripple is measured against the control, not the icon.
fn start_ripple(
    mut press: Signal<(u64, bool)>,
    mut ripple: Signal<(u64, f64, f64, f64, f64)>,
    mounted: Option<Rc<MountedData>>,
    x: f64,
    y: f64,
) {
    let next_id = press().0.wrapping_add(1);
    press.set((next_id, true));
    spawn(async move {
        if let Some(mounted) = mounted
            && let Ok(rect) = mounted.get_client_rect().await
            && press.peek().0 == next_id
        {
            let (width, height) = (rect.size.width, rect.size.height);
            let (x, y) = if x.is_finite() && y.is_finite() {
                (x - rect.origin.x, y - rect.origin.y)
            } else {
                (width / 2.0, height / 2.0)
            };
            ripple.set((next_id, x, y, width, height));
        }
    });
}

/// A floating action button built on a native `<button>`.
///
/// Pass `label` to make it an extended FAB. Icon-only buttons need `aria_label`,
/// because the icon is hidden from assistive technology. Press feedback is a
/// ripple from the pointer, as in the Button pilot, plus the pressed elevation.
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
    // (press id, pointer or Space held)
    let press = use_signal(|| (0_u64, false));
    // (ripple id, x, y, width, height) relative to the control
    let ripple = use_signal(|| (0_u64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64));
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut suppress_click_ripple = use_signal(|| false);
    let (_, held) = press();
    let extended_class = if label.is_some() { " m3-fab--extended" } else { "" };
    let lowered_class = if lowered { " m3-fab--lowered" } else { "" };
    let branded_class = if branded { " m3-fab--branded" } else { "" };
    let toolbar_class = if toolbar { " m3-fab--toolbar" } else { "" };
    let pressed_class = if held && !disabled {
        " m3-fab--pressed"
    } else {
        ""
    };
    let class = format!(
        "m3-fab m3-fab--{} m3-fab--{}{extended_class}{lowered_class}{branded_class}{toolbar_class}{pressed_class} {class}",
        size.class(),
        color.class(),
    );

    let (ripple_id, ripple_x, ripple_y, ripple_width, ripple_height) = ripple();
    let max_dimension = ripple_width.max(ripple_height);
    let initial_size = (max_dimension * 0.2).floor().max(1.0);
    let soft_edge = (max_dimension * 0.35).max(75.0);
    let ripple_scale = (ripple_width.hypot(ripple_height) + 10.0 + soft_edge) / initial_size;
    let ripple_style = format!(
        "--ripple-size: {initial_size}px; --ripple-from-x: {}px; --ripple-from-y: {}px; --ripple-to-x: {}px; --ripple-to-y: {}px; --ripple-scale: {ripple_scale};",
        ripple_x - initial_size / 2.0,
        ripple_y - initial_size / 2.0,
        (ripple_width - initial_size) / 2.0,
        (ripple_height - initial_size) / 2.0,
    );

    rsx! {
        button {
            r#type: "button",
            class,
            disabled,
            aria_label,
            onmounted: move |event| mounted.set(Some(event.data())),
            onpointerdown: move |event| {
                if !disabled && event.is_primary() && event.trigger_button() == Some(MouseButton::Primary) {
                    let point = event.client_coordinates();
                    start_ripple(press, ripple, mounted(), point.x, point.y);
                    suppress_click_ripple.set(true);
                }
            },
            onpointerup: move |_| {
                let (id, _) = press();
                press.clone().set((id, false));
            },
            onpointercancel: move |_| {
                let (id, _) = press();
                press.clone().set((id, false));
                suppress_click_ripple.set(false);
            },
            onpointerleave: move |_| {
                let (id, _) = press();
                press.clone().set((id, false));
                suppress_click_ripple.set(false);
            },
            onkeydown: move |event| {
                if !disabled && event.code() == Code::Space && !event.is_auto_repeating() {
                    start_ripple(press, ripple, mounted(), f64::NAN, f64::NAN);
                    suppress_click_ripple.set(true);
                }
            },
            onkeyup: move |event| {
                if event.code() == Code::Space {
                    let (id, _) = press();
                    press.clone().set((id, false));
                }
            },
            onblur: move |_| {
                let (id, _) = press();
                press.clone().set((id, false));
                suppress_click_ripple.set(false);
            },
            onclick: move |event| {
                if suppress_click_ripple() {
                    suppress_click_ripple.set(false);
                } else {
                    // Keyboard activation (Enter) produces a click without pointer events.
                    start_ripple(press, ripple, mounted(), f64::NAN, f64::NAN);
                    let (id, _) = press();
                    press.clone().set((id, false));
                }
                onclick.call(event);
            },
            if ripple_id > 0 {
                span {
                    key: "{ripple_id}",
                    class: "m3-fab__ripple",
                    style: ripple_style,
                    aria_hidden: "true"
                }
            }
            Icon { icon, class: "m3-fab__icon" }
            if let Some(text) = label {
                span { class: "m3-fab__label", "{text}" }
            }
        }
    }
}
