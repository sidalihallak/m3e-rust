const id = __ID__;
const dialog = document.getElementById(id);
if(!dialog) return false;
window.__m3Dialogs = window.__m3Dialogs || {};
window.__m3Dialogs[id]?.();
const surface=dialog.querySelector('.m3-dialog__surface'), scrim=dialog.querySelector('.m3-dialog__scrim'), body=dialog.querySelector('.m3-dialog__body');
const media=matchMedia('(prefers-reduced-motion: reduce)');
let generation=0, animations=[], disposed=false, restoreTarget=null, outsideDown=false;
window.__m3DialogLocks=window.__m3DialogLocks||{owners:new Set(),overflow:''};
const locks=window.__m3DialogLocks;
const lock=()=>{ if(!locks.owners.size) {locks.overflow=document.documentElement.style.overflow; document.documentElement.style.overflow='hidden';} locks.owners.add(id); };
const unlock=()=>{ if(!locks.owners.has(id))return; locks.owners.delete(id); if(!locks.owners.size) document.documentElement.style.overflow=locks.overflow; };
const easing=name=>getComputedStyle(dialog).getPropertyValue('--m3-'+name).trim();
const ms = name => { const value=getComputedStyle(dialog).getPropertyValue('--m3-'+name+'-duration').trim(); return (parseFloat(value)||0)*(value.endsWith('ms')?1:1000); };
const cancelAnimations=()=>{ ++generation; animations.forEach(a=>a.cancel()); animations=[]; };
const measure=()=>{
  const more=body.scrollHeight>body.clientHeight+1;
  body.tabIndex=more?0:-1;
  body.dataset.moreAbove=body.scrollTop>0?'true':'false';
  body.dataset.moreBelow=body.scrollTop+body.clientHeight<body.scrollHeight-1?'true':'false';
};
const focusable=()=>[...dialog.querySelectorAll('a[href],button:not(:disabled),input:not(:disabled),textarea:not(:disabled),select:not(:disabled),[tabindex]')].filter(e=>e.tabIndex>=0&&e.getClientRects().length&&!e.closest('[inert],[aria-hidden="true"]'));
const focusInitial=()=>{
  measure(); const requested=document.getElementById(dialog.dataset.initialFocus);
  const target=requested&&dialog.contains(requested)&&!requested.disabled&&requested.getClientRects().length?requested:focusable()[0];
  (target||dialog).focus({preventScroll:true});
};
const closeMenus=()=>{
  for(const p of dialog.querySelectorAll('[popover]')) {
    if(p.matches(':popover-open')) { if(p.dataset.menuRoot===p.id) window.__m3Menus?.[p.id]?.requestClose(false); p.hidePopover(); }
  }
};
const sync=()=>{
  if(disposed) return;
  const wants=dialog.dataset.open==='true';
  if(wants) {
    if(dialog.open&&dialog.dataset.phase!=='closing') { measure(); return; }
    const continuing=dialog.open;
    const previous=continuing?{opacity:getComputedStyle(surface).opacity,transform:getComputedStyle(surface).transform,scrim:getComputedStyle(scrim).opacity}:null;
    cancelAnimations(); const token=generation;
    dialog.inert=false;
    if(!dialog.open) { restoreTarget=document.activeElement; dialog.showModal(); lock(); }
    dialog.dataset.phase='opening';
    focusInitial();
    if(media.matches) { dialog.dataset.phase='open'; return; }
    animations=[
      surface.animate([{opacity:previous?.opacity||0},{opacity:1}],{duration:ms('default-effects'),easing:easing('default-effects'),fill:'both'}),
      surface.animate([{transform:previous?.transform||'scale(.8)'},{transform:'scale(1)'}],{duration:ms('default-spatial'),easing:easing('default-spatial'),fill:'both'}),
      scrim.animate([{opacity:previous?.scrim||0},{opacity:1}],{duration:ms('slow-effects'),easing:easing('slow-effects'),fill:'both'})
    ];
    Promise.all(animations.map(a=>a.finished)).then(()=>{if(!disposed&&token===generation){dialog.dataset.phase='open'; animations.forEach(a=>a.cancel()); animations=[];}}).catch(()=>{});
  } else {
    if(!dialog.open||dialog.dataset.phase==='closing') return;
    const previous={opacity:getComputedStyle(surface).opacity,transform:getComputedStyle(surface).transform,scrim:getComputedStyle(scrim).opacity};
    cancelAnimations(); const token=generation;
    dialog.dataset.phase='closing'; dialog.inert=true; closeMenus();
    const finish=()=>{
      if(disposed||token!==generation||dialog.dataset.open==='true') return;
      dialog.close(); dialog.inert=false; dialog.dataset.phase='closed'; unlock();
      animations.forEach(a=>a.cancel()); animations=[];
      if(restoreTarget?.isConnected&&!restoreTarget.disabled) restoreTarget.focus({preventScroll:true});
    };
    if(media.matches) { finish(); return; }
    animations=[
      surface.animate([previous,{opacity:0,transform:'scale(.9)'}],{duration:150,easing:'cubic-bezier(.3,0,.8,.15)',fill:'both'}),
      scrim.animate([{opacity:previous.scrim},{opacity:0}],{duration:ms('slow-effects'),easing:easing('slow-effects'),fill:'both'})
    ];
    Promise.all(animations.map(a=>a.finished)).then(finish).catch(()=>{});
  }
};
const dismiss=()=>dioxus.send(true);
const cancel=e=>{e.preventDefault(); if(dialog.dataset.dismissEscape==='true') dismiss();};
const closed=()=>{if(dialog.dataset.open==='true') dismiss();};
const pointerDown=e=>{ outsideDown=e.target===dialog||e.target===scrim; };
const pointerUp=e=>{if(outsideDown&&(e.target===dialog||e.target===scrim)&&dialog.dataset.dismissOutside==='true')dismiss(); outsideDown=false;};
const pointerCancel=()=>{outsideDown=false;};
const keyboard=e=>{
  if(e.defaultPrevented||e.key!=='Tab'||!dialog.open||dialog.inert) return;
  const list=focusable();
  if(!list.length) {e.preventDefault(); dialog.focus();return;}
  const first=list[0],last=list[list.length-1];
  if(e.shiftKey&&(document.activeElement===first||!dialog.contains(document.activeElement))) {e.preventDefault();last.focus();}
  else if(!e.shiftKey&&(document.activeElement===last||!dialog.contains(document.activeElement))) {e.preventDefault();first.focus();}
};
const reduced=()=>{if(media.matches) for(const a of animations) if(a.playState!=='finished')a.finish();};
const changes=new MutationObserver(sync); changes.observe(dialog,{attributes:true,attributeFilter:['data-open']});
const resize=new ResizeObserver(measure); resize.observe(body); resize.observe(surface);
const contents=new MutationObserver(measure); contents.observe(body,{childList:true,subtree:true,characterData:true});
dialog.addEventListener('cancel',cancel); dialog.addEventListener('close',closed); dialog.addEventListener('keydown',keyboard);
dialog.addEventListener('pointerdown',pointerDown); dialog.addEventListener('pointerup',pointerUp); dialog.addEventListener('pointercancel',pointerCancel);
body.addEventListener('scroll',measure); media.addEventListener('change',reduced);
window.__m3Dialogs[id]=()=>{
  disposed=true; cancelAnimations(); changes.disconnect(); resize.disconnect(); contents.disconnect();
  dialog.removeEventListener('cancel',cancel); dialog.removeEventListener('close',closed); dialog.removeEventListener('keydown',keyboard);
  dialog.removeEventListener('pointerdown',pointerDown); dialog.removeEventListener('pointerup',pointerUp); dialog.removeEventListener('pointercancel',pointerCancel);
  body.removeEventListener('scroll',measure); media.removeEventListener('change',reduced);
  const wasOpen=dialog.open; closeMenus(); if(wasOpen)dialog.close(); unlock();
  if(wasOpen&&restoreTarget?.isConnected&&!restoreTarget.disabled)restoreTarget.focus({preventScroll:true});
  delete window.__m3Dialogs[id];
};
sync();
