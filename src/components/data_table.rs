use super::{
    Checkbox, Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TablePagination,
    TableRow, TableSort,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DataColumn {
    pub key: String,
    pub label: String,
    pub numeric: bool,
    pub sortable: bool,
}
impl DataColumn {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            numeric: false,
            sortable: true,
        }
    }
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }
}
/// Text is used for filtering and text sorting. A numeric key sorts formatted
/// numbers numerically; custom content preserves its searchable plain text.
#[derive(Clone, PartialEq)]
pub struct DataCell {
    pub text: String,
    pub number: Option<f64>,
    pub content: Option<Element>,
}
impl DataCell {
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            text: value.into(),
            number: None,
            content: None,
        }
    }
    pub fn number(value: f64) -> Self {
        Self::formatted_number(value, value.to_string())
    }
    pub fn formatted_number(value: f64, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            number: value.is_finite().then_some(value),
            content: None,
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct DataRow {
    pub id: String,
    pub cells: Vec<DataCell>,
    pub disabled: bool,
}
impl DataRow {
    pub fn new(id: impl Into<String>, cells: Vec<DataCell>) -> Self {
        Self {
            id: id.into(),
            cells,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataCellEdit {
    pub row_id: String,
    pub column_key: String,
    pub text: String,
    pub number: Option<f64>,
}

/// Local data composition on a semantic HTML table, not an ARIA grid. Unique
/// column keys and row IDs are required; cells follow column order. Selection
/// is controlled by stable IDs; query/sort/page/visibility are local UI state.
/// Server queries, virtualization, editing and range selection are extensions.
#[component]
pub fn DataTable(
    label: String,
    columns: Vec<DataColumn>,
    rows: Vec<DataRow>,
    #[props(default)] selectable: bool,
    #[props(default)] multisort: bool,
    #[props(default)] resizable: bool,
    #[props(default)] exportable: bool,
    #[props(default)] editable_columns: Vec<String>,
    #[props(default)] oncellchange: EventHandler<DataCellEdit>,
    #[props(default)] selected: Vec<String>,
    #[props(default = 5)] initial_page_size: usize,
    #[props(default)] loading: bool,
    #[props(default)] error: Option<String>,
    #[props(default="No matching rows.".into())] empty_message: String,
    #[props(default)] class: String,
    #[props(default)] onselectionchange: EventHandler<Vec<String>>,
) -> Element {
    let id = use_hook(|| super::motion::next_id("m3-data-table"));
    let mut query = use_signal(String::new);
    let mut sort = use_signal(Vec::<(String, bool)>::new);
    let mut widths = use_signal(Vec::<(String, u16)>::new);
    let mut editing = use_signal(|| None::<(String, String)>);
    let mut draft = use_signal(String::new);
    let mut edit_error = use_signal(|| false);
    use_effect({
        let id = id.clone();
        move || {
            let active = editing();
            let script = format!(
                r#"
const root=document.getElementById({id:?});if(!root)return;
requestAnimationFrame(()=>{{if(!root.isConnected)return;const input=root.querySelector('.m3-data-table__edit input');if(input){{root.dataset.editRow=input.dataset.editRow;root.dataset.editColumn=input.dataset.editColumn;input.focus();input.select();}}else if(root.dataset.editRow){{[...root.querySelectorAll('[data-edit-row]')].find(e=>e.dataset.editRow===root.dataset.editRow&&e.dataset.editColumn===root.dataset.editColumn)?.focus();delete root.dataset.editRow;delete root.dataset.editColumn;}}}});
"#
            );
            let _ = active;
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });
    let mut page = use_signal(|| 1usize);
    let mut size = use_signal(|| initial_page_size.max(1));
    let mut hidden = use_signal(Vec::<String>::new);
    let visible = columns
        .iter()
        .enumerate()
        .filter(|(_, c)| !hidden().contains(&c.key))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let text = query().to_lowercase();
    let mut filtered = rows
        .iter()
        .filter(|r| {
            r.cells
                .iter()
                .any(|c| c.text.to_lowercase().contains(&text))
                || text.is_empty()
        })
        .cloned()
        .collect::<Vec<_>>();
    let sorting = sort();
    filtered.sort_by(|a, b| {
        for (key, desc) in &sorting {
            if let Some(index) = columns.iter().position(|c| &c.key == key && c.sortable) {
                let ac = a.cells.get(index);
                let bc = b.cells.get(index);
                let order = if columns[index].numeric {
                    match (ac.and_then(|c| c.number), bc.and_then(|c| c.number)) {
                        (Some(a), Some(b)) => a.total_cmp(&b),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                } else {
                    ac.map(|c| c.text.to_lowercase())
                        .cmp(&bc.map(|c| c.text.to_lowercase()))
                };
                let order = if *desc { order.reverse() } else { order };
                if order != std::cmp::Ordering::Equal {
                    return order;
                }
            }
        }
        std::cmp::Ordering::Equal
    });
    let csv = if exportable {
        data_csv(&columns, &filtered, &visible)
    } else {
        String::new()
    };
    let total = filtered.len();
    let count = total.div_ceil(size()).max(1);
    let current = page().clamp(1, count);
    use_effect(move || {
        if page() > count {
            page.set(count);
        }
    });
    let shown = filtered
        .into_iter()
        .skip((current - 1) * size())
        .take(size())
        .collect::<Vec<_>>();
    let selected = selected
        .into_iter()
        .filter(|id| rows.iter().any(|r| r.id == *id && !r.disabled))
        .collect::<Vec<_>>();
    let eligible = shown
        .iter()
        .filter(|r| !r.disabled)
        .map(|r| r.id.clone())
        .collect::<Vec<_>>();
    let chosen = eligible.iter().filter(|id| selected.contains(id)).count();
    let blocked = loading || error.is_some();
    let mut all_next = selected.clone();
    if chosen == eligible.len() {
        all_next.retain(|id| !eligible.contains(id));
    } else {
        for id in &eligible {
            if !all_next.contains(id) {
                all_next.push(id.clone());
            }
        }
    }
    let table_width = visible
        .iter()
        .map(|i| {
            widths()
                .iter()
                .find(|(k, _)| k == &columns[*i].key)
                .map(|(_, v)| usize::from(*v))
                .unwrap_or(180)
        })
        .sum::<usize>()
        + if selectable { 64 } else { 0 };
    let span = visible.len() + usize::from(selectable);
    let selection_count = selected.len();
    rsx! {section{id,class:"m3-data-table {class}","data-resizable":resizable,style:"--m3-data-width:{table_width}px",aria_label:label.clone(),aria_busy:loading,
           div{class:"m3-data-table__toolbar",
               super::TextField{class:"m3-data-table__filter",label:format!("Filter {label}"),input_type:"search",value:query(),disabled:blocked,oninput:move|text|{query.set(text);page.set(1);}}

               details{class:"m3-data-table__columns",summary{"Columns"}div{class:"m3-data-table__column-options",for c in columns.iter(){
                   {let key=c.key.clone();let checked=!hidden().contains(&key);let only=checked&&visible.len()==1;rsx!{label{key:"{key}",Checkbox{aria_label:Some(format!("Show {} column",c.label)),checked,disabled:blocked||only,onchange:move|checked|{let mut next=hidden();if checked{next.retain(|k|k!=&key);}else if !next.contains(&key){next.push(key.clone());}if !checked {let next=sort().into_iter().filter(|(old,_)|old!=&key).collect();sort.set(next);page.set(1);}
    hidden.set(next);}},"{c.label}"}}}
               }}}
           }
           if exportable{a{class:"m3-data-table__export",href:format!("data:text/csv;charset=utf-8,{}",url_data(&csv)),download:"table.csv","Export filtered CSV"}}
           if multisort{p{class:"m3-data-table__sort-status",role:"status", "Sort priority: {sorting:?}. Shift-activate a heading to add another sort."}}
           if resizable {
            details {
             summary {"Column widths"}
             for column in columns.iter() {
              {let key=column.key.clone();let width=widths().iter().find(|(k,_)|k==&key).map(|(_,v)|*v).unwrap_or(180);
               rsx!{label{"{column.label}" input{r#type:"range",class:"m3-data-table__resize",min:"100",max:"400",step:"10",value:width,aria_label:"Width of {column.label} column",oninput:move|e|{if let Ok(value)=e.value().parse::<u16>(){let mut next=widths();next.retain(|(k,_)|k!=&key);next.push((key.clone(),value));widths.set(next);}}}}}
              }
             }
            }
           }
           if selectable{div{class:"m3-data-table__selection",span{role:"status",aria_live:"polite","{selection_count} selected"}if selection_count>0{button{r#type:"button",class:"m3-data-table__clear",disabled:blocked,onclick:move |_|onselectionchange.call(Vec::new()),"Clear selection"}}}}
           Table{label:label.clone(),row_count:if blocked{None}else{Some(total+1)},column_count:Some(columns.len()+usize::from(selectable)),
               if resizable{colgroup{if selectable{col{style:"width:64px"}}for index in visible.iter().copied(){{let width=widths().iter().find(|(k,_)|k==&columns[index].key).map(|(_,v)|*v).unwrap_or(180);rsx!{col{style:"width:{width}px"}}}}}}
               TableCaption{"{label}" span{class:"m3-data-table__sr",". Activate a sortable heading to cycle ascending, descending, and original order."}}
               TableHeader{TableRow{row_index:Some(1),
                   if selectable{TableHead{column_index:Some(1),Checkbox{aria_label:Some("Select rows on this page".into()),checked:!eligible.is_empty()&&chosen==eligible.len(),indeterminate:chosen>0&&chosen<eligible.len(),disabled:blocked||eligible.is_empty(),onchange:move |_|onselectionchange.call(all_next.clone())}}}
                   for index in visible.iter().copied(){
                       {let c=&columns[index];let key=c.key.clone();let direction=sort().iter().find(|s|s.0==key).map(|s|if s.1{TableSort::Descending}else{TableSort::Ascending}).unwrap_or_default();rsx!{TableHead{key:"{key}",numeric:c.numeric,sort:if sorting.first().is_some_and(|(k,_)|k==&key){direction}else{TableSort::None},column_index:Some(index+1+usize::from(selectable)),
                           if c.sortable{button{r#type:"button",class:"m3-data-table__sort",disabled:blocked,aria_label:format!("Sort by {}",c.label),onclick:move |e|{let old=sort();let direction=old.iter().find(|(k,_)|k==&key).map(|(_,d)|*d);let mut next=if multisort&&e.modifiers().shift(){old}else{vec![]};let at=next.iter().position(|(k,_)|k==&key);match direction{Some(true)=>next.retain(|(k,_)|k!=&key),Some(false)=>{if let Some(at)=at{next[at].1=true;}else{next.push((key.clone(),true));}},None=>next.push((key.clone(),false))}sort.set(next);page.set(1);},"{c.label}" span{class:"m3-data-table__sort-icon",aria_hidden:"true",{match direction{TableSort::None=>"↕",TableSort::Ascending=>"↑",TableSort::Descending=>"↓"}}}}}
                           else{"{c.label}"}
                       }}}
                   }
               }}
               TableBody{
                   if loading{TableRow{TableCell{colspan:span.max(1),div{class:"m3-data-table__message",role:"status","Loading rows…"}}}}
                   else if let Some(message)=error{TableRow{TableCell{colspan:span.max(1),div{class:"m3-data-table__message",role:"alert","{message}"}}}}
                   else if shown.is_empty(){TableRow{TableCell{colspan:span.max(1),div{class:"m3-data-table__message",role:"status","{empty_message}"}}}}
                   else{for (offset,row) in shown.into_iter().enumerate(){
                       {let checked=selected.contains(&row.id);let mut next=selected.clone();if checked{next.retain(|id|id!=&row.id);}else{next.push(row.id.clone());}rsx!{TableRow{key:"{row.id}",selected:checked,row_index:Some((current-1)*size()+offset+2),
                           if selectable{TableCell{column_index:Some(1),Checkbox{aria_label:Some(format!("Select {}",row.cells.first().map(|c|c.text.as_str()).unwrap_or(&row.id))),checked,disabled:row.disabled,onchange:move |_|onselectionchange.call(next.clone())}}}
                           for index in visible.iter().copied(){TableCell{key:"{columns[index].key}",numeric:columns[index].numeric,column_index:Some(index+1+usize::from(selectable)),
                               if let Some(cell)=row.cells.get(index){
                                if editable_columns.contains(&columns[index].key)&&!row.disabled{
                                 {let row_id=row.id.clone();let column_key=columns[index].key.clone();let numeric=columns[index].numeric;let active=editing()==Some((row_id.clone(),column_key.clone()));let save_row=row_id.clone();let save_key=column_key.clone();let save=EventHandler::new(move |_|{let text=draft();let number=if numeric{text.parse::<f64>().ok().filter(|n|n.is_finite())}else{None};if numeric&&number.is_none(){edit_error.set(true);}else{oncellchange.call(DataCellEdit{row_id:save_row.clone(),column_key:save_key.clone(),text,number});editing.set(None);}});
                                  rsx!{if active{form{class:"m3-data-table__edit",onsubmit:move |event|{event.prevent_default();save.call(());},
                                   input{class:"m3-input m3-input--outlined","data-edit-row":row_id.clone(),"data-edit-column":column_key.clone(),r#type:"text",inputmode:if numeric{"decimal"}else{"text"},aria_label:format!("Edit {} for {}",columns[index].label,row_id),aria_invalid:edit_error(),value:draft(),oninput:move|e|{draft.set(e.value());edit_error.set(false);},onkeydown:move|e|{if e.key()==Key::Escape{editing.set(None);}}}
                                   if edit_error(){span{role:"alert","Enter a finite number."}}
                                   button{r#type:"submit","Save cell"}
                                   button{r#type:"button",onclick:move |_|editing.set(None),"Cancel edit"}
                                  }}else{button{r#type:"button",class:"m3-data-table__cell-edit","data-edit-row":row_id.clone(),"data-edit-column":column_key.clone(),aria_label:format!("Edit {} for {}",columns[index].label,row_id),onclick:{let text=if numeric{cell.number.map(|n|n.to_string()).unwrap_or(cell.text.clone())}else{cell.text.clone()};move |_|{editing.set(Some((row_id.clone(),column_key.clone())));draft.set(text.clone());edit_error.set(false);}},"{cell.text}"}}}
                                 }
                                }else if let Some(content)=cell.content.clone(){ {content} }else{"{cell.text}"}}

                           }}
                       }}}
                   }}
               }
           }
           TablePagination{page:current,total_rows:total,page_size:size(),disabled:blocked,onpagechange:move|v|page.set(v),onpagesizechange:move|v|{size.set(v);page.set(1);}}
       }}
}

fn data_csv(columns: &[DataColumn], rows: &[DataRow], visible: &[usize]) -> String {
    let quote = |s: &str| format!("\"{}\"", s.replace('"', "\"\""));
    let mut lines = vec![
        visible
            .iter()
            .map(|i| quote(&columns[*i].label))
            .collect::<Vec<_>>()
            .join(","),
    ];
    for row in rows {
        lines.push(
            visible
                .iter()
                .map(|i| {
                    let cell = row.cells.get(*i);
                    let text = cell.map(|c| c.text.as_str()).unwrap_or("");
                    if cell.is_some_and(|c| c.number.is_none())
                        && text.trim_start().starts_with(['=', '+', '-', '@'])
                    {
                        quote(&format!("'{text}"))
                    } else {
                        quote(text)
                    }
                })
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    lines.join("\r\n")
}
fn url_data(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("%{b:02X}")).collect()
}
