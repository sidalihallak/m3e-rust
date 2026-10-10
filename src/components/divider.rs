use dioxus::prelude::*;

/// Divider direction. A horizontal divider spans its container's width; a
/// vertical one stretches through a flex/grid row's cross axis. Its parent
/// determines the line height; a plain block parent does not provide stretching.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    const fn class(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Inset of a horizontal divider. `Start` and `Middle` leave 16dp on the start
/// side, and on both sides for `Middle`, as list dividers do.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerInset {
    #[default]
    None,
    Start,
    Middle,
}

impl DividerInset {
    const fn class(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Start => " m3-divider--inset-start",
            Self::Middle => " m3-divider--inset-middle",
        }
    }
}

/// A 1dp outline-variant line that separates content. It is a `role="separator"`
/// with the matching `aria-orientation`.
#[component]
pub fn Divider(
    #[props(default)] orientation: DividerOrientation,
    #[props(default)] inset: DividerInset,
    #[props(default)] class: String,
) -> Element {
    let class = format!(
        "m3-divider m3-divider--{}{} {class}",
        orientation.class(),
        inset.class()
    );
    let aria_orientation = orientation.class();

    rsx! {
        div { class, role: "separator", "aria-orientation": aria_orientation }
    }
}
