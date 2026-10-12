use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const MENUBAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Menubar, MenubarEntry, MenubarSelection, MenuItem};
#[component]
fn EditorMenus() -> Element {
    let mut result = use_signal(String::new);
    rsx! { Menubar { label: "Editor menus", entries: vec![
        MenubarEntry::new("File", vec![MenuItem::new("New document").into(), MenuItem::new("Open…").into()]),
        MenubarEntry::new("Edit", vec![MenuItem::new("Undo").into()]),
    ], onselect: move |event: MenubarSelection| result.set(format!("Menu {}: {:?}", event.menu, event.selection.path)) }
    p { "{result}" } }
}"#;
const NAV_MENU_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{NavigationMenu, NavigationMenuEntry, NavigationLink};
#[component]
fn ProductNavigation() -> Element {
    rsx! { NavigationMenu { label: "Product navigation", entries: vec![
        NavigationMenuEntry::group("Explore", vec![NavigationLink::new("Overview", "/overview"), NavigationLink::new("Projects", "/projects")]),
        NavigationMenuEntry::link("About", "/about"),
    ] } }
}"#;
const SIDEBAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Sidebar, SidebarLayout, SidebarInset, NavigationDestination, icons};
#[component]
fn WorkspaceShell() -> Element {
    let mut expanded = use_signal(|| true);
    let mut mobile = use_signal(|| false);
    let mut selected = use_signal(|| "home".to_string());
    rsx! { SidebarLayout {
        Sidebar { items: vec![NavigationDestination::new("home", "Home", icons::HOME, icons::HOME_FILL)],
            selected: selected(), expanded: expanded(), mobile_open: mobile(),
            onexpandedchange: move |value| expanded.set(value), onmobileopenchange: move |value| mobile.set(value),
            onchange: move |value| selected.set(value),
        }
        SidebarInset { h2 { "Workspace" } p { "Your content goes here." } }
    } }
}"#;
fn destinations() -> Vec<NavigationDestination> {
    let mut disabled =
        NavigationDestination::new("reports", "Reports", icons::LAYERS, icons::LAYERS_FILL);
    disabled.disabled = true;
    vec![
        NavigationDestination::new("home", "Home", icons::HOME, icons::HOME_FILL),
        NavigationDestination::new("projects", "Projects", icons::LAYERS, icons::LAYERS_FILL),
        NavigationDestination::new("settings", "Settings", icons::TUNE, icons::TUNE_FILL),
        disabled,
    ]
}
#[component]
pub fn NavigationExtensionsGallery(active: String) -> Element {
    let mut result = use_signal(String::new);
    let mut autosave = use_signal(|| true);
    let mut expanded = use_signal(|| true);
    let mut mobile = use_signal(|| false);
    let mut selected = use_signal(|| "home".to_string());
    let mut unavailable = MenuItem::new("Export workspace");
    unavailable.disabled = true;
    let mut share = MenuItem::new("Share");
    share.submenu = vec![
        MenuItem::new("Copy project link").into(),
        MenuItem::new("Invite people…").into(),
    ];
    let mut checked = MenuItem::new("Auto save");
    checked.kind = MenuItemKind::Checkbox(autosave());
    let mut disabled_menu = MenubarEntry::new("Tools", vec![MenuItem::new("Extensions").into()]);
    disabled_menu.disabled = true;
    let mut unavailable_nav = NavigationMenuEntry::group("Upcoming", vec![]);
    unavailable_nav.disabled = true;
    let mut overview = NavigationLink::new("Getting started", "#navigation-menu");
    overview.active = true;
    overview.description = Some("Explore the building blocks of your next project.".into());
    let mut components = NavigationLink::new("Components", "#buttons");
    components.description = Some("Copy a component and its styles into your app.".into());
    rsx! {
     section{id:"menubars",class:"roles-section wrap",hidden:active!="menubars",div{class:"section-header",div{p{class:"eyebrow","NAVIGATION / MENUBAR"}h2{"Actions within reach"}}p{class:"section-description","An application menubar with roving focus, cross-menu arrow navigation and nested actions."}}
      article{class:"m3-navigation-extensions-card",Menubar{label:"Workspace actions",entries:vec![MenubarEntry::new("File",vec![MenuItem::new("New project").into(),MenuItem::new("Open project…").into(),MenuEntry::Divider,share.into(),unavailable.into()]),MenubarEntry::new("Edit",vec![MenuItem::new("Undo").into(),MenuItem::new("Redo").into()]),MenubarEntry::new("View",vec![checked.into(),MenuItem::new("Show details").into()]),disabled_menu],onselect:move |event:MenubarSelection|{if event.menu==2&&event.selection.path==vec![0]{if let Some(value)=event.selection.checked{autosave.set(value);}}result.set(format!("Menu {} · item {:?}",event.menu+1,event.selection.path));}}}
      p{role:"status","Last action: {result}"}Button{variant:ButtonVariant::Text,"Focus after the menubar"}
      CodeCard{eyebrow:"Copy into your Dioxus app",title:"Menubar usage",code:MENUBAR_USAGE}
     }
     section{id:"navigation-menu",class:"roles-section wrap",hidden:active!="navigation-menu",div{class:"section-header",div{p{class:"eyebrow","NAVIGATION / DESTINATIONS"}h2{"Explore related places"}}p{class:"section-description","Native links in nonmodal popups. Tab through destinations or use Escape to return to the opener."}}
      article{class:"m3-navigation-extensions-card",NavigationMenu{label:"Product sections",entries:vec![NavigationMenuEntry::group("Explore",vec![overview,components]),NavigationMenuEntry::group("Resources",vec![NavigationLink::new("Commands","#command-palette"),NavigationLink::new("Breadcrumbs","#breadcrumbs")]),NavigationMenuEntry::link("About","#foundations"),unavailable_nav]}}
      CodeCard{eyebrow:"Copy into your Dioxus app",title:"Navigation menu usage",code:NAV_MENU_USAGE}
     }
     section{id:"sidebar-helpers",class:"roles-section wrap",hidden:active!="sidebar-helpers",div{class:"section-header",div{p{class:"eyebrow","NAVIGATION / SIDEBAR"}h2{"A shell for your workspace"}}p{class:"section-description","Controlled desktop expansion and independent compact modal navigation, built on expressive rails."}}
      article{class:"m3-navigation-extensions-card",SidebarLayout{
       Sidebar{label:"Demo workspace",items:destinations(),selected:selected(),expanded:expanded(),mobile_open:mobile(),onexpandedchange:move |v|expanded.set(v),onmobileopenchange:move |v|mobile.set(v),onchange:move |v|selected.set(v),header:Some(rsx!{h3{"Atlas workspace"}}),footer:Some(rsx!{p{"Personal workspace"}})}
       SidebarInset{h3{match selected().as_str(){"projects"=>"Projects","settings"=>"Settings",_=>"Home"}}p{"A stable content area beside your navigation."}SidebarGroup{label:"Workspace summary",SidebarContent{p{"Navigation state is owned by the app."}p{"At compact widths, use the menu button to open all destinations."}}}}
      }}
      CodeCard{eyebrow:"Copy into your Dioxus app",title:"Sidebar shell usage",code:SIDEBAR_USAGE}
     }
    }
}
