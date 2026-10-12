const root=document.getElementById(__ID__);
if(!root||root.dataset.bound)return;
root.dataset.bound='true';
const view=root.querySelector('.m3-scroll-area__viewport'),content=root.querySelector('.m3-scroll-area__content');
const bars=[...root.querySelectorAll('.m3-scroll-area__bar')];
const cleanups=[],timers={};let drag=null;
const on=(el,name,fn,options)=>{el.addEventListener(name,fn,options);cleanups.push(()=>el.removeEventListener(name,fn,options));};
const rtl=()=>getComputedStyle(view).direction==='rtl';
const position=axis=>axis==='y'?view.scrollTop:Math.abs(view.scrollLeft);
const setPosition=(axis,value)=>{if(axis==='y')view.scrollTop=value;else view.scrollLeft=rtl()?-value:value;};
const geometry=bar=>{
 const axis=bar.dataset.axis,vertical=axis==='y',client=vertical?view.clientHeight:view.clientWidth,full=vertical?view.scrollHeight:view.scrollWidth;
 const track=Math.max(0,(vertical?bar.clientHeight:bar.clientWidth)-2);
 const thumb=Math.min(track,Math.max(16,track*client/Math.max(full,1)));
 const max=Math.max(0,full-client),travel=Math.max(0,track-thumb);
 return {axis,vertical,thumb,max,travel,offset:max?position(axis)/max*travel:0};
};
const update=()=>{
 const x=view.scrollWidth>view.clientWidth+1,y=view.scrollHeight>view.clientHeight+1;
 root.dataset.overflowX=String(x);root.dataset.overflowY=String(y);
 for(const bar of bars){bar.dataset.overflow=String(bar.dataset.axis==='y'?y:x);const g=geometry(bar),thumb=bar.firstElementChild;thumb.style[g.vertical?'height':'width']=g.thumb+'px';thumb.style.transform=g.vertical?`translateY(${g.offset}px)`:`translateX(${rtl()?g.travel-g.offset:g.offset}px)`;}
};
on(view,'scroll',()=>{update();for(const bar of bars){const axis=bar.dataset.axis;bar.dataset.scrolling='true';clearTimeout(timers[axis]);timers[axis]=setTimeout(()=>bar.dataset.scrolling='false',500);}});
for(const bar of bars){
 const coordinate=e=>bar.dataset.axis==='y'?e.clientY:e.clientX;
 on(bar,'pointerdown',e=>{
  if(e.button!==0)return;const g=geometry(bar);if(!g.max||!g.travel)return;
  e.preventDefault();view.focus({preventScroll:true});bar.setPointerCapture(e.pointerId);bar.dataset.dragging='true';
  const rect=bar.getBoundingClientRect(),point=coordinate(e)-(g.vertical?rect.top:rect.left)-1;
  if(e.target!==bar.firstElementChild){const offset=Math.max(0,Math.min(g.travel,point-g.thumb/2));setPosition(g.axis,(g.axis==='x'&&rtl()?g.travel-offset:offset)/g.travel*g.max);update();}
  drag={bar,pointer:e.pointerId,origin:coordinate(e),start:position(g.axis),g};
 });
 on(bar,'pointermove',e=>{if(!drag||drag.bar!==bar||drag.pointer!==e.pointerId)return;const g=drag.g,sign=g.axis==='x'&&rtl()?-1:1;setPosition(g.axis,Math.max(0,Math.min(g.max,drag.start+sign*(coordinate(e)-drag.origin)/g.travel*g.max)));update();});
 const finish=e=>{if(!drag||drag.bar!==bar||drag.pointer!==e.pointerId)return;drag=null;bar.dataset.dragging='false';if(bar.hasPointerCapture(e.pointerId))bar.releasePointerCapture(e.pointerId);};
 on(bar,'pointerup',finish);on(bar,'pointercancel',finish);on(bar,'lostpointercapture',finish);
 on(bar,'wheel',e=>{if(e.ctrlKey)return;const g=geometry(bar),physical=g.vertical?e.deltaY:e.deltaX,delta=g.axis==='x'&&rtl()?-physical:physical,current=position(g.axis);if((current<=0&&delta<0)||(current>=g.max&&delta>0)||!delta)return;e.preventDefault();setPosition(g.axis,current+delta);},{passive:false});
}
const resize=new ResizeObserver(update);resize.observe(view);resize.observe(content);
const mutations=new MutationObserver(update);mutations.observe(content,{childList:true,subtree:true,characterData:true});
window.__m3ScrollAreas=window.__m3ScrollAreas||{};
window.__m3ScrollAreas[__ID__]=()=>{resize.disconnect();mutations.disconnect();cleanups.forEach(fn=>fn());Object.values(timers).forEach(clearTimeout);delete window.__m3ScrollAreas[__ID__];};
update();
