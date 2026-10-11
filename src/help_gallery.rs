use super::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::{
    Autocomplete, Checkbox, Combobox, DialogAction, HelpTrigger, HoverCard, NativeSelect, Popover,
    RichTooltip, Select, SelectOption, SelectSize, TextField, TextFieldVariant, Tooltip, icons,
};

const TOOLTIP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{HelpTrigger, Tooltip, icons};

#[component]
fn SaveWithHelp() -> Element {
    rsx! {
        HelpTrigger { id: "save-help", label: "Save", icon: icons::FAVORITE, icon_only: true }
        Tooltip { anchor_id: "save-help", text: "Save to favorites" }
    }
}"#;
const RICH_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{HelpTrigger, RichTooltip, DialogAction};

#[component]
fn SharingHelp() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        HelpTrigger { id: "sharing-help", label: "Sharing help", onclick: move |_| open.set(!open()) }
        RichTooltip {
            anchor_id: "sharing-help", title: "Share with your team", open: open(),
            onopenchange: move |next| open.set(next),
            actions: rsx! { DialogAction { label: "Got it", onclick: move |_| open.set(false) } },
            p { "People you invite can view this project." }
        }
    }
}"#;
const POPOVER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{HelpTrigger, Popover, TextField, DialogAction};

#[component]
fn QuickSettings() -> Element {
    let mut open = use_signal(|| false);
    let mut name = use_signal(String::new);
    rsx! {
        HelpTrigger { id: "quick-settings", label: "Quick settings", onclick: move |_| open.set(!open()) }
        Popover {
            anchor_id: "quick-settings", title: "Preferences", open: open(),
            onopenchange: move |next| open.set(next),
            TextField { label: "Display name", value: name(), oninput: move |next| name.set(next) }
            DialogAction { label: "Done", onclick: move |_| open.set(false) }
        }
    }
}"#;
const HOVER_USAGE: &str = r##"use dioxus::prelude::*;
use m3e_rust_ui::HoverCard;

#[component]
fn ProjectPreview() -> Element {
    rsx! {
        a { id: "project-preview", href: "#project-details", "Material project" }
        HoverCard { anchor_id: "project-preview",
            strong { "Material project" }
            p { "A shared design workspace." }
        }
        section { id: "project-details", h3 { "Material project" } p { "A shared design workspace." } }
    }
}"##;
const SELECT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Select, SelectOption};

#[component]
fn RegionPicker() -> Element {
    let mut value = use_signal(|| "eu".to_string());
    let mut open = use_signal(|| false);
    rsx! {
        Select {
            label: "Region", name: "region", value: value(), open: open(),
            options: vec![SelectOption::new("eu", "Europe"), SelectOption::new("us", "Americas")],
            onchange: move |next| value.set(next), onopenchange: move |next| open.set(next),
        }
    }
}"#;
const NATIVE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{NativeSelect, SelectOption};

#[component]
fn NativeRegionPicker() -> Element {
    let mut value = use_signal(|| "eu".to_string());
    rsx! {
        NativeSelect {
            label: "Region", name: "region", value: value(),
            options: vec![SelectOption::new("eu", "Europe"), SelectOption::new("us", "Americas")],
            onchange: move |next| value.set(next),
        }
    }
}"#;
const COMBOBOX_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Combobox, SelectOption};

#[component]
fn LanguagePicker() -> Element {
    let mut values = use_signal(Vec::<String>::new);
    let mut query = use_signal(String::new);
    let mut open = use_signal(|| false);
    rsx! {
        Combobox {
            label: "Language", values: values(), input_value: query(), open: open(),
            options: vec![SelectOption::new("rust", "Rust"), SelectOption::new("ts", "TypeScript")],
            onchange: move |next| values.set(next), oninput: move |next| query.set(next),
            onopenchange: move |next| open.set(next),
        }
    }
}"#;
const AUTOCOMPLETE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Autocomplete, SelectOption};

#[component]
fn TagEditor() -> Element {
    let mut value = use_signal(String::new);
    let mut open = use_signal(|| false);
    rsx! {
        Autocomplete {
            label: "Tag", value: value(), open: open(),
            options: vec![SelectOption::new("design", "Design"), SelectOption::new("development", "Development")],
            oninput: move |next| value.set(next), onopenchange: move |next| open.set(next),
            onselect: move |id| println!("Selected {id}"),
        }
    }
}"#;

fn regions() -> Vec<SelectOption> {
    vec![
        SelectOption::new("eu", "Europe"),
        SelectOption::new("us", "Americas"),
        SelectOption {
            disabled: true,
            ..SelectOption::new("moon", "Moon — unavailable")
        },
        SelectOption::new("apac", "Asia Pacific"),
        SelectOption::new("africa", "Africa"),
    ]
}
fn languages() -> Vec<SelectOption> {
    [
        ("rust", "Rust", "Systems"),
        ("go", "Go", "Systems"),
        ("cpp", "C++", "Systems"),
        ("ts", "TypeScript", "Web"),
        ("js", "JavaScript", "Web"),
        ("python", "Python", "Data"),
        ("r", "R", "Data"),
    ]
    .into_iter()
    .map(|(v, l, g)| SelectOption {
        group: Some(g.into()),
        ..SelectOption::new(v, l)
    })
    .collect()
}

#[component]
pub fn HelpGallery(#[props(default)] active: String) -> Element {
    let mut rich = use_signal(|| false);
    let mut popup = use_signal(|| false);
    let mut display_name = use_signal(|| "Material pilot".to_string());
    let mut notifications = use_signal(|| true);
    let mut region = use_signal(|| "eu".to_string());
    let mut select_open = use_signal(|| false);
    let mut filled = use_signal(|| "us".to_string());
    let mut filled_open = use_signal(|| false);
    let mut native = use_signal(|| "eu".to_string());
    let mut small = use_signal(|| "apac".to_string());
    let mut small_open = use_signal(|| false);
    let mut language = use_signal(|| vec!["rust".to_string()]);
    let mut query = use_signal(String::new);
    let mut combo_open = use_signal(|| false);
    let mut multi = use_signal(|| vec!["rust".to_string(), "ts".to_string()]);
    let mut multi_query = use_signal(String::new);
    let mut multi_open = use_signal(|| false);
    let mut tag = use_signal(String::new);
    let mut tag_open = use_signal(|| false);
    let mut picked = use_signal(|| "None".to_string());
    let mut rtl = use_signal(|| "eu".to_string());
    let mut rtl_open = use_signal(|| false);
    let mut filled_query = use_signal(String::new);
    let mut filled_combo_open = use_signal(|| false);
    let mut custom_topics = use_signal(Vec::<String>::new);
    let mut custom_query = use_signal(String::new);
    let mut custom_open = use_signal(|| false);
    let mut required_value = use_signal(String::new);
    let mut required_open = use_signal(|| false);
    rsx! {
        section {id:"help-and-selection",hidden:!active.is_empty()&&active!="help-and-selection",class:"wrap roles-section help-section",
            div {class:"section-heading",div {p {class:"section-index","22 — CONTEXTUAL HELP"}h2 {"Tooltips and popovers"}p {class:"section-intro","Brief labels, persistent guidance, quick settings and supplementary link previews. Keyboard focus and Escape work alongside pointer interaction."}}span {class:"pill","PLAIN · RICH · NONMODAL"}}
            div {class:"composite-grid",
                div {class:"demo-card",div {class:"demo-label","PLAIN · HOVER OR FOCUS"}div {class:"help-trigger-row",
                    HelpTrigger {id:"help-save",label:"Save favorite",icon:icons::FAVORITE,icon_only:true}
                    Tooltip {anchor_id:"help-save",text:"Save to favorites"}
                    HelpTrigger {id:"help-info",label:"Project information",icon:icons::INFO,icon_only:true}
                    Tooltip {anchor_id:"help-info",text:"Project information"}
                    HelpTrigger {id:"help-disabled",label:"Unavailable help",icon:icons::ADD,icon_only:true,disabled:true}
                    Tooltip {anchor_id:"help-disabled",text:"Disabled action"}
                }p {class:"control-label","Hover for 500ms or focus with Tab. Escape dismisses."}}
                div {class:"demo-card",div {class:"demo-label","RICH · EXPLICIT OPEN"}
                    HelpTrigger {id:"sharing-help",label:"How sharing works",onclick:move |_|rich.set(!rich())}
                    RichTooltip {anchor_id:"sharing-help",title:"Share with your team",open:rich(),onopenchange:move|v|rich.set(v),
                        actions:rsx! {DialogAction {label:"Got it",onclick:move |_|rich.set(false)}},
                        p {"Invite people to view this project. You can change their access later."}
                    }
                }
                div {class:"demo-card",div {class:"demo-label","POPOVER · QUICK SETTINGS"}
                    HelpTrigger {id:"quick-settings",label:"View preferences",onclick:move |_|popup.set(!popup())}
                    Popover {anchor_id:"quick-settings",title:"Preferences",description:"Changes apply to this preview.",open:popup(),onopenchange:move|v|popup.set(v),
                        TextField {label:"Display name",value:display_name(),oninput:move|v|display_name.set(v)}
                        div {class:"help-check-row",Checkbox {checked:notifications(),aria_label:"Enable preview notifications",onchange:move|v|notifications.set(v)}span {"Notifications"}}
                        DialogAction {label:"Done",onclick:move |_|popup.set(false)}
                    }
                    p {class:"control-label","Name: {display_name}"}
                }
                div {class:"demo-card",div {class:"demo-label","HOVER CARD · LINK PREVIEW"}
                    a {id:"project-preview",class:"help-preview-link",href:"#help-selection-details","Material project"}
                    HoverCard {anchor_id:"project-preview",strong {"Material project"}p {"A shared workspace for high-fidelity Material 3 components."}}
                    p {class:"control-label","Supplementary preview; the link remains the accessible interface."}
                }
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Tooltip usage",code:TOOLTIP_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Rich tooltip usage",code:RICH_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Popover usage",code:POPOVER_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Hover card usage",code:HOVER_USAGE}
            }
        }
        section {id:"selection-fields",hidden:!active.is_empty()&&active!="selection-fields",class:"wrap roles-section help-section",
            div {class:"section-heading",div {p {class:"section-index","23 — EXPOSED SELECTION"}h2 {"Select and autocomplete"}p {class:"section-intro","56dp fields with expressive listbox surfaces. Arrow keys preview choices, Enter commits, Escape cancels. Editable fields filter while preserving native text editing."}}span {class:"pill","OUTLINED · FILLED · EDITABLE"}}
            div {class:"composite-grid",
                div {class:"demo-card",div {class:"demo-label","OUTLINED · SELECT"}Select {label:"Region",options:regions(),value:region(),open:select_open(),supporting:"Choose where your project is hosted.",name:"region",onchange:move|v|region.set(v),onopenchange:move|v|select_open.set(v)}p {class:"control-label","Region ID: {region}"}}
                div {class:"demo-card",div {class:"demo-label","FILLED · SELECT"}Select {label:"Billing region",options:regions(),value:filled(),open:filled_open(),variant:TextFieldVariant::Filled,onchange:move|v|filled.set(v),onopenchange:move|v|filled_open.set(v)}}
                div {class:"demo-card",div {class:"demo-label","EDITABLE · GROUPED OPTIONS"}Combobox {label:"Language",options:languages(),values:language(),input_value:query(),open:combo_open(),name:"language",supporting:"Type to filter; choose a language.",onchange:move|v|language.set(v),oninput:move|v|query.set(v),onopenchange:move|v|combo_open.set(v)}p {class:"control-label","Language IDs: {language:?}"}}
                div {class:"demo-card",div {class:"demo-label","MULTIPLE · CHIPS"}Combobox {label:"Project languages",options:languages(),values:multi(),input_value:multi_query(),open:multi_open(),multiple:true,onchange:move|v|multi.set(v),oninput:move|v|multi_query.set(v),onopenchange:move|v|multi_open.set(v)}}
                div {class:"demo-card",div {class:"demo-label","AUTOCOMPLETE · FREE TEXT"}Autocomplete {label:"Project tag",name:"project_tag",options:vec![SelectOption::new("design","Design"),SelectOption::new("development","Development"),SelectOption::new("research","Research")],value:tag(),open:tag_open(),supporting:"Choose a suggestion or enter your own tag.",oninput:move|v|tag.set(v),onopenchange:move|v|tag_open.set(v),onselect:move|v|picked.set(v)}p {class:"control-label","Tag: {tag} · Last pick: {picked}"}}
                div {class:"demo-card",div {class:"demo-label","NATIVE PICKER"}NativeSelect {label:"Native region",options:regions(),value:native(),onchange:move|v|native.set(v)}p {class:"control-label","Native value: {native}. The OS draws the opened picker."}}
                div {class:"demo-card",div {class:"demo-label","DISABLED · ERROR · OPTIONAL DENSITY"}div {class:"composite-stack",
                    Select {label:"Unavailable region",options:regions(),value:"eu",disabled:true}
                    Select {label:"Required region",options:regions(),value:required_value(),open:required_open(),onchange:move|v|required_value.set(v),onopenchange:move|v|required_open.set(v),required:true,error:required_value().is_empty(),supporting:"* Required. Choose a region before continuing."}
                    Select {label:"Compact region",options:regions(),value:small(),open:small_open(),size:SelectSize::Small,onchange:move|v|small.set(v),onopenchange:move|v|small_open.set(v)}
                }}
                div {class:"demo-card",dir:"rtl",div {class:"demo-label","RIGHT TO LEFT"}Select {label:"RTL region",options:regions(),value:rtl(),open:rtl_open(),onchange:move|v|rtl.set(v),onopenchange:move|v|rtl_open.set(v)}}
                div {class:"demo-card",div {class:"demo-label","EDITABLE · FILLED AND DISABLED"}div {class:"composite-stack",
                    Combobox {label:"Search billing region",options:regions(),values:if filled().is_empty(){vec![]}else{vec![filled()]},input_value:filled_query(),open:filled_combo_open(),variant:TextFieldVariant::Filled,oninput:move|v|filled_query.set(v),onopenchange:move|v|filled_combo_open.set(v),onchange:move|v:Vec<String>|filled.set(v.first().cloned().unwrap_or_default())}
                    Combobox {label:"Disabled language",options:languages(),values:vec!["rust".to_string()],disabled:true}
                }}
                div {class:"demo-card",div {class:"demo-label","CUSTOM VALUES · REQUIRED AND ERROR"}
                    Combobox {label:"Custom topics",options:vec![SelectOption::new("design","Design"),SelectOption::new("research","Research")],values:custom_topics(),input_value:custom_query(),open:custom_open(),multiple:true,allow_custom:true,required:true,error:custom_topics().is_empty(),supporting:"* Required. Pick a topic or type one and press Enter.",onchange:move|v|custom_topics.set(v),oninput:move|v|custom_query.set(v),onopenchange:move|v|custom_open.set(v)}
                }
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Select usage",code:SELECT_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Native select usage",code:NATIVE_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Combobox usage",code:COMBOBOX_USAGE}
                CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Autocomplete usage",code:AUTOCOMPLETE_USAGE}
            }
            div {id:"help-selection-details",class:"demo-card help-project-details",h3 {"Material project"}p {"A shared workspace for high-fidelity Material 3 components."}}
        }
    }
}
