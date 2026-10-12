const id=__ROOT_ID__,root=document.getElementById(id);
if(root){
 let frame=0;
 const active=()=>root.querySelector('fieldset:not([hidden])');
 const focus=()=>{cancelAnimationFrame(frame);frame=requestAnimationFrame(()=>{const item=active();(root.dataset.invalid==='true'?item?.querySelector('input:not(:disabled),textarea:not(:disabled)'):item)?.focus({preventScroll:true});});};
 const key=e=>{
  if(e.defaultPrevented||e.isComposing||e.repeat)return;
  const item=active();if(!item)return;
  const controls=Array.from(item.querySelectorAll('input:not(:disabled)'));
  const target=e.target,typing=target.matches('textarea,select,input:not([type=checkbox]):not([type=radio]),[contenteditable=true]');
  const next=()=>root.querySelector('.m3-questionnaire-next')?.click();
  if(e.key==='Enter'&&(e.ctrlKey||e.metaKey)&&!e.altKey&&!e.shiftKey){e.preventDefault();next();return;}
  if(e.ctrlKey||e.metaKey||e.altKey)return;
  if(e.key==='Enter'&&target.matches('input')){e.preventDefault();next();return;}
  if(!typing&&target.type!=='radio'&&(e.key==='ArrowLeft'||e.key==='ArrowRight')){e.preventDefault();if(e.key==='ArrowLeft')root.querySelector('.m3-questionnaire-actions button')?.textContent?.includes('Previous')&&root.querySelector('.m3-questionnaire-actions button')?.click();else if(controls.some(c=>c.checked))next();return;}
  if(!typing&&root.dataset.shortcuts!=='none'){
   const choiceInputs=Array.from(item.querySelectorAll('input[type=radio],input[type=checkbox]'));
   const index=root.dataset.shortcuts==='letters'?e.key.toUpperCase().charCodeAt(0)-65:Number(e.key)-1;
   const node=choiceInputs[index];if(e.key.length===1&&node&&!node.disabled&&index>=0){e.preventDefault();node.focus();node.click();}
  }
 };
 root.addEventListener('keydown',key);
 const observer=new MutationObserver(records=>{if(records.some(r=>r.attributeName==='data-step')||root.dataset.invalid==='true')focus();});observer.observe(root,{attributes:true,attributeFilter:['data-step','data-invalid']});
 window.__m3Questionnaires=window.__m3Questionnaires||{};window.__m3Questionnaires[id]=()=>{cancelAnimationFrame(frame);observer.disconnect();root.removeEventListener('keydown',key);delete window.__m3Questionnaires[id];};
}
