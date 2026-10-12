use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const COMMAND_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Command, CommandEntry};
#[component]
fn ProjectCommands() -> Element {
    let mut query = use_signal(String::new);
    let mut selected = use_signal(String::new);
    let mut create = CommandEntry::new("create", "Create project");
    create.group = "Projects".into();
    create.keywords = vec!["new".into()];
    rsx! { Command { label: "Project commands", entries: vec![create], query: query(),
        onquerychange: move |value| query.set(value), onselect: move |value| selected.set(value),
    } p { "Selected: {selected}" } }
}"#;
const DIALOG_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, CommandDialog, CommandEntry};
#[component]
fn ProjectPalette() -> Element {
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut selected = use_signal(String::new);
    rsx! { Button { onclick: move |_| { query.set(String::new()); open.set(true); }, "Open commands" }
        CommandDialog { open: open(), entries: vec![CommandEntry::new("create", "Create project")],
            query: query(), onquerychange: move |value| query.set(value),
            onopenchange: move |value| open.set(value), onselect: move |value| selected.set(value),
        } p { "Selected: {selected}" }
    }
}"#;
const BREADCRUMB_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{BreadcrumbTrail, BreadcrumbEntry};
#[component]
fn ProjectLocation() -> Element {
    rsx! { BreadcrumbTrail { label: "Project location", entries: vec![
        BreadcrumbEntry::link("Home", "/"), BreadcrumbEntry::link("Projects", "/projects"),
        BreadcrumbEntry::page("Atlas"),
    ] } }
}"#;
const CUSTOM_BREADCRUMB_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};
#[component]
fn DocumentLocation() -> Element {
    rsx! { Breadcrumb { label: "Document location", BreadcrumbList {
        BreadcrumbItem { BreadcrumbLink { href: "/projects", "Projects" } }
        BreadcrumbSeparator { span { "/" } }
        BreadcrumbItem { BreadcrumbPage { "Design brief" } }
    } } }
}"#;
fn commands() -> Vec<CommandEntry> {
    let mut entries = vec![
        CommandEntry::new("create", "Create project"),
        CommandEntry::new("open", "Open project"),
        CommandEntry::new("archive", "Archive workspace"),
        CommandEntry::new("settings", "Settings"),
        CommandEntry::new("notifications", "Notifications"),
    ];
    for entry in &mut entries {
        entry.group = if ["settings", "notifications"].contains(&entry.value.as_str()) {
            "Preferences"
        } else {
            "Projects"
        }
        .into();
        entry.icon = Some(if entry.group == "Projects" {
            icons::HOME
        } else {
            icons::SEARCH
        });
    }
    entries[0].keywords = vec!["new".into(), "workspace".into()];
    entries[0].shortcut = Some("⌘ N".into());
    entries[2].disabled = true;
    entries[3].shortcut = Some("⌘ ,".into());
    entries[4].checked = true;
    entries
}
#[component]
pub fn WayfindingGallery(active: String) -> Element {
    let mut query = use_signal(String::new);
    let mut dialog_query = use_signal(String::new);
    let mut open = use_signal(|| false);
    let mut selected = use_signal(String::new);
    rsx! {
        section {id:"command-palette",class:"roles-section wrap",hidden:active!="command-palette",
            div {class:"section-header",div {p {class:"eyebrow","ACTIONS / COMMAND PALETTE"}h2 {"Find an action, stay in flow"}}p {class:"section-description","Search labels and keywords. Arrow keys preview actions; Enter runs the active command. Shortcut hints are illustrative."}}
            article {class:"visual-gallery__card",h3 {"Inline commands"}
                Command {label:"Project commands",entries:commands(),query:query(),onquerychange:move |v|query.set(v),onselect:move |v|selected.set(v)}
            }
            div {class:"button-row",Button {onclick:move |_|{dialog_query.set(String::new());open.set(true);},"Open command palette"}}
            p {role:"status","Last command: {selected}"}
            CommandDialog {title:"Command palette",open:open(),entries:commands(),query:dialog_query(),onquerychange:move |v|dialog_query.set(v),onopenchange:move |v|open.set(v),onselect:move |v|selected.set(v)}
            details {summary {"Disabled command surface"}Command {label:"Unavailable commands",entries:commands(),query:String::new(),disabled:true,onquerychange:move |_|{},onselect:move |_|{}}}
            CodeCard {eyebrow:"Copy into your Dioxus app",title:"Inline command usage",code:COMMAND_USAGE}
            CodeCard {eyebrow:"Copy into your Dioxus app",title:"Command dialog usage",code:DIALOG_USAGE}
        }
        section {id:"breadcrumbs",class:"roles-section wrap",hidden:active!="breadcrumbs",
            div {class:"section-header",div {p {class:"eyebrow","NAVIGATION / BREADCRUMBS"}h2 {"A clear path back"}}p {class:"section-description","Native parent links and a non-interactive current page. Trails wrap at narrow widths."}}
            article {class:"visual-gallery__card",h3 {"Project location"}BreadcrumbTrail {label:"Project location",entries:vec![BreadcrumbEntry::link("Home","#foundations"),BreadcrumbEntry::link("Projects","#cards"),BreadcrumbEntry::page("Atlas")]}}
            article {class:"visual-gallery__card",h3 {"Custom separator"}Breadcrumb {label:"Document location",BreadcrumbList {
                BreadcrumbItem {BreadcrumbLink {href:"#cards",Icon {icon:icons::HOME} "Projects"}}
                BreadcrumbSeparator {span {"/"}}
                BreadcrumbItem {BreadcrumbPage {"Design brief"}}
            }}}
            article {class:"visual-gallery__card",h3 {"Long hierarchy"}BreadcrumbTrail {label:"Long project location",entries:vec![BreadcrumbEntry::link("Workspace","#foundations"),BreadcrumbEntry::link("Research and product development","#lists"),BreadcrumbEntry::link("Design systems and accessibility","#help-and-selection"),BreadcrumbEntry::page("A detailed design specification for the Atlas workspace")]}}
            CodeCard {eyebrow:"Copy into your Dioxus app",title:"Breadcrumb trail usage",code:BREADCRUMB_USAGE}
            CodeCard {eyebrow:"Copy into your Dioxus app",title:"Composed breadcrumb usage",code:CUSTOM_BREADCRUMB_USAGE}
        }
    }
}
