use dioxus::prelude::*;
use m3e_rust_ui::{Icon, NavigationDestination, icons};

pub const PAGES: &[(&str, &str, &str)] = &[
    ("foundations", "Color system", "Foundations"),
    ("icons", "Icons", "Foundations"),
    ("buttons", "Buttons", "Actions"),
    ("icon-buttons", "Icon buttons", "Actions"),
    ("fabs", "Floating action buttons", "Actions"),
    ("standard-button-groups", "Standard groups", "Actions"),
    ("connected-button-groups", "Connected groups", "Actions"),
    ("split-buttons", "Split buttons", "Actions"),
    ("controls", "Switches & checkboxes", "Selection"),
    ("radios", "Radio buttons", "Selection"),
    ("sliders", "Sliders", "Selection"),
    ("chips", "Chips", "Selection"),
    ("segmented-buttons", "Segmented buttons", "Selection"),
    ("text-fields", "Text fields", "Selection"),
    ("selection-fields", "Select & autocomplete", "Selection"),
    ("dialogs", "Dialogs", "Communication"),
    ("menus", "Menus", "Communication"),
    ("help-and-selection", "Tooltips & help", "Communication"),
    ("progress", "Progress indicators", "Communication"),
    ("badges", "Badges", "Communication"),
    ("cards", "Cards", "Content"),
    ("dividers", "Dividers", "Content"),
    ("bottom-sheets", "Bottom sheets", "Content"),
    ("side-sheets", "Side sheets", "Content"),
    ("sheets", "Sheet composition", "Content"),
    ("navigation", "Navigation overview", "Navigation"),
    ("app-bars", "App bars", "Navigation"),
    ("navigation-bars", "Navigation bars", "Navigation"),
    ("navigation-rails", "Navigation rails", "Navigation"),
    ("toolbars", "Toolbars", "Navigation"),
    ("tabs", "Tabs", "Navigation"),
];
pub fn page(value: &str) -> (&'static str, &'static str, &'static str) {
    PAGES
        .iter()
        .copied()
        .find(|p| {
            p.0 == if value == "help-selection-details" {
                "selection-fields"
            } else {
                value
            }
        })
        .unwrap_or(PAGES[0])
}
pub fn first(family: &str) -> &'static str {
    PAGES
        .iter()
        .find(|p| p.2 == family)
        .map(|p| p.0)
        .unwrap_or("foundations")
}
pub fn destinations() -> Vec<NavigationDestination> {
    [
        ("Foundations", icons::PALETTE, icons::PALETTE_FILL),
        ("Actions", icons::TOUCH_APP, icons::TOUCH_APP_FILL),
        ("Selection", icons::TUNE, icons::TUNE_FILL),
        ("Communication", icons::CHAT_BUBBLE, icons::CHAT_BUBBLE_FILL),
        ("Content", icons::LAYERS, icons::LAYERS_FILL),
        ("Navigation", icons::EXPLORE, icons::EXPLORE_FILL),
    ]
    .into_iter()
    .map(|(name, icon, fill)| {
        NavigationDestination::new(
            name,
            if name == "Communication" {
                "Feedback"
            } else {
                name
            },
            icon,
            fill,
        )
    })
    .collect()
}
pub fn navigate(value: &str) {
    let value = page(value).0;
    let script = format!("location.hash={value:?};");
    spawn(async move {
        let _ = document::eval(&script).await;
    });
}

pub fn use_route() -> (Signal<String>, Signal<bool>, Signal<bool>) {
    let mut route = use_signal(|| "foundations".to_string());
    let mut scrolled = use_signal(|| false);
    let mut wide = use_signal(|| false);
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(include_str!("showcase.js"));
            while let Ok((value, is_scrolled, is_wide)) = eval.recv::<(String, bool, bool)>().await
            {
                let target = if value.is_empty()
                    || value == "help-selection-details"
                    || PAGES.iter().any(|p| p.0 == value)
                {
                    Some(page(&value).0)
                } else {
                    None
                };
                if let Some(target) = target {
                    if route.peek().as_str() != target {
                        route.set(target.to_string());
                    }
                }
                if *scrolled.peek() != is_scrolled {
                    scrolled.set(is_scrolled);
                }
                if *wide.peek() != is_wide {
                    wide.set(is_wide);
                }
            }
        });
    });
    use_drop(|| {
        spawn(async {
            let _ = document::eval("window.__m3ShowcaseCleanup?.();").await;
        });
    });
    use_effect(move || {
        let _value = route();
        spawn(async {
            let _ = document::eval("if(location.hash==='#help-selection-details'){requestAnimationFrame(()=>document.getElementById('help-selection-details')?.scrollIntoView({block:'start'}));}else{window.scrollTo({top:0,behavior:'instant'});}").await;
        });
    });
    (route, scrolled, wide)
}

#[component]
pub fn Catalog(
    active: String,
    #[props(default)] compact: bool,
    #[props(default = true)] open: bool,
) -> Element {
    let mut query = use_signal(String::new);
    let family = page(&active).2;
    let entries: Vec<_> = PAGES
        .iter()
        .copied()
        .filter(|p| {
            if query().is_empty() {
                p.2 == family
            } else {
                format!("{} {}", p.1, p.2)
                    .to_lowercase()
                    .contains(&query().to_lowercase())
            }
        })
        .collect();
    rsx! {aside {class:if compact {"showcase-catalog showcase-catalog--compact"}else{"showcase-catalog"},aria_label:"Component catalogue",inert:if !open {Some("")}else{None},"aria-hidden":if !open {Some("true")}else{None},
        div {class:"catalog-inner",
        div {class:"catalog-heading",p {class:"eyebrow","M3E / RUST"}h2 {"{family}"}p {"Copyable Dioxus components"}}
        div {class:"catalog-search",Icon {icon:icons::SEARCH}input {r#type:"search",aria_label:"Find a component",placeholder:"Find a component…",value:query(),oninput:move|e|query.set(e.value())}
            if !query().is_empty(){button {r#type:"button",class:"catalog-search__clear",aria_label:"Clear component search",onclick:move |_|query.set(String::new()),Icon {icon:icons::CLOSE}}}
        }
        nav {aria_label:"Components",for (slug,label,group) in entries.iter().copied(){
            a {class:"catalog-link",href:"#{slug}",aria_current:if slug==active {Some("page")}else{None},
                "{label}" if !query().is_empty(){small {"{group}"}}
            }
        }}
        if entries.is_empty(){p {class:"catalog-empty",role:"status","No matching components."}}
        a {class:"catalog-source",href:"https://github.com/sidalihallak/m3e-rust",target:"_blank",rel:"noreferrer","View source ↗"}
    }}}
}

#[component]
pub fn PageIntro(active: String) -> Element {
    let (_, title, family) = page(&active);
    let description = match family {
        "Actions" => {
            "Help people take action with expressive shapes, clear emphasis and responsive motion."
        }
        "Selection" => {
            "Make choices feel natural with clear states, accessible controls and helpful feedback."
        }
        "Communication" => {
            "Keep people informed with contextual help, focused conversations and timely feedback."
        }
        "Content" => {
            "Give content a clear hierarchy with tonal surfaces, purposeful spacing and flexible layouts."
        }
        "Navigation" => {
            "Guide people through your app with adaptive navigation and a clear sense of place."
        }
        _ => "Create a consistent visual language with semantic colors and Material Symbols.",
    };
    rsx! {header {class:"showcase-page-intro wrap",hidden:active=="foundations",
        p {class:"showcase-breadcrumb","Components / " if family=="Communication" {"Feedback"}else{"{family}"}}
        h1 {"{title}"}
        p {class:"showcase-page-description","{description}"}
        div {class:"showcase-page-links",a {href:"#showcase-examples","Explore examples"}a {href:"https://github.com/sidalihallak/m3e-rust",target:"_blank",rel:"noreferrer","View source ↗"}}
    }}
}
