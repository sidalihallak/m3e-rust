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
    #[props(default)] selected: Vec<String>,
    #[props(default = 5)] initial_page_size: usize,
    #[props(default)] loading: bool,
    #[props(default)] error: Option<String>,
    #[props(default="No matching rows.".into())] empty_message: String,
    #[props(default)] class: String,
    #[props(default)] onselectionchange: EventHandler<Vec<String>>,
) -> Element {
    let mut query = use_signal(String::new);
    let mut sort = use_signal(|| None::<(String, bool)>);
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
    if let Some((key, desc)) = sort() {
        if let Some(index) = columns.iter().position(|c| c.key == key && c.sortable) {
            let numeric = columns[index].numeric;
            filtered.sort_by(|a, b| {
                let a = a.cells.get(index);
                let b = b.cells.get(index);
                let order = if numeric {
                    match (a.and_then(|c| c.number), b.and_then(|c| c.number)) {
                        (Some(a), Some(b)) => a.total_cmp(&b),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                } else {
                    a.map(|c| c.text.to_lowercase())
                        .cmp(&b.map(|c| c.text.to_lowercase()))
                };
                if desc { order.reverse() } else { order }
            });
        }
    }
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
    let span = visible.len() + usize::from(selectable);
    let selection_count = selected.len();
    rsx! {section{class:"m3-data-table {class}",aria_label:label.clone(),aria_busy:loading,
           div{class:"m3-data-table__toolbar",
               super::TextField{class:"m3-data-table__filter",label:format!("Filter {label}"),input_type:"search",value:query(),disabled:blocked,oninput:move|text|{query.set(text);page.set(1);}}

               details{class:"m3-data-table__columns",summary{"Columns"}div{class:"m3-data-table__column-options",for c in columns.iter(){
                   {let key=c.key.clone();let checked=!hidden().contains(&key);let only=checked&&visible.len()==1;rsx!{label{key:"{key}",Checkbox{aria_label:Some(format!("Show {} column",c.label)),checked,disabled:blocked||only,onchange:move|checked|{let mut next=hidden();if checked{next.retain(|k|k!=&key);}else if !next.contains(&key){next.push(key.clone());}if !checked && sort().as_ref().is_some_and(|(old,_)|old==&key){sort.set(None);page.set(1);}
    hidden.set(next);}},"{c.label}"}}}
               }}}
           }
           if selectable{div{class:"m3-data-table__selection",span{role:"status",aria_live:"polite","{selection_count} selected"}if selection_count>0{button{r#type:"button",class:"m3-data-table__clear",disabled:blocked,onclick:move |_|onselectionchange.call(Vec::new()),"Clear selection"}}}}
           Table{label:label.clone(),row_count:if blocked{None}else{Some(total+1)},column_count:Some(columns.len()+usize::from(selectable)),
               TableCaption{"{label}" span{class:"m3-data-table__sr",". Activate a sortable heading to cycle ascending, descending, and original order."}}
               TableHeader{TableRow{row_index:Some(1),
                   if selectable{TableHead{column_index:Some(1),Checkbox{aria_label:Some("Select rows on this page".into()),checked:!eligible.is_empty()&&chosen==eligible.len(),indeterminate:chosen>0&&chosen<eligible.len(),disabled:blocked||eligible.is_empty(),onchange:move |_|onselectionchange.call(all_next.clone())}}}
                   for index in visible.iter().copied(){
                       {let c=&columns[index];let key=c.key.clone();let direction=sort().filter(|s|s.0==key).map(|s|if s.1{TableSort::Descending}else{TableSort::Ascending}).unwrap_or_default();rsx!{TableHead{key:"{key}",numeric:c.numeric,sort:direction,column_index:Some(index+1+usize::from(selectable)),
                           if c.sortable{button{r#type:"button",class:"m3-data-table__sort",disabled:blocked,aria_label:format!("Sort by {}",c.label),onclick:move |_|{let next=if let Some((old,desc))=sort(){if old==key{if desc{None}else{Some((key.clone(),true))}}else{Some((key.clone(),false))}}else{Some((key.clone(),false))};sort.set(next);page.set(1);},"{c.label}" span{class:"m3-data-table__sort-icon",aria_hidden:"true",{match direction{TableSort::None=>"↕",TableSort::Ascending=>"↑",TableSort::Descending=>"↓"}}}}}
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
                               if let Some(cell)=row.cells.get(index){if let Some(content)=cell.content.clone(){ {content} }else{"{cell.text}"}}
                           }}
                       }}}
                   }}
               }
           }
           TablePagination{page:current,total_rows:total,page_size:size(),disabled:blocked,onpagechange:move|v|page.set(v),onpagesizechange:move|v|{size.set(v);page.set(1);}}
       }}
}
