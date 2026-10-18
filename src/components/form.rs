use super::motion::next_id;
use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum InputVariant {
    #[default]
    Outlined,
    Filled,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum InputSize {
    #[default]
    Default,
    Small,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}
#[derive(Clone)]
struct FieldContext {
    id: String,
    invalid: Memo<bool>,
    disabled: Memo<bool>,
}

/// A composable group owning exactly one Input or Textarea. Description and
/// error slots have stable IDs; conditional slots are linked only when present.
#[component]
pub fn Field(
    #[props(default)] invalid: bool,
    #[props(default)] disabled: bool,
    #[props(default)] orientation: FieldOrientation,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id = use_hook(|| next_id("m3-form-field"));
    let invalid_signal = use_memo(use_reactive((&invalid,), |(v,)| v));
    let disabled_signal = use_memo(use_reactive((&disabled,), |(v,)| v));
    use_context_provider(|| FieldContext {
        id: id.clone(),
        invalid: invalid_signal,
        disabled: disabled_signal,
    });
    let orientation = match orientation {
        FieldOrientation::Vertical => "vertical",
        FieldOrientation::Horizontal => "horizontal",
        FieldOrientation::Responsive => "responsive",
    };
    rsx! {div{id:"{id}-root",role:"group",class:"m3-form-field {class}","data-orientation":orientation,"data-invalid":invalid,"data-disabled":disabled,{children}}}
}

#[component]
pub fn Input(
    #[props(default)] id: Option<String>,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] described_by: Option<String>,
    #[props(default)] variant: InputVariant,
    #[props(default)] size: InputSize,
    #[props(default="text".into())] input_type: String,
    #[props(default="text".into())] input_mode: String,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] name: String,
    #[props(default="off".into())] autocomplete: String,
    #[props(default)] disabled: bool,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    #[props(default)] invalid: bool,
    #[props(default)] max_length: Option<usize>,
    #[props(default)] class: String,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let field = try_use_context::<FieldContext>();
    let own_id = use_hook(|| next_id("m3-input"));
    let id = field
        .as_ref()
        .map(|f| f.id.clone())
        .or(id)
        .unwrap_or(own_id);
    let invalid = invalid || field.as_ref().is_some_and(|f| (f.invalid)());
    let disabled = disabled || field.as_ref().is_some_and(|f| (f.disabled)());
    let variant = match variant {
        InputVariant::Outlined => "outlined",
        InputVariant::Filled => "filled",
    };
    let size = if size == InputSize::Small {
        "small"
    } else {
        "default"
    };
    // Link rendered slots only. The observer also handles error slots appearing
    // after validation without requiring the caller to reconstruct descriptions.
    use_effect({
        let id = id.clone();
        let field = field.clone();
        let described_by = described_by.clone();
        move || {
            let script = format!(
                r#"const e=document.getElementById({id:?});if(!e)return;window.__m3Fields??={{}};window.__m3Fields[{id:?}]?.();const root=e.closest('.m3-form-field');const extra={extra:?};const update=()=>{{const ids=[extra,...(root?[...root.querySelectorAll('[data-field-description],[data-field-error]')].map(n=>n.id):[])].filter(Boolean);if(ids.length)e.setAttribute('aria-describedby',ids.join(' '));else e.removeAttribute('aria-describedby');}};update();const o=new MutationObserver(update);if(root)o.observe(root,{{childList:true,subtree:true}});window.__m3Fields[{id:?}]=()=>{{o.disconnect();delete window.__m3Fields[{id:?}];}};"#,
                extra = described_by.clone().unwrap_or_default()
            );
            let _ = &field;
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Fields?.[{id:?}]?.();"));
        }
    });
    rsx! {input{id,class:"m3-input m3-input--{variant} m3-input--{size} {class}",aria_label,
    r#type:input_type,inputmode:input_mode,
    value,placeholder,name,autocomplete,disabled,readonly:read_only,required,
    maxlength:max_length.map(|v|v.to_string()),"aria-invalid":invalid,oninput:move|e|oninput.call(e.value())}}
}

#[component]
pub fn Textarea(
    #[props(default)] id: Option<String>,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] described_by: Option<String>,
    #[props(default)] variant: InputVariant,
    #[props(default = 3)] rows: usize,
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] name: String,
    #[props(default="off".into())] autocomplete: String,
    #[props(default)] disabled: bool,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    #[props(default)] invalid: bool,
    #[props(default)] max_length: Option<usize>,
    #[props(default)] class: String,
    #[props(default)] oninput: EventHandler<String>,
) -> Element {
    let field = try_use_context::<FieldContext>();
    let own_id = use_hook(|| next_id("m3-input"));
    let id = field
        .as_ref()
        .map(|f| f.id.clone())
        .or(id)
        .unwrap_or(own_id);
    let invalid = invalid || field.as_ref().is_some_and(|f| (f.invalid)());
    let disabled = disabled || field.as_ref().is_some_and(|f| (f.disabled)());
    let variant = match variant {
        InputVariant::Outlined => "outlined",
        InputVariant::Filled => "filled",
    };
    let size = "textarea";
    // Link rendered slots only. The observer also handles error slots appearing
    // after validation without requiring the caller to reconstruct descriptions.
    use_effect({
        let id = id.clone();
        let field = field.clone();
        let described_by = described_by.clone();
        move || {
            let script = format!(
                r#"const e=document.getElementById({id:?});if(!e)return;window.__m3Fields??={{}};window.__m3Fields[{id:?}]?.();const root=e.closest('.m3-form-field');const extra={extra:?};const update=()=>{{const ids=[extra,...(root?[...root.querySelectorAll('[data-field-description],[data-field-error]')].map(n=>n.id):[])].filter(Boolean);if(ids.length)e.setAttribute('aria-describedby',ids.join(' '));else e.removeAttribute('aria-describedby');}};update();const o=new MutationObserver(update);if(root)o.observe(root,{{childList:true,subtree:true}});window.__m3Fields[{id:?}]=()=>{{o.disconnect();delete window.__m3Fields[{id:?}];}};"#,
                extra = described_by.clone().unwrap_or_default()
            );
            let _ = &field;
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Fields?.[{id:?}]?.();"));
        }
    });
    rsx! {textarea{id,class:"m3-input m3-input--{variant} m3-input--{size} {class}",aria_label,
    rows:rows.to_string(),
    value,placeholder,name,autocomplete,disabled,readonly:read_only,required,
    maxlength:max_length.map(|v|v.to_string()),"aria-invalid":invalid,oninput:move|e|oninput.call(e.value())}}
}

#[component]
pub fn Label(
    #[props(default)] html_for: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {label{r#for:html_for,class:"m3-label {class}","data-disabled":disabled,{children}}}
}
#[component]
pub fn FieldLabel(#[props(default)] class: String, children: Element) -> Element {
    let field = use_context::<FieldContext>();
    rsx! {Label{html_for:field.id,disabled:(field.disabled)(),class:format!("m3-field-label {class}"),{children}}}
}
#[component]
pub fn FieldDescription(#[props(default)] class: String, children: Element) -> Element {
    let field = use_context::<FieldContext>();
    rsx! {p{id:"{field.id}-description",class:"m3-field-description {class}","data-field-description":"",{children}}}
}
#[component]
pub fn FieldError(
    #[props(default)] errors: Vec<String>,
    #[props(default)] class: String,
) -> Element {
    let field = use_context::<FieldContext>();
    let mut unique = Vec::new();
    for error in errors {
        if !error.trim().is_empty() && !unique.contains(&error) {
            unique.push(error);
        }
    }
    if unique.is_empty() {
        return rsx! {};
    }
    rsx! {div{id:"{field.id}-error",role:"alert",class:"m3-field-error {class}","data-field-error":"",
    if unique.len()==1 {"{unique[0]}"}else{ul{for error in unique{li{"{error}"}}}}}}
}
#[component]
pub fn FieldSet(
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {fieldset{disabled,class:"m3-field-set {class}",{children}}}
}
#[component]
pub fn FieldLegend(
    #[props(default)] compact: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {legend{class:"m3-field-legend {class}","data-compact":compact,{children}}}
}
#[component]
pub fn FieldGroup(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-field-group {class}",{children}}}
}
#[component]
pub fn FieldContent(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-field-content {class}",{children}}}
}
#[component]
pub fn FieldTitle(#[props(default)] class: String, children: Element) -> Element {
    rsx! {div{class:"m3-field-title {class}",{children}}}
}
#[component]
pub fn FieldSeparator(
    #[props(default)] class: String,
    #[props(default)] children: Element,
) -> Element {
    rsx! {div{class:"m3-field-separator {class}",span{class:"m3-field-separator__line",aria_hidden:"true"}span{class:"m3-field-separator__text",{children}}}}
}
