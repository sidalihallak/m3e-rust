"""Actual screenshot comparisons: external sprite versus direct source path."""
import io,json,os,time
from pathlib import Path
from PIL import Image,ImageChops
from playwright.sync_api import sync_playwright
url=os.environ.get('M3E_PREVIEW_URL','http://127.0.0.1:8080/')
out=Path(os.environ.get('M3E_EVIDENCE_DIR','browser-evidence'));out.mkdir(exist_ok=True)
results=[]
with sync_playwright() as p:
 for engine in ['chromium','webkit']:
  browser=getattr(p,engine).launch();page=browser.new_page(viewport={'width':1280,'height':900},reduced_motion='reduce')
  page.goto(url+'#icons');page.get_by_role('combobox',name='Icon style',exact=True).wait_for()
  page.get_by_text('Verify sprite against source SVG',exact=True).click()
  for filled in [False,True]:
   page.get_by_role('checkbox',name='Filled',exact=True).set_checked(filled)
   page.wait_for_function('(v)=>Array.from(document.querySelectorAll(".m3-icon-artwork-pair svg[data-symbol-filled]")).every(e=>e.dataset.symbolFilled===String(v))',arg=filled)
   for pair in page.locator('.m3-icon-artwork-pair').all():
    actual,expected=pair.locator('svg').all();label=pair.get_attribute('data-style')+'-'+pair.get_attribute('data-glyph')
    deadline=time.monotonic()+10
    while True:
     a=actual.screenshot();b=expected.screenshot()
     im_a=Image.open(io.BytesIO(a)).convert('RGB');im_b=Image.open(io.BytesIO(b)).convert('RGB')
     assert im_a.size==im_b.size,label
     differences=ImageChops.difference(im_a,im_b)
     changed=sum(max(pixel)>8 for pixel in differences.getdata())
     dark=sum(max(pixel)-min(pixel)<100 and sum(pixel)<600 for pixel in im_a.getdata())
     if dark>20 and changed/(im_a.width*im_a.height)<0.01:break
     if time.monotonic()>deadline:raise AssertionError(f'{engine} {label} fill={filled}: {changed} mismatched pixels, {dark} glyph pixels')
     page.wait_for_timeout(100)
    stem=f'{engine}-{label}-fill{int(filled)}';(out/(stem+'-sprite.png')).write_bytes(a);(out/(stem+'-source.png')).write_bytes(b)
    results.append({'engine':engine,'glyph':label,'filled':filled,'changed_pixels':changed,'dark_pixels':dark})
  for style in ['rounded','outlined','sharp']:
   page.get_by_role('combobox',name='Icon style',exact=True).select_option(style)
   page.wait_for_function('(v)=>document.querySelector(".m3-icon-selected svg").dataset.symbolStyle===v',arg=style)
   assert all(e==style for e in page.locator('.icon-specimen svg').evaluate_all('(els)=>els.map(e=>e.dataset.symbolStyle)'))
   live=page.locator('.m3-icon-artwork-pair--live');actual,expected=live.locator('svg').all()
   page.wait_for_function('(v)=>document.querySelector(".m3-icon-artwork-pair--live").dataset.style===v',arg=style)
   a=Image.open(io.BytesIO(actual.screenshot())).convert('RGB');b=Image.open(io.BytesIO(expected.screenshot())).convert('RGB')
   changed=sum(max(pixel)>8 for pixel in ImageChops.difference(a,b).getdata())
   assert changed/(a.width*a.height)<0.01,f'{engine} live style change {style}: {changed} wrong pixels'
   results.append({'engine':engine,'live_style_change':style,'changed_pixels':changed})
   page.get_by_role('button',name='Copy Rust code',exact=True).first.click()
   # The visible selected snippet must match the family, not only the select DOM.
   assert 'icons::'+style+'::DELETE_FILL' in page.locator('#icons article').filter(has=page.get_by_role('heading',name='Selected icon usage')).inner_text()
  page.screenshot(path=str(out/(engine+'-preview.png')))
  browser.close()
(out/'icons-results.json').write_text(json.dumps(results,indent=2)+'\n')
print(f'{len(results)} sprite/source comparisons passed in Chromium and WebKit.')
