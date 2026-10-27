//! Component stylesheets, bundled into the library so apps need no stylesheet links.
//! Order matches the cascade the preview app used. `base.css` comes first.

use dioxus::prelude::*;

use crate::theme::{ThemePreview, css_variables};

/// Every component stylesheet, in cascade order.
pub const COMPONENT_CSS: &[&str] = &[
    include_str!("../assets/base.css"),
    include_str!("../assets/sheet.css"),
    include_str!("../assets/content.css"),
    include_str!("../assets/feedback.css"),
    include_str!("../assets/list.css"),
    include_str!("../assets/search.css"),
    include_str!("../assets/date-picker.css"),
    include_str!("../assets/time-picker.css"),
    include_str!("../assets/accordion.css"),
    include_str!("../assets/table.css"),
    include_str!("../assets/pagination.css"),
    include_str!("../assets/data-table.css"),
    include_str!("../assets/chart.css"),
    include_str!("../assets/carousel.css"),
    include_str!("../assets/command.css"),
    include_str!("../assets/breadcrumb.css"),
    include_str!("../assets/navigation-extensions.css"),
    include_str!("../assets/input-extensions.css"),
    include_str!("../assets/layout-helpers.css"),
    include_str!("../assets/expression-helpers.css"),
    include_str!("../assets/theme-helpers.css"),
    include_str!("../assets/application-blocks.css"),
    include_str!("../assets/form.css"),
    include_str!("../assets/item.css"),
    include_str!("../assets/toaster.css"),
    include_str!("../assets/sidebar-provider.css"),
    include_str!("../assets/advanced.css"),
    include_str!("../assets/spinner.css"),
    include_str!("../assets/toggle.css"),
    include_str!("../assets/button.css"),
    include_str!("../assets/ripple.css"),
    include_str!("../assets/switch.css"),
    include_str!("../assets/checkbox.css"),
    include_str!("../assets/icon.css"),
    include_str!("../assets/fab.css"),
    include_str!("../assets/fab-menu.css"),
    include_str!("../assets/icon-button.css"),
    include_str!("../assets/chip.css"),
    include_str!("../assets/progress.css"),
    include_str!("../assets/loading-indicator.css"),
    include_str!("../assets/radio.css"),
    include_str!("../assets/slider.css"),
    include_str!("../assets/segmented-button.css"),
    include_str!("../assets/button-group.css"),
    include_str!("../assets/composite-motion.css"),
    include_str!("../assets/action-control.css"),
    include_str!("../assets/standard-button-group.css"),
    include_str!("../assets/split-button.css"),
    include_str!("../assets/menu.css"),
    include_str!("../assets/dialog.css"),
    include_str!("../assets/help.css"),
    include_str!("../assets/select.css"),
    include_str!("../assets/tabs.css"),
    include_str!("../assets/badge.css"),
    include_str!("../assets/card.css"),
    include_str!("../assets/divider.css"),
    include_str!("../assets/text-field.css"),
];

/// All component CSS joined into one stylesheet.
pub fn component_css() -> String {
    COMPONENT_CSS.concat()
}

/// Injects every component stylesheet and the default Material tokens into the page
/// head. Render it once near the root of the app; `ThemeProvider` renders it too.
/// Apps link no component stylesheets of their own.
#[component]
pub fn M3eStyles() -> Element {
    let css = use_hook(|| {
        let default_light = ThemePreview::from_seed(DEFAULT_SEED).light;
        format!(":root {{ {} }}\n{}", css_variables(&default_light), component_css())
    });
    rsx! {
        document::Style { "{css}" }
    }
}

/// Seed used for the root tokens. A `ThemeScope` or `ThemeProvider` overrides it.
pub const DEFAULT_SEED: u32 = 0xff6750a4;
