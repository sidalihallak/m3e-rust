(async()=>{
 const root=document.getElementById(__ID__);if(!root)return;
 window.__m3Commands??={};window.__m3Commands[root.id]?.();
 const input=root.querySelector('input'),list=root.querySelector('[role=listbox]');let active=null,resolve;
 const options=()=>Array.from(list.querySelectorAll('[role=option]')).filter(n=>!n.disabled);
 function select(node,scroll=false){active=node?.dataset.value??null;list.querySelectorAll('[role=option]').forEach(n=>n.setAttribute('aria-selected',String(n===node)));if(node){input.setAttribute('aria-activedescendant',node.id);if(scroll&&document.activeElement===input)node.scrollIntoView({block:'nearest'});}else input.removeAttribute('aria-activedescendant');}
 function sync(){const rows=options();select(rows.find(n=>n.dataset.value===active)||rows[0]);}
 function key(e){if(e.target!==input||e.isComposing||e.ctrlKey||e.metaKey||e.altKey)return;
  const rows=options(),at=rows.findIndex(n=>n.dataset.value===active);
  if(e.key==='ArrowDown'||e.key==='ArrowUp'){e.preventDefault();if(rows.length)select(rows[Math.max(0,Math.min(rows.length-1,(at<0?(e.key==='ArrowUp'?rows.length: -1):at)+(e.key==='ArrowDown'?1:-1)))],true);}
  else if(e.key==='Enter'){const node=rows[at];if(node){e.preventDefault();node.click();}}
  else if(e.key==='Escape'&&!root.closest('dialog')){e.preventDefault();dioxus.send('clear');}
 }
 const pointer=e=>{const node=e.target.closest('[role=option]');if(node&&!node.disabled)select(node);};
 const down=e=>{if(e.target.closest('[role=option]'))e.preventDefault();};
 const observer=new MutationObserver(sync);observer.observe(list,{childList:true,subtree:true,attributes:true,attributeFilter:['disabled','id']});observer.observe(input,{attributes:true,attributeFilter:['value','disabled']});
 root.addEventListener('keydown',key);list.addEventListener('pointermove',pointer);list.addEventListener('pointerdown',down);const inputChanged=()=>queueMicrotask(sync);input.addEventListener('input',inputChanged);sync();
 window.__m3Commands[root.id]=()=>{observer.disconnect();input.removeEventListener('input',inputChanged);root.removeEventListener('keydown',key);list.removeEventListener('pointermove',pointer);list.removeEventListener('pointerdown',down);delete window.__m3Commands[root.id];resolve?.();};
 await new Promise(r=>resolve=r);
})();
