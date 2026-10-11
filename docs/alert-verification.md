# Alert banner verification — 2026-10-11

## Scope and references

`src/components/feedback.rs; assets/feedback.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Upstream alert](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/alert.tsx)

[WAI alert pattern](https://www.w3.org/WAI/ARIA/apg/patterns/alert/) · [Overview](https://m3.material.io/components/snackbar/overview) · [Specs](https://m3.material.io/components/snackbar/specs) · [Guidelines](https://m3.material.io/components/snackbar/guidelines) · [Accessibility](https://m3.material.io/components/snackbar/accessibility)

## Contract and runtime checks

This is an upstream library extension; Material has no standalone Alert banner
component with overview/specs/guidelines/accessibility pages. Official snackbar
notification guidance informs its use; this banner is persistent inline content.

16px padding/corners, 20px icon, 14px/20px title and body; Neutral, Info, Tertiary
and Error use semantic container/on-container roles. Status is polite by default;
Alert is available for urgent dynamic content and Note for static information.
It does not move focus or time itself out. Native action and close controls are
siblings of content. Actual pointer dismissal removed Working offline. EmptyState's
native action was separately exercised with Enter. No audible announcement claim.

### Remaining scope

All tones are implemented, but the gallery samples Info/Error. Restore-after-close
and urgent dynamic announcements need screen-reader checks. This is not a port of
AlertDialog (already implemented separately).


## Build, evidence and reproduction

Rust/Wasm check and the live Dioxus build passed. The independent exported library
and all 53 usage examples passed `cargo check --locked --offline --target
wasm32-unknown-unknown --lib --examples`. Eight new gallery copy controls matched
the displayed Rust code exactly. See [family verification](feedback-content-verification.md)
and [raw samples](feedback-content-samples.json). Sample timestamps are external
wall-clock times from a concurrent observation loop; they are not event-relative
animation timestamps. Bounding rectangles include transforms.

Run `./scripts/dev.sh`, wait for a successful Dioxus build, reload the preview,
and open the component's catalogue page. Repeat the inputs above. Include the
styles/runtime dependencies in [copy instructions](copy-components.md).

Desktop evidence does not establish Android touch or native Dioxus Android.
Android remains paused. Audible VoiceOver, forced colors, custom seed contrast,
200% text, broad RTL, interrupted animation and held/touch-cancellation matrices
remain unverified unless explicitly listed below. Do not treat this report as
exhaustive pixel or platform parity.
