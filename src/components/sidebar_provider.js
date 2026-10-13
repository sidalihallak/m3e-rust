const id=__ID__,root=document.getElementById(id);if(!root)return;
window.__m3SidebarProviders??={};window.__m3SidebarProviders[id]?.();
let initialized=false,lastFocused=null;
const storageKey=root.dataset.key;
// Failure (blocked storage/private mode) must not prevent navigation.
if(storageKey&&root.dataset.controlled!=='true'){
 try{const saved=localStorage.getItem(storageKey);if(saved==='true'||saved==='false')dioxus.send(`restore:${saved}`);}catch{}
}
queueMicrotask(()=>{initialized=true;});
const focus=e=>{lastFocused=e.target;};root.addEventListener('focusin',focus);
const persist=()=>{const panel=lastFocused?.closest('.m3-sidebar-panel');if(panel&&root.contains(panel)&&(panel.dataset.hidden==='true'||!lastFocused.getClientRects().length)&&(root.contains(document.activeElement)||document.activeElement===document.body)){root.querySelector('.m3-sidebar-trigger:not(:disabled)')?.focus();}if(initialized&&storageKey&&root.dataset.controlled!=='true'){try{localStorage.setItem(storageKey,root.dataset.expanded);}catch{}}};
const key=e=>{if(root.dataset.shortcut!=='true'||!root.getClientRects().length||e.defaultPrevented||e.repeat||e.altKey||e.shiftKey)return;
 if((e.ctrlKey||e.metaKey)&&e.key.toLowerCase()==='b'&&!e.target.closest('input,textarea,[contenteditable=true]')){e.preventDefault();dioxus.send('toggle');}
};
const observer=new MutationObserver(persist);observer.observe(root,{attributes:true,attributeFilter:['data-expanded']});
document.addEventListener('keydown',key);
window.__m3SidebarProviders[id]=()=>{observer.disconnect();root.removeEventListener('focusin',focus);document.removeEventListener('keydown',key);delete window.__m3SidebarProviders[id];};
