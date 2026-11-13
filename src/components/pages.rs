//! Complete page templates: full screens assembled from the kit's shell, layout and form components.
//!
//! Pages with app navigation share [`AppPage`]: a top app bar, a navigation rail from
//! 600dp, and a navigation bar below that. Standalone pages (sign-in and not found) fill
//! the viewport on their own. Styles are in `assets/pages.css`. Breakpoints follow the
//! window size classes used by the layout templates.

use dioxus::prelude::*;

use super::{
    Alert, AlertRole, AlertVariant, AppBar, AppBarSize, Avatar, AvatarSize, Button, ButtonType,
    ButtonVariant, Card, CardBody, CardVariant, EmptyState, FeedLayout, FormLayout, IconButton,
    ListDetailLayout, NavigationBar, NavigationDestination, NavigationRail, PageLayout, PageWidth,
    SupportingPaneLayout, TextField,
};
use crate::icons;

/// The app shell shared by the pages that have navigation. The rail shows from
/// 600dp; below that the navigation bar takes its place.
#[component]
pub fn AppPage(
    title: String,
    destinations: Vec<NavigationDestination>,
    selected: String,
    #[props(default)] trailing: Option<Element>,
    #[props(default)] class: String,
    onnavigate: EventHandler<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "m3-app-page {class}",
            AppBar { title, size: AppBarSize::Small, trailing }
            div { class: "m3-app-page__frame",
                aside { class: "m3-app-page__rail",
                    NavigationRail {
                        items: destinations.clone(),
                        selected: selected.clone(),
                        aria_label: "Main navigation",
                        onchange: move |value| onnavigate.call(value),
                    }
                }
                main { class: "m3-app-page__main", {children} }
            }
            div { class: "m3-app-page__bar",
                NavigationBar {
                    items: destinations,
                    selected,
                    aria_label: "Main navigation",
                    onchange: move |value| onnavigate.call(value),
                }
            }
        }
    }
}

/// A stat tile on the dashboard: a label, a value and an optional change note.
#[derive(Clone, Debug, PartialEq)]
pub struct StatCard {
    pub label: String,
    pub value: String,
    pub change: Option<String>,
}

impl StatCard {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into(), change: None }
    }

    /// Adds a change note such as "+12% vs last week".
    pub fn change(mut self, change: impl Into<String>) -> Self {
        self.change = Some(change.into());
        self
    }
}

/// One line of the dashboard's recent activity.
#[derive(Clone, Debug, PartialEq)]
pub struct ActivityEntry {
    pub title: String,
    pub detail: String,
    pub time: String,
}

impl ActivityEntry {
    pub fn new(title: impl Into<String>, detail: impl Into<String>, time: impl Into<String>) -> Self {
        Self { title: title.into(), detail: detail.into(), time: time.into() }
    }
}

/// A dashboard: stat tiles on top, a chart beside the recent activity.
///
/// `chart` is any element you supply, such as a [`Chart`](crate::Chart). The stat
/// tiles stack on phones and fill the window's columns on wide screens.
#[component]
pub fn DashboardPage(
    #[props(default = "Workspace".to_string())] app_title: String,
    #[props(default = "Overview".to_string())] title: String,
    #[props(default)] description: Option<String>,
    destinations: Vec<NavigationDestination>,
    selected: String,
    onnavigate: EventHandler<String>,
    stats: Vec<StatCard>,
    activity: Vec<ActivityEntry>,
    chart: Element,
    #[props(default)] actions: Option<Element>,
    #[props(default)] trailing: Option<Element>,
) -> Element {
    rsx! {
        AppPage { title: app_title, destinations, selected, onnavigate, trailing,
            class: "m3-dashboard",
            PageLayout { title, description, width: PageWidth::Full, actions,
                FeedLayout { class: "m3-dashboard__stats",
                    for stat in stats {
                        Card { variant: CardVariant::Filled, CardBody {
                            span { class: "m3-dashboard__stat-label", "{stat.label}" }
                            strong { class: "m3-dashboard__stat-value", "{stat.value}" }
                            if let Some(change) = &stat.change {
                                span { class: "m3-dashboard__stat-change", "{change}" }
                            }
                        } }
                    }
                }
                SupportingPaneLayout {
                    supporting_label: "Recent activity".to_string(),
                    main: rsx! {
                        Card { variant: CardVariant::Outlined, class: "m3-dashboard__chart",
                            h2 { class: "m3-dashboard__heading", "Visits" }
                            {chart}
                        }
                    },
                    supporting: rsx! {
                        h2 { class: "m3-dashboard__heading", "Recent activity" }
                        ol { class: "m3-dashboard__activity",
                            for entry in activity {
                                li {
                                    span { class: "m3-dashboard__activity-title", "{entry.title}" }
                                    span { class: "m3-dashboard__activity-detail", "{entry.detail}" }
                                    time { class: "m3-dashboard__activity-time", "{entry.time}" }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

/// A standalone sign-in screen. It validates the email on submit and reports
/// `(email, password)` through `onsubmit`. Show `error` when the server rejects the
/// credentials, and set `busy` while the request is in flight.
#[component]
pub fn SignInPage(
    #[props(default = "Sign in".to_string())] title: String,
    #[props(default = "Use your account to continue.".to_string())] description: String,
    #[props(default)] brand: Option<Element>,
    #[props(default)] error: Option<String>,
    #[props(default)] busy: bool,
    #[props(default)] footer: Option<Element>,
    onsubmit: EventHandler<(String, String)>,
) -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut show_password = use_signal(|| false);
    let mut email_error = use_signal(|| None::<String>);

    rsx! {
        div { class: "m3-sign-in",
            if let Some(brand) = brand {
                div { class: "m3-sign-in__brand", {brand} }
            }
            FormLayout { title, description, actions: footer,
                if let Some(message) = error {
                    Alert { title: "Couldn't sign in".to_string(), description: message,
                        variant: AlertVariant::Error, role: AlertRole::Alert }
                }
                form {
                    onsubmit: move |event| {
                        event.prevent_default();
                        let address = email();
                        if address.trim().is_empty() {
                            email_error.set(Some("Enter your email address.".to_string()));
                            return;
                        }
                        email_error.set(None);
                        onsubmit.call((address, password()));
                    },
                    TextField {
                        label: "Email".to_string(),
                        input_type: "email".to_string(),
                        value: email(),
                        error: email_error().is_some(),
                        supporting: email_error(),
                        oninput: move |value| email.set(value),
                    }
                    TextField {
                        label: "Password".to_string(),
                        input_type: if show_password() { "text" } else { "password" },
                        value: password(),
                        oninput: move |value| password.set(value),
                    }
                    Button {
                        variant: ButtonVariant::Text,
                        toggle: true,
                        selected: show_password(),
                        onclick: move |_| show_password.toggle(),
                        if show_password() { "Hide password" } else { "Show password" }
                    }
                    Button {
                        button_type: ButtonType::Submit,
                        variant: ButtonVariant::Filled,
                        disabled: busy,
                        if busy { "Signing in…" } else { "Sign in" }
                    }
                }
            }
        }
    }
}

/// One message in the inbox.
#[derive(Clone, Debug, PartialEq)]
pub struct MailMessage {
    pub id: String,
    pub sender: String,
    pub subject: String,
    pub preview: String,
    pub body: String,
    pub time: String,
    pub unread: bool,
}

impl MailMessage {
    pub fn new(
        id: impl Into<String>,
        sender: impl Into<String>,
        subject: impl Into<String>,
        preview: impl Into<String>,
        body: impl Into<String>,
        time: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            sender: sender.into(),
            subject: subject.into(),
            preview: preview.into(),
            body: body.into(),
            time: time.into(),
            unread: false,
        }
    }

    /// Marks the message unread (shown in bold).
    pub fn unread(mut self) -> Self {
        self.unread = true;
        self
    }
}

/// An inbox: a search field above a list of messages with a detail pane. The
/// search filters by sender and subject. Selecting a message opens it; below 840dp
/// the detail replaces the list and a back control returns to it.
#[component]
pub fn InboxPage(
    #[props(default = "Workspace".to_string())] app_title: String,
    #[props(default = "Inbox".to_string())] title: String,
    destinations: Vec<NavigationDestination>,
    selected: String,
    onnavigate: EventHandler<String>,
    messages: Vec<MailMessage>,
) -> Element {
    let mut query = use_signal(String::new);
    let mut active = use_signal(|| None::<String>);
    let mut detail_open = use_signal(|| false);

    let needle = query().trim().to_lowercase();
    let visible: Vec<MailMessage> = messages
        .iter()
        .filter(|m| {
            needle.is_empty()
                || m.subject.to_lowercase().contains(&needle)
                || m.sender.to_lowercase().contains(&needle)
        })
        .cloned()
        .collect();
    let current: Option<MailMessage> = active()
        .and_then(|id| messages.iter().find(|m| m.id == id).cloned())
        .or_else(|| visible.first().cloned());
    let current_id = current.as_ref().map(|m| m.id.clone()).unwrap_or_default();

    rsx! {
        AppPage { title: app_title, destinations, selected, onnavigate, class: "m3-inbox",
            PageLayout { title, width: PageWidth::Full,
                div { class: "m3-inbox__search",
                    TextField {
                        label: "Search mail".to_string(),
                        value: query(),
                        leading_icon: Some(icons::SEARCH),
                        oninput: move |value| query.set(value),
                    }
                }
                ListDetailLayout {
                    show_detail: detail_open(),
                    list_label: "Messages".to_string(),
                    detail_label: "Message".to_string(),
                    list: rsx! {
                        if visible.is_empty() {
                            p { class: "m3-inbox__empty", "No messages match \"{query}\"." }
                        }
                        for message in visible.clone() {
                            button {
                                r#type: "button",
                                class: "m3-mail-item",
                                "data-unread": "{message.unread}",
                                "aria-current": if message.id == current_id { "true" } else { "false" },
                                onclick: {
                                    let id = message.id.clone();
                                    move |_| {
                                        active.set(Some(id.clone()));
                                        detail_open.set(true);
                                    }
                                },
                                span { class: "m3-mail-item__top",
                                    span { class: "m3-mail-item__sender", "{message.sender}" }
                                    time { class: "m3-mail-item__time", "{message.time}" }
                                }
                                span { class: "m3-mail-item__subject", "{message.subject}" }
                                span { class: "m3-mail-item__preview", "{message.preview}" }
                            }
                        }
                    },
                    detail: rsx! {
                        if let Some(message) = current.clone() {
                            article { class: "m3-mail-detail",
                                IconButton {
                                    icon: icons::ARROW_BACK,
                                    class: "m3-mail-detail__back",
                                    aria_label: "Back to messages".to_string(),
                                    onclick: move |_| detail_open.set(false),
                                }
                                h2 { class: "m3-mail-detail__subject", "{message.subject}" }
                                p { class: "m3-mail-detail__meta", "{message.sender} · {message.time}" }
                                p { class: "m3-mail-detail__body", "{message.body}" }
                            }
                        } else {
                            p { class: "m3-mail-detail__empty", "Select a message to read it." }
                        }
                    },
                }
            }
        }
    }
}

/// A settings screen: the profile summary, then the sections you pass as children
/// (usually [`SettingsSection`](crate::SettingsSection) groups).
#[component]
pub fn SettingsPage(
    #[props(default = "Workspace".to_string())] app_title: String,
    #[props(default = "Settings".to_string())] title: String,
    #[props(default)] description: Option<String>,
    destinations: Vec<NavigationDestination>,
    selected: String,
    onnavigate: EventHandler<String>,
    user_name: String,
    user_email: String,
    children: Element,
) -> Element {
    rsx! {
        AppPage { title: app_title, destinations, selected, onnavigate, class: "m3-settings",
            PageLayout { title, description, width: PageWidth::Narrow,
                Card { variant: CardVariant::Filled, class: "m3-settings__profile",
                    Avatar { label: user_name.clone(), size: AvatarSize::Large }
                    div { class: "m3-settings__identity",
                        strong { "{user_name}" }
                        span { "{user_email}" }
                    }
                }
                {children}
            }
        }
    }
}

/// A standalone "page not found" screen with one recovery action.
#[component]
pub fn NotFoundPage(
    #[props(default = "404".to_string())] code: String,
    #[props(default = "Page not found".to_string())] title: String,
    #[props(default = "The page may have moved, or the link is wrong.".to_string())] description: String,
    #[props(default = "Go to dashboard".to_string())] action_label: String,
    #[props(default)] brand: Option<Element>,
    onaction: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "m3-status-page",
            if let Some(brand) = brand {
                div { class: "m3-sign-in__brand", {brand} }
            }
            p { class: "m3-status-page__code", "{code}" }
            EmptyState { title, description, icon: Some(icons::SEARCH_OFF),
                Button { variant: ButtonVariant::Filled, onclick: move |_| onaction.call(()), "{action_label}" }
            }
        }
    }
}
