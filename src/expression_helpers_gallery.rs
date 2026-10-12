use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const KBD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Kbd, KbdGroup};
#[component]
fn ShortcutHint() -> Element {
    rsx! { p { "Search ", KbdGroup {
        Kbd { key_label: "Command", "⌘" }
        Kbd { "K" }
    } } }
}"#;
const SHAPE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Shape, ShapeName, Button};
#[component]
fn ShapePreview() -> Element {
    let mut name = use_signal(|| ShapeName::FourLeafClover);
    rsx! { Shape { name: name(), size: 160, label: format!("Shape preview: {}", name().name()) }
        Button { onclick: move |_| name.set(ShapeName::Heart), "Choose heart" }
        Button { onclick: move |_| name.set(ShapeName::FourLeafClover), "Choose clover" }
    }
}"#;
const MARKER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Marker, MarkerContent, MarkerIcon, MarkerVariant, Icon, icons};
#[component]
fn ActivityMarkers() -> Element {
    rsx! { Marker { variant: MarkerVariant::Separator,
        MarkerContent { "Today" }
    }
    Marker { variant: MarkerVariant::Border,
        MarkerIcon { Icon { icon: icons::INFO } }
        MarkerContent { "Changes saved to your workspace" }
    } }
}"#;
const MARKER_LINK_USAGE: &str = r##"use dioxus::prelude::*;
use m3e_rust_ui::{Marker, MarkerContent};
#[component]
fn HelpMarker() -> Element {
    rsx! { Marker { href: "#keyboard-hints", MarkerContent { "View keyboard hints" } } }
}"##;
#[component]
pub fn ExpressionHelpersGallery(active: String) -> Element {
    let mut name = use_signal(|| ShapeName::FourLeafClover);
    let mut presses = use_signal(|| 0u32);
    let image = asset!("/assets/card-image.svg");
    rsx! {
        section{id:"keyboard-hints",class:"roles-section wrap",hidden:active!="keyboard-hints",
            div{class:"section-header",div{p{class:"eyebrow","FOUNDATIONS / KEYBOARD HINTS"}h2{"A small reminder"}}p{class:"section-description","Native keyboard notation with readable modifier names. Your app supplies the actual shortcut bindings."}}
            article{class:"m3-expression-card",
                div{class:"m3-shortcut-row",span{"Search"}KbdGroup{Kbd{key_label:"Command","⌘"}Kbd{"K"}}}
                div{class:"m3-shortcut-row",span{"Save changes"}KbdGroup{Kbd{key_label:"Command","⌘"}Kbd{key_label:"Shift","⇧"}Kbd{"S"}}}
                div{class:"m3-shortcut-row",span{"Move through items"}KbdGroup{Kbd{key_label:"Arrow up","↑"}Kbd{key_label:"Arrow down","↓"}}}
                div{class:"m3-shortcut-row",span{"Windows / Linux"}KbdGroup{Kbd{"Ctrl"}span{aria_hidden:"true","+"}Kbd{"K"}}}
                Button{variant:ButtonVariant::Text,onclick:move |_|presses+=1,"Demo action ",Kbd{key_label:"Enter","↵"}}
                p{role:"status","Demo activated {presses} times"}
            }
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Keyboard hint usage",code:KBD_USAGE}
        }
        section{id:"shape-library",class:"roles-section wrap",hidden:active!="shape-library",
            div{class:"section-header",div{p{class:"eyebrow","FOUNDATIONS / MATERIAL SHAPES"}h2{"A little expression"}}p{class:"section-description","All 35 Material shapes, with matched polygon points for interaction-driven morphing and media masking."}}
            article{class:"m3-expression-card",
                div{class:"m3-shape-stage",Shape{name:name(),size:160,label:format!("Shape preview: {}",name().name()),class:"m3-shape-live"}Shape{name:ShapeName::Flower,size:160,img{src:image,alt:"Colorful landscape masked by a flower shape"}}}
                p{role:"status","Current shape: {name().name()}"}
                div{class:"m3-expression-actions",
                    Button{variant:ButtonVariant::Text,onclick:move |_|name.set(ShapeName::Heart),"Choose heart"}
                    Button{variant:ButtonVariant::Text,onclick:move |_|name.set(ShapeName::FourLeafClover),"Choose clover"}
                    Button{variant:ButtonVariant::Text,onclick:move |_|name.set(ShapeName::Sunny),"Choose sunny"}
                    Button{variant:ButtonVariant::Text,disabled:true,"Unavailable shape action"}
                }
            }
            div{class:"m3-shape-grid",for item in ShapeName::ALL{div{class:"m3-shape-tile",Shape{name:item,size:64}span{"{item.name()}"}}}}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Shape morph usage",code:SHAPE_USAGE}
        }
        section{id:"markers",class:"roles-section wrap",hidden:active!="markers",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / MARKERS"}h2{"Context along the way"}}p{class:"section-description","Small metadata, separators and section borders. Links keep native navigation and focus behavior."}}
            article{class:"m3-expression-card m3-marker-demo",
                Marker{MarkerIcon{Icon{icon:icons::INFO}}MarkerContent{"Changes saved to your workspace"}}
                Marker{variant:MarkerVariant::Separator,MarkerContent{"Today"}}
                Marker{variant:MarkerVariant::Border,MarkerIcon{Icon{icon:icons::CALENDAR_TODAY}}MarkerContent{"Monday, 12 October"}}
                Marker{variant:MarkerVariant::Separator,MarkerContent{"A longer marker wraps without covering the separator lines when space is limited"}}
                Marker{href:"#keyboard-hints",MarkerContent{"View keyboard hints"}}
            }
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Marker usage",code:MARKER_USAGE}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Marker link usage",code:MARKER_LINK_USAGE}
        }
    }
}
