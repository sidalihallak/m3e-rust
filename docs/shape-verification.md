# Shape library — verification,2026-10-12

## Scope and official contract

`src/components/shape.rs`, generated `src/shape_library.rs`,
`scripts/gen-shape-library.mjs`, `assets/expression-helpers.css` and shared
`composite-motion.css`. ShapeName exposes35 typed names, ALL, name() and
clip_path(); Shape supplies a noninteractive48px default mask, configurable size,
optional image label and caller-owned children/classes. Omitting name preserves
an unmasked box, matching the source's optional name.

Read the rendered current official [overview/principles](https://m3.material.io/styles/shape/overview-principles),
[corner specs](https://m3.material.io/styles/shape/corner-radius-scale) and
[morph guidance](https://m3.material.io/styles/shape/shape-morph) on2026-10-12.
The library contains35 shapes; use decorative shapes thoughtfully and connect
morphing with interactions or changing state. The published availability section
lists platform APIs; this web port uses the upstream CSS implementation.
[Accessibility principles](https://m3.material.io/foundations/overview/principles)
were reviewed in this session. Optional media labels/alt text remain app owned.

Inspected actual [shape.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/shape.tsx)
and complete [shape-library.ts](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/lib/shape-library.ts)
at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. The generator executes the inspected
pure sampling module with Node24 type stripping: same path sampling,300 points,
normalized bounds, winding/phase alignment, two-decimal percentages. Copyright
and MIT notices are preserved in THIRD_PARTY_NOTICES.md and the source export.

## Tokens and source comparison

- Default48×48px, aspect-square, Primary fill; media children fill with object-cover.
- All35 polygons contain300 points. Rust enum names map to the source's exact names.
- SlowEffects330ms and its existing linear spring are reused from composite-motion.css.
  Shape changes clip-path; element size and transform remain unchanged.
- Reduced motion disables clip-path transition. Meaningful media may retain alt
  text; a label supplies role=img for a named composite. Decorative shapes add no role.
- All35 rendered64px catalogue clips were compared numerically with actual upstream
  output: zero coordinate error at source precision,600 coordinates per shape.
  [Comparison](shape-source-comparison.json) records each name/count/error.

## Actual interaction and motion evidence

Codex in-app browser, pointer and keyboard:

- Native Choose heart click morphs clover→heart. Space on Choose clover reverses it.
  Both have changing clip coordinates and stable160×160 dimensions/transform:none.
- Concurrent input/style sampling catches300-point frames through the transition.
  First observed heart target to first resting sample344ms (330ms token plus frame/
  driver sampling granularity). No shape box shrink or scale was introduced.
- Sunny interrupted by Heart settles on Heart; latest input wins without a timer race.
- Tab from Choose sunny skips the disabled action and focuses Copy Rust code.
- At390px, catalogue still contains35 shapes; body/client375px;160px stage wraps
  within the card. Desktop and compact measurements are separate from Android.
- Actual macOS Reduce Motion was initially off, temporarily enabled under existing
  user approval, and restored off. With it on, transition-property:none and every
  sampled clip hash is already the clover endpoint. The global gallery rule reports
  duration1e-05s, but there is no transitioning property. Normal mode returns clip-path
  /0.33s. System Settings returned to its original General page.
- Dark Primary fill measured rgb207,189,254; light theme restored.
- Copy control exactly matches448 characters.

[Raw frame/state evidence](expression-helpers-samples.json),
[desktop](evidence/expression-helpers/shapes-desktop.png),
[390px](evidence/expression-helpers/shapes-390.png).
Screenshots show appearance; time samples establish motion.

## Build and gaps

Actual Dioxus build38.43s succeeded after gallery delimiter corrections. Fresh exporter bundle
and all95 usages pass offline Wasm compilation in9.24s; only existing unused
ChipSize import warning. Export includes shape_library.rs, CSS, motion dependency
and third-party notices. No separate dx build ran into the live output.

Android touch/native rendering, audible screen readers, forced colors/zoom,
all directed shape-pair combinations, media video playback, missing-name-to-shape
transitions and arbitrary clipped interactive children are unverified. Fidelity
comparison covers the upstream CSS polygons; separate Compose analytic morph/
curvature parity is not established. Use ordinary semantic controls for actions
and keep text-heavy content outside decorative masks.

## Reproduce

Run `./scripts/dev.sh`, confirm success, reload Foundations → Shape library.
Activate heart/clover/sunny with pointer and keyboard; interrupt one morph. Sample
clip-path and box bounds during interaction. Compare the35 catalogue polygons with
upstream output using the saved comparison data. Resize to390px. With explicit
approval, toggle OS Reduce Motion and restore it. Copy/compile the snippet with
shape_library.rs, expression-helpers.css, composite-motion.css and notices.

To regenerate data, use Node24+:
`node --experimental-strip-types scripts/gen-shape-library.mjs /path/to/inspected/shape-library.mts`.
