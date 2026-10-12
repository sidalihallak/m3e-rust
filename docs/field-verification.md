# Field compositions verification

## Scope and references

2026-10-12. Dioxus core port in `src/components/form.rs`, CSS `assets/form.css`, usage/gallery `src/form_gallery.rs`.

Current rendered official Material pages read in the in-app browser: [overview](https://m3.material.io/components/text-fields/overview); [specs](https://m3.material.io/components/text-fields/specs); [guidelines](https://m3.material.io/components/text-fields/guidelines); [accessibility](https://m3.material.io/components/text-fields/accessibility). Field/Label are compositions around Material text fields, rather than separately named M3 catalogue components.

Actual [upstream field.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/field.tsx) retrieved and read, plus input/textarea/label/field/toggle/toggle-group sources and shared m3e.css. Revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. Existing group/button reports describe shared motion/runtime dependencies.

## Contract and choices

Field owns one native Input or Textarea. Context supplies stable input and description/error IDs and reactive invalid/disabled state. FieldSet/Legend use native grouping; FieldGroup gap28px, nested16px; Field gap8px; FieldContent gap4px; FieldDescription/Error BodySmall12/16/.4px. Legend TitleMedium16/24; compact LabelLarge14/20. Separator20px,1px outline-variant with surface-backed label. Responsive orientation uses the upstream md container threshold28rem. Errors deduplicate nonempty messages and use role=alert. A subtree observer links only existing helper/error slots; cleanup removes the observer. The example swaps helper/error rather than adding both.

## Runtime evidence

Empty Save profile produced a single deduplicated alert and aria-invalid=true, described-by error ID. Typing Pilot cleared error and restored supporting description ID. Disabled FieldSet disabled both descendants; re-enable retained value. Pointer label and native Tab navigation passed. Exact639-character copy passed. Compact responsive field stays a column; at a1600px viewport its476px parent exceeded28rem and orientation became row. Inherited disabled labels measured on-surface38%.

Desktop in-app browser and a390px desktop viewport were checked separately. Final geometry and screenshot evidence are in [family samples](form-primitives-samples.json) and `docs/evidence/form-primitives/`. Final compact inputs measured279px width,56px default height (Textarea minimum96px); both groups fitted their279px container, with body375px inside390px viewport. Phone-width results establish responsive desktop layout, not Android touch behavior.

## Build and copies

Live `./scripts/dev.sh` Wasm build succeeded (Rust builds24.49s and5.25s; final CSS hot reload followed). One showcase-shell was confirmed. Exported standalone kit with all109 usages passed `cargo check --locked --offline --target wasm32-unknown-unknown --profile wasm-dev --lib --examples` in10.13s (only pre-existing ChipSize unused-import warning). All six native clipboard controls returned exact current snippets.

## Gaps

Android remains paused. Audible VoiceOver/TalkBack, actual OS Reduce Motion for this family, IME/autofill, all input types, all theme/contrast permutations, RTL and all size/variant combinations are not claimed verified. Reduced-motion CSS uses none for inputs and the existing group/button reduced paths. Inter is the current kit font; upstream uses Roboto Flex. The port uses explicit Rust props and native controls, not complete Base UI form integration or arbitrary React HTML prop/slot parity. Field owns exactly one control and at most one description/error slot; use separate Fields for multiple controls. Input IDs within Field are owned by Field. Dynamic definition reordering requires stable values in toggle groups.

## Reproduce

1. Run `./scripts/dev.sh`, wait for successful build, reload `http://127.0.0.1:8080/#form-fields`.
2. Enter multiline/single-line text; click labels; submit an empty Profile, correct it, disable/re-enable. Tab past disabled inputs.
3. Click/Space/Enter Favourite, use arrows and selection in both groups. Compare computed corners/widths while input happens.
4. Copy each usage and compare clipboard to the rendered code; resize to390px and restore normal viewport.
