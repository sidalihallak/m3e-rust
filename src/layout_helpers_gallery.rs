use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const ASPECT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::AspectRatio;
#[component]
fn MediaFrame() -> Element {
    rsx! { AspectRatio { ratio: 16.0 / 9.0,
        div { style: "position:absolute;inset:0;display:grid;place-items:center;background:var(--md-sys-color-secondary-container);color:var(--md-sys-color-on-secondary-container)", "16:9 media" }
    } }
}"#;
const SCROLL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::ScrollArea;
#[component]
fn ActivityFeed() -> Element {
    rsx! { ScrollArea { label: "Recent activity", height: "240px",
        ol { for item in 1..=30 { li { "Activity {item}" } } }
    } }
}"#;
const HORIZONTAL_SCROLL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ScrollArea, Scrollbars};
#[component]
fn HorizontalGallery() -> Element {
    rsx! { ScrollArea { label: "Gallery", height: "180px", scrollbars: Scrollbars::Horizontal,
        div { style: "display:flex;gap:16px;padding:20px",
            for item in 1..=8 { div { style: "width:160px;height:120px;flex:none", "Item {item}" } }
        }
    } }
}"#;
const RESIZE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ResizablePanelGroup, ResizablePanel, ResizableHandle};
#[component]
fn WorkspacePanes() -> Element {
    let mut layout = use_signal(|| vec![65.0, 35.0]);
    rsx! { div { style: "height:360px",
        ResizablePanelGroup { adaptive: true, onlayoutchange: move |value| layout.set(value),
            ResizablePanel { label: "Document", default_size: 65.0, min_size: 25.0, h3 { "Document" } }
            ResizableHandle { label: "Document size" }
            ResizablePanel { label: "Comments", default_size: 35.0, min_size: 20.0, h3 { "Comments" } }
        }
    }
    p { "Pane sizes: {layout():?}" } }
}"#;
const VERTICAL_RESIZE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ResizablePanelGroup, ResizablePanel, ResizableHandle, PanelDirection};
#[component]
fn EditorAndConsole() -> Element {
    rsx! { div { style: "height:400px",
        ResizablePanelGroup { direction: PanelDirection::Vertical,
            ResizablePanel { label: "Editor", default_size: 70.0, min_size: 20.0, collapsible: true, "Editor" }
            ResizableHandle { label: "Editor size" }
            ResizablePanel { label: "Console", default_size: 30.0, min_size: 20.0, "Console" }
        }
    } }
}"#;
#[component]
pub fn LayoutHelpersGallery(active: String) -> Element {
    let mut layout = use_signal(|| vec![65.0, 35.0]);
    let mut locked = use_signal(|| false);
    let image = asset!("/assets/card-image.svg");
    rsx! {
        section{id:"aspect-ratio",class:"roles-section wrap",hidden:active!="aspect-ratio",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / ASPECT RATIO"}h2{"Give media a frame"}}p{class:"section-description","A stable CSS ratio that follows the available width. Image descriptions and cropping remain yours."}}
            div{class:"m3-layout-demo m3-layout-ratios",
                figure{AspectRatio{ratio:16.0/9.0,img{src:image,alt:"Illustration of a colorful landscape"}}figcaption{"Landscape · 16:9"}}
                figure{AspectRatio{ratio:1.0,div{class:"m3-layout-placeholder",h3{"1:1"}p{"Square frame"}}}figcaption{"Square · 1:1"}}
            }
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Aspect ratio usage",code:ASPECT_USAGE}
        }
        section{id:"scroll-areas",class:"roles-section wrap",hidden:active!="scroll-areas",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / SCROLL AREA"}h2{"A place to explore"}}p{class:"section-description","Native scrolling and keyboard access, with Material color roles on the custom scrollbar."}}
            h3{"Recent activity"}
            div{class:"m3-layout-demo m3-layout-demo--surface",ScrollArea{label:"Recent activity",height:"240px",div{class:"m3-layout-content",for item in 1..=24 {p{"Project update {item} · Your workspace is up to date."}}}}}
            h3{"Horizontal gallery"}
            div{class:"m3-layout-demo m3-layout-demo--surface",ScrollArea{label:"Horizontal gallery",height:"160px",scrollbars:Scrollbars::Horizontal,div{class:"m3-layout-scroll-row",for item in 1..=8 {div{class:"m3-layout-scroll-tile","Collection {item}"}}}}}
            h3{"Right-to-left gallery"}
            div{class:"m3-layout-demo m3-layout-demo--surface",dir:"rtl",ScrollArea{label:"RTL gallery",height:"160px",scrollbars:Scrollbars::Horizontal,div{class:"m3-layout-scroll-row",for item in 1..=8{div{class:"m3-layout-scroll-tile","Collection {item}"}}}}}
            h3{"Both axes"}
            div{class:"m3-layout-demo m3-layout-demo--surface",ScrollArea{label:"Project planning board",height:"220px",scrollbars:Scrollbars::Both,div{class:"m3-layout-content",style:"width:1000px",for row in 1..=16{p{"Planning row {row} · Research → Design → Build → Review → Release"}}}}}
            h3{"Content that fits"}
            div{class:"m3-layout-demo m3-layout-demo--surface",ScrollArea{label:"Short note",height:"100px",div{class:"m3-layout-content","No scrollbar is drawn when the content fits."}}}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Scroll area usage",code:SCROLL_USAGE}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Horizontal scroll usage",code:HORIZONTAL_SCROLL_USAGE}
        }
        section{id:"resizable-panels",class:"roles-section wrap",hidden:active!="resizable-panels",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / RESIZABLE PANELS"}h2{"Make space for your work"}}p{class:"section-description","Drag the separator or use arrows, Home and End. The adaptive example stacks below840px."}}
            Button{variant:ButtonVariant::Text,onclick:move |_|locked.toggle(),if locked(){"Unlock panels"}else{"Lock panels"}}
            div{class:"m3-layout-demo m3-layout-demo--surface m3-layout-resize-frame",ResizablePanelGroup{adaptive:true,disabled:locked(),onlayoutchange:move |v|layout.set(v),
                ResizablePanel{label:"Document",default_size:65.0,min_size:25.0,class:"m3-layout-content",h3{"Document"}p{"Keep your work in focus."}Button{variant:ButtonVariant::Text,"Document action"}}
                ResizableHandle{label:"Document size"}
                ResizablePanel{label:"Comments",default_size:35.0,min_size:20.0,class:"m3-layout-content",h3{"Comments"}p{"Supporting details stay nearby."}Button{variant:ButtonVariant::Text,"Comment action"}}
            }}
            p{role:"status","Document {layout().first().copied().unwrap_or(65.0):.1}% · Comments {layout().get(1).copied().unwrap_or(35.0):.1}%"}
            h3{"Editor and console"}p{"Press Enter on the separator to collapse or restore the editor."}
            div{class:"m3-layout-demo m3-layout-demo--surface m3-layout-resize-frame--vertical",ResizablePanelGroup{direction:PanelDirection::Vertical,
                ResizablePanel{label:"Editor",default_size:70.0,min_size:20.0,collapsible:true,class:"m3-layout-content",h3{"Editor"}p{"The panel can collapse without leaving hidden controls in the tab order."}Button{variant:ButtonVariant::Text,"Editor action"}}
                ResizableHandle{label:"Editor size"}
                ResizablePanel{label:"Console",default_size:30.0,min_size:20.0,class:"m3-layout-content",h3{"Console"}p{"Build completed successfully."}}
            }}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Resizable panels usage",code:RESIZE_USAGE}
            CodeCard{eyebrow:"Copy into your Dioxus app",title:"Vertical panels usage",code:VERTICAL_RESIZE_USAGE}
        }
    }
}
