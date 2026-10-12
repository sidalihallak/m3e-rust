use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const ITEM_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn DocumentItems()->Element {
 let mut saved=use_signal(||false);
 rsx!{ItemGroup {label:"Documents",
  Item {variant:ItemVariant::Outlined,
   ItemMedia {variant:ItemMediaVariant::Icon,Icon {icon:icons::INFO}}
   ItemContent {ItemTitle {"Project brief"}ItemDescription {"A shared document for your team."}}
   ItemActions {Toggle {label:"Save brief",icon:icons::FAVORITE,icon_only:true,selected:saved(),onchange:move|v|saved.set(v)}}
  }
 }}
}"#;
const ITEM_SEGMENTED_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn SelectableItems()->Element {
 let mut selected=use_signal(||None::<usize>);
 rsx!{ItemGroup {label:"Project choices",selection:ListSelection::Single,
  for (index,title) in ["Personal project","Team project","Organization"].into_iter().enumerate(){
   Item {key:"{index}",variant:ItemVariant::Segmented,label:title,selected:Some(selected()==Some(index)),onclick:move|_|selected.set(if selected()==Some(index){None}else{Some(index)}),
    ItemContent {ItemTitle {"{title}"}}
    if selected()==Some(index){ItemMedia {Icon {icon:icons::CHECK}}}
   }
  }
 }}
}"#;
const SPINNER_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn LoadingStatus()->Element {
 rsx!{div {role:"status",style:"display:flex;gap:8px;align-items:center",
  Spinner {decorative:true}
  "Loading your project…"
 }}
}"#;
#[component]
pub fn ItemGallery(active: String) -> Element {
    let mut saved = use_signal(|| false);
    let mut choice = use_signal(|| None::<usize>);
    let mut activations = use_signal(|| 0);
    let mut loading = use_signal(|| true);
    rsx! {
    section{id:"items",class:"roles-section wrap",hidden:active!="items",
     div{class:"section-header",div{p{class:"eyebrow","CONTENT / ITEM COMPOSITIONS"}h2{"Content, composed your way"}}p{class:"section-description","Flexible media, title, description, actions, header and footer slots. Native actions remain separate from static content."}}
     article{class:"item-demo",
      h3{"Documents and actions"}
      ItemGroup {label:"Documents",
       Item {variant:ItemVariant::Outlined,
        ItemHeader {ItemTitle {"Shared workspace"}span{class:"item-demo-meta","Updated today"}}
        ItemMedia {variant:ItemMediaVariant::Icon,Icon{icon:icons::INFO}}
        ItemContent {ItemTitle {"Project brief"}ItemDescription {"A shared document for your team."}}
        ItemActions {Toggle{label:"Save brief",icon:icons::FAVORITE,icon_only:true,selected:saved(),onchange:move|v|saved.set(v)}}
        ItemFooter {span{class:"item-demo-meta","PDF · 2.4 MB"}span{class:"item-demo-meta","Available offline"}}
       }
       ItemSeparator{}
       Item {href:"#spinners",label:"Open spinner examples",size:ItemSize::Small,variant:ItemVariant::Muted,
        ItemMedia {variant:ItemMediaVariant::Icon,Icon {icon:icons::ARROW_DOWNWARD}}
        ItemContent {ItemTitle {"Loading feedback"}ItemDescription {"Open the compact spinner examples."}}
       }
       Item {size:ItemSize::ExtraSmall,variant:ItemVariant::Outlined,
        ItemMedia {variant:ItemMediaVariant::Image,svg{view_box:"0 0 24 24",width:"24",height:"24",role:"img","aria-label":"Teal project tile",rect{width:"24",height:"24",fill:"#008577"}}}
        ItemContent {ItemTitle {"Compact metadata"}ItemDescription {"Extra small static content"}}
       }
      }
      p{role:"status","Brief saved: {saved}"}
     }
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Item usage",code:ITEM_USAGE}
     article{class:"item-demo",
      h3{"Segmented selection"}
      ItemGroup {label:"Project choices",selection:ListSelection::Single,for (index,title) in ["Personal project","Team project","Organization"].into_iter().enumerate(){
       Item {key:"{index}",variant:ItemVariant::Segmented,label:title,selected:Some(choice()==Some(index)),onclick:move|_|{activations+=1;choice.set(if choice()==Some(index){None}else{Some(index)});},
        ItemContent {ItemTitle {"{title}"}ItemDescription {if index==0{"For your ideas and experiments."}else if index==1{"Collaborate with people on your team."}else{"Manage projects across your organization."}}}
        if choice()==Some(index){ItemMedia {variant:ItemMediaVariant::Icon,Icon {icon:icons::CHECK}}}
       }
      }}
      ItemGroup {label:"Unavailable choices",Item {variant:ItemVariant::Segmented,interactive:true,disabled:true,label:"Unavailable project",onclick:move|_|activations+=1,ItemContent {ItemTitle {"Unavailable project"}ItemDescription {"This option cannot be activated."}}}}
      p{role:"status","Selection: {choice():?} · Activations: {activations}"}
     }
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Segmented Item usage",code:ITEM_SEGMENTED_USAGE}
    }
    section{id:"spinners",class:"roles-section wrap",hidden:active!="spinners",
     div{class:"section-header",div{p{class:"eyebrow","FEEDBACK / SPINNER"}h2{"A compact loading signal"}}p{class:"section-description","An indeterminate circular status for inline work. Use LoadingIndicator for expressive shape morphing and CircularProgress for measurable progress."}}
     article{class:"item-demo",
      div{class:"spinner-demo-row",Spinner{label:"Loading compact example"}Spinner{size:32,label:"Loading medium example"}Spinner{size:48,label:"Loading large example"}}
      div{class:"spinner-demo-row",role:"status",if loading(){Spinner{decorative:true}"Loading your project…"}else{"Project ready"}}
      Button{onclick:move|_|loading.toggle(),if loading(){"Complete example loading"}else{"Restart example loading"}}
      Button{disabled:true,Spinner{decorative:true,inherit_color:true}"Saving"}
     }
     CodeCard{eyebrow:"Copy into your Dioxus app",title:"Spinner usage",code:SPINNER_USAGE}
    }
    }
}
