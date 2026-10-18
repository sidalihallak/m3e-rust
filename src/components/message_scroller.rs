use super::motion::next_id;
use super::{IconButton, IconButtonSize, IconButtonVariant};
use crate::icons;
use dioxus::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum MessageScrollPosition {
    Start,
    #[default]
    End,
    LastAnchor,
}
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum MessageScrollDirection {
    Start,
    #[default]
    End,
}
#[derive(Clone, Default, Debug, PartialEq)]
pub struct MessageScrollerStatus {
    pub following: bool,
    pub scrollable: bool,
    pub at_start: bool,
    pub at_end: bool,
    pub visible_messages: Vec<String>,
    pub current_anchor_id: Option<String>,
}
#[derive(Clone)]
pub struct MessageScrollerHandle {
    id: String,
    status: Signal<MessageScrollerStatus>,
}
impl MessageScrollerHandle {
    pub fn status(&self) -> MessageScrollerStatus {
        (self.status)()
    }
    pub fn pause(&self) {
        self.command("pause", None);
    }
    pub fn resume(&self) {
        self.command("resume", None);
    }
    pub fn peek_to_message(&self, id: &str) {
        self.command("peek", Some(id));
    }
    pub fn scroll_to_end(&self) {
        self.command("end", None);
    }
    pub fn scroll_to_start(&self) {
        self.command("start", None);
    }
    pub fn scroll_to_message(&self, message_id: &str) {
        self.command("message", Some(message_id));
    }
    fn command(&self, command: &str, message_id: Option<&str>) {
        let script = format!(
            "window.__m3MessageScrollers?.[{:?}]?.({command:?},{:?});",
            self.id,
            message_id.unwrap_or("")
        );
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    }
}
pub fn use_message_scroller() -> MessageScrollerHandle {
    use_context()
}
/// Scoped controller and native viewport. Initial position is applied when visible.
/// Auto-follow is opt-in, pauses while reading history, and resumes at the end.
#[component]
pub fn MessageScroller(
    label: String,
    #[props(default="360px".to_string())] height: String,
    #[props(default)] auto_scroll: bool,
    #[props(default)] streaming: bool,
    #[props(default)] stream_anchor: Option<String>,
    #[props(default)] initial_position: MessageScrollPosition,
    #[props(default)] jump_direction: MessageScrollDirection,
    #[props(default = 8.0)] edge_threshold: f64,
    #[props(default = true)] preserve_prepend: bool,
    #[props(default)] scroll_margin: f64,
    #[props(default = 64.0)] previous_item_peek: f64,
    #[props(default)] controls: Option<Element>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-message-scroller"));
    let mut status = use_signal(MessageScrollerStatus::default);
    use_context_provider(|| MessageScrollerHandle {
        id: id.clone(),
        status,
    });
    use_effect({
        let id = id.clone();
        move || {
            let script =
                include_str!("message_scroller.js").replace("__ROOT_ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok((
                    following,
                    scrollable,
                    at_start,
                    at_end,
                    visible_messages,
                    current_anchor_id,
                )) = eval
                    .recv::<(bool, bool, bool, bool, Vec<String>, Option<String>)>()
                    .await
                {
                    let next = MessageScrollerStatus {
                        following,
                        scrollable,
                        at_start,
                        at_end,
                        visible_messages,
                        current_anchor_id,
                    };
                    if *status.peek() != next {
                        status.set(next);
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3MessageScrollerCleanup?.[{id:?}]?.();"));
        }
    });
    rsx! {div{id:id.clone(),class:"m3-message-scroller {class}",style:"height:{height}","data-auto-scroll":auto_scroll,"data-margin":scroll_margin.max(0.0),"data-peek":previous_item_peek.max(0.0),"data-streaming":streaming,"data-stream-anchor":stream_anchor.unwrap_or_default(),"data-initial":match initial_position{MessageScrollPosition::Start=>"start",MessageScrollPosition::End=>"end",MessageScrollPosition::LastAnchor=>"last-anchor"},"data-threshold":edge_threshold.max(0.0),"data-preserve-prepend":preserve_prepend,
     if let Some(controls)=controls{div{class:"m3-message-scroller-controls",{controls}}}
     div{class:"m3-message-scroller-viewport",role:"log",aria_label:label,aria_live:"polite",aria_relevant:"additions text",aria_atomic:"false",tabindex:0,
      div{class:"m3-message-scroller-content",{children}}
      div{class:"m3-message-scroller-spacer",aria_hidden:"true"}
     }
     div{class:"m3-message-scroller-jump","data-direction":if jump_direction==MessageScrollDirection::Start{"start"}else{"end"},"data-active":"false",aria_hidden:"true",inert:true,
      IconButton{aria_label:if jump_direction==MessageScrollDirection::Start{"Scroll to start"}else{"Scroll to end"},icon:icons::ARROW_DOWNWARD,size:IconButtonSize::Small,variant:IconButtonVariant::Outlined,onclick:{let id=id.clone();move |_|{let command=if jump_direction==MessageScrollDirection::Start{"start"}else{"end"};let script=format!("window.__m3MessageScrollers?.[{id:?}]?.({command:?});");spawn(async move{let _=document::eval(&script).await;});}}}
     }
    }}
}
#[component]
pub fn MessageScrollerItem(
    message_id: String,
    #[props(default)] scroll_anchor: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-message-scroller-item {class}","data-message-id":message_id,"data-scroll-anchor":scroll_anchor,{children}}}
}
