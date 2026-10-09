use dioxus::prelude::*;
use m3e_rust_ui::icons;
use m3e_rust_ui::{Button, ButtonShape, ButtonSize, ButtonVariant, Checkbox, Fab, FabColor, FabMenu, FabMenuColor, FabMenuItem, FabSize, Icon, IconButton, IconButtonShape, IconButtonSize, IconButtonVariant, Switch};

mod theme;

use material_colors::color::Rgb;

const BUTTON_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, ButtonSize, ButtonVariant};

#[component]
fn CreateButton() -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Tonal,
            size: ButtonSize::Medium,
            leading_icon: Some(rsx! { span { "+" } }),
            "Create project"
        }
    }
}"#;

const SWITCH_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::Switch;

#[component]
fn NotificationsToggle() -> Element {
    let mut enabled = use_signal(|| false);
    rsx! {
        Switch {
            checked: enabled(),
            aria_label: Some("Enable notifications".to_string()),
            onchange: move |value| enabled.set(value),
        }
    }
}"#;

const CHECKBOX_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::Checkbox;

#[component]
fn TermsCheckbox() -> Element {
    let mut accepted = use_signal(|| false);
    rsx! {
        Checkbox {
            checked: accepted(),
            aria_label: Some("Accept terms".to_string()),
            onchange: move |value| accepted.set(value),
        }
    }
}"#;

const ICON_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{icons, Icon};

#[component]
fn SavedBadge() -> Element {
    rsx! {
        Icon { icon: icons::CHECK, aria_label: Some("Saved".to_string()) }
    }
}"#;

const FAB_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{icons, Fab, FabColor, FabSize};

#[component]
fn ComposeButton() -> Element {
    rsx! {
        Fab {
            icon: icons::ADD,
            size: FabSize::Standard,
            color: FabColor::PrimaryContainer,
            aria_label: Some("Compose".to_string()),
            onclick: move |_| {},
        }
    }
}"#;

const FAB_MENU_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{icons, FabMenu, FabMenuColor, FabMenuItem};

#[component]
fn ActionsMenu() -> Element {
    let mut open = use_signal(|| false);
    let items = vec![
        FabMenuItem { icon: icons::FAVORITE, label: "Favourite" },
        FabMenuItem { icon: icons::INFO, label: "Details" },
        FabMenuItem { icon: icons::ADD, label: "Add" },
    ];
    rsx! {
        FabMenu {
            items,
            color: FabMenuColor::Primary,
            open: open(),
            aria_label: Some("Actions".to_string()),
            onchange: move |value| open.set(value),
            onselect: move |index: usize| println!("picked {index}"),
        }
    }
}"#;

const ICON_BUTTON_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{icons, IconButton, IconButtonVariant};

#[component]
fn FavoriteButton() -> Element {
    let mut favorite = use_signal(|| false);
    rsx! {
        IconButton {
            icon: icons::FAVORITE,
            variant: IconButtonVariant::Tonal,
            toggle: true,
            selected: favorite(),
            aria_label: Some("Favourite".to_string()),
            onclick: move |_| favorite.toggle(),
        }
    }
}"#;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut seed = use_signal(|| theme::SEEDS[0].1);
    let mut dark = use_signal(|| false);
    let mut selected = use_signal(|| false);
    let mut switch_on = use_signal(|| true);
    let mut switch_icon_on = use_signal(|| false);
    let mut check_a = use_signal(|| false);
    let mut check_b = use_signal(|| true);
    // (indeterminate, checked): the mixed box resolves to checked when activated.
    let mut check_mixed = use_signal(|| (true, false));
    let mut menu_open = use_signal(|| false);
    let mut fav = use_signal(|| false);
    let mut bookmarked = use_signal(|| false);
    let mut menu_pick = use_signal(|| String::from("none"));
    let theme = theme::ThemePreview::from_seed(seed());
    let scheme = if dark() {
        theme.dark
    } else {
        theme.light
    };
    let theme_style = theme::css_variables(&scheme);

    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/button.css") }
        document::Stylesheet { href: asset!("/assets/switch.css") }
        document::Stylesheet { href: asset!("/assets/checkbox.css") }
        document::Stylesheet { href: asset!("/assets/icon.css") }
        document::Stylesheet { href: asset!("/assets/fab.css") }
        document::Stylesheet { href: asset!("/assets/fab-menu.css") }
        document::Stylesheet { href: asset!("/assets/icon-button.css") }

        main { class: "app-shell min-h-screen", style: theme_style,
            header { class: "topbar",
                a { class: "brand", href: "#", aria_label: "M3E home",
                    span { class: "brand-mark", "m" }
                    span { "m3e" }
                }
                nav { class: "topnav", "Foundations", span { class: "nav-current", "Color" }, "Components" }
                button {
                    class: "mode-toggle",
                    onclick: move |_| dark.toggle(),
                    span { class: "mode-icon", if dark() { "☼" } else { "◐" } }
                    if dark() { "Light mode" } else { "Dark mode" }
                }
            }

            section { class: "hero wrap",
                div { class: "hero-copy",
                    div { class: "eyebrow", span { class: "status-dot" } "THEME FOUNDATIONS / 01" }
                    h1 { "Color that " em { "means" } " something." }
                    p { class: "hero-description", "A live Material 3 color system, generated from a single source color. Change the seed or mode to see semantic roles adapt." }
                    div { class: "seed-row",
                        span { class: "seed-label", "SOURCE COLOR" }
                        for (name, value) in theme::SEEDS {
                            button {
                                class: if seed() == value { "seed-chip seed-active" } else { "seed-chip" },
                                onclick: move |_| seed.set(value),
                                span { class: "seed-dot", style: format!("background: {}", theme::css_rgb(Rgb::from_u32(value))) }
                                "{name}"
                            }
                        }
                    }
                }
                div { class: "hero-art",
                    div { class: "art-ring ring-one" }
                    div { class: "art-ring ring-two" }
                    div { class: "art-core", "M3" }
                    span { class: "art-label label-top", "HCT" }
                    span { class: "art-label label-right", "TONAL PALETTES" }
                    span { class: "art-label label-bottom", "SEMANTIC ROLES" }
                }
            }

            section { class: "wrap palette-section",
                div { class: "section-heading",
                    div {
                        p { class: "eyebrow", "01 — COLOR SYSTEM" }
                        h2 { "Tonal palettes" }
                        p { class: "section-description", "One hue, a full range of useful tones. MCU derives these palettes from the selected seed." }
                    }
                    div { class: "palette-legend", span { class: "legend-light" } "LIGHT" span { class: "legend-dark" } "DARK" }
                }
                div { class: "palette-grid",
                    PaletteColumn { name: "Primary", light: theme.palettes.primary_palette.tone(40), dark: theme.palettes.primary_palette.tone(80) }
                    PaletteColumn { name: "Secondary", light: theme.palettes.secondary_palette.tone(40), dark: theme.palettes.secondary_palette.tone(80) }
                    PaletteColumn { name: "Tertiary", light: theme.palettes.tertiary_palette.tone(40), dark: theme.palettes.tertiary_palette.tone(80) }
                    PaletteColumn { name: "Neutral", light: theme.palettes.neutral_palette.tone(40), dark: theme.palettes.neutral_palette.tone(80) }
                    PaletteColumn { name: "Neutral variant", light: theme.palettes.neutral_variant_palette.tone(40), dark: theme.palettes.neutral_variant_palette.tone(80) }
                }
            }

            section { class: "wrap roles-section button-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "02 — COMPONENT PILOT" }
                        h2 { "Buttons" }
                        p { class: "section-description", "Five emphasis styles, five Expressive sizes, and round or square shapes." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE" }
                }
                div { class: "button-demo-grid",
                    article { class: "demo-card button-specimen variants-specimen",
                        div { class: "card-topline", span { "COLOR STYLES" } span { class: "component-index", "A" } }
                        div { class: "variant-grid",
                            div { class: "button-example", Button { variant: ButtonVariant::Elevated, "Elevated" } span { "ELEVATED" } }
                            div { class: "button-example", Button { "Filled" } span { "FILLED" } }
                            div { class: "button-example", Button { variant: ButtonVariant::Tonal, "Next" } span { "TONAL" } }
                            div { class: "button-example", Button { variant: ButtonVariant::Outlined, "Outlined" } span { "OUTLINED" } }
                            div { class: "button-example", Button { variant: ButtonVariant::Text, "Cancel" } span { "TEXT" } }
                        }
                    }
                    article { class: "demo-card button-specimen sizes-specimen",
                        div { class: "card-topline", span { "EXPRESSIVE SIZES" } span { class: "component-index", "B" } }
                        div { class: "size-grid",
                            div { class: "button-example", Button { size: ButtonSize::ExtraSmall, leading_icon: Some(rsx! { span { "+" } }), "Create" } span { "XS · 32" } }
                            div { class: "button-example", Button { size: ButtonSize::Small, "Add item" } span { "S · 40" } }
                            div { class: "button-example", Button { size: ButtonSize::Medium, "Continue" } span { "M · 56" } }
                            div { class: "button-example", Button { size: ButtonSize::Large, "Confirm" } span { "L · 96" } }
                            div { class: "button-example", Button { size: ButtonSize::ExtraLarge, "Get started" } span { "XL · 136" } }
                        }
                    }
                    article { class: "demo-card button-specimen states-specimen",
                        div { class: "card-topline", span { "SHAPE & SELECTION" } span { class: "component-index", "C" } }
                        div { class: "shape-samples",
                            div { class: "button-example", Button { variant: ButtonVariant::Tonal, shape: ButtonShape::Round, "Round" } span { "ROUND" } }
                            div { class: "button-example", Button { variant: ButtonVariant::Tonal, shape: ButtonShape::Square, "Square" } span { "SQUARE" } }
                            div { class: "button-example", Button {
                                variant: ButtonVariant::Outlined,
                                toggle: true,
                                selected: selected(),
                                leading_icon: Some(rsx! { span { if selected() { "♥" } else { "♡" } } }),
                                onclick: move |_| selected.toggle(),
                                if selected() { "Favorited" } else { "Favorite" }
                            } span { "TOGGLE" } }
                        }
                        div { class: "disabled-sample",
                            div { class: "button-example", Button { disabled: true, "Disabled" } span { "DISABLED" } }
                            p { "Keyboard: Tab to focus, then Space or Enter to activate." }
                        }
                    }
                }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Button usage", code: BUTTON_USAGE }
            }

            section { class: "wrap roles-section control-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "03 — SELECTION CONTROLS" }
                        h2 { "Switch and checkbox" }
                        p { class: "section-description", "Native buttons with switch and checkbox roles. Space toggles them from the keyboard; the pressed state follows pointer and Space input." }
                    }
                    span { class: "token-note", "M3 · MATERIAL WEB TOKENS" }
                }
                div { class: "control-grid",
                    article { class: "demo-card control-specimen",
                        div { class: "card-topline", span { "SWITCH" } span { class: "component-index", "A" } }
                        div { class: "control-row",
                            span { class: "control-label", if switch_on() { "On" } else { "Off" } }
                            Switch { checked: switch_on(), aria_label: Some("Notifications".to_string()), onchange: move |value| switch_on.set(value) }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "With icons" }
                            Switch {
                                checked: switch_icon_on(),
                                aria_label: Some("Wi-Fi".to_string()),
                                checked_icon: Some(rsx! { Icon { icon: icons::CHECK } }),
                                unchecked_icon: Some(rsx! { Icon { icon: icons::CLOSE } }),
                                onchange: move |value| switch_icon_on.set(value),
                            }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "Disabled" }
                            div { class: "control-pair",
                                Switch { disabled: true, aria_label: Some("Disabled off".to_string()) }
                                Switch { disabled: true, checked: true, aria_label: Some("Disabled on".to_string()) }
                            }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "Error" }
                            Switch { error: true, aria_label: Some("Error".to_string()) }
                        }
                    }
                    article { class: "demo-card control-specimen",
                        div { class: "card-topline", span { "CHECKBOX" } span { class: "component-index", "B" } }
                        div { class: "control-row",
                            span { class: "control-label", "Unchecked" }
                            Checkbox { checked: check_a(), aria_label: Some("Unchecked".to_string()), onchange: move |value| check_a.set(value) }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "Checked" }
                            Checkbox { checked: check_b(), aria_label: Some("Checked".to_string()), onchange: move |value| check_b.set(value) }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "Indeterminate" }
                            Checkbox {
                                checked: check_mixed().1,
                                indeterminate: check_mixed().0,
                                aria_label: Some("Select all".to_string()),
                                onchange: move |value| check_mixed.set((false, value)),
                            }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "Disabled" }
                            div { class: "control-pair",
                                Checkbox { disabled: true, aria_label: Some("Disabled unchecked".to_string()) }
                                Checkbox { disabled: true, checked: true, aria_label: Some("Disabled checked".to_string()) }
                            }
                        }
                        div { class: "control-row",
                            span { class: "control-label", "Error" }
                            Checkbox { error: true, aria_label: Some("Error".to_string()) }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Switch usage", code: SWITCH_USAGE }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Checkbox usage", code: CHECKBOX_USAGE }
                }
            }
            section { class: "wrap roles-section icon-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "04 — ICONS" }
                        h2 { "Material Symbols" }
                        p { class: "section-description", "Official Material Symbols Rounded glyphs, generated from @material-symbols/svg-400@0.48.0. Icons fill with the current text colour and size through --m3-icon-size." }
                    }
                    span { class: "token-note", "APACHE-2.0 · GOOGLE" }
                }
                    article { class: "demo-card icon-specimen",
                        div { class: "card-topline", span { "ICON" } span { class: "component-index", "C" } }
                        div { class: "icon-grid",
                            for (label, icon) in [("add", icons::ADD), ("favorite", icons::FAVORITE), ("check", icons::CHECK), ("close", icons::CLOSE), ("info", icons::INFO), ("chevron_left", icons::CHEVRON_LEFT), ("arrow_back", icons::ARROW_BACK), ("remove", icons::REMOVE)] {
                                div { class: "icon-cell", Icon { icon, class: "icon-glyph" } span { "{label}" } }
                            }
                        }
                        p { class: "icon-note", "Official Material Symbols Rounded paths, generated from @material-symbols/svg-400@0.48.0." }
                    }
                CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Icon usage", code: ICON_USAGE }
            }
            section { class: "wrap roles-section fab-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "05 — FLOATING ACTION BUTTON" }
                        h2 { "Floating action button" }
                        p { class: "section-description", "Four sizes, seven colour roles, lowered elevation, and extended labels. Elevation rises on hover and returns to rest while pressed." }
                    }
                    span { class: "token-note", "M3 · MATERIAL WEB TOKENS" }
                }
                div { class: "fab-grid",
                    article { class: "demo-card fab-specimen",
                        div { class: "card-topline", span { "SIZES" } span { class: "component-index", "A" } }
                        div { class: "fab-row",
                            div { class: "button-example", Fab { icon: icons::ADD, size: FabSize::Small, aria_label: Some("Small".to_string()) } span { "S · 40" } }
                            div { class: "button-example", Fab { icon: icons::ADD, size: FabSize::Standard, aria_label: Some("Standard".to_string()) } span { "STANDARD · 56" } }
                            div { class: "button-example", Fab { icon: icons::ADD, size: FabSize::Medium, aria_label: Some("Medium".to_string()) } span { "M · 80" } }
                            div { class: "button-example", Fab { icon: icons::ADD, size: FabSize::Large, aria_label: Some("Large".to_string()) } span { "L · 96" } }
                        }
                    }
                    article { class: "demo-card fab-specimen",
                        div { class: "card-topline", span { "COLOUR ROLES" } span { class: "component-index", "B" } }
                        div { class: "fab-row fab-row-wrap",
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::PrimaryContainer, aria_label: Some("Primary container".to_string()) } span { "PRIMARY C." } }
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::Primary, aria_label: Some("Primary".to_string()) } span { "PRIMARY" } }
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::SecondaryContainer, aria_label: Some("Secondary container".to_string()) } span { "SECOND. C." } }
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::Secondary, aria_label: Some("Secondary".to_string()) } span { "SECONDARY" } }
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::TertiaryContainer, aria_label: Some("Tertiary container".to_string()) } span { "TERT. C." } }
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::Tertiary, aria_label: Some("Tertiary".to_string()) } span { "TERTIARY" } }
                            div { class: "button-example", Fab { icon: icons::ADD, color: FabColor::Surface, aria_label: Some("Surface".to_string()) } span { "SURFACE" } }
                        }
                    }
                    article { class: "demo-card fab-specimen",
                        div { class: "card-topline", span { "EXTENDED · LOWERED · DISABLED" } span { class: "component-index", "C" } }
                        div { class: "fab-row fab-row-wrap",
                            Fab { icon: icons::ADD, size: FabSize::Small, label: Some("Compose".to_string()), aria_label: Some("Compose".to_string()) }
                            Fab { icon: icons::ADD, size: FabSize::Standard, label: Some("Create".to_string()), aria_label: Some("Create".to_string()) }
                            Fab { icon: icons::ADD, size: FabSize::Medium, label: Some("New item".to_string()), aria_label: Some("New item".to_string()) }
                            Fab { icon: icons::ADD, size: FabSize::Large, label: Some("New".to_string()), aria_label: Some("New".to_string()) }
                        }
                        div { class: "fab-row",
                            Fab { icon: icons::FAVORITE, lowered: true, aria_label: Some("Favourite, lowered".to_string()) }
                            Fab { icon: icons::FAVORITE, size: FabSize::Large, lowered: true, aria_label: Some("Favourite large, lowered".to_string()) }
                            Fab { icon: icons::ADD, disabled: true, aria_label: Some("Disabled".to_string()) }
                            Fab { icon: icons::ADD, size: FabSize::Medium, label: Some("Disabled".to_string()), disabled: true, aria_label: Some("Disabled extended".to_string()) }
                        }
                    }
                    article { class: "demo-card fab-specimen",
                        div { class: "card-topline", span { "BRANDED · TOOLBAR" } span { class: "component-index", "D" } }
                        div { class: "fab-row fab-row-wrap",
                            div { class: "button-example", Fab { icon: icons::FAVORITE, branded: true, aria_label: Some("Branded standard".to_string()) } span { "BRANDED · 56" } }
                            div { class: "button-example", Fab { icon: icons::FAVORITE, branded: true, size: FabSize::Large, aria_label: Some("Branded large".to_string()) } span { "BRANDED · 96" } }
                            div { class: "button-example", Fab { icon: icons::ADD, toolbar: true, color: FabColor::SecondaryContainer, aria_label: Some("Toolbar standard".to_string()) } span { "TOOLBAR S" } }
                            div { class: "button-example", Fab { icon: icons::ADD, toolbar: true, color: FabColor::TertiaryContainer, aria_label: Some("Toolbar vibrant".to_string()) } span { "VIBRANT" } }
                            div { class: "button-example", Fab { icon: icons::ADD, toolbar: true, size: FabSize::Medium, color: FabColor::SecondaryContainer, aria_label: Some("Toolbar medium".to_string()) } span { "TOOLBAR M · 80" } }
                        }
                    }
                    article { class: "demo-card fab-specimen fab-menu-specimen",
                        div { class: "card-topline", span { "FAB MENU" } span { class: "component-index", "E" } }
                        div { class: "fab-menu-row",
                            FabMenu {
                                items: menu_items(),
                                open: menu_open(),
                                aria_label: Some("Actions".to_string()),
                                onchange: move |value| menu_open.set(value),
                                onselect: move |index: usize| menu_pick.set(menu_items()[index].label.to_string()),
                            }
                            FabMenu { items: menu_items(), color: FabMenuColor::Tertiary, open: true, aria_label: Some("Open tertiary menu".to_string()) }
                        }
                        p { class: "icon-note", "Last picked: {menu_pick}. Escape closes the menu." }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "FAB menu usage", code: FAB_MENU_USAGE }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "FAB usage", code: FAB_USAGE }
                }
            }
            section { class: "wrap roles-section icon-button-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "06 — ICON BUTTONS" }
                        h2 { "Icon buttons" }
                        p { class: "section-description", "Four variants, five sizes, and round or square shapes. Pressing morphs the corner to the expressive press shape; toggles swap round and square when selected." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE · MATERIAL WEB TOKENS" }
                }
                div { class: "icon-button-grid",
                    article { class: "demo-card icon-button-specimen",
                        div { class: "card-topline", span { "VARIANTS · UNSELECTED / SELECTED" } span { class: "component-index", "A" } }
                        div { class: "ib-row",
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Standard, aria_label: Some("Standard".to_string()) } span { "STANDARD" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Filled, aria_label: Some("Filled".to_string()) } span { "FILLED" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Tonal, aria_label: Some("Tonal".to_string()) } span { "TONAL" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Outlined, aria_label: Some("Outlined".to_string()) } span { "OUTLINED" } }
                        }
                        div { class: "ib-row",
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Standard, toggle: true, selected: fav(), aria_label: Some("Standard toggle".to_string()), onclick: move |_| fav.toggle() } span { "STANDARD · TOGGLE" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Filled, toggle: true, selected: fav(), aria_label: Some("Filled toggle".to_string()), onclick: move |_| fav.toggle() } span { "FILLED · TOGGLE" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Tonal, toggle: true, selected: fav(), aria_label: Some("Tonal toggle".to_string()), onclick: move |_| fav.toggle() } span { "TONAL · TOGGLE" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Outlined, toggle: true, selected: fav(), aria_label: Some("Outlined toggle".to_string()), onclick: move |_| fav.toggle() } span { "OUTLINED · TOGGLE" } }
                        }
                    }
                    article { class: "demo-card icon-button-specimen",
                        div { class: "card-topline", span { "SIZES · SHAPES" } span { class: "component-index", "B" } }
                        div { class: "ib-row ib-row-wrap",
                            div { class: "button-example", IconButton { icon: icons::ADD, size: IconButtonSize::ExtraSmall, variant: IconButtonVariant::Tonal, aria_label: Some("XS".to_string()) } span { "XS · 32" } }
                            div { class: "button-example", IconButton { icon: icons::ADD, size: IconButtonSize::Small, variant: IconButtonVariant::Tonal, aria_label: Some("S".to_string()) } span { "S · 40" } }
                            div { class: "button-example", IconButton { icon: icons::ADD, size: IconButtonSize::Medium, variant: IconButtonVariant::Tonal, aria_label: Some("M".to_string()) } span { "M · 56" } }
                            div { class: "button-example", IconButton { icon: icons::ADD, size: IconButtonSize::Large, variant: IconButtonVariant::Tonal, aria_label: Some("L".to_string()) } span { "L · 96" } }
                            div { class: "button-example", IconButton { icon: icons::ADD, size: IconButtonSize::ExtraLarge, variant: IconButtonVariant::Tonal, aria_label: Some("XL".to_string()) } span { "XL · 136" } }
                        }
                        div { class: "ib-row",
                            div { class: "button-example", IconButton { icon: icons::INFO, shape: IconButtonShape::Square, variant: IconButtonVariant::Filled, aria_label: Some("Square".to_string()) } span { "SQUARE" } }
                            div { class: "button-example", IconButton { icon: icons::INFO, shape: IconButtonShape::Square, variant: IconButtonVariant::Filled, toggle: true, selected: bookmarked(), aria_label: Some("Square toggle".to_string()), onclick: move |_| bookmarked.toggle() } span { "SQUARE · TOGGLE" } }
                            div { class: "button-example", IconButton { icon: icons::CHECK, variant: IconButtonVariant::Filled, disabled: true, aria_label: Some("Disabled".to_string()) } span { "DISABLED" } }
                            div { class: "button-example", IconButton { icon: icons::CHECK, variant: IconButtonVariant::Outlined, disabled: true, aria_label: Some("Disabled outlined".to_string()) } span { "DISABLED · OUTLINED" } }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Icon button usage", code: ICON_BUTTON_USAGE }
                }
            }
            footer { class: "wrap footer", span { "M3E · COMPONENT PILOT" } span { "Aligned with Material 3 Expressive guidance" } }
        }
    }
}

fn menu_items() -> Vec<FabMenuItem> {
    vec![
        FabMenuItem { icon: icons::FAVORITE, label: "Favourite" },
        FabMenuItem { icon: icons::INFO, label: "Details" },
        FabMenuItem { icon: icons::ADD, label: "Add" },
    ]
}

#[component]
fn CodeCard(eyebrow: &'static str, title: &'static str, code: &'static str) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        article { class: "code-card",
            div { class: "code-card-header",
                div {
                    p { class: "eyebrow", "{eyebrow}" }
                    h3 { "{title}" }
                }
                button {
                    class: "copy-code",
                    aria_label: if copied() { "Code copied" } else { "Copy Rust code" },
                    onclick: move |_| async move {
                        let script = format!(
                            "await navigator.clipboard.writeText({:?}); return true;",
                            code
                        );
                        copied.set(document::eval(&script).join::<bool>().await.unwrap_or(false));
                    },
                    span { class: "copy-icon", if copied() { "✓" } else { "▢" } }
                    if copied() { "Copied" } else { "Copy code" }
                }
            }
            pre { class: "code-sample", code { "{code}" } }
        }
    }
}

#[component]
fn PaletteColumn(name: &'static str, light: Rgb, dark: Rgb) -> Element {
    let light_hex = theme::css_rgb(light);
    let dark_hex = theme::css_rgb(dark);
    rsx! {
        div { class: "palette-column",
            div { class: "palette-name", "{name}" }
            div { class: "tone-pair",
                span { class: "tone-swatch", style: format!("background: {light_hex}") }
                span { class: "tone-swatch", style: format!("background: {dark_hex}") }
            }
            div { class: "tone-values", span { "40" } span { "80" } }
        }
    }
}
