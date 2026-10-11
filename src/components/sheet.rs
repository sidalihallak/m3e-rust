use super::{IconButton, motion::next_id};
use crate::icons;

const SHEET_RUNTIME: &str = include_str!("sheet.js");
use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    Bottom,
    Top,
    Start,
    #[default]
    End,
}
impl SheetSide {
    fn name(self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top => "top",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetVariant {
    #[default]
    Modal,
    Standard,
}

/// Controlled supplementary surface. Standard side sheets belong in a flex row;
/// standard bottom sheets belong in a positioned pane. Modal sheets use native
/// dialog modality. Keep children mounted and update both controlled callbacks.
#[component]
pub fn Sheet(
    open: bool,
    title: String,
    #[props(default)] side: SheetSide,
    #[props(default)] variant: SheetVariant,
    #[props(default)] description: Option<String>,
    #[props(default)] expanded: bool,
    #[props(default)] show_handle: bool,
    #[props(default)] draggable: bool,
    #[props(default)] detached: bool,
    #[props(default = 360)] width: u16,
    #[props(default)] footer: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onexpandedchange: EventHandler<bool>,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-sheet"));
    use_effect({
        let id = id.clone();
        move || {
            let script = SHEET_RUNTIME.replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(action) = eval.recv::<String>().await {
                    match action.as_str() {
                        "close" => onopenchange.call(false),
                        "expand" => onexpandedchange.call(true),
                        "collapse" => onexpandedchange.call(false),
                        _ => {}
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3Sheets?.[{id:?}]?.();")).await;
            });
        }
    });
    let modal = variant == SheetVariant::Modal;
    let bottom = side == SheetSide::Bottom;
    let width = width.clamp(256, 400);
    let described = description.as_ref().map(|_| format!("{id}-description"));
    let content = rsx! {
        if modal {div {class:"m3-sheet__scrim",aria_hidden:"true"}}
        div {class:"m3-sheet__surface",
            if bottom&&show_handle {button {r#type:"button",class:"m3-sheet__handle","data-sheet-handle":"true",
                aria_label:if expanded {"Collapse sheet"}else{"Expand sheet"},aria_expanded:expanded,
                onclick:move |_|onexpandedchange.call(!expanded),span {aria_hidden:"true"}
            }}
            header {class:"m3-sheet__header",h2 {id:"{id}-title","{title}"}
                IconButton {icon:icons::CLOSE,aria_label:format!("Close {title}"),onclick:move |_|onopenchange.call(false)}
            }
            div {class:"m3-sheet__body",
                if let Some(text)=description {p {id:"{id}-description",class:"m3-sheet__description","{text}"}}
                {children}
            }
            if let Some(footer)=footer {footer {class:"m3-sheet__footer",{footer}}}
        }
    };
    if modal {
        rsx! {dialog {id:"{id}",class:"m3-sheet {class}","data-side":side.name(),"data-modal":"true","data-open":open,"data-expanded":expanded,"data-draggable":draggable,"data-detached":detached,
            style:"--m3-sheet-width:{width}px",aria_modal:"true",aria_labelledby:"{id}-title",aria_describedby:described,{content}
        }}
    } else {
        rsx! {aside {id:"{id}",class:"m3-sheet m3-sheet--standard {class}",role:if bottom {"region"}else{"dialog"},"data-side":side.name(),"data-modal":"false","data-open":open,"data-expanded":expanded,"data-draggable":draggable,"data-detached":detached,
            style:"--m3-sheet-width:{width}px",aria_labelledby:"{id}-title",aria_describedby:described,
            inert:if !open {Some("")}else{None},aria_hidden:if !open {Some("true")}else{None},{content}
        }}
    }
}

#[component]
pub fn BottomSheet(
    open: bool,
    title: String,
    #[props(default)] variant: SheetVariant,
    #[props(default)] expanded: bool,
    #[props(default = true)] show_handle: bool,
    #[props(default = true)] draggable: bool,
    #[props(default)] description: Option<String>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    #[props(default)] onexpandedchange: EventHandler<bool>,
    children: Element,
) -> Element {
    rsx! {Sheet {open,title,variant,side:SheetSide::Bottom,expanded,show_handle,draggable,description,footer,class,onopenchange,onexpandedchange,{children}}}
}

#[component]
pub fn SideSheet(
    open: bool,
    title: String,
    #[props(default = SheetVariant::Standard)] variant: SheetVariant,
    #[props(default = SheetSide::End)] side: SheetSide,
    #[props(default)] detached: bool,
    #[props(default = 360)] width: u16,
    #[props(default)] description: Option<String>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] onopenchange: EventHandler<bool>,
    children: Element,
) -> Element {
    let side = if side == SheetSide::Start {
        SheetSide::Start
    } else {
        SheetSide::End
    };
    rsx! {Sheet {open,title,variant,side,detached,width,description,footer,class,onopenchange,{children}}}
}
