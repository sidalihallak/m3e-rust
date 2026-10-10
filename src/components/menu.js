// Browser behavior copied with menu.rs. Native manual popovers escape clipping,
// inherit the consuming theme, and allow independent cascading panels.
const id = __ID__;
const root = document.getElementById(id);
if (!root) return false;
window.__m3Menus = window.__m3Menus || {};
window.__m3Menus[id]?.cleanup();
const media = matchMedia('(prefers-reduced-motion: reduce)');
const state = new Map();
let disposed = false, restore = true, positioningFrame = 0, buffer = '', typedAt = 0;
const panels = () => [root,...root.querySelectorAll('.m3-menu-panel')];
const items = panel => [...panel.querySelectorAll(':scope > button[data-menu-item]')];
const anchor = () => document.getElementById(root.dataset.anchor);
const easing = name => getComputedStyle(root).getPropertyValue('--m3-'+name).trim();
const ms = name => { const value=getComputedStyle(root).getPropertyValue('--m3-'+name+'-duration').trim(); return (parseFloat(value)||0)*(value.endsWith('ms')?1:1000); };
const shown = panel => panel.matches(':popover-open');
const record = panel => {
  if (!state.has(panel)) state.set(panel,{generation:0,animations:[]});
  return state.get(panel);
};
const stop = panel => { const s=record(panel); s.generation++; s.animations.forEach(a=>a.cancel()); s.animations=[]; return s; };
const place = (panel, trigger) => {
  if (!trigger || !shown(panel)) return;
  const r = trigger.getBoundingClientRect(), sub = panel !== root;
  const rtl = getComputedStyle(trigger).direction === 'rtl';
  panel.style.maxHeight = 'calc(100dvh - 16px)';
  let width = Math.min(280, Math.max(192,sub ? 192 : r.width));
  const edge=sub?trigger.closest('.m3-menu-panel').getBoundingClientRect():null;
  if(sub) {
    const room=Math.max(edge.left-8,window.innerWidth-edge.right-8);
    if(room>=112) width=Math.min(width,room);
  }
  panel.style.width = Math.min(width,window.innerWidth-16)+'px';
  const h=panel.offsetHeight, w=panel.offsetWidth;
  let x, y, origin='top';
  if(sub) {
    x = rtl ? edge.left-w : edge.right;
    if(x+w>window.innerWidth-8) x=edge.left-w;
    if(x<8) x=edge.right;
    y=r.top-2;
    panel.dataset.side=x<r.left?'left':'right';
    origin = x<r.left?'right top':'left top';
  } else if(root.hasAttribute('data-point-x')) {
    x=Number(root.dataset.pointX); y=Number(root.dataset.pointY);
  } else {
    x=rtl?r.right-w:r.left; y=r.bottom+4;
    const below=window.innerHeight-r.bottom-12, above=r.top-12;
    if(h>below && above>below) { y=r.top-h-4; origin='bottom'; }
  }
  x=Math.max(8,Math.min(x,window.innerWidth-w-8));
  y=Math.max(8,Math.min(y,window.innerHeight-h-8));
  panel.style.left=x+'px'; panel.style.top=y+'px';
  panel.style.setProperty('--m3-menu-origin',origin);
};
const focusEdge = (panel,last=false) => { const list=items(panel); (list[last?list.length-1:0]||panel).focus({preventScroll:true}); };
const openPanel = (panel,trigger,last=false,focus=true) => {
  const continuing=shown(panel)&&panel.dataset.phase==='closing';
  const previous=continuing?{opacity:getComputedStyle(panel).opacity,transform:getComputedStyle(panel).transform}:null;
  const s=stop(panel), token=s.generation;
  if(!shown(panel)) panel.showPopover();
  panel.removeAttribute('aria-hidden'); panel.removeAttribute('inert'); panel.dataset.phase='opening';
  if(panel!==root) trigger.setAttribute('aria-expanded','true');
  place(panel,trigger);
  if(!media.matches) {
    s.animations=[
      panel.animate([{opacity:previous?.opacity||0},{opacity:1}],{duration:ms('fast-effects'),easing:easing('fast-effects'),fill:'both'}),
      panel.animate([{transform:previous?.transform||'scaleY(.6)'},{transform:'scaleY(1)'}],{duration:ms('default-spatial'),easing:easing('default-spatial'),fill:'both'})
    ];
    Promise.all(s.animations.map(a=>a.finished)).then(()=>{ if(!disposed&&s.generation===token) { panel.dataset.phase='open'; s.animations.forEach(a=>a.cancel()); s.animations=[]; } }).catch(()=>{});
  } else panel.dataset.phase='open';
  if(focus) focusEdge(panel,last);
  if(!positioningFrame) positioningFrame=requestAnimationFrame(trackPosition);
};
const closePanel = (panel,animate=true) => {
  const previous={opacity:getComputedStyle(panel).opacity,transform:getComputedStyle(panel).transform};
  const s=stop(panel), token=s.generation;
  panel.setAttribute('aria-hidden','true'); panel.setAttribute('inert','');
  const parent=document.getElementById(panel.dataset.menuParent); parent?.setAttribute('aria-expanded','false');
  if(!shown(panel)) return Promise.resolve();
  panel.dataset.phase='closing';
  const finish=()=>{ if(!disposed&&s.generation===token) { if(shown(panel)) panel.hidePopover(); panel.dataset.phase='closed'; s.animations.forEach(a=>a.cancel()); s.animations=[]; } };
  if(media.matches||!animate) { finish(); return Promise.resolve(); }
  const animation=panel.animate([previous,{opacity:0,transform:'scaleY(.9)'}],{duration:120,easing:'cubic-bezier(.3,0,.8,.15)',fill:'both'});
  s.animations=[animation];
  return animation.finished.then(finish).catch(()=>{});
};
const closeDescendants = panel => { for(const p of panels().slice().reverse()) if(p!==panel&&panel.contains(p)) closePanel(p); };
const openSubmenu = (item,focus=true) => {
  if(item.getAttribute('aria-disabled')==='true') return;
  const child=document.getElementById(item.dataset.submenu); if(!child) return;
  const parent=item.closest('.m3-menu-panel');
  for(const p of panels()) if(p!==child&&p.dataset.menuParent&&document.getElementById(p.dataset.menuParent)?.closest('.m3-menu-panel')===parent) { closeDescendants(p); closePanel(p); }
  if(!shown(child)||child.dataset.phase==='closing') openPanel(child,item,false,focus);
  else if(focus) focusEdge(child);
};
const requestClose = (returnFocus=true) => { restore=returnFocus; dioxus.send(true); };
const sync = () => {
  if(disposed) return;
  if(root.dataset.open==='true') {
    if(!shown(root)||root.dataset.phase==='closing') { restore=true; buffer=''; openPanel(root,anchor(),root.dataset.initialLast==='true'); }
    else { place(root,anchor()); for(const p of panels().slice(1)) if(shown(p)) place(p,document.getElementById(p.dataset.menuParent)); }
  } else {
    if(!shown(root)) return;
    closeDescendants(root);
    closePanel(root).then(()=>{ if(!disposed&&root.dataset.open!=='true'&&restore) anchor()?.focus({preventScroll:true}); });
  }
};
const keyboard = e => {
  if(!shown(root)||root.dataset.open!=='true') return;
  const panel=e.target.closest('.m3-menu-panel'); if(!panel||panel.dataset.menuRoot!==id) return;
  const list=items(panel), index=list.indexOf(document.activeElement), key=e.key;
  if(['ArrowDown','ArrowUp','Home','End'].includes(key)) {
    e.preventDefault(); e.stopPropagation();
    if(list.length) list[key==='Home'?0:key==='End'?list.length-1:key==='ArrowDown'?(index+1)%list.length:(index<=0?list.length-1:index-1)].focus();
  } else if(key==='Escape') {
    e.preventDefault(); e.stopPropagation();
    if(panel!==root) { closeDescendants(panel); closePanel(panel); document.getElementById(panel.dataset.menuParent)?.focus(); }
    else requestClose(true);
  } else if(key==='Tab') {
    e.preventDefault(); e.stopPropagation();
    const trigger=anchor(), scope=root.closest('dialog[open]')||document;
    const candidates=[...scope.querySelectorAll('a[href],button:not(:disabled),input:not(:disabled),select:not(:disabled),textarea:not(:disabled),[tabindex]')].filter(el=>el.tabIndex>=0&&el.getClientRects().length&&!root.contains(el)&&!el.closest('[inert],[aria-hidden="true"]'));
    const at=candidates.indexOf(trigger); let next;
    if(at>=0) next=candidates[e.shiftKey?at-1:at+1];
    if(!next&&scope!==document) next=candidates[e.shiftKey?candidates.length-1:0];
    requestClose(false); (next||trigger)?.focus();
  } else if(key==='ArrowRight'||key==='ArrowLeft') {
    const forward=(getComputedStyle(panel).direction==='rtl')?'ArrowLeft':'ArrowRight';
    if(key===forward&&document.activeElement?.dataset.submenu) { e.preventDefault(); e.stopPropagation(); openSubmenu(document.activeElement); }
    else if(key!==forward&&panel!==root) { e.preventDefault(); e.stopPropagation(); closeDescendants(panel); closePanel(panel); document.getElementById(panel.dataset.menuParent)?.focus(); }
  } else if(key.length===1&&key!==' '&&!e.ctrlKey&&!e.metaKey&&!e.altKey) {
    e.preventDefault(); const now=Date.now(); buffer=now-typedAt>700?'':buffer; typedAt=now;
    buffer+=key.toLocaleLowerCase();
    if([...buffer].every(c=>c===buffer[0])) buffer=buffer[0];
    for(let offset=1;offset<=list.length;offset++) { const item=list[(index+offset+list.length)%list.length]; if(item.dataset.label.toLocaleLowerCase().startsWith(buffer)) { item.focus(); break; } }
  }
};
const click = e => { const item=e.target.closest('[data-submenu]'); if(item&&root.contains(item)) openSubmenu(item); };
const hover = e => {
  if(e.pointerType!=='mouse') return;
  const item=e.target.closest('button[data-menu-item]'); if(!item||!root.contains(item)||item.contains(e.relatedTarget)) return;
  item.focus({preventScroll:true});
  if(item.dataset.submenu) openSubmenu(item,false);
  else closeDescendants(item.closest('.m3-menu-panel'));
};
const outside = e => { if(root.dataset.open==='true'&&!root.contains(e.target)&&!anchor()?.contains(e.target)) requestClose(false); };
const focusedOutside = e => { if(e.target===document.body||e.target===document.documentElement) return; if(root.dataset.open==='true'&&!root.contains(e.target)&&!anchor()?.contains(e.target)) requestClose(false); };
const reposition = () => { if(shown(root)) { place(root,anchor()); for(const p of panels().slice(1)) if(shown(p)) place(p,document.getElementById(p.dataset.menuParent)); } };
const trackPosition = () => {
  positioningFrame=0; if(disposed) return; reposition();
  if([...state.values()].some(s=>s.animations.some(a=>a.playState==='running'))) positioningFrame=requestAnimationFrame(trackPosition);
};
const toggled=e=>{ if(e.newState==='closed'&&root.dataset.open==='true'&&root.dataset.phase!=='closing')requestClose(false); };
root.addEventListener('toggle',toggled);
const changed = new MutationObserver(sync);
changed.observe(root,{attributes:true,attributeFilter:['data-open','data-point-x','data-point-y'],childList:true,subtree:true});
const resize=new ResizeObserver(reposition); resize.observe(root); if(anchor()) resize.observe(anchor());
const reduced = () => { if(media.matches) for(const s of state.values()) for(const a of s.animations) { if(a.playState!=='finished') a.finish(); } };
root.addEventListener('keydown',keyboard); root.addEventListener('click',click); root.addEventListener('pointerover',hover);
document.addEventListener('pointerdown',outside,true); document.addEventListener('focusin',focusedOutside);
window.addEventListener('resize',reposition); window.addEventListener('scroll',reposition,true); media.addEventListener('change',reduced);
window.__m3Menus[id]={ requestClose, cleanup:()=>{
  disposed=true; cancelAnimationFrame(positioningFrame); changed.disconnect(); resize.disconnect();
  root.removeEventListener('toggle',toggled); root.removeEventListener('keydown',keyboard); root.removeEventListener('click',click); root.removeEventListener('pointerover',hover);
  document.removeEventListener('pointerdown',outside,true); document.removeEventListener('focusin',focusedOutside);
  window.removeEventListener('resize',reposition); window.removeEventListener('scroll',reposition,true); media.removeEventListener('change',reduced);
  const wasOpen=shown(root); for(const p of panels().reverse()) { stop(p); if(shown(p)) p.hidePopover(); }
  if(wasOpen&&restore) anchor()?.focus({preventScroll:true}); delete window.__m3Menus[id];
}};
sync();
