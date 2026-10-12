use dioxus::prelude::*;
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MarkerVariant {
    #[default]
    Default,
    Separator,
    Border,
}
impl MarkerVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Separator => "separator",
            Self::Border => "border",
        }
    }
}
/// Context/metadata marker. Optional href supplies a real link; no fake button semantics.
#[component]
pub fn Marker(
    #[props(default)] variant: MarkerVariant,
    #[props(default)] href: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let class = format!("m3-marker m3-marker--{} {class}", variant.class());
    rsx! {if let Some(href)=href {a{class,href,{children}}}else{div{class,{children}}}}
}
#[component]
pub fn MarkerIcon(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-marker__icon {class}",aria_hidden:"true",{children}}}
}
#[component]
pub fn MarkerContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-marker__content {class}",{children}}}
}
