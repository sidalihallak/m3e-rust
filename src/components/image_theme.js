const id=__ROOT_ID__, root=document.getElementById(id);
if(root){
 const input=root.querySelector('input'), preview=root.querySelector('img');
 let generation=0,url=null,alive=true;
 const release=()=>{if(url)URL.revokeObjectURL(url);url=null;};
 const change=async()=>{
  const mine=++generation,file=input.files?.[0]; if(!file)return;
  dioxus.send(['loading',[]]);
  let bitmap;
  try{
   if(file.size>20*1024*1024)throw Error('Choose an image smaller than 20 MB.');
   bitmap=await createImageBitmap(file);
   if(!alive||mine!==generation)return;
   const scale=Math.min(1,128/Math.max(bitmap.width,bitmap.height));
   const canvas=document.createElement('canvas');canvas.width=Math.max(1,Math.round(bitmap.width*scale));canvas.height=Math.max(1,Math.round(bitmap.height*scale));
   const ctx=canvas.getContext('2d',{willReadFrequently:true});ctx.drawImage(bitmap,0,0,canvas.width,canvas.height);
   const data=ctx.getImageData(0,0,canvas.width,canvas.height).data,pixels=[];
   for(let i=0;i<data.length;i+=4)if(data[i+3]===255)pixels.push((0xff000000|(data[i]<<16)|(data[i+1]<<8)|data[i+2])>>>0);
   if(!pixels.length)throw Error('No opaque colors found. Choose another image.');
   release();url=URL.createObjectURL(file);preview.src=url;preview.hidden=false;
   dioxus.send(['pixels',pixels]);
  }catch(e){if(alive&&mine===generation)dioxus.send([e.message||'Could not decode this image.',[]]);}
  finally{bitmap?.close();if(mine===generation)input.value='';}
 };
 input.addEventListener('change',change);
 window.__m3ImageThemes=window.__m3ImageThemes||{};
 window.__m3ImageThemes[id]=()=>{alive=false;++generation;release();input.removeEventListener('change',change);delete window.__m3ImageThemes[id];};
}
