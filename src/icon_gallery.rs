use super::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::{
    Button, ButtonVariant, Checkbox, Icon, MaterialSymbol, NativeSelect, SelectOption, SymbolStyle,
    TextField, icons,
};
const COMPLETE_ICON_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{icons, Icon, IconButton, MaterialSymbol, SymbolStyle};
#[component]
fn ProjectIcons()->Element {
 rsx! {
  Icon {icon:icons::rounded::DELETE,aria_label:Some("Delete".into())}
  Icon {icon:icons::outlined::ROCKET_LAUNCH_FILL,aria_label:Some("Launch".into())}
  IconButton {icon:icons::sharp::SETTINGS,aria_label:"Settings"}
  MaterialSymbol {name:"travel_explore",style:SymbolStyle::Rounded,filled:true}
 }
}
"#;
#[component]
pub fn IconCatalogue() -> Element {
    let mut query = use_signal(String::new);
    let mut style = use_signal(|| SymbolStyle::Rounded);
    let mut filled = use_signal(|| false);
    let mut page = use_signal(|| 0usize);
    let mut chosen = use_signal(|| "delete".to_string());
    let q = query().trim().to_ascii_lowercase().replace(' ', "_");
    let names = icons::SYMBOL_NAMES
        .iter()
        .copied()
        .filter(|n| n.contains(&q))
        .collect::<Vec<_>>();
    let pages = names.len().div_ceil(48).max(1);
    let current = page().min(pages - 1);
    let selected = chosen();
    let appearance = if filled() { "solid" } else { "outline" };
    let ident = format!(
        "{}{}{}",
        if selected.starts_with(|c: char| c.is_ascii_digit()) {
            "ICON_"
        } else {
            ""
        },
        selected.to_ascii_uppercase(),
        if filled() { "_FILL" } else { "" }
    );
    let code = format!(
        "use dioxus::prelude::*;\nuse m3e_rust_ui::{{icons, Icon}};\n#[component]\nfn SelectedIcon()->Element {{\n rsx! {{Icon {{icon:icons::{}::{ident}}}}}\n}}\n",
        style().module()
    );
    rsx! {article { class:"demo-card icon-specimen",
      div {class:"card-topline",span {"ICON EXAMPLES"} span {"{style().module()}"}}
      div {class:"icon-grid",for name in ["add","favorite","check","close","info","chevron_left","arrow_back","remove"] {
        div {class:"icon-cell",MaterialSymbol {name,style:style(),filled:filled(),class:"icon-glyph"}span {"{name}"}}
      }}
      p {class:"icon-note","These examples follow the style and fill controls below."}
    }
    article{class:"demo-card m3-icon-catalogue",
      TextField{label:"Search all Material Symbols",input_type:"search",value:query(),oninput:move|v|{query.set(v);page.set(0);}}
      div{class:"m3-icon-catalogue-controls",NativeSelect{label:"Icon style",value:style().module(),options:vec![SelectOption::new("rounded","Rounded"),SelectOption::new("outlined","Outlined"),SelectOption::new("sharp","Sharp")],onchange:move|v:String|style.set(match v.as_str(){"outlined"=>SymbolStyle::Outlined,"sharp"=>SymbolStyle::Sharp,_=>SymbolStyle::Rounded})}
       label{Checkbox{aria_label:Some("Filled".into()),checked:filled(),onchange:move|v|filled.set(v)}"Filled"}
      }
      div{class:"m3-icon-selected",MaterialSymbol{name:selected.clone(),style:style(),filled:filled(),aria_label:Some(format!("Selected {selected}"))}div {strong {"{selected}"}p {"{style().module()} · {appearance}"}}}
      details {class:"m3-icon-comparison-details",summary {"Compare all three styles"}
      div {class:"m3-icon-style-comparison",aria_label:format!("{selected} style comparison"),
        for variant in [SymbolStyle::Rounded,SymbolStyle::Outlined,SymbolStyle::Sharp] {
          div {class:"m3-icon-style-comparison__item","data-selected":style()==variant,
            MaterialSymbol {name:selected.clone(),style:variant,filled:filled()}
            span {"{variant.module()}"}
          }
        }
      }
      }
      p {class:"icon-note","Style changes corner shapes. Filled switches between outline and solid. Some glyphs share the same shape across styles."}
      p{role:"status","{names.len()} of {icons::SYMBOL_COUNT} symbols · Page {current+1} of {pages}"}
      div{class:"m3-icon-catalogue-grid",for name in names.into_iter().skip(current*48).take(48){button{key:"{name}",r#type:"button",class:"m3-icon-catalogue-item",aria_label:format!("Choose {name} icon"),aria_pressed:chosen()==name,onclick:move|_|chosen.set(name.into()),MaterialSymbol{name,style:style(),filled:filled()}span{"{name}"}}}}
      div{class:"m3-icon-catalogue-controls",Button{variant:ButtonVariant::Text,disabled:current==0,onclick:move|_|page.set(current-1),"Previous icons"}Button{variant:ButtonVariant::Text,disabled:current+1>=pages,onclick:move|_|page.set(current+1),"Next icons"}}

    }
    details {class:"demo-card m3-icon-artwork-check",summary {"Verify sprite against source SVG"}
      p {"Each pair shows the live sprite followed by its direct SVG path. Both must have the same shape. The Filled control applies to both."}
      div {class:"m3-icon-artwork-grid",
        div {class:"m3-icon-artwork-pair m3-icon-artwork-pair--live","data-style":style().module(),"data-glyph":"delete",
          span {"Current style / delete"}
          div {class:"m3-icon-artwork-pair__glyphs",MaterialSymbol {name:"delete",style:style(),filled:filled()}
            Icon {icon:match (style(),filled()) {
              (SymbolStyle::Rounded,false)=>icons::rounded::DELETE,
              (SymbolStyle::Rounded,true)=>icons::rounded::DELETE_FILL,
              (SymbolStyle::Outlined,false)=>icons::outlined::DELETE,
              (SymbolStyle::Outlined,true)=>icons::outlined::DELETE_FILL,
              (SymbolStyle::Sharp,false)=>icons::sharp::DELETE,
              (SymbolStyle::Sharp,true)=>icons::sharp::DELETE_FILL,
            }}
          }
        }
        for variant in [SymbolStyle::Rounded,SymbolStyle::Outlined,SymbolStyle::Sharp] {
        for name in ["delete","computer","check"] {
          div {class:"m3-icon-artwork-pair","data-style":variant.module(),"data-glyph":name,
            span {"{variant.module()} / {name}"}
            div {class:"m3-icon-artwork-pair__glyphs",
              MaterialSymbol {name,style:variant,filled:filled()}
              Icon {icon:match (variant,name,filled()) {
                (SymbolStyle::Rounded,"delete",false)=>icons::rounded::DELETE,
                (SymbolStyle::Rounded,"delete",true)=>icons::rounded::DELETE_FILL,
                (SymbolStyle::Outlined,"delete",false)=>icons::outlined::DELETE,
                (SymbolStyle::Outlined,"delete",true)=>icons::outlined::DELETE_FILL,
                (SymbolStyle::Sharp,"delete",false)=>icons::sharp::DELETE,
                (SymbolStyle::Sharp,"delete",true)=>icons::sharp::DELETE_FILL,
                (SymbolStyle::Rounded,"computer",false)=>icons::rounded::COMPUTER,
                (SymbolStyle::Rounded,"computer",true)=>icons::rounded::COMPUTER_FILL,
                (SymbolStyle::Outlined,"computer",false)=>icons::outlined::COMPUTER,
                (SymbolStyle::Outlined,"computer",true)=>icons::outlined::COMPUTER_FILL,
                (SymbolStyle::Sharp,"computer",false)=>icons::sharp::COMPUTER,
                (SymbolStyle::Sharp,"computer",true)=>icons::sharp::COMPUTER_FILL,
                (SymbolStyle::Rounded,_,false)=>icons::rounded::CHECK,
                (SymbolStyle::Rounded,_,true)=>icons::rounded::CHECK_FILL,
                (SymbolStyle::Outlined,_,false)=>icons::outlined::CHECK,
                (SymbolStyle::Outlined,_,true)=>icons::outlined::CHECK_FILL,
                (SymbolStyle::Sharp,_,false)=>icons::sharp::CHECK,
                (SymbolStyle::Sharp,_,true)=>icons::sharp::CHECK_FILL,
              }}
            }
          }
        }
      }}
    }
    CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Selected icon usage",code}
    CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Complete Material Symbols usage",code:COMPLETE_ICON_USAGE}

    }
}
