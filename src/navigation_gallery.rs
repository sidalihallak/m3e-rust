use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::{
    AppBar, AppBarSize, Button, IconButton, ModalNavigationRail, NavigationBar,
    NavigationDestination, NavigationLayout, NavigationRail, Toolbar, ToolbarVariant, icons,
};

const APP_BAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{AppBar, AppBarSize, IconButton, icons};
#[component]
fn PageHeader() -> Element {
    rsx! { AppBar { title: "Library", subtitle: "Your collection", size: AppBarSize::Medium,
        leading: rsx! { IconButton { icon: icons::MENU, aria_label: "Open navigation" } },
        trailing: rsx! { IconButton { icon: icons::SEARCH, aria_label: "Search library" } }
    } }
}"#;
const RAIL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{NavigationRail, NavigationDestination, IconButton, icons};
#[component]
fn PrimaryNavigation() -> Element {
    let mut selected = use_signal(|| "home".to_string());
    let mut expanded = use_signal(|| false);
    rsx! { NavigationRail { selected: selected(), expanded: expanded(),
        items: vec![NavigationDestination::new("home", "Home", icons::HOME, icons::HOME_FILL),
            NavigationDestination::new("library", "Library", icons::LAYERS, icons::LAYERS_FILL),
            NavigationDestination::new("explore", "Explore", icons::EXPLORE, icons::EXPLORE_FILL)],
        header: rsx! { IconButton { icon: icons::MENU, aria_label: "Toggle rail", onclick: move |_| expanded.toggle() } },
        onchange: move |value| selected.set(value)
    } }
}"#;
const BAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{NavigationBar, NavigationDestination, icons};
#[component]
fn CompactNavigation() -> Element {
    let mut selected = use_signal(|| "home".to_string());
    rsx! { NavigationBar { aria_label: "Main destinations", selected: selected(),
        items: vec![NavigationDestination::new("home", "Home", icons::HOME, icons::HOME_FILL),
            NavigationDestination::new("library", "Library", icons::LAYERS, icons::LAYERS_FILL),
            NavigationDestination::new("explore", "Explore", icons::EXPLORE, icons::EXPLORE_FILL)],
        onchange: move |value| selected.set(value)
    } }
}"#;
const MODAL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, ModalNavigationRail, NavigationDestination, icons};
#[component]
fn ModalNavigation() -> Element {
    let mut open = use_signal(|| false);
    let mut selected = use_signal(|| "home".to_string());
    rsx! { Button { onclick: move |_| open.set(true), "Open navigation" }
        ModalNavigationRail { title: "Library navigation", open: open(), selected: selected(),
            items: vec![NavigationDestination::new("home", "Home", icons::HOME, icons::HOME_FILL),
                NavigationDestination::new("library", "Library", icons::LAYERS, icons::LAYERS_FILL),
                NavigationDestination::new("explore", "Explore", icons::EXPLORE, icons::EXPLORE_FILL)],
            onopenchange: move |value| open.set(value), onchange: move |value| selected.set(value)
        }
    }
}"#;
const TOOLBAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Toolbar, ToolbarVariant, IconButton, icons};
#[component]
fn EditingActions() -> Element {
    let mut favorite = use_signal(|| false);
    rsx! { Toolbar { aria_label: "Editing actions", floating: true, variant: ToolbarVariant::Vibrant,
        IconButton { icon: icons::FAVORITE, selected_icon: icons::FAVORITE_FILL,
            toggle: true, selected: favorite(), aria_label: "Favorite", onclick: move |_| favorite.toggle() }
        IconButton { icon: icons::ADD, aria_label: "Add item" }
        IconButton { icon: icons::SEARCH, aria_label: "Find item" }
    } }
}"#;

fn destinations() -> Vec<NavigationDestination> {
    let mut items = vec![
        NavigationDestination::new("home", "Home", icons::HOME, icons::HOME_FILL),
        NavigationDestination::new("library", "Library", icons::LAYERS, icons::LAYERS_FILL),
        NavigationDestination::new("explore", "Explore", icons::EXPLORE, icons::EXPLORE_FILL),
    ];
    items[1].badge = Some(3);
    items
}

#[component]
pub fn NavigationGallery(active: String) -> Element {
    let mut selected = use_signal(|| "home".to_string());
    let mut expanded = use_signal(|| false);
    let mut modal = use_signal(|| false);
    let mut scrolled = use_signal(|| false);
    let mut favorite = use_signal(|| false);
    let mut action_count = use_signal(|| 0_u32);
    let mut disabled_items = destinations();
    disabled_items[2].disabled = true;
    rsx! {section {id:"navigation",class:"wrap roles-section navigation-section",hidden:!["navigation","app-bars","navigation-bars","navigation-rails","toolbars"].contains(&active.as_str()),
        div {class:"section-heading",div {p {class:"eyebrow","24 — NAVIGATION"}h2 {"{crate::showcase::page(&active).1}"}
            p {class:"section-description","Expressive rails, destination bars, flexible app bars and action toolbars. Native controls, named landmarks and copyable Rust."}}
        }
        div {class:"navigation-demo",
            div {class:"demo-card",hidden:active!="navigation"&&active!="app-bars",div {class:"demo-label","APP BAR · SMALL"}
                AppBar {title:"Library",scrolled:scrolled(),leading:rsx!{IconButton {icon:icons::MENU,aria_label:"Demo navigation",onclick:move |_|modal.set(true)}},
                    trailing:rsx!{IconButton {icon:icons::SEARCH,aria_label:"Demo search",onclick:move |_|action_count+=1}}}
                Button {onclick:move |_|scrolled.toggle(),"Toggle scrolled surface"}
                p {class:"control-label","Actions: {action_count}"}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="app-bars",div {class:"demo-label","FLEXIBLE · MEDIUM + SUBTITLE"}
                AppBar {title:"Your collection",subtitle:"Everything in one place",size:AppBarSize::Medium,
                    leading:rsx!{IconButton {icon:icons::ARROW_BACK,aria_label:"Demo back",onclick:move |_|action_count+=1}}}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="app-bars",div {class:"demo-label","FLEXIBLE · LARGE"}AppBar {title:"Explore",size:AppBarSize::Large,
                leading:rsx!{IconButton {icon:icons::ARROW_BACK,aria_label:"Back from Explore",onclick:move |_|action_count+=1}}}}
            div {class:"demo-card",hidden:active!="navigation"&&active!="app-bars",div {class:"demo-label","CENTERED · UNEQUAL ACTIONS"}AppBar {title:"Centered",centered:true,
                leading:rsx!{IconButton {icon:icons::ARROW_BACK,aria_label:"Centered back",onclick:move |_|action_count+=1}},
                trailing:rsx!{IconButton {icon:icons::SEARCH,aria_label:"Centered search",onclick:move |_|action_count+=1}IconButton {icon:icons::ADD,aria_label:"Centered add",onclick:move |_|action_count+=1}}}}
            div {class:"demo-card",hidden:active!="navigation"&&active!="app-bars",div {class:"demo-label","FLEXIBLE · MEDIUM"}AppBar {title:"Collection",size:AppBarSize::Medium}}
            div {class:"demo-card",hidden:active!="navigation"&&active!="app-bars",div {class:"demo-label","FLEXIBLE · LARGE + SUBTITLE"}AppBar {title:"Discover",subtitle:"A little inspiration",size:AppBarSize::Large}}
            div {class:"demo-card",hidden:active!="navigation"&&active!="navigation-bars",div {class:"demo-label","NAVIGATION BAR · VERTICAL"}
                NavigationBar {items:destinations(),selected:selected(),aria_label:"Vertical bar demo",onchange:move|v|selected.set(v)}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="navigation-bars",div {class:"demo-label","NAVIGATION BAR · HORIZONTAL"}
                NavigationBar {items:destinations(),selected:selected(),layout:NavigationLayout::Horizontal,aria_label:"Horizontal bar demo",onchange:move|v|selected.set(v)}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="navigation-bars",div {class:"demo-label","NAVIGATION BAR · TALL"}
                NavigationBar {items:destinations(),selected:selected(),tall:true,aria_label:"Tall bar demo",onchange:move|v|selected.set(v)}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="navigation-bars",div {class:"demo-label","DISABLED DESTINATION · RTL"}div {dir:"rtl",
                NavigationBar {items:disabled_items,selected:selected(),aria_label:"RTL disabled bar demo",onchange:move|v|selected.set(v)}
            }}
            div {class:"demo-card",hidden:active!="navigation"&&active!="navigation-rails",div {class:"demo-label","RAIL · COLLAPSED / EXPANDED"}div {class:"rail-preview",
                NavigationRail {items:destinations(),selected:selected(),expanded:expanded(),aria_label:"Rail demo",
                    header:rsx!{IconButton {icon:if expanded(){icons::MENU_OPEN}else{icons::MENU},aria_label:"Toggle demo rail",onclick:move |_|expanded.toggle()}},
                    onchange:move|v|selected.set(v)}p {"Selected: {selected}"}
            }}
            div {class:"demo-card",hidden:active!="navigation"&&active!="navigation-rails",div {class:"demo-label","MODAL EXPANDED RAIL"}
                Button {onclick:move |_|modal.set(true),"Open modal rail"}
                ModalNavigationRail {open:modal(),items:destinations(),selected:selected(),title:"Demo destinations",onopenchange:move|v|modal.set(v),onchange:move|v|selected.set(v)}
                p {class:"control-label","Escape and scrim dismiss; focus returns to the opener."}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="toolbars",div {class:"demo-label","FLOATING TOOLBAR · VIBRANT"}
                Toolbar {aria_label:"Floating editing toolbar",floating:true,variant:ToolbarVariant::Vibrant,
                    IconButton {icon:icons::FAVORITE,selected_icon:icons::FAVORITE_FILL,toggle:true,selected:favorite(),aria_label:"Favorite item",onclick:move |_|favorite.toggle()}
                    IconButton {icon:icons::ADD,aria_label:"Add toolbar item",onclick:move |_|action_count+=1}
                    IconButton {icon:icons::SEARCH,aria_label:"Find toolbar item",onclick:move |_|action_count+=1}
                }
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="toolbars",div {class:"demo-label","DOCKED TOOLBAR"}
                Toolbar {aria_label:"Docked editing toolbar",IconButton {icon:icons::ARROW_BACK,aria_label:"Previous item",onclick:move |_|action_count+=1}
                    IconButton {icon:icons::ADD,aria_label:"Insert item",onclick:move |_|action_count+=1}
                    IconButton {icon:icons::CHECK,aria_label:"Confirm item",onclick:move |_|action_count+=1}}
            }
            div {class:"demo-card",hidden:active!="navigation"&&active!="toolbars",div {class:"demo-label","FLOATING TOOLBAR · VERTICAL"}
                Toolbar {aria_label:"Vertical editing toolbar",floating:true,orientation:NavigationLayout::Vertical,
                    IconButton {icon:icons::ADD,aria_label:"Vertical add",onclick:move |_|action_count+=1}
                    IconButton {icon:icons::CHECK,aria_label:"Vertical confirm",onclick:move |_|action_count+=1}
                    IconButton {icon:icons::CLOSE,aria_label:"Vertical unavailable",disabled:true}}
            }
            CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"App bar usage",hidden:active!="navigation"&&active!="app-bars",code:APP_BAR_USAGE}
            CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Navigation rail usage",hidden:active!="navigation"&&active!="navigation-rails",code:RAIL_USAGE}
            CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Navigation bar usage",hidden:active!="navigation"&&active!="navigation-bars",code:BAR_USAGE}
            CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Modal navigation usage",hidden:active!="navigation"&&active!="navigation-rails",code:MODAL_USAGE}
            CodeCard {eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Toolbar usage",hidden:active!="navigation"&&active!="toolbars",code:TOOLBAR_USAGE}
        }
    }}
}
