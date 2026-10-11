(async()=>{
 const id=__ID__,root=document.getElementById(id);if(!root)return;window.__m3Charts??={};window.__m3Charts[id]?.();const plot=root.querySelector('.m3-chart__plot');let dead=false,last='',resolve;
 const resize=new ResizeObserver(()=>{const r=plot.getBoundingClientRect();if(r.width>0&&r.height>0){const key=r.width+'x'+r.height;if(key!==last){last=key;dioxus.send([r.width,r.height]);}}});resize.observe(plot);
 const keys=event=>{const button=event.target.closest('.m3-chart__point');if(!button)return;const buttons=Array.from(root.querySelectorAll('.m3-chart__point:not(:disabled)'));let index=buttons.indexOf(button);if(event.key==='ArrowRight')index++;else if(event.key==='ArrowLeft')index--;else if(event.key==='Home')index=0;else if(event.key==='End')index=buttons.length-1;else return;event.preventDefault();buttons[Math.min(buttons.length-1,Math.max(0,index))]?.focus();};root.addEventListener('keydown',keys);
 window.__m3Charts[id]=()=>{dead=true;resize.disconnect();root.removeEventListener('keydown',keys);delete window.__m3Charts[id];resolve?.();};await new Promise(r=>resolve=r);
})();
