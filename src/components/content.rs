use super::{Icon, IconData};
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Small,
    #[default]
    Medium,
    Large,
}
impl AvatarSize {
    fn pixels(self) -> u16 {
        match self {
            Self::Small => 32,
            Self::Medium => 40,
            Self::Large => 56,
        }
    }
}
#[component]
pub fn Avatar(
    label: String,
    #[props(default)] src: Option<String>,
    #[props(default)] fallback: Option<String>,
    #[props(default)] size: AvatarSize,
    #[props(default)] decorative: bool,
    #[props(default)] status: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let initials = fallback.unwrap_or_else(|| {
        label
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    });
    let accessible = if let Some(status) = &status {
        format!("{label}, {status}")
    } else {
        label.clone()
    };
    let px = size.pixels();
    let badge = match size {
        AvatarSize::Small => 8,
        AvatarSize::Medium => 10,
        AvatarSize::Large => 12,
    };
    rsx! {span {class:"m3-avatar {class}",style:"--m3-avatar-size:{px}px;--m3-avatar-badge:{badge}px",role:if decorative {None}else{Some("img")},aria_label:if decorative {None}else{Some(accessible)},aria_hidden:if decorative {Some("true")}else{None},
        span {class:"m3-avatar__fallback",aria_hidden:"true","{initials}"}
        if let Some(src)=src {AvatarImage {key:"{src}",src}}
        if status.is_some(){span {class:"m3-avatar__badge",aria_hidden:"true"}}
    }}
}
#[component]
pub fn AvatarGroup(
    aria_label: String,
    #[props(default)] additional: Option<u32>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div {class:"m3-avatar-group {class}",role:"group",aria_label,{children}if let Some(count)=additional {span {class:"m3-avatar-group__count",role:"img",aria_label:"{count} more participants",span {aria_hidden:"true","+{count}"}}}}}
}
#[component]
pub fn Skeleton(
    #[props(default = "100%".to_string())] width: String,
    #[props(default = 20)] height: u16,
    #[props(default)] circle: bool,
    #[props(default)] class: String,
) -> Element {
    rsx! {span {class:"m3-skeleton {class}",aria_hidden:"true","data-circle":circle,style:"width:{width};height:{height}px"}}
}
/// Supply skeleton shapes in placeholder; the real content and busy state belong
/// to the same container, with one concise polite status announcement.
#[component]
pub fn LoadingPlaceholder(
    loading: bool,
    #[props(default = "Loading content".to_string())] label: String,
    placeholder: Element,
    children: Element,
) -> Element {
    rsx! {div {class:"m3-loading-placeholder",aria_busy:loading,if loading {span {class:"m3-content__sr",role:"status","{label}"}{placeholder}}else{{children}}}}
}
#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] media: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] children: Element,
) -> Element {
    rsx! {section {class:"m3-empty {class}",if let Some(media)=media {div {class:"m3-empty__media",{media}}}else if let Some(icon)=icon {div {class:"m3-empty__icon",Icon {icon}}}h3 {"{title}"}p {"{description}"}div {class:"m3-empty__actions",{children}}}}
}

#[component]
fn AvatarImage(src: String) -> Element {
    let mut loaded = use_signal(|| false);
    let mut failed = use_signal(|| false);
    rsx! {if !failed(){img {src,alt:"",class:if loaded(){"m3-avatar__image"}else{"m3-avatar__image m3-avatar__image--pending"},onload:move |_|loaded.set(true),onerror:move |_|failed.set(true)}}}
}
