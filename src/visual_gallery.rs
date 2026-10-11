use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const BAR_CHART_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Chart, ChartSeries, ChartPoint};
#[component]
fn ActivityChart() -> Element {
    rsx! { Chart { label: "Weekly activity",
        series: vec![ChartSeries::new("desktop", "Desktop", "var(--md-sys-color-primary)"), ChartSeries::new("mobile", "Mobile", "var(--md-sys-color-tertiary)")],
        data: vec![ChartPoint::new("Mon", vec![180.0, 95.0]), ChartPoint::new("Tue", vec![240.0, 120.0]), ChartPoint::new("Wed", vec![195.0, 170.0])],
    } }
}"#;
const LINE_CHART_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Chart, ChartKind, ChartSeries, ChartPoint};
#[component]
fn TrendChart() -> Element {
    let mut target = ChartSeries::new("target", "Target", "var(--md-sys-color-secondary)");
    target.dashed = true;
    rsx! { Chart { label: "Daily trend", kind: ChartKind::Line,
        series: vec![ChartSeries::new("actual", "Actual", "var(--md-sys-color-primary)"), target],
        data: vec![ChartPoint::new("Mon", vec![12.0, 10.0]), ChartPoint::new("Tue", vec![18.0, 15.0]), ChartPoint::new("Wed", vec![16.0, 20.0])],
    } }
}"#;
const AREA_CHART_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Chart, ChartKind, ChartSeries, ChartPoint};
#[component]
fn GrowthChart() -> Element {
    rsx! { Chart { label: "Project growth", kind: ChartKind::Area,
        series: vec![ChartSeries::new("projects", "Projects", "var(--md-sys-color-primary)")],
        data: vec![ChartPoint::new("Jan", vec![8.0]), ChartPoint::new("Feb", vec![12.0]), ChartPoint::new("Mar", vec![20.0])],
    } }
}"#;
const DONUT_CHART_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Chart, ChartKind, ChartSeries, ChartPoint};
#[component]
fn ChannelChart() -> Element {
    rsx! { Chart { label: "Channels", kind: ChartKind::Donut,
        series: vec![ChartSeries::new("web", "Web", "var(--md-sys-color-primary)"), ChartSeries::new("app", "App", "var(--md-sys-color-tertiary)")],
        data: vec![ChartPoint::new("Visits", vec![620.0, 380.0])],
    } }
}"#;
const CAROUSEL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Carousel, CarouselEntry, CarouselSlide};
#[component]
fn FeaturedProjects() -> Element {
    let mut selected = use_signal(String::new);
    rsx! { Carousel { label: "Featured projects", onactivate: move |key| selected.set(key),
        items: ["Atlas", "Beacon", "Canvas"].into_iter().map(|name| CarouselEntry::new(name, name,
            rsx! { CarouselSlide { h3 { "{name}" } } }
        )).collect::<Vec<_>>(),
    } p { "Selected: {selected}" } }
}"#;
const EXPRESSIVE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ExpressiveCarousel, ExpressiveCarouselVariant, CarouselEntry, CarouselSlide, Dialog, DialogVariant};
#[component]
fn Collections() -> Element {
    let mut all = use_signal(|| false);
    let names = ["Atlas", "Beacon", "Canvas", "Drift", "Echo"];
    rsx! {
        ExpressiveCarousel { label: "Collections", variant: ExpressiveCarouselVariant::MultiBrowse,
            onshowall: move |_| all.set(true),
            items: names.into_iter().map(|name| CarouselEntry::new(name, name,
                rsx! { CarouselSlide { h3 { class: "m3-carousel__title", "{name}" } } }
            )).collect::<Vec<_>>(),
        }
        Dialog { title: "All collections", open: all(), variant: DialogVariant::FullScreen,
            show_close: true, onopenchange: move |value| all.set(value),
            for name in names { article { h3 { "{name}" } p { "Collection details." } } }
        }
    }
}"#;
const HERO_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ExpressiveCarousel, ExpressiveCarouselVariant, CarouselEntry, CarouselSlide};
#[component]
fn HeroCollection() -> Element {
    let mut all = use_signal(|| false);
    rsx! { ExpressiveCarousel { label: "Highlights", variant: ExpressiveCarouselVariant::Hero,
        onshowall: move |_| all.set(true),
        items: ["Atlas", "Beacon", "Canvas"].into_iter().map(|name| CarouselEntry::new(name, name, rsx! { CarouselSlide { h3 { class: "m3-carousel__title", "{name}" } } })).collect::<Vec<_>>(),
    } if all() { for name in ["Atlas", "Beacon", "Canvas"] { article { h3 { "{name}" } } } } }
}"#;
const UNCONTAINED_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ExpressiveCarousel, ExpressiveCarouselVariant, CarouselEntry, CarouselSlide};
#[component]
fn ReadingCollection() -> Element {
    let mut all = use_signal(|| false);
    rsx! { ExpressiveCarousel { label: "Reading list", variant: ExpressiveCarouselVariant::Uncontained,
        onshowall: move |_| all.set(true), item_width: 240.0,
        items: ["Atlas", "Beacon", "Canvas"].into_iter().map(|name| CarouselEntry::new(name, name, rsx! { CarouselSlide { h3 { "{name}" } p { "Supporting description." } } })).collect::<Vec<_>>(),
    } if all() { for name in ["Atlas", "Beacon", "Canvas"] { article { h3 { "{name}" } } } } }
}"#;
const FULLSCREEN_CAROUSEL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ExpressiveCarousel, ExpressiveCarouselVariant, CarouselEntry, CarouselSlide};
#[component]
fn ImmersiveCollection() -> Element {
    rsx! { ExpressiveCarousel { label: "Immersive collection", variant: ExpressiveCarouselVariant::FullScreen,
        height: 480, onshowall: move |_| {},
        items: ["Atlas", "Beacon", "Canvas"].into_iter().map(|name| CarouselEntry::new(name, name, rsx! { CarouselSlide { h3 { "{name}" } } })).collect::<Vec<_>>(),
    } }
}"#;
fn series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("desktop", "Desktop", "var(--md-sys-color-primary)"),
        ChartSeries::new("mobile", "Mobile", "var(--md-sys-color-tertiary)"),
    ]
}
fn weekly() -> Vec<ChartPoint> {
    [
        ("Mon", 180.0, 95.0),
        ("Tue", 240.0, 120.0),
        ("Wed", 195.0, 170.0),
        ("Thu", 280.0, 155.0),
        ("Fri", 220.0, 185.0),
    ]
    .into_iter()
    .map(|(label, a, b)| ChartPoint::new(label, vec![a, b]))
    .collect()
}
fn entries() -> Vec<CarouselEntry> {
    ["Atlas","Beacon","Canvas","Drift","Echo","Field notes","Garden","Harbor"].into_iter().enumerate().map(|(i,name)|CarouselEntry::new(name,name,rsx!{CarouselSlide{class:"visual-slide visual-slide--{i%3}",span{class:"visual-slide__art",aria_hidden:"true",span{}span{}span{}}h3{class:"m3-carousel__title","{name}"}p{class:"m3-carousel__supporting","A collection of ideas"}}})).collect()
}
#[component]
pub fn VisualGallery(active: String) -> Element {
    let mut selected = use_signal(String::new);
    let mut all = use_signal(|| false);
    let mut all_title = use_signal(|| "All collections".to_string());
    let mut chart_selection = use_signal(String::new);
    let mut disabled_entry = CarouselEntry::new(
        "restricted",
        "Restricted",
        rsx! {CarouselSlide{h3{"Restricted"}}},
    );
    disabled_entry.disabled = true;
    let mut missing = ChartPoint::new("Tue", vec![0.0]);
    missing.values[0] = None;
    rsx! {
        section{class:"roles-section wrap visual-gallery",id:"charts",hidden:active!="charts",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / DATA VISUALIZATION"}h2{"Explore the story in your data"}}p{class:"section-description","Responsive SVG charts with labeled values, keyboard inspection and readable data tables."}}
            div{class:"visual-gallery__charts",
                article{class:"visual-gallery__card",h3{"Weekly activity"}Chart{label:"Weekly activity",series:series(),data:weekly(),onselect:move |i|chart_selection.set(format!("Point {}",i+1))}}
                article{class:"visual-gallery__card",h3{"Daily trend"}Chart{label:"Daily trend",kind:ChartKind::Line,series:{let mut s=series();s[1].dashed=true;s},data:weekly()}}
                article{class:"visual-gallery__card",h3{"Project growth"}Chart{label:"Project growth",kind:ChartKind::Area,series:vec![ChartSeries::new("projects","Projects","var(--md-sys-color-primary)")],data:vec![ChartPoint::new("Jan",vec![8.0]),ChartPoint::new("Feb",vec![12.0]),ChartPoint::new("Mar",vec![20.0]),ChartPoint::new("Apr",vec![28.0])]}}
                article{class:"visual-gallery__card",h3{"Channels"}Chart{label:"Channels",kind:ChartKind::Donut,series:series(),data:vec![ChartPoint::new("Visits",vec![620.0,380.0])]}}
            }
            p{role:"status","{chart_selection}"}
            details{class:"visual-gallery__edge-cases",summary{"Empty, missing and negative data"}div{class:"visual-gallery__charts",
                article{class:"visual-gallery__card",h3{"No data"}Chart{label:"Empty chart",series:series(),data:vec![]}}
                article{class:"visual-gallery__card",h3{"Signed values"}Chart{label:"Signed values",series:vec![ChartSeries::new("balance","Balance","var(--md-sys-color-primary)")],data:vec![ChartPoint::new("Mon",vec![-8.0]),ChartPoint::new("Tue",vec![0.0]),ChartPoint::new("Wed",vec![12.0])]}}
                article{class:"visual-gallery__card",h3{"Missing values"}Chart{label:"Missing values",kind:ChartKind::Line,series:vec![ChartSeries::new("value","Value","var(--md-sys-color-primary)")],data:vec![ChartPoint::new("Mon",vec![8.0]),missing,ChartPoint::new("Wed",vec![12.0])]}}
                article{class:"visual-gallery__card",h3{"Invalid donut"}Chart{label:"Invalid donut",kind:ChartKind::Donut,series:series(),data:vec![ChartPoint::new("Values",vec![8.0,-1.0])],disabled:true}}
            }}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Bar chart usage",code:BAR_CHART_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Line chart usage",code:LINE_CHART_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Area chart usage",code:AREA_CHART_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Donut chart usage",code:DONUT_CHART_USAGE}
        }
        section{class:"roles-section wrap visual-gallery",id:"carousels",hidden:active!="carousels",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / CAROUSEL"}h2{"Browse one item at a time"}}p{class:"section-description","Standard snap carousel with keyboard navigation and controls below the content."}}
            article{class:"visual-gallery__card",Carousel{label:"Featured projects",items:entries(),height:260,onactivate:move|key|selected.set(key)}p{role:"status","Selected: {selected}"}}
            article{class:"visual-gallery__card",h3{"Vertical"}Carousel{label:"Vertical projects",items:entries(),orientation:CarouselOrientation::Vertical,height:260,onactivate:move|key|selected.set(key)}}
            article{class:"visual-gallery__card",h3{"Disabled item"}Carousel{label:"Disabled example",items:vec![disabled_entry],height:120}}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Carousel usage",code:CAROUSEL_USAGE}
        }
        section{class:"roles-section wrap visual-gallery",id:"expressive-carousels",hidden:active!="expressive-carousels",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / EXPRESSIVE CAROUSEL"}h2{"A preview of what comes next"}}p{class:"section-description","Items adapt around large, medium and small keylines. Reduced motion uses fixed-size content without parallax."}}
            article{class:"visual-gallery__card",h3{"Multi-browse"}ExpressiveCarousel{label:"Collections",items:entries(),onactivate:move|key|selected.set(key),onshowall:move |_|{all_title.set("All collections".into());all.set(true);}}}
            article{class:"visual-gallery__card",h3{"Hero"}ExpressiveCarousel{label:"Highlights",variant:ExpressiveCarouselVariant::Hero,items:entries(),height:260,onactivate:move|key|selected.set(key),onshowall:move |_|{all_title.set("All highlights".into());all.set(true);}}}
            article{class:"visual-gallery__card",h3{"Uncontained"}ExpressiveCarousel{label:"Reading list",variant:ExpressiveCarouselVariant::Uncontained,items:entries(),item_width:240.0,onactivate:move|key|selected.set(key),onshowall:move |_|{all_title.set("All reading items".into());all.set(true);}}}
            article{class:"visual-gallery__card visual-gallery__portrait",h3{"Full-screen layout"}p{"Portrait, vertical snap layout shown in a bounded preview."}ExpressiveCarousel{label:"Immersive collection",variant:ExpressiveCarouselVariant::FullScreen,items:entries(),height:480,show_controls:true,onactivate:move|key|selected.set(key),onshowall:move |_|{all_title.set("All immersive items".into());all.set(true);}}}
            p{role:"status","Selected: {selected}"}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Expressive carousel usage",code:EXPRESSIVE_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Hero carousel usage",code:HERO_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Uncontained carousel usage",code:UNCONTAINED_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Full-screen carousel usage",code:FULLSCREEN_CAROUSEL_USAGE}
        }
        Dialog{title:all_title(),open:all(),variant:DialogVariant::FullScreen,show_close:true,onopenchange:move|v|all.set(v),class:"visual-gallery__all",div{class:"visual-gallery__vertical-list",for item in entries(){article{h3{"{item.label}"}p{"A collection of ideas"}Button{variant:ButtonVariant::Tonal,onclick:move |_|{selected.set(item.key.clone());all.set(false);},"Open {item.label}"}}}}}
    }
}
