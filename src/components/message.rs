use dioxus::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}
impl MessageAlign {
    pub(crate) fn class(self) -> &'static str {
        if self == Self::End { "end" } else { "start" }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BubbleVariant {
    #[default]
    Primary,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Error,
}
impl BubbleVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Muted => "muted",
            Self::Tinted => "tinted",
            Self::Outline => "outline",
            Self::Ghost => "ghost",
            Self::Error => "error",
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ReactionSide {
    Top,
    #[default]
    Bottom,
}
#[component]
pub fn Bubble(
    #[props(default)] variant: BubbleVariant,
    #[props(default)] align: MessageAlign,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-bubble {class}","data-variant":variant.class(),"data-align":align.class(),{children}}}
}
/// Static content by default; href or onclick selects native link/button semantics.
/// Interactive content must not contain other controls; put those in reactions.
#[component]
pub fn BubbleContent(
    #[props(default)] href: Option<String>,
    #[props(default)] action: bool,
    #[props(default)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    if let Some(href) = href.filter(|_| !disabled) {
        rsx! {a{class:"m3-bubble-content {class}",href,onclick,{children}}}
    } else if action || disabled {
        rsx! {button{class:"m3-bubble-content {class}",r#type:"button",disabled,onclick,{children}}}
    } else {
        rsx! {div{class:"m3-bubble-content {class}",{children}}}
    }
}
#[component]
pub fn BubbleReactions(
    #[props(default)] side: ReactionSide,
    #[props(default=MessageAlign::End)] align: MessageAlign,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-bubble-reactions {class}","data-side":if side==ReactionSide::Top{"top"}else{"bottom"},"data-align":align.class(),{children}}}
}
#[component]
pub fn BubbleGroup(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-bubble-group {class}",{children}}}
}
#[component]
pub fn Message(
    #[props(default)] align: MessageAlign,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {article{class:"m3-message {class}","data-align":align.class(),aria_label:label,{children}}}
}
#[component]
pub fn MessageGroup(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-message-group {class}",{children}}}
}
#[component]
pub fn MessageAvatar(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-message-avatar {class}",{children}}}
}
#[component]
pub fn MessageContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-message-content {class}",{children}}}
}
#[component]
pub fn MessageHeader(#[props(default)] class: String, children: Element) -> Element {
    rsx! {header{class:"m3-message-header {class}",{children}}}
}
#[component]
pub fn MessageFooter(#[props(default)] class: String, children: Element) -> Element {
    rsx! {footer{class:"m3-message-footer {class}",{children}}}
}
