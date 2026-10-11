use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppBarSize {
    #[default]
    Small,
    Medium,
    Large,
}

/// Material Expressive small and flexible medium/large app bars. The consumer
/// owns scrolling and passes `scrolled`; leading/trailing slots hold named actions.
#[component]
pub fn AppBar(
    title: String,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] size: AppBarSize,
    #[props(default)] centered: bool,
    #[props(default)] scrolled: bool,
    #[props(default)] leading: Option<Element>,
    #[props(default)] trailing: Option<Element>,
    #[props(default)] class: String,
) -> Element {
    let kind = match size {
        AppBarSize::Small => "small",
        AppBarSize::Medium => "medium",
        AppBarSize::Large => "large",
    };
    let text = rsx! {div {class:"m3-app-bar__text",h1 {"{title}"} if let Some(text)=subtitle {p {"{text}"}}}};
    rsx! {header {class:"m3-app-bar m3-app-bar--{kind} {class}","data-centered":centered,"data-scrolled":scrolled,
        div {class:"m3-app-bar__row", if let Some(leading)=leading {div {class:"m3-app-bar__leading",{leading}}}
            if size==AppBarSize::Small {{text.clone()}}else{div {class:"m3-app-bar__spacer"}}
            if let Some(trailing)=trailing {div {class:"m3-app-bar__trailing",{trailing}}}
        }
        if size!=AppBarSize::Small {div {class:"m3-app-bar__flexible",{text}}}
    }}
}
