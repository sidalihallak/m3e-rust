# Collapsible verification — 2026-10-11

## Component scope

Controlled `Collapsible` with label, open state, disabled state and consumer
trigger class/content. Files: `src/components/collapsible.rs`, shared
`accordion.rs` runtime helper, `disclosure.js`, `assets/accordion.css`.

## References and scope

Retrieved 2026-10-11. The rendered [current Material 3 catalog](https://m3.material.io/components)
has no dedicated accordion, collapsible, table, pagination or data-table
component pages. There are consequently no current four-page M3 contracts to
claim for this family. These are library compositions using Material color,
typography, shape and motion roles. Table guidance below is explicitly
[archived Material 2](https://m2.material.io/components/data-tables), not a new M3 spec.

Upstream revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Reference contract and implementation

[Actual upstream collapsible](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/collapsible.tsx),
[Base UI](https://base-ui.com/react/components/collapsible),
[WAI disclosure pattern](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/).

Upstream is an unstyled Base UI wrapper with no animation classes. This port
therefore exposes consumer styling and immediate content visibility, not the
accordion's styled geometry/height animation. The native trigger has a 48px
minimum target and focus outline. `aria-expanded` and `aria-controls` point to a
stable owned panel; collapsed content is hidden and inert. Gallery outlined
button styling is app code, not a new official Material collapsible variant.

## Runtime verification

Enter opens the panel; Space closes it. Tab from the expanded trigger reaches
View project source. Restricted details is disabled. Open panel height and
inert state were measured. Collapsible code copy exactly matches clipboard.
No height animation is part of its contract. Panel focus recovery uses the
shared runtime but external programmatic collapse while a descendant has
focus was not independently exercised. Hidden-until-found is not implemented.

## Build and evidence

Dioxus web preview built successfully (13.74s, 14.39s, 14.53s and final
13.68s). Reloaded after the successful build; one `.app-shell` mounted.
The independently exported Rust/Wasm library and all 66 real usage examples
pass `cargo check --locked --offline --target wasm32-unknown-unknown`
with the existing `wasm-dev` profile/cache. The pre-existing ChipSize import
warning in the chip example is unrelated.

Raw browser evidence: [data-family-samples.json](data-family-samples.json).
Desktop screenshots and responsive evidence: [evidence/data](evidence/data).
390×844 is desktop responsive emulation, not Android touch verification.
Android remains paused. OS Reduce Motion was temporarily enabled with the
user's existing authorization and restored off. Audible screen readers,
forced colors, RTL, text zoom and all custom seed contrasts remain untested
for these new components. At 320px the existing showcase body's 360px
minimum causes page overflow; 390px has no page overflow. Do not call the
320px showcase responsive behavior verified.

## Reproduce

Open `/#collapsibles`; Enter on Show project details, Tab to its link, return
to the trigger and press Space. Check Restricted details and copy the example.
