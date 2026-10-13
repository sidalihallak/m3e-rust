#!/usr/bin/env python3
"""Install editable Material 3 Dioxus sources. Python 3.11+, no Python packages."""
from pathlib import Path,PurePosixPath
import argparse,json,hashlib,re,os,tempfile,sys
ROOT=Path(__file__).resolve().parent.parent
REGISTRY=json.loads((ROOT/'registry/registry.json').read_text())
def digest(data):return hashlib.sha256(data).hexdigest()
def key(value):
 value=value.replace('_','-')
 if value in REGISTRY['components']:return value
 matches=[k for k,e in REGISTRY['components'].items() if value in e['exports']]
 if len(matches)==1:return matches[0]
 raise ValueError(f'Unknown component {value!r}. Run m3e.py list.')
def closure(roots):
 result=set();queue=list(roots)+['icon']
 while queue:
  item=queue.pop()
  if item in result:continue
  result.add(item);queue.extend(REGISTRY['components'][item]['dependencies'])
 return sorted(result)
def read_source(path):
 relative=PurePosixPath(path)
 if relative.is_absolute() or '..' in relative.parts:raise ValueError('Unsafe registry path: '+path)
 source=(ROOT/path).resolve()
 if not source.is_relative_to(ROOT):raise ValueError('Unsafe source: '+path)
 data=source.read_bytes()
 if digest(data)!=REGISTRY['sha256'][path]:raise ValueError(f'Registry is stale: {path}. Run scripts/build-registry.py.')
 return data

def generated(roots,modules):
 selected={REGISTRY['components'][k]['module'] for k in modules}
 original=(ROOT/'src/components/mod.rs').read_text()
 lines=[]
 for visibility,module in re.findall(r'pub(\(crate\))? mod (\w+);',original):
  if module in selected:lines.append(f'pub{visibility} mod {module};')
 for match in re.finditer(r'pub use (\w+)::(?:\{[^}]+\}|\w+);',original,re.S):
  if match.group(1) in selected:lines.append(match.group(0))
 files=set(REGISTRY['foundation'])
 for module in modules:files.update(REGISTRY['components'][module]['files'])
 result={path:read_source(path) for path in sorted(files)}
 result['src/components/mod.rs']=('\n'.join(lines)+'\n').encode()
 extras=''.join(f'pub mod {name};\n' for name in ['loading_shapes','shape_library'] if f'src/{name}.rs' in files)
 result['src/lib.rs']=('pub mod components;\npub use components::*;\npub mod icons;\npub mod theme;\npub use theme::{ThemePreview,ThemeVariant,ThemeContrast};\n'+extras+'mod ui_styles;\npub use ui_styles::UiStyles;\n').encode()
 css={Path(p).name for p in files if p.startswith('assets/') and p.endswith('.css')}
 ordered=[s for s in REGISTRY['style_order'] if s in css]
 ordered+=sorted(css-set(ordered))
 styles='use dioxus::prelude::*;\n#[component]\npub fn UiStyles()->Element {rsx! {\n'+''.join(f' document::Stylesheet {{href:asset!("/assets/{s}")}}\n' for s in ordered)+'}}\n'
 result['src/ui_styles.rs']=styles.encode()
 result['README.md']=('''# Editable Material 3 UI sources

Installed by scripts/m3e.py; component files are yours to edit.
Load `UiStyles {}` once in your app and provide semantic theme variables:

```rust
let theme = m3e_rust_ui::theme::ThemePreview::from_seed(0xff6750a4);
let variables = m3e_rust_ui::theme::css_variables(&theme.light);
// rsx! { UiStyles {} div { style: variables, ... } }
```

Use `m3e-rust-ui = { path = "ui-kit" }` in your app's dependencies.
No Tailwind, gallery CSS, font service or gallery Rust modules are required.
Icon static constants embed the referenced SVG path. MaterialSymbol uses local
sprite assets; preserve their directory and Apache license.

Requested entries: '''+', '.join(roots)+'\nSnapshot: '+REGISTRY['snapshot']+'\n').encode()
 return result,ordered

def library_path(args):
 target=Path(args.target).expanduser().resolve();directory=PurePosixPath(args.directory)
 if directory.is_absolute() or '..' in directory.parts or str(directory)=='.':raise ValueError('--directory must be a child directory, e.g. ui-kit')
 lib=target/str(directory)
 if lib.is_symlink() or not lib.resolve().is_relative_to(target):raise ValueError('Install destination must stay inside target')
 return target,lib

def install(args):
 target,library=library_path(args);state_file=library/'m3e.json'
 if library.exists() and not state_file.is_file():raise ValueError(f'{library} exists without an installer manifest; choose a new directory.')
 old=json.loads(state_file.read_text()) if state_file.exists() else {'requested':[],'files':{}}
 names=[k for k,e in REGISTRY['components'].items() if e['public']] if args.all else args.components
 if not names and args.command not in ['init','update']:raise ValueError('Choose component names or --all.')
 requested=sorted(set(old['requested'])|{key(v) for v in names})
 modules=closure(requested);payload,css=generated(requested,modules)
 conflicts=[];writes={};preserved=[]
 for path,data in payload.items():
  destination=library/path
  if destination.is_symlink() or not destination.resolve().is_relative_to(library.resolve()):raise ValueError('Unsafe destination: '+str(destination))
  if destination.exists():
   actual=digest(destination.read_bytes());candidate=digest(data);previous=old['files'].get(path)
   if actual==candidate:continue
   if previous==candidate:
    preserved.append(path);continue # unchanged upstream, locally edited source stays yours
   if actual!=previous and not args.overwrite:conflicts.append(path);continue
  writes[path]=data
 if conflicts:raise ValueError('Local changes conflict with this update: '+', '.join(conflicts)+'. Review first; --overwrite explicitly replaces conflicting files.')
 plan={'requested':requested,'resolved':modules,'stylesheets':css,'write_files':sorted(writes),'preserved_local_edits':preserved,'destination':str(library),'snapshot':REGISTRY['snapshot']}
 if args.command=='plan' or args.dry_run:print(json.dumps(plan,indent=2));return
 library.mkdir(parents=True,exist_ok=True)
 for path,data in writes.items():
  file=library/path;file.parent.mkdir(parents=True,exist_ok=True)
  # Atomic individual replacements; all conflicts are detected before any write.
  fd,temp=tempfile.mkstemp(prefix='.m3e-',dir=file.parent)
  with os.fdopen(fd,'wb') as f:f.write(data)
  os.replace(temp,file)
 manifest={'schema_version':1,'snapshot':REGISTRY['snapshot'],'requested':requested,'resolved':modules,'stylesheets':css,'files':{p:digest(data) for p,data in payload.items()}}
 # For preserved edits, retain the last installed upstream checksum for conflict detection.
 for path in preserved:manifest['files'][path]=old['files'][path]
 state_file.write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
 print(f'Installed {len(requested)} requested entries ({len(modules)} modules); {len(writes)} files written. Preserved {len(preserved)} local edits.')
 print(f'Add to {target}/Cargo.toml:\n[dependencies]\nm3e-rust-ui = {{ path = "{args.directory}" }}')
 print('Import m3e_rust_ui::* and load UiStyles {} once. Apply semantic theme variables to an ancestor.')

def main():
 parser=argparse.ArgumentParser(description=__doc__)
 commands=parser.add_subparsers(dest='command',required=True)
 listing=commands.add_parser('list',help='List installable modules and exports');listing.add_argument('--json',action='store_true')
 for name in ['init','add','update','plan']:
  command=commands.add_parser(name)
  command.add_argument('components',nargs='*')
  command.add_argument('--all',action='store_true',help='Install every public registry entry')
  command.add_argument('--target',required=True)
  command.add_argument('--directory',default='ui-kit')
  command.add_argument('--dry-run',action='store_true')
  command.add_argument('--overwrite',action='store_true')
 command=commands.add_parser('status');command.add_argument('--target',required=True);command.add_argument('--directory',default='ui-kit')
 args=parser.parse_args()
 if args.command=='list':
  items={k:v for k,v in REGISTRY['components'].items() if v['public']}
  if args.json:print(json.dumps(items,indent=2))
  else:
   for k,e in items.items():print(f'{k:24} '+', '.join(e['exports']))
 elif args.command=='status':
  _,library=library_path(args);state=json.loads((library/'m3e.json').read_text())
  missing=[];edited=[]
  for path,sha in state['files'].items():
   file=library/path
   if not file.exists():missing.append(path)
   elif digest(file.read_bytes())!=sha:edited.append(path)
  print(json.dumps({'requested':state['requested'],'missing':missing,'locally_edited':edited,'snapshot':state['snapshot']},indent=2))
 else:install(args)
if __name__=='__main__':
 try:main()
 except (ValueError,FileNotFoundError,json.JSONDecodeError) as error:print(f'm3e: {error}',file=sys.stderr);sys.exit(1)
