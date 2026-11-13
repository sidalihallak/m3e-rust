use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;

const DASHBOARD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ActivityEntry, DashboardPage, NavigationDestination, StatCard, icons};

#[component]
fn Overview() -> Element {
    let mut section = use_signal(|| "dashboard".to_string());
    rsx! {
        DashboardPage {
            app_title: "Acme".to_string(),
            title: "Overview".to_string(),
            description: Some("Results for the last 7 days.".to_string()),
            destinations: vec![
                NavigationDestination::new("dashboard", "Dashboard", icons::DASHBOARD, icons::DASHBOARD_FILL),
                NavigationDestination::new("inbox", "Inbox", icons::INBOX, icons::INBOX_FILL),
                NavigationDestination::new("settings", "Settings", icons::SETTINGS, icons::SETTINGS_FILL),
            ],
            selected: section(),
            onnavigate: move |value| section.set(value),
            stats: vec![
                StatCard::new("Active users", "12,480").change("+8% vs last week"),
                StatCard::new("Revenue", "$48.2k"),
                StatCard::new("Open tickets", "37"),
            ],
            activity: vec![
                ActivityEntry::new("New signup", "Mira joined Design", "09:12"),
                ActivityEntry::new("Invoice paid", "Order 2041", "08:40"),
            ],
            // Put your chart here, for example a `Chart` component.
            chart: rsx! { div { "Chart goes here" } },
        }
    }
}"#;

const SIGN_IN_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Button, ButtonVariant, Icon, SignInPage, icons};

#[component]
fn Login() -> Element {
    let mut error = use_signal(|| None::<String>);
    let mut busy = use_signal(|| false);
    rsx! {
        SignInPage {
            brand: Some(rsx! {
                Icon { icon: icons::DASHBOARD }
                span { "Acme" }
            }),
            error: error(),
            busy: busy(),
            footer: Some(rsx! {
                Button { variant: ButtonVariant::Text, "Create an account" }
            }),
            onsubmit: move |(_, password): (String, String)| {
                // Call your sign-in API here. Set `busy` while it runs and `error` on failure.
                busy.set(true);
                error.set(None);
                if password.len() < 8 {
                    error.set(Some("Check your email and password.".to_string()));
                }
                busy.set(false);
            },
        }
    }
}"#;

const INBOX_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{InboxPage, MailMessage, NavigationDestination, icons};

#[component]
fn Mail() -> Element {
    let mut section = use_signal(|| "inbox".to_string());
    rsx! {
        InboxPage {
            app_title: "Acme".to_string(),
            destinations: vec![
                NavigationDestination::new("dashboard", "Dashboard", icons::DASHBOARD, icons::DASHBOARD_FILL),
                NavigationDestination::new("inbox", "Inbox", icons::INBOX, icons::INBOX_FILL),
                NavigationDestination::new("settings", "Settings", icons::SETTINGS, icons::SETTINGS_FILL),
            ],
            selected: section(),
            onnavigate: move |value| section.set(value),
            messages: vec![
                MailMessage::new(
                    "1", "Mira Chen", "Design review",
                    "Can we move the review to 3pm?",
                    "Hi team, the design review moves to 3pm. Please bring the onboarding flow.",
                    "10:24",
                ).unread(),
                MailMessage::new(
                    "2", "Jon Park", "Release notes",
                    "Version 2.3 is ready for testing.",
                    "The release notes for 2.3 are attached. Testing closes on Friday.",
                    "09:02",
                ),
            ],
        }
    }
}"#;

const SETTINGS_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Divider, DividerInset, NavigationDestination, SettingsItem, SettingsPage, SettingsSection, Switch, icons};

#[component]
fn Settings() -> Element {
    let mut section = use_signal(|| "settings".to_string());
    let mut email_updates = use_signal(|| true);
    rsx! {
        SettingsPage {
            app_title: "Acme".to_string(),
            destinations: vec![
                NavigationDestination::new("dashboard", "Dashboard", icons::DASHBOARD, icons::DASHBOARD_FILL),
                NavigationDestination::new("inbox", "Inbox", icons::INBOX, icons::INBOX_FILL),
                NavigationDestination::new("settings", "Settings", icons::SETTINGS, icons::SETTINGS_FILL),
            ],
            selected: section(),
            onnavigate: move |value| section.set(value),
            user_name: "Alex Morgan".to_string(),
            user_email: "alex@example.com".to_string(),
            SettingsSection {
                title: "Notifications".to_string(),
                SettingsItem {
                    title: "Email updates".to_string(),
                    description: Some("A weekly summary".to_string()),
                    Switch {
                        checked: email_updates(),
                        aria_label: Some("Email updates".to_string()),
                        onchange: move |value| email_updates.set(value),
                    }
                }
                Divider { inset: DividerInset::Start }
                SettingsItem {
                    title: "Push alerts".to_string(),
                    description: Some("Mentions and approvals only".to_string()),
                    Switch { checked: false, aria_label: Some("Push alerts".to_string()) }
                }
            }
        }
    }
}"#;

const NOT_FOUND_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Icon, NotFoundPage, icons};

#[component]
fn Missing() -> Element {
    rsx! {
        NotFoundPage {
            brand: Some(rsx! {
                Icon { icon: icons::DASHBOARD }
                span { "Acme" }
            }),
            onaction: move |_| {
                // Navigate to your home route here, for example with your router.
            },
        }
    }
}"#;

fn destinations() -> Vec<NavigationDestination> {
    vec![
        NavigationDestination::new("dashboard", "Dashboard", icons::DASHBOARD, icons::DASHBOARD_FILL),
        NavigationDestination::new("inbox", "Inbox", icons::INBOX, icons::INBOX_FILL),
        NavigationDestination::new("settings", "Settings", icons::SETTINGS, icons::SETTINGS_FILL),
    ]
}

#[component]
pub fn PagesGallery(active: String) -> Element {
    let mut dashboard_section = use_signal(|| "dashboard".to_string());
    let mut inbox_section = use_signal(|| "inbox".to_string());
    let mut settings_section = use_signal(|| "settings".to_string());
    let mut sign_in_error = use_signal(|| None::<String>);
    let mut sign_in_busy = use_signal(|| false);
    let mut signed_in = use_signal(|| false);
    let mut not_found_action = use_signal(|| 0_u32);
    let mut email_updates = use_signal(|| true);

    rsx! {
        section { id: "dashboard-page", class: "roles-section wrap", hidden: active != "dashboard-page",
            div { class: "section-header",
                div { p { class: "eyebrow", "PAGES / DASHBOARD" } h2 { "A complete dashboard" } }
                p { class: "section-description", "Navigation, stat tiles, a chart beside recent activity. The rail shows from 600dp and the bar below it. The page below scrolls inside its frame." }
            }
            div { class: "template-frame",
                DashboardPage {
                    app_title: "Acme".to_string(),
                    title: "Overview".to_string(),
                    description: Some("Results for the last 7 days.".to_string()),
                    destinations: destinations(),
                    selected: dashboard_section(),
                    onnavigate: move |value| dashboard_section.set(value),
                    stats: vec![
                        StatCard::new("Active users", "12,480").change("+8% vs last week"),
                        StatCard::new("Revenue", "$48.2k").change("+3% vs last week"),
                        StatCard::new("Open tickets", "37"),
                        StatCard::new("Conversion", "4.8%"),
                    ],
                    activity: vec![
                        ActivityEntry::new("New signup", "Mira joined Design", "09:12"),
                        ActivityEntry::new("Invoice paid", "Order 2041", "08:40"),
                        ActivityEntry::new("Ticket closed", "Login loop on Safari", "Yesterday"),
                    ],
                    chart: rsx! {
                        div { class: "template-bars", role: "img", aria_label: "Visits by day",
                            for height in [40, 70, 55, 90, 65, 80, 50] {
                                span { style: "height:{height}%" }
                            }
                        }
                    },
                }
            }
            p { role: "status", class: "template-status", "Section: {dashboard_section}" }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Dashboard page usage", code: DASHBOARD_USAGE }
        }

        section { id: "sign-in-page", class: "roles-section wrap", hidden: active != "sign-in-page",
            div { class: "section-header",
                div { p { class: "eyebrow", "PAGES / SIGN IN" } h2 { "A sign-in screen" } }
                p { class: "section-description", "Validates the email on submit, shows a password toggle, a server error, and a busy state. Submit with a password shorter than 8 characters to see the error." }
            }
            div { class: "template-frame",
                SignInPage {
                    brand: Some(rsx! {
                        Icon { icon: icons::DASHBOARD }
                        span { "Acme" }
                    }),
                    error: sign_in_error(),
                    busy: sign_in_busy(),
                    footer: Some(rsx! {
                        Button { variant: ButtonVariant::Text, "Create an account" }
                    }),
                    onsubmit: move |(_, password): (String, String)| {
                        sign_in_busy.set(true);
                        sign_in_error.set(None);
                        if password.len() < 8 {
                            sign_in_error.set(Some("Check your email and password.".to_string()));
                        } else {
                            signed_in.set(true);
                        }
                        sign_in_busy.set(false);
                    },
                }
            }
            p { role: "status", class: "template-status",
                if signed_in() { "Signed in." } else { "Not signed in." }
            }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Sign-in page usage", code: SIGN_IN_USAGE }
        }

        section { id: "inbox-page", class: "roles-section wrap", hidden: active != "inbox-page",
            div { class: "section-header",
                div { p { class: "eyebrow", "PAGES / INBOX" } h2 { "A mail inbox" } }
                p { class: "section-description", "Search filters by sender and subject. Select a message to read it; below 840dp the message replaces the list and a back control returns to it." }
            }
            div { class: "template-frame",
                InboxPage {
                    app_title: "Acme".to_string(),
                    destinations: destinations(),
                    selected: inbox_section(),
                    onnavigate: move |value| inbox_section.set(value),
                    messages: vec![
                        MailMessage::new("1", "Mira Chen", "Design review", "Can we move the review to 3pm?", "Hi team, the design review moves to 3pm. Please bring the onboarding flow.", "10:24").unread(),
                        MailMessage::new("2", "Jon Park", "Release notes", "Version 2.3 is ready for testing.", "The release notes for 2.3 are attached. Testing closes on Friday.", "09:02"),
                        MailMessage::new("3", "Finance", "Invoice 2041", "Payment received, thank you.", "Invoice 2041 was paid on Monday. No action is needed.", "Mon"),
                    ],
                }
            }
            p { role: "status", class: "template-status", "Section: {inbox_section}" }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Inbox page usage", code: INBOX_USAGE }
        }

        section { id: "settings-page", class: "roles-section wrap", hidden: active != "settings-page",
            div { class: "section-header",
                div { p { class: "eyebrow", "PAGES / SETTINGS" } h2 { "A settings screen" } }
                p { class: "section-description", "The profile summary sits above grouped settings rows. The sections are ordinary children, so they take any switch or field." }
            }
            div { class: "template-frame",
                SettingsPage {
                    app_title: "Acme".to_string(),
                    destinations: destinations(),
                    selected: settings_section(),
                    onnavigate: move |value| settings_section.set(value),
                    user_name: "Alex Morgan".to_string(),
                    user_email: "alex@example.com".to_string(),
                    SettingsSection {
                        title: "Notifications".to_string(),
                        description: Some("Choose how we reach you.".to_string()),
                        SettingsItem {
                            title: "Email updates".to_string(),
                            description: Some("A weekly summary".to_string()),
                            Switch {
                                checked: email_updates(),
                                aria_label: Some("Email updates".to_string()),
                                onchange: move |value| email_updates.set(value),
                            }
                        }
                        Divider { inset: DividerInset::Start }
                        SettingsItem {
                            title: "Push alerts".to_string(),
                            description: Some("Mentions and approvals only".to_string()),
                            Switch { checked: false, aria_label: Some("Push alerts".to_string()) }
                        }
                    }
                }
            }
            p { role: "status", class: "template-status", "Section: {settings_section}" }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Settings page usage", code: SETTINGS_USAGE }
        }

        section { id: "not-found-page", class: "roles-section wrap", hidden: active != "not-found-page",
            div { class: "section-header",
                div { p { class: "eyebrow", "PAGES / NOT FOUND" } h2 { "A recovery screen" } }
                p { class: "section-description", "A status code, an explanation and one action that leads back to a known page." }
            }
            div { class: "template-frame",
                NotFoundPage {
                    brand: Some(rsx! {
                        Icon { icon: icons::DASHBOARD }
                        span { "Acme" }
                    }),
                    onaction: move |_| not_found_action.set(not_found_action() + 1),
                }
            }
            p { role: "status", class: "template-status", "Recovery action used {not_found_action} time(s)." }
            CodeCard { eyebrow: "COPY INTO YOUR DIOXUS APP", title: "Not found page usage", code: NOT_FOUND_USAGE }
        }
    }
}
