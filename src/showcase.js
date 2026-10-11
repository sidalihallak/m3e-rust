window.__m3ShowcaseCleanup?.();
let frame,last='';
const send=()=>{cancelAnimationFrame(frame);frame=requestAnimationFrame(()=>{const value=[location.hash.slice(1),window.scrollY>8,innerWidth>=1400],key=JSON.stringify(value);if(key!==last){last=key;dioxus.send(value);}});};
window.addEventListener('hashchange',send);window.addEventListener('resize',send);window.addEventListener('scroll',send,{passive:true});send();
window.__m3ShowcaseCleanup=()=>{cancelAnimationFrame(frame);window.removeEventListener('hashchange',send);window.removeEventListener('resize',send);window.removeEventListener('scroll',send);};
