use super::motion::next_id;
use dioxus::prelude::*;
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PanelDirection {
    #[default]
    Horizontal,
    Vertical,
}
/// Percentage-based adjacent panels. Defaults initialize once; the runtime owns resize state.
#[component]
pub fn ResizablePanelGroup(
    #[props(default)] direction: PanelDirection,
    #[props(default)] disabled: bool,
    #[props(default)] adaptive: bool,
    #[props(default)] class: String,
    #[props(default)] onlayoutchange: EventHandler<Vec<f64>>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-resizable"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("resizable.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(layout) = eval.recv::<Vec<f64>>().await {
                    onlayoutchange.call(layout);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Resizable?.[{id:?}]?.();"));
        }
    });
    rsx! {div{id,class:"m3-resizable {class}","data-direction":if direction==PanelDirection::Horizontal{"horizontal"}else{"vertical"},"data-disabled":disabled,"data-adaptive":adaptive,{children}}}
}
#[component]
pub fn ResizablePanel(
    label: String,
    #[props(default = 50.0)] default_size: f64,
    #[props(default = 10.0)] min_size: f64,
    #[props(default = 100.0)] max_size: f64,
    #[props(default)] collapsible: bool,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-panel"));
    let finite = |v: f64, fallback: f64| {
        if v.is_finite() {
            v.clamp(0.0, 100.0)
        } else {
            fallback
        }
    };
    let min = finite(min_size, 10.0);
    let max = finite(max_size, 100.0).max(min);
    let size = finite(default_size, 50.0);
    rsx! {div{id,class:"m3-resizable__panel",role:"region",aria_label:label,"data-default":size,"data-min":min,"data-max":max,"data-collapsible":collapsible,"data-disabled":disabled,style:"flex-grow:{size}",div{class:"m3-resizable__content {class}",{children}}}}
}
#[component]
pub fn ResizableHandle(
    label: String,
    #[props(default = true)] with_handle: bool,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {div{class:"m3-resizable__handle",role:"separator",aria_label:label,tabindex:if disabled{"-1"}else{"0"},"data-disabled":disabled,aria_disabled:disabled,if with_handle{span{class:"m3-resizable__grip",aria_hidden:"true"}}}}
}
