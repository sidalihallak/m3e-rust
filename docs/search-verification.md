# SearchBar / SearchView verification — 2026-10-11

## Scope and references

`src/components/search.rs, search.js; assets/search.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Overview](https://m3.material.io/components/search/overview) · [Specs](https://m3.material.io/components/search/specs) · [Guidelines](https://m3.material.io/components/search/guidelines) · [Accessibility](https://m3.material.io/components/search/accessibility)

[Upstream search](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/search.tsx)

## Contract and runtime checks

Current official contained search is the default; upstream's divided baseline,
default elevation 3 and header divider are intentionally not copied. An unfocused
bar is 56px high with 24px exterior margins. Focused pill uses 12px margins.
Docked container uses surface-container-high, minimum 240px height, maximum two
thirds of viewport, and bounded width 720px. Full-screen uses surface-container-low
and a high filled pill. A native modal dialog provides scrim, background modality,
scroll lock and focus return. Back/clear are 48px controls. Enter submits a query
or chooses the arrow-previewed suggestion; IME composition is not intercepted.

Measured closed pill 592×56px and focused pill 616×56px in a 640px
container: 24→12px margins on each side. Actual entry samples include docked
container widths 616→617.10→621.10→629.09→636.62→640 using DefaultSpatial 440ms;
opacity uses DefaultEffects 240ms. Exit was sampled during Escape. An early sample
had 372px container height; the final two-line result rows were corrected to 72px.

Typing travel filtered to one result. ArrowDown kept input focus and associated
aria-activedescendant; Enter committed Travel plans, closed and returned focus to
SearchBar. Initial ArrowUp now reaches Personal notes, skipping disabled Archive
(after fixing an initial reverse-arrow off-by-one). Full-screen desktop filled
1337px viewport. The final gallery keeps submitted results visible: plain Enter
submitted notes, released input focus to the listbox, and ArrowDown/Enter then
committed Personal notes. Closing restored the actual full-screen opener. Clear writes empty controlled query. Reopening clears preview selection.
At 390×844px adaptive used a 390×844px surface and 48px back control, no horizontal
overflow. Actual reduced-motion input completed without width interpolation.

### Remaining scope

Only contained style is shipped; no baseline divided layout/search-app-bar helper.
Async result lists are supplied by the app and child changes reset preview, but
network/virtual suggestions and live anchor/viewport changes during motion need
checks. Full-screen opens with effect fade; a source-to-full-screen spatial morph
is not implemented. Tab from the empty-query input wrapped to Back; Shift+Tab
returned to input. Scrim pointer dismissal and query-dependent Tab cycles need
broader checks. Android on-screen keyboard/predictive back are unverified.


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
