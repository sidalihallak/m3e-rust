const id=__ROOT_ID__,root=document.getElementById(id);
if(root){
 const viewport=root.querySelector('.m3-message-scroller-viewport'),content=root.querySelector('.m3-message-scroller-content'),jump=root.querySelector('.m3-message-scroller-jump');
 const reduced=matchMedia('(prefers-reduced-motion: reduce)');
 let initialized=false,following=false,jumping=false,first=null,anchor=null,anchorTop=0,frame=0,settleFrame=0,generation=0;
 const items=()=>Array.from(content.children).filter(e=>e.hasAttribute('data-message-id'));
 const max=()=>Math.max(0,viewport.scrollHeight-viewport.clientHeight);
 const threshold=()=>Number(root.dataset.threshold)||0;
 const remember=()=>{const vr=viewport.getBoundingClientRect();anchor=items().find(e=>e.getBoundingClientRect().bottom>vr.top);anchorTop=anchor?anchor.getBoundingClientRect().top-vr.top:0;first=items()[0]||null;};
 const update=()=>{
  const near=jump.dataset.direction==='start'?viewport.scrollTop<=threshold():max()-viewport.scrollTop<=threshold();
  if(near&&jump.contains(document.activeElement))viewport.focus({preventScroll:true});
  jump.dataset.active=String(!near);jump.inert=near;jump.setAttribute('aria-hidden',String(near));jump.querySelector('button').tabIndex=near?-1:0;
  root.dataset.following=String(following);root.dataset.scrollable=String(max()>0);
  remember();
 };
 const stop=()=>{++generation;cancelAnimationFrame(settleFrame);jumping=false;following=false;viewport.scrollTo({top:viewport.scrollTop,behavior:'instant'});update();};
 const scroll=(command,messageId)=>{
  let top=command==='start'?0:max();
  if(command==='message'){const item=items().find(e=>e.dataset.messageId===messageId);if(!item)return;top=viewport.scrollTop+item.getBoundingClientRect().top-viewport.getBoundingClientRect().top;}
  ++generation;const mine=generation;cancelAnimationFrame(settleFrame);jumping=true;following=command==='end'&&root.dataset.autoScroll==='true';
  viewport.scrollTo({top,behavior:reduced.matches?'instant':'smooth'});
  let last=-1,still=0;const settle=()=>{if(mine!==generation)return;still=Math.abs(viewport.scrollTop-last)<.5?still+1:0;last=viewport.scrollTop;if(still>=3){jumping=false;following=root.dataset.autoScroll==='true'&&max()-viewport.scrollTop<=threshold();update();}else settleFrame=requestAnimationFrame(settle);};settleFrame=requestAnimationFrame(settle);
 };
 const change=()=>{
  if(!viewport.clientHeight)return;
  const list=items();
  if(!initialized){initialized=true;let top=root.dataset.initial==='start'?0:max();if(root.dataset.initial==='last-anchor'){const item=list.filter(e=>e.dataset.scrollAnchor==='true').at(-1);if(item)top=item.offsetTop-content.offsetTop;}
   viewport.scrollTop=top;following=root.dataset.autoScroll==='true'&&max()-viewport.scrollTop<=threshold();
  }else if(!jumping){
   if(following&&root.dataset.autoScroll==='true')viewport.scrollTop=max();
   else if(first&&first!==list[0]&&anchor?.isConnected&&root.dataset.preservePrepend==='true'){viewport.scrollTop+=anchor.getBoundingClientRect().top-viewport.getBoundingClientRect().top-anchorTop;}
  }
  update();
 };
 const schedule=()=>{cancelAnimationFrame(frame);frame=requestAnimationFrame(change);};
 const onscroll=()=>{if(!jumping)following=root.dataset.autoScroll==='true'&&max()-viewport.scrollTop<=threshold();update();};
 const key=e=>{if(e.target!==viewport)return;if(['ArrowDown','ArrowUp','PageDown','PageUp','Home','End',' '].includes(e.key))stop();};
 viewport.addEventListener('scroll',onscroll,{passive:true});viewport.addEventListener('wheel',stop,{passive:true});viewport.addEventListener('touchmove',stop,{passive:true});viewport.addEventListener('keydown',key);
 const resize=new ResizeObserver(schedule);resize.observe(viewport);resize.observe(content);
 const mutation=new MutationObserver(schedule);mutation.observe(content,{childList:true,subtree:true,characterData:true});
 const props=new MutationObserver(()=>{following=root.dataset.autoScroll==='true'&&max()-viewport.scrollTop<=threshold();schedule();});props.observe(root,{attributes:true,attributeFilter:['data-auto-scroll','data-threshold','data-preserve-prepend']});
 window.__m3MessageScrollers=window.__m3MessageScrollers||{};window.__m3MessageScrollers[id]=scroll;
 window.__m3MessageScrollerCleanup=window.__m3MessageScrollerCleanup||{};window.__m3MessageScrollerCleanup[id]=()=>{
  ++generation;cancelAnimationFrame(frame);cancelAnimationFrame(settleFrame);resize.disconnect();mutation.disconnect();props.disconnect();viewport.removeEventListener('scroll',onscroll);viewport.removeEventListener('wheel',stop);viewport.removeEventListener('touchmove',stop);viewport.removeEventListener('keydown',key);delete window.__m3MessageScrollers[id];delete window.__m3MessageScrollerCleanup[id];
 };
 schedule();
}
