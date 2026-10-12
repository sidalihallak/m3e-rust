use super::{Button, ButtonSize, ButtonType, ButtonVariant, motion::next_id};
use dioxus::prelude::*;

#[derive(Clone)]
struct GroupIds {
    input: String,
    support: String,
    error: Memo<bool>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum InputGroupAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}
impl InputGroupAlign {
    fn class(self) -> &'static str {
        match self {
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
            Self::BlockStart => "block-start",
            Self::BlockEnd => "block-end",
        }
    }
}

/// One native control with a persistent adjacent label and composable add-ons.
/// The disabled fieldset disables all descendants, including action buttons.
#[component]
pub fn InputGroup(
    label: String,
    #[props(default)] supporting: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-input-group"));
    let invalid = use_memo(use_reactive((&error,), |(error,)| error));
    let ids = GroupIds {
        input: format!("{id}-input"),
        support: format!("{id}-support"),
        error: invalid,
    };
    use_context_provider(|| ids.clone());
    use_effect({
        let id = id.clone();
        move || {
            let script = format!(
                r#"const root=document.getElementById({id:?}); if(!root||root.dataset.bound)return;root.dataset.bound='true';root.addEventListener('click',e=>{{if(e.target.closest('button,a,input,textarea'))return;root.querySelector('input,textarea')?.focus();}});"#
            );
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    rsx! {fieldset{id:id.clone(),class:"m3-input-group-field {class}",disabled,"data-error":error,"aria-labelledby":"{id}-label",
        label{id:"{id}-label",r#for:ids.input.clone(),class:"m3-input-group__label","{label}"}
        div{class:"m3-input-group",{children}}
        p{id:ids.support,class:"m3-input-group__support",role:if error{"alert"}else{"none"},if let Some(text)=supporting{"{text}"}}
    }}
}
#[component]
pub fn InputGroupAddon(
    #[props(default)] align: InputGroupAlign,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {div{class:"m3-input-group__addon {class}","data-align":align.class(),{children}}}
}
#[component]
pub fn InputGroupText(#[props(default)] class: String, children: Element) -> Element {
    rsx! {span{class:"m3-input-group__text {class}",{children}}}
}
#[component]
pub fn InputGroupButton(
    #[props(default)] disabled: bool,
    #[props(default = ButtonVariant::Text)] variant: ButtonVariant,
    #[props(default)] button_type: ButtonType,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {Button{disabled,variant,size:ButtonSize::Small,button_type,class:"m3-input-group__button",onclick,{children}}}
}
#[component]
pub fn InputGroupInput(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default="text".into())] input_type: String,
    #[props(default="text".into())] input_mode: String,
    #[props(default)] name: String,
    #[props(default="off".into())] autocomplete: String,
    #[props(default)] read_only: bool,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(default)] error: bool,
    #[props(default)] max_length: Option<usize>,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let ids = use_context::<GroupIds>();
    rsx! {input{id:ids.input,class:"m3-input-group__control",r#type:input_type,inputmode:input_mode,name,autocomplete,value,placeholder,readonly:read_only,disabled,required,maxlength:max_length.map(|v|v.to_string()),"aria-invalid":error || (ids.error)(),"aria-describedby":ids.support,oninput:move |e|oninput.call(e.value())}}
}
#[component]
pub fn InputGroupTextarea(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] name: String,
    #[props(default = 3)] rows: usize,
    #[props(default)] read_only: bool,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    #[props(default)] error: bool,
    #[props(default)] max_length: Option<usize>,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let ids = use_context::<GroupIds>();
    rsx! {textarea{id:ids.input,class:"m3-input-group__control",name,rows:rows.to_string(),value,placeholder,readonly:read_only,disabled,required,maxlength:max_length.map(|v|v.to_string()),"aria-invalid":error || (ids.error)(),"aria-describedby":ids.support,oninput:move |e|oninput.call(e.value())}}
}
