// Shared top-layer placement/motion; ship with anchored.rs and component CSS.
const id=__ID__, root=document.getElementById(id);
if(!root) return;
window.__m3Anchors ||= {};
window.__m3Anchors[id]?.cleanup();
const trigger=document.getElementById(root.dataset.anchor);
if(!trigger) return;
const positionTrigger=document.getElementById(root.dataset.positionAnchor)||trigger;
const kind=root.dataset.kind, automatic=kind==='tooltip'||kind==='hover';
const heightLimit=(kind==='select'||kind==='combobox')?288:Infinity;
const media=matchMedia('(prefers-reduced-motion: reduce)');
let disposed=false, generation=0, animations=[], frame=0, timer=0, closeTimer=0, opened=false, suppressed=false, touch=null, consumedTouch=false;
const original=new Map();
const own=(name,value)=>{if(!original.has(name)) original.set(name,trigger.getAttribute(name)); if(value==null) trigger.removeAttribute(name); else trigger.setAttribute(name,value);};
const shown=()=>root.matches(':popover-open');
const send=(value)=>dioxus.send(['open',String(value)]);
const stop=()=>{generation++;animations.forEach(a=>a.cancel());animations=[];return generation;};
const token=(name)=>getComputedStyle(root).getPropertyValue('--m3-'+name).trim();
const duration=name=>{const v=token(name+'-duration');return (parseFloat(v)||0)*(v.endsWith('ms')?1:1000);};
const interactive=()=>kind==='popover'||kind==='rich';
const focusables=()=>[...root.querySelectorAll('button,a[href],input,select,textarea,[tabindex]')].filter(e=>e.tabIndex>=0&&!e.disabled&&!e.closest('[inert]')&&e.getClientRects().length);
const position=()=>{
  if(!shown()) return;
  const r=positionTrigger.getBoundingClientRect(),gap=Number(root.dataset.offset||4),rtl=getComputedStyle(trigger).direction==='rtl';
  const vw=visualViewport?.width||innerWidth,vh=visualViewport?.height||innerHeight,ox=visualViewport?.offsetLeft||0,oy=visualViewport?.offsetTop||0;
  root.style.maxWidth=Math.max(0,vw-16)+'px';
  if(kind==='select'||kind==='combobox')root.style.width=Math.min(r.width,vw-16)+'px';
  root.style.maxHeight=Math.min(heightLimit,Math.max(0,vh-16))+'px';
  let side=root.dataset.side||'bottom';const natural=root.offsetHeight;
  const below=Math.max(0,oy+vh-r.bottom-gap-8),above=Math.max(0,r.top-oy-gap-8);
  if(side==='bottom'&&natural>below&&above>below)side='top';
  else if(side==='top'&&natural>above&&below>above)side='bottom';
  if(side==='bottom'||side==='top')root.style.maxHeight=Math.min(heightLimit,Math.max(1,side==='bottom'?below:above))+'px';
  let w=root.offsetWidth,h=root.offsetHeight,x,y;
  const align=root.dataset.align||'center',shift=Number(root.dataset.alignOffset||0)*(rtl?-1:1);
  if(side==='top'||side==='bottom'){
    x=align==='center'?r.left+(r.width-w)/2:((align==='start')!==rtl?r.left:r.right-w);x+=shift;
    y=side==='top'?r.top-h-gap:r.bottom+gap;
  }else{
    x=side==='left'?r.left-w-gap:r.right+gap;
    if(x<ox+8){side='right';x=r.right+gap;}else if(x+w>ox+vw-8){side='left';x=r.left-w-gap;}
    y=align==='start'?r.top:align==='end'?r.bottom-h:r.top+(r.height-h)/2;
  }
  const px=Math.max(ox+8,Math.min(x,ox+vw-w-8)),py=Math.max(oy+8,Math.min(y,oy+vh-h-8));
  root.style.left=px+'px';root.style.top=py+'px';root.dataset.actualSide=side;
  // Base UI: aligned arrowless surfaces grow from their aligned edge. After
  // collision shifting, use the anchor centre. Include the anchor-side gap.
  const vertical=side==='top'||side==='bottom',shifted=Math.abs(vertical?px-x:py-y)>1;
  const cross=align!=='center'&&!shifted?(vertical?((align==='start')!==rtl?0:w):(align==='start'?0:h)):
    Math.max(0,Math.min(vertical?w:h,vertical?r.left+r.width/2-px:r.top+r.height/2-py));
  let axis=side==='top'?h+gap:side==='bottom'?-gap:side==='left'?w+gap:-gap;
  if(Math.abs(vertical?py-y:px-x)>gap)axis=vertical?r.top+r.height/2-py:r.left+r.width/2-px;
  root.style.transformOrigin=vertical?`${cross}px ${axis}px`:`${axis}px ${cross}px`;
};
const track=()=>{frame=0;if(disposed)return;position();if(animations.some(a=>a.playState==='running'))frame=requestAnimationFrame(track);};
const close=(restore=false,notify=true,animate=true)=>{
  if(kind==='tooltip'&&shown())window.__m3TooltipUntil=Date.now()+400;
  clearTimeout(timer);clearTimeout(closeTimer);opened=false;
  const previous={opacity:getComputedStyle(root).opacity,transform:getComputedStyle(root).transform};const g=stop();
  if(!automatic)own('aria-expanded','false');
  root.setAttribute('aria-hidden','true');root.setAttribute('inert','');root.dataset.phase='closing';
  const finish=()=>{if(disposed||g!==generation)return;if(shown())root.hidePopover();root.dataset.phase='closed';animations.forEach(a=>a.cancel());animations=[];};
  if(!shown()||media.matches||!animate)finish();
  else {const a=root.animate([previous,{opacity:0,transform:kind==='select'||kind==='combobox'?'scaleY(.9)':'scale(.95)'}],{duration:kind==='select'||kind==='combobox'?120:150,easing:'cubic-bezier(.3,0,.8,.15)',fill:'both'});animations=[a];a.finished.then(finish).catch(()=>{});}
  if(restore)trigger.focus({preventScroll:true});
  if(!automatic&&notify)send(false);
};
const open=()=>{
  clearTimeout(timer);clearTimeout(closeTimer);
  if(disposed||root.dataset.disabled==='true'||trigger.disabled||trigger.getAttribute('aria-disabled')==='true')return;
  if(opened&&shown()&&root.dataset.phase!=='closing')return;
  if(kind==='tooltip')for(const entry of Object.values(window.__m3Anchors))if(entry.kind==='tooltip'&&entry.id!==id)entry.close(false,false,false);
  const prior=shown()?{opacity:getComputedStyle(root).opacity,transform:getComputedStyle(root).transform}:null,g=stop();
  opened=true;root.removeAttribute('inert');if(kind!=='hover')root.removeAttribute('aria-hidden');
  if(!shown())root.showPopover();root.dataset.phase='opening';position();
  if(!automatic)own('aria-expanded','true');
  if(kind==='select'||kind==='combobox')trigger.focus({preventScroll:true});
  if(!media.matches){
    const menu=kind==='select'||kind==='combobox';
    animations=[root.animate([{opacity:prior?.opacity||0},{opacity:1}],{duration:duration(menu?'fast-effects':'default-effects'),easing:token(menu?'fast-effects':'default-effects'),fill:'both'}),root.animate([{transform:prior?.transform||(menu?'scaleY(.6)':'scale(.85)')},{transform:menu?'scaleY(1)':'scale(1)'}],{duration:duration(menu?'default-spatial':'fast-spatial'),easing:token(menu?'default-spatial':'fast-spatial'),fill:'both'})];
    Promise.all(animations.map(a=>a.finished)).then(()=>{if(!disposed&&g===generation){root.dataset.phase='open';animations.forEach(a=>a.cancel());animations=[];}}).catch(()=>{});
  }else root.dataset.phase='open';
  if(interactive())(focusables()[0]||root).focus({preventScroll:true});
  if(!frame)frame=requestAnimationFrame(track);
};
const sync=()=>{if(automatic){if(root.dataset.disabled==='true')close(false,false,false);return;}if(root.dataset.open==='true')open();else if(opened||shown()&&root.dataset.phase!=='closing')close(root.contains(document.activeElement),false);};
const laterOpen=()=>{clearTimeout(closeTimer);if(suppressed)return;clearTimeout(timer);
  const warm=kind==='tooltip'&&(Date.now()<(window.__m3TooltipUntil||0)||Object.values(window.__m3Anchors).some(p=>p.kind==='tooltip'&&p.root.matches(':popover-open')));
  if(warm)open();else timer=setTimeout(open,Number(root.dataset.delay||500));
};
const laterClose=()=>{clearTimeout(timer);clearTimeout(closeTimer);closeTimer=setTimeout(()=>close(false,false),Number(root.dataset.closeDelay||0));};
const enter=e=>{if(e.pointerType==='mouse'&&automatic){suppressed=false;laterOpen();}};
const leave=e=>{if(e.pointerType==='mouse'&&automatic){suppressed=false;laterClose();}};
const focus=()=>{if(automatic&&!suppressed&&!touch)open();};
const blur=e=>{suppressed=false;if(automatic&&!root.contains(e.relatedTarget)&&e.relatedTarget!==trigger)laterClose();};
const down=e=>{
  if(kind==='tooltip'&&e.pointerType==='touch'){touch={x:e.clientX,y:e.clientY};consumedTouch=false;clearTimeout(timer);timer=setTimeout(()=>{open();consumedTouch=shown();},Number(root.dataset.delay||500));}
  else if(kind==='tooltip'){suppressed=true;close(false,false);}
};
const move=e=>{if(touch&&Math.hypot(e.clientX-touch.x,e.clientY-touch.y)>10){touch=null;close(false,false,false);consumedTouch=false;}};
const end=()=>{if(touch){touch=null;clearTimeout(timer);if(consumedTouch)laterClose();}};
const cancel=()=>{touch=null;consumedTouch=false;if(kind==='tooltip')close(false,false,false);};
const clicked=e=>{if(consumedTouch){e.preventDefault();e.stopPropagation();consumedTouch=false;return;}if(kind==='tooltip'){suppressed=true;close(false,false);}};
const key=e=>{
  if(e.defaultPrevented||!shown()||root.dataset.phase==='closing')return;
  if(e.key==='Escape'&&(root.contains(e.target)||e.target===trigger)){e.preventDefault();e.stopPropagation();suppressed=true;close(interactive());}
  else if(e.key==='Tab'&&interactive()&&root.contains(e.target)){
    const list=focusables();if(e.shiftKey&&document.activeElement===list[0]){e.preventDefault();close(true);}
    else if(!e.shiftKey&&document.activeElement===list.at(-1)){
      const scope=root.closest('dialog[open]')||document;
      const all=[...scope.querySelectorAll('a[href],button,input,select,textarea,[tabindex]')].filter(el=>el.tabIndex>=0&&!el.disabled&&!root.contains(el)&&!el.closest('[inert],[aria-hidden="true"]')&&el.getClientRects().length);
      const next=all[all.indexOf(trigger)+1];if(next){e.preventDefault();close(false);next.focus();}
    }
  }
};
const outside=e=>{if(!automatic&&shown()&&!root.contains(e.target)&&!positionTrigger.contains(e.target))close(false);};
const focusOutside=e=>{if(!automatic&&shown()&&e.target!==document.body&&!root.contains(e.target)&&!positionTrigger.contains(e.target))close(false);};
const fieldClick=e=>{if(kind==='combobox'&&!trigger.disabled&&!e.target.closest('button,input,select,a[href]')){trigger.focus({preventScroll:true});send(true);}};
const reduced=()=>{if(media.matches)animations.forEach(a=>{if(a.playState==='running')a.finish();});};
const scrolled=()=>{if(touch)cancel();position();};
const changed=new MutationObserver(()=>{sync();position();});changed.observe(root,{attributes:true,attributeFilter:['data-open','data-disabled','data-side','data-align','data-offset'],childList:true,subtree:true});
const resize=new ResizeObserver(position);resize.observe(root);resize.observe(positionTrigger);
if(kind==='tooltip'){const descriptions=new Set((trigger.getAttribute('aria-describedby')||'').split(/\s+/).filter(Boolean));descriptions.add(id);own('aria-describedby',[...descriptions].join(' '));}
if(!automatic){own('aria-haspopup',interactive()?'dialog':'listbox');own('aria-controls',id);own('aria-expanded','false');}
trigger.addEventListener('pointerenter',enter);trigger.addEventListener('pointerleave',leave);trigger.addEventListener('focus',focus);trigger.addEventListener('blur',blur);trigger.addEventListener('pointerdown',down);trigger.addEventListener('pointermove',move);trigger.addEventListener('pointerup',end);trigger.addEventListener('pointercancel',cancel);trigger.addEventListener('click',clicked,true);
positionTrigger.addEventListener('click',fieldClick);
root.addEventListener('pointerenter',enter);root.addEventListener('pointerleave',leave);
document.addEventListener('keydown',key);document.addEventListener('pointerdown',outside,true);document.addEventListener('focusin',focusOutside);
window.addEventListener('resize',position);window.addEventListener('scroll',scrolled,true);visualViewport?.addEventListener('resize',position);media.addEventListener('change',reduced);
window.__m3Anchors[id]={id,kind,root,trigger,open,close,position,cleanup:()=>{
  disposed=true;stop();clearTimeout(timer);clearTimeout(closeTimer);cancelAnimationFrame(frame);changed.disconnect();resize.disconnect();
  trigger.removeEventListener('pointerenter',enter);trigger.removeEventListener('pointerleave',leave);trigger.removeEventListener('focus',focus);trigger.removeEventListener('blur',blur);trigger.removeEventListener('pointerdown',down);trigger.removeEventListener('pointermove',move);trigger.removeEventListener('pointerup',end);trigger.removeEventListener('pointercancel',cancel);trigger.removeEventListener('click',clicked,true);
  positionTrigger.removeEventListener('click',fieldClick);
  root.removeEventListener('pointerenter',enter);root.removeEventListener('pointerleave',leave);document.removeEventListener('keydown',key);document.removeEventListener('pointerdown',outside,true);document.removeEventListener('focusin',focusOutside);
  window.removeEventListener('resize',position);window.removeEventListener('scroll',scrolled,true);visualViewport?.removeEventListener('resize',position);media.removeEventListener('change',reduced);
  for(const [name,value]of original){if(value==null)trigger.removeAttribute(name);else trigger.setAttribute(name,value);}
  if(shown())root.hidePopover();delete window.__m3Anchors[id];
}};
sync();
