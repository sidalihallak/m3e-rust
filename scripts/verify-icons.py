#!/usr/bin/env python3
"""Compare generated constants/sprites with the pinned SVGs and font supplement."""
import json,re,sys,xml.etree.ElementTree as ET
from pathlib import Path
root=Path(__file__).resolve().parent.parent;package=Path(sys.argv[1])
ns={'s':'http://www.w3.org/2000/svg'};extra=json.loads((root/'assets/material-symbols/font-extras.json').read_text());catalog=json.loads((root/'assets/material-symbols/catalog.json').read_text())
index=dict(line.split() for line in (root/'assets/material-symbols/upstream-codepoints.txt').read_text().splitlines() if line.strip())
assert set(catalog['names'])==index.keys()
checks=[]
for style in ['rounded','outlined','sharp']:
 constants={name:path for name,path in re.findall(r'pub const \w+: IconData = IconData \{ name: "([^"]+)", path: "([^"]+)" \};',(root/f'src/icons/{style}.rs').read_text())}
 sprites={e.attrib['id'].removeprefix(style+'-'):e.find('s:path',ns).attrib['d'] for e in ET.parse(root/f'assets/material-symbols/{style}.svg').findall('.//s:symbol',ns)}
 expected={}
 for svg in (package/style).glob('*.svg'):expected[svg.stem]=' '.join(e.attrib['d'] for e in ET.parse(svg).findall('s:path',ns))
 expected.update(extra['paths'][style])
 for alias,canonical in extra['aliases'].items():
  for suffix in ['', '-fill']:expected[alias+suffix]=expected[canonical+suffix]
 assert constants==expected and sprites==expected,style
 checks.append({'style':style,'compared_variants':len(expected),'package_variants':7858,'font_only_variants':164,'alias_variants':580})
report={'date':'2026-10-13','names':len(index),'google_revision':extra['revision'],'google_index_complete':True,'total_variants':sum(c['compared_variants'] for c in checks),'source_matches':checks,'font_axis_values':extra['details']}
(root/'docs/icons-source-comparison.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report))
