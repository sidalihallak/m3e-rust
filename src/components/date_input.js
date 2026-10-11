const id=__ID__,root=document.getElementById(id),input=root?.querySelector('input');if(!input)return;
window.__m3DateInputs??={};window.__m3DateInputs[id]?.();input.inputMode='numeric';input.autocomplete='off';
const commit=()=>dioxus.send(input.value),key=e=>{if(e.key==='Enter'&&!e.isComposing){e.preventDefault();commit();}},focus=()=>{if(root.dataset.autoFocus==='true'&&root.getClientRects().length&&!root.closest('[inert],[aria-hidden=true]'))input.focus({preventScroll:true});};
input.addEventListener('blur',commit);input.addEventListener('keydown',key);const parent=root.closest('dialog');const observer=new MutationObserver(focus);if(parent)observer.observe(parent,{attributes:true,attributeFilter:['open']});const frame=requestAnimationFrame(focus);
window.__m3DateInputs[id]=()=>{cancelAnimationFrame(frame);observer.disconnect();input.removeEventListener('blur',commit);input.removeEventListener('keydown',key);delete window.__m3DateInputs[id];};
