use dioxus::prelude::*;

/// Card style. `Filled` sits on surface-container-highest, `Elevated` on
/// surface-container-low with a shadow, and `Outlined` on surface with a 1dp outline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Filled,
    Elevated,
    Outlined,
}

impl CardVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Elevated => "elevated",
            Self::Outlined => "outlined",
        }
    }
}

/// A 12dp card with optional content, or an interactive card built on a native
/// `<button>` with a state layer and a hover, press and focus response.
///
/// Set `interactive` and `onclick` to make the whole card an action. Interactive
/// cards use phrasing content only (text, icons, spans), because a button cannot
/// contain block content. Non-interactive cards accept any content.
#[component]
pub fn Card(
    #[props(default)] variant: CardVariant,
    #[props(default)] interactive: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let variant_class = variant.class();
    let interactive_class = if interactive { " m3-card--interactive" } else { "" };
    let disabled_class = if disabled { " m3-card--disabled" } else { "" };
    let class = format!("m3-card m3-card--{variant_class}{interactive_class}{disabled_class} {class}");

    if interactive {
        rsx! {
            button { r#type: "button", class, disabled, onclick: move |event| onclick.call(event),
                span { class: "m3-card__state", aria_hidden: "true" }
                span { class: "m3-card__content", {children} }
            }
        }
    } else {
        rsx! {
            div { class,
                {children}
            }
        }
    }
}

/// A full-width image at the top of a card, with 12dp top corners and a 16:9 frame.
/// Put it first in the card; the card then drops its padding so the image is edge to edge.
#[component]
pub fn CardMedia(src: String, alt: String, #[props(default)] class: String) -> Element {
    rsx! {
        img { class: "m3-card__media {class}", src, alt }
    }
}

/// The padded text area of a card, below any [`CardMedia`]. It is a span so it is
/// valid inside an interactive card's button.
#[component]
pub fn CardBody(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        span { class: "m3-card__body {class}", {children} }
    }
}
