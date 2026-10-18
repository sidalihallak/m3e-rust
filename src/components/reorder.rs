use super::motion::next_id;
use dioxus::prelude::*;
#[derive(Clone, PartialEq)]
pub struct ReorderItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
    pub content: Element,
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ReorderAxis {
    #[default]
    Vertical,
    Horizontal,
}

/// Explicit drag handles preserve scrolling begun on card/chip content. Native
/// handles also support Space, arrows, Enter/Space commit, Escape cancellation.
#[component]
pub fn ReorderGroup(
    label: String,
    #[props(default)] axis: ReorderAxis,
    items: Vec<ReorderItem>,
    #[props(default)] disabled: bool,
    #[props(default)] onchange: EventHandler<Vec<String>>,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| next_id("m3-reorder"));
    let change = use_callback(move |values| onchange.call(values));
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("reorder.js").replace("__ID__", &format!("{id:?}"));
            let id = id.clone();
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(values) = eval.recv::<Vec<String>>().await {
                    change.call(values);
                    let _ =
                        document::eval(&format!("window.__m3ReorderCommits?.[{id:?}]?.();")).await;
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let _ = document::eval(&format!("window.__m3Reorders?.[{id:?}]?.();"));
        }
    });
    let order = items
        .iter()
        .map(|item| item.value.clone())
        .collect::<Vec<_>>();
    rsx! {div{id:id.clone(),class:"m3-reorder {class}",role:"group",aria_label:label,"data-disabled":disabled,"data-axis":if axis==ReorderAxis::Horizontal{"horizontal"}else{"vertical"},
     p{class:"m3-feedback__sr",id:"{id}-help","Use Space to pick up, arrows to move, Enter or Space to drop, Escape to cancel."}
     div{class:"m3-reorder-list",for (index,item) in items.into_iter().enumerate(){{let previous=order.clone();let following=order.clone();let row_value=item.value.clone();let back_id=id.clone();let next_id=id.clone();let back_value=row_value.clone();let next_value=row_value.clone();rsx!{div{key:"{item.value}",class:"m3-reorder-item","data-value":item.value,"data-label":item.label.clone(),button{r#type:"button",class:"m3-reorder-handle",disabled:disabled||item.disabled,aria_label:"Reorder {item.label}",aria_describedby:"{id}-help",super::Icon{icon:crate::icons::MENU}}div{class:"m3-reorder-content",{item.content}}
      div{class:"m3-reorder-alternatives",
       button{r#type:"button",class:"m3-reorder-move",disabled:disabled||item.disabled||index==0,aria_label:"Move {item.label} earlier","data-move-value":back_value.clone(),"data-move-direction":"earlier",onclick:move |_|{if disabled || item.disabled || index == 0 { return; } let mut values=previous.clone();values.swap(index,index-1);change.call(values);restore_move_focus(back_id.clone(),back_value.clone(),"earlier");},super::Icon{icon:crate::icons::CHEVRON_LEFT}}
       button{r#type:"button",class:"m3-reorder-move",disabled:disabled||item.disabled||index+1>=following.len(),aria_label:"Move {item.label} later","data-move-value":next_value.clone(),"data-move-direction":"later",onclick:move |_|{if disabled || item.disabled || index+1 >= following.len() { return; } let mut values=following.clone();values.swap(index,index+1);change.call(values);restore_move_focus(next_id.clone(),next_value.clone(),"later");},super::Icon{icon:crate::icons::CHEVRON_RIGHT}}
      }
     }}}}}
     span{class:"m3-feedback__sr",role:"status",aria_live:"polite","data-reorder-status":""}
    }}
}

fn restore_move_focus(id: String, value: String, direction: &str) {
    let script = format!(
        r#"requestAnimationFrame(()=>{{const root=document.getElementById({id:?});if(root&&(document.activeElement===document.body||root.contains(document.activeElement))){{const row=[...root.querySelectorAll('.m3-reorder-item')].find(e=>e.dataset.value==={value:?});const status=root.querySelector('[data-reorder-status]');if(row&&status)status.textContent=row.dataset.label+' moved to position '+([...row.parentElement.children].indexOf(row)+1);const buttons=[...root.querySelectorAll('.m3-reorder-move')].filter(b=>b.dataset.moveValue==={value:?}&&!b.disabled);(buttons.find(b=>b.dataset.moveDirection==={direction:?})||buttons[0]||root.querySelector('.m3-reorder-handle:not(:disabled)'))?.focus({{preventScroll:true}});}}}});"#
    );
    spawn(async move {
        {
            let _ = document::eval(&script).await;
        }
    });
}
