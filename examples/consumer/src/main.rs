use dioxus::prelude::*;
use m3e_rust_ui::icons;
use m3e_rust_ui::*;
fn main() {
    dioxus::launch(App);
}

/// The whole library is one import plus one provider: pick a seed colour and the
/// Material tokens, the component styles and the light/dark scheme follow from it.
#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            default_theme: ThemeSettings { seed: 0xff006a60, ..Default::default() },
            Consumer {}
        }
    }
}
#[component]
fn Consumer() -> Element {
    let mut theme = use_theme();
    let mut clicks = use_signal(|| 0usize);
    let mut dialog = use_signal(|| false);
    let mut title = use_signal(String::new);
    let mut query = use_signal(String::new);
    let mut choice = use_signal(Vec::<String>::new);
    let mut open = use_signal(|| false);
    let mut selected = use_signal(|| 0usize);
    let mut enabled = use_signal(|| false);
    let mut rows = use_signal(|| {
        vec![
            DataRow::new("r1", vec![DataCell::text("Ada"), DataCell::number(10.0)]),
            DataRow::new("r2", vec![DataCell::text("Lin"), DataCell::number(20.0)]),
        ]
    });
    rsx! {document::Stylesheet{href:asset!("/assets/app.css")}
    div{class:"consumer",main{
     h1{"Material 3 kit"}p{"An independent Dioxus app that imports the crate as a normal dependency."}
     Button{variant:ButtonVariant::Tonal,onclick:move |_|theme.with_mut(|t| t.mode = if t.mode == ColorMode::Dark {ColorMode::Light} else {ColorMode::Dark}),"Toggle consumer theme"}
     section{id:"consumer-actions",h2{"Icons and actions"}div{class:"row",
      Button{onclick:move |_|clicks+=1,"Count click"}Button{disabled:true,"Disabled action"}
      IconButton{icon:icons::rounded::ROCKET_LAUNCH,aria_label:"Launch",onclick:move |_|clicks+=1}
      Icon{icon:icons::outlined::ROCKET_LAUNCH_FILL,aria_label:Some("Outlined launch".into())}
      Icon{icon:icons::sharp::SETTINGS,aria_label:Some("Sharp settings".into())}
      MaterialSymbol{name:"travel_explore",style:SymbolStyle::Rounded,aria_label:Some("Travel".into())}
     }p{role:"status","Clicks: {clicks}"}Card{variant:CardVariant::Elevated,CardBody{h3{"Installed card"}p{"Tonal surface, elevation and spacing come from the copied component."}}CardActions{Button{onclick:move |_|clicks+=1,"Card action"}}}}
     section{id:"consumer-fields",h2{"Fields and selection"}div{class:"fields",
      TextField{label:"Project name",value:title(),oninput:move|v|title.set(v)}
      Combobox{label:"Owner",options:vec![SelectOption::new("ada","Ada"),SelectOption::new("lin","Lin"),SelectOption{value:"disabled".into(),label:"Unavailable".into(),disabled:true,group:None,icon:None}],values:choice(),input_value:query(),open:open(),onopenchange:move|v|open.set(v),oninput:move|v|query.set(v),onchange:move|v|choice.set(v)}
     }p{role:"status","Project: {title} · Owner: {choice:?}"}}
     section{h2{"Dialog and controls"}div{class:"row",Button{onclick:move |_|dialog.set(true),"Create project"}span{class:"consumer-switch","Notifications" Switch{aria_label:"Notifications",checked:enabled(),draggable:true,onchange:move|v|enabled.set(v)}}}
      Dialog{open:dialog(),title:"Create project",onopenchange:move |v|dialog.set(v),actions:rsx!{DialogAction{label:"Save project",onclick:move |_|dialog.set(false)}},TextField{label:"Dialog project name",value:title(),oninput:move|v|title.set(v)}}
      Tabs{aria_label:"Consumer pages",items:vec![TabItem::new("Overview"),TabItem::new("Activity"),TabItem::new("Settings")],selected:selected(),onchange:move|v|selected.set(v)}p{role:"status","Selected page: {selected}"}
     }
     section{h2{"Composable workspace"}SidebarProvider{SidebarConsumer{}}}
     section{id:"consumer-table",h2{"Editable local table"}DataTable{label:"Installed table",columns:vec![DataColumn::new("name","Name"),DataColumn::new("score","Score").numeric()],rows:rows(),multisort:true,resizable:true,exportable:true,editable_columns:vec!["name".into(),"score".into()],oncellchange:move|e:DataCellEdit|{if let Some(row)=rows.write().iter_mut().find(|r|r.id==e.row_id){let index=if e.column_key=="name"{0}else{1};row.cells[index]=e.number.map(|v|DataCell::formatted_number(v,e.text.clone())).unwrap_or_else(||DataCell::text(e.text));}}}}
     section{h2{"Message following"}MessageScroller{label:"Installed conversation",height:"240px",auto_scroll:true,MessageScrollerItem{message_id:"q",p{"How do themes work?"}}MessageScrollerItem{message_id:"a",p{"Semantic color roles and shared motion tokens come from the installed kit."}}}}
    }}}
}
#[component]
fn SidebarConsumer() -> Element {
    let sidebar = use_sidebar();
    let mut page = use_signal(|| "Overview".to_string());
    rsx! {SidebarToggle{}SidebarLayout{class:"consumer-layout",SidebarPanel{label:"Installed workspace",SidebarHeader{h3{"Workspace"}}SidebarContent{SidebarMenu{for name in ["Overview","Activity"]{SidebarMenuItem{key:"{name}",SidebarMenuButton{label:name,icon:if name=="Overview"{icons::HOME}else{icons::ROCKET_LAUNCH},active_icon:Some(if name=="Overview"{icons::HOME_FILL}else{icons::ROCKET_LAUNCH_FILL}),active:page()==name,onclick:move|_|{page.set(name.into());sidebar.set_mobile_open(false);}}}}}}SidebarFooter{p{"Local sources"}}}SidebarInset{h3{"{page}"}}}}
}
