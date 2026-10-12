# Toggle verification

## Scope and references

2026-10-12. Dioxus core port in `src/components/toggle.rs`, CSS `assets/toggle.css`, usage/gallery `src/form_gallery.rs`.

Current rendered official Material pages read in the in-app browser: [overview](https://m3.material.io/components/button-groups/overview); [specs](https://m3.material.io/components/button-groups/specs); [guidelines](https://m3.material.io/components/button-groups/guidelines); [accessibility](https://m3.material.io/components/button-groups/accessibility). Field/Label are compositions around Material text fields, rather than separately named M3 catalogue components.

Actual [upstream toggle.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/toggle.tsx) retrieved and read, plus input/textarea/label/field/toggle/toggle-group sources and shared m3e.css. Revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`. Existing group/button reports describe shared motion/runtime dependencies.

Also read current official [button overview](https://m3.material.io/components/buttons/overview), [specs](https://m3.material.io/components/buttons/specs), [guidelines](https://m3.material.io/components/buttons/guidelines) and [accessibility](https://m3.material.io/components/buttons/accessibility).

## Contract and choices

Controlled native aria-pressed button with Default/Filled/Tonal/Outlined/Elevated roles, size/shape and optional selected icon/label. ActionControl supplies existing ripple and generation-owned press behavior. Small height40px, rest20px, selected12px, pressed8px, shape DefaultEffects240ms; colors retain the kit200ms standard easing. Default variant uses transparent container and on-surface-variant→primary content. Disabled selected controls retain disabled content roles. Source swaps and five source size tokens inspected; Rust API also offers existing XL. Native target at least48px through hit area.

Built-in paired Material glyphs automatically use the filled variant when selected; custom glyphs can supply selected_icon. The transparent Default variant is retained for upstream parity. Official button specs do not recommend text-style toggles; prefer Filled/Tonal/Outlined/Elevated for Material button compositions. Source and Material differ here.

## Runtime evidence

Final native pointer/Space verification confirmed favorite→favorite-fill→favorite. Pointer, Space and Enter each changed controlled selection. Three successive activations ended selected=true. Sampled 40 frames: width125.140625px and height40px remained constant; corners visibly interpolated through press and release, settling12px selected. Disabled native toggle exposed disabled+pressed. Exact265-character copy passed.

Desktop in-app browser and a390px desktop viewport were checked separately. Final geometry and screenshot evidence are in [family samples](form-primitives-samples.json) and `docs/evidence/form-primitives/`. Final compact inputs measured279px width,56px default height (Textarea minimum96px); both groups fitted their279px container, with body375px inside390px viewport. Phone-width results establish responsive desktop layout, not Android touch behavior.

## Build and copies

Live `./scripts/dev.sh` Wasm build succeeded (Rust builds24.49s and5.25s; final CSS hot reload followed). One showcase-shell was confirmed. Exported standalone kit with all109 usages passed `cargo check --locked --offline --target wasm32-unknown-unknown --profile wasm-dev --lib --examples` in10.13s (only pre-existing ChipSize unused-import warning). All six native clipboard controls returned exact current snippets.

## Gaps

Android remains paused. Audible VoiceOver/TalkBack, actual OS Reduce Motion for this family, IME/autofill, all input types, all theme/contrast permutations, RTL and all size/variant combinations are not claimed verified. Reduced-motion CSS uses none for inputs and the existing group/button reduced paths. Inter is the current kit font; upstream uses Roboto Flex. The port uses explicit Rust props and native controls, not complete Base UI form integration or arbitrary React HTML prop/slot parity. Field owns exactly one control and at most one description/error slot; use separate Fields for multiple controls. Input IDs within Field are owned by Field. Dynamic definition reordering requires stable values in toggle groups.

## Reproduce

1. Run `./scripts/dev.sh`, wait for successful build, reload `http://127.0.0.1:8080/#toggles`.
2. Enter multiline/single-line text; click labels; submit an empty Profile, correct it, disable/re-enable. Tab past disabled inputs.
3. Click/Space/Enter Favourite, use arrows and selection in both groups. Compare computed corners/widths while input happens.
4. Copy each usage and compare clipboard to the rendered code; resize to390px and restore normal viewport.
