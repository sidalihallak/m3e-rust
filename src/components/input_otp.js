const root=document.getElementById('__ROOT_ID__');
if(!root || root.dataset.bound)return;
root.dataset.bound='true';
const input=root.querySelector('input'), slots=[...root.querySelectorAll('[data-index]')];
const update=()=>{
 const focused=document.activeElement===input;
 const start=input.selectionStart??0,end=input.selectionEnd??start;
 const active=Math.min(start,slots.length-1);
 slots.forEach((slot,i)=>{
  slot.dataset.active=String(focused&&(i===active||(i>=start&&i<end)));
  slot.dataset.caret=String(focused&&!input.readOnly&&start===end&&i===active&&!input.value[i]);
  const char=slot.querySelector('.m3-otp__char'),text=input.value[i]||' ';
  if(char.textContent!==text)char.textContent=text;
 });
};
const clean=value=>value.replace(root.querySelector('.m3-otp').dataset.pattern==='digits'?/[^0-9]/g:/[^a-zA-Z0-9]/g,'').slice(0,input.maxLength);
input.addEventListener('input',()=>{
 const before=input.value,position=input.selectionStart??before.length;
 const next=clean(before);
 if(next!==before){input.value=next;const caret=clean(before.slice(0,position)).length;input.setSelectionRange(caret,caret);}
 update();
},true);
input.addEventListener('paste',event=>{
 if(input.disabled||input.readOnly)return;
 const text=event.clipboardData?.getData('text');if(text==null)return;
 const filtered=clean(text);
 if(filtered===text)return;
 event.preventDefault();
 const start=input.selectionStart??input.value.length,end=input.selectionEnd??start;
 const room=input.maxLength-(input.value.length-(end-start));
 input.setRangeText(filtered.slice(0,Math.max(0,room)),start,end,'end');
 input.dispatchEvent(new Event('input',{bubbles:true}));
});
root.querySelector('.m3-otp__slots').addEventListener('click',event=>{
 if(input.disabled)return;
 const slot=event.target.closest('[data-index]');
 input.focus();
 const index=Math.min(Number(slot?.dataset.index??input.value.length),input.value.length);
 input.setSelectionRange(index,Math.min(index+1,input.value.length));update();
});
input.addEventListener('focus',()=>{input.setSelectionRange(Math.min(input.value.length,input.maxLength-1),input.value.length);update();});input.addEventListener('blur',update);
let previous=[input.selectionStart,input.selectionEnd];
const selection=()=>{
 if(document.activeElement!==input)return;
 let start=input.selectionStart,end=input.selectionEnd,direction=input.selectionDirection;
 const length=input.value.length,max=input.maxLength;
 if(length&&start===end&&!(start===length&&length<max)){
  const caret=start;
  if(caret===0){start=0;end=1;direction='forward';}
  else if(caret===max){start=caret-1;end=caret;direction='backward';}
  else if(max>1&&length>1){
   direction=caret<previous[1]?'backward':'forward';
   const inserting=previous[0]===previous[1]&&previous[0]<max;
   const offset=direction==='backward'&&!inserting?-1:0;
   start=caret+offset;end=start+1;
  }
  if(start!==end)input.setSelectionRange(start,end,direction);
 }
 previous=[start,end];update();
};
document.addEventListener('selectionchange',selection);
const observer=new MutationObserver(()=>{if(!root.isConnected){document.removeEventListener('selectionchange',selection);observer.disconnect();}else update();});
observer.observe(document.body,{childList:true,subtree:true});
update();
