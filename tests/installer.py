"""Installer preservation, conflict preflight, dry-run and archive checks."""
import hashlib,json,subprocess,sys,tarfile,tempfile,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class Installer(unittest.TestCase):
 def run_cli(self,target,*args,ok=True):
  p=subprocess.run([sys.executable,str(ROOT/'scripts/m3e.py'),*args,'--target',str(target)],capture_output=True,text=True)
  self.assertEqual(p.returncode==0,ok,p.stdout+p.stderr);return p
 def fingerprints(self,path):
  return {str(p.relative_to(path)):hashlib.sha256(p.read_bytes()).hexdigest() for p in path.rglob('*') if p.is_file()}
 def test_preservation_and_preflight(self):
  with tempfile.TemporaryDirectory() as d:
   target=Path(d);self.run_cli(target,'add','button')
   kit=target/'ui-kit';p=kit/'src/components/button.rs';p.write_text(p.read_text()+'\n// local edit\n')
   self.run_cli(target,'add','dialog');self.assertTrue(p.read_text().endswith('// local edit\n'))
   mod=kit/'src/components/mod.rs';mod.write_text(mod.read_text()+'\n// edited exports\n');before=self.fingerprints(kit)
   self.run_cli(target,'add','tabs',ok=False);self.assertEqual(before,self.fingerprints(kit))
   self.run_cli(target,'add','tabs','--overwrite','--dry-run');self.assertEqual(before,self.fingerprints(kit))
 def test_unsafe_destination(self):
  with tempfile.TemporaryDirectory() as d:
   self.run_cli(Path(d),'add','button','--directory','../outside',ok=False)
 def test_all_manifest(self):
  with tempfile.TemporaryDirectory() as d:
   self.run_cli(Path(d),'add','--all');state=json.loads((Path(d)/'ui-kit/m3e.json').read_text())
   expected=sum(v['public'] for v in json.loads((ROOT/'registry/registry.json').read_text())['components'].values())
   self.assertEqual(len(state['requested']),expected)
 def test_release_roundtrip(self):
  with tempfile.TemporaryDirectory() as d:
   work=Path(d)
   for name in ['a','b']:
    subprocess.run([sys.executable,str(ROOT/'scripts/package-registry.py'),'--output',str(work/name)],check=True,capture_output=True)
   self.assertEqual((work/'a/SHA256SUMS').read_bytes(),(work/'b/SHA256SUMS').read_bytes())
   archive=next((work/'a').glob('*.tar.gz'))
   with tarfile.open(archive) as t:t.extractall(work/'extracted',filter='data')
   release=next((work/'extracted').iterdir())
   subprocess.run([sys.executable,str(release/'scripts/verify-distribution.py')],check=True,capture_output=True)
   subprocess.run([sys.executable,str(release/'scripts/m3e.py'),'add','--all','--target',str(work/'consumer')],check=True,capture_output=True)
   self.assertTrue((work/'consumer/ui-kit/src/ui_styles.rs').is_file())
if __name__=='__main__':unittest.main()
