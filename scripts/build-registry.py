#!/usr/bin/env python3
"""Build a content-hashed local source registry from the real module/export graph."""
import re,json,hashlib,argparse,tomllib
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--check',action='store_true');args=parser.parse_args()
module_text=(ROOT/'src/components/mod.rs').read_text()
modules={m.group(2):m.group(1) or '' for m in re.finditer(r'pub(\(crate\))? mod (\w+);',module_text)}
exports={m:[] for m in modules}
for m,group,single in re.findall(r'pub use (\w+)::(?:\{([^}]+)\}|(\w+));',module_text,re.S):
 exports[m].extend(re.findall(r'\b\w+\b',group or single))
owners={name:m for m,names in exports.items() for name in names}
# Shared composition styles intentionally cover multiple APIs in the same module.
style_override={
'action_control':['action-control.css'], 'anchored':['help.css'],
'app_bar':['navigation.css'],'toolbar':['navigation.css'],
'attachment':['application-blocks.css'],'message':['application-blocks.css'],
'message_scroller':['application-blocks.css','advanced.css'],'questionnaire':['application-blocks.css'],
'aspect_ratio':['layout-helpers.css'],'scroll_area':['layout-helpers.css'],'resizable':['layout-helpers.css'],
'calendar':['date-picker.css'],'collapsible':['accordion.css'],
'combobox':['select.css'],'select':['select.css'],'tooltip':['help.css'],
'popover':['help.css','action-control.css'],'hover_card':['help.css'],
'kbd':['expression-helpers.css'],'shape':['expression-helpers.css'],'marker':['expression-helpers.css'],
'menubar':['navigation-extensions.css'],'navigation_menu':['navigation-extensions.css'],
'sidebar':['navigation-extensions.css','navigation.css'],
'sidebar_provider':['sidebar-provider.css','navigation-extensions.css','navigation.css'],
'input_group':['input-extensions.css'],'input_otp':['input-extensions.css'],
'theme_scope':['theme-helpers.css'],'image_theme':['theme-helpers.css'],
'list':['list.css','accordion.css','advanced.css'],'reorder':['advanced.css'],
'data_table':['data-table.css','advanced.css'],'switch':['switch.css'],
'material_symbol':['icon.css'],
}
entries={}
for module in modules:
 file=f'src/components/{module}.rs';code=(ROOT/file).read_text()
 # Lexical token over-approximation is deliberate: extra source is safer than a
 # missing enum/helper. Compilation checks validate the resulting closures.
 tokens=set(re.findall(r'\b\w+\b',code))
 deps={owners[t] for t in tokens if t in owners and owners[t]!=module}
 deps.update(m for m in re.findall(r'super::(\w+)::',code) if m in modules and m!=module)
 for group in re.findall(r'use super::\{(.*?)\};',code,re.S):
  deps.update(m for m in re.findall(r'(\w+)::',group) if m in modules and m!=module)
 files=[file]
 for relative in re.findall(r'include_(?:str|bytes)!\("([^"]+)"\)',code):
  resolved=(ROOT/file).parent/relative
  if not resolved.is_file():raise RuntimeError(f'Missing include in {file}: {relative}')
  files.append(resolved.relative_to(ROOT).as_posix())
 for root_module in ['loading_shapes','shape_library']:
  if f'crate::{root_module}' in code:files.append(f'src/{root_module}.rs')
 styles=style_override.get(module,[])
 own=f'{module.replace("_","-")}.css'
 if (ROOT/'assets'/own).exists() and own not in styles:styles=[own,*styles]
 files.extend('assets/'+s for s in styles)
 if module=='material_symbol':files.extend(p.relative_to(ROOT).as_posix() for p in sorted((ROOT/'assets/material-symbols').iterdir()))
 entries[module.replace('_','-')]={'module':module,'public':not modules[module], 'exports':sorted(set(exports[module])), 'dependencies':sorted(d.replace('_','-') for d in deps),'files':sorted(set(files))}
foundation=['src/components/mod.rs','src/lib.rs','src/icons.rs','src/theme.rs','assets/base.css','assets/composite-motion.css','assets/icon.css','assets/ripple.css','Cargo.toml','Cargo.lock','.cargo/config.toml','THIRD_PARTY_NOTICES.md','assets/material-symbols/LICENSE']
foundation += [p.relative_to(ROOT).as_posix() for p in sorted((ROOT/'src/icons').glob('*.rs'))]
style_order=['base.css','composite-motion.css','icon.css','ripple.css']
for css in re.findall(r'asset!\("/assets/([^"/]+\.css)"\)',(ROOT/'src/main.rs').read_text()):
 if css not in style_order and css not in ['main.css','tailwind.css','showcase.css']:style_order.append(css)
files=sorted(set(foundation+[f for item in entries.values() for f in item['files']]))
checksums={f:hashlib.sha256((ROOT/f).read_bytes()).hexdigest() for f in files}
result={'schema_version':1,'name':'m3e-rust','version':tomllib.loads((ROOT/'Cargo.toml').read_text())['package']['version'],'components':entries,'foundation':foundation,'style_order':style_order,'sha256':checksums}
result['snapshot']=hashlib.sha256(json.dumps(result,sort_keys=True).encode()).hexdigest()
output=json.dumps(result,indent=2,sort_keys=True)+'\n'
if args.check:
 if (ROOT/'registry/registry.json').read_text()!=output:raise SystemExit('Registry is stale. Run scripts/build-registry.py and commit it.')
else:(ROOT/'registry/registry.json').write_text(output)
print(f'Registry: {len(entries)} modules, {sum(e["public"] for e in entries.values())} public entries; snapshot {result["snapshot"][:12]}')
