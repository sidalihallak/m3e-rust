#!/usr/bin/env python3
"""Check registry hashes and generated icon consistency without external inputs."""
import hashlib,json,re,subprocess,sys,xml.etree.ElementTree as ET
from pathlib import Path
root=Path(__file__).resolve().parent.parent
subprocess.run([sys.executable,str(root/'scripts/build-registry.py'),'--check'],check=True)
r=json.loads((root/'registry/registry.json').read_text())
for name,expected in r['sha256'].items():
 assert hashlib.sha256((root/name).read_bytes()).hexdigest()==expected,name
catalog=json.loads((root/'assets/material-symbols/catalog.json').read_text())
names=set(catalog['names']);index=set(line.split()[0] for line in (root/'assets/material-symbols/upstream-codepoints.txt').read_text().splitlines() if line.strip())
assert names==index
variants={name+suffix for name in names for suffix in ['', '-fill']}
ns={'s':'http://www.w3.org/2000/svg'}
for style in ['rounded','outlined','sharp']:
 sprite=root/f'assets/material-symbols/{style}.svg'
 assert hashlib.sha256(sprite.read_bytes()).hexdigest()==catalog['sha256'][style],style
 paths={s.attrib['id'].removeprefix(style+'-'):s.find('s:path',ns).attrib['d'] for s in ET.parse(sprite).findall('.//s:symbol',ns)}
 rust=dict(re.findall(r'pub const \w+: IconData = IconData \{ name: "([^"]+)", path: "([^"]+)" \};',(root/f'src/icons/{style}.rs').read_text()))
 assert paths.keys()==variants and paths==rust,style
 assert len(paths)==catalog['counts'][style],style
print(f"Verified registry {r['snapshot'][:12]}, {len(names)} names, {len(variants)*3} sprite/Rust entries.")
