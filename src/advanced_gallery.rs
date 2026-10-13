use super::CodeCard;
use crate::icons;
use dioxus::prelude::*;
use m3e_rust_ui::*;
const SIDEBAR_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
use m3e_rust_ui::icons;
#[component]
fn Workspace()->Element {
 rsx!{SidebarProvider {persistence_key:"my-app-sidebar".to_string(),
  WorkspaceBody {}
 }}
}
#[component]
fn WorkspaceBody()->Element {
 let sidebar=use_sidebar();
 let mut current=use_signal(||"Home".to_string());
 rsx!{SidebarToggle {} SidebarLayout {
  SidebarPanel {SidebarHeader {h3 {"Workspace"}}
   SidebarContent {SidebarMenu {SidebarMenuItem {
    SidebarMenuButton {label:"Home",icon:icons::HOME,active_icon:Some(icons::HOME_FILL),active:current()=="Home",
     onclick:move |_|{current.set("Home".into());sidebar.set_mobile_open(false);}}
   }}}
  }
  SidebarInset {h2 {"{current}"}}
 }}
}
"#;
#[component]
fn SidebarDemo() -> Element {
    let state = use_sidebar();
    let mut current = use_signal(|| "Home".to_string());
    let mut side = use_signal(|| SidebarSide::Start);
    let mut variant = use_signal(|| SidebarVariant::Standard);
    let mut collapse = use_signal(|| SidebarCollapse::Icon);
    let mut filter = use_signal(String::new);
    let mut actions = use_signal(|| 0u32);
    let mut submenu = use_signal(|| false);
    rsx! {div{class:"m3-advanced-controls",
     SidebarToggle{}
     Button{variant:ButtonVariant::Outlined,onclick:move |_|side.set(if side()==SidebarSide::Start{SidebarSide::End}else{SidebarSide::Start}),"Change sidebar side"}
     Button{variant:ButtonVariant::Outlined,onclick:move |_|variant.set(match variant(){SidebarVariant::Standard=>SidebarVariant::Floating,SidebarVariant::Floating=>SidebarVariant::Inset,SidebarVariant::Inset=>SidebarVariant::Standard}),"Change sidebar surface"}
     Button{variant:ButtonVariant::Outlined,onclick:move |_|collapse.set(if collapse()==SidebarCollapse::Icon{SidebarCollapse::Offcanvas}else{SidebarCollapse::Icon}),"Toggle offcanvas behavior"}
    }
    SidebarLayout{
     SidebarPanel{side:side(),variant:variant(),collapsible:collapse(),label:"Pilot workspace",
      SidebarHeader{h3{"Pilot workspace"}}
      SidebarInput{label:"Filter destinations",value:filter(),oninput:move |v|filter.set(v)}
      SidebarContent{SidebarMenu{
       for (name,icon) in [("Home",icons::HOME),("Projects",icons::LAYERS),("Reports",icons::EXPLORE),("Settings",icons::TUNE)]{
        if name.to_lowercase().contains(&filter().to_lowercase()){
         SidebarMenuItem{SidebarMenuButton{label:name,icon,active_icon:Some(match name{"Home"=>icons::HOME_FILL,"Projects"=>icons::LAYERS_FILL,"Reports"=>icons::EXPLORE_FILL,_=>icons::TUNE_FILL}),active:current()==name,disabled:name=="Reports",badge:if name=="Projects"{Some(7)}else{None},onclick:move |_|{current.set(name.into());state.set_mobile_open(false);}}
          if name=="Projects"{SidebarMenuAction{label:"Project actions",icon:icons::MENU,onclick:move |_|actions+=1}}
         }
        }
       }
       SidebarMenuItem{SidebarMenuButton{label:"Collections",icon:icons::LAYERS,expanded:Some(submenu()),onclick:move |_|submenu.set(!submenu())}
        if submenu(){SidebarMenuSub{SidebarMenuItem{SidebarMenuButton{label:"Archive",icon:icons::LAYERS,onclick:move |_|current.set("Archive".into())}}}}
       }
      }}
      SidebarSeparator{}
      SidebarFooter{p{"Local workspace"}}
     }
     SidebarInset{h3{"{current}"}p{"Actions: {actions}"}p{"Expanded: {state.expanded()} · Compact: {state.compact()}"}p{"Ctrl/Cmd+B toggles this provider. Text editing keeps its own shortcut. Reload to check saved desktop expansion."}}
    }
    }
}
#[component]
pub fn AdvancedGallery(active: String) -> Element {
    let sections = [
        (
            "sidebar-provider",
            "A workspace that remembers",
            "Sidebar provider usage",
            SIDEBAR_USAGE,
        ),
        (
            "advanced-selection",
            "Async suggestions and editable chips",
            "Async selection usage",
            ASYNC_USAGE,
        ),
        (
            "advanced-tabs",
            "Scrollable collections",
            "Scrollable tabs usage",
            TABS_USAGE,
        ),
        (
            "advanced-gestures",
            "Drag with a keyboard alternative",
            "Drag and reorder usage",
            GESTURE_USAGE,
        ),
        (
            "advanced-tables",
            "Editable project data",
            "Advanced table usage",
            TABLE_USAGE,
        ),
        (
            "advanced-lists",
            "Expandable lists and swipe actions",
            "Advanced list usage",
            LIST_USAGE,
        ),
        (
            "advanced-messages",
            "Read while responses stream",
            "Streaming messages usage",
            MESSAGE_USAGE,
        ),
    ];
    rsx! {for (route,title,code_title,code) in sections{section{id:route,class:"roles-section wrap",hidden:active!=route,
     div{class:"section-header",div{p{class:"eyebrow","MATERIAL / ADVANCED BEHAVIOR"}h2{"{title}"}}}
     if active==route{match route{
      "sidebar-provider"=>rsx!{SidebarProviderDemo{}},
      "advanced-selection"=>rsx!{AdvancedSelectionDemo{}},"advanced-tabs"=>rsx!{AdvancedTabsDemo{}},"advanced-gestures"=>rsx!{AdvancedGesturesDemo{}},"advanced-tables"=>rsx!{AdvancedTableDemo{}},"advanced-lists"=>rsx!{AdvancedListDemo{}},_=>rsx!{AdvancedMessageDemo{}}
     }}
     CodeCard{eyebrow:"Copy into your Dioxus app",title:code_title,code}
    }}}
}

const ASYNC_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn AsyncProjects()->Element {
 let options=use_async_options();
 let mut query=use_signal(String::new);
 let mut open=use_signal(||false);
 let mut selected=use_signal(Vec::<String>::new);
 let state=options.snapshot();
 rsx!{Combobox {label:"Projects",options:state.options,loading:state.loading,
  load_error:state.error,virtualize:true,input_value:query(),values:selected(),open:open(),
  onopenchange:move |v|open.set(v),onchange:move |v|selected.set(v),
  oninput:move |text:String|{
   query.set(text.clone());let request=options.begin(text.clone());
   spawn(async move {request.resolve(async move {
    // Replace this future with your application data source.
    Ok(vec![SelectOption::new(text.clone(),text)])
   }).await;});
  }
 }}
}
"#;
#[component]
fn AdvancedSelectionDemo() -> Element {
    let source = use_async_options();
    let mut query = use_signal(String::new);
    let mut open = use_signal(|| false);
    let mut values = use_signal(Vec::<String>::new);
    let load = use_callback(move |(text, delay, fail): (String, u32, bool)| {
        query.set(text.clone());
        open.set(true);
        let request = source.begin(text.clone());
        spawn(async move {
            request
                .resolve(async move {
                    let _ = document::eval(&format!(
                        "await new Promise(r=>setTimeout(r,{delay}));return true;"
                    ))
                    .join::<bool>()
                    .await;
                    if fail {
                        Err("Could not load projects. Try again.".into())
                    } else {
                        Ok((0..2000)
                            .map(|i| {
                                SelectOption::new(format!("p{i}"), format!("{text} Project {i:04}"))
                            })
                            .collect())
                    }
                })
                .await;
        });
    });
    let snapshot = source.snapshot();
    let mut tags = use_signal(|| vec!["Alice".to_string(), "Bob".to_string(), "Casey".to_string()]);
    let mut draft = use_signal(String::new);
    let mut tags_open = use_signal(|| false);
    rsx! {article{class:"m3-advanced-demo",
     div{class:"m3-advanced-controls",Button{onclick:move |_|load.call(("Slow".into(),1800,false)),"Load slow results"}Button{onclick:move |_|load.call(("Fast".into(),100,false)),"Load fast results"}Button{variant:ButtonVariant::Outlined,onclick:move |_|load.call(("Retry".into(),100,true)),"Fail options request"}Button{variant:ButtonVariant::Text,onclick:move |_|source.cancel(),"Cancel options request"}}
     Combobox{label:"Remote projects",options:snapshot.options.clone(),loading:snapshot.loading,load_error:snapshot.error.clone(),virtualize:true,input_value:query(),values:values(),open:open(),onopenchange:move |v|open.set(v),oninput:move |v|load.call((v,300,false)),onchange:move |v|values.set(v)}
     p{role:"status","Results: {snapshot.options.len()} · Loading: {snapshot.loading} · Selected: {values:?}"}
     h3{"Editable people chips"}
     Combobox{label:"People",multiple:true,allow_custom:true,values:tags(),input_value:draft(),open:tags_open(),options:vec![SelectOption::new("Alice","Alice"),SelectOption::new("Bob","Bob"),SelectOption::new("Casey","Casey")],onopenchange:move |v|tags_open.set(v),oninput:move |v|draft.set(v),onchange:move |v|tags.set(v),onedit:move |value:String|{tags.write().retain(|v|v!=&value);draft.set(value);tags_open.set(true);}}
     p{"Backspace on an empty draft focuses the last chip. Arrows move between chips; Delete removes; F2 edits."}p{role:"status","People: {tags:?}"}
    }}
}
const TABS_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ScrollablePages()->Element {
 let mut selected=use_signal(||0usize);
 rsx!{Tabs {items:(0..8).map(|i|TabItem::new(format!("Page {i}"))).collect(),
  selected:selected(),scrollable:true,badges:vec![None,Some(7)],
  onchange:move |i|selected.set(i)}
 TabsViewport {selected:selected(),enabled:(0..8).collect(),onchange:move |i|selected.set(i),
  p {"Page {selected}"}
 }}
}
"#;
#[component]
fn AdvancedTabsDemo() -> Element {
    let mut selected = use_signal(|| 0usize);
    let mut rtl = use_signal(|| false);
    let items = (0..12)
        .map(|i| {
            let mut item = TabItem::new(format!("Collection {}", i + 1));
            item.disabled = i == 3;
            item
        })
        .collect();
    rsx! {article{class:"m3-advanced-demo",dir:if rtl(){"rtl"}else{"ltr"},Button{variant:ButtonVariant::Text,onclick:move |_|rtl.set(!rtl()),"Toggle tab direction"}
     Tabs{items,selected:selected(),scrollable:true,badges:vec![None,Some(7)],aria_label:"Scrollable collections",onchange:move |v|selected.set(v)}
     TabsViewport{selected:selected(),enabled:(0..12).filter(|i|*i!=3).collect(),onchange:move |v|selected.set(v),div{class:"m3-advanced-tab-panel",role:"tabpanel",aria_label:format!("Collection {}",selected()+1),h3{"Collection {selected()+1}"}p{"Swipe this panel horizontally to change collections. Vertical scrolling and gestures on controls remain available."}}}
    }}
}
const GESTURE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ReorderCards()->Element {
 let mut order=use_signal(||vec!["Draft".to_string(),"Review".to_string(),"Publish".to_string()]);
 let mut enabled=use_signal(||false);
 rsx!{span {"Notifications"} Switch {aria_label:"Notifications",draggable:true,checked:enabled(),onchange:move |v|enabled.set(v)}
 ReorderGroup {label:"Stages",items:order().into_iter().map(|name|ReorderItem {
  value:name.clone(),label:name.clone(),disabled:false,
  content:rsx!{Card {variant:CardVariant::Elevated,CardBody {"{name}"}}}
 }).collect(),onchange:move |v|order.set(v)}}
}
"#;
#[component]
fn AdvancedGesturesDemo() -> Element {
    let mut order = use_signal(|| {
        vec![
            "Draft".to_string(),
            "Review".to_string(),
            "Publish".to_string(),
        ]
    });
    let mut checked = use_signal(|| false);
    let mut changes = use_signal(|| 0usize);
    let mut chip_order = use_signal(|| {
        vec![
            "Design".to_string(),
            "Rust".to_string(),
            "Mobile".to_string(),
        ]
    });
    rsx! {article{class:"m3-advanced-demo",div{class:"m3-advanced-controls",span{"Notifications"} Switch{aria_label:"Drag notifications",draggable:true,checked:checked(),onchange:move |v|{checked.set(v);changes+=1;}}span{"Unavailable setting"} Switch{aria_label:"Disabled draggable switch",draggable:true,disabled:true}span{"Enabled: {checked} · Changes: {changes}"}}
    ReorderGroup{label:"Publication stages",items:order().into_iter().map(|name|ReorderItem{value:name.clone(),label:name.clone(),disabled:name=="Publish",content:rsx!{Card{variant:CardVariant::Elevated,CardBody{h3{"{name}"}p{"Use the handle to reorder."}}}}}).collect(),onchange:move |v|order.set(v)}p{role:"status","Card order: {order:?}"}
    ReorderGroup{label:"Topics",axis:ReorderAxis::Horizontal,items:chip_order().into_iter().map(|name|ReorderItem{value:name.clone(),label:name.clone(),disabled:false,content:rsx!{Chip{label:name}}}).collect(),onchange:move |v|chip_order.set(v)}p{role:"status","Chip order: {chip_order:?}"}
    }}
}
const TABLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn EditableTable()->Element {
 let mut rows=use_signal(||vec![DataRow::new("r1",vec![DataCell::text("Ada"),DataCell::number(10.0)])]);
 rsx!{DataTable {label:"Owners",columns:vec![DataColumn::new("name","Name"),DataColumn::new("score","Score").numeric()],
 rows:rows(),multisort:true,resizable:true,exportable:true,editable_columns:vec!["name".into(),"score".into()],
 oncellchange:move |edit:DataCellEdit|{if let Some(row)=rows.write().iter_mut().find(|r|r.id==edit.row_id){
  let index=if edit.column_key=="name"{0}else{1};
  row.cells[index]=if let Some(number)=edit.number{DataCell::formatted_number(number,edit.text)}else{DataCell::text(edit.text)};
 }}
 }}
}
"#;
#[component]
fn AdvancedTableDemo() -> Element {
    let mut rows = use_signal(|| {
        (0..24)
            .map(|i| {
                DataRow::new(
                    format!("r{i}"),
                    vec![
                        DataCell::text(if i % 2 == 0 { "Ada" } else { "Lin" }),
                        DataCell::number((24 - i) as f64),
                        DataCell::text(format!("Project {i}")),
                    ],
                )
            })
            .collect::<Vec<_>>()
    });
    rsx! {article{class:"m3-advanced-demo",DataTable{label:"Editable projects",columns:vec![DataColumn::new("owner","Owner"),DataColumn::new("score","Score").numeric(),DataColumn::new("project","Project")],rows:rows(),multisort:true,resizable:true,exportable:true,editable_columns:vec!["owner".into(),"score".into()],oncellchange:move|edit:DataCellEdit|{if let Some(row)=rows.write().iter_mut().find(|r|r.id==edit.row_id){let index=if edit.column_key=="owner"{0}else{1};row.cells[index]=if let Some(number)=edit.number{DataCell::formatted_number(number,edit.text)}else{DataCell::text(edit.text)};}}}}}
}
const MESSAGE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn StreamingConversation()->Element {
 rsx!{MessageScroller {label:"Conversation",auto_scroll:true,streaming:true,stream_anchor:"question",
  ConversationControls {}
 }}
}
#[component]
fn ConversationControls()->Element {
 let controller=use_message_scroller();let state=controller.status();
 rsx!{MessageScrollerItem {message_id:"question",p {"Explain the theme"}}
 MessageScrollerItem {message_id:"answer",p {"A streamed response…"}}
 button {onclick:{let c=controller.clone();move |_|c.pause()},"Pause following"}
 button {onclick:move |_|controller.resume(),"Resume following"}
 p {"Visible: {state.visible_messages:?}"}}
}
"#;
#[component]
fn StreamingControls(mut chunks: Signal<usize>, mut history: Signal<usize>) -> Element {
    let controller = use_message_scroller();
    let state = controller.status();
    rsx! {div{class:"m3-advanced-controls",Button{onclick:move |_|chunks+=1,"Append stream chunk"}Button{variant:ButtonVariant::Outlined,onclick:move |_|history+=1,"Prepend history"}Button{variant:ButtonVariant::Text,onclick:{let c=controller.clone();move |_|c.pause()},"Pause following"}Button{variant:ButtonVariant::Text,onclick:{let c=controller.clone();move |_|c.resume()},"Resume following"}Button{variant:ButtonVariant::Text,onclick:{let c=controller.clone();move |_|c.peek_to_message("h1")},"Peek at history"}}
    p{class:"m3-advanced-scroll-status","Following: {state.following} · Scrollable: {state.scrollable} · Visible: {state.visible_messages:?}"}}
}
#[component]
fn AdvancedMessageDemo() -> Element {
    let chunks = use_signal(|| 1usize);
    let history = use_signal(|| 4usize);
    rsx! {article{class:"m3-advanced-demo",MessageScroller{label:"Streaming pilot",height:"520px",auto_scroll:true,streaming:true,stream_anchor:"question",initial_position:MessageScrollPosition::LastAnchor,controls:rsx!{StreamingControls{chunks,history}},
    for i in (0..history()).rev(){MessageScrollerItem{message_id:format!("h{i}"),p{"History {i}: previous discussion about the Rust UI kit."}}}
    MessageScrollerItem{message_id:"question",scroll_anchor:true,p{"Can you explain Material themes?"}}
    MessageScrollerItem{message_id:"answer",p{{"A Material theme maps semantic roles to generated colors. ".repeat(chunks())}}}
    }}}
}

const LIST_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::*;
#[component]
fn ExpandableThreads()->Element {
 let mut open=use_signal(||false);let mut archived=use_signal(||false);
 rsx!{List {aria_label:"Threads",
  ListItem {label:"Design",expanded:Some(open()),onexpandedchange:move |v|open.set(v),
   expanded_content:rsx!{p {"Details about the thread"}}}
  if !archived(){SwipeAction {label:"Rust thread actions",action_label:"Archive",onaction:move |_|archived.set(true),
   ListItem {label:"Rust",interactive:true}
  }}
 }if archived(){Button {onclick:move |_|archived.set(false),"Undo archive"}}}
}
"#;
#[component]
fn AdvancedListDemo() -> Element {
    let mut expanded = use_signal(|| false);
    let mut archived = use_signal(|| false);
    let mut opened = use_signal(|| 0usize);
    rsx! {article{class:"m3-advanced-demo",List{aria_label:"Advanced threads",
     ListItem{label:"Design",expanded:Some(expanded()),onexpandedchange:move |v|expanded.set(v),expanded_content:rsx!{p{"Design review details"}Button{onclick:move |_|opened+=1,"Open review"}}}
     if !archived(){SwipeAction{label:"Rust thread actions",action_label:"Archive",onaction:move |_|archived.set(true),ListItem{label:"Rust",interactive:true,onactivate:move |_|opened+=1}}}
     ListItem{label:"Mobile",interactive:true,disabled:true}
     ListItem{label:"Themes",interactive:true,onactivate:move |_|opened+=1}
    }if archived(){Button{onclick:move |_|archived.set(false),"Undo archive"}}
    p{role:"status","Archived: {archived} · Opened: {opened}"}p{"Type a label prefix to move focus. Swiping Rust or its Archive button performs the same reversible action."}
    }}
}

#[component]
fn SidebarProviderDemo() -> Element {
    let mut controlled = use_signal(|| false);
    let mut expanded = use_signal(|| true);
    let mut reject = use_signal(|| false);
    rsx! {Button{variant:ButtonVariant::Text,onclick:move |_|reject.set(!reject()),"Toggle parent rejection"}p{"Parent rejects: {reject}"}Button{variant:ButtonVariant::Text,onclick:move |_|controlled.set(!controlled()),"Toggle state ownership"}p{"Controlled by parent: {controlled}"}SidebarProvider{expanded:if controlled(){Some(expanded())}else{None},onexpandedchange:move |v|{if !reject(){expanded.set(v);}},persistence_key:if controlled(){None}else{Some("m3e-pilot-sidebar".to_string())},SidebarDemo{}}}
}
