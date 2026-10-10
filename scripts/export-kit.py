#!/usr/bin/env python3
"""Export an editable Dioxus component library, including all runtime dependencies."""
from pathlib import Path
import argparse
import re
import shutil

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('destination', type=Path, help='New directory for the exported Rust library')
args = parser.parse_args()
source = Path(__file__).resolve().parent.parent
destination = args.destination.resolve()
if destination.exists():
    parser.error('Destination already exists; choose a new directory to avoid overwriting work.')
(destination / 'src').mkdir(parents=True)
shutil.copytree(source / 'src/components', destination / 'src/components')
for filename in ['lib.rs', 'icons.rs', 'loading_shapes.rs', 'theme.rs']:
    shutil.copy2(source / 'src' / filename, destination / 'src' / filename)
with (destination / 'src/lib.rs').open('a') as file:
    file.write('\npub mod theme;\n')
for filename in ['Cargo.toml', 'Cargo.lock']:
    shutil.copy2(source / filename, destination / filename)
(destination / 'assets').mkdir()
for file in (source / 'assets').glob('*.css'):
    if file.name not in ['main.css', 'tailwind.css']:
        shutil.copy2(file, destination / 'assets' / file.name)
guide = (source / 'docs/copy-components.md').read_text()
guide = re.sub(r'\(([\w-]+\.md)\)',
               r'(https://github.com/sidalihallak/m3e-rust/blob/main/docs/\1)', guide)
(destination / 'README.md').write_text(guide)
print(f'Exported editable Rust sources and component CSS to {destination}')
