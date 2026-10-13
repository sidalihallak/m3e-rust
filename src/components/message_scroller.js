const id=__ROOT_ID__,root=document.getElementById(id);
if(root){
 const viewport=root.querySelector('.m3-message-scroller-viewport'),content=root.querySelector('.m3-message-scroller-content'),jump=root.querySelector('.m3-message-scroller-jump'),spacer=root.querySelector('.m3-message-scroller-spacer');
 const reduced=matchMedia('(prefers-reduced-motion: reduce)');
 let initialized=false,following=false,jumping=false,paused=false,reported='',streamId='',first=null,anchor=null,anchorTop=0,frame=0,settleFrame=0,generation=0;
 const items=()=>Array.from(content.children).filter(e=>e.hasAttribute('data-message-id'));
 const max=()=>Math.max(0,viewport.scrollHeight-viewport.clientHeight);
 const threshold=()=>Number(root.dataset.threshold)||0;
 const stream=()=>root.dataset.streaming==='true'&&root.dataset.streamAnchor?items().find(e=>e.dataset.messageId===root.dataset.streamAnchor):null;
 const margin=()=>Number(root.dataset.margin)||0,peek=()=>Number(root.dataset.peek)||0;
 const itemTop=item=>viewport.scrollTop+item.getBoundingClientRect().top-viewport.getBoundingClientRect().top-viewport.clientTop-(parseFloat(getComputedStyle(viewport).paddingTop)||0);
 const endTarget=()=>{const item=stream();if(!item)return max();const available=viewport.clientHeight-peek()-margin()-(parseFloat(getComputedStyle(viewport).paddingTop)||0)*2;const tail=content.getBoundingClientRect().bottom-item.getBoundingClientRect().top;return tail>available?max():Math.max(0,Math.min(max(),itemTop(item)-peek()-margin()));};
 const endDistance=()=>stream()?Math.max(0,endTarget()-viewport.scrollTop):max()-viewport.scrollTop;
 const report=()=>{if(!viewport.clientHeight){const value=[false,false,true,true,[],null],signature=JSON.stringify(value);if(signature!==reported){reported=signature;dioxus.send(value);}return;}const r=viewport.getBoundingClientRect(),visible=items().filter(e=>{const b=e.getBoundingClientRect();return b.bottom>r.top&&b.top<r.bottom;}).map(e=>e.dataset.messageId),current=items().filter(e=>e.dataset.scrollAnchor==='true'&&e.getBoundingClientRect().top<=r.top+viewport.clientTop+(parseFloat(getComputedStyle(viewport).paddingTop)||0)+peek()+margin()+.5).at(-1)?.dataset.messageId||null;
 const value=[following,max()>0,viewport.scrollTop<=threshold(),endDistance()<=threshold(),visible,current];const signature=JSON.stringify(value);if(signature!==reported){reported=signature;dioxus.send(value);}};
 const remember=()=>{const vr=viewport.getBoundingClientRect();const visible=items().find(e=>e.getBoundingClientRect().bottom>vr.top);anchor=visible?.dataset.messageId||null;anchorTop=visible?visible.getBoundingClientRect().top-vr.top:0;first=items()[0]?.dataset.messageId||null;};
 const update=()=>{
  const near=jump.dataset.direction==='start'?viewport.scrollTop<=threshold():endDistance()<=threshold();
  if(near&&jump.contains(document.activeElement))viewport.focus({preventScroll:true});
  jump.dataset.active=String(!near);jump.inert=near;jump.setAttribute('aria-hidden',String(near));jump.querySelector('button').tabIndex=near?-1:0;
  root.dataset.following=String(following);root.dataset.scrollable=String(max()>0);
  remember();report();
 };
 const stop=()=>{++generation;cancelAnimationFrame(settleFrame);jumping=false;following=false;viewport.scrollTo({top:viewport.scrollTop,behavior:'instant'});update();};
 const scroll=(command,messageId)=>{
  if(command==='pause'){paused=true;stop();return;}
  if(command==='resume'){paused=false;following=root.dataset.autoScroll==='true';viewport.scrollTo({top:endTarget(),behavior:'instant'});update();return;}
  let top=command==='start'?0:endTarget();
  if(command==='message'||command==='peek'){const item=items().find(e=>e.dataset.messageId===messageId);if(!item)return;top=Math.max(0,itemTop(item)-margin());if(command==='peek'){const r=item.getBoundingClientRect(),v=viewport.getBoundingClientRect();top=r.top<v.top?top:r.bottom>v.bottom?viewport.scrollTop+r.bottom-v.bottom:viewport.scrollTop;paused=true;}}
  ++generation;const mine=generation;cancelAnimationFrame(settleFrame);jumping=true;following=command==='end'&&root.dataset.autoScroll==='true';if(command==='end')paused=false;
  viewport.scrollTo({top,behavior:reduced.matches?'instant':'smooth'});
  let last=-1,still=0;const settle=()=>{if(mine!==generation)return;still=Math.abs(viewport.scrollTop-last)<.5?still+1:0;last=viewport.scrollTop;if(still>=3){jumping=false;following=!paused&&root.dataset.autoScroll==='true'&&(stream()?Math.abs(endTarget()-viewport.scrollTop)<=threshold():max()-viewport.scrollTop<=threshold());update();}else settleFrame=requestAnimationFrame(settle);};settleFrame=requestAnimationFrame(settle);
 };
 const change=()=>{
  if(!viewport.clientHeight){report();return;}
  const list=items(),anchorItem=stream();
  const spare=anchorItem?Math.max(0,viewport.clientHeight-peek()-margin()-(parseFloat(getComputedStyle(viewport).paddingTop)||0)*2-(content.getBoundingClientRect().bottom-anchorItem.getBoundingClientRect().top)):0;
  spacer.style.height=`${spare}px`;
  if(streamId!==root.dataset.streamAnchor){streamId=root.dataset.streamAnchor;if(initialized&&following&&anchorItem)viewport.scrollTop=endTarget();}
  if(!initialized){initialized=true;let top=root.dataset.initial==='start'?0:max();if(root.dataset.initial==='last-anchor'){const item=list.filter(e=>e.dataset.scrollAnchor==='true').at(-1);if(item)top=Math.max(0,itemTop(item)-peek()-margin());}
   viewport.scrollTop=top;following=!paused&&root.dataset.autoScroll==='true'&&(stream()?Math.abs(endTarget()-viewport.scrollTop)<=threshold():max()-viewport.scrollTop<=threshold());
  }else if(!jumping){
   if(following&&root.dataset.autoScroll==='true')viewport.scrollTop=endTarget();
   else if(first&&first!==list[0]?.dataset.messageId&&root.dataset.preservePrepend==='true'){const preserved=list.find(e=>e.dataset.messageId===anchor);if(preserved)viewport.scrollTop+=preserved.getBoundingClientRect().top-viewport.getBoundingClientRect().top-anchorTop;}
  }
  update();
 };
 const schedule=()=>{cancelAnimationFrame(frame);frame=requestAnimationFrame(change);};
 const onscroll=()=>{if(!jumping)following=!paused&&root.dataset.autoScroll==='true'&&(stream()?Math.abs(endTarget()-viewport.scrollTop)<=threshold():max()-viewport.scrollTop<=threshold());update();};
 const key=e=>{if(e.target!==viewport)return;if(['ArrowDown','ArrowUp','PageDown','PageUp','Home','End',' '].includes(e.key))stop();};
 viewport.addEventListener('scroll',onscroll,{passive:true});viewport.addEventListener('wheel',stop,{passive:true});viewport.addEventListener('touchmove',stop,{passive:true});viewport.addEventListener('keydown',key);
 const resize=new ResizeObserver(schedule);resize.observe(viewport);resize.observe(content);
 const mutation=new MutationObserver(schedule);mutation.observe(content,{childList:true,subtree:true,characterData:true});
 const props=new MutationObserver(()=>{following=!paused&&root.dataset.autoScroll==='true'&&(stream()?Math.abs(endTarget()-viewport.scrollTop)<=threshold():max()-viewport.scrollTop<=threshold());schedule();});props.observe(root,{attributes:true,attributeFilter:['data-auto-scroll','data-threshold','data-preserve-prepend','data-streaming','data-stream-anchor','data-margin','data-peek']});
 window.__m3MessageScrollers=window.__m3MessageScrollers||{};window.__m3MessageScrollers[id]=scroll;
 window.__m3MessageScrollerCleanup=window.__m3MessageScrollerCleanup||{};window.__m3MessageScrollerCleanup[id]=()=>{
  ++generation;cancelAnimationFrame(frame);cancelAnimationFrame(settleFrame);resize.disconnect();mutation.disconnect();props.disconnect();viewport.removeEventListener('scroll',onscroll);viewport.removeEventListener('wheel',stop);viewport.removeEventListener('touchmove',stop);viewport.removeEventListener('keydown',key);delete window.__m3MessageScrollers[id];delete window.__m3MessageScrollerCleanup[id];
 };
 schedule();
}
