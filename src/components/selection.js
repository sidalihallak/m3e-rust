// Runs in the same evaluator after anchored.js; DOM focus stays on combobox.
const popup=window.__m3Anchors[id];
if(popup){
  const input=popup.trigger, panel=popup.root, editable=panel.dataset.kind==='combobox';
  let active=null,buffer='',typed=0,waiting=null;
  const options=()=>[...panel.querySelectorAll('[role=option]')].filter(e=>e.getAttribute('aria-disabled')!=='true');
  const setActive=e=>{for(const item of options())item.removeAttribute('data-active');active=e?.id||null;if(e){e.dataset.active='true';input.setAttribute('aria-activedescendant',e.id);e.scrollIntoView({block:'nearest'});}else input.removeAttribute('aria-activedescendant');};
  const initial=()=>{const items=options();if(waiting==='type'){waiting=null;return;}if(waiting==='last')setActive(items.at(-1));else if(waiting==='first')setActive(items[0]);else if(!editable)setActive(items.find(e=>e.getAttribute('aria-selected')==='true')||items[0]);else setActive(null);waiting=null;};
  const navigate=(direction)=>{const items=options();if(!items.length)return;const index=items.findIndex(e=>e.id===active);const next=index<0?(direction>0?0:items.length-1):index+direction;setActive(editable&&(next<0||next>=items.length)?null:items[Math.max(0,Math.min(items.length-1,next))]);};
  const keyboard=e=>{
    if(e.isComposing||e.ctrlKey||e.metaKey)return;
    const open=panel.matches(':popover-open')&&panel.dataset.phase!=='closing';
    if(e.key==='ArrowDown'||e.key==='ArrowUp'){
      e.preventDefault();if(!open){waiting=e.key==='ArrowUp'?'last':'first';dioxus.send(['open','true']);}else navigate(e.key==='ArrowDown'?1:-1);
    }else if(e.key==='Escape'&&open){e.preventDefault();e.stopPropagation();popup.close(false);setActive(null);}
    else if(e.key==='Tab'&&open){popup.close(false);setActive(null);}
    else if((e.key==='Home'||e.key==='End')&&open&&!editable){e.preventDefault();setActive(e.key==='Home'?options()[0]:options().at(-1));}
    else if(e.key==='Enter'||e.key===' '&&!editable){
      if(open){e.preventDefault();const chosen=options().find(e=>e.id===active);if(chosen)chosen.click();else if(editable&&panel.dataset.custom==='true')dioxus.send(['custom',input.value]);}
      else if(!editable){e.preventDefault();waiting='selected';dioxus.send(['open','true']);}
    }else if(!editable&&e.key.length===1&&!e.altKey){
      e.preventDefault();const now=Date.now();buffer=now-typed>700?'':buffer;typed=now;buffer+=e.key.toLocaleLowerCase();if([...buffer].every(c=>c===buffer[0]))buffer=buffer[0];
      if(!open){waiting='type';dioxus.send(['open','true']);}
      const items=options(),at=items.findIndex(e=>e.id===active);for(let i=1;i<=items.length;i++){const item=items[(at+i+items.length)%items.length];if(item.dataset.label.toLocaleLowerCase().startsWith(buffer)){setActive(item);break;}}
    }
  };
  const hover=e=>{if(e.pointerType!=='mouse')return;const item=e.target.closest('[role=option]');if(item&&item.getAttribute('aria-disabled')!=='true')setActive(item);};
  const changed=new MutationObserver(records=>{
    if(panel.dataset.open!=='true'){setActive(null);return;}
    if(records.some(r=>r.attributeName==='data-open'))initial();
    else if(records.some(r=>r.type==='childList'||r.attributeName==='data-query')){
      setActive(options().find(e=>e.id===active));
    }
  });
  changed.observe(panel,{childList:true,subtree:true,attributes:true,attributeFilter:['data-open','data-query']});
  input.addEventListener('keydown',keyboard);panel.addEventListener('pointerover',hover);
  const cleanup=popup.cleanup;popup.cleanup=()=>{changed.disconnect();input.removeEventListener('keydown',keyboard);panel.removeEventListener('pointerover',hover);input.removeAttribute('aria-activedescendant');cleanup();};
  if(panel.dataset.open==='true')initial();
}
