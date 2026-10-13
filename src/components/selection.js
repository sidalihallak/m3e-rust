// Runs after anchored.js. Logical navigation includes unmounted virtual options.
const popup=window.__m3Anchors[id];
if(popup){
 const input=popup.trigger,panel=popup.root,field=input.closest('.m3-choice'),editable=panel.dataset.kind==='combobox';
 let active=null,buffer='',typed=0,waiting=null,requested=-1;
 const virtual=()=>panel.dataset.virtual==='true';
 const model=()=>{try{return JSON.parse(panel.dataset.model||'[]').map(([id,label,disabled,value,pos])=>({id,label,disabled,value,pos}));}catch{return[];}};
 const options=()=>virtual()?model().filter(e=>!e.disabled):[...panel.querySelectorAll('[role=option]')].filter(e=>e.getAttribute('aria-disabled')!=='true').map((e,pos)=>({id:e.id,label:e.dataset.label,value:e.dataset.value,pos}));
 const windowAt=pos=>{const count=model().length,start=Math.max(0,Math.min(count-1,pos-4));if(start!==requested){requested=start;dioxus.send(['window',String(start)]);}};
 const paint=()=>{for(const item of panel.querySelectorAll('[role=option]'))item.removeAttribute('data-active');const e=active&&document.getElementById(active);if(e){e.dataset.active='true';input.setAttribute('aria-activedescendant',e.id);}else input.removeAttribute('aria-activedescendant');};
 const setActive=item=>{active=item?.id||null;if(item&&virtual()){
   const top=item.pos*48+4,bottom=top+48;if(top<panel.scrollTop)panel.scrollTop=top;else if(bottom>panel.scrollTop+panel.clientHeight)panel.scrollTop=bottom-panel.clientHeight;
   windowAt(Math.floor(panel.scrollTop/48));
  }else if(item)document.getElementById(item.id)?.scrollIntoView({block:'nearest'});
  paint();
 };
 const initial=()=>{const items=options();if(waiting==='type'){waiting=null;return;}if(waiting==='last')setActive(items.at(-1));else if(waiting==='first')setActive(items[0]);else if(!editable){let values=[];try{values=JSON.parse(panel.dataset.values||'[]');}catch{}setActive(items.find(e=>values.includes(e.value))||items[0]);}else setActive(null);waiting=null;};
 const navigate=direction=>{const items=options();if(!items.length)return;const at=items.findIndex(e=>e.id===active),next=at<0?(direction>0?0:items.length-1):at+direction;setActive(editable&&(next<0||next>=items.length)?null:items[Math.max(0,Math.min(items.length-1,next))]);};
 const chips=()=>[...field.querySelectorAll('.m3-choice-chip__main:not(:disabled)')];
 const chipKey=e=>{
  if(e.isComposing||e.ctrlKey||e.metaKey||e.altKey)return;
  const list=chips(),main=e.target.closest('.m3-choice-chip')?.querySelector('.m3-choice-chip__main');
  const rtl=getComputedStyle(field).direction==='rtl';const previous=rtl?'ArrowRight':'ArrowLeft',next=rtl?'ArrowLeft':'ArrowRight';
  if(e.target===input&&(e.key===previous||e.key==='Backspace')&&input.value===''&&input.selectionStart===0&&list.length){e.preventDefault();list.at(-1).focus();return;}
  if(!main)return;const at=list.indexOf(main),value=main.closest('.m3-choice-chip').dataset.chipValue;
  if(e.key===previous||e.key===next||e.key==='Home'||e.key==='End'){e.preventDefault();const target=e.key==='Home'?list[0]:e.key==='End'?input:e.key===previous?list[Math.max(0,at-1)]:list[at+1]||input;target?.focus();}
  else if(e.key==='Delete'||e.key==='Backspace'){e.preventDefault();(list[at+1]||list[at-1]||input).focus();dioxus.send(['chip-remove',value]);}
  else if(e.key==='F2'){e.preventDefault();dioxus.send(['chip-edit',value]);input.focus();}
  else if(e.key==='Escape'){e.preventDefault();input.focus();}
 };
 const keyboard=e=>{
  if(e.isComposing||e.ctrlKey||e.metaKey)return;
  const open=panel.matches(':popover-open')&&panel.dataset.phase!=='closing';
  if(e.key==='ArrowDown'||e.key==='ArrowUp'){
   e.preventDefault();if(!open){waiting=e.key==='ArrowUp'?'last':'first';dioxus.send(['open','true']);}else navigate(e.key==='ArrowDown'?1:-1);
  }else if(e.key==='Escape'&&open){e.preventDefault();e.stopPropagation();popup.close(false);setActive(null);}
  else if(e.key==='Tab'&&open){popup.close(false);setActive(null);}
  else if((e.key==='Home'||e.key==='End')&&open&&!editable){e.preventDefault();setActive(e.key==='Home'?options()[0]:options().at(-1));}
  else if(e.key==='Enter'||e.key===' '&&!editable){
   if(open){e.preventDefault();const chosen=options().find(e=>e.id===active);if(chosen){if(virtual())dioxus.send(['select',chosen.value]);else document.getElementById(chosen.id)?.click();}else if(editable&&panel.dataset.custom==='true'&&panel.getAttribute('aria-busy')!=='true')dioxus.send(['custom',input.value]);}
   else if(!editable){e.preventDefault();waiting='selected';dioxus.send(['open','true']);}
  }else if(!editable&&e.key.length===1&&!e.altKey){
   e.preventDefault();const now=Date.now();buffer=now-typed>700?'':buffer;typed=now;buffer+=e.key.toLocaleLowerCase();if([...buffer].every(c=>c===buffer[0]))buffer=buffer[0];
   if(!open){waiting='type';dioxus.send(['open','true']);}
   const items=options(),at=items.findIndex(e=>e.id===active);for(let i=1;i<=items.length;i++){const item=items[(at+i+items.length)%items.length];if(item.label.toLocaleLowerCase().startsWith(buffer)){setActive(item);break;}}
  }
 };
 const hover=e=>{if(e.pointerType!=='mouse')return;const el=e.target.closest('[role=option]'),item=el&&options().find(o=>o.id===el.id);if(item)setActive(item);};
 const scroll=()=>{if(virtual())windowAt(Math.floor(panel.scrollTop/48));};
 const changed=new MutationObserver(records=>{
  if(panel.dataset.open!=='true'){setActive(null);return;}
  if(records.some(r=>r.attributeName==='data-open'))initial();
  else if(records.some(r=>r.attributeName==='data-query'||r.attributeName==='data-model')){requested=-1;setActive(null);if(virtual()){panel.scrollTop=0;windowAt(0);}}
  else if(records.some(r=>r.type==='childList'))paint();
 });
 changed.observe(panel,{childList:true,subtree:true,attributes:true,attributeFilter:['data-open','data-query','data-model']});
 input.addEventListener('keydown',keyboard);field.addEventListener('keydown',chipKey);panel.addEventListener('pointerover',hover);panel.addEventListener('scroll',scroll,{passive:true});
 const cleanup=popup.cleanup;popup.cleanup=()=>{changed.disconnect();input.removeEventListener('keydown',keyboard);field.removeEventListener('keydown',chipKey);panel.removeEventListener('pointerover',hover);panel.removeEventListener('scroll',scroll);input.removeAttribute('aria-activedescendant');cleanup();};
 if(panel.dataset.open==='true')initial();
}
