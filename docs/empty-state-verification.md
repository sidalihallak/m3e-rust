# EmptyState verification — 2026-10-11

## Scope and references

`src/components/content.rs; assets/content.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Upstream empty](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/empty.tsx)

[Overview](https://m3.material.io/components/lists/overview) · [Specs](https://m3.material.io/components/lists/specs) · [Guidelines](https://m3.material.io/components/lists/guidelines) · [Accessibility](https://m3.material.io/components/lists/accessibility)

## Contract and runtime checks

EmptyState is an upstream composition helper, not a standalone official Material
component with separate official specifications. Semantic heading, explanatory
text and named native action make its purpose available without relying on an icon.
Upstream layout: 48px padding, gap 16px,16px corners, 384px text/action maximum,
24/32 heading, 14/20 description, 40px iconcontainer with 24px glyph. Compact padding
is 32px vertically and 16px horizontally to fit narrow containers.

Actual Enter on Create project replaced the empty state with Project created.
The copy control exactly matched 313 characters. No animation is added to the
composition; supplied actions retain their existing accepted interaction behavior.

### Remaining scope

Custom media and multiple action layouts, RTL, long localization and text zoom
need consumer checks. The gallery example is a local state change, not project
persistence. Choose heading level via surrounding document structure; current
component uses h3 and may require an API extension for other contexts.


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
