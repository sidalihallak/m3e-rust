use super::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::{
    AlertDialog, Button, ButtonShape, ButtonSize, ButtonVariant, ContextMenu, Dialog, DialogAction,
    DialogVariant, DropdownMenu, MenuColor, MenuEntry, MenuItem, MenuItemKind, MenuSelection,
    MenuVariant, SplitButton, StandardButtonGroup, StandardButtonItem, StandardGroupSelection,
    TextField, icons,
};

const STANDARD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{StandardButtonGroup, StandardButtonItem};

#[component]
fn EditActions() -> Element {
    rsx! {
        StandardButtonGroup {
            aria_label: "Edit actions",
            items: ["Cut", "Copy", "Paste"].into_iter().map(StandardButtonItem::new).collect::<Vec<_>>(),
            onaction: move |index| println!("Action: {index}"),
        }
    }
}"#;
const SPLIT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{SplitButton, MenuEntry, MenuItem, MenuSelection};

#[component]
fn SaveAction() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        SplitButton {
            label: "Save",
            entries: vec![MenuEntry::Item(MenuItem::new("Save a copy")), MenuItem::new("Export").into()],
            open: open(),
            onclick: move |_| println!("Save"),
            onopenchange: move |value| open.set(value),
            onselect: move |event: MenuSelection| println!("Related action: {:?}", event.path),
        }
    }
}"#;
const DIALOG_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, Dialog, DialogAction, TextField};

#[component]
fn CreateProject() -> Element {
    let mut open = use_signal(|| false);
    let mut name = use_signal(String::new);
    rsx! {
        Button { onclick: move |_| open.set(true), "Create project" }
        Dialog {
            title: "Create project", open: open(),
            description: "Choose a name for your project.",
            onopenchange: move |value| open.set(value),
            actions: rsx! {
                DialogAction { label: "Cancel", onclick: move |_| open.set(false) }
                DialogAction { label: "Create", disabled: name().trim().is_empty(),
                    onclick: move |_| { println!("Create {}", name()); open.set(false); }
                }
            },
            TextField { label: "Project name", value: name(), oninput: move |value| name.set(value) }
        }
    }
}"#;
const ALERT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{AlertDialog, Button, DialogAction};

#[component]
fn DiscardDraft() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Discard draft" }
        AlertDialog {
            title: "Discard draft?", open: open(),
            description: "This draft will be removed.",
            onopenchange: move |value| open.set(value),
            actions: rsx! {
                DialogAction { label: "Keep draft", onclick: move |_| open.set(false) }
                DialogAction { label: "Discard", onclick: move |_| { println!("Discard"); open.set(false); } }
            },
            span {}
        }
    }
}"#;
const FULLSCREEN_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, Dialog, DialogAction, DialogVariant};

#[component]
fn FullScreenEditor() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button { onclick: move |_| open.set(true), "Edit details" }
        Dialog {
            title: "Edit details", variant: DialogVariant::FullScreen, open: open(),
            onopenchange: move |value| open.set(value),
            header_action: rsx! { DialogAction { label: "Save", onclick: move |_| open.set(false) } },
            p { "Editor content" }
        }
    }
}"#;
const MENU_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{DropdownMenu, MenuItem, MenuEntry, MenuSelection};

#[component]
fn DocumentActions() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        DropdownMenu {
            label: "Document actions", open: open(),
            entries: vec![MenuItem::new("Open").into(), MenuEntry::Divider, MenuItem::new("Rename").into()],
            onopenchange: move |value| open.set(value),
            onselect: move |event: MenuSelection| println!("Chosen {:?}", event.path),
        }
    }
}"#;
const CONTEXT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ContextMenu, MenuItem, MenuSelection};

#[component]
fn DocumentContext() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        ContextMenu {
            label: "Document context", open: open(),
            entries: vec![MenuItem::new("Open").into(), MenuItem::new("Duplicate").into()],
            onopenchange: move |value| open.set(value),
            onselect: move |event: MenuSelection| println!("Chosen {:?}", event.path),
            p { "Right click, or focus and press Shift+F10." }
        }
    }
}"#;

fn actions() -> Vec<MenuEntry> {
    vec![
        MenuItem {
            icon: Some(icons::INFO),
            ..MenuItem::new("Open")
        }
        .into(),
        MenuItem::new("Rename").into(),
        MenuEntry::Divider,
        MenuItem {
            submenu: vec![
                MenuItem::new("PDF").into(),
                MenuItem::new("Markdown").into(),
            ],
            ..MenuItem::new("Export")
        }
        .into(),
        MenuItem {
            disabled: true,
            ..MenuItem::new("Unavailable")
        }
        .into(),
    ]
}
fn view_options(details: bool, compact: bool) -> Vec<MenuEntry> {
    vec![
        MenuEntry::Label("View".into()),
        MenuItem {
            kind: MenuItemKind::Checkbox(details),
            ..MenuItem::new("Show details")
        }
        .into(),
        MenuItem {
            kind: MenuItemKind::Radio(compact),
            ..MenuItem::new("Compact")
        }
        .into(),
        MenuItem {
            kind: MenuItemKind::Radio(!compact),
            ..MenuItem::new("Comfortable")
        }
        .into(),
        MenuEntry::Divider,
        MenuItem {
            submenu: vec![MenuItem::new("Date").into(), MenuItem::new("Name").into()],
            ..MenuItem::new("Sort")
        }
        .into(),
    ]
}
fn action_name(event: &MenuSelection) -> &'static str {
    match event.path.as_slice() {
        [0] => "Open",
        [1] => "Rename",
        [3, 0] => "PDF",
        [3, 1] => "Markdown",
        _ => "None",
    }
}

#[component]
pub fn CompositeGallery() -> Element {
    let mut standard_pick = use_signal(|| "None".to_string());
    let mut standard_selection = use_signal(|| vec![1_usize]);
    let mut standard_multiple = use_signal(|| vec![0_usize]);
    let mut standard_sizes_pick = use_signal(|| 0_u32);
    let mut split_open = use_signal(|| [false; 4]);
    let mut split_sizes_open = use_signal(|| [false; 5]);
    let mut split_rtl_open = use_signal(|| false);
    let mut save_count = use_signal(|| 0_u32);
    let mut related = use_signal(|| "None".to_string());
    let mut dropdown_open = use_signal(|| false);
    let mut vibrant_open = use_signal(|| false);
    let mut rtl_open = use_signal(|| false);
    let mut baseline_open = use_signal(|| false);
    let mut context_open = use_signal(|| false);
    let mut details = use_signal(|| true);
    let mut compact = use_signal(|| false);
    let mut sorted = use_signal(|| "Date".to_string());
    let mut create_open = use_signal(|| false);
    let mut alert_open = use_signal(|| false);
    let mut scroll_open = use_signal(|| false);
    let mut fullscreen_open = use_signal(|| false);
    let mut modal_menu_open = use_signal(|| false);
    let mut project_name = use_signal(String::new);
    let mut created = use_signal(|| "None".to_string());
    let mut discarded = use_signal(|| false);
    let mut editor = use_signal(|| "Material pilot".to_string());
    rsx! {
        section { class: "wrap roles-section composite-section", id: "standard-button-groups",
            div { class: "section-heading roles-heading",
                div { p { class: "eyebrow", "18 — M3 EXPRESSIVE" } h2 { "Standard button groups" }
                    p { class: "section-description", "Buttons respond to a press together: the active button grows, and adjacent buttons make room. Arrow keys move focus; Space or Enter activates." }
                }
                span { class: "token-note", "ACTIONS · SINGLE · MULTIPLE" }
            }
            div { class: "composite-grid",
                article { class: "demo-card",
                    div { class: "card-topline", span { "RELATED ACTIONS · MIXED EMPHASIS" } }
                    StandardButtonGroup {
                        aria_label: "Standard edit actions",
                        items: vec![StandardButtonItem { variant: ButtonVariant::Tonal, icon: Some(icons::REMOVE), ..StandardButtonItem::new("Cut") }, StandardButtonItem { icon: Some(icons::ADD), ..StandardButtonItem::new("Copy") }, StandardButtonItem { variant: ButtonVariant::Outlined, ..StandardButtonItem::new("Paste") }],
                        onaction: move |index: usize| standard_pick.set(["Cut","Copy","Paste"][index].into()),
                    }
                    p { class: "control-label", "Last action: {standard_pick}" }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "REQUIRED SINGLE SELECT" } }
                    StandardButtonGroup {
                        aria_label: "Standard period", selection: StandardGroupSelection::Single,
                        selected: standard_selection(), selection_required: true,
                        items: ["Daily","Weekly","Monthly"].into_iter().map(StandardButtonItem::new).collect::<Vec<_>>(),
                        onchange: move |next| standard_selection.set(next),
                    }
                }
                article { class: "demo-card", dir: "rtl",
                    div { class: "card-topline", span { "RTL · MULTIPLE · DISABLED" } }
                    StandardButtonGroup {
                        aria_label: "Standard RTL choices", selection: StandardGroupSelection::Multiple, shape: ButtonShape::Square,
                        selected: standard_multiple(),
                        items: vec![StandardButtonItem::new("Save"), StandardButtonItem::new("Info"), StandardButtonItem { disabled: true, ..StandardButtonItem::new("Add") }],
                        onchange: move |next| standard_multiple.set(next),
                    }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "ALL SIZES · ICON ACTIONS" } }
                    div { class: "composite-stack",
                        for (size,label) in [(ButtonSize::ExtraSmall,"XS"),(ButtonSize::Small,"S"),(ButtonSize::Medium,"M"),(ButtonSize::Large,"L"),(ButtonSize::ExtraLarge,"XL")] {
                            div { class: "composite-size-row", span { class: "control-label", "{label}" }
                                StandardButtonGroup { aria_label: "{label} standard icons", size,
                                    items: vec![StandardButtonItem { icon: Some(icons::FAVORITE), icon_only: true, ..StandardButtonItem::new("Favourite") }, StandardButtonItem { icon: Some(icons::INFO), icon_only: true, ..StandardButtonItem::new("Details") }],
                                    onaction: move |_| standard_sizes_pick += 1,
                                }
                            }
                        }
                    }
                    p { class: "control-label", "Activated {standard_sizes_pick} times" }
                }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Standard button group usage", code: STANDARD_USAGE }
            }
        }
        section { class: "wrap roles-section composite-section", id: "split-buttons",
            div { class: "section-heading roles-heading",
                div { p { class: "eyebrow", "19 — M3 EXPRESSIVE" } h2 { "Split buttons" }
                    p { class: "section-description", "A primary action with related options. Opening the menu rounds the trailing button and rotates its chevron; both halves keep their color roles." }
                } span { class: "token-note", "FOUR STYLES · FIVE SIZES" }
            }
            div { class: "composite-grid",
                article { class: "demo-card",
                    div { class: "card-topline", span { "EMPHASIS STYLES" } }
                    div { class: "composite-stack",
                        for (index,variant,label) in [(0,ButtonVariant::Filled,"Filled"),(1,ButtonVariant::Tonal,"Tonal"),(2,ButtonVariant::Outlined,"Outlined"),(3,ButtonVariant::Elevated,"Elevated")] {
                            div { class: "composite-size-row", span { class: "control-label", "{label}" }
                                SplitButton { label: "Save", leading_icon: icons::CHECK, entries: actions(), variant,
                                    open: split_open()[index], trailing_label: "More {label} save options",
                                    onopenchange: move |value| split_open.write()[index]=value,
                                    onclick: move |_| save_count += 1,
                                    onselect: move |e: MenuSelection| related.set(action_name(&e).into()),
                                }
                            }
                        }
                    }
                    p { class: "control-label", "Save clicked {save_count} times · related: {related}" }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "SIZES" } }
                    div { class: "composite-stack",
                        for (index,size,label) in [(0,ButtonSize::ExtraSmall,"XS"),(1,ButtonSize::Small,"S"),(2,ButtonSize::Medium,"M"),(3,ButtonSize::Large,"L"),(4,ButtonSize::ExtraLarge,"XL")] {
                            div { class: "composite-size-row", span { class: "control-label", "{label}" }
                                SplitButton { label: "Add", leading_icon: icons::ADD, icon_only: true, entries: actions(), size,
                                    open: split_sizes_open()[index], trailing_label: "More {label} add options",
                                    onopenchange: move |value| split_sizes_open.write()[index]=value,
                                    onclick: move |_| save_count += 1,
                                    onselect: move |e: MenuSelection| related.set(action_name(&e).into()),
                                }
                            }
                        }
                    }
                }
                article { class: "demo-card", dir: "rtl",
                    div { class: "card-topline", span { "RTL · DISABLED TRAILING ACTION" } }
                    SplitButton { label: "Save RTL", entries: actions(), open: split_rtl_open(), trailing_label: "More RTL save options",
                        onopenchange: move |value| split_rtl_open.set(value), onclick: move |_| save_count += 1,
                        onselect: move |e: MenuSelection| related.set(action_name(&e).into()),
                    }
                    SplitButton { label: "Disabled", entries: actions(), open: false, disabled: true, trailing_label: "Disabled split options" }
                    SplitButton { label: "Primary only", entries: actions(), open: false, trailing_disabled: true,
                        onclick: move |_| save_count += 1, trailing_label: "Unavailable primary options",
                    }
                }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Split button usage", code: SPLIT_USAGE }
            }
        }
        section { class: "wrap roles-section composite-section", id: "dialogs",
            div { class: "section-heading roles-heading",
                div { p { class: "eyebrow", "20 — MODAL SURFACES" } h2 { "Dialogs" }
                    p { class: "section-description", "Basic forms, urgent confirmations, scrollable content and a full-screen editor. Focus stays within the dialog and returns to its opener." }
                } span { class: "token-note", "BASIC · ALERT · FULL SCREEN" }
            }
            div { class: "composite-grid",
                article { class: "demo-card",
                    div { class: "card-topline", span { "FORM · BASIC DIALOG" } }
                    Button { onclick: move |_| create_open.set(true), "Create project dialog" }
                    p { class: "control-label", "Latest project: {created}" }
                    Dialog { title: "Create project", open: create_open(), description: "Choose a name for your project.",
                        onopenchange: move |value| create_open.set(value),
                        actions: rsx! {
                            DialogAction { label: "Cancel", onclick: move |_| create_open.set(false) }
                            DialogAction { label: "Create", disabled: project_name().trim().is_empty(), onclick: move |_| { created.set(project_name()); create_open.set(false); } }
                        },
                        TextField { label: "Project name", value: project_name(), oninput: move |value| project_name.set(value) }
                        DropdownMenu { label: "Project template", entries: vec![MenuItem::new("Blank").into(),MenuItem::new("Dashboard").into()], open: modal_menu_open(),
                            onopenchange: move |value| modal_menu_open.set(value), onselect: move |_| {},
                        }
                    }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "ALERT · ICON · SAFE CANCEL" } }
                    Button { variant: ButtonVariant::Tonal, onclick: move |_| alert_open.set(true), "Discard draft dialog" }
                    p { class: "control-label", if discarded() { "Draft discarded" } else { "Draft retained" } }
                    AlertDialog { title: "Discard draft?", open: alert_open(), icon: icons::INFO,
                        description: "This clears the draft in this preview.", onopenchange: move |value| alert_open.set(value),
                        actions: rsx! {
                            DialogAction { label: "Keep draft", onclick: move |_| alert_open.set(false) }
                            DialogAction { label: "Discard", onclick: move |_| { discarded.set(true); alert_open.set(false); } }
                        },
                        span {}
                    }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "SCROLLABLE CONTENT" } }
                    Button { variant: ButtonVariant::Outlined, onclick: move |_| scroll_open.set(true), "Read guidelines dialog" }
                    Dialog { title: "Project guidelines", open: scroll_open(), onopenchange: move |value| scroll_open.set(value),
                        actions: rsx! { DialogAction { label: "Done", onclick: move |_| scroll_open.set(false) } },
                        for i in 1..=24 { p { "Guideline {i}: Keep labels clear, support keyboard navigation, and provide meaningful interaction feedback." } }
                    }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "FULL-SCREEN EDITOR" } }
                    Button { variant: ButtonVariant::Tonal, onclick: move |_| fullscreen_open.set(true), "Open full-screen editor" }
                    Dialog { title: "Edit details", open: fullscreen_open(), variant: DialogVariant::FullScreen,
                        onopenchange: move |value| fullscreen_open.set(value),
                        header_action: rsx! { DialogAction { label: "Save details", onclick: move |_| fullscreen_open.set(false) } },
                        TextField { label: "Document title", value: editor(), oninput: move |value| editor.set(value) }
                        p { "This example always fills the window. Use Adaptive in responsive apps to show full-screen dialogs on compact windows and basic dialogs on wider windows." }
                    }
                }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Dialog usage", code: DIALOG_USAGE }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Alert dialog usage", code: ALERT_USAGE }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Full-screen dialog usage", code: FULLSCREEN_USAGE }
            }
        }
        section { class: "wrap roles-section composite-section", id: "menus",
            div { class: "section-heading roles-heading",
                div { p { class: "eyebrow", "21 — ANCHORED SURFACES" } h2 { "Menus" }
                    p { class: "section-description", "Standard and vibrant menus with related actions, selection and submenus. Use arrows, Home/End and typeahead; Escape closes and Tab leaves the menu." }
                } span { class: "token-note", "DROPDOWN · CONTEXT · CASCADING" }
            }
            div { class: "composite-grid",
                article { class: "demo-card",
                    div { class: "card-topline", span { "ACTIONS · SUPPORTING TEXT · SUBMENU" } }
                    DropdownMenu { label: "Document actions", entries: actions(), open: dropdown_open(),
                        onopenchange: move |value| dropdown_open.set(value),
                        onselect: move |e: MenuSelection| related.set(action_name(&e).into()),
                    }
                    p { class: "control-label", "Chosen: {related}" }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "VIBRANT · CHECKBOX / RADIO" } }
                    DropdownMenu { label: "View options", entries: view_options(details(),compact()), open: vibrant_open(), color: MenuColor::Vibrant,
                        onopenchange: move |value| vibrant_open.set(value),
                        onselect: move |e: MenuSelection| {
                            match e.path.as_slice() {
                                [1] => details.set(e.checked.unwrap_or(false)),
                                [2] => compact.set(true), [3] => compact.set(false),
                                [5,0] => sorted.set("Date".into()), [5,1] => sorted.set("Name".into()), _=>{}
                            }
                        },
                    }
                    p { class: "control-label", if details() { "Details shown" } else { "Details hidden" } }
                    p { class: "control-label", if compact() { "Compact layout" } else { "Comfortable layout" } }
                    p { class: "control-label", "Sort: {sorted}" }
                }
                article { class: "demo-card", dir: "rtl",
                    div { class: "card-topline", span { "RTL CASCADING" } }
                    DropdownMenu { label: "RTL actions", entries: actions(), open: rtl_open(),
                        onopenchange: move |value| rtl_open.set(value), onselect: move |e: MenuSelection| related.set(action_name(&e).into()),
                    }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "BASELINE OPTION" } }
                    DropdownMenu { label: "Baseline menu", entries: vec![MenuItem { supporting: Some("Current document".into()), trailing_text: Some("Online".into()), ..MenuItem::new("Open") }.into(), MenuItem::new("Rename").into()], open: baseline_open(), variant: MenuVariant::Baseline,
                        onopenchange: move |value| baseline_open.set(value), onselect: move |_| related.set("Baseline action".into()),
                    }
                }
                article { class: "demo-card",
                    div { class: "card-topline", span { "CONTEXT MENU · SHIFT + F10" } }
                    ContextMenu { label: "Document context", entries: actions(), open: context_open(),
                        onopenchange: move |value| context_open.set(value), onselect: move |e: MenuSelection| related.set(action_name(&e).into()),
                        p { "Right-click this document, or focus it and press Shift+F10." }
                    }
                }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Menu usage", code: MENU_USAGE }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Context menu usage", code: CONTEXT_USAGE }
            }
        }
    }
}
