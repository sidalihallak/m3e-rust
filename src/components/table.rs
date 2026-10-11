use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TableSort {
    #[default]
    None,
    Ascending,
    Descending,
}
impl TableSort {
    pub(crate) fn attr(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Ascending => Some("ascending"),
            Self::Descending => Some("descending"),
        }
    }
}
/// Native table in a keyboard-scrollable container. Use a caption and scoped
/// headings; keep interactive controls in cells rather than making rows buttons.
#[component]
pub fn Table(
    label: String,
    #[props(default)] class: String,
    #[props(default)] container_class: String,
    #[props(default)] row_count: Option<usize>,
    #[props(default)] column_count: Option<usize>,
    children: Element,
) -> Element {
    rsx! {div {class:"m3-table-container {container_class}",role:"region",aria_label:"{label} table scroll area",tabindex:"0",table {class:"m3-table {class}",aria_label:label,aria_rowcount:row_count,aria_colcount:column_count,{children}}}}
}
#[component]
pub fn TableRow(
    #[props(default)] row_index: Option<usize>,
    #[props(default)] selected: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {tr{class:"m3-table__row {class}","data-selected":selected,aria_rowindex:row_index,{children}}}
}
#[component]
pub fn TableHead(
    #[props(default)] column_index: Option<usize>,
    #[props(default)] numeric: bool,
    #[props(default)] sort: TableSort,
    #[props(default="col".into())] scope: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {th{class:"m3-table__head {class}",scope,aria_sort:sort.attr(),"data-numeric":numeric,aria_colindex:column_index,{children}}}
}
#[component]
pub fn TableCell(
    #[props(default)] column_index: Option<usize>,
    #[props(default)] numeric: bool,
    #[props(default = 1)] colspan: usize,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {td{class:"m3-table__cell {class}",colspan,"data-numeric":numeric,aria_colindex:column_index,{children}}}
}

#[component]
pub fn TableHeader(#[props(default)] class: String, children: Element) -> Element {
    rsx! {thead{class:"m3-table__header {class}",{children}}}
}

#[component]
pub fn TableBody(#[props(default)] class: String, children: Element) -> Element {
    rsx! {tbody{class:"m3-table__body {class}",{children}}}
}

#[component]
pub fn TableFooter(#[props(default)] class: String, children: Element) -> Element {
    rsx! {tfoot{class:"m3-table__footer {class}",{children}}}
}

#[component]
pub fn TableCaption(#[props(default)] class: String, children: Element) -> Element {
    rsx! {caption{class:"m3-table__caption {class}",{children}}}
}
