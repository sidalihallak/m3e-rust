const id=__ID__,root=document.getElementById(id);
if(!root)return;
window.__m3ItemGroups??={};window.__m3ItemGroups[id]?.();
const controls=()=>[...root.querySelectorAll('button,a[href],input,textarea,select')].filter(e=>e.closest('.m3-item-group')===root&&!e.matches(':disabled')&&e.getAttribute('aria-disabled')!=='true');
const sync=()=>{const list=controls();const current=list.includes(document.activeElement)?document.activeElement:(list.find(e=>e.getAttribute('aria-selected')==='true')||list[0]);for(const e of list)e.tabIndex=e===current?0:-1;};
const focus=()=>sync();
const blur=()=>queueMicrotask(sync);
const key=e=>{if(e.altKey||e.metaKey||e.ctrlKey||e.isComposing)return;const list=controls(),index=list.indexOf(document.activeElement);if(index<0)return;
 if(e.key===' '&&document.activeElement.matches('a[href]')){e.preventDefault();document.activeElement.click();return;}
 if(e.target.matches('input,textarea,select')||e.target.isContentEditable)return;
 if(!['ArrowDown','ArrowUp','ArrowRight','ArrowLeft','Home','End'].includes(e.key))return;
 e.preventDefault();const rtl=getComputedStyle(root).direction==='rtl';const next=e.key==='Home'?0:e.key==='End'?list.length-1:(index+(e.key==='ArrowDown'||(e.key==='ArrowRight'&&!rtl)||(e.key==='ArrowLeft'&&rtl)?1:-1)+list.length)%list.length;list[next]?.focus();};
const observer=new MutationObserver(sync);observer.observe(root,{childList:true,subtree:true,attributes:true,attributeFilter:['aria-selected','disabled','aria-disabled']});
root.addEventListener('focusin',focus);root.addEventListener('focusout',blur);root.addEventListener('keydown',key);sync();
window.__m3ItemGroups[id]=()=>{observer.disconnect();root.removeEventListener('focusin',focus);root.removeEventListener('focusout',blur);root.removeEventListener('keydown',key);delete window.__m3ItemGroups[id];};
