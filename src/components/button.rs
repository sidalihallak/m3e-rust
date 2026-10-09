use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    Elevated,
    #[default]
    Filled,
    Tonal,
    Outlined,
    Text,
}

impl ButtonVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Elevated => "elevated",
            Self::Filled => "filled",
            Self::Tonal => "tonal",
            Self::Outlined => "outlined",
            Self::Text => "text",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    ExtraSmall,
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl ButtonSize {
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonShape {
    #[default]
    Round,
    Square,
}

impl ButtonShape {
    const fn class(self) -> &'static str {
        match self {
            Self::Round => "round",
            Self::Square => "square",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

fn start_press_feedback(
    mut press: Signal<(u64, bool, bool)>,
    mut ripple: Signal<(u64, f64, f64, f64, f64)>,
    mounted: Option<Rc<MountedData>>,
    x: f64,
    y: f64,
) {
    let next_press_id = press().0.wrapping_add(1);
    press.set((next_press_id, true, false));
    // Measure the control, rather than the label/icon event target, so the
    // ripple starts at the pointer and travels to the button's actual centre.
    spawn(async move {
        if let Some(mounted) = mounted
            && let Ok(rect) = mounted.get_client_rect().await
            && press.peek().0 == next_press_id
        {
            let (width, height) = (rect.size.width, rect.size.height);
            let (x, y) = if x.is_finite() && y.is_finite() {
                (x - rect.origin.x, y - rect.origin.y)
            } else {
                (width / 2.0, height / 2.0)
            };
            ripple.set((next_press_id, x, y, width, height));
        }
    });

    let mut press_for_timer = press;
    spawn(async move {
        let _ = document::eval("await new Promise(resolve => setTimeout(resolve, 225));").await;
        let (current_id, pointer_down, _) = press_for_timer();
        if current_id == next_press_id {
            press_for_timer.set((current_id, pointer_down, true));
        }
    });
}

fn release_press_feedback(mut press: Signal<(u64, bool, bool)>) {
    let (current_id, _, min_press_elapsed) = press();
    press.set((current_id, false, min_press_elapsed));
}

impl ButtonType {
    const fn attr(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] shape: ButtonShape,
    #[props(default)] disabled: bool,
    #[props(default)] button_type: ButtonType,
    #[props(default)] toggle: bool,
    #[props(default)] selected: bool,
    #[props(default)] class: String,
    #[props(default)] leading_icon: Option<Element>,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let press = use_signal(|| (0_u64, false, true));
    let ripple = use_signal(|| (0_u64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64));
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    let mut suppress_click_feedback = use_signal(|| false);
    let toggle_class = if toggle { " m3-button--toggle" } else { "" };
    let selected_class = if toggle && selected {
        " m3-button--selected"
    } else {
        ""
    };
    let (_, pointer_down, min_press_elapsed) = press();
    let press_class = if pointer_down || !min_press_elapsed {
        " m3-button--pressed"
    } else {
        ""
    };
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
    let class = format!(
        "m3-button m3-button--{} m3-button--{} m3-button--{}{}{}{} {}",
        variant.class(),
        size.class(),
        shape.class(),
        toggle_class,
        selected_class,
        press_class,
        class
    );

    rsx! {
        button {
            r#type: button_type.attr(),
            class,
            aria_pressed: if toggle { Some(selected) } else { None },
            disabled,
            onmounted: move |event| mounted.set(Some(event.data())),
            onpointerdown: move |event| {
                if !disabled && event.is_primary() && event.trigger_button() == Some(MouseButton::Primary) {
                    let point = event.client_coordinates();
                    let (x, y) = (point.x, point.y);
                    start_press_feedback(press, ripple, mounted(), x, y);
                    suppress_click_feedback.set(true);
                }
            },
            onpointerup: move |_| {
                release_press_feedback(press);
            },
            onpointercancel: move |_| {
                release_press_feedback(press);
                suppress_click_feedback.set(false);
            },
            onpointerleave: move |_| {
                release_press_feedback(press);
                suppress_click_feedback.set(false);
            },
            onkeydown: move |event| {
                if !disabled && event.code() == Code::Space && !event.is_auto_repeating() {
                    start_press_feedback(press, ripple, mounted(), f64::NAN, f64::NAN);
                    suppress_click_feedback.set(true);
                }
            },
            onkeyup: move |event| {
                if event.code() == Code::Space {
                    release_press_feedback(press);
                }
            },
            onblur: move |_| {
                release_press_feedback(press);
                suppress_click_feedback.set(false);
            },
            onclick: move |event| {
                if suppress_click_feedback() {
                    suppress_click_feedback.set(false);
                } else {
                    // Keyboard activation produces a click without pointer events.
                    start_press_feedback(press, ripple, mounted(), f64::NAN, f64::NAN);
                    release_press_feedback(press);
                }
                onclick.call(event);
            },
            if ripple_id > 0 {
                span {
                    key: "{ripple_id}",
                    class: "m3-button__ripple",
                    style: ripple_style,
                    aria_hidden: "true"
                }
            }
            span { class: "m3-button__content",
                if let Some(icon) = leading_icon {
                    span { class: "m3-button__icon", aria_hidden: "true", {icon} }
                }
                span { class: "m3-button__label", {children} }
            }
        }
    }
}
