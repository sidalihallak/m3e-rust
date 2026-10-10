use super::action_control::ActionControl;
use super::motion::next_id;
use super::{ButtonShape, ButtonSize, ButtonVariant, IconData};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct StandardButtonItem {
    pub label: String,
    pub icon: Option<IconData>,
    pub icon_only: bool,
    pub disabled: bool,
    pub variant: ButtonVariant,
}
impl StandardButtonItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            icon_only: false,
            disabled: false,
            variant: ButtonVariant::Filled,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StandardGroupSelection {
    #[default]
    Actions,
    Single,
    Multiple,
}

/// Horizontal M3 Expressive standard group. Pressing borrows up to 15% width
/// from adjacent controls, bounded by their content and 48px target spacing.
/// `onaction` handles plain actions; selection modes use controlled `selected`.
/// Initialize a valid selection when `selection_required` is true.
#[component]
pub fn StandardButtonGroup(
    items: Vec<StandardButtonItem>,
    #[props(default)] size: ButtonSize,
    #[props(default)] shape: ButtonShape,
    #[props(default)] selection: StandardGroupSelection,
    #[props(default)] selected: Vec<usize>,
    #[props(default)] selection_required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<Vec<usize>>,
    #[props(default)] onaction: EventHandler<usize>,
) -> Element {
    let id = use_hook(|| next_id("m3-standard"));
    let size_class = match size {
        ButtonSize::ExtraSmall => "xs",
        ButtonSize::Small => "s",
        ButtonSize::Medium => "m",
        ButtonSize::Large => "l",
        ButtonSize::ExtraLarge => "xl",
    };
    let mut selected: Vec<usize> = selected.into_iter().filter(|i| *i < items.len()).collect();
    selected.sort_unstable();
    selected.dedup();
    if selection == StandardGroupSelection::Single {
        selected.truncate(1);
    }
    use_effect({
        let id = id.clone();
        move || {
            let script = STANDARD_SCRIPT.replace("__ID__", &format!("{id:?}"));
            spawn(async move {
                let _ = document::eval(&script).join::<bool>().await;
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3StandardGroups?.[{id:?}]?.();")).await;
            });
        }
    });
    rsx! {
        div { id, class: "m3-standard m3-standard--{size_class} {class}", role: "group", aria_label,
            for (index, item) in items.into_iter().enumerate() {
                { let current = selected.clone(); let active = current.contains(&index);
                  let item_disabled = disabled || item.disabled;
                  rsx! { ActionControl {
                    key: "{index}", label: item.label, icon: item.icon, icon_only: item.icon_only,
                    variant: item.variant, size, shape, disabled: item_disabled,
                    pressed: if selection == StandardGroupSelection::Actions { None } else { Some(active) },
                    onclick: move |_| {
                        if item_disabled { return; }
                        if selection == StandardGroupSelection::Actions { onaction.call(index); return; }
                        let mut next = current.clone();
                        if active {
                            if selection_required && next.len() == 1 { return; }
                            next.retain(|i| *i != index);
                        } else if selection == StandardGroupSelection::Single { next = vec![index]; }
                        else { next.push(index); next.sort_unstable(); }
                        onchange.call(next);
                    },
                  }}
                }
            }
        }
    }
}

const STANDARD_SCRIPT: &str = r#"
const id = __ID__;
const group = document.getElementById(id);
if (!group) return false;
window.__m3StandardGroups = window.__m3StandardGroups || {};
window.__m3StandardGroups[id]?.();
const media = matchMedia('(prefers-reduced-motion: reduce)');
const savedWidth = group.style.width;
let base = new Map(), minimum = new Map(), timer = 0, disposed = false;
const items = () => [...group.children].filter(e => e.matches('button'));
const active = () => items().some(e => e.dataset.ripplePressed === 'true');
const measure = () => {
  if (disposed || active() || items().some(e=>e.getAnimations().some(a=>a.transitionProperty==='width'))) return false;
  const list = items();
  for (const e of list) { e.style.transition = 'none'; e.style.width = ''; }
  group.style.width = savedWidth;
  base = new Map(); minimum = new Map();
  for (const e of list) {
    const width = e.getBoundingClientRect().width;
    base.set(e, width);
    const content = e.querySelector('.m3-action__content').getBoundingClientRect().width;
    // Preserve content, with at least 4px either side. Icon targets may extend
    // into the inter-button spacing; clamp borrowing before targets overlap.
    minimum.set(e, Math.min(width, Math.max(24, content + 8)));
  }
  const computed = getComputedStyle(group);
  const gap = parseFloat(computed.columnGap) || 0;
  const padding = (parseFloat(computed.paddingInlineStart)||0)+(parseFloat(computed.paddingInlineEnd)||0);
  group.style.width = (padding + list.reduce((sum,e) => sum + base.get(e), 0) + gap * Math.max(0,list.length-1)) + 'px';
  for (const e of list) e.style.width = base.get(e) + 'px';
  group.getBoundingClientRect();
  for (const e of list) e.style.transition = '';
  return true;
};
const layout = () => {
  const list = items();
  if(list.some(e=>!base.has(e))) return;
  const index = media.matches ? -1 : list.findIndex(e => e.dataset.ripplePressed === 'true');
  const neighbors = index < 0 ? [] : [index-1,index+1].filter(i => i >= 0 && i < list.length);
  const capacity = i => Math.max(0, base.get(list[i]) - minimum.get(list[i]));
  let expansion = index < 0 ? 0 : Math.min(base.get(list[index]) * .15, neighbors.reduce((sum,i) => sum + capacity(i),0));
  const reductions = new Map();
  for (const i of neighbors) reductions.set(i, Math.min(capacity(i), expansion / neighbors.length));
  let remaining = expansion - [...reductions.values()].reduce((a,b)=>a+b,0);
  for (const i of neighbors) { const extra = Math.min(remaining, capacity(i)-reductions.get(i)); reductions.set(i,reductions.get(i)+extra); remaining -= extra; }
  // Clamp again if enlarged 48px hit areas would overlap their neighbors.
  const gap = parseFloat(getComputedStyle(group).columnGap) || 0;
  for (const i of neighbors) {
    const next = base.get(list[i]) - reductions.get(i);
    const pressed = base.get(list[index]) + expansion;
    const overlap = Math.max(0,(48-next)/2) + Math.max(0,(48-pressed)/2) - gap;
    if (overlap > 0) { const giveBack = Math.min(reductions.get(i), overlap*2); reductions.set(i,reductions.get(i)-giveBack); expansion -= giveBack; }
  }
  list.forEach((e,i) => { if(base.has(e)) e.style.width = (base.get(e) + (i === index ? expansion : - (reductions.get(i) || 0))) + 'px'; });
};
const remeasure = () => {
  clearTimeout(timer);
  if(disposed) return;
  if(!measure()) { timer=setTimeout(remeasure,50); return; }
  layout();
};
const observer = new MutationObserver(records => {
  if (records.some(r => !(r.target.nodeType===1?r.target:r.target.parentElement)?.closest('.m3-ripple') && (r.type === 'childList' || r.type === 'characterData' || r.attributeName === 'class'))) {
    clearTimeout(timer); timer=setTimeout(remeasure,50);
  }
  layout();
});
observer.observe(group,{subtree:true, childList:true, characterData:true, attributes:true, attributeFilter:['data-ripple-pressed','class']});
const resize = new ResizeObserver(() => { clearTimeout(timer); timer = setTimeout(remeasure,50); });
if(group.parentElement) resize.observe(group.parentElement);
const onPreference = () => { layout(); };
media.addEventListener('change',onPreference);
const keyboard = e => {
  if (!['ArrowLeft','ArrowRight','Home','End'].includes(e.key)) return;
  const list = items().filter(b => !b.disabled), pos = list.indexOf(document.activeElement);
  if(pos < 0 || !list.length) return;
  e.preventDefault();
  const delta = (e.key === 'ArrowRight' ? 1 : -1) * (getComputedStyle(group).direction === 'rtl' ? -1 : 1);
  list[e.key === 'Home' ? 0 : e.key === 'End' ? list.length-1 : (pos+delta+list.length)%list.length].focus();
};
group.addEventListener('keydown',keyboard);
measure();
if(document.fonts) document.fonts.ready.then(remeasure);
window.__m3StandardGroups[id] = () => {
  disposed = true; clearTimeout(timer); observer.disconnect(); resize.disconnect();
  media.removeEventListener('change',onPreference); group.removeEventListener('keydown',keyboard);
  group.style.width = savedWidth; for(const e of items()) { e.style.width = ''; e.style.transition = ''; }
  delete window.__m3StandardGroups[id];
};
return true;
"#;
