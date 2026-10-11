use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;

const ACCORDION_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Accordion, AccordionEntry};
#[component]
fn ProjectQuestions() -> Element {
    let mut value = use_signal(Vec::<String>::new);
    rsx! { Accordion {
        value: value(), onchange: move |next| value.set(next),
        items: vec![
            AccordionEntry::new("ownership", "Can I own the source?", rsx! { p { "Copy the Rust component and its CSS into your app." } }),
            AccordionEntry::new("theme", "How does theming work?", rsx! { p { "Use the shared Material color roles." } }),
        ],
    } }
}"#;
const MULTIPLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Accordion, AccordionEntry};
#[component]
fn ProjectSettings() -> Element {
    let mut value = use_signal(|| vec!["general".to_string()]);
    let mut locked = AccordionEntry::new("locked", "Managed settings", rsx! { p { "Contact your administrator." } });
    locked.disabled = true;
    rsx! { Accordion { multiple: true, value: value(), onchange: move |next| value.set(next),
        items: vec![
            AccordionEntry::new("general", "General", rsx! { p { "Workspace preferences." } }),
            AccordionEntry::new("notifications", "Notifications", rsx! { p { "Choose updates to receive." } }), locked,
        ],
    } }
}"#;
const COLLAPSIBLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::Collapsible;
#[component]
fn ProjectDetails() -> Element {
    let mut open = use_signal(|| false);
    rsx! { Collapsible { label: "Show project details", open: open(), onopenchange: move |next| open.set(next),
        p { "Created in October. Two contributors." }
    } }
}"#;
const TABLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{Table, TableCaption, TableHeader, TableBody, TableFooter, TableRow, TableHead, TableCell};
#[component]
fn InvoiceTable() -> Element {
    rsx! { Table { label: "Invoices",
        TableCaption { "Monthly invoices" }
        TableHeader { TableRow { TableHead { "Invoice" } TableHead { numeric: true, "Amount" } } }
        TableBody { TableRow { TableHead { scope: "row", "INV-001" } TableCell { numeric: true, "$250.00" } } }
        TableFooter { TableRow { TableCell { "Total" } TableCell { numeric: true, "$250.00" } } }
    } }
}"#;
const PAGINATION_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::Pagination;
#[component]
fn ProjectPages() -> Element {
    let mut page = use_signal(|| 1usize);
    rsx! { Pagination { page: page(), total_pages: 20, onchange: move |next| page.set(next) } }
}"#;
const TABLE_PAGINATION_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::TablePagination;
#[component]
fn InvoicePages() -> Element {
    let mut page = use_signal(|| 1usize);
    let mut size = use_signal(|| 10usize);
    rsx! { TablePagination { page: page(), total_rows: 137, page_size: size(),
        onpagechange: move |next| page.set(next),
        onpagesizechange: move |next| { size.set(next); page.set(1); },
    } }
}"#;
const DATA_TABLE_USAGE: &str = r#"use dioxus::prelude::*;
use m3e_rust_ui::{DataTable, DataColumn, DataRow, DataCell};
#[component]
fn ProjectTable() -> Element {
    let mut selected = use_signal(Vec::<String>::new);
    rsx! { DataTable { label: "Projects", selectable: true,
        selected: selected(), onselectionchange: move |next| selected.set(next),
        columns: vec![DataColumn::new("name", "Project"), DataColumn::new("budget", "Budget").numeric()],
        rows: vec![
            DataRow::new("studio", vec![DataCell::text("Design studio"), DataCell::formatted_number(1200.0, "$1,200")]),
            DataRow::new("notes", vec![DataCell::text("Field notes"), DataCell::formatted_number(250.0, "$250")]),
        ],
    } }
}"#;
fn project_rows() -> Vec<DataRow> {
    [
        ("Atlas", "In progress", 1200.0),
        ("Beacon", "Planning", 250.0),
        ("Canvas", "Complete", 9000.0),
        ("Drift", "In progress", 80.0),
        ("Echo", "Planning", 450.0),
        ("Field notes", "Complete", 1500.0),
        ("Garden", "In progress", 750.0),
        ("Harbor", "Planning", 10000.0),
        ("Iris", "Complete", 300.0),
        ("Juniper", "In progress", 600.0),
        ("Kite", "Planning", 2200.0),
        ("Legacy archive", "Archived", 0.0),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (name, status, budget))| {
        let mut row = DataRow::new(
            format!("project-{i}"),
            vec![
                DataCell::text(name),
                DataCell::text(status),
                DataCell::formatted_number(budget, format!("${budget:.2}")),
            ],
        );
        row.disabled = i == 11;
        row
    })
    .collect()
}
#[component]
pub fn DataGallery(active: String) -> Element {
    let mut one = use_signal(Vec::<String>::new);
    let mut many = use_signal(|| vec!["general".to_string()]);
    let mut details = use_signal(|| false);
    let mut disabled_details = use_signal(|| false);
    let mut page = use_signal(|| 1usize);
    let mut table_page = use_signal(|| 1usize);
    let mut size = use_signal(|| 10usize);
    let mut selected = use_signal(Vec::<String>::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| false);
    let mut locked = AccordionEntry::new(
        "managed",
        "Managed settings",
        rsx! {p{"Contact your administrator."}},
    );
    locked.disabled = true;
    rsx! {
        section{class:"roles-section wrap data-gallery",id:"accordions",hidden:active!="accordions",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / DISCLOSURE"}h2{"Progressive disclosure"}}p{class:"section-description","Single and multiple expanded panels. Each heading stays in the normal keyboard sequence."}}
            div{class:"data-gallery__disclosures",
                article{class:"data-gallery__preview",h3{"One section at a time"}Accordion{heading_level:4,value:one(),onchange:move|v|one.set(v),items:vec![
                    AccordionEntry::new("ownership","Can I own the source?",rsx!{p{"Copy the Rust component and its CSS into your app. You control the implementation."}a{href:"https://github.com/sidalihallak/m3e-rust","Browse the source"}}),
                    AccordionEntry::new("theme","How does theming work?",rsx!{p{"The kit uses Material semantic color roles generated from your seed color."}p{"Light and dark themes share the same component API."}}),
                    AccordionEntry::new("motion","Does it support reduced motion?",rsx!{p{"Reduced motion removes height interpolation and shape transitions."}}),
                ]}}
                article{class:"data-gallery__preview",h3{"Multiple sections"}Accordion{heading_level:4,multiple:true,value:many(),onchange:move|v|many.set(v),items:vec![AccordionEntry::new("general","General",rsx!{p{"Workspace preferences."}}),AccordionEntry::new("notifications","Notifications",rsx!{p{"Choose the updates you receive."}}),locked]}}
            }
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Accordion usage",code:ACCORDION_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Multiple accordion usage",code:MULTIPLE_USAGE}
        }
        section{class:"roles-section wrap data-gallery",id:"collapsibles",hidden:active!="collapsibles",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / DISCLOSURE"}h2{"Reveal supporting detail"}}p{class:"section-description","An unstyled, controlled disclosure primitive. Style the trigger and content for your composition."}}
            article{class:"data-gallery__preview",Collapsible{label:if details(){"Hide project details"}else{"Show project details"},open:details(),class:"data-gallery__collapsible",onopenchange:move|v|details.set(v),p{"Created in October. Two contributors."}a{href:"https://github.com/sidalihallak/m3e-rust","View project source"}}
                Collapsible{label:"Restricted details",open:disabled_details(),disabled:true,onopenchange:move|v|disabled_details.set(v),p{"Unavailable."}}
            }
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Collapsible usage",code:COLLAPSIBLE_USAGE}
        }
        section{class:"roles-section wrap data-gallery",id:"tables",hidden:active!="tables",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / DATA"}h2{"Clear rows and columns"}}p{class:"section-description","Native HTML table semantics, scoped headings, numeric alignment and horizontal overflow."}}
            article{class:"data-gallery__preview",Table{label:"Invoices",TableCaption{"Monthly invoices — amounts in USD"}TableHeader{TableRow{TableHead{"Invoice"}TableHead{"Status"}TableHead{"Method"}TableHead{numeric:true,"Amount"}}}TableBody{
                for (invoice,status,method,amount) in [("INV-001","Paid","Credit card","$250.00"),("INV-002","Pending","Bank transfer","$150.00"),("INV-003","Paid","Credit card","$350.00")]{TableRow{TableHead{scope:"row","{invoice}"}TableCell{"{status}"}TableCell{"{method}"}TableCell{numeric:true,"{amount}"}}}
            }TableFooter{TableRow{TableCell{colspan:3,"Total"}TableCell{numeric:true,"$750.00"}}}}}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Table usage",code:TABLE_USAGE}
        }
        section{class:"roles-section wrap data-gallery",id:"pagination",hidden:active!="pagination",
            div{class:"section-header",div{p{class:"eyebrow","NAVIGATION / PAGES"}h2{"Move through results"}}p{class:"section-description","Numbered pages for local views, and a row-range footer for tables. Both use one-based page numbers."}}
            article{class:"data-gallery__preview",h3{"Numbered pages"}Pagination{page:page(),total_pages:20,onchange:move|v|page.set(v)}p{class:"data-gallery__status",role:"status","Page {page()} of 20"}h3{"Disabled pagination"}Pagination{page:2,total_pages:5,disabled:true,label:"Disabled pagination"}}
            article{class:"data-gallery__preview",h3{"Table footer"}TablePagination{page:table_page(),total_rows:137,page_size:size(),onpagechange:move|v|table_page.set(v),onpagesizechange:move|v|{size.set(v);table_page.set(1);}}}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Pagination usage",code:PAGINATION_USAGE}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Table pagination usage",code:TABLE_PAGINATION_USAGE}
        }
        section{class:"roles-section wrap data-gallery",id:"data-tables",hidden:active!="data-tables",
            div{class:"section-header",div{p{class:"eyebrow","CONTENT / DATA"}h2{"Explore project data"}}p{class:"section-description","Filter, sort, show columns and select rows across pages. Numbers sort by value rather than formatted text."}}
            div{class:"data-gallery__states",Button{variant:ButtonVariant::Outlined,toggle:true,selected:loading(),onclick:move |_|{loading.set(!loading());error.set(false);},"Loading state"}Button{variant:ButtonVariant::Outlined,toggle:true,selected:error(),onclick:move |_|{error.set(!error());loading.set(false);},"Error state"}}
            DataTable{label:"Projects",columns:vec![DataColumn::new("project","Project"),DataColumn::new("status","Status"),DataColumn::new("budget","Budget").numeric()],rows:project_rows(),selectable:true,selected:selected(),onselectionchange:move|v|selected.set(v),loading:loading(),error:if error(){Some("Could not load projects. Try again.".into())}else{None}}
            CodeCard{eyebrow:"COPY INTO YOUR DIOXUS APP",title:"Data table usage",code:DATA_TABLE_USAGE}
        }
    }
}
