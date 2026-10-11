// Owned dialog input/dial runtime. No native HTML time popup or number steppers.
const id=__ID__,root=document.getElementById(id);if(!root)return;
window.__m3Times??={};window.__m3Times[id]?.();
let drag=null,frame=0,previousMode,previousField,labels,labelField,labelFade;
const motion=matchMedia('(prefers-reduced-motion: reduce)');
const send=(...values)=>dioxus.send(values.map(String));
const size=()=>{
 const width=visualViewport?.width||innerWidth,height=visualViewport?.height||innerHeight;
 send('viewport',width>=620&&height<600?1:0,height<600&&width<620||height<410||width<320?1:0,width>=620?1:0);
};
const selectedTick=()=>root.querySelector('.m3-time-picker__dial [data-selected=true]');
const sync=()=>{
 frame=0;if(root.dataset.open!=='true'&&drag)cancel();
 const mode=root.dataset.mode,field=root.dataset.field;
 const nextLabels=root.querySelector('.m3-time-picker__labels');
 if(nextLabels!==labels||field!==labelField){
  labelFade?.cancel();labels=nextLabels;labelField=field;
  if(labels&&!motion.matches){
   const style=getComputedStyle(labels);
   const duration=style.getPropertyValue('--m3-time-label-duration').trim();
   const milliseconds=parseFloat(duration)*(duration.endsWith('ms')?1:1000);
   labelFade=labels.animate([{opacity:0},{opacity:1}],{duration:Number.isFinite(milliseconds)?Math.max(0,milliseconds):200,easing:style.getPropertyValue('--m3-time-label-easing').trim()||'ease'});
  }
 }
 if(root.dataset.open==='true'&&root.closest('dialog')?.open){
  if(mode!==previousMode&&mode==='input'){
   const input=root.querySelector('input[data-part="'+(field==='hours'?'0':'1')+'"]');
   input?.focus({preventScroll:true});input?.select();
  }else if(field!==previousField&&root.contains(document.activeElement)&&document.activeElement.closest('.m3-time-picker__dial')){
   selectedTick()?.focus({preventScroll:true});
  }
 }
 previousMode=mode;previousField=field;
};
const request=()=>{cancelAnimationFrame(frame);frame=requestAnimationFrame(sync);};
const position=e=>{
 const dial=root.querySelector('.m3-time-picker__dial');if(!dial)return null;
 const r=dial.getBoundingClientRect(),dx=(e.clientX-r.left-r.width/2)*256/r.width,dy=(e.clientY-r.top-r.height/2)*256/r.height;
 const degrees=(Math.atan2(dx,-dy)*180/Math.PI+360)%360;
 if(root.dataset.field==='minutes')return Math.round(degrees/6)%60;
 const step=Math.round(degrees/30)%12;
 const inner=root.dataset.hour24==='true'&&Math.hypot(dx,dy)<128*.62;
 return inner?(step===0?0:step+12):(step===0?12:step);
};
const down=e=>{
 const dial=e.target.closest('.m3-time-picker__dial');if(!dial||e.button!==0)return;
 e.preventDefault();selectedTick()?.focus({preventScroll:true});
 drag={pointer:e.pointerId,dial,hour:root.dataset.hours,minute:root.dataset.minutes};
 dial.dataset.dragging='true';dial.setPointerCapture(e.pointerId);
 const n=position(e);if(n!==null)send('dial',n);
};
const move=e=>{if(!drag||drag.pointer!==e.pointerId)return;e.preventDefault();const n=position(e);if(n!==null)send('dial',n);};
const up=e=>{
 if(!drag||drag.pointer!==e.pointerId)return;
 const n=position(e),dial=drag.dial;drag=null;delete dial.dataset.dragging;
 if(dial.hasPointerCapture(e.pointerId))dial.releasePointerCapture(e.pointerId);
 if(n!==null)send('finish',n);
};
const cancel=e=>{
 if(!drag||e&&drag.pointer!==e.pointerId)return;
 const prior=drag;drag=null;delete prior.dial.dataset.dragging;
 if(prior.dial.hasPointerCapture(prior.pointer))prior.dial.releasePointerCapture(prior.pointer);
 send('cancel',prior.hour,prior.minute);
};
const click=e=>{const tick=e.target.closest('.m3-time-picker__tick');if(tick&&e.detail===0)send('finish',tick.dataset.value);};
const key=e=>{
 if(e.isComposing||e.altKey||e.ctrlKey||e.metaKey)return;
 const period=e.target.closest('[data-period]');
 if(period&&['ArrowLeft','ArrowRight','ArrowUp','ArrowDown','Home','End'].includes(e.key)){
  e.preventDefault();let pm=e.key==='End'||e.key!=='Home'&&period.dataset.period==='am';
  let target=root.querySelector('[data-period="'+(pm?'pm':'am')+'"]');target?.click();target?.focus();return;
 }
 const tick=e.target.closest('.m3-time-picker__tick');if(!tick)return;
 const hours=root.dataset.field==='hours',hour24=root.dataset.hour24==='true',max=hours?(hour24?23:12):59,min=hours&&!hour24?1:0;
 let current=hours?Number(root.dataset.hours):Number(root.dataset.minutes);
 if(hours&&!hour24)current=current%12||12;
 let delta;if(['ArrowRight','ArrowUp'].includes(e.key))delta=1;else if(['ArrowLeft','ArrowDown'].includes(e.key))delta=-1;
 else if(e.key==='PageUp')delta=hours?1:5;else if(e.key==='PageDown')delta=hours?-1:-5;
 else if(e.key==='Home')delta=min-current;else if(e.key==='End')delta=max-current;else return;
 e.preventDefault();const length=max-min+1,n=((current-min+delta)%length+length)%length+min;send('dial',n);
 requestAnimationFrame(()=>selectedTick()?.focus({preventScroll:true}));
};
const blur=e=>{const input=e.target.closest('input[data-part]');if(input)send('commit',input.dataset.part);};
const inputKey=e=>{if(e.key==='Enter'&&!e.isComposing&&e.target.matches('input[data-part]')){e.preventDefault();send('commit',e.target.dataset.part);}};
const observer=new MutationObserver(request);observer.observe(root,{subtree:true,childList:true,attributes:true,attributeFilter:['data-angle','data-radius','data-mode','data-field','data-hours','data-minutes','data-open']});
root.addEventListener('pointerdown',down);root.addEventListener('pointermove',move);root.addEventListener('pointerup',up);root.addEventListener('pointercancel',cancel);root.addEventListener('lostpointercapture',cancel);root.addEventListener('click',click);root.addEventListener('keydown',key);root.addEventListener('keydown',inputKey);root.addEventListener('focusout',blur);
const reduce=()=>{if(motion.matches)labelFade?.finish();};motion.addEventListener('change',reduce);
window.addEventListener('resize',size);visualViewport?.addEventListener('resize',size);
size();request();
window.__m3Times[id]=()=>{cancel();cancelAnimationFrame(frame);labelFade?.cancel();motion.removeEventListener('change',reduce);observer.disconnect();for(const [type,fn]of [['pointerdown',down],['pointermove',move],['pointerup',up],['pointercancel',cancel],['lostpointercapture',cancel],['click',click],['keydown',key],['keydown',inputKey],['focusout',blur]])root.removeEventListener(type,fn);window.removeEventListener('resize',size);visualViewport?.removeEventListener('resize',size);delete window.__m3Times[id];};
