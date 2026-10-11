use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::{
    BottomSheet, Button, ButtonVariant, Checkbox, Sheet, SheetSide, SheetVariant, SideSheet,
    TextField,
};

const BOTTOM_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{BottomSheet, Button};
#[component]
fn ShareSheet() -> Element {
    let mut open = use_signal(|| false);
    let mut expanded = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Share" }
        BottomSheet { title: "Share project", open: open(), expanded: expanded(),
            onopenchange: move |value| open.set(value),
            onexpandedchange: move |value| expanded.set(value),
            p { "Choose where to share your project." }
            Button { onclick: move |_| open.set(false), "Copy link" }
        }
    }
}"#;
const STANDARD_BOTTOM_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{BottomSheet, SheetVariant, Button};
#[component]
fn PlayerPane() -> Element {
    let mut open = use_signal(|| true);
    let mut expanded = use_signal(|| false);
    rsx! {
        div { style: "position:relative;height:480px;overflow:hidden",
            Button { onclick: move |_| open.set(true), "Show player" }
            BottomSheet { title: "Now playing", variant: SheetVariant::Standard,
                open: open(), expanded: expanded(),
                onopenchange: move |value| open.set(value),
                onexpandedchange: move |value| expanded.set(value),
                p { "A little inspiration · Serafina" }
            }
        }
    }
}"#;
const SIDE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{SideSheet, Button};
#[component]
fn Inspector() -> Element {
    let mut open = use_signal(|| true);
    rsx! {
        div { style: "display:flex;min-height:400px",
            main { style: "flex:1;min-width:0",
                Button { onclick: move |_| open.toggle(), "Toggle details" }
            }
            SideSheet { title: "Project details", open: open(), width: 320,
                onopenchange: move |value| open.set(value),
                p { "Supplementary content beside your workspace." }
            }
        }
    }
}"#;
const MODAL_SIDE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{SideSheet, SheetVariant, Button};
#[component]
fn FilterPanel() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Filters" }
        SideSheet { title: "Filters", open: open(), variant: SheetVariant::Modal,
            onopenchange: move |value| open.set(value),
            p { "Choose filters, then return to your results." }
            Button { onclick: move |_| open.set(false), "Apply filters" }
        }
    }
}"#;
const SHEET_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Sheet, SheetSide, Button};
#[component]
fn SupplementaryPanel() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Open panel" }
        Sheet { title: "Project notes", side: SheetSide::Start, open: open(),
            onopenchange: move |value| open.set(value),
            p { "Editable contextual information." }
        }
    }
}"#;

#[component]
pub fn SurfacesGallery(active: String) -> Element {
    let mut bottom = use_signal(|| false);
    let mut expanded = use_signal(|| false);
    let mut standard_bottom = use_signal(|| true);
    let mut standard_expanded = use_signal(|| false);
    let mut side = use_signal(|| true);
    let mut detached = use_signal(|| false);
    let mut modal_side = use_signal(|| false);
    let mut rtl_side = use_signal(|| false);
    let mut generic = use_signal(|| false);
    let mut generic_side = use_signal(|| SheetSide::Start);
    let mut name = use_signal(|| "Material workspace".to_string());
    let mut notify = use_signal(|| true);
    let mut primary = use_signal(|| 0);
    rsx! {
        section {id:"bottom-sheets",class:"roles-section wrap",hidden:active!="bottom-sheets",
            div {class:"section-heading",div {p {class:"eyebrow","25 — SURFACES"}h2 {"Bottom sheets"}p {class:"section-description","Supplementary content with partial and expanded heights, an accessible 48px handle, and native modal focus."}}}
            div {class:"surface-gallery",
                article {class:"demo-card",div {class:"demo-label","MODAL · PARTIAL / EXPANDED"}
                    p {"Try the handle with pointer, Enter/Space or arrow keys. A downward drag dismisses a partial sheet."}
                    Button {onclick:move |_|{expanded.set(false);bottom.set(true);},"Open share sheet"}
                    BottomSheet {open:bottom(),expanded:expanded(),title:"Share project",description:"A workspace for expressive interfaces.",
                        onopenchange:move|v|bottom.set(v),onexpandedchange:move|v|expanded.set(v),
                        footer:rsx!{Button {variant:ButtonVariant::Text,onclick:move |_|bottom.set(false),"Cancel"}Button {onclick:move |_|bottom.set(false),"Done"}},
                        div {class:"sheet-actions",for label in ["Copy project link","Invite a collaborator","Export Rust sources","Save to your library","Send to another workspace","Open sharing preferences"]{
                            Button {variant:ButtonVariant::Text,onclick:move |_|bottom.set(false),"{label}"}
                        }}
                        p {"The body scrolls independently when the content exceeds the available space."}
                    }
                }
                article {class:"demo-card",div {class:"demo-label","STANDARD · PRIMARY CONTENT STAYS INTERACTIVE"}
                    div {class:"standard-bottom-stage",
                        div {class:"sheet-primary-content",h3 {"Your collection"}p {"Browse your collection while the player stays open."}
                            Button {variant:ButtonVariant::Tonal,onclick:move |_|primary+=1,"Browse albums ({primary})"}
                            Button {variant:ButtonVariant::Text,onclick:move |_|standard_bottom.set(true),"Show player"}
                        }
                        BottomSheet {variant:SheetVariant::Standard,open:standard_bottom(),expanded:standard_expanded(),title:"Now playing",
                            onopenchange:move|v|standard_bottom.set(v),onexpandedchange:move|v|standard_expanded.set(v),
                            p {"A little inspiration · Serafina"}p {"The standard sheet has no scrim and does not lock the page."}
                        }
                    }
                }
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Modal bottom sheet usage",code:BOTTOM_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Standard bottom sheet usage",code:STANDARD_BOTTOM_USAGE}
            }
        }
        section {id:"side-sheets",class:"roles-section wrap",hidden:active!="side-sheets",
            div {class:"section-heading",div {p {class:"eyebrow","26 — SURFACES"}h2 {"Side sheets"}p {class:"section-description","Standard panels share the layout; modal panels focus the task. Logical start/end placement follows RTL."}}}
            div {class:"surface-gallery",
                article {class:"demo-card side-sheet-demo",div {class:"demo-label","STANDARD · DOCKED / DETACHED"}
                    div {class:"side-sheet-stage",
                        div {class:"sheet-primary-content",h3 {"Project workspace"}p {"The details panel shrinks this content as it opens."}
                            Button {variant:ButtonVariant::Tonal,onclick:move |_|side.toggle(),"Toggle project details"}
                            Button {variant:ButtonVariant::Text,onclick:move |_|detached.toggle(),"Toggle detached panel"}
                            Button {variant:ButtonVariant::Text,onclick:move |_|primary+=1,"Main action ({primary})"}
                        }
                        SideSheet {open:side(),detached:detached(),title:"Project details",width:320,onopenchange:move|v|side.set(v),
                            TextField {label:"Workspace name",value:name(),oninput:move|v|name.set(v)}
                            div {class:"sheet-toggle-row",Checkbox {checked:notify(),aria_label:"Project notifications",onchange:move|v|notify.set(v)}span {"Notifications"}}
                            p {"Dioxus · Rust · Material 3 Expressive"}
                        }
                    }
                }
                article {class:"demo-card",div {class:"demo-label","MODAL · TRAILING EDGE"}
                    Button {onclick:move |_|modal_side.set(true),"Open project filters"}
                    SideSheet {variant:SheetVariant::Modal,open:modal_side(),title:"Project filters",onopenchange:move|v|modal_side.set(v),
                        footer:rsx!{Button {onclick:move |_|modal_side.set(false),"Apply filters"}},
                        TextField {label:"Filter by name",value:name(),oninput:move|v|name.set(v)}
                        div {class:"sheet-toggle-row",Checkbox {checked:notify(),aria_label:"Include archived projects",onchange:move|v|notify.set(v)}span {"Include archived"}}
                        Button {disabled:true,"Unavailable filter"}
                    }
                }
                article {class:"demo-card",dir:"rtl",div {class:"demo-label","MODAL · RTL"}
                    Button {onclick:move |_|rtl_side.set(true),"Open RTL details"}
                    SideSheet {variant:SheetVariant::Modal,open:rtl_side(),title:"RTL project details",onopenchange:move|v|rtl_side.set(v),p {"The trailing-edge panel opens on the left in RTL."}}
                }
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Standard side sheet usage",code:SIDE_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Modal side sheet usage",code:MODAL_SIDE_USAGE}
            }
        }
        section {id:"sheets",class:"roles-section wrap",hidden:active!="sheets",
            div {class:"section-heading",div {p {class:"eyebrow","27 — SURFACES"}h2 {"Sheet composition"}p {class:"section-description","The shared sheet primitive supports logical side panels plus top/bottom placements. Top placement is an upstream extension."}}}
            article {class:"demo-card",div {class:"sheet-actions",for (side,label) in [(SheetSide::Start,"Open start sheet"),(SheetSide::End,"Open end sheet"),(SheetSide::Top,"Open top sheet"),(SheetSide::Bottom,"Open bottom sheet")]{
                Button {variant:ButtonVariant::Tonal,onclick:move |_|{generic_side.set(side);generic.set(true);},"{label}"}
            }}
                Sheet {open:generic(),side:generic_side(),title:"Project notes",onopenchange:move|v|generic.set(v),
                    TextField {label:"Notes",value:name(),multiline:true,oninput:move|v|name.set(v)}
                    p {"Your draft is preserved when the surface closes."}
                }
            }
            CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Sheet composition usage",code:SHEET_USAGE}
        }
    }
}
