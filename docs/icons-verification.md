# Complete Material Symbols verification

## Scope and sources — 2026-10-13

`Icon`, generated `icons::{rounded,outlined,sharp}` constants, `MaterialSymbol`,
and the searchable icon gallery. Existing unqualified constants remain rounded.

Official rendered pages read: [overview](https://m3.material.io/styles/icons/overview),
[designing icons](https://m3.material.io/styles/icons/designing-icons),
[applying icons, including accessibility](https://m3.material.io/styles/icons/applying-icons),
and [Google Symbols guide](https://developers.google.com/fonts/docs/material_symbols).
Icon guidance has three style pages rather than component specs tabs.

Actual upstream [icon.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/icon.tsx)
and its adjacent icon-registry.ts were read at that revision: offline/registered
SVGs, dynamic names, default 24px and selected fill support.

## Catalogue and contract

4,301 official names (4,001 distinct codepoints), three styles and two fill
states: 25,806 entries, including aliases. Input SVG package is community
maintained `@material-symbols/svg-400@0.48.0`, not Google maintained. Its 3,929
names are supplemented by 290 codepoint aliases and 82 font-only names covering
78 codepoints, from Google revision `49d4db35df873165d6bd6ba09b063c7dafbac2f4`.
No guessed glyph substitution. Retained metadata includes hashes and provenance.

Fixed weight 400, grade 0, font extraction optical size 48, fills 0/1. Standard
rendered icon size 24px; controls provide their own target geometry. Decorative
icons are hidden from accessibility; standalone semantic icons require a label.
Use a consistent style in an application. Static Rust SVG constants and local
external sprites work offline without an icon API or web font. Numeric names
use an `ICON_` prefix; filled constants use `_FILL`.

## Runtime and build evidence

Desktop pointer and keyboard: search, all three styles, fill toggle, page
navigation, selection, numeric constants, exact code clipboard copies. The
font-only magic_button glyph renders in outlined filled mode. Screenshots caught
an initial blank external symbol viewport: setting use x=0/y=-960 and width/height
960 corrected the nested negative-coordinate grid. This was checked visually.

All 25,806 generated Rust/sprite paths match package plus pinned-font inputs;
all official names match. The 38 legacy constants remain byte-identical.
[Source comparison](icons-source-comparison.json), [browser observations](icons-browser-samples.json),
[desktop screenshot](icons-desktop.png). New complete usage and numeric/font-only
examples compile with 123 previous examples: 126 exported usages passed Wasm
Cargo check, with non-fatal example warnings. Dioxus preview builds succeeded.

## Reproduction and gaps

Search #icons for rocket, 10k and magic_button. Change style/fill, select a tile
and copy its Rust usage. `scripts/verify-icons.py` checks pinned generated paths.
Generation uses gen-icons.mjs, gen-extra-icons.py and FontTools 4.66.1; fetch the
exact package and font inputs recorded in catalogue metadata first.

Variable font axes/morphs, upstream automatic parent-fill adapter, audible
screen readers and Android touch are not established by this desktop work.
Unknown dynamic names render a help fallback with data-symbol-valid=false.

Final catalogue search cleared:4,301 names,90 pages. The exact clipboard output
for outlined MAGIC_BUTTON_FILL was checked again after the final build.
Shared CodeCard now accepts String for live selected-icon examples; static
examples continue compiling. It resets copied state when its snippet changes.

## Style-switch regression —2026-10-13

User reported selector changes without visible glyph changes. Reproduced in the
in-app WebKit browser: select value, copied code and SVG href changed to sharp,
while rocket_launch retained its rounded outline. Previous style checks only
observed state/href and did not establish visible switching; those claims are
superseded by this follow-up.

The external sprites shared fragment IDs across styles. Generated fragment IDs
are now namespaced by style (rounded-name / outlined-name / sharp-name). The
use instance also has a reference key to recreate it when name/style/fill changes.
Keying alone did not resolve the reproduced style problem. Glyph artwork is
unchanged; the verifier strips style prefixes before checking all25,806 paths.

Final fix recreates the entire SVG using a keyed dynamic list when its external
reference changes, rather than keying only a nested use node. After successful
Dioxus rebuild (16.37s) and reload, three styles and filled toggle were exercised
and their actual glyph pixels compared. [Regression observations](icons-style-regression.json)
and icons-style-{rounded,outlined,sharp,outlined-filled}.png record visible
differences. Wasm library check passed. No new motion is introduced; keyboard
semantics remain the native select and kit checkbox. Android remains untested.

Official rocket_launch Outlined and Sharp share the same path, so identical
pixels between those two are expected. Repeated comparison with computer,
whose three official paths differ, records distinct pixels for every style pair
in the regression JSON and icons-computer-*.png screenshots.

Computer comparison after allowing each Dioxus update to render: Rounded→
Outlined58 changed glyph pixels, Outlined→Sharp253, Rounded→Sharp252. Screenshots
were taken in separate action/observation calls; immediate screenshots after
selectOption can capture the preceding frame. The final selected computer
preview uses sharp-computer and preserves its accessible label.
[Fixed preview](icons-style-fixed.png).

## Preview controls follow-up —2026-10-13

User still perceived no changes. Native picker pointer opening and Down/Return
commits updated selected style and generated Rust code. The introductory eight
icons were static rounded constants outside IconCatalogue, so the selector had
no effect on them. They now share catalogue style/fill state and render
MaterialSymbol. All eight observed sharp/filled after control changes.

Selected glyph defaults to delete, with three labeled48px reference previews
and an active-style highlight. Catalogue and selected icon remain linked to
controls; the comparison deliberately displays all three styles simultaneously.
The selected preview is48px; grid and control icons remain24px. This uses the
existing fixed400 glyph data, without modifying official artwork.

Actual native keyboard commit, checkbox activation and screenshot verified.
Dioxus rebuild passed in16.11s and browser was reloaded.
[Control observations](icons-preview-controls.json),
[comparison screenshot](icons-comparison-preview.png). Desktop only.

## Repeated perceived no-change report —2026-10-13

Re-read Google's [Symbols guide](https://developers.google.com/fonts/docs/material_symbols):
style family and FILL are separate choices. Rendered delete sprites were
compared in the same page against direct Icon SVG paths for all three styles.
They matched their respective reference artwork; no further glyph substitution
defect was established. Temporary source comparison UI was removed.

The always-visible comparison row itself intentionally retained all three
families, which made it misleading as feedback for a style selection. It is
now a closed native details panel. A single48px selected-style preview appears
immediately below the controls and shows name, family and outline/solid state.
No glyph geometry was exaggerated.

Successful Dioxus build10.23s followed by reload. Observed states across actual
control changes: Rounded · Outline → Rounded · Solid → Sharp · Solid → Sharp ·
Outline. Screenshots saved after state updates in separate calls:
icons-solid-preview.png, icons-sharp-solid-preview.png and
[final selected preview](icons-selected-style-preview.png).
Native keyboard picker was tested earlier; no motion added. Desktop only.
Earlier explanations attributing every perceived problem to external-use
caching should not be treated as proof of that cause.

## Conclusive sprite/source check —2026-10-13

Re-fetched @material-symbols/svg-4000.48.0 with pinned SHA512 and the three
official Google fonts at the recorded commit with pinned SHA256. All25,806
generated paths match those inputs. Font-only regeneration is byte-identical.
The upstream package integrity is retained in upstream-package.json.

The preview's opt-in Verify sprite against source SVG panel displays the actual
MaterialSymbol next to a direct Icon path with matching color, size and fill.
For delete/computer/check ×3 families ×2 fill states, all18 rendered comparisons
have zero pixel differences greater than8 per RGB channel at48px. Each crop
contains a nonempty glyph, ruling out blank-pair false positives.

A live pair changes both renderers through the actual Icon style control.
Rounded →Outlined →Sharp and a Sharp fill change were captured after the
Dioxus state update; all four pairs also have zero mismatched pixels.
A successful17.04s Dioxus build was reloaded before these checks.
[Raw pixel comparisons](icons-artwork-verification.json),
[visible proof](icons-source-proof.png), full icons-artwork-{outline,solid}.png
and icons-live-{rounded,outlined,sharp,sharp-filled}.png.

Measured family differences for outline delete are219 changed pixels for
Rounded→Outlined and57 for Outlined→Sharp; computer25/105, check87/0.
These are corner changes, and Outlined/Sharp check genuinely share artwork.
Solid fill changes delete's interior independently of family. Thus no remaining
wrong-glyph or live style-switch defect is established in these desktop cases.
Earlier repeated visual no-change claims were not evidence of one single cache
failure; the fixed top examples, static comparison UI and subtle source shapes
were separate concerns. Pixel checks now prevent relying on selector/href alone.

GitHub CI additionally prepares Chromium/WebKit screenshot checks. That hosted
job has not run yet; local evidence here is in-app WebKit, not Chromium, Android
or native Dioxus. Variable font axes and automatic parent-fill remain out of scope.
