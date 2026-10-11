# Avatar / AvatarGroup verification — 2026-10-11

## Scope and references

`src/components/content.rs; assets/content.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Upstream avatar](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/avatar.tsx)

[Overview](https://m3.material.io/components/lists/overview) · [Specs](https://m3.material.io/components/lists/specs) · [Guidelines](https://m3.material.io/components/lists/guidelines) · [Accessibility](https://m3.material.io/components/lists/accessibility)

## Contract and runtime checks

Avatar is a library helper rather than a standalone official Material component.
Official lists specify circular leading media for people/entities; images/products
can use rectangular list media instead. Upstream sizes 32/40/56px, primary-container
fallback, entity name, optional activity/status dot and 8px overlapping group are retained.
AvatarGroup has a named group and a named additional-participants count. Decorative
avatars suppress duplicate labels; the inner image has empty alt within the named
entity. A keyed image child resets load/error state when the URL changes.

Measured32/40/56px sizes. Local image completed with natural width 267 and opacity 1.
Toggling to a missing URL removed the failed image and showed PA initials;
toggling back restored the image. Group entity labels/status appeared in the
accessibility properties. Copy example matched 352 characters exactly.

### Remaining scope

Group count/ring geometry, every size/status combination, RTL overlap and consumer
image loading races need deeper checks. No image decoding animation or expressive
shape variants are claimed. The deliberate missing URL produces an expected404.


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
