use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;

const SNACKBAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, Snackbar};
#[component]
fn SavedMessage() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Save" }
        Snackbar { open: open(), message: "Project saved", action_label: Some("Undo".into()),
            onopenchange: move |value| open.set(value), onaction: move |_| {} }
    }
}"#;
const TOAST_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, Toast};
#[component]
fn DownloadNotice() -> Element {
    let mut saved = use_signal(|| false);
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| { saved.set(true); open.set(true); }, "Download" }
        if saved() { p { "Download complete" } }
        Toast { open: open(), message: "Download complete", duration_ms: 6000,
            onopenchange: move |value| open.set(value) }
    }
}"#;
const ALERT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Alert, AlertRole, AlertVariant};
#[component]
fn ConnectionNotice() -> Element {
    rsx! { Alert { title: "Working offline", description: "Changes will sync when you reconnect.",
        variant: AlertVariant::Info, role: AlertRole::Note } }
}"#;
const SEARCH_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{SearchBar, SearchView, SearchResult};
#[component]
fn ProjectSearch() -> Element {
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    rsx! {
        SearchBar { id: "project-search", label: "Search projects", value: query(), onclick: move |_| open.set(true) }
        SearchView { anchor_id: "project-search", label: "Search projects", open: open(), value: query(),
            results: vec![SearchResult::new("studio", "Design studio")],
            oninput: move |value| query.set(value), onopenchange: move |value| open.set(value),
            onselect: move |value| query.set(value), onsubmit: move |_| open.set(false) }
    }
}"#;
const LIST_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{List, ListItem, ListSelection};
#[component]
fn WorkspaceChoice() -> Element {
    let mut selected = use_signal(|| 0usize);
    rsx! { List { aria_label: "Choose one workspace", selection: ListSelection::Single,
        for (index, label) in ["Personal", "Team", "Archive"].into_iter().enumerate() {
            ListItem { label, selected: Some(selected() == index), onactivate: move |_| selected.set(index) }
        }
    } }
}"#;
const AVATAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Avatar, AvatarGroup, AvatarSize};
#[component]
fn Contributors() -> Element {
    rsx! { AvatarGroup { aria_label: "Project contributors", additional: Some(3),
        Avatar { label: "Maya Chen", size: AvatarSize::Medium, status: Some("New activity".into()) }
        Avatar { label: "Alex Rivera" }
    } }
}"#;
const SKELETON_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{LoadingPlaceholder, Skeleton};
#[component]
fn LoadingProject() -> Element {
    let loading = use_signal(|| true);
    rsx! { LoadingPlaceholder { loading: loading(), label: "Loading project",
        placeholder: rsx! { Skeleton { height: 120 } Skeleton { width: "65%", height: 20 } },
        p { "Design studio" }
    } }
}"#;
const EMPTY_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{EmptyState, Button, icons};
#[component]
fn NoProjects() -> Element {
    rsx! { EmptyState { title: "Your next idea starts here", description: "Create a project to get started.", icon: Some(icons::ADD),
        Button { onclick: move |_| {}, "Create project" }
    } }
}"#;

#[component]
pub fn FeedbackGallery(active: String) -> Element {
    let gallery_class = "roles-section wrap feedback-gallery";
    let mut notification = use_signal(|| false);
    let mut timed = use_signal(|| false);
    let mut revision = use_signal(|| 0u64);
    let mut saved = use_signal(|| false);
    let mut undone = use_signal(|| false);
    let mut alert = use_signal(|| true);
    let mut search = use_signal(|| false);
    let mut full = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut choice = use_signal(|| 0usize);
    let mut selections = use_signal(|| vec![true, false, false]);
    let mut detail = use_signal(String::new);
    let mut loading = use_signal(|| true);
    let mut created = use_signal(|| false);
    let mut image = use_signal(|| true);
    let results = [
        "Design studio",
        "Travel plans",
        "Personal notes",
        "Archived project",
    ]
    .into_iter()
    .enumerate()
    .filter(|(_, name)| name.to_lowercase().contains(&query().to_lowercase()))
    .map(|(i, name)| {
        let mut r = SearchResult::new(name, name);
        r.supporting = Some("Workspace project".into());
        r.disabled = i == 3;
        r.icon = Some(icons::LAYERS);
        r
    })
    .collect::<Vec<_>>();
    rsx! {
        section { class:gallery_class, hidden:active!="snackbars", id:"snackbars",
            p { "One message at a time. Actionable messages persist. Alt+G focuses the snackbar; Escape dismisses it." }
            div { class:"feedback-row",
                Button { onclick:move |_| {timed.set(false);notification.set(true);revision+=1;undone.set(false);}, "Save with undo" }
                Button { variant:ButtonVariant::Outlined, onclick:move |_|{saved.set(true);timed.set(true);notification.set(true);revision+=1;}, "Show timed toast" }
            }
            if saved(){p { role:"status", "Project saved — inline confirmation" }}
            if undone(){p {role:"status", "Save undone"}}
            Snackbar { open:notification(), message:if timed(){"Project saved"}else{"Changes saved to your project"}, action_label:if timed(){None}else{Some("Undo".into())}, duration_ms:if timed(){6000}else{0},revision:revision(),onopenchange:move|v|notification.set(v),onaction:move |_|undone.set(true) }
            CodeCard {eyebrow:"FEEDBACK",title:"Snackbar",code:SNACKBAR_USAGE}
            CodeCard {eyebrow:"FEEDBACK",title:"Toast",code:TOAST_USAGE}
        }
        section {class:gallery_class,hidden:active!="alerts",id:"alerts",
            if alert(){Alert {title:"Working offline",description:"Changes will sync when you reconnect.",variant:AlertVariant::Info,role:AlertRole::Note,icon:Some(icons::INFO),dismissible:true,onclose:move |_|alert.set(false)}}
            Button {onclick:move |_|alert.set(true),"Show offline notice"}
            Alert {title:"Unable to upload",description:"Check the file and try again.",variant:AlertVariant::Error,role:AlertRole::Note,icon:Some(icons::INFO),action:Some(rsx!{Button {variant:ButtonVariant::Text,"Retry"}})}
            CodeCard {eyebrow:"FEEDBACK",title:"Alert banner",code:ALERT_USAGE}
        }
        section {class:gallery_class,hidden:active!="search",id:"search",
            p {"Contained search expands near its anchor on desktop and fills the screen at phone width."}
            div {style:"max-width:640px", SearchBar {id:"gallery-search",label:"Search projects",value:query(),onclick:move |_|{full.set(false);search.set(true);}}}
            Button {variant:ButtonVariant::Outlined,onclick:move |_|{full.set(true);search.set(true);},"Open full-screen search"}
            SearchView {anchor_id:"gallery-search",label:"Search projects",open:search(),value:query(),results,layout:if full(){SearchLayout::FullScreen}else{SearchLayout::Adaptive},oninput:move|v|query.set(v),onopenchange:move|v|search.set(v),onselect:move|v:String|{query.set(v.clone());detail.set(format!("Opened {v}"));},onsubmit:move|v|{detail.set(format!("Search submitted: {v}"));}}
            p {role:"status","{detail}"}
            CodeCard {eyebrow:"FEEDBACK",title:"Contained search",code:SEARCH_USAGE}
        }
        section {class:gallery_class,hidden:active!="lists",id:"lists",
            h3 {"Standard list geometry"}
            List {aria_label:"Project details",style:ListStyle::Standard,
                ListItem {label:"Design studio",leading:Some(rsx!{Icon {icon:icons::LAYERS}})}
                ListItem {label:"Shared workspace",supporting:Some("Collaborate with your team".into()),leading:Some(rsx!{Avatar {label:"Maya Chen",decorative:true}})}
                ListItem {label:"Launch planning",overline:Some("PROJECT".into()),supporting:Some("A three-line item with supporting detail".into()),trailing_text:Some("Today".into())}
            }
            h3 {"Choose one workspace"}
            List {aria_label:"Choose one workspace",selection:ListSelection::Single,
                for (i,label) in ["Personal","Team","Archive"].into_iter().enumerate(){ListItem {label,selected:Some(choice()==i),disabled:i==2,onactivate:move |_|choice.set(i)}}
            }
            h3 {"Choose multiple projects"}
            List {aria_label:"Choose multiple projects",selection:ListSelection::Multiple,
                for (i,label) in ["Design","Travel","Notes"].into_iter().enumerate(){ListItem {label,selected:Some(selections()[i]),onactivate:move |_|{let mut next=selections();next[i]=!next[i];selections.set(next);}}}
            }
            h3 {"Independent secondary action"}
            List {aria_label:"Project actions",ListItem {label:"Design studio",interactive:true,onactivate:move |_|detail.set("Opened Design studio".into()),secondary:Some(rsx!{IconButton {icon:icons::INFO,aria_label:Some("Design studio details".into()),onclick:move |_|detail.set("Design studio details requested".into())}})}}
            p {role:"status","{detail}"}
            CodeCard {eyebrow:"CONTENT",title:"Selection list",code:LIST_USAGE}
        }
        section {class:gallery_class,hidden:active!="avatars",id:"avatars",
            div {class:"feedback-row",Avatar {label:"Maya Chen",size:AvatarSize::Small,status:Some("New activity".into())}Avatar {label:"Alex Rivera"}Avatar {label:"Sam Lee",size:AvatarSize::Large}
                Avatar {label:"Project artwork",src:Some(if image(){asset!("/assets/card-image.svg").to_string()}else{"/missing-avatar.svg".into()})}}
            Button {variant:ButtonVariant::Outlined,onclick:move |_|image.toggle(),"Toggle image fallback"}
            AvatarGroup {aria_label:"Project contributors",additional:Some(4),Avatar {label:"Maya Chen"}Avatar {label:"Alex Rivera"}Avatar {label:"Sam Lee"}}
            CodeCard {eyebrow:"CONTENT",title:"Avatar group",code:AVATAR_USAGE}
        }
        section {class:gallery_class,hidden:active!="loading-placeholders",id:"loading-placeholders",
            Button {onclick:move |_|loading.toggle(),"Toggle loading"}
            LoadingPlaceholder {loading:loading(),label:"Loading project details",placeholder:rsx!{div {style:"display:grid;gap:16px;max-width:400px",Skeleton {height:160}Skeleton {width:"70%"}Skeleton {width:"90%"}}},p {"Design studio — project details loaded"}}
            CodeCard {eyebrow:"CONTENT",title:"Loading placeholder",code:SKELETON_USAGE}
        }
        section {class:gallery_class,hidden:active!="empty-states",id:"empty-states",
            if !created(){EmptyState {title:"Your next idea starts here",description:"Create a project to get started.",icon:Some(icons::ADD),Button {onclick:move |_|created.set(true),"Create project"}}}else{p {role:"status","Project created"}Button {variant:ButtonVariant::Text,onclick:move |_|created.set(false),"Reset example"}}
            CodeCard {eyebrow:"CONTENT",title:"Empty state",code:EMPTY_USAGE}
        }
    }
}
