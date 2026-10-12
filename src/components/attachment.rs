use super::{IconButton, IconButtonSize, IconButtonVariant, IconData};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum AttachmentState {
    Idle,
    Uploading,
    Processing,
    Error,
    #[default]
    Done,
}
impl AttachmentState {
    fn class(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Uploading => "uploading",
            Self::Processing => "processing",
            Self::Error => "error",
            Self::Done => "done",
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum AttachmentSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
}
impl AttachmentSize {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Small => "small",
            Self::ExtraSmall => "extra-small",
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}
#[component]
pub fn Attachment(
    #[props(default)] state: AttachmentState,
    #[props(default)] size: AttachmentSize,
    #[props(default)] orientation: AttachmentOrientation,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-attachment {class}","data-state":state.class(),"data-size":size.class(),"data-orientation":if orientation==AttachmentOrientation::Vertical{"vertical"}else{"horizontal"},aria_busy:matches!(state,AttachmentState::Uploading|AttachmentState::Processing),{children}}}
}
#[component]
pub fn AttachmentGroup(
    label: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-attachment-group {class}",role:"region",aria_label:label,tabindex:0,{children}}}
}
#[component]
pub fn AttachmentMedia(
    #[props(default)] image: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-attachment-media {class}","data-image":image,{children}}}
}
#[component]
pub fn AttachmentContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-attachment-content {class}",{children}}}
}
#[component]
pub fn AttachmentTitle(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-attachment-title {class}",{children}}}
}
#[component]
pub fn AttachmentDescription(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-attachment-description {class}",{children}}}
}
#[component]
pub fn AttachmentActions(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-attachment-actions {class}",{children}}}
}
/// Named native icon action, layered separately from the whole attachment trigger.
#[component]
pub fn AttachmentAction(
    label: String,
    #[props(default=icons::CLOSE)] icon: IconData,
    #[props(default)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {IconButton{aria_label:label,icon,size:IconButtonSize::ExtraSmall,variant:IconButtonVariant::Standard,disabled,class:"m3-attachment-action",onclick}}
}
#[component]
pub fn AttachmentTrigger(
    label: String,
    #[props(default)] href: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    if let Some(href) = href.filter(|_| !disabled) {
        rsx! {a{class:"m3-attachment-trigger",href,aria_label:label,onclick}}
    } else {
        rsx! {button{class:"m3-attachment-trigger",r#type:"button",aria_label:label,disabled,onclick}}
    }
}
