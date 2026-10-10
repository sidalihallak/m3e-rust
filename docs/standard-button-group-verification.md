# Standard button groups — 2026-10-10

## Scope and references

Sources: `src/components/standard_button_group.rs`, `action_control.rs`,
`assets/standard-button-group.css`, `action-control.css`, `composite-motion.css`.
The gallery shows actions, required single selection, multiple selection, RTL,
disabled items, icons and all five sizes. Controlled selection belongs to the app.

Read the rendered Material [overview](https://m3.material.io/components/button-groups/overview),
[specs](https://m3.material.io/components/button-groups/specs),
[guidelines](https://m3.material.io/components/button-groups/guidelines), and
[accessibility](https://m3.material.io/components/button-groups/accessibility).
Inspected shadcn-m3e at revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`:
[button-group.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/button-group.tsx),
[use-standard-group.ts](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/use-standard-group.ts),
and its shared motion styles. Retrieval date: 2026-10-10.

## Contract and implementation

| Feature | Implemented contract |
| --- | --- |
| Heights XS/S/M/L/XL | 32 / 40 / 56 / 96 / 136px |
| Gaps XS/S/M/L/XL | 18 / 12 / 8 / 8 / 8px; no wrapping |
| Target | At least 48 × 48px; small icon endpoints reserve the extra area |
| Group | Non-focusable named group, native buttons; Tab enters, arrows/Home/End move focus |
| Selection | Actions have no pressed state; selection buttons expose `aria-pressed` |
| Width choreography | Upstream target +15%, nearest neighbours donate space; total width fixed |
| Width motion | Upstream Expressive FastSpatial spring, 360ms |
| Press shape/ripple | Accepted kit button baseline: 240ms effects, 225ms minimum press, existing Ripple |

Expansion is clamped when a neighbour reaches its content/target minimum. The
ResizeObserver, font and content observers remeasure natural widths after motion,
without treating ripple DOM updates as content changes. Required selection prevents
removing the last item; consumers must initialise their controlled selection.

## Runtime evidence

Desktop pointer and keyboard checks exercised required selection, RTL arrows,
disabled skipping and keyboard activation. In the edit-action group the container
remained 279.10px wide. Copy's natural width was 97.58px; the 15% target was
112.21px and the spring briefly reached 113.43px before returning to 97.58px.
This overshoot comes from the upstream spring, not an increased target.

Actual macOS Reduce Motion was enabled with user approval: Copy remained 97.58px
through activation, with functional click feedback. The original off setting was
restored. See [raw composite samples](composite-samples.json) for the final run.

## Build, copying and reproduction

Build and independent snippet results are in [the composite verification index](composite-verification.md).
Follow [the source-copy dependency table](copy-components.md). Run
`./scripts/dev.sh`, wait for a successful build, reload, and visit
`/#standard-button-groups`. Click Copy while sampling button widths; then exercise
the single-select and RTL examples with arrows and Enter/Space.

## Remaining scope

Android touch/scroll cancellation and native Dioxus Android remain paused. Audible
screen-reader output, forced colors, enlarged text and arbitrary dynamic content
permutations are not verified. Extended held pointer and pointercancel paths still
need dedicated input tooling. This report does not establish pixel parity for
every font or consumer layout.
