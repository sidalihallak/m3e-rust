use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const THEME_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ThemeScope, ColorMode, ThemeContrast, Button};
#[component]
fn ScopedTheme() -> Element {
    let mut mode = use_signal(|| ColorMode::System);
    rsx! { ThemeScope { seed: 0xff006a60u32, mode: mode(), contrast: ThemeContrast::Standard,
        h3 { "A local theme" }
        Button { onclick: move |_| mode.set(ColorMode::Dark), "Use dark" }
        ThemeScope { seed: 0xff9c4146u32, mode: ColorMode::Light,
            Button { "Nested rose theme" }
        }
    } }
}"#;
const IMAGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ThemeScope, ImageThemePicker, ThemeVariant, Button};
#[component]
fn ImageTheme() -> Element {
    let mut seed = use_signal(|| 0xff6750a4u32);
    rsx! { ThemeScope { seed: seed(), variant: ThemeVariant::Content,
        ImageThemePicker { onseedchange: move |value| seed.set(value) }
        Button { "Image-colored action" }
    } }
}"#;
const PROVIDER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{ThemeProvider, ThemeSettings, ColorMode, use_theme, Button};
#[component]
fn ThemedApp() -> Element {
    rsx! { ThemeProvider { default_theme: ThemeSettings::default(), AppearanceAction {} } }
}
#[component]
fn AppearanceAction() -> Element {
    let mut theme = use_theme();
    rsx! { Button { onclick: move |_| {
        let next = if theme().mode == ColorMode::Dark { ColorMode::Light } else { ColorMode::Dark };
        theme.write().mode = next;
    }, "Toggle scoped appearance" } }
}"#;
#[component]
fn AppearanceAction() -> Element {
    let mut theme = use_theme();
    rsx! { Button { onclick: move |_| {
        let next = if theme().mode == ColorMode::Dark { ColorMode::Light } else { ColorMode::Dark };
        theme.write().mode = next;
    }, "Toggle provider appearance" }
    p { role: "status", "Provider mode: {theme().mode:?}" } }
}
#[component]
pub fn ThemeHelpersGallery(active: String) -> Element {
    let mut seed = use_signal(|| 0xff006a60u32);
    let mut image_seed = use_signal(|| 0xff6750a4u32);
    let mut mode = use_signal(|| ColorMode::System);
    let mut contrast = use_signal(|| ThemeContrast::Standard);
    rsx! {
      section{id:"theme-scopes",class:"roles-section wrap",hidden:active!="theme-scopes",
       div{class:"section-header",div{p{class:"eyebrow","FOUNDATIONS / THEME SCOPES"}h2{"Color with context"}}p{class:"section-description","Seed-generated color roles, independent nested scopes, and light, dark or system appearance. Contrast choices support different vision needs."}}
       div{class:"m3-theme-helper-grid",
        ThemeScope{seed:seed(),mode:mode(),contrast:contrast(),class:"m3-theme-demo m3-scope-live",
         h3{"Your local theme"}p{"The page theme stays independent of this preview."}
         div{class:"m3-theme-demo-controls",for (name,value) in [("Light",ColorMode::Light),("Dark",ColorMode::Dark),("System",ColorMode::System)]{Button{variant:ButtonVariant::Tonal,toggle:true,selected:mode()==value,onclick:move |_|mode.set(value),"{name}"}}}
         div{class:"m3-theme-demo-controls",for (name,value) in [("Standard contrast",ThemeContrast::Standard),("Medium contrast",ThemeContrast::Medium),("High contrast",ThemeContrast::High)]{Button{variant:ButtonVariant::Text,toggle:true,selected:contrast()==value,onclick:move |_|contrast.set(value),"{name}"}}}
         div{class:"m3-theme-demo-controls",Button{onclick:move |_|seed.set(0xff006a60u32),"Teal seed"}Button{onclick:move |_|seed.set(0xff9c4146u32),"Rose seed"}}
         p{role:"status","Chosen mode: {mode:?}. Contrast: {contrast:?}."}
         ThemeScope{seed:0xff6750a4u32,mode:ColorMode::Light,class:"m3-theme-demo m3-nested-scope",h3{"Always light violet"}Button{"Nested action"}}
        }
        ThemeScope{seed:0xff9c4146u32,mode:ColorMode::Dark,class:"m3-theme-demo m3-sibling-scope",h3{"Always dark rose"}p{"This sibling has its own complete set of color roles."}Button{"Independent action"}Button{disabled:true,"Disabled action"}}
       }
       ThemeProvider{class:"m3-theme-demo m3-provider-demo",h3{"Reactive theme context"}AppearanceAction{}}
    CodeCard{eyebrow:"Copy into your Dioxus app",title:"Theme scope usage",code:THEME_USAGE}
    CodeCard{eyebrow:"Copy into your Dioxus app",title:"Theme provider usage",code:PROVIDER_USAGE}
      }
      section{id:"image-themes",class:"roles-section wrap",hidden:active!="image-themes",
       div{class:"section-header",div{p{class:"eyebrow","FOUNDATIONS / IMAGE THEMES"}h2{"Colors from your content"}}p{class:"section-description","Choose a local image. Rust Material Color Utilities ranks its colors and generates a content scheme. Choose another swatch to change the seed."}}
       ThemeScope{seed:image_seed(),variant:ThemeVariant::Content,class:"m3-theme-demo m3-image-scope",
        ImageThemePicker{onseedchange:move |value|image_seed.set(value)}
        Button{"Image-colored action"}
        ImageThemePicker{label:"Unavailable image picker",disabled:true}
       }
       CodeCard{eyebrow:"Copy into your Dioxus app",title:"Image theme usage",code:IMAGE_USAGE}
      }
      }
}
