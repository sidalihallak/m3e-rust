# Bubble verification —2026-10-12

## Scope and reference

Read rendered official [component catalogue](https://m3.material.io/components),
[accessibility principles](https://m3.material.io/foundations/overview/principles)
and [assistive technology](https://m3.material.io/foundations/overview/assistive-technology)
on2026-10-12. None of these application blocks has a dedicated official Material
component page/spec. Geometry comes from upstream; Material guidance informs
semantic roles, keyboard access, named controls, targets and readable text.
Existing button/icon-button/TextField reports cover reused primitive contracts.

Actual [upstream source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/bubble.tsx), revision
c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972, retrieved2026-10-12.

## Reference contract and implementation

Sources: message.rs and application-blocks.css. Seven variants Primary,
Secondary, Muted, Tinted, Outline, Ghost and Error; start/end alignment. Default
max80% width, ghost100%; body14px with1.625 line height, padding10×14px,
32px corners. Outline uses OutlineVariant/Surface; other fills pair semantic roles.
Error uses OnErrorContainer rather than source Error, preserving correct pairing.
Static content is div; action/href selects native button/link. Interactive content
must not contain nested controls. Native minimum48px target and Secondary3px
focus ring. Hover/selection color transitions follow source Tailwind150ms
cubic-bezier(.4,0,.2,1), not a claimed universal Material motion token.
Reactions have highest surface,3px surface ring,4px gap, logical12px edge offsets
and±75% vertical placement. Interactive reaction target48px; explicit24px group
spacing reserves room for overlap, an accessibility/layout addition to source.
RTL uses logical edges. Reduced motion removes new color transitions.

## Runtime/build evidence

Desktop raw measurements and tested actions are saved in
[application-blocks-samples.json](application-blocks-samples.json). Screenshots
are in docs/evidence/application-blocks. Results below describe sampled states,
not every permutation. The live Wasm build and fresh independent export are
checked before completion; five copyable usages are included in the gallery.

## Platform scope and gaps

Android remains paused. Desktop/390px geometry does not establish mobile touch,
on-screen keyboard, native Android package behavior or audible screen-reader
announcements. Reduced-motion permutations beyond the sampled states, forced colors, text zoom, full RTL and
all dynamic/unmount/interruption permutations require wider evidence. No claim
of full upstream engine/API parity follows from the sampled core behavior.

## Reproduce

Run ./scripts/dev.sh, wait for successful build, reload the appropriate catalogue
route. Exercise pointer and native keyboard actions, inspect measured state and
focus, and use390px width separately for bounds. Copy the usage and compile a
fresh exported library. Compare the source contract before modifying behavior.

Token-resolution correction: source `m3e.css` overrides Tailwind radii.
`rounded-xl/2xl/3xl/lg` resolve to20/28/32/16px, not stock Tailwind
12/16/24/8px. The final CSS uses the inspected Material token values. Source:
https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css

## Final build and evidence

Live Wasm serve: initial42.08s success; final Rust rebuild32.64s success, then
reload before the final inspection. One .app-shell measured. Fresh editable
export /private/tmp/m3e-application-export-1012 compiles its library and all103
examples in12.81s. The only warning is a pre-existing unused ChipSize import.
Five clipboard comparisons are exact: attachment615, bubble512, message417,
scroller527 and questionnaire673 characters. Rust/UI copy instructions and all
required JS/CSS/dependencies are in docs/copy-components.md.

Pilot typography remains Inter; upstream uses Roboto Flex. Full font/pixel
parity is not claimed. Android and audible screen readers remain open. Actual
OS Reduce Motion was sampled for shimmer/jump and restored; other reduced-motion
permutations and forced-colors/zoom/full RTL checks remain.

## Measured desktop and compact results

All seven fills and native content tags are recorded. Static/content corners32px
(ghost0px) follow the resolved source tokens. Pointer/Enter/Space activate the
message action three times. A Space-activated named reaction is48×48px and
changes aria-pressed totrue/count1. Disabled action is native-disabled; href uses
a native link. Desktop dark role results are captured separately.

390px body375px: ordinary bubbles fit within221.59px (80% of277px content),
ghost277px. Long unbroken content wraps inside the bubble; no page overflow.
The final token-corrected values are in finalGeometry; earlier radius samples
precede token-alias correction. Screenshots: messages-desktop.png/messages-390.png.
Reaction top/start permutations, nested content and full RTL/forced colors remain.
