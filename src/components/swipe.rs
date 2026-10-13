use dioxus::prelude::*;
pub(crate) fn use_swipe(id: String, events: EventHandler<i32>) {
    let events = use_callback(move |direction| events.call(direction));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("swipe.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(direction) = eval.recv::<i32>().await {
                    events.call(direction);
                }
            });
        }
    });
    use_drop(move || {
        spawn(async move {
            let _ = document::eval(&format!("window.__m3Swipes?.[{id:?}]?.();")).await;
        });
    });
}
