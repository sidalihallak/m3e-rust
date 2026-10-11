use super::{NavigationLayout, motion::next_id, navigation::use_navigation_keyboard};
use dioxus::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolbarVariant {
    #[default]
    Standard,
    Vibrant,
}
/// A named Material toolbar. Native Tab is retained; arrow keys/Home/End move
/// among enabled child actions. The consumer supplies named 48px controls.
#[component]
pub fn Toolbar(
    aria_label: String,
    #[props(default)] floating: bool,
    #[props(default)] variant: ToolbarVariant,
    #[props(default = NavigationLayout::Horizontal)] orientation: NavigationLayout,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-toolbar"));
    use_navigation_keyboard(id.clone());
    let vertical = orientation == NavigationLayout::Vertical;
    rsx! {div {id,class:"m3-toolbar {class}",role:"toolbar",aria_label,
        aria_orientation:if vertical {"vertical"}else{"horizontal"},"data-nav-axis":if vertical {"vertical"}else{"horizontal"},
        "data-floating":floating,"data-vibrant":variant==ToolbarVariant::Vibrant,"data-vertical":vertical,{children}
    }}
}
