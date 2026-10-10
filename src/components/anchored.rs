use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopupSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}
impl PopupSide {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopupAlign {
    Start,
    #[default]
    Center,
    End,
}
impl PopupAlign {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

// One owned evaluator/listener lifecycle per popup, including cleanup on unmount.
// anchored.js supplies collision-aware, anchor-relative transform origins.
pub(crate) fn use_popup_runtime(
    id: String,
    extra: &'static str,
    events: EventHandler<Vec<String>>,
) {
    let events = use_callback(move |message| events.call(message));
    use_effect({
        let id = id.clone();
        move || {
            let script = format!("{}\n{}", include_str!("anchored.js"), extra)
                .replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<Vec<String>>().await {
                    events.call(message);
                }
            });
        }
    });
    use_drop(move || {
        let id = id.clone();
        spawn(async move {
            let _ = document::eval(&format!("window.__m3Anchors?.[{id:?}]?.cleanup();")).await;
        });
    });
}
