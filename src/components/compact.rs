use super::motion::next_id;
use dioxus::prelude::*;

/// Reactive compact breakpoint, also used to adapt dialog structure.
pub(crate) fn use_compact() -> Signal<bool> {
    let mut compact = use_signal(|| false);
    let id = use_hook(|| next_id("m3-compact"));
    use_effect({
        let id = id.clone();
        move || {
            let script = format!(
                r#"
const media=matchMedia('(max-width: 599px)');
const send=()=>dioxus.send(media.matches);
window.__m3Compact=window.__m3Compact||{{}};
media.addEventListener('change',send);
window.__m3Compact[{id:?}]=()=>{{media.removeEventListener('change',send);delete window.__m3Compact[{id:?}];}};
send();
"#
            );
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(value) = eval.recv::<bool>().await {
                    compact.set(value);
                }
            });
        }
    });
    use_drop(move || {
        spawn(async move {
            let _ = document::eval(&format!("window.__m3Compact?.[{id:?}]?.();")).await;
        });
    });
    compact
}
