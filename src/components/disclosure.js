(() => {
 const id=__ID__,root=document.getElementById(id);if(!root)return;
 window.__m3Disclosure??={};window.__m3Disclosure[id]?.();
 const reduced=matchMedia('(prefers-reduced-motion: reduce)'),states=new Map();let dead=false;
 const duration=e=>{const v=getComputedStyle(e).getPropertyValue('--m3-accordion-duration').trim();const n=parseFloat(v);return Number.isFinite(n)?n*(v.endsWith('ms')?1:1000):200;};
 function settle(panel,state){state.animation?.cancel();state.animation=null;panel.style.height=state.open?'auto':'0px';panel.style.visibility=state.open?'visible':'hidden';}
 function update(panel,initial=false){
  const open=panel.dataset.open==='true';let state=states.get(panel);
  if(!state){state={open,animation:null,generation:0};states.set(panel,state);initial=true;}
  if(!initial&&open===state.open)return;
  const from=panel.getBoundingClientRect().height;state.animation?.cancel();state.animation=null;state.open=open;const generation=++state.generation;
  if(!open&&panel.contains(document.activeElement))root.querySelector(`[aria-controls="${panel.id}"]`)?.focus();
  if(initial||reduced.matches||panel.dataset.animate!=='true'){settle(panel,state);return;}
  panel.style.visibility='visible';panel.style.height=from+'px';
  const target=open?panel.firstElementChild.getBoundingClientRect().height:0;state.target=target;
  const animation=panel.animate([{height:from+'px'},{height:target+'px'}],{duration:duration(panel),easing:'ease-out',fill:'forwards'});state.animation=animation;
  animation.finished.then(()=>{if(!dead&&state.generation===generation)settle(panel,state);}).catch(()=>{});
 }
 function scan(){root.querySelectorAll('.m3-disclosure__panel').forEach(panel=>update(panel));for(const [panel,state] of states)if(!root.contains(panel)){state.animation?.cancel();states.delete(panel);}}
 const observer=new MutationObserver(scan);observer.observe(root,{subtree:true,childList:true,attributes:true,attributeFilter:['data-open']});scan();
 const resize=new ResizeObserver(entries=>{for(const entry of entries){const panel=entry.target.parentElement,state=states.get(panel);if(state?.open&&state.animation&&Math.abs(entry.target.getBoundingClientRect().height-state.target)>.5){const from=panel.getBoundingClientRect().height;state.animation.cancel();state.animation=null;panel.style.height=from+'px';state.open=false;update(panel);}}});root.querySelectorAll('.m3-disclosure__inner').forEach(e=>resize.observe(e));
 const motion=()=>{if(reduced.matches)for(const [panel,state]of states){++state.generation;settle(panel,state);}};reduced.addEventListener('change',motion);
 window.__m3Disclosure[id]=()=>{dead=true;observer.disconnect();resize.disconnect();reduced.removeEventListener('change',motion);for(const state of states.values())state.animation?.cancel();delete window.__m3Disclosure[id];};
})();
