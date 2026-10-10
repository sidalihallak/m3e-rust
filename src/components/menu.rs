use super::action_control::ActionControl;
use super::motion::next_id;
use super::{ButtonVariant, Icon, IconData};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuColor {
    #[default]
    Standard,
    Vibrant,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuVariant {
    #[default]
    Expressive,
    Baseline,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemKind {
    #[default]
    Action,
    Checkbox(bool),
    Radio(bool),
}
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub icon: Option<IconData>,
    pub supporting: Option<String>,
    pub trailing_text: Option<String>,
    pub disabled: bool,
    pub kind: MenuItemKind,
    pub submenu: Vec<MenuEntry>,
}
impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            supporting: None,
            trailing_text: None,
            disabled: false,
            kind: MenuItemKind::Action,
            submenu: vec![],
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum MenuEntry {
    Item(MenuItem),
    Label(String),
    Divider,
    Gap,
}
impl From<MenuItem> for MenuEntry {
    fn from(value: MenuItem) -> Self {
        Self::Item(value)
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MenuSelection {
    /// Entry indices from the root to the activated leaf, including labels/dividers.
    pub path: Vec<usize>,
    /// Requested checked state; radio items always request true.
    pub checked: Option<bool>,
    pub kind: MenuItemKind,
}

/// Controlled anchored menu in the browser's top layer (manual Popover API).
/// The anchor must be a mounted, focusable element with a stable unique ID.
/// Disabled items remain focusable, but never activate. `onselect` identifies
/// leaves by entry path; callers own checked state. DropdownMenu/SplitButton
/// provide their trigger semantics automatically. Custom triggers need
/// aria-haspopup=menu, aria-expanded and aria-controls.
#[component]
pub fn Menu(
    anchor_id: String,
    entries: Vec<MenuEntry>,
    open: bool,
    #[props(default)] id: Option<String>,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] color: MenuColor,
    #[props(default)] variant: MenuVariant,
    #[props(default)] initial_focus_last: bool,
    #[props(default)] position: Option<(f64, f64)>,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onselect: EventHandler<MenuSelection>,
) -> Element {
    let generated = use_hook(|| next_id("m3-menu"));
    let id = id.unwrap_or(generated);
    use_effect({
        let id = id.clone();
        move || {
            let script = MENU_SCRIPT.replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(_) = eval.recv::<bool>().await {
                    onopenchange.call(false);
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Menus?.[{id:?}]?.cleanup();")).await;
            });
        }
    });
    let selection = {
        let id = id.clone();
        move |event: MenuSelection| {
            let keep_open = matches!(event.kind, MenuItemKind::Checkbox(_));
            onselect.call(event);
            if keep_open {
                return;
            }
            let script = format!("window.__m3Menus?.[{id:?}]?.requestClose(true);");
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    };
    rsx! { MenuPanel { root_id: id, entries, path: vec![], anchor_id, open, color, variant,
        aria_label, initial_focus_last, position, onselect: selection,
    }}
}

#[component]
fn MenuPanel(
    root_id: String,
    entries: Vec<MenuEntry>,
    path: Vec<usize>,
    anchor_id: String,
    open: bool,
    color: MenuColor,
    variant: MenuVariant,
    aria_label: Option<String>,
    initial_focus_last: bool,
    position: Option<(f64, f64)>,
    onselect: EventHandler<MenuSelection>,
) -> Element {
    let suffix = path
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join("-");
    let is_root = path.is_empty();
    let panel_id = if is_root {
        root_id.clone()
    } else {
        format!("{root_id}-sub-{suffix}")
    };
    let parent = if is_root {
        None
    } else {
        Some(format!("{root_id}-item-{suffix}"))
    };
    let color_class = if color == MenuColor::Vibrant {
        "vibrant"
    } else {
        "standard"
    };
    let variant_class = if variant == MenuVariant::Baseline {
        "baseline"
    } else {
        "expressive"
    };
    rsx! {
        div { id: "{panel_id}", class: "m3-menu-panel m3-menu-panel--{color_class} m3-menu-panel--{variant_class}",
            role: "menu", aria_label, "popover": "manual", tabindex: "-1",
            "data-menu-root": "{root_id}", "data-menu-parent": parent,
            "data-anchor": anchor_id, "data-open": is_root && open,
            "data-initial-last": initial_focus_last,
            "data-point-x": position.map(|p| p.0.to_string()), "data-point-y": position.map(|p| p.1.to_string()),
            aria_hidden: if is_root && open { None } else { Some("true") },
            for (index, entry) in entries.into_iter().enumerate() {
                { let mut entry_path = path.clone(); entry_path.push(index);
                  let entry_suffix = entry_path.iter().map(usize::to_string).collect::<Vec<_>>().join("-");
                  match entry {
                    MenuEntry::Label(label) => rsx! { div { class: "m3-menu-label", role: "presentation", "{label}" } },
                    MenuEntry::Divider => rsx! { div { class: "m3-menu-divider", role: "separator", aria_orientation: "horizontal" } },
                    MenuEntry::Gap => rsx! { div { class: "m3-menu-gap", role: "presentation" } },
                    MenuEntry::Item(item) => {
                        let has_submenu = !item.submenu.is_empty();
                        let sub_id = format!("{root_id}-sub-{entry_suffix}");
                        let item_id = format!("{root_id}-item-{entry_suffix}");
                        let description_id = format!("{item_id}-description");
                        let checked = match item.kind { MenuItemKind::Action => None, MenuItemKind::Checkbox(c) | MenuItemKind::Radio(c) => Some(c) };
                        let role = if has_submenu { "menuitem" } else { match item.kind { MenuItemKind::Action => "menuitem", MenuItemKind::Checkbox(_) => "menuitemcheckbox", MenuItemKind::Radio(_) => "menuitemradio" } };
                        let requested = match item.kind { MenuItemKind::Checkbox(c) => Some(!c), MenuItemKind::Radio(_) => Some(true), _ => None };
                        let has_description = item.supporting.is_some() || item.trailing_text.is_some();
                        let description = format!("{} {}", item.supporting.as_deref().unwrap_or(""), item.trailing_text.as_deref().unwrap_or(""));
                        let event_path = entry_path.clone();
                        rsx! {
                            button { id: item_id, r#type: "button", class: "m3-menu-item", role,
                                tabindex: "-1", aria_label: item.label.clone(),
                                aria_disabled: if item.disabled { Some("true") } else { None },
                                aria_checked: if has_submenu { None } else { checked },
                                aria_haspopup: if has_submenu { Some("menu") } else { None },
                                aria_expanded: if has_submenu { Some("false") } else { None },
                                aria_controls: if has_submenu { Some(sub_id.clone()) } else { None },
                                aria_describedby: if has_description { Some(description_id.clone()) } else { None },
                                "data-menu-item": "", "data-submenu": if has_submenu { Some(sub_id) } else { None },
                                "data-label": item.label.clone(),
                                onclick: move |_| { if !item.disabled && !has_submenu { onselect.call(MenuSelection { path: event_path.clone(), checked: requested, kind: item.kind }); } },
                                super::ripple::Ripple {}
                                if let Some(icon) = item.icon { Icon { icon, class: "m3-menu-item__icon" } }
                                span { class: "m3-menu-item__text",
                                    span { "{item.label}" }
                                    if let Some(supporting) = item.supporting.clone() { span { class: "m3-menu-item__supporting", aria_hidden: "true", "{supporting}" } }
                                }
                                if let Some(trailing) = item.trailing_text.clone() { span { class: "m3-menu-item__trailing", aria_hidden: "true", "{trailing}" } }
                                if checked == Some(true) && !has_submenu { Icon { icon: icons::CHECK, class: "m3-menu-item__check" } }
                                if has_submenu { Icon { icon: icons::CHEVRON_RIGHT, class: "m3-menu-item__arrow" } }
                                if has_description {
                                    span { id: description_id.clone(), class: "m3-menu-description",
                                        "{description}"
                                    }
                                }
                            }
                            if has_submenu { MenuPanel { root_id: root_id.clone(), entries: item.submenu,
                                path: entry_path, anchor_id: String::new(), open: false, color, variant,
                                aria_label: Some(item.label.clone()), initial_focus_last: false, position: None, onselect,
                            } }
                        }
                    }
                  }
                }
            }
        }
    }
}

/// A labeled native menu trigger with an anchored Menu. Own open and checked
/// state in the consuming component. Closed-menu Up/Down opens first/last item.
#[component]
pub fn DropdownMenu(
    label: String,
    entries: Vec<MenuEntry>,
    open: bool,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] disabled: bool,
    #[props(default = ButtonVariant::Tonal)] button_variant: ButtonVariant,
    #[props(default)] color: MenuColor,
    #[props(default)] variant: MenuVariant,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onselect: EventHandler<MenuSelection>,
) -> Element {
    let id = use_hook(|| next_id("m3-dropdown"));
    let trigger_id = format!("{id}-trigger");
    let menu_id = format!("{id}-menu");
    let mut last = use_signal(|| false);
    rsx! {
        ActionControl { id: trigger_id.clone(), label: label.clone(), icon, variant: button_variant, disabled,
            expanded: Some(open), controls: Some(menu_id.clone()),
            onclick: move |_| { last.set(false); onopenchange.call(!open); },
            onkeydown: move |e: KeyboardEvent| {
                if !disabled && matches!(e.key(), Key::ArrowDown | Key::ArrowUp) {
                    e.prevent_default(); last.set(e.key() == Key::ArrowUp); onopenchange.call(true);
                }
            },
        }
        Menu { id: Some(menu_id.clone()), anchor_id: trigger_id.clone(), entries, open,
            aria_label: Some(label.clone()), color, variant, initial_focus_last: last(), onopenchange, onselect,
        }
    }
}

/// Pointer context menu plus Shift+F10/ContextMenu keyboard invocation. The
/// focusable region is the restoration target; children may be app content.
#[component]
pub fn ContextMenu(
    label: String,
    entries: Vec<MenuEntry>,
    open: bool,
    #[props(default)] color: MenuColor,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onselect: EventHandler<MenuSelection>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-context"));
    let menu_id = format!("{id}-menu");
    let mut point = use_signal(|| None::<(f64, f64)>);
    rsx! {
        div { id: "{id}", class: "m3-context-region", role: "group", tabindex: "0", aria_label: label.clone(),
            aria_haspopup: "menu", aria_expanded: open, aria_controls: menu_id.clone(),
            oncontextmenu: move |e| { e.prevent_default(); let p = e.client_coordinates(); point.set(Some((p.x,p.y))); onopenchange.call(true); },
            onkeydown: { let id = id.clone(); move |e: KeyboardEvent| {
                if e.key() == Key::ContextMenu || (e.key() == Key::F10 && e.modifiers().shift()) {
                    e.prevent_default();
                    let script = format!("const r=document.getElementById({id:?}).getBoundingClientRect(); dioxus.send([r.left+12,r.top+12]);");
                    spawn(async move { let mut eval = document::eval(&script); if let Ok(p) = eval.recv::<(f64,f64)>().await { point.set(Some(p)); onopenchange.call(true); } });
                }
            }},
            {children}
        }
        Menu { id: Some(menu_id.clone()), anchor_id: id.clone(), entries, open, position: point(), aria_label: Some(label.clone()), color, onopenchange, onselect }
    }
}

const MENU_SCRIPT: &str = include_str!("menu.js");
