const id=__ID__,root=document.getElementById(id);
if(!root || root.dataset.keyboardReady)return true;
root.dataset.keyboardReady='true';
const key=e=>{
 const selector=root.getAttribute('role')==='toolbar'?'button:not(:disabled),a[href]':'[data-nav-destination]:not(:disabled)';
 const items=[...root.querySelectorAll(selector)].filter(el=>el.getClientRects().length);
 const current=e.target.closest(selector),at=items.indexOf(current);if(at<0)return;
 const vertical=root.dataset.navAxis==='vertical',rtl=getComputedStyle(root).direction==='rtl';
 let next;
 if(e.key==='Home')next=0;else if(e.key==='End')next=items.length-1;
 else if(e.key===(vertical?'ArrowDown':rtl?'ArrowLeft':'ArrowRight'))next=(at+1)%items.length;
 else if(e.key===(vertical?'ArrowUp':rtl?'ArrowRight':'ArrowLeft'))next=(at+items.length-1)%items.length;
 if(next!==undefined){e.preventDefault();items[next].focus();}
};
root.addEventListener('keydown',key);
window.__m3Navigation??={};window.__m3Navigation[id]=()=>{root.removeEventListener('keydown',key);delete root.dataset.keyboardReady;delete window.__m3Navigation[id];};
return true;
