use super::{Icon, motion::next_id};
use crate::icons;
use dioxus::prelude::*;

/// Presentational, stable-key slide content; do not nest another interactive control inside
/// the owned native item button. Pointer focus preserves click geometry; keys
/// must be unique and stable. Text can adapt to the measured mask inset.
/// Drag release preserves focus on the snapped item.
#[derive(Clone, PartialEq)]
pub struct CarouselEntry {
    pub key: String,
    pub label: String,
    pub content: Element,
    pub disabled: bool,
}
impl CarouselEntry {
    pub fn new(key: impl Into<String>, label: impl Into<String>, content: Element) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            content,
            disabled: false,
        }
    }
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ExpressiveCarouselVariant {
    #[default]
    MultiBrowse,
    Hero,
    Uncontained,
    FullScreen,
}
impl ExpressiveCarouselVariant {
    fn attr(self) -> &'static str {
        match self {
            Self::MultiBrowse => "multi-browse",
            Self::Hero => "hero",
            Self::Uncontained => "uncontained",
            Self::FullScreen => "full-screen",
        }
    }
}

#[component]
pub fn Carousel(
    label: String,
    items: Vec<CarouselEntry>,
    #[props(default)] orientation: CarouselOrientation,
    #[props(default = 1)] per_view: usize,
    #[props(default = 221)] height: usize,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onactivate: EventHandler<String>,
) -> Element {
    rsx! {CarouselView{label,items,kind:"standard",orientation,per_view,height,item_width:186.0,gap:16,disabled,class,show_controls:true,onactivate}}
}
/// Material adaptive carousel. `onshowall` must expose a vertically scrollable
/// view of all items. FullScreen is a vertical snap layout filling this region;
/// the consuming app supplies its portrait viewport height.
#[component]
pub fn ExpressiveCarousel(
    label: String,
    items: Vec<CarouselEntry>,
    onshowall: EventHandler<()>,
    #[props(default)] variant: ExpressiveCarouselVariant,
    #[props(default = 186.0)] item_width: f64,
    #[props(default = 221)] height: usize,
    #[props(default)] gap: Option<usize>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] show_controls: bool,
    #[props(default)] onactivate: EventHandler<String>,
) -> Element {
    let fullscreen = variant == ExpressiveCarouselVariant::FullScreen;
    rsx! {CarouselView{label,items,kind:variant.attr(),orientation:if fullscreen{CarouselOrientation::Vertical}else{CarouselOrientation::Horizontal},per_view:1,height,item_width,gap:gap.unwrap_or(if fullscreen{16}else{8}),disabled,class,show_controls,onactivate,onshowall:if fullscreen{None}else{Some(onshowall)}}}
}
#[component]
pub fn CarouselSlide(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-carousel-slide {class}",{children}}}
}

#[component]
fn CarouselView(
    label: String,
    items: Vec<CarouselEntry>,
    kind: String,
    orientation: CarouselOrientation,
    per_view: usize,
    height: usize,
    item_width: f64,
    gap: usize,
    disabled: bool,
    class: String,
    show_controls: bool,
    onactivate: EventHandler<String>,
    #[props(default)] onshowall: Option<EventHandler<()>>,
) -> Element {
    let id = use_hook(|| next_id("m3-carousel"));
    let mut position = use_signal(|| (0usize, 0usize));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("carousel.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(value) = eval.recv::<(usize, usize)>().await {
                    position.set(value);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Carousels?.[{id:?}]?.();"));
        }
    });
    let count = items.len();
    let height = height.clamp(64, 2000);
    let width = if item_width.is_finite() {
        item_width.clamp(56.0, 1600.0)
    } else {
        186.0
    };
    let gap = gap.min(64);
    rsx! {section{id,class:"m3-carousel {class}",role:"region",aria_roledescription:"carousel",aria_label:label.clone(),"data-variant":kind,"data-orientation":if orientation==CarouselOrientation::Vertical{"vertical"}else{"horizontal"},"data-per-view":per_view.clamp(1,10),"data-item-width":"{width}","data-disabled":disabled,"data-empty":count==0,style:"--m3-carousel-height:{height}px;--m3-carousel-gap:{gap}px;",
        if items.is_empty(){p{role:"status","No items."}}
        div{class:"m3-carousel__track",div{class:"m3-carousel__range",
            div{class:"m3-carousel__snap-points",aria_hidden:"true"}
            div{class:"m3-carousel__layer",for (index,item) in items.into_iter().enumerate(){
                button{key:"{item.key}",r#type:"button",class:"m3-carousel__item",aria_label:"{item.label}, {index+1} of {count}",disabled:disabled||item.disabled,"data-index":index,"data-item-key":item.key.clone(),onclick:move |_|onactivate.call(item.key.clone()),
                    super::ripple::Ripple {},div{class:"m3-carousel__content",{item.content}}
                }
            }}
        }}
        if show_controls||onshowall.is_some(){div{class:"m3-carousel__footer",
            if show_controls{div{class:"m3-carousel__controls",aria_label:"{label} controls",
                button{r#type:"button",class:"m3-carousel__nav",aria_label:"Previous {label} item","data-nav":"previous",disabled:disabled||count==0||position().0==0,super::ripple::Ripple {},Icon{icon:icons::CHEVRON_LEFT}}
                span{class:"m3-carousel__position",role:"status",aria_live:"polite",if count>0{"View {position().0+1} of {position().1+1}"}else{"0 of 0"}}
                button{r#type:"button",class:"m3-carousel__nav",aria_label:"Next {label} item","data-nav":"next",disabled:disabled||count==0||position().0>=position().1,super::ripple::Ripple {},Icon{icon:icons::CHEVRON_RIGHT}}
            }}
            if let Some(show)=onshowall{button{r#type:"button",class:"m3-carousel__show-all",aria_label:"Show all {label}",disabled:disabled||count==0,onclick:move |_|show.call(()),super::ripple::Ripple {},span{class:"m3-carousel__show-all-label","Show all"}}}
        }}
    }}
}
