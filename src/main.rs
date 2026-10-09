use dioxus::prelude::*;
use m3e_rust_ui::icons;
use m3e_rust_ui::{Button, ButtonShape, ButtonSize, ButtonVariant, Checkbox, Chip, ChipVariant, ChipSize, CircularProgress, Fab, FabColor, LinearProgress, LoadingIndicator, LoadingIndicatorVariant, Radio, RadioGroup, SegmentedButton, SegmentedButtonSet, Slider, SliderSize, TabItem, Tabs, TabsVariant, BadgeAnchor, Card, CardActions, Divider, DividerInset, DividerOrientation, TextField, TextFieldVariant, CardBody, CardMedia, CardVariant, NotificationBadge, FabMenu, FabMenuColor, FabMenuItem, FabSize, Icon, IconButton, IconButtonShape, IconButtonSize, IconButtonVariant, IconButtonWidth, Switch};

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

const CHIP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Chip, ChipVariant, ChipSize};

#[component]
fn DietFilter() -> Element {
    let mut vegan = use_signal(|| false);
    rsx! {
        Chip {
            label: "Vegan".to_string(),
            variant: ChipVariant::Filter,
            selected: vegan(),
            onclick: move |_| vegan.toggle(),
        }
    }
}"#;

const PROGRESS_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{LinearProgress, CircularProgress};

#[component]
fn UploadProgress() -> Element {
    let mut fraction = use_signal(|| 0.0_f64);
    rsx! {
        LinearProgress { value: Some(fraction()), wavy: true, aria_label: Some("Upload".to_string()) }
        CircularProgress { value: None, aria_label: Some("Saving".to_string()) }
        button { onclick: move |_| fraction.set((fraction() + 0.1).min(1.0)), "Advance" }
    }
}"#;

const LOADING_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{LoadingIndicator, LoadingIndicatorVariant};

#[component]
fn Saving() -> Element {
    rsx! {
        LoadingIndicator { variant: LoadingIndicatorVariant::Contained }
    }
}"#;

const RADIO_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Radio, RadioGroup};

#[component]
fn SizePicker() -> Element {
    let mut size = use_signal(|| "medium".to_string());
    rsx! {
        RadioGroup { aria_label: Some("Size".to_string()),
            for (value, label) in [("small", "Small"), ("medium", "Medium"), ("large", "Large")] {
                Radio {
                    name: "size".to_string(),
                    value: value.to_string(),
                    checked: size() == value,
                    aria_label: Some(label.to_string()),
                    onchange: move |_| size.set(value.to_string()),
                }
            }
        }
    }
}"#;

const SLIDER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::Slider;

#[component]
fn VolumeControl() -> Element {
    let mut volume = use_signal(|| 40.0_f64);
    rsx! {
        Slider {
            min: 0.0,
            max: 100.0,
            value: volume(),
            aria_label: Some("Volume".to_string()),
            onchange: move |value| volume.set(value),
        }
    }
}"#;

const RANGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::Slider;

#[component]
fn PriceRange() -> Element {
    let mut range = use_signal(|| (20.0_f64, 70.0_f64));
    rsx! {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 10.0,
            ticks: true,
            label: true,
            value: range().0,
            value_end: Some(range().1),
            onchange_range: move |(start, end): (f64, f64)| range.set((start, end)),
        }
        Slider {
            min: 0.0,
            max: 100.0,
            value: 60.0,
            vertical: true,
            label: true,
            aria_label: Some("Level".to_string()),
        }
    }
}"#;

const SEGMENTED_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{SegmentedButton, SegmentedButtonSet};

#[component]
fn PeriodPicker() -> Element {
    let mut period = use_signal(|| 1_usize);
    let labels = ["Day", "Week", "Month"];
    rsx! {
        SegmentedButtonSet { aria_label: "Period".to_string(),
            for (index, label) in labels.into_iter().enumerate() {
                SegmentedButton {
                    key: "{label}",
                    label: label.to_string(),
                    name: "period".to_string(),
                    value: label.to_string(),
                    checked: period() == index,
                    onchange: move |checked: bool| if checked { period.set(index) },
                }
            }
        }
    }
}"#;

const TABS_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{TabItem, Tabs};

#[component]
fn TripTabs() -> Element {
    let mut selected = use_signal(|| 0_usize);
    let labels = ["Flights", "Hotels", "Cars"];
    rsx! {
        Tabs {
            items: labels.iter().map(|label| TabItem::new(*label)).collect::<Vec<_>>(),
            selected: selected(),
            tab_ids: (0..3).map(|i| format!("trip-tab-{i}")).collect::<Vec<_>>(),
            panel_ids: (0..3).map(|i| format!("trip-panel-{i}")).collect::<Vec<_>>(),
            aria_label: Some("Trip type".to_string()),
            onchange: move |index: usize| selected.set(index),
        }
        for (index, label) in labels.into_iter().enumerate() {
            div { id: "trip-panel-{index}", role: "tabpanel", aria_labelledby: "trip-tab-{index}", hidden: selected() != index, "{label} content" }
        }
    }
}"#;

const BADGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{BadgeAnchor, Icon, NotificationBadge};
use m3e_rust_ui::icons;

#[component]
fn Inbox(unread: u32) -> Element {
    rsx! {
        BadgeAnchor {
            Icon { icon: icons::INFO }
            // Omit `count` for the dot; a count above 999 shows as 999+.
            NotificationBadge { count: unread }
        }
    }
}"#;

const CARD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, ButtonVariant, Card, CardActions, CardBody, CardMedia, CardVariant};

#[component]
fn EventCard() -> Element {
    rsx! {
        Card { variant: CardVariant::Elevated,
            CardMedia { src: "/assets/event.jpg", alt: "Crowd under balloons" }
            CardBody {
                span { class: "m3-card__headline", "Glass Souls' World Tour" }
                span { class: "m3-card__subhead", "From your recent favorites" }
                span { class: "m3-card__supporting", "Tickets go on sale Friday at 10:00." }
            }
            CardActions { start: true,
                Button { variant: ButtonVariant::Filled, "Buy tickets" }
            }
        }
    }
}"#;

const DIVIDER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Divider, DividerInset};

#[component]
fn Settings() -> Element {
    rsx! {
        span { "Wi-Fi" }
        Divider { inset: DividerInset::Start }
        span { "Bluetooth" }
    }
}"#;

const TEXT_FIELD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{TextField, TextFieldVariant};
use m3e_rust_ui::icons;

#[component]
fn Signup() -> Element {
    let mut email = use_signal(String::new);
    let invalid = !email().contains('@') && !email().is_empty();
    rsx! {
        TextField {
            variant: TextFieldVariant::Outlined,
            label: "Email",
            leading_icon: icons::INFO,
            max_length: 80,
            input_type: "email",
            value: email(),
            error: invalid,
            supporting: if invalid { "Enter a valid email address".to_string() } else { "We never share it".to_string() },
            oninput: move |value: String| email.set(value),
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
    let mut tertiary_menu_open = use_signal(|| true);
    let mut menu_open = use_signal(|| false);
    let mut fav = use_signal(|| false);
    let mut bookmarked = use_signal(|| false);
    let mut filter_a = use_signal(|| false);
    let mut filter_b = use_signal(|| true);
    let mut input_shown = use_signal(|| true);
    let mut progress = use_signal(|| 0.4_f64);
    let mut radio = use_signal(|| 1_usize);
    let mut volume = use_signal(|| 40.0_f64);
    let mut stepped = use_signal(|| 50.0_f64);
    let mut price = use_signal(|| (20.0_f64, 70.0_f64));
    let mut level = use_signal(|| 60.0_f64);
    let mut balance = use_signal(|| 30.0_f64);
    let mut dynamic_progress = use_signal(|| false);
    let mut renamed_field = use_signal(|| false);
    let mut tick_value = use_signal(|| 30.0_f64);
    let mut period = use_signal(|| 1_usize);
    let mut trip = use_signal(|| 0_usize);
    let mut detail = use_signal(|| 1_usize);
    let mut spec = use_signal(|| 0_usize);
    let mut card_taps = use_signal(|| 0_u32);
    let card_image = asset!("/assets/card-image.svg");
    let mut views = use_signal(|| [true, false, true]);
    let mut name_value = use_signal(String::new);
    let mut email_value = use_signal(|| String::from("not-an-email"));
    let mut note_value = use_signal(String::new);
    let mut search_value = use_signal(String::new);
    let mut price_value = use_signal(String::new);
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
        document::Stylesheet { href: asset!("/assets/ripple.css") }
        document::Stylesheet { href: asset!("/assets/switch.css") }
        document::Stylesheet { href: asset!("/assets/checkbox.css") }
        document::Stylesheet { href: asset!("/assets/icon.css") }
        document::Stylesheet { href: asset!("/assets/fab.css") }
        document::Stylesheet { href: asset!("/assets/fab-menu.css") }
        document::Stylesheet { href: asset!("/assets/icon-button.css") }
        document::Stylesheet { href: asset!("/assets/chip.css") }
        document::Stylesheet { href: asset!("/assets/progress.css") }
        document::Stylesheet { href: asset!("/assets/loading-indicator.css") }
        document::Stylesheet { href: asset!("/assets/radio.css") }
        document::Stylesheet { href: asset!("/assets/slider.css") }
        document::Stylesheet { href: asset!("/assets/segmented-button.css") }
        document::Stylesheet { href: asset!("/assets/tabs.css") }
        document::Stylesheet { href: asset!("/assets/badge.css") }
        document::Stylesheet { href: asset!("/assets/card.css") }
        document::Stylesheet { href: asset!("/assets/divider.css") }
        document::Stylesheet { href: asset!("/assets/text-field.css") }

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
                            FabMenu { items: menu_items(), color: FabMenuColor::Tertiary, open: tertiary_menu_open(), aria_label: Some("Open tertiary menu".to_string()), onchange: move |v| tertiary_menu_open.set(v), onselect: move |index: usize| menu_pick.set(menu_items()[index].label.to_string()) }
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
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Standard, toggle: true, selected: fav(), selected_icon: icons::FAVORITE_FILL, aria_label: Some("Standard toggle".to_string()), onclick: move |_| fav.toggle() } span { "STANDARD · TOGGLE" } }
                            div { class: "button-example", IconButton { icon: icons::FAVORITE, variant: IconButtonVariant::Filled, toggle: true, selected: fav(), selected_icon: icons::FAVORITE_FILL, aria_label: Some("Filled toggle".to_string()), onclick: move |_| fav.toggle() } span { "FILLED · TOGGLE" } }
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
                    article { class: "demo-card icon-button-specimen",
                        div { class: "card-topline", span { "WIDTHS · 40DP" } }
                        div { class: "icon-button-row",
                            IconButton { icon: icons::ADD, width: IconButtonWidth::Narrow, aria_label: "Narrow icon button" }
                            IconButton { icon: icons::ADD, width: IconButtonWidth::Default, aria_label: "Default width icon button" }
                            IconButton { icon: icons::ADD, width: IconButtonWidth::Wide, aria_label: "Wide icon button" }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Icon button usage", code: ICON_BUTTON_USAGE }
                }
            }
            section { class: "wrap roles-section chip-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "07 — CHIPS" }
                        h2 { "Chips" }
                        p { class: "section-description", "Assist, filter, input and suggestion chips in flat and elevated styles. Filter chips show a check when selected; input chips remove on their trailing action." }
                    }
                    span { class: "token-note", "M3 · MATERIAL WEB TOKENS" }
                }
                div { class: "chip-grid",
                    article { class: "demo-card chip-specimen",
                        div { class: "card-topline", span { "ASSIST · SUGGESTION" } span { class: "component-index", "A" } }
                        div { class: "chip-row",
                            Chip { label: "Add to calendar".to_string(), variant: ChipVariant::Assist, icon: Some(icons::ADD) }
                            Chip { label: "Set reminder".to_string(), variant: ChipVariant::Assist, elevated: true }
                            Chip { label: "Yes, thanks".to_string(), variant: ChipVariant::Suggestion }
                            Chip { label: "Disabled".to_string(), variant: ChipVariant::Assist, disabled: true }
                        }
                    }
                    article { class: "demo-card chip-specimen",
                        div { class: "card-topline", span { "FILTER · INPUT" } span { class: "component-index", "B" } }
                        div { class: "chip-row",
                            Chip { label: "Vegan".to_string(), variant: ChipVariant::Filter, selected: filter_a(), onclick: move |_| filter_a.toggle() }
                            Chip { label: "Gluten-free".to_string(), variant: ChipVariant::Filter, selected: filter_b(), onclick: move |_| filter_b.toggle() }
                            Chip { label: "Elevated".to_string(), variant: ChipVariant::Filter, elevated: true, selected: filter_a(), onclick: move |_| filter_a.toggle() }
                        }
                        div { class: "chip-row",
                            if input_shown() {
                                Chip { label: "Design review".to_string(), variant: ChipVariant::Input, onremove: move |_| input_shown.set(false) }
                            } else {
                                button { class: "copy-code", onclick: move |_| input_shown.set(true), "Restore input chip" }
                            }
                        }
                    }
                    article { class: "demo-card chip-specimen",
                        div { class: "card-topline", span { "EXPRESSIVE SIZES" } }
                        div { class: "chip-row",
                            Chip { label: "Medium chip", size: ChipSize::Medium, icon: icons::ADD }
                            Chip { label: "Large chip", size: ChipSize::Large, icon: icons::ADD }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Chip usage", code: CHIP_USAGE }
                }
            }
            section { class: "wrap roles-section progress-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "08 — PROGRESS" }
                        h2 { "Progress indicators" }
                        p { class: "section-description", "Flat and wavy linear tracks, thick tracks, determinate and indeterminate. Circular indicators support flat two-half spinners and wavy sweeps." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE · MATERIAL WEB TOKENS" }
                }
                div { class: "progress-grid",
                    article { class: "demo-card progress-specimen",
                        div { class: "card-topline", span { "LINEAR · DETERMINATE" } span { class: "component-index", "A" } }
                        div { class: "progress-stack",
                            LinearProgress { value: Some(progress()), aria_label: Some("Flat progress".to_string()) }
                            LinearProgress { value: Some(progress()), wavy: true, aria_label: Some("Wavy progress".to_string()) }
                            LinearProgress { value: Some(progress()), wavy: true, thick: true, aria_label: Some("Thick wavy progress".to_string()) }
                            button { class: "copy-code", onclick: move |_| progress.set(if progress() >= 1.0 { 0.0 } else { (progress() + 0.2).min(1.0) }), "Advance · {(progress() * 100.0).round()}%" }
                        }
                    }
                    article { class: "demo-card progress-specimen",
                        div { class: "card-topline", span { "INDETERMINATE · CIRCULAR" } span { class: "component-index", "B" } }
                        div { class: "progress-stack",
                            LinearProgress { aria_label: Some("Flat indeterminate".to_string()) }
                            LinearProgress { wavy: true, aria_label: Some("Wavy indeterminate".to_string()) }
                        }
                        div { class: "progress-circles",
                            CircularProgress { value: Some(progress()), aria_label: Some("Circular determinate".to_string()) }
                            CircularProgress { aria_label: Some("Circular indeterminate".to_string()) }
                            CircularProgress { value: Some(progress()), thick: true, aria_label: Some("Circular thick".to_string()) }
                        }
                        div { class: "progress-circles",
                            CircularProgress { value: Some(progress()), wavy: true, aria_label: Some("Wavy determinate".to_string()) }
                            CircularProgress { wavy: true, aria_label: Some("Wavy indeterminate".to_string()) }
                            CircularProgress { value: Some(progress()), wavy: true, thick: true, aria_label: Some("Wavy thick".to_string()) }
                        }
                    }
                    article { class: "demo-card progress-specimen",
                        div { class: "card-topline", span { "LOADING INDICATOR" } span { class: "component-index", "C" } }
                        div { class: "progress-circles",
                            LoadingIndicator {}
                            LoadingIndicator { variant: LoadingIndicatorVariant::Contained }
                        }
                    }
                    article { class: "demo-card progress-specimen",
                        div { class: "card-topline", span { "SWITCHING MODES" } }
                        CircularProgress { value: if dynamic_progress() { None } else { Some(progress()) }, wavy: true, aria_label: "Dynamic circular" }
                        button { class: "copy-code", onclick: move |_| dynamic_progress.toggle(), "Toggle progress mode" }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Progress usage", code: PROGRESS_USAGE }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Loading indicator usage", code: LOADING_USAGE }
                }
            }
            section { class: "wrap roles-section radio-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "09 — RADIO" }
                        h2 { "Radio buttons" }
                        p { class: "section-description", "Native radio inputs, so arrow keys move and select within a group. The dot scales in on the expressive fast spatial spring." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE · MATERIAL WEB TOKENS" }
                }
                div { class: "radio-grid",
                    article { class: "demo-card radio-specimen",
                        div { class: "card-topline", span { "GROUP" } span { class: "component-index", "A" } }
                        RadioGroup { aria_label: Some("Size".to_string()), class: "radio-list",
                            for (index, label) in ["Small", "Medium", "Large"].iter().enumerate() {
                                div { class: "radio-row",
                                    Radio {
                                        name: "size-preview".to_string(),
                                        value: label.to_string(),
                                        checked: radio() == index,
                                        aria_label: Some(label.to_string()),
                                        onchange: move |_| radio.set(index),
                                    }
                                    span { class: "control-label", "{label}" }
                                }
                            }
                        }
                    }
                    article { class: "demo-card radio-specimen",
                        div { class: "card-topline", span { "STATES" } span { class: "component-index", "B" } }
                        div { class: "radio-list",
                            div { class: "radio-row",
                                Radio { name: "state-a".to_string(), value: "unselected".to_string(), aria_label: Some("Unselected".to_string()) }
                                span { class: "control-label", "Unselected" }
                            }
                            div { class: "radio-row",
                                Radio { name: "state-b".to_string(), value: "selected".to_string(), checked: true, aria_label: Some("Selected".to_string()) }
                                span { class: "control-label", "Selected" }
                            }
                            div { class: "radio-row",
                                Radio { name: "state-c".to_string(), value: "disabled".to_string(), disabled: true, aria_label: Some("Disabled".to_string()) }
                                span { class: "control-label", "Disabled" }
                            }
                            div { class: "radio-row",
                                Radio { name: "state-d".to_string(), value: "disabled-selected".to_string(), checked: true, disabled: true, aria_label: Some("Disabled selected".to_string()) }
                                span { class: "control-label", "Disabled, selected" }
                            }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Radio usage", code: RADIO_USAGE }
                }
            }
            section { class: "wrap roles-section slider-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "10 — SLIDER" }
                        h2 { "Sliders" }
                        p { class: "section-description", "Native range inputs, so arrow keys, Home, End and page keys work. The handle is 4×44dp, narrows to 2dp while pressed, and keeps an 8dp gap to the active and inactive tracks." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE · MATERIAL WEB TOKENS" }
                }
                div { class: "slider-grid",
                    article { class: "demo-card slider-specimen",
                        div { class: "card-topline", span { "CONTINUOUS · STEPPED" } span { class: "component-index", "A" } }
                        div { class: "slider-stack",
                            div { class: "slider-row",
                                span { class: "control-label", "Volume · {volume().round()}" }
                                Slider { min: 0.0, max: 100.0, value: volume(), aria_label: Some("Volume".to_string()), onchange: move |v: f64| volume.set(v) }
                            }
                            div { class: "slider-row",
                                span { class: "control-label", "Stepped · {stepped().round()}" }
                                Slider { min: 0.0, max: 100.0, step: 10.0, value: stepped(), aria_label: Some("Stepped".to_string()), onchange: move |v: f64| stepped.set(v) }
                            }
                        }
                    }
                    article { class: "demo-card slider-specimen",
                        div { class: "card-topline", span { "MAXIMUM · DISABLED" } span { class: "component-index", "B" } }
                        div { class: "slider-stack",
                            div { class: "slider-row",
                                span { class: "control-label", "At maximum" }
                                Slider { min: 0.0, max: 100.0, value: 100.0, aria_label: Some("Maximum".to_string()) }
                            }
                            div { class: "slider-row",
                                span { class: "control-label", "Disabled" }
                                Slider { min: 0.0, max: 100.0, value: 30.0, disabled: true, aria_label: Some("Disabled".to_string()) }
                            }
                        }
                    }
                    article { class: "demo-card slider-specimen",
                        div { class: "card-topline", span { "RANGE · TICKS · VALUE LABEL" } span { class: "component-index", "C" } }
                        div { class: "slider-stack",
                            div { class: "slider-row",
                                span { class: "control-label", "Price {price().0.round()} to {price().1.round()}" }
                                Slider { min: 0.0, max: 100.0, step: 10.0, ticks: true, label: true, value: price().0, value_end: Some(price().1), aria_label: Some("Price".to_string()), onchange_range: move |(a, b): (f64, f64)| price.set((a, b)) }
                            }
                            div { class: "slider-row",
                                span { class: "control-label", "Ticks · value {tick_value().round()}" }
                                Slider { min: 0.0, max: 100.0, step: 30.0, ticks: true, label: true, value: tick_value(), aria_label: Some("Ticks".to_string()), onchange: move |v: f64| tick_value.set(v) }
                            }
                        }
                    }
                    article { class: "demo-card slider-specimen slider-vertical-card",
                        div { class: "card-topline", span { "VERTICAL" } span { class: "component-index", "D" } }
                        div { class: "slider-vertical-row",
                            Slider { min: 0.0, max: 100.0, value: level(), vertical: true, label: true, aria_label: Some("Level".to_string()), onchange: move |v: f64| level.set(v) }
                            Slider { min: 0.0, max: 100.0, value: 30.0, value_end: Some(80.0), vertical: true, label: true, aria_label: Some("Vertical range".to_string()) }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Range, ticks and vertical usage", code: RANGE_USAGE }
                    article { class: "demo-card slider-specimen",
                        div { class: "card-topline", span { "EXPRESSIVE SIZES · CENTERED · ICONS" } }
                        div { class: "slider-stack",
                            Slider { value: volume(), size: SliderSize::Small, aria_label: "Small slider", onchange: move |v| volume.set(v) }
                            Slider { value: volume(), size: SliderSize::Medium, leading_icon: icons::REMOVE, trailing_icon: icons::ADD, aria_label: "Medium slider", onchange: move |v| volume.set(v) }
                            Slider { value: volume(), size: SliderSize::Large, aria_label: "Large slider", onchange: move |v| volume.set(v) }
                            Slider { value: volume(), size: SliderSize::ExtraLarge, aria_label: "Extra large slider", onchange: move |v| volume.set(v) }
                            Slider { min: -100.0, max: 100.0, value: balance(), centered: true, aria_label: "Balance", onchange: move |v| balance.set(v) }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Slider usage", code: SLIDER_USAGE }
                }
            }
            section { class: "wrap roles-section segmented-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "11 — SEGMENTED BUTTON" }
                        h2 { "Segmented buttons" }
                        p { class: "section-description", "Connected native inputs: radios for single select, so arrow keys move the selection, and checkboxes for multi select, so Space toggles each button. Selected buttons replace their leading icon with a check." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE · MATERIAL WEB TOKENS" }
                }
                div { class: "segmented-grid",
                    article { class: "demo-card segmented-specimen",
                        div { class: "card-topline", span { "SINGLE SELECT" } span { class: "component-index", "A" } }
                        div { class: "segmented-stack",
                            SegmentedButtonSet { aria_label: "Period".to_string(),
                                for (index, label) in ["Day", "Week", "Month"].into_iter().enumerate() {
                                    SegmentedButton {
                                        key: "{label}",
                                        label: label.to_string(),
                                        name: "period".to_string(),
                                        value: label.to_string(),
                                        checked: period() == index,
                                        onchange: move |checked: bool| if checked { period.set(index) },
                                    }
                                }
                            }
                            span { class: "control-label", "Selected: {[\"Day\", \"Week\", \"Month\"][period()]}" }
                        }
                    }
                    article { class: "demo-card segmented-specimen",
                        div { class: "card-topline", span { "MULTI SELECT · ICONS" } span { class: "component-index", "B" } }
                        div { class: "segmented-stack",
                            SegmentedButtonSet { aria_label: "Show".to_string(),
                                SegmentedButton {
                                    label: "Favourite".to_string(),
                                    name: "show".to_string(),
                                    value: "favourite".to_string(),
                                    icon: icons::FAVORITE,
                                    multiple: true,
                                    checked: views()[0],
                                    onchange: move |checked: bool| views.write()[0] = checked,
                                }
                                SegmentedButton {
                                    label: "Details".to_string(),
                                    name: "show".to_string(),
                                    value: "details".to_string(),
                                    multiple: true,
                                    checked: views()[1],
                                    onchange: move |checked: bool| views.write()[1] = checked,
                                }
                                SegmentedButton {
                                    label: "Add".to_string(),
                                    name: "show".to_string(),
                                    value: "add".to_string(),
                                    multiple: true,
                                    checked: views()[2],
                                    onchange: move |checked: bool| views.write()[2] = checked,
                                }
                            }
                        }
                    }
                    article { class: "demo-card segmented-specimen",
                        div { class: "card-topline", span { "DISABLED" } span { class: "component-index", "C" } }
                        div { class: "segmented-stack",
                            SegmentedButtonSet { aria_label: "Layout".to_string(),
                                SegmentedButton { label: "List".to_string(), name: "layout-off".to_string(), value: "list".to_string(), checked: true, disabled: true }
                                SegmentedButton { label: "Grid".to_string(), name: "layout-off".to_string(), value: "grid".to_string(), disabled: true }
                            }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Segmented button usage", code: SEGMENTED_USAGE }
                }
            }
            section { class: "wrap roles-section tabs-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "12 — TABS" }
                        h2 { "Tabs" }
                        p { class: "section-description", "Native buttons with role tab. Arrow keys, Home and End move focus and select the tab; only the selected tab is in the tab order. The indicator slides to the selected tab." }
                    }
                    span { class: "token-note", "M3 EXPRESSIVE · MATERIAL WEB TOKENS" }
                }
                div { class: "tabs-grid",
                    article { class: "demo-card tabs-specimen",
                        div { class: "card-topline", span { "PRIMARY" } span { class: "component-index", "A" } }
                        div { class: "tabs-stack",
                            Tabs {
                                items: vec![TabItem::new("Flights"), TabItem::new("Hotels"), TabItem::new("Cars")],
                                selected: trip(),
                                tab_ids: (0..3).map(|i| format!("trip-tab-{i}")).collect::<Vec<_>>(),
                                panel_ids: (0..3).map(|i| format!("trip-panel-{i}")).collect::<Vec<_>>(),
                                aria_label: Some("Trip type".to_string()),
                                onchange: move |index: usize| trip.set(index),
                            }
                            for (index, label) in ["Flights", "Hotels", "Cars"].into_iter().enumerate() {
                                div { class: "tabs-panel", id: "trip-panel-{index}", role: "tabpanel", aria_labelledby: "trip-tab-{index}", hidden: trip() != index, "Panel: {label}" }
                            }
                        }
                    }
                    article { class: "demo-card tabs-specimen",
                        div { class: "card-topline", span { "PRIMARY · ICONS" } span { class: "component-index", "B" } }
                        div { class: "tabs-stack",
                            Tabs {
                                items: vec![
                                    TabItem { label: "Favourites".to_string(), icon: Some(icons::FAVORITE), disabled: false },
                                    TabItem { label: "Details".to_string(), icon: Some(icons::INFO), disabled: false },
                                    TabItem { label: "Add".to_string(), icon: Some(icons::ADD), disabled: false },
                                ],
                                selected: detail(),
                                tab_ids: (0..3).map(|i| format!("detail-tab-{i}")).collect::<Vec<_>>(),
                                panel_ids: (0..3).map(|i| format!("detail-panel-{i}")).collect::<Vec<_>>(),
                                aria_label: Some("Item view".to_string()),
                                onchange: move |index: usize| detail.set(index),
                            }
                            for (index, label) in ["Favourites", "Details", "Add"].into_iter().enumerate() {
                                div { class: "tabs-panel", id: "detail-panel-{index}", role: "tabpanel", aria_labelledby: "detail-tab-{index}", hidden: detail() != index, "Panel: {label}" }
                            }
                        }
                    }
                    article { class: "demo-card tabs-specimen",
                        div { class: "card-topline", span { "SECONDARY · DISABLED TAB" } span { class: "component-index", "C" } }
                        div { class: "tabs-stack",
                            Tabs {
                                variant: TabsVariant::Secondary,
                                items: vec![
                                    TabItem::new("Overview"),
                                    TabItem::new("Specs"),
                                    TabItem { label: "Reviews".to_string(), icon: None, disabled: true },
                                ],
                                selected: spec(),
                                tab_ids: (0..3).map(|i| format!("spec-tab-{i}")).collect::<Vec<_>>(),
                                panel_ids: (0..3).map(|i| format!("spec-panel-{i}")).collect::<Vec<_>>(),
                                aria_label: Some("Product".to_string()),
                                onchange: move |index: usize| spec.set(index),
                            }
                            for (index, label) in ["Overview", "Specs", "Reviews"].into_iter().enumerate() {
                                div { class: "tabs-panel", id: "spec-panel-{index}", role: "tabpanel", aria_labelledby: "spec-tab-{index}", hidden: spec() != index, "Panel: {label}" }
                            }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Tabs usage", code: TABS_USAGE }
                }
            }
            section { class: "wrap roles-section badge-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "13 — BADGE" }
                        h2 { "Notification badges" }
                        p { class: "section-description", "A 6dp error dot, or a 16dp count pill with label-small text. Counts above 999 show as 999+. Place the badge over an icon with a BadgeAnchor." }
                    }
                    span { class: "token-note", "MATERIAL WEB TOKENS" }
                }
                div { class: "badge-grid",
                    article { class: "demo-card badge-specimen",
                        div { class: "card-topline", span { "DOT" } span { class: "component-index", "A" } }
                        div { class: "badge-row",
                            BadgeAnchor { Icon { icon: icons::FAVORITE } NotificationBadge {} }
                            BadgeAnchor { Icon { icon: icons::INFO } NotificationBadge {} }
                        }
                    }
                    article { class: "demo-card badge-specimen",
                        div { class: "card-topline", span { "COUNT" } span { class: "component-index", "B" } }
                        div { class: "badge-row",
                            BadgeAnchor { Icon { icon: icons::FAVORITE } NotificationBadge { count: 3 } }
                            BadgeAnchor { Icon { icon: icons::INFO } NotificationBadge { count: 99 } }
                            BadgeAnchor { Icon { icon: icons::ADD } NotificationBadge { count: 1000 } }
                        }
                    }
                    article { class: "demo-card badge-specimen", dir: "rtl",
                        div { class: "card-topline", span { "RTL · FIXED ANCHOR" } }
                        div { class: "badge-row badge-row--rtl",
                            BadgeAnchor { Icon { icon: icons::FAVORITE } NotificationBadge {} }
                            BadgeAnchor { Icon { icon: icons::INFO } NotificationBadge { count: 99 } }
                            BadgeAnchor { Icon { icon: icons::ADD } NotificationBadge { count: 1000 } }
                        }
                    }
                    article { class: "demo-card badge-specimen",
                        div { class: "card-topline", span { "INLINE BADGES" } }
                        div { class: "badge-row badge-row--inline",
                            NotificationBadge {}
                            NotificationBadge { count: 3 }
                            NotificationBadge { count: 99 }
                            NotificationBadge { count: 1000 }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Badge usage", code: BADGE_USAGE }
                }
            }
            section { class: "wrap roles-section card-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "14 — CARD" }
                        h2 { "Cards" }
                        p { class: "section-description", "Filled, elevated and outlined cards with 12dp corners. Interactive cards are native buttons: they respond to hover, press and keyboard focus, and disabled cards show at 38%." }
                    }
                    span { class: "token-note", "MATERIAL WEB TOKENS" }
                }
                div { class: "card-grid",
                    article { class: "demo-card card-specimen",
                        div { class: "card-topline", span { "FILLED" } span { class: "component-index", "A" } }
                        div { class: "card-stack",
                            Card { variant: CardVariant::Filled,
                                p { class: "m3-card__headline", "Filled card" }
                                p { class: "m3-card__supporting", "surface-container-highest, no elevation at rest." }
                            }
                        }
                    }
                    article { class: "demo-card card-specimen",
                        div { class: "card-topline", span { "ELEVATED · INTERACTIVE" } span { class: "component-index", "B" } }
                        div { class: "card-stack",
                            Card { variant: CardVariant::Elevated, interactive: true, onclick: move |_| card_taps += 1,
                                span { class: "m3-card__headline", "Elevated card" }
                                span { class: "m3-card__supporting", "Tapped {card_taps()} times. Hover raises it to level 2." }
                            }
                        }
                    }
                    article { class: "demo-card card-specimen",
                        div { class: "card-topline", span { "OUTLINED · INTERACTIVE" } span { class: "component-index", "C" } }
                        div { class: "card-stack",
                            Card { variant: CardVariant::Outlined, interactive: true, onclick: move |_| card_taps += 1,
                                span { class: "m3-card__headline", "Outlined card" }
                                span { class: "m3-card__supporting", "1dp outline-variant. Press to change the count." }
                            }
                        }
                    }
                    article { class: "demo-card card-specimen",
                        div { class: "card-topline", span { "DISABLED" } span { class: "component-index", "D" } }
                        div { class: "card-stack",
                            Card { variant: CardVariant::Filled, interactive: true, disabled: true,
                                span { class: "m3-card__headline", "Disabled card" }
                                span { class: "m3-card__supporting", "38% opacity; not focusable." }
                            }
                        }
                    }
                    article { class: "demo-card card-specimen card-specimen--media",
                        div { class: "card-topline", span { "WITH IMAGE · ACTIONS" } span { class: "component-index", "E" } }
                        div { class: "card-stack",
                            Card { variant: CardVariant::Elevated,
                                CardMedia { src: card_image.to_string(), alt: "A pale sky over purple hills".to_string() }
                                CardBody {
                                    span { class: "m3-card__headline", "Headline" }
                                    span { class: "m3-card__subhead", "Subhead" }
                                    span { class: "m3-card__supporting", "Explain more about the topic shown in the headline and subhead through supporting text." }
                                }
                                CardActions {
                                    Button { variant: ButtonVariant::Text, "Action" }
                                    Button { variant: ButtonVariant::Filled, "Action" }
                                }
                            }
                        }
                    }
                    article { class: "demo-card card-specimen card-specimen--media",
                        div { class: "card-topline", span { "WITH IMAGE · START ACTION" } span { class: "component-index", "F" } }
                        div { class: "card-stack",
                            Card { variant: CardVariant::Filled,
                                CardMedia { src: card_image.to_string(), alt: "A pale sky over purple hills".to_string() }
                                CardBody {
                                    span { class: "m3-card__headline", "Glass Souls' World Tour" }
                                    span { class: "m3-card__subhead", "From your recent favorites" }
                                }
                                CardActions { start: true,
                                    Button { variant: ButtonVariant::Filled, "Buy tickets" }
                                }
                            }
                        }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Card usage", code: CARD_USAGE }
                }
            }
            section { class: "wrap roles-section divider-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "15 — DIVIDER" }
                        h2 { "Dividers" }
                        p { class: "section-description", "A 1dp outline-variant line, horizontal or vertical, with an optional 16dp list inset. Dividers are role separator elements." }
                    }
                    span { class: "token-note", "MATERIAL WEB TOKENS" }
                }
                div { class: "divider-grid",
                    article { class: "demo-card divider-specimen",
                        div { class: "card-topline", span { "HORIZONTAL" } span { class: "component-index", "A" } }
                        div { class: "divider-stack",
                            span { class: "control-label", "Full width" }
                            Divider {}
                            span { class: "control-label", "Inset at start" }
                            Divider { inset: DividerInset::Start }
                            span { class: "control-label", "Inset on both sides" }
                            Divider { inset: DividerInset::Middle }
                        }
                    }
                    article { class: "demo-card divider-specimen",
                        div { class: "card-topline", span { "VERTICAL" } span { class: "component-index", "B" } }
                        div { class: "divider-row",
                            span { class: "control-label", "Left" }
                            Divider { orientation: DividerOrientation::Vertical }
                            span { class: "control-label", "Right" }
                        }
                    }
                    article { class: "demo-card divider-specimen", dir: "rtl",
                        div { class: "card-topline", span { "RIGHT TO LEFT" } }
                        div { class: "divider-stack", Divider { inset: DividerInset::Start } }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Divider usage", code: DIVIDER_USAGE }
                }
            }
            section { class: "wrap roles-section field-section",
                div { class: "section-heading roles-heading",
                    div {
                        p { class: "eyebrow", "16 — TEXT FIELD" }
                        h2 { "Text fields" }
                        p { class: "section-description", "Native inputs with a label that floats above the text when the field is focused or filled. Outlined and filled variants, with supporting text, error and disabled states." }
                    }
                    span { class: "token-note", "MATERIAL WEB TOKENS" }
                }
                div { class: "field-grid",
                    article { class: "demo-card field-specimen",
                        div { class: "card-topline", span { "OUTLINED" } span { class: "component-index", "A" } }
                        div { class: "field-stack",
                            TextField { label: "Name", value: name_value(), supporting: "Used on your receipt", oninput: move |v: String| name_value.set(v) }
                        }
                    }
                    article { class: "demo-card field-specimen",
                        div { class: "card-topline", span { "FILLED · ERROR" } span { class: "component-index", "B" } }
                        div { class: "field-stack",
                            TextField { variant: TextFieldVariant::Filled, label: "Email", value: email_value(), error: true, supporting: "Enter a valid email address", oninput: move |v: String| email_value.set(v) }
                        }
                    }
                    article { class: "demo-card field-specimen",
                        div { class: "card-topline", span { "DISABLED" } span { class: "component-index", "C" } }
                        div { class: "field-stack",
                            TextField { label: "Account number", value: "1234 5678", disabled: true, supporting: "Read only" }
                            TextField { variant: TextFieldVariant::Filled, label: "Filled disabled", disabled: true }
                        }
                    }
                    article { class: "demo-card field-specimen",
                        div { class: "card-topline", span { "ICONS · PREFIX · COUNTER · MULTI-LINE" } span { class: "component-index", "D" } }
                        div { class: "field-stack",
                            TextField { label: "Search", value: search_value(), leading_icon: icons::FAVORITE, trailing_icon: icons::CLOSE, oninput: move |v: String| search_value.set(v) }
                            TextField { label: "Price", value: price_value(), input_type: "number", prefix: "$".to_string(), suffix: "USD".to_string(), supporting: "Before tax".to_string(), oninput: move |v: String| price_value.set(v) }
                            TextField { label: "Note", value: note_value(), multiline: true, max_length: 60, supporting: "Shown to the recipient".to_string(), oninput: move |v: String| note_value.set(v) }
                        }
                    }
                    article { class: "demo-card text-field-specimen",
                        div { class: "card-topline", span { "READ ONLY · CHANGING LABEL" } }
                        TextField { label: if renamed_field() { "Longer account reference" } else { "Reference" }, value: "REF-123", read_only: true }
                        button { class: "copy-code", onclick: move |_| renamed_field.toggle(), "Change field label" }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Text field usage", code: TEXT_FIELD_USAGE }
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
