use super::Icon;
use crate::icons;
use dioxus::prelude::*;
/// One-based, controlled local pagination. The component emits page changes;
/// consumers that navigate URLs should compose real links instead of callbacks.
#[component]
pub fn Pagination(
    page: usize,
    total_pages: usize,
    #[props(default="Pagination".into())] label: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<usize>,
) -> Element {
    let count = total_pages.max(1);
    let current = page.clamp(1, count);
    let mut pages = vec![1, count];
    for p in current.saturating_sub(1).max(1)..=current.saturating_add(1).min(count) {
        pages.push(p);
    }
    if current <= 3 {
        pages.extend(1..=count.min(4));
    }
    if current >= count.saturating_sub(2) {
        pages.extend(count.saturating_sub(3).max(1)..=count);
    }
    pages.sort_unstable();
    pages.dedup();
    let mut entries = Vec::new();
    for (index, p) in pages.iter().copied().enumerate() {
        if index > 0 && p > pages[index - 1] + 1 {
            entries.push((format!("gap-{p}"), None));
        }
        entries.push((format!("page-{p}"), Some(p)));
    }
    rsx! {nav{class:"m3-pagination {class}",aria_label:label,
        ul{class:"m3-pagination__items",
            li{button{r#type:"button",class:"m3-page-button m3-page-button--arrow",aria_label:"Previous page",disabled:disabled||current==1,onclick:move |_|onchange.call(current-1),super::ripple::Ripple {},Icon{icon:icons::CHEVRON_LEFT}}}
            for (key,entry) in entries {li{key:"{key}",
                if let Some(p)=entry {button{r#type:"button",class:"m3-page-button",aria_label:"Page {p}",aria_current:if p==current{Some("page")}else{None},disabled,onclick:move |_|onchange.call(p),super::ripple::Ripple {},"{p}"}}
                else{span{class:"m3-pagination__ellipsis",aria_hidden:"true","…"}}
            }}
            li{button{r#type:"button",class:"m3-page-button m3-page-button--arrow",aria_label:"Next page",disabled:disabled||current==count,onclick:move |_|onchange.call(current+1),super::ripple::Ripple {},Icon{icon:icons::CHEVRON_RIGHT}}}
        }
    }}
}

/// Material table footer: row range, page-size choice and previous/next actions.
#[component]
pub fn TablePagination(
    page: usize,
    total_rows: usize,
    page_size: usize,
    #[props(default=vec![5,10,25])] page_sizes: Vec<usize>,
    #[props(default)] disabled: bool,
    #[props(default)] onpagechange: EventHandler<usize>,
    #[props(default)] onpagesizechange: EventHandler<usize>,
) -> Element {
    let size = page_size.max(1);
    let count = total_rows.div_ceil(size).max(1);
    let current = page.clamp(1, count);
    let start = if total_rows == 0 {
        0
    } else {
        (current - 1) * size + 1
    };
    let end = (current * size).min(total_rows);
    let mut choices = page_sizes
        .into_iter()
        .filter(|v| *v > 0)
        .collect::<Vec<_>>();
    choices.push(size);
    choices.sort_unstable();
    choices.dedup();
    rsx! {nav{class:"m3-table-pagination",aria_label:"Table pagination",
        div{class:"m3-table-pagination__size",super::NativeSelect{label:"Rows per page",options:choices.into_iter().map(|v|super::SelectOption::new(v.to_string(),v.to_string())).collect::<Vec<_>>(),value:size.to_string(),disabled,onchange:move|text:String|{if let Ok(v)=text.parse::<usize>(){onpagesizechange.call(v.max(1));}}}}

        span{class:"m3-table-pagination__range",role:"status",aria_live:"polite",aria_atomic:"true","{start}–{end} of {total_rows}"}
        div{class:"m3-table-pagination__actions",
            button{r#type:"button",class:"m3-page-button m3-page-button--arrow",aria_label:"Previous table page",disabled:disabled||current==1,onclick:move |_|onpagechange.call(current-1),super::ripple::Ripple {},Icon{icon:icons::CHEVRON_LEFT}}
            button{r#type:"button",class:"m3-page-button m3-page-button--arrow",aria_label:"Next table page",disabled:disabled||current==count,onclick:move |_|onpagechange.call(current+1),super::ripple::Ripple {},Icon{icon:icons::CHEVRON_RIGHT}}
        }
    }}
}
