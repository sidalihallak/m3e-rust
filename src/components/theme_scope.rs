use super::motion::next_id;
use crate::theme::{ThemeContrast, ThemePreview, ThemeVariant, css_variables};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ColorMode {
    Light,
    Dark,
    #[default]
    System,
}

/// Independently themed subtree. Controlled mode/seed; follows OS while in System.
/// CSS variables inherit through descendants, including nested scopes. Native
/// dialogs/popovers remain descendants; externally portalled content must be scoped separately.
#[component]
pub fn ThemeScope(
    #[props(default = 0xff6750a4)] seed: u32,
    #[props(default)] mode: ColorMode,
    #[props(default)] variant: ThemeVariant,
    #[props(default)] contrast: ThemeContrast,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let mut system_dark = use_signal(|| false);
    let id = use_hook(|| next_id("m3-theme-scope"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("theme_scope.js").replace("__ROOT_ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(value) = eval.recv::<bool>().await {
                    system_dark.set(value);
                }
            });
        }
    });
    let root_id = id.clone();
    use_drop(move || {
        let _ = document::eval(&format!("window.__m3ThemeModes?.[{id:?}]?.();"));
    });
    let dark = match mode {
        ColorMode::Light => false,
        ColorMode::Dark => true,
        ColorMode::System => system_dark(),
    };
    let theme = ThemePreview::from_options(seed, variant, contrast);
    let style = format!(
        "{} color-scheme: {};",
        css_variables(if dark { &theme.dark } else { &theme.light }),
        if dark { "dark" } else { "light" }
    );
    rsx! {div {id:root_id,class:"m3-theme-scope {class}",style, "data-color-mode":if dark {"dark"} else {"light"}, {children}}}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ThemeSettings {
    pub seed: u32,
    pub mode: ColorMode,
    pub variant: ThemeVariant,
    pub contrast: ThemeContrast,
}
impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            seed: 0xff6750a4,
            mode: ColorMode::System,
            variant: ThemeVariant::TonalSpot,
            contrast: ThemeContrast::Standard,
        }
    }
}

/// A scoped Dioxus context for application-owned theme changes. Defaults initialize
/// once; use_theme exposes the nearest provider's reactive settings. No storage
/// or document-global mutations, so independent applications/scopes can coexist.
#[component]
pub fn ThemeProvider(
    #[props(default)] default_theme: ThemeSettings,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let settings = use_signal(|| default_theme);
    use_context_provider(|| settings);
    let value = settings();
    rsx! {
        crate::styles::M3eStyles {}
        ThemeScope {seed:value.seed,mode:value.mode,variant:value.variant,contrast:value.contrast,class,{children}}
    }
}

/// Must be called below ThemeProvider. The consumer owns persistence, if desired.
pub fn use_theme() -> Signal<ThemeSettings> {
    use_context::<Signal<ThemeSettings>>()
}
