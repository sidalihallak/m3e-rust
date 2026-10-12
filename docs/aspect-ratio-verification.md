# Aspect ratio — verification, 2026-10-12

## Scope and references

`src/components/aspect_ratio.rs`, `assets/layout-helpers.css` and the aspect-ratio
page in `src/layout_helpers_gallery.rs`. This is a CSS layout helper, not a
standalone Material component with its own overview/spec/accessibility pages.

Read the rendered official [canonical layout overview](https://m3.material.io/foundations/layout/canonical-examples/overview),
[supporting-pane guidelines](https://m3.material.io/foundations/layout/canonical-examples/supporting-pane),
[usability guidance](https://m3.material.io/foundations/usability/overview) and
[accessibility principles](https://m3.material.io/foundations/overview/principles)
on2026-10-12. They inform adaptable containment and readable, operable layouts;
they do not prescribe the ratios used here.

Inspected the actual upstream [aspect-ratio.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/aspect-ratio.tsx)
at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`: relative div, CSS ratio custom
property, caller-supplied children/classes, no interaction or intrinsic motion.

## Contract and measured evidence

- Ratio is width/height; parent owns available width. Invalid/nonpositive/nonfinite
  ratios fall back to1 locally. The helper adds no component role or focus stop.
- Direct image/video children fill the frame with object-fit:cover. Caller supplies
  alt text and may override cropping. Gallery uses its existing landscape SVG.
- Corners/background are gallery choices, not imposed by the reusable wrapper.
- Desktop16:9 and1:1 match the requested ratios; raw widths/heights are saved.
- At390px, frames327×183.94 and327×327px; ratios1.77778 and1. No page overflow.
- Copy control exactly matches346 characters. Exported usage has no missing media
  dependency: it contains a semantic-color placeholder.

[Raw measurements](layout-helpers-samples.json),
[desktop](evidence/layout-helpers/aspect-desktop.png),
[390px](evidence/layout-helpers/aspect-390.png).
There is no press/selection/motion state to verify for this noninteractive helper.

## Build, gaps and reproduction

The actual Dioxus preview built successfully; all91 independent exported usages
compile for Wasm. Current build results are recorded in the raw evidence and the
scroll-area report. The existing unused ChipSize import is the only check warning.

Android is paused.390px is desktop responsive evidence. Video playback, arbitrary
intrinsic children, invalid ratio permutations, text zoom and audible screen-reader
output are untested. This container does not make inaccessible media accessible.

Run `./scripts/dev.sh`, wait for a successful build and reload. Open Content →
Aspect ratio, inspect native frame bounds, resize to390px, and copy/compile the
usage with layout-helpers.css and Material color roles.
