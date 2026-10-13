#!/usr/bin/env python3
"""Check each installable module as an independent Wasm library (no gallery)."""
import argparse,json,os,runpy,subprocess,tempfile,time
from pathlib import Path
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--cargo',default='cargo');parser.add_argument('--target-dir',required=True)
parser.add_argument('--report',default='/private/tmp/m3e-registry-checks.json')
args=parser.parse_args();root=Path(__file__).resolve().parent.parent
api=runpy.run_path(str(root/'scripts/m3e.py'))
entries=[k for k,v in api['REGISTRY']['components'].items() if v['public']]
work=Path(tempfile.mkdtemp(prefix='m3e-registry-check-'))
kit=work/'ui-kit';kit.mkdir();env=dict(os.environ,CARGO_TARGET_DIR=str(Path(args.target_dir).resolve()),CARGO_INCREMENTAL='0')
results=[]
for index,key in enumerate(entries):
 modules=api['closure']([key]);payload,styles=api['generated']([key],modules)
 for name,data in payload.items():
  file=kit/name;file.parent.mkdir(parents=True,exist_ok=True);file.write_bytes(data)
 started=time.monotonic()
 result=subprocess.run([args.cargo,'check','--locked','--offline','--jobs','2','--target','wasm32-unknown-unknown','--profile','wasm-dev','--config','profile.wasm-dev.inherits="dev"','--manifest-path',str(kit/'Cargo.toml'),'--lib'],capture_output=True,text=True,env=env)
 item={'entry':key,'resolved':modules,'seconds':round(time.monotonic()-started,2),'passed':result.returncode==0}
 if result.returncode:item['diagnostics']=result.stderr[-9000:]
 results.append(item)
 Path(args.report).write_text(json.dumps({'snapshot':api['REGISTRY']['snapshot'],'work':str(work),'checks':results},indent=2)+'\n')
 print(f'{index+1}/{len(entries)} {key}: {"PASS" if item["passed"] else "FAIL"} ({item["seconds"]}s)',flush=True)
print(f'Report: {args.report}',flush=True)
raise SystemExit(0 if all(r['passed'] for r in results) else 1)
