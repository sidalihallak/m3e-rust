# NavigationRail verification — 2026-10-11

## Scope and source

Implementation: `src/components/navigation.rs`, `assets/navigation.css`. Collapsed/narrow/expanded, width limits, header slot and full-width active pill.

Official reference read on 2026-10-10: [overview](https://m3.material.io/components/navigation-rail/overview) · [specs](https://m3.material.io/components/navigation-rail/specs) · [guidelines](https://m3.material.io/components/navigation-rail/guidelines) · [accessibility](https://m3.material.io/components/navigation-rail/accessibility).
Actual upstream revision, implementation values, deliberate differences and
measured results are in [the navigation report](navigation-verification.md).

## Verification and reproduction

The family report records per-state pointer/keyboard checks, motion observations,
actual OS reduced motion, build/export results and steps to reproduce. This report
inherits those results only for the states explicitly measured there. Keep this
component's Rust/CSS and required runtime dependencies together using
[the copy guide](copy-components.md). Each component has a real copyable gallery
example in `src/navigation_gallery.rs`.

## Remaining gaps

Android remains paused; audible screen-reader, text enlargement and full pixel
parity are unverified. Final browser recheck was blocked by URL policy, and raw
sample arrays from the earlier session were not retained. See the family report
for component-specific untested variants and final-edit limitations.
