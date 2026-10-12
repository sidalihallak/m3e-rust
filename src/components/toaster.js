const id=__ID__,root=document.getElementById(id);if(!root)return;
window.__m3Toasters??={};window.__m3Toasters[id]?.();
let row=null,stamp='',animations=[],epoch=0,timer=0,remaining=0,started=0,expirySent=false,opener,restore=false;
const holds=new Set(),media=matchMedia('(prefers-reduced-motion:reduce)');
const measure=()=>{if(row){const title=row.querySelector('.m3-toast-title'),description=row.querySelector('.m3-toast-description');row.dataset.multiline=(title.offsetHeight+(description?.offsetHeight||0)>20)?'true':'false';}};
const sizeObserver=new ResizeObserver(measure);
const stop=()=>{clearTimeout(timer);if(started){remaining=Math.max(0,remaining-(performance.now()-started));started=0;}};
const resume=()=>{if(started||!row||row.dataset.open!=='true'||holds.size||Number(row.dataset.duration)<=0||expirySent)return;const token=epoch,uid=Number(row.dataset.uid),revision=Number(row.dataset.revision);started=performance.now();timer=setTimeout(()=>{started=0;remaining=0;expirySent=true;if(token===epoch&&row?.dataset.open==='true')dioxus.send(['timeout',uid,revision]);},Math.max(0,remaining));};
const ms=name=>{if(media.matches)return 0;const raw=getComputedStyle(root).getPropertyValue(`--m3-${name}-duration`).trim(),value=parseFloat(raw);return Number.isFinite(value)?value*(raw.endsWith('ms')?1:1000):0;};
const sync=()=>{
 const next=root.querySelector('.m3-toast-card');
 if(!next){++epoch;stop();animations.forEach(a=>a.cancel());sizeObserver.disconnect();row=null;stamp='';const live=root.querySelector('[data-toast-announcement]');if(live)live.textContent='';return;}
 const signature=[next.dataset.uid,next.dataset.revision,next.dataset.open,next.dataset.duration].join(':');if(signature===stamp&&next===row)return;
 const same=next===row,oldStyle=same?getComputedStyle(row):null,from=oldStyle?.transform,opacity=oldStyle?.opacity;
 stop();animations.forEach(a=>a.cancel());const token=++epoch;stamp=signature;
 if(!same){opener=document.activeElement;restore=false;sizeObserver.disconnect();}row=next;row.dataset.ready='true';measure();sizeObserver.observe(row.querySelector('.m3-toast-text'));
 const open=row.dataset.open==='true',uid=Number(row.dataset.uid),revision=Number(row.dataset.revision),style=getComputedStyle(root);row.dataset.phase=open?'opening':'closing';
 animations=[row.animate([{transform:from&&from!=='none'?from:open?'scale(.8)':'scale(1)'},{transform:open?'scale(1)':'scale(.8)'}],{duration:ms('fast-spatial'),easing:style.getPropertyValue('--m3-fast-spatial').trim(),fill:'both'}),row.animate([{opacity:opacity??(open?'0':'1')},{opacity:open?1:0}],{duration:ms('fast-effects'),easing:style.getPropertyValue('--m3-fast-effects').trim(),fill:'both'})];
 if(open){const live=root.querySelector('[data-toast-announcement]');live.textContent='';requestAnimationFrame(()=>{if(token===epoch)live.textContent=row.dataset.message;});remaining=Number(row.dataset.duration)||0;expirySent=false;
  if(root.contains(document.activeElement))holds.add('focus');else holds.delete('focus');if(row.matches(':hover'))holds.add('hover');else holds.delete('hover');if(document.hidden)holds.add('hidden');else holds.delete('hidden');}
 Promise.all(animations.map(a=>a.finished)).then(()=>{if(token!==epoch)return;row.dataset.phase=open?'open':'closed';animations.forEach(a=>a.cancel());
  if(open)resume();else {if((restore||root.contains(document.activeElement))&&(document.activeElement===document.body||root.contains(document.activeElement))&&opener?.isConnected)opener.focus({preventScroll:true});row.hidden=true;dioxus.send(['closed',uid,revision]);}
 }).catch(()=>{});
};
const enter=()=>{holds.add('hover');stop();},leave=()=>{holds.delete('hover');resume();};
const focus=()=>{holds.add('focus');stop();},blur=e=>{if(root.contains(e.relatedTarget))return;if(row?.dataset.open!=='true')restore=true;holds.delete('focus');resume();};
const key=e=>{if(!row||row.dataset.open!=='true')return;if(e.altKey&&e.key.toLowerCase()==='g'){e.preventDefault();(row.querySelector('button:not(:disabled)')||root).focus();}else if(e.key==='Escape'&&root.contains(document.activeElement)){e.preventDefault();restore=true;dioxus.send(['close',Number(row.dataset.uid),Number(row.dataset.revision)]);}};
const visibility=()=>{if(document.hidden){holds.add('hidden');stop();}else{holds.delete('hidden');resume();}};
const reduced=()=>{if(media.matches)animations.forEach(a=>a.finish());};
const click=e=>{if(e.target.closest('button'))restore=root.contains(document.activeElement);};
const observer=new MutationObserver(records=>{if(records.some(r=>r.target===root||(r.type==='attributes'&&r.target.matches?.('.m3-toast-card'))))sync();});
observer.observe(root,{childList:true,subtree:true,attributes:true,attributeFilter:['data-open','data-uid','data-revision','data-duration']});
root.addEventListener('pointerenter',enter);root.addEventListener('pointerleave',leave);root.addEventListener('focusin',focus);root.addEventListener('focusout',blur);root.addEventListener('click',click,true);document.addEventListener('keydown',key);document.addEventListener('visibilitychange',visibility);media.addEventListener('change',reduced);sync();
window.__m3Toasters[id]=()=>{++epoch;stop();observer.disconnect();sizeObserver.disconnect();animations.forEach(a=>a.cancel());root.removeEventListener('pointerenter',enter);root.removeEventListener('pointerleave',leave);root.removeEventListener('focusin',focus);root.removeEventListener('focusout',blur);root.removeEventListener('click',click,true);document.removeEventListener('keydown',key);document.removeEventListener('visibilitychange',visibility);media.removeEventListener('change',reduced);delete window.__m3Toasters[id];};
