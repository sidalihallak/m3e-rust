(async()=>{
const root=document.getElementById(__ID__);if(!root)return;window.__m3Menubars??={};window.__m3Menubars[root.id]?.();let current=0,hoverOpened=null,desired=root.dataset.openIndex===undefined?null:Number(root.dataset.openIndex),pending=false,resolve;
const triggers=()=>Array.from(root.querySelectorAll('[data-bar-index]'));
const rtl=()=>getComputedStyle(root).direction==='rtl';
function focus(index){const rows=triggers();if(!rows.length)return;current=(index+rows.length)%rows.length;rows.forEach((n,i)=>n.tabIndex=i===current?0:-1);rows[current].focus({preventScroll:true});}
function closePanels(restore=false){root.querySelectorAll('.m3-menu-panel[data-menu-root]').forEach(n=>{if(n.dataset.open==='true')window.__m3Menus?.[n.id]?.requestClose(restore);});}
function close(restore=false){closePanels(restore);desired=null;pending=true;dioxus.send(['close']);}
function open(index,last=false){const n=triggers()[index];if(!n||n.getAttribute('aria-disabled')==='true')return;closePanels(false);desired=index;pending=true;dioxus.send(['open',String(index),last?'last':'first']);}
function key(e){if(!root.contains(e.target)||e.defaultPrevented)return;hoverOpened=null;const button=e.target.closest('[data-bar-index]'),panel=e.target.closest('.m3-menu-panel');
 if(e.key==='Tab'){const active=triggers()[current],all=Array.from(document.querySelectorAll('a[href],button,input,select,textarea,[tabindex]')).filter(n=>n.tabIndex>=0&&!n.disabled&&n.getClientRects().length&&!n.closest('[inert],[aria-hidden=true]')&&(!root.contains(n)||n===active));const at=all.indexOf(active),next=all[at+(e.shiftKey?-1:1)];if(next){e.preventDefault();e.stopPropagation();close(false);next.focus();}return;}
 const right=rtl()?'ArrowLeft':'ArrowRight',left=rtl()?'ArrowRight':'ArrowLeft';
 if(button){current=Number(button.dataset.barIndex);if([right,left,'Home','End'].includes(e.key)){e.preventDefault();focus(e.key==='Home'?0:e.key==='End'?triggers().length-1:current+(e.key===right?1:-1));if(desired!==null)open(current);}else if(e.key==='ArrowDown'||e.key==='ArrowUp'){e.preventDefault();open(current,e.key==='ArrowUp');}}
 else if(panel&&panel.dataset.menuRoot===panel.id&&(e.key===right||e.key===left)){
  if(e.key===right&&e.target.closest('[data-submenu]'))return;e.preventDefault();e.stopPropagation();focus(current+(e.key===right?1:-1));open(current);
 }
}
const hover=e=>{const n=e.target.closest('[data-bar-index]');if(n&&desired!==null&&Number(n.dataset.barIndex)!==current){current=Number(n.dataset.barIndex);hoverOpened=current;focus(current);open(current);}};
const clicked=e=>{const n=e.target.closest('[data-bar-index]');if(!n)return;e.preventDefault();e.stopPropagation();const index=Number(n.dataset.barIndex);if(n.getAttribute('aria-disabled')==='true')return;const active=desired;if(e.detail>0&&hoverOpened===index){hoverOpened=null;if(active!==index)open(index);else document.getElementById(`${root.id}-panel-${index}`)?.querySelector(':scope > [data-menu-item]')?.focus({preventScroll:true});return;}hoverOpened=null;if(active===index)close(true);else open(index);};
const focused=e=>{const n=e.target.closest('[data-bar-index]');if(n){current=Number(n.dataset.barIndex);triggers().forEach((n,i)=>n.tabIndex=i===current?0:-1);}};
const refresh=()=>triggers().forEach((n,i)=>n.tabIndex=i===Math.min(current,triggers().length-1)?0:-1);
const observer=new MutationObserver(()=>{refresh();const actual=root.dataset.openIndex===undefined?null:Number(root.dataset.openIndex);if(!pending||actual===desired){desired=actual;pending=false;}});observer.observe(root,{childList:true,attributes:true,attributeFilter:['data-open-index']});refresh();root.addEventListener('keydown',key,true);root.addEventListener('pointerover',hover);root.addEventListener('click',clicked,true);root.addEventListener('focusin',focused);
window.__m3Menubars[root.id]=()=>{observer.disconnect();root.removeEventListener('keydown',key,true);root.removeEventListener('pointerover',hover);root.removeEventListener('click',clicked,true);root.removeEventListener('focusin',focused);delete window.__m3Menubars[root.id];resolve?.();};await new Promise(r=>resolve=r);
})();
