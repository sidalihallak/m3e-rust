# Textarea verification

## Scope and references

2026-10-12. Dioxus core port in `src/components/form.rs`, CSS `assets/form.css`, usage/gallery `src/form_gallery.rs`.

Current rendered official Material pages read in the in-app browser: [overview](https://m3.material.io/components/text-fields/overview); [specs](https://m3.material.io/components/text-fields/specs); [guidelines](https://m3.material.io/components/text-fields/guidelines); [accessibility](https://m3.material.io/components/text-fields/accessibility). Field/Label are compositions around Material text fields, rather than separately named M3 catalogue components.

Actual [upstream textarea.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/textarea.tsx) retrieved and read, plus input/textarea/label/field/toggle/toggle-group sources and shared m3e.css. Revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. Existing group/button reports describe shared motion/runtime dependencies.

## Contract and choices

Shares Input roles, border and focus states. Minimum96px, 16px padding, resize:none and field-sizing:content follow upstream. This intentionally chooses upstream growing behavior; official web text-area guidance also describes fixed-height scrolling areas. Consumers can override field-sizing and height with a class for fixed-height areas. Native rows default3.

## Runtime evidence

Native typing with Enter retained2 lines at96px minimum. Native multiline fill retained five lines; height153px after growth, width317px in the first desktop measurement. Focus/keyboard input is native. Description linked. Exact297-character copy passed.

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
