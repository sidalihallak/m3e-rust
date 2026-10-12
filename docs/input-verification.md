# Input verification

## Scope and references

2026-10-12. Dioxus core port in `src/components/form.rs`, CSS `assets/form.css`, usage/gallery `src/form_gallery.rs`.

Current rendered official Material pages read in the in-app browser: [overview](https://m3.material.io/components/text-fields/overview); [specs](https://m3.material.io/components/text-fields/specs); [guidelines](https://m3.material.io/components/text-fields/guidelines); [accessibility](https://m3.material.io/components/text-fields/accessibility). Field/Label are compositions around Material text fields, rather than separately named M3 catalogue components.

Actual [upstream input.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/input.tsx) retrieved and read, plus input/textarea/label/field/toggle/toggle-group sources and shared m3e.css. Revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. Existing group/button reports describe shared motion/runtime dependencies.

## Contract and choices

Outlined and filled native inputs. Default 56px height, 16px horizontal/8px vertical padding, BodyLarge16/24, 4px corners; small40px with12px horizontal padding and BodyMedium14/20. Filled uses surface-container-highest and a bottom indicator; focus adds an inset second stroke without geometry movement. Disabled content38%, outlined stroke12%, filled surface4%. Native number steppers are suppressed as in the previous TextField correction. Small coarse-pointer height48px is a local accessibility adjustment.

Focus motion sampling caught a specificity conflict: hover was overriding the primary focus border. Focus and invalid selectors now match hover specificity so their later state rules win. Focus does not change geometry.

## Runtime evidence

Final focus entry sampled the inset stroke at0.2685px/71ms,0.8098px/106ms,0.9632px/139ms and1px/206ms, with width617.609375px and height56px unchanged. Resting focus border/shadow measured primary rgb(101,85,143); invalid focus measured error rgb(186,26,26). Typing and controlled update passed. Default56px/small40px heights and outlined4px/filled4px4px0px0px corners measured. Read-only Price remains25; Shift+Tab from Compact input skipped the disabled input and reached read-only Price. Supporting description IDs resolve. Validation input toggled aria-invalid and error role colors. Exact322-character copy passed.

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
