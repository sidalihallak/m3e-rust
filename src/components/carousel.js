(async () => {
 const id=__ID__,root=document.getElementById(id);if(!root)return;
 window.__m3Carousels??={};window.__m3Carousels[id]?.();
 const track=root.querySelector('.m3-carousel__track'),range=root.querySelector('.m3-carousel__range'),layer=root.querySelector('.m3-carousel__layer'),snaps=root.querySelector('.m3-carousel__snap-points');if(!track)return;
 const reduced=matchMedia('(prefers-reduced-motion: reduce)');let nodes=[],geo=null,step=1,max=0,current=0,pendingFocus=null,programmatic=false,dead=false,frame=0,last='',waitResolve;
 const vertical=()=>root.dataset.orientation==='vertical',rtl=()=>!vertical()&&getComputedStyle(root).direction==='rtl';
 const scroll=()=>Math.abs(vertical()?track.scrollTop:track.scrollLeft);
 // Material's boundary treatment shifts keylines, rather than blending item
 // layouts. Logical centers stay one large-item step apart; masks map them to
 // the physical centers of the current keyline state.
 function keylines(sizes){
  const gap=Number(root.dataset.gap),focal=sizes.findIndex(v=>v===geo.large);
  let edge=0;const keys=sizes.map(size=>{const key={size,offset:edge+size/2};edge+=size+gap;return key;});
  const focalOffset=keys[focal].offset;
  keys.forEach((key,i)=>key.loc=focalOffset+(i-focal)*step);
  const first=keys[0],last=keys.at(-1);
  return [{size:geo.small,offset:-geo.small/2-gap,loc:first.loc-step},...keys,{size:0,offset:track.clientWidth+gap,loc:last.loc+step}];
 }
 function shiftedKeys(offset){
  const states=geo.states,range=states[0].at(-1).loc-states.at(-1).at(-1).loc;
  const shift=Math.max(0,Math.min(range,offset-(geo.limit-range)));
  let previous=states[0],distance=0;
  for(let i=1;i<states.length;i++){
   const next=states[i],part=previous.at(-1).loc-next.at(-1).loc;
   if(shift<=distance+part||i===states.length-1){const t=part?Math.max(0,Math.min(1,(shift-distance)/part)):1;return previous.map((key,j)=>({loc:key.loc+(next[j].loc-key.loc)*t,offset:key.offset+(next[j].offset-key.offset)*t,size:key.size+(next[j].size-key.size)*t}));}
   distance+=part;previous=next;
  }
  return previous;
 }
 function maskedItem(center,keys){
  let a=keys[0],b=keys[1];
  for(let i=1;i<keys.length;i++){b=keys[i];if(center<=b.loc)break;a=b;}
  const t=Math.max(0,Math.min(1,(center-a.loc)/(b.loc-a.loc||1)));
  const size=a.size+(b.size-a.size)*t,offset=a.offset+(b.offset-a.offset)*t;
  return {size,x:offset-size/2};
 }
 function indexAt(offset){if(geo&&max&&offset>=(Math.max(0,max-1)*step+geo.limit)/2)return max;return Math.min(max,Math.max(0,Math.round(offset/step)));}
 function notify(){const next=JSON.stringify([current,max]);if(last!==next){last=next;dioxus.send([current,max]);}}
 function visible(node,value){node.inert=!value;node.setAttribute('aria-hidden',String(!value));}
 function paint(){frame=0;if(dead)return;
  current=indexAt(scroll());root.dataset.current=String(current);
  if(geo){
   const w=track.clientWidth,raw=scroll();
   // Native scroll positions are quantized to device pixels. Use the exact
   // terminal keyline within that precision instead of a fractional mask.
   const offset=Math.abs(raw-geo.limit)<=1/(window.devicePixelRatio||1)?geo.limit:Math.min(raw,geo.limit);
   const keys=shiftedKeys(offset);
   nodes.forEach((node,i)=>{
    const item=maskedItem(geo.large/2+i*step-offset,keys),size=item.size,position=item.x;
    node.style.width=size+'px';node.style.setProperty('--m3-carousel-mask-inset',Math.max(0,(geo.large-size)/2)+'px');
    node.style.transform=`translateX(${rtl()?w-position-size:position}px)`;
    const inView=size>=1&&position+size>0&&position<w;node.style.visibility=inView?'visible':'hidden';
    node.dataset.size=size>=geo.large-1?'large':size>geo.small+1?'medium':'small';visible(node,size>=48&&inView);
   });
  }
  else{const t=track.getBoundingClientRect();nodes.forEach(node=>{const r=node.getBoundingClientRect();visible(node,vertical()?r.bottom>t.top+1&&r.top<t.bottom-1:r.right>t.left+1&&r.left<t.right-1);});}
  if(pendingFocus!==null){const node=nodes[pendingFocus];if(node&&!node.inert){programmatic=true;node.focus({preventScroll:true});programmatic=false;pendingFocus=null;}}
  notify();
 }
 const schedule=()=>{if(!frame)frame=requestAnimationFrame(paint);};
 function measure(){
  if(dead)return;nodes=Array.from(layer.querySelectorAll('.m3-carousel__item'));if(!nodes.length){max=current=0;geo=null;notify();return;}const kind=root.dataset.variant,standard=kind==='standard',dynamic=!standard&&['multi-browse','hero'].includes(kind)&&!reduced.matches,w=track.clientWidth,h=track.clientHeight;
  if(!w||!h)return;
  const old=indexAt(scroll()),gap=parseFloat(getComputedStyle(root).getPropertyValue('--m3-carousel-gap'))||0,pref=Number(root.dataset.itemWidth)||186;root.dataset.gap=String(gap);root.dataset.dynamic=String(dynamic);root.dataset.reduced=String(reduced.matches);
  geo=null;const count=nodes.length;
  if(dynamic){if(kind==='hero'){const large=Math.max(56,w-56-gap);geo={big:1,large,medium:56,small:56};}else{const big=Math.max(1,Math.floor((w-84-gap)/(pref+gap))),large=Math.max(56,(w-84-(big+1)*gap)/(big+.5));geo={big,large,medium:(large+56)/2,small:56};}
   step=geo.large+gap;
   const initial=Array(geo.big).fill(geo.large).concat(kind==='hero'?[geo.small]:[geo.medium,geo.small]);
   geo.states=[keylines(initial)];let sizes=initial.slice();
   for(let n=0;n<(kind==='hero'?1:2);n++){const moving=sizes.pop();sizes.splice(n,0,moving);geo.states.push(keylines(sizes));}
   geo.limit=Math.max(0,count*step-gap-w);max=Math.ceil(geo.limit/step);
   // Fold a terminal snap closer than one inter-item gap into the edge snap.
   // Otherwise rounding can add a second tiny size adjustment at rest.
   if(max>1&&geo.limit-(max-1)*step<gap)max--;
   range.style.width=(w+geo.limit)+'px';range.style.height='100%';layer.style.width=w+'px';layer.style.height='100%';root.style.setProperty('--m3-carousel-content-width',geo.large+'px');
  }else{
   const per=Math.max(1,Number(root.dataset.perView)||1),size=vertical()?(h-(per-1)*gap)/per:standard?(w-(per-1)*gap)/per:(kind==='hero'&&reduced.matches?Math.max(56,w-40):Math.min(pref,w));
   step=Math.max(1,size+gap);max=Math.max(0,count-(standard?per:1));if(!vertical())max=Math.min(max,Math.ceil(Math.max(0,count*size+(count-1)*gap-w)/step));
   range.style.width='';range.style.height='';layer.style.width='';layer.style.height='';root.style.setProperty('--m3-carousel-content-width','100%');
   nodes.forEach(node=>{node.style.width=vertical()?'100%':size+'px';node.style.height=vertical()?size+'px':'100%';node.style.transform='none';node.style.visibility='visible';node.dataset.size='large';});
  }
  if(dynamic){snaps.replaceChildren(...Array.from({length:max+1},(_,i)=>{const span=document.createElement('span');span.style[rtl()?'right':'left']=(i===max?geo.limit:Math.min(i*step,geo.limit))+'px';return span;}));}else snaps.replaceChildren();
  go(Math.min(old,max),false,false);paint();
 }
 function go(index,focus=false,smooth=true){
  const i=Math.min(max,Math.max(0,index));if(focus)pendingFocus=Math.min(nodes.length-1,Math.max(0,index));
  const target=geo?(i===max?geo.limit:Math.min(i*step,geo.limit)):i*step,behavior=smooth&&!reduced.matches?'smooth':'instant';track.scrollTo(vertical()?{top:target,behavior}:{left:rtl()?-target:target,behavior});schedule();
 }
 function onKey(event){pointerFocus=false;if(root.dataset.disabled==='true')return;const item=event.target.closest('.m3-carousel__item');if(!item)return;let index=Number(item.dataset.index),delta=0;
  if(vertical()){if(event.key==='ArrowDown')delta=1;if(event.key==='ArrowUp')delta=-1;}else{if(event.key==='ArrowRight')delta=rtl()?-1:1;if(event.key==='ArrowLeft')delta=rtl()?1:-1;if(['ArrowDown','ArrowUp'].includes(event.key)){const next=root.querySelector('.m3-carousel__show-all')||root.querySelector('.m3-carousel__nav:not(:disabled)');if(next){event.preventDefault();next.focus();}return;}}
  if(event.key==='Home')index=0;else if(event.key==='End')index=nodes.length-1;else if(delta)index+=delta;else return;
  event.preventDefault();index=Math.min(nodes.length-1,Math.max(0,index));const direction=delta||(event.key==='End'?-1:1);while(index>=0&&index<nodes.length&&nodes[index].disabled)index+=direction;if(index<0||index>=nodes.length)return;go(index,true);
 }
 function focusIn(event){const item=event.target.closest('.m3-carousel__item');if(item&&pointerFocus){pointerFocus=false;return;}if(item&&!programmatic)go(Number(item.dataset.index),false);}
 function onClick(event){const nav=event.target.closest('[data-nav]');if(nav&&!nav.disabled)go(current+(nav.dataset.nav==='next'?1:-1));}
 let drag=null,suppressClick=false,pointerFocus=false;
 const down=event=>{pointerFocus=true;if(event.pointerType!=='mouse'||event.button!==0||root.dataset.disabled==='true')return;suppressClick=false;drag={id:event.pointerId,x:event.clientX,y:event.clientY,left:track.scrollLeft,top:track.scrollTop,active:false};};
 const move=event=>{if(!drag||event.pointerId!==drag.id)return;const dx=event.clientX-drag.x,dy=event.clientY-drag.y,delta=vertical()?dy:dx,other=vertical()?dx:dy;
  if(!drag.active){if(Math.abs(other)>Math.abs(delta)+6){suppressClick=true;drag=null;return;}if(Math.abs(delta)<6)return;drag.active=true;track.setPointerCapture(event.pointerId);track.style.scrollSnapType='none';root.dataset.dragging='true';}
  event.preventDefault();if(vertical())track.scrollTop=drag.top-dy;else track.scrollLeft=drag.left-dx;
 };
 const up=event=>{if(!drag||event.pointerId!==drag.id)return;const d=drag;drag=null;if(d.active){suppressClick=true;delete root.dataset.dragging;track.style.scrollSnapType='';if(track.hasPointerCapture(event.pointerId))track.releasePointerCapture(event.pointerId);if(event.type==='pointercancel'){track.scrollTo({left:d.left,top:d.top,behavior:'instant'});}else go(indexAt(scroll()),true);}};
 const suppress=event=>{if(suppressClick&&event.detail>0){event.preventDefault();event.stopImmediatePropagation();suppressClick=false;}else if(event.detail===0){suppressClick=false;}};
 track.addEventListener('pointerdown',down,{capture:true});track.addEventListener('pointermove',move);track.addEventListener('pointerup',up);track.addEventListener('pointercancel',up);root.addEventListener('click',suppress,{capture:true});
 const resize=new ResizeObserver(measure);resize.observe(track);
 const mutations=new MutationObserver(measure);mutations.observe(root,{attributes:true,attributeFilter:['data-item-width','data-per-view','data-variant','data-orientation'],childList:true});mutations.observe(layer,{childList:true});
 const motion=()=>measure();reduced.addEventListener('change',motion);track.addEventListener('scroll',schedule,{passive:true});root.addEventListener('keydown',onKey);root.addEventListener('focusin',focusIn);root.addEventListener('click',onClick);measure();
 window.__m3Carousels[id]=()=>{dead=true;cancelAnimationFrame(frame);resize.disconnect();mutations.disconnect();reduced.removeEventListener('change',motion);track.removeEventListener('scroll',schedule);root.removeEventListener('keydown',onKey);root.removeEventListener('focusin',focusIn);root.removeEventListener('click',onClick);track.removeEventListener('pointerdown',down,{capture:true});track.removeEventListener('pointermove',move);track.removeEventListener('pointerup',up);track.removeEventListener('pointercancel',up);root.removeEventListener('click',suppress,{capture:true});delete window.__m3Carousels[id];waitResolve?.();};
 await new Promise(resolve=>waitResolve=resolve);
})();
