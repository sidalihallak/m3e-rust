use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;

const PAGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, ButtonVariant, Card, CardBody, FeedLayout, PageLayout, PageWidth};

#[component]
fn ProjectsPage() -> Element {
    rsx! {
        PageLayout {
            eyebrow: Some("Workspace".to_string()),
            title: "Projects".to_string(),
            description: Some("Drafts, shared folders and recent exports.".to_string()),
            width: PageWidth::Standard,
            actions: Some(rsx! {
                Button { variant: ButtonVariant::Tonal, "Filter" }
                Button { variant: ButtonVariant::Filled, "New project" }
            }),
            FeedLayout {
                for name in ["Brand kit", "Q3 report", "Onboarding"] {
                    Card { CardBody { "{name}" } }
                }
            }
        }
    }
}"#;

const SETTINGS_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Divider, DividerInset, PageLayout, PageWidth, SettingsItem, SettingsSection, Switch};

#[component]
fn NotificationSettings() -> Element {
    let mut email_updates = use_signal(|| true);
    let mut push_alerts = use_signal(|| false);
    rsx! {
        PageLayout { title: "Settings".to_string(), width: PageWidth::Narrow,
            SettingsSection {
                title: "Notifications".to_string(),
                description: Some("Choose how we reach you.".to_string()),
                SettingsItem {
                    title: "Email updates".to_string(),
                    description: Some("A weekly summary of activity".to_string()),
                    Switch {
                        checked: email_updates(),
                        aria_label: Some("Email updates".to_string()),
                        onchange: move |value| email_updates.set(value),
                    }
                }
                Divider { inset: DividerInset::Start }
                SettingsItem {
                    title: "Push alerts".to_string(),
                    description: Some("Only for mentions and approvals".to_string()),
                    Switch {
                        checked: push_alerts(),
                        aria_label: Some("Push alerts".to_string()),
                        onchange: move |value| push_alerts.set(value),
                    }
                }
            }
        }
    }
}"#;

const FORM_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, ButtonType, ButtonVariant, FormLayout, TextField, icons};

#[component]
fn SignIn() -> Element {
    let mut email = use_signal(String::new);
    rsx! {
        FormLayout {
            title: "Sign in".to_string(),
            icon: Some(icons::PERSON),
            description: Some("Use your work email to continue.".to_string()),
            actions: Some(rsx! {
                Button { variant: ButtonVariant::Text, "Create an account" }
            }),
            form {
                onsubmit: move |event| event.prevent_default(),
                TextField {
                    label: "Email".to_string(),
                    input_type: "email".to_string(),
                    value: email(),
                    oninput: move |value| email.set(value),
                }
                Button { button_type: ButtonType::Submit, variant: ButtonVariant::Filled, "Continue" }
            }
        }
    }
}"#;

const LIST_DETAIL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{IconButton, ListDetailLayout, icons};

const MESSAGES: [(&str, &str); 3] = [
    ("Design review", "Mira · 10:24"),
    ("Release notes", "Jon · 09:02"),
    ("Invoice 2041", "Finance · Mon"),
];

#[component]
fn Inbox() -> Element {
    let mut selected = use_signal(|| 0_usize);
    let mut detail_open = use_signal(|| false);
    rsx! {
        ListDetailLayout {
            show_detail: detail_open(),
            list_label: "Messages".to_string(),
            detail_label: "Message".to_string(),
            list: rsx! {
                for (index, (subject, meta)) in MESSAGES.iter().enumerate() {
                    // Style these buttons in your app; the template only lays out the panes.
                    button {
                        r#type: "button",
                        class: "my-list-item",
                        "aria-current": if selected() == index { "true" } else { "false" },
                        onclick: move |_| {
                            selected.set(index);
                            detail_open.set(true);
                        },
                        span { "{subject}" }
                        small { "{meta}" }
                    }
                }
            },
            detail: rsx! {
                IconButton {
                    icon: icons::ARROW_BACK,
                    aria_label: Some("Back to messages".to_string()),
                    onclick: move |_| detail_open.set(false),
                }
                h3 { "{MESSAGES[selected()].0}" }
                p { "{MESSAGES[selected()].1}" }
            },
        }
    }
}"#;

const SUPPORTING_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::SupportingPaneLayout;

#[component]
fn LaunchPlan() -> Element {
    rsx! {
        SupportingPaneLayout {
            supporting_label: "Comments".to_string(),
            main: rsx! {
                article {
                    h3 { "Launch plan" }
                    p { "Ship the beta to the first ten teams on Monday." }
                }
            },
            supporting: rsx! {
                h3 { "Comments" }
                ul {
                    li { "Mira: the timeline works." }
                    li { "Jon: confirm the budget first." }
                }
            },
        }
    }
}"#;

const DASHBOARD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Card, CardBody, FeedLayout, PageLayout, PageWidth};

#[component]
fn Overview() -> Element {
    rsx! {
        PageLayout {
            eyebrow: Some("Overview".to_string()),
            title: "Dashboard".to_string(),
            description: Some("Results for the last 7 days.".to_string()),
            width: PageWidth::Full,
            FeedLayout {
                Card { CardBody {
                    span { "Active users" }
                    strong { "12,480" }
                } }
                Card { CardBody {
                    span { "Revenue" }
                    strong { "$48.2k" }
                } }
            }
        }
    }
}"#;

#[component]
pub fn TemplatesGallery(active: String) -> Element {
    let mut selected = use_signal(|| 0_usize);
    let mut detail_open = use_signal(|| false);
    let mut email_updates = use_signal(|| true);
    let mut push_alerts = use_signal(|| false);
    let mut sign_in_email = use_signal(String::new);
    let mut signed_in = use_signal(|| false);
    let messages = [
        ("Design review", "Mira · 10:24"),
        ("Release notes", "Jon · 09:02"),
        ("Invoice 2041", "Finance · Mon"),
    ];

    rsx! {
        section { id: "page-layouts", class: "roles-section wrap", hidden: active != "page-layouts",
            div { class: "section-header",
                div { p { class: "eyebrow", "LAYOUTS / PAGE TEMPLATES" } h2 { "Start a page from a template" } }
                p { class: "section-description", "A page scaffold with a headline, actions and body; a settings group; and a centred single-task form. Each template is a plain component, so it takes any control in the kit." }
            }
            div { class: "template-stage",
                PageLayout {
                    eyebrow: Some("Workspace".to_string()),
                    title: "Projects".to_string(),
                    description: Some("Drafts, shared folders and recent exports.".to_string()),
                    actions: Some(rsx! {
                        Button { variant: ButtonVariant::Tonal, "Filter" }
                        Button { variant: ButtonVariant::Filled, "New project" }
                    }),
                    FeedLayout {
                        for name in ["Brand kit", "Q3 report", "Onboarding"] {
                            Card { CardBody { "{name}" } }
                        }
                    }
                }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Page template usage", code: PAGE_USAGE }

            div { class: "template-stage template-stage--padded",
                PageLayout { title: "Settings".to_string(), width: PageWidth::Narrow,
                    SettingsSection {
                        title: "Notifications".to_string(),
                        description: Some("Choose how we reach you.".to_string()),
                        SettingsItem {
                            title: "Email updates".to_string(),
                            description: Some("A weekly summary of activity".to_string()),
                            Switch {
                                checked: email_updates(),
                                aria_label: Some("Email updates".to_string()),
                                onchange: move |value| email_updates.set(value),
                            }
                        }
                        Divider { inset: DividerInset::Start }
                        SettingsItem {
                            title: "Push alerts".to_string(),
                            description: Some("Only for mentions and approvals".to_string()),
                            Switch {
                                checked: push_alerts(),
                                aria_label: Some("Push alerts".to_string()),
                                onchange: move |value| push_alerts.set(value),
                            }
                        }
                    }
                }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Settings template usage", code: SETTINGS_USAGE }

            div { class: "template-stage template-stage--padded",
                FormLayout {
                    title: "Sign in".to_string(),
                    icon: Some(icons::PERSON),
                    description: Some("Use your work email to continue.".to_string()),
                    actions: Some(rsx! {
                        Button { variant: ButtonVariant::Text, "Create an account" }
                    }),
                    form {
                        onsubmit: move |event| {
                            event.prevent_default();
                            signed_in.set(true);
                        },
                        TextField {
                            label: "Email".to_string(),
                            input_type: "email".to_string(),
                            value: sign_in_email(),
                            oninput: move |value| sign_in_email.set(value),
                        }
                        Button { button_type: ButtonType::Submit, variant: ButtonVariant::Filled, "Continue" }
                    },
                }
            }
            p { role: "status", class: "template-status",
                if signed_in() { "Signed in as {sign_in_email}" } else { "Enter an email and continue." }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Form template usage", code: FORM_USAGE }
        }

        section { id: "list-detail-layouts", class: "roles-section wrap", hidden: active != "list-detail-layouts",
            div { class: "section-header",
                div { p { class: "eyebrow", "LAYOUTS / LIST-DETAIL AND SUPPORTING PANE" } h2 { "Two panes that adapt to the window" } }
                p { class: "section-description", "Resize the window: from 840dp both panes sit side by side. Below it one pane shows at a time, and selecting a message opens its detail. The supporting pane takes a 3:7 share beside the main content and stacks below it on narrow windows." }
            }
            div { class: "template-stage",
                ListDetailLayout {
                    show_detail: detail_open(),
                    list_label: "Messages".to_string(),
                    detail_label: "Message".to_string(),
                    list: rsx! {
                        div { class: "template-list",
                            for (index, (subject, meta)) in messages.iter().enumerate() {
                                button {
                                    r#type: "button",
                                    class: "template-list-item",
                                    "aria-current": if selected() == index { "true" } else { "false" },
                                    onclick: move |_| {
                                        selected.set(index);
                                        detail_open.set(true);
                                    },
                                    span { "{subject}" }
                                    small { "{meta}" }
                                }
                            }
                        }
                    },
                    detail: rsx! {
                        div { class: "template-detail",
                            IconButton {
                                icon: icons::ARROW_BACK,
                                class: "template-back",
                                aria_label: Some("Back to messages".to_string()),
                                onclick: move |_| detail_open.set(false),
                            }
                            h3 { "{messages[selected()].0}" }
                            p { "{messages[selected()].1}" }
                        }
                    },
                }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "List-detail template usage", code: LIST_DETAIL_USAGE }

            div { class: "template-stage template-stage--padded",
                SupportingPaneLayout {
                    supporting_label: "Comments".to_string(),
                    main: rsx! {
                        article { class: "template-prose",
                            h3 { "Launch plan" }
                            p { "Ship the beta to the first ten teams on Monday." }
                            p { "The budget review follows on Wednesday, with the pricing draft." }
                        }
                    },
                    supporting: rsx! {
                        h3 { "Comments" }
                        ul { class: "template-prose",
                            li { "Mira: the timeline works." }
                            li { "Jon: confirm the budget first." }
                        }
                    },
                }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Supporting pane template usage", code: SUPPORTING_USAGE }
        }

        section { id: "feed-layouts", class: "roles-section wrap", hidden: active != "feed-layouts",
            div { class: "section-header",
                div { p { class: "eyebrow", "LAYOUTS / FEED AND DASHBOARD" } h2 { "A grid that fills the window" } }
                p { class: "section-description", "The feed places items in as many 280dp columns as fit, so one column suits a phone and several suit a desktop. Combine it with a page scaffold for a dashboard." }
            }
            div { class: "template-stage template-stage--padded",
                PageLayout {
                    eyebrow: Some("Overview".to_string()),
                    title: "Dashboard".to_string(),
                    description: Some("Results for the last 7 days.".to_string()),
                    width: PageWidth::Full,
                    actions: Some(rsx! {
                        Button { variant: ButtonVariant::Outlined, "Export" }
                    }),
                    FeedLayout {
                        Card { CardBody {
                            span { class: "template-stat-label", "Active users" }
                            strong { class: "template-stat-value", "12,480" }
                        } }
                        Card { CardBody {
                            span { class: "template-stat-label", "Revenue" }
                            strong { class: "template-stat-value", "$48.2k" }
                        } }
                        Card { variant: CardVariant::Elevated, CardBody {
                            span { class: "template-stat-label", "Open tickets" }
                            strong { class: "template-stat-value", "37" }
                        } }
                        Card { variant: CardVariant::Outlined, CardBody {
                            span { class: "template-stat-label", "Conversion" }
                            strong { class: "template-stat-value", "4.8%" }
                        } }
                    }
                }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Dashboard template usage", code: DASHBOARD_USAGE }
        }
    }
}
