//! Page-level templates: canonical layouts assembled from the kit's surfaces.
//!
//! Breakpoints follow Material window size classes: compact below 600dp, medium
//! from 600dp, and expanded from 840dp. Styles are in `assets/templates.css`, and
//! every template is plain markup, so it composes with any component in the kit.

use dioxus::prelude::*;

use super::motion::next_id;
use super::{Icon, IconData};

/// Maximum content width of a [`PageLayout`]. `Full` fills the viewport.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageWidth {
    /// 640dp: forms, settings and reading columns.
    Narrow,
    /// 1040dp: the default for overview pages and feeds.
    #[default]
    Standard,
    /// No maximum width.
    Full,
}

impl PageWidth {
    const fn attr(self) -> &'static str {
        match self {
            Self::Narrow => "narrow",
            Self::Standard => "standard",
            Self::Full => "full",
        }
    }
}

/// A page scaffold: an optional eyebrow, a headline, a description and actions,
/// followed by the page body. The page is a labelled `section` whose heading is an `h1`.
///
/// Use one `PageLayout` per screen. Put the page's primary actions in `actions`.
#[component]
pub fn PageLayout(
    title: String,
    #[props(default)] eyebrow: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] actions: Option<Element>,
    #[props(default)] width: PageWidth,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-page-title"));
    rsx! {
        section { class: "m3-page {class}", "data-width": width.attr(), "aria-labelledby": "{id}",
            header { class: "m3-page__header",
                div { class: "m3-page__heading",
                    if let Some(eyebrow) = eyebrow {
                        p { class: "m3-page__eyebrow", "{eyebrow}" }
                    }
                    h1 { class: "m3-page__title", id: "{id}", "{title}" }
                    if let Some(description) = description {
                        p { class: "m3-page__description", "{description}" }
                    }
                }
                if let Some(actions) = actions {
                    div { class: "m3-page__actions", {actions} }
                }
            }
            div { class: "m3-page__body", {children} }
        }
    }
}

/// A list pane and a detail pane. From the expanded class both panes are visible
/// side by side (a 360dp list beside the detail). Below it only one pane shows:
/// the list while `show_detail` is false, and the detail once it is true.
///
/// Set `show_detail` when the person picks an item, and clear it from a back
/// control in the detail. The hidden pane stays in the DOM, so its state survives.
#[component]
pub fn ListDetailLayout(
    list: Element,
    detail: Element,
    #[props(default)] show_detail: bool,
    #[props(default = "List".to_string())] list_label: String,
    #[props(default = "Detail".to_string())] detail_label: String,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div { class: "m3-list-detail {class}", "data-detail": "{show_detail}",
            section { class: "m3-list-detail__list", "aria-label": "{list_label}", {list} }
            section { class: "m3-list-detail__detail", "aria-label": "{detail_label}", {detail} }
        }
    }
}

/// A main content column with a supporting pane. From the expanded class the
/// supporting pane sits at the end in a 7:3 split. Below it, the pane stacks under
/// the main content.
#[component]
pub fn SupportingPaneLayout(
    main: Element,
    supporting: Element,
    #[props(default = "Supporting content".to_string())] supporting_label: String,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div { class: "m3-supporting {class}",
            div { class: "m3-supporting__main", {main} }
            aside { class: "m3-supporting__pane", "aria-label": "{supporting_label}", {supporting} }
        }
    }
}

/// A responsive grid for cards and other feed items. It fills each row with as many
/// columns as fit at least 280dp wide, so one column shows on a phone and four or
/// more on a wide window.
#[component]
pub fn FeedLayout(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "m3-feed {class}", {children} }
    }
}

/// A focused, single-task page: an optional icon, a headline, a description, the
/// form body and an action row. The panel is centred and at most 480dp wide, for
/// sign-in, onboarding and confirmation tasks.
///
/// Put a `<form>` with the fields and the submit button in the children. Use `actions`
/// for secondary links, such as "Create an account", below the form.
#[component]
pub fn FormLayout(
    title: String,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] description: Option<String>,
    #[props(default)] actions: Option<Element>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-form-title"));
    rsx! {
        div { class: "m3-form {class}",
            section { class: "m3-form__panel", "aria-labelledby": "{id}",
                header { class: "m3-form__header",
                    if let Some(icon) = icon {
                        span { class: "m3-form__icon", Icon { icon } }
                    }
                    h1 { class: "m3-form__title", id: "{id}", "{title}" }
                    if let Some(description) = description {
                        p { class: "m3-form__description", "{description}" }
                    }
                }
                div { class: "m3-form__body", {children} }
                if let Some(actions) = actions {
                    div { class: "m3-form__actions", {actions} }
                }
            }
        }
    }
}

/// A titled group of settings rows inside a tonal container. Put rows (switches,
/// list items, or fields) in the children, and [`Divider`](crate::Divider)s between them.
#[component]
pub fn SettingsSection(
    title: String,
    #[props(default)] description: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-settings-title"));
    rsx! {
        section { class: "m3-settings-section {class}", "aria-labelledby": "{id}",
            div { class: "m3-settings-section__heading",
                h2 { id: "{id}", "{title}" }
                if let Some(description) = description {
                    p { class: "m3-settings-section__description", "{description}" }
                }
            }
            div { class: "m3-settings-section__group", {children} }
        }
    }
}

/// One row of a [`SettingsSection`]: a title and optional description on the start
/// side, and the control (a switch, a button or a value) on the end side. The row is
/// at least 56dp high, so it is a comfortable touch target.
#[component]
pub fn SettingsItem(
    title: String,
    #[props(default)] description: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "m3-settings-item {class}",
            div { class: "m3-settings-item__text",
                span { class: "m3-settings-item__title", "{title}" }
                if let Some(description) = description {
                    span { class: "m3-settings-item__description", "{description}" }
                }
            }
            div { class: "m3-settings-item__control", {children} }
        }
    }
}
