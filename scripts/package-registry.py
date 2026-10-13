#!/usr/bin/env python3
"""Create a deterministic, self-contained GitHub Release source registry."""
import argparse,gzip,hashlib,io,json,re,subprocess,sys,tarfile,tomllib
from pathlib import Path
root=Path(__file__).resolve().parent.parent
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--version',help='Must match Cargo.toml, e.g. 0.1.0')
parser.add_argument('--output',type=Path,required=True)
args=parser.parse_args()
version=tomllib.loads((root/'Cargo.toml').read_text())['package']['version']
if args.version is not None and args.version!=version:parser.error('Release tag version must match Cargo.toml.')
if not re.fullmatch(r'\d+\.\d+\.\d+(?:-[A-Za-z0-9.-]+)?',version):parser.error('Invalid release version.')
subprocess.run([sys.executable,str(root/'scripts/verify-distribution.py')],check=True)
r=json.loads((root/'registry/registry.json').read_text())
files=set(r['sha256'])|{'registry/registry.json','scripts/m3e.py','scripts/build-registry.py','scripts/verify-distribution.py','docs/registry-installer.md','docs/github-registry-release.md','docs/release-notes.md','THIRD_PARTY_NOTICES.md','Cargo.toml'}
# The graph builder derives CSS order from main; it is metadata input only.
files.add('src/main.rs')
files.update('examples/consumer/'+name for name in ['Cargo.toml','Cargo.lock','Dioxus.toml','README.md','.gitignore','src/main.rs','assets/app.css'])
prefix=f'm3e-registry-{version}';out=args.output.resolve();out.mkdir(parents=True,exist_ok=True)
payload={name:(root/name).read_bytes() for name in sorted(files)}
release={'schema_version':1,'version':version,'registry_snapshot':r['snapshot'],'files':{name:hashlib.sha256(data).hexdigest() for name,data in payload.items()}}
payload['release.json']=(json.dumps(release,indent=2,sort_keys=True)+'\n').encode()
archive=out/(prefix+'.tar.gz')
with archive.open('wb') as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=0) as zipped,tarfile.open(fileobj=zipped,mode='w') as tar:
 for name,data in sorted(payload.items()):
  info=tarfile.TarInfo(prefix+'/'+name);info.size=len(data);info.mode=0o644;info.mtime=0;info.uid=info.gid=0;info.uname=info.gname='';tar.addfile(info,io.BytesIO(data))
checksum=hashlib.sha256(archive.read_bytes()).hexdigest()
(out/'SHA256SUMS').write_text(f'{checksum}  {archive.name}\n')
(out/'release.json').write_bytes(payload['release.json'])
print(f'{archive}: {checksum}')
