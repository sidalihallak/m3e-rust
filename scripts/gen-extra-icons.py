#!/usr/bin/env python3
"""Extract font-only Google Symbols at weight400/grade0/optical48, plus aliases.
Requires FontTools (version recorded in upstream-fonts.json).
Usage: python gen-extra-icons.py <pinned-font-directory> <svg-400-package>
"""
import sys,json,hashlib
from pathlib import Path
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
root=Path(__file__).resolve().parent.parent
fonts=Path(sys.argv[1]);package=Path(sys.argv[2])
index=dict(line.split() for line in (root/'assets/material-symbols/upstream-codepoints.txt').read_text().splitlines() if line.strip())
known={f.stem for f in (package/'rounded').glob('*.svg') if not f.stem.endswith('-fill')}
assert known<=index.keys()
canonical={index[n]:n for n in sorted(known)}
aliases={n:canonical[c] for n,c in index.items() if n not in known and c in canonical}
missing={n:c for n,c in index.items() if n not in known and c not in canonical}
source=json.loads((root/'assets/material-symbols/upstream-fonts.json').read_text())
paths={};details=[]
for style in ['rounded','outlined','sharp']:
 file=fonts/(style+'.ttf');assert hashlib.sha256(file.read_bytes()).hexdigest()==source['fonts'][style]['sha256']
 font=TTFont(file);cmap=font.getBestCmap();scale=960/font['head'].unitsPerEm
 assert all(int(c,16) in cmap for c in index.values()),style
 outlines={}
 for filled in [0,1]:
  glyphs=font.getGlyphSet(location={'wght':400,'GRAD':0,'opsz':48,'FILL':filled})
  cache={}
  for name,code in sorted(missing.items()):
   if code not in cache:
    pen=SVGPathPen(glyphs,ntos=lambda n:format(n,'.6f').rstrip('0').rstrip('.'))
    glyphs[cmap[int(code,16)]].draw(TransformPen(pen,(scale,0,0,-scale,0,0)))
    cache[code]=pen.getCommands()
    assert cache[code],f'No outline: {style}/{name}'
   outlines[name+('-fill' if filled else '')]=cache[code]
 paths[style]=outlines
 details.append({'style':style,'units_per_em':font['head'].unitsPerEm,'axis_values':{'wght':400,'GRAD':0,'opsz':48,'FILL':[0,1]},'font_only_names':len(missing),'font_only_codepoints':len(set(missing.values()))})
report={'revision':source['revision'],'aliases':aliases,'font_only':missing,'paths':paths,'details':details}
(root/'assets/material-symbols/font-extras.json').write_text(json.dumps(report,indent=2,sort_keys=True)+'\n')
print(f'Extracted {len(missing)} font-only names ({len(set(missing.values()))} glyphs) and {len(aliases)} aliases; all {len(index)} official names covered.')
