#!/usr/bin/env python3
"""Fetch checksum-pinned npm SVG input and official Google font inputs for CI."""
import argparse,base64,hashlib,json,tarfile,urllib.request
from pathlib import Path
root=Path(__file__).resolve().parent.parent
parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--directory',type=Path,required=True);args=parser.parse_args()
out=args.directory.resolve();out.mkdir(parents=True,exist_ok=True)
def fetch(url,path,expected,algorithm='sha256'):
 data=urllib.request.urlopen(url,timeout=120).read()
 calculated=hashlib.new(algorithm,data).digest()
 wanted=base64.b64decode(expected) if algorithm=='sha512' else bytes.fromhex(expected)
 if calculated!=wanted:raise ValueError('Source checksum mismatch: '+url)
 path.write_bytes(data)
package=json.loads((root/'assets/material-symbols/upstream-package.json').read_text())
archive=out/'svg-400.tgz';fetch(package['source'],archive,package['integrity'].removeprefix('sha512-'),'sha512')
with tarfile.open(archive) as tar:tar.extractall(out,filter='data')
fonts=out/'fonts';fonts.mkdir(exist_ok=True)
for style,meta in json.loads((root/'assets/material-symbols/upstream-fonts.json').read_text())['fonts'].items():
 fetch(meta['source'],fonts/(style+'.ttf'),meta['sha256'])
print('Verified pinned SVG package and three Google fonts.')
