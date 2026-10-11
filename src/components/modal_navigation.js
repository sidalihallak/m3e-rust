const id=__ID__,rail=document.getElementById(id);if(!rail||rail.dataset.ready)return;
const surface=rail.querySelector('.m3-modal-rail__surface'),scrim=rail.querySelector('.m3-modal-rail__scrim');
if(!surface||!scrim)return;
rail.dataset.ready='true';let animations=[],epoch=0,opener,overflow,locked=false;
const reduced=matchMedia('(prefers-reduced-motion:reduce)');
const unlock=()=>{if(locked){document.documentElement.style.overflow=overflow;locked=false;}if(opener?.isConnected)opener.focus({preventScroll:true});};
const sync=()=>{
 const open=rail.dataset.open==='true',token=++epoch,wasOpen=rail.open;
 // Read the current interpolated values before cancelling a superseded gesture.
 const from=wasOpen?getComputedStyle(surface).transform:null;
 const shade=wasOpen?getComputedStyle(scrim).opacity:'0';
 const shadow=wasOpen?getComputedStyle(surface).boxShadow:null;
 animations.forEach(animation=>animation.cancel());
 const off=getComputedStyle(rail).direction==='rtl'?'translateX(100%)':'translateX(-100%)';
 if(open&&!wasOpen){opener=document.activeElement;overflow=document.documentElement.style.overflow;document.documentElement.style.overflow='hidden';locked=true;rail.showModal();rail.querySelector('button:not(:disabled)')?.focus();}
 if(!open&&!wasOpen)return;
 const style=getComputedStyle(rail),restingShadow=getComputedStyle(surface).boxShadow;
 const clearShadow='0 1px 2px 0 rgb(0 0 0 / 0), 0 2px 6px 2px rgb(0 0 0 / 0)';
 const duration=name=>{if(reduced.matches)return 0;const value=style.getPropertyValue(`--m3-${name}-duration`).trim(),amount=parseFloat(value);return Number.isFinite(amount)?amount*(value.endsWith('ms')?1:1000):(name==='default-spatial'?440:240);};
 const easing=name=>style.getPropertyValue(`--m3-${name}`).trim()||'cubic-bezier(.2,0,0,1)';
 const panelEasing=style.getPropertyValue('--m3-modal-rail-spatial').trim()||easing('default-spatial');
 rail.dataset.phase=open?'opening':'closing';
 // Position uses bounded default spatial samples so the edge-attached panel
 // cannot pull away from the viewport; opacity uses the effects spring.
 // The scrim stays still while the panel moves so it does not appear in one frame.
 animations=[
   surface.animate([{transform:from&&from!=='none'?from:open?off:'translateX(0)'},{transform:open?'translateX(0)':off}],{duration:duration('default-spatial'),easing:panelEasing,fill:'both'}),
   scrim.animate([{opacity:shade},{opacity:open?1:0}],{duration:duration('default-effects'),easing:easing('default-effects'),fill:'both'}),
   // The off-screen panel's blur must not linger after its body has left.
   surface.animate([{boxShadow:shadow||clearShadow},{boxShadow:open?restingShadow:clearShadow}],{duration:duration('default-effects'),easing:easing('default-effects'),fill:'both'})
 ];
 Promise.all(animations.map(animation=>animation.finished)).then(()=>{if(token!==epoch)return;rail.dataset.phase=open?'open':'closed';if(!open){rail.close();unlock();}animations.forEach(animation=>animation.cancel());}).catch(()=>{});
};
const cancel=e=>{e.preventDefault();dioxus.send(false);};
const outside=e=>{if(e.target===scrim||e.target===rail)dioxus.send(false);};
const motion=()=>{if(reduced.matches)animations.forEach(animation=>animation.finish());};
const tab=e=>{if(e.key!=='Tab')return;const controls=[...rail.querySelectorAll('button:not(:disabled),a[href],input:not(:disabled),select:not(:disabled),[tabindex="0"]')].filter(el=>el.getClientRects().length);const first=controls[0],last=controls.at(-1);if(!first){e.preventDefault();rail.focus();}else if(e.shiftKey&&(document.activeElement===first||!rail.contains(document.activeElement))){e.preventDefault();last.focus();}else if(!e.shiftKey&&(document.activeElement===last||!rail.contains(document.activeElement))){e.preventDefault();first.focus();}};
rail.addEventListener('cancel',cancel);rail.addEventListener('click',outside);rail.addEventListener('keydown',tab);reduced.addEventListener('change',motion);
const observer=new MutationObserver(sync);observer.observe(rail,{attributes:true,attributeFilter:['data-open']});sync();
window.__m3ModalRails??={};window.__m3ModalRails[id]=()=>{++epoch;animations.forEach(animation=>animation.cancel());observer.disconnect();rail.removeEventListener('cancel',cancel);rail.removeEventListener('click',outside);rail.removeEventListener('keydown',tab);reduced.removeEventListener('change',motion);if(rail.open)rail.close();unlock();delete window.__m3ModalRails[id];};
