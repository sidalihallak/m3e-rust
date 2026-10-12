use dioxus::prelude::*;
/// A CSS aspect-ratio container. The caller owns image alt text and cropping.
#[component]
pub fn AspectRatio(ratio: f64, #[props(default)] class: String, children: Element) -> Element {
    let ratio = if ratio.is_finite() && ratio > 0.0 {
        ratio
    } else {
        1.0
    };
    rsx! { div { class:"m3-aspect-ratio {class}", style:"--m3-aspect-ratio:{ratio}", {children} } }
}
