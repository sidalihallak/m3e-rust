const id=__ID__,rail=document.getElementById(id);if(!rail||rail.dataset.ready)return;
rail.dataset.ready='true';let animation,epoch=0,opener,overflow,locked=false;
const reduced=matchMedia('(prefers-reduced-motion:reduce)');
const unlock=()=>{if(locked){document.documentElement.style.overflow=overflow;locked=false;}if(opener?.isConnected)opener.focus({preventScroll:true});};
const sync=()=>{
 const open=rail.dataset.open==='true',token=++epoch;
 const from=rail.open?getComputedStyle(rail).transform:null;animation?.cancel();
 const off=getComputedStyle(rail).direction==='rtl'?'translateX(100%)':'translateX(-100%)';
 if(open&&!rail.open){opener=document.activeElement;overflow=document.documentElement.style.overflow;document.documentElement.style.overflow='hidden';locked=true;rail.showModal();rail.querySelector('button:not(:disabled)')?.focus();}
 if(!open&&!rail.open)return;
 const style=getComputedStyle(rail),duration=reduced.matches?0:parseFloat(style.getPropertyValue('--m3-default-spatial-duration'))||440;
 rail.dataset.phase=open?'opening':'closing';
 animation=rail.animate([{transform:from&&from!=='none'?from:open?off:'translateX(0)'},{transform:open?'translateX(0)':off}],{duration,easing:style.getPropertyValue('--m3-default-spatial').trim()||'cubic-bezier(.2,0,0,1)',fill:'both'});
 animation.finished.then(()=>{if(token!==epoch)return;rail.dataset.phase=open?'open':'closed';if(!open){rail.close();unlock();}animation.cancel();}).catch(()=>{});
};
const cancel=e=>{e.preventDefault();dioxus.send(false);};
const outside=e=>{if(e.target!==rail)return;const r=rail.getBoundingClientRect();if(e.clientX<r.left||e.clientX>r.right||e.clientY<r.top||e.clientY>r.bottom)dioxus.send(false);};
const motion=()=>{if(reduced.matches)animation?.finish();};
const tab=e=>{if(e.key!=='Tab')return;const controls=[...rail.querySelectorAll('button:not(:disabled),a[href],input:not(:disabled),select:not(:disabled),[tabindex="0"]')].filter(el=>el.getClientRects().length);const first=controls[0],last=controls.at(-1);if(!first){e.preventDefault();rail.focus();}else if(e.shiftKey&&(document.activeElement===first||!rail.contains(document.activeElement))){e.preventDefault();last.focus();}else if(!e.shiftKey&&(document.activeElement===last||!rail.contains(document.activeElement))){e.preventDefault();first.focus();}};
rail.addEventListener('cancel',cancel);rail.addEventListener('click',outside);rail.addEventListener('keydown',tab);reduced.addEventListener('change',motion);
const observer=new MutationObserver(sync);observer.observe(rail,{attributes:true,attributeFilter:['data-open']});sync();
window.__m3ModalRails??={};window.__m3ModalRails[id]=()=>{++epoch;animation?.cancel();observer.disconnect();rail.removeEventListener('cancel',cancel);rail.removeEventListener('click',outside);rail.removeEventListener('keydown',tab);reduced.removeEventListener('change',motion);if(rail.open)rail.close();unlock();delete window.__m3ModalRails[id];};
