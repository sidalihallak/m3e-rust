use dioxus::prelude::*;

/// A notification badge: a 6dp error dot when `count` is omitted, or a 16dp
/// high pill with a label-small count when it is given. Counts above `max` show
/// as `{max}+`. Put it inside a [`BadgeAnchor`] to place it over an icon.
///
/// The dot is decorative and labelled "New notification". A count is a live
/// status, so screen readers announce changes to it.
#[component]
pub fn NotificationBadge(
    #[props(default)] count: Option<u32>,
    #[props(default = 999)] max: u32,
    #[props(default)] class: String,
) -> Element {
    let large = count.is_some();
    let text = count.map(|value| if value > max { format!("{max}+") } else { value.to_string() });
    let size_class = if large { "m3-badge--large" } else { "m3-badge--dot" };
    let class = format!("m3-badge {size_class} {class}");
    let label = match &text {
        Some(text) => format!("{text} notifications"),
        None => "New notification".to_string(),
    };

    rsx! {
        if let Some(text) = text {
            span { class, role: "status", "aria-label": label,
                bdi { dir: "ltr", "{text}" }
            }
        } else {
            span { class, role: "img", "aria-label": label }
        }
    }
}

/// Positions its children as the anchor for a [`NotificationBadge`]. The badge
/// sits at the upper trailing edge, using the official icon-corner offsets.
/// Count pills grow toward that edge without moving their leading anchor; RTL mirrors it.
#[component]
pub fn BadgeAnchor(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        span { class: "m3-badge-anchor {class}", {children} }
    }
}
