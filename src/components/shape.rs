use crate::shape_library::ShapeName;
use dioxus::prelude::*;
/// Noninteractive Material shape mask. No role or semantic meaning by default.
/// Explicit names morph using the shared Slow Effects token; label names an optional image role.
#[component]
pub fn Shape(
    #[props(default)] name: Option<ShapeName>,
    #[props(default = 48)] size: usize,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] children: Element,
) -> Element {
    let clip = name.map(ShapeName::clip_path).unwrap_or("none");
    let role = label.as_ref().map(|_| "img");
    rsx! {div{class:"m3-shape {class}",style:"--m3-shape-size:{size}px;clip-path:{clip}","data-shape":name.map(ShapeName::name),role,aria_label:label,{children}}}
}
