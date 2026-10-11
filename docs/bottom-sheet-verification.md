# BottomSheet verification — 2026-10-11

## Scope and sources

Standard/modal, partial/expanded height, handle gestures and keyboard alternatives. Implementation: `src/components/sheet.rs`, `sheet.js`, `assets/sheet.css`.
Copyable gallery examples: `src/surfaces_gallery.rs`.
Official pages read in the rendered browser: [overview](https://m3.material.io/components/bottom-sheets/overview) · [specs](https://m3.material.io/components/bottom-sheets/specs) · [guidelines](https://m3.material.io/components/bottom-sheets/guidelines) · [accessibility](https://m3.material.io/components/bottom-sheets/accessibility).
Actual upstream sources/revision, observed values and deliberate differences are
recorded in [the family report](surfaces-verification.md).

## Contract, input checks and build

The family report records component-specific geometry, pointer/keyboard states,
measured motion, disabled/RTL behavior, source-export results and reproduction
steps. [Raw samples](surfaces-samples.json) distinguish ordinary desktop and 390px
resized desktop checks. The report's explicit matrix defines verified scope;
shared implementation alone does not establish every variant on every platform.
See [the copy guide](copy-components.md) for complete dependencies and controlled
state callbacks. Build and all 45 independent examples pass.

## Remaining differences

Android is paused. Nested drawer stacks, arbitrary snap points, velocity fling,
whole-surface swipe and broad screen-reader/browser/text-scaling coverage are
unverified or unported as detailed in the family report. No full mobile or exact
pixel parity claim is made. Generic top sheets have no official M3 counterpart.
