# Accordion verification — 2026-10-11

## Component scope

Controlled vertical `Accordion` with `AccordionEntry`, single or multiple values,
disabled items/root and configurable heading level. Files:
`src/components/accordion.rs`, `src/components/disclosure.js`, `assets/accordion.css`.

## References and scope

Retrieved 2026-10-11. The rendered [current Material 3 catalog](https://m3.material.io/components)
has no dedicated accordion, collapsible, table, pagination or data-table
component pages. There are consequently no current four-page M3 contracts to
claim for this family. These are library compositions using Material color,
typography, shape and motion roles. Table guidance below is explicitly
[archived Material 2](https://m2.material.io/components/data-tables), not a new M3 spec.

Upstream revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Reference contract and implementation

[Actual upstream accordion](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/accordion.tsx),
[Base UI](https://base-ui.com/react/components/accordion),
[WAI accordion pattern](https://www.w3.org/WAI/ARIA/apg/patterns/accordion/).

- Upstream: 2px group gap; 4px inner corners; 16px end/open corners;
  hover 12px; expanded margin 6px except exterior edges; surface fill.
- Trigger: minimum 56px, 16px horizontal/12px vertical padding, 16/24 title,
  20px chevron, native button, inset focus outline, disabled opacity .38.
- Content: 14/20 body, 16px horizontal and bottom padding, primary links.
- Up/down icon swaps are represented by a static 180° chevron change.
- Upstream `animate-accordion-*` comes from tw-animate-css 1.4.0, default
  200ms ease-out. The package keyframes reference Radix-style height variables,
  while this source uses Base UI. This port measures the actual content height
  instead of copying that variable mismatch. Shape uses the kit DefaultEffects
  240ms token. No guessed height or max-height animation.
- Retained panels become inert/aria-hidden immediately on collapse and hidden
  after exit. Closing a focused panel returns focus to its controlling button.
- Current APG/Base UI guidance keeps normal Tab order; no roving-arrow scheme.

## Runtime verification

| Input/state | Observed result |
| --- | --- |
| Pointer / Enter / Space | Correct open/close state and corresponding content |
| Single mode | Opening theme closes ownership |
| Multiple mode | General and Notifications remain expanded together |
| Disabled heading | Native disabled semantics; Managed settings not actionable |
| Tab in open panel | Focus reaches Browse the source; collapsed content inert |
| Entry/exit | Measured 0→92px and 92→0px; exit visible until zero, then hidden |
| Rapid keyboard reversal | In-progress 88.42→60.13px reverses and reaches 92px; stale completion does not close the new panel |
| Actual Reduce Motion | 92→0 and 0→92 with no sampled intermediate heights |
| Responsive 390px | Single column, 295px triggers; 56px minimum, long title grows to 72px |
| Copy | Both usage snippets exactly match clipboard |

Pointer automation waited for stable geometry, so its attempted rapid sequence
is retained as evidence but is not claimed as mid-transition pointer reversal.
Keyboard samples establish reversal. Dynamic fonts/content during an animation,
root disabling mid-press, held pointer cancellation and browser find-in-page
expansion are not independently verified; hidden-until-found is not implemented.

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

Open `/#accordions`. Activate each heading with click, Enter and Space. Tab
through an open panel. Repeatedly press Enter within 200ms, and compare height
samples. Enable Reduce Motion and repeat, then restore. Test both groups and
copy both snippets. Screenshot: `evidence/data/accordion-phone.png`.
