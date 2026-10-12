use super::motion::next_id;
use dioxus::prelude::*;
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Scrollbars {
    #[default]
    Vertical,
    Horizontal,
    Both,
}
impl Scrollbars {
    fn class(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both => "both",
        }
    }
}
/// Named, focusable native scroll viewport with pointer-operated decorative bars.
#[component]
pub fn ScrollArea(
    label: String,
    #[props(default)] scrollbars: Scrollbars,
    #[props(default="240px".into())] height: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-scroll-area"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("scroll_area.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3ScrollAreas?.[{id:?}]?.();")).await;
            });
        }
    });
    rsx! {div{id,class:"m3-scroll-area {class}","data-scrollbars":scrollbars.class(),style:"height:{height}",
        div{class:"m3-scroll-area__viewport",role:"region",aria_label:label,tabindex:"0",div{class:"m3-scroll-area__content",{children}}}
        if scrollbars!=Scrollbars::Horizontal {ScrollBar{}}
        if scrollbars!=Scrollbars::Vertical {ScrollBar{horizontal:true}}
        if scrollbars==Scrollbars::Both {div{class:"m3-scroll-area__corner",aria_hidden:"true"}}
    }}
}
/// Insert inside ScrollArea; the parent runtime associates its native viewport.
#[component]
pub fn ScrollBar(#[props(default)] horizontal: bool) -> Element {
    rsx! {div{class:"m3-scroll-area__bar",aria_hidden:"true","data-axis":if horizontal{"x"}else{"y"},div{class:"m3-scroll-area__thumb"}}}
}
