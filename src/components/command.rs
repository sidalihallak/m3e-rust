use super::{Dialog, Icon, IconData, motion::next_id};
use crate::icons;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandEntry {
    pub value: String,
    pub label: String,
    pub group: String,
    pub keywords: Vec<String>,
    pub shortcut: Option<String>,
    pub icon: Option<IconData>,
    pub disabled: bool,
    pub checked: bool,
}
impl CommandEntry {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            group: String::new(),
            keywords: vec![],
            shortcut: None,
            icon: None,
            disabled: false,
            checked: false,
        }
    }
}
/// Filterable command list. The app owns query/actions; shortcuts are display
/// hints, not registered global hotkeys. Values must be unique and stable.
#[component]
pub fn Command(
    label: String,
    entries: Vec<CommandEntry>,
    query: String,
    onquerychange: EventHandler<String>,
    onselect: EventHandler<String>,
    #[props(default = "Search commands…".to_string())] placeholder: String,
    #[props(default = "No commands found.".to_string())] empty_text: String,
    #[props(default)] input_id: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| next_id("m3-command"));
    let input_id = input_id.unwrap_or_else(|| format!("{id}-input"));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("command.js").replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(action) = eval.recv::<String>().await {
                    if action == "clear" {
                        onquerychange.call(String::new());
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Commands?.[{id:?}]?.();"));
        }
    });
    let terms: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    let filtered: Vec<_> = entries
        .into_iter()
        .filter(|entry| {
            let haystack = format!(
                "{} {} {}",
                entry.label,
                entry.group,
                entry.keywords.join(" ")
            )
            .to_lowercase();
            terms.iter().all(|term| haystack.contains(term))
        })
        .collect();
    let mut groups = Vec::new();
    for entry in &filtered {
        if !groups.contains(&entry.group) {
            groups.push(entry.group.clone());
        }
    }
    let count = filtered.len();
    rsx! {div {id:"{id}",class:"m3-command {class}","data-disabled":disabled,
        div {class:"m3-command__input-wrap",Icon {icon:icons::SEARCH}
            input {id:input_id,r#type:"text",role:"combobox",aria_label:label.clone(),aria_controls:"{id}-list",aria_expanded:"true",aria_autocomplete:"list",autocomplete:"off",spellcheck:"false",placeholder,value:query,disabled,oninput:move |e|onquerychange.call(e.value())}
        }
        div {id:"{id}-list",class:"m3-command__list",role:"listbox",tabindex:"-1",aria_label:format!("{label} results"),
            if count==0 {div {class:"m3-command__empty","{empty_text}"}}
            for (group_index,group) in groups.into_iter().enumerate(){
                div {key:"{group}",role:"group",aria_label:if group.is_empty(){None}else{Some(group.clone())},class:"m3-command__group",
                    if group_index>0 {div {class:"m3-command__separator",aria_hidden:"true"}}
                    if !group.is_empty(){div {class:"m3-command__heading","{group}"}}
                    for (index,entry) in filtered.iter().enumerate().filter(|(_,e)|e.group==group){
                        button {id:"{id}-item-{index}",key:"{entry.value}",r#type:"button",class:"m3-command__item",role:"option",aria_label:if entry.checked {format!("{} (enabled)",entry.label)}else{entry.label.clone()},aria_disabled:disabled||entry.disabled,disabled:disabled||entry.disabled,tabindex:"-1","data-value":entry.value.clone(),"data-checked":entry.checked,
                            onclick:{let value=entry.value.clone();move |_|onselect.call(value.clone())},
                            if let Some(icon)=entry.icon {Icon {icon}}
                            span {class:"m3-command__label","{entry.label}"}
                            if let Some(shortcut)=&entry.shortcut {span {class:"m3-command__shortcut",aria_hidden:"true","{shortcut}"}}
                            else if entry.checked {Icon {icon:icons::CHECK,class:"m3-command__check"}}
                        }
                    }
                }
            }
        }
        p {class:"m3-command__status",role:"status",aria_live:"polite","{count} commands"}
    }}
}
/// Material-tonal command composition. Uses native Dialog focus/inertness and
/// motion. Query persists across reopen until the consuming app resets it.
#[component]
pub fn CommandDialog(
    open: bool,
    entries: Vec<CommandEntry>,
    query: String,
    onquerychange: EventHandler<String>,
    onselect: EventHandler<String>,
    onopenchange: EventHandler<bool>,
    #[props(default = "Command palette".to_string())] title: String,
    #[props(default = "Search for an action to run.".to_string())] description: String,
    #[props(default)] disabled: bool,
) -> Element {
    let input_id = use_hook(|| next_id("m3-command-dialog-input"));
    rsx! {Dialog {title:title.clone(),description,open,class:"m3-command-dialog",initial_focus_id:Some(input_id.clone()),show_close:true,onopenchange,
        Command {label:format!("Search {title}"),entries,query,onquerychange,input_id,disabled,onselect:move |value|{onselect.call(value);onopenchange.call(false);}}
    }}
}
