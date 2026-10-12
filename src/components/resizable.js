const root=document.getElementById(__ID__);
if(!root||root.dataset.bound)return;
root.dataset.bound='true';
const panels=[...root.children].filter(e=>e.classList.contains('m3-resizable__panel'));
const handles=[...root.children].filter(e=>e.classList.contains('m3-resizable__handle'));
const initial=panels.map(p=>Number(p.dataset.default));
const sum=initial.reduce((a,b)=>a+b,0);
let sizes=initial.map(v=>sum?v/sum*100:100/panels.length),drag=null;
const restored=new Map(),listeners=[];
const on=(el,name,fn)=>{el.addEventListener(name,fn);listeners.push(()=>el.removeEventListener(name,fn));};
const media=matchMedia('(max-width:839px)');
const direction=()=>{root.dataset.axis=root.dataset.adaptive==='true'&&media.matches?'vertical':root.dataset.direction;};
direction();
const horizontal=()=>root.dataset.axis==='horizontal';
const blocked=(handle,i)=>root.dataset.disabled==='true'||handle.dataset.disabled==='true'||panels[i]?.dataset.disabled==='true'||panels[i+1]?.dataset.disabled==='true'||root.dataset.invalid==='true';
const minimum=i=>Number(panels[i].dataset.min),maximum=i=>Number(panels[i].dataset.max),collapsible=i=>panels[i].dataset.collapsible==='true';
const lower=i=>collapsible(i)?0:minimum(i);
const limits=i=>{const pair=sizes[i]+sizes[i+1];return {pair,min:Math.max(lower(i),pair-maximum(i+1)),max:Math.min(maximum(i),pair-lower(i+1))};};
const publish=()=>dioxus.send(sizes.map(v=>Math.round(v*1000)/1000));
const apply=()=>{
 panels.forEach((p,i)=>{p.style.flexGrow=String(sizes[i]);p.dataset.size=sizes[i].toFixed(3);const collapsed=sizes[i]<.001;p.dataset.collapsed=String(collapsed);p.inert=collapsed;});
 handles.forEach((h,i)=>{if(!panels[i+1]){h.hidden=true;return;}const l=limits(i),disabled=blocked(h,i);h.setAttribute('aria-orientation',horizontal()?'vertical':'horizontal');h.setAttribute('aria-controls',panels[i].id);h.setAttribute('aria-valuemin',l.min.toFixed(3));h.setAttribute('aria-valuemax',l.max.toFixed(3));h.setAttribute('aria-valuenow',sizes[i].toFixed(3));h.setAttribute('aria-valuetext',`${Math.round(sizes[i])}%`);h.setAttribute('aria-disabled',String(disabled));h.tabIndex=disabled?-1:0;});
};
// Initial constraints: redistribute excess between flexible panels without changing total100.
for(let i=0;i<sizes.length;i++)sizes[i]=Math.max(lower(i),Math.min(maximum(i),sizes[i]));
for(let round=0;round<panels.length+1;round++){
 const delta=100-sizes.reduce((a,b)=>a+b,0);if(Math.abs(delta)<.001)break;
 const adjustable=panels.map((p,i)=>i).filter(i=>delta>0?sizes[i]<maximum(i):sizes[i]>lower(i));if(!adjustable.length){root.dataset.invalid='true';break;}
 for(const i of adjustable)sizes[i]=Math.max(lower(i),Math.min(maximum(i),sizes[i]+delta/adjustable.length));
}
const resize=(i,requested)=>{
 const l=limits(i);let value=Math.max(l.min,Math.min(l.max,requested));
 if(collapsible(i)&&value<minimum(i))value=value<minimum(i)/2?0:minimum(i);
 const other=l.pair-value;
 if(collapsible(i+1)&&other<minimum(i+1))value=other<minimum(i+1)/2?l.pair:l.pair-minimum(i+1);
 value=Math.max(l.min,Math.min(l.max,value));sizes[i]=value;sizes[i+1]=l.pair-value;apply();
};
handles.forEach((h,i)=>{
 const point=e=>horizontal()?e.clientX:e.clientY;
 on(h,'pointerdown',e=>{if(e.button!==0||blocked(h,i))return;e.preventDefault();h.focus();h.setPointerCapture(e.pointerId);const total=panels.reduce((n,p)=>n+(horizontal()?p.getBoundingClientRect().width:p.getBoundingClientRect().height),0);if(!total)return;drag={h,i,pointer:e.pointerId,start:point(e),sizes:[...sizes],total};root.dataset.resizing='true';});
 on(h,'pointermove',e=>{if(!drag||drag.h!==h||drag.pointer!==e.pointerId)return;const sign=horizontal()&&getComputedStyle(root).direction==='rtl'?-1:1;resize(i,drag.sizes[i]+sign*(point(e)-drag.start)/drag.total*100);});
 const finish=(e,cancel)=>{if(!drag||drag.h!==h||drag.pointer!==e.pointerId)return;if(cancel){sizes=drag.sizes;apply();}drag=null;root.dataset.resizing='false';if(h.hasPointerCapture(e.pointerId))h.releasePointerCapture(e.pointerId);if(!cancel)publish();};
 on(h,'pointerup',e=>finish(e,false));on(h,'pointercancel',e=>finish(e,true));on(h,'lostpointercapture',e=>finish(e,true));
 on(h,'keydown',e=>{
  if(blocked(h,i))return;
  if(e.key==='Escape'&&drag){e.preventDefault();finish({pointerId:drag.pointer},true);return;}
  const l=limits(i),step=e.shiftKey?10:1,rtl=horizontal()&&getComputedStyle(root).direction==='rtl';
  let next=null;
  if(e.key==='Home')next=l.min;else if(e.key==='End')next=l.max;
  else if(e.key==='Enter'&&collapsible(i)){if(sizes[i]>.001){restored.set(i,sizes[i]);next=0;}else next=restored.get(i)??Number(panels[i].dataset.default);}
  else if(horizontal()&&e.key==='ArrowLeft')next=sizes[i]+(rtl?step:-step);
  else if(horizontal()&&e.key==='ArrowRight')next=sizes[i]+(rtl?-step:step);
  else if(!horizontal()&&e.key==='ArrowUp')next=sizes[i]-step;
  else if(!horizontal()&&e.key==='ArrowDown')next=sizes[i]+step;
  if(next!==null){e.preventDefault();resize(i,next);publish();}
 });
 on(h,'dblclick',()=>{if(blocked(h,i))return;const pair=sizes[i]+sizes[i+1],ratio=initial[i]/Math.max(1,initial[i]+initial[i+1]);resize(i,pair*ratio);publish();});
});
const sync=()=>{if(drag){sizes=drag.sizes;const active=drag;drag=null;root.dataset.resizing='false';if(active.h.hasPointerCapture(active.pointer))active.h.releasePointerCapture(active.pointer);}direction();apply();};
on(media,'change',sync);
const observer=new MutationObserver(sync);
observer.observe(root,{attributes:true,subtree:true,attributeFilter:['data-disabled','data-direction','data-adaptive']});
window.__m3Resizable=window.__m3Resizable||{};
window.__m3Resizable[__ID__]=()=>{observer.disconnect();listeners.forEach(fn=>fn());delete window.__m3Resizable[__ID__];};
apply();publish();
