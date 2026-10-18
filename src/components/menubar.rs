use super::{Menu, MenuEntry, MenuSelection, MenuVariant, motion::next_id};
use dioxus::prelude::*;
#[derive(Clone, Debug, PartialEq)]
pub struct MenubarEntry {
    pub label: String,
    pub entries: Vec<MenuEntry>,
    pub disabled: bool,
}
impl MenubarEntry {
    pub fn new(label: impl Into<String>, entries: Vec<MenuEntry>) -> Self {
        Self {
            label: label.into(),
            entries,
            disabled: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MenubarSelection {
    pub menu: usize,
    pub selection: MenuSelection,
}
/// Application action menus. Shortcut text is illustrative; apps own hotkeys.
#[component]
pub fn Menubar(
    label: String,
    entries: Vec<MenubarEntry>,
    onselect: EventHandler<MenubarSelection>,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| next_id("m3-menubar"));
    let mut open = use_signal(|| None::<usize>);
    let mut last = use_signal(|| false);
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("menubar.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<Vec<String>>().await {
                    if message.first().is_some_and(|s| s == "open") {
                        if let Some(index) = message.get(1).and_then(|s| s.parse().ok()) {
                            last.set(message.get(2).is_some_and(|s| s == "last"));
                            open.set(Some(index));
                        }
                    } else {
                        open.set(None);
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Menubars?.[{id:?}]?.();"));
        }
    });
    rsx! {div{id:"{id}",class:"m3-menubar {class}",role:"menubar",aria_label:label,"data-open-index":open().map(|i|i.to_string()),
        for (index,entry) in entries.into_iter().enumerate(){div{role:"presentation",
            button{id:"{id}-trigger-{index}",r#type:"button",role:"menuitem",class:"m3-menubar__trigger",aria_label:entry.label.clone(),aria_haspopup:"menu",aria_expanded:open()==Some(index),aria_controls:"{id}-panel-{index}",aria_disabled:entry.disabled,"data-bar-index":index,"data-label":entry.label.clone(),super::ripple::Ripple{},span{"{entry.label}"}}
            Menu{id:Some(format!("{id}-panel-{index}")),anchor_id:format!("{id}-trigger-{index}"),aria_label:Some(entry.label.clone()),entries:entry.entries,open:open()==Some(index),initial_focus_last:last(),variant:MenuVariant::Expressive,onopenchange:move |value:bool|{if !value&&open()==Some(index){open.set(None);}},onselect:move |selection|onselect.call(MenubarSelection{menu:index,selection})}
        }}
    }}
}
