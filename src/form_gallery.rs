use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const INPUT_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn EmailField()->Element {
 let mut email=use_signal(String::new);
 rsx!{Field {
  FieldLabel {"Email address"}
  Input {input_type:"email",autocomplete:"email",value:email(),oninput:move|v|email.set(v)}
  FieldDescription {"We will use this to contact you."}
 }}
}"#;
const TEXTAREA_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn NotesField()->Element {
 let mut notes=use_signal(String::new);
 rsx!{Field {FieldLabel {"Notes"}
  Textarea {variant:InputVariant::Filled,value:notes(),oninput:move|v|notes.set(v)}
  FieldDescription {"Add context for your team."}
 }}
}"#;
const LABEL_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn LabelledInput()->Element {
 let mut value=use_signal(String::new);
 rsx!{Label {html_for:"project-code","Project code"}
  Input {id:"project-code",value:value(),oninput:move|v|value.set(v)}
 }
}"#;
const FIELD_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ProfileForm()->Element {
 let mut name=use_signal(String::new);
 let mut attempted=use_signal(||false);
 let invalid=attempted() && name().trim().is_empty();
 rsx!{form {onsubmit:move|e|{e.prevent_default();attempted.set(true);},
  FieldSet {FieldLegend {"Profile"}
   FieldGroup {Field {invalid,
    FieldLabel {"Display name"}
    Input {value:name(),oninput:move|v|name.set(v)}
    if invalid {FieldError {errors:vec!["Enter a display name.".into()]}} else {FieldDescription {"How your team sees you."}}
   }}
  }
  Button {button_type:ButtonType::Submit,"Save profile"}
 }}
}"#;
const TOGGLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn FavouriteToggle()->Element {
 let mut selected=use_signal(||false);
 rsx!{Toggle {label:"Favourite",icon:icons::FAVORITE,selected:selected(),variant:ToggleVariant::Tonal,onchange:move|v|selected.set(v)}}
}"#;
const TOGGLE_GROUP_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn FormatOptions()->Element {
 let mut selected=use_signal(Vec::<String>::new);
 rsx!{ToggleGroup {label:"Text formatting",multiple:true,connected:true,
  items:vec![ToggleGroupItem::new("bold","Bold"),ToggleGroupItem::new("italic","Italic"),ToggleGroupItem::new("underline","Underline")],
  selected:selected(),onchange:move|v|selected.set(v)
 }}
}"#;
#[component]
pub fn FormGallery(active: String) -> Element {
    let mut email = use_signal(String::new);
    let mut notes = use_signal(String::new);
    let mut code = use_signal(String::new);
    let mut name = use_signal(String::new);
    let mut attempted = use_signal(|| false);
    let mut disabled = use_signal(|| false);
    let mut selected = use_signal(|| false);
    let mut format = use_signal(Vec::<String>::new);
    let mut align = use_signal(|| vec!["left".to_string()]);
    let invalid = attempted() && name().trim().is_empty();
    let mut unavailable = ToggleGroupItem::new("unavailable", "Unavailable");
    unavailable.disabled = true;
    rsx! {
    section {hidden:active!="form-fields",class:"roles-section wrap form-gallery",id:"form-fields",
     h2 {"Form fields"}
     p {class:"section-description","Standalone native controls with persistent labels and composable descriptions and validation. Use TextField for the floating-label Material pattern."}
     div {class:"form-preview-grid",
      article {class:"demo-card",h3 {"Input"}
       Field {FieldLabel {"Email address"}Input {input_type:"email",autocomplete:"email",value:email(),oninput:move|v|email.set(v)}FieldDescription {"We will use this to contact you."}}
       Field {FieldLabel {"Price"}Input {variant:InputVariant::Filled,input_type:"number",value:"25",read_only:true}FieldDescription {"Filled, read only; no native stepper."}}
       Field {disabled:true,FieldLabel {"Disabled input"}Input {value:"Unavailable"}}
       Field {FieldLabel {"Compact input"}Input {size:InputSize::Small,value:"Compact"}}
       CodeCard {eyebrow:"Copyable Rust",title:"Input · Rust usage",code:INPUT_USAGE}
      }
      article {class:"demo-card",h3 {"Textarea"}Field {FieldLabel {"Notes"}Textarea {variant:InputVariant::Filled,value:notes(),oninput:move|v|notes.set(v)}FieldDescription {"Add context for your team."}}CodeCard {eyebrow:"Copyable Rust",title:"Textarea · Rust usage",code:TEXTAREA_USAGE}}
      article {class:"demo-card",h3 {"Label"}Label {html_for:"preview-project-code","Project code"}Input {id:"preview-project-code",value:code(),oninput:move|v|code.set(v)}CodeCard {eyebrow:"Copyable Rust",title:"Label · Rust usage",code:LABEL_USAGE}}
      article {class:"demo-card",h3 {"Field compositions"}
       form {onsubmit:move|e|{e.prevent_default();attempted.set(true);},
        FieldSet {disabled:disabled(),FieldLegend {"Profile"}FieldGroup {
         Field {invalid,disabled:disabled(),orientation:FieldOrientation::Responsive,
          FieldLabel {"Display name"}FieldContent {Input {value:name(),oninput:move|v|name.set(v)}if invalid {FieldError {errors:vec!["Enter a display name.".into(),"Enter a display name.".into()]}} else {FieldDescription {"How your team sees you."}}}
         }
         FieldSeparator {"Optional"}
         Field {FieldLabel {"Company"}Input {placeholder:"Company name"}}
        }}
        Button {button_type:ButtonType::Submit,disabled:disabled(),"Save profile"}
       }
       Button {variant:ButtonVariant::Text,onclick:move|_|disabled.toggle(),if disabled(){"Enable form"}else{"Disable form"}}
       CodeCard {eyebrow:"Copyable Rust",title:"Field · Rust usage",code:FIELD_USAGE}
      }
     }
    }
    section {hidden:active!="toggles",class:"roles-section wrap form-gallery",id:"toggles",
     h2 {"Toggles"}p {class:"section-description","Controlled selection using stable values and the accepted expressive group press feedback."}
     article {class:"demo-card",h3 {"Toggle"}
      Toggle {label:"Favourite",icon:icons::FAVORITE,selected:selected(),variant:ToggleVariant::Tonal,onchange:move|v|selected.set(v)}
      Toggle {label:"Disabled toggle",disabled:true,selected:true}
      p {role:"status","Favourite: {selected}"}CodeCard {eyebrow:"Copyable Rust",title:"Toggle · Rust usage",code:TOGGLE_USAGE}
     }
     article {class:"demo-card",h3 {"Toggle group"}
      ToggleGroup {label:"Text formatting",multiple:true,connected:true,items:vec![ToggleGroupItem::new("bold","Bold"),ToggleGroupItem::new("italic","Italic"),ToggleGroupItem::new("underline","Underline")],selected:format(),onchange:move|v|format.set(v)}
      p {role:"status","Formatting: {format():?}"}
      ToggleGroup {label:"Alignment",variant:ToggleVariant::Filled,selection_required:true,items:vec![ToggleGroupItem::new("left","Left"),ToggleGroupItem::new("centre","Centre"),unavailable],selected:align(),onchange:move|v|align.set(v)}
      p {role:"status","Alignment: {align():?}"}CodeCard {eyebrow:"Copyable Rust",title:"ToggleGroup · Rust usage",code:TOGGLE_GROUP_USAGE}
     }
    }
    }
}
