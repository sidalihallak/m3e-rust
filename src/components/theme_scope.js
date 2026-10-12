const id=__ROOT_ID__, root=document.getElementById(id);
const query=matchMedia('(prefers-color-scheme: dark)');
const send=()=>dioxus.send(query.matches);
let first=0,second=0;
const pause=()=>{
 if(!root)return;
 cancelAnimationFrame(first);cancelAnimationFrame(second);
 root.classList.add('m3-theme-changing');
 getComputedStyle(root).color;
 first=requestAnimationFrame(()=>{second=requestAnimationFrame(()=>root.classList.remove('m3-theme-changing'));});
};
const observer=new MutationObserver(pause);
if(root){observer.observe(root,{attributes:true,attributeFilter:['style']});pause();}
window.__m3ThemeModes=window.__m3ThemeModes||{};
query.addEventListener('change',send);
window.__m3ThemeModes[id]=()=>{
 query.removeEventListener('change',send);observer.disconnect();
 cancelAnimationFrame(first);cancelAnimationFrame(second);root?.classList.remove('m3-theme-changing');
 delete window.__m3ThemeModes[id];
};
send();
