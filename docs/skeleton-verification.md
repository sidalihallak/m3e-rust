# Skeleton / LoadingPlaceholder verification — 2026-10-11

## Scope and references

`src/components/content.rs; assets/content.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Upstream skeleton](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/skeleton.tsx)

[Overview](https://m3.material.io/components/lists/overview) · [Specs](https://m3.material.io/components/lists/specs) · [Guidelines](https://m3.material.io/components/lists/guidelines) · [Accessibility](https://m3.material.io/components/lists/accessibility)

## Contract and runtime checks

Skeleton is an upstream helper, not an official standalone Material component.
It uses surface-container-highest, 12px corners or circle, and Tailwind's 2s
cubic-bezier(.4,0,.6,1) opacity pulse (1→.5→1). This duration belongs to upstream,
not the Material spring scheme. Skeleton shapes are hidden from assistive technology.
LoadingPlaceholder associates aria-busy with content and supplies a concise status.

A 2100ms sample captured changing opacity during real loading state. Toggle loading
removed all skeletons, set aria-busy=false and displayed real project details.
With actual macOS Reduce Motion, animation-name became none, opacity stayed1
through the 600ms sample. Original OS preference was restored off.

### Remaining scope

Audible loading/completion announcement and data/network cancellation are consumer
verification tasks. Skeleton is a placeholder helper, not a replacement for
accessible progress when progress is known. Arbitrary widths/heights require an
appropriate consuming layout; content shift is not automatically prevented.


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
