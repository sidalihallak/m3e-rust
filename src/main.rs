use dioxus::prelude::*;
use m3e_rust_ui::icons;
use m3e_rust_ui::{Button, ButtonShape, ButtonSize, ButtonVariant, Checkbox, Icon, Switch};

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
                    article { class: "demo-card control-specimen icon-specimen",
                        div { class: "card-topline", span { "ICON" } span { class: "component-index", "C" } }
                        div { class: "icon-grid",
                            for (label, icon) in [("add", icons::ADD), ("favorite", icons::FAVORITE), ("check", icons::CHECK), ("close", icons::CLOSE), ("info", icons::INFO), ("chevron_left", icons::CHEVRON_LEFT), ("arrow_back", icons::ARROW_BACK), ("remove", icons::REMOVE)] {
                                div { class: "icon-cell", Icon { icon, class: "icon-glyph" } span { "{label}" } }
                            }
                        }
                        p { class: "icon-note", "Official Material Symbols Rounded paths, generated from @material-symbols/svg-400@0.48.0." }
                    }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Switch usage", code: SWITCH_USAGE }
                    CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Checkbox usage", code: CHECKBOX_USAGE }
                }
            }
            footer { class: "wrap footer", span { "M3E · COMPONENT PILOT" } span { "Aligned with Material 3 Expressive guidance" } }
        }
    }
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
