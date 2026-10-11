# Snackbar verification — 2026-10-11

## Scope and references

`src/components/feedback.rs, snackbar.js; assets/feedback.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Overview](https://m3.material.io/components/snackbar/overview) · [Specs](https://m3.material.io/components/snackbar/specs) · [Guidelines](https://m3.material.io/components/snackbar/guidelines) · [Accessibility](https://m3.material.io/components/snackbar/accessibility)

[Material-oriented upstream sonner](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sonner.tsx) · [Base UI toast](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/toast.tsx)

## Contract and runtime checks

One controlled notification at a time, inverse surface/on-surface/primary roles,
4px corners, 48px minimum height, 14px/20px supporting text. Native text action and
close controls have 48px targets. Desktop width 344–600px; compact width viewport
minus 24px. Bottom offset defaults to 12px and is configurable by CSS.

The Material-oriented `sonner.tsx` uses a 0.8→1 scale with FastSpatial (360ms)
and FastEffects (150ms) opacity; the existing generated spring tokens are reused.
The earlier stacked Base UI toast is not the chosen behavior. An action always
makes the message persistent. Auto-dismiss is opt-in, clamped to 4–10 seconds,
and requires equivalent inline feedback. Hover/focus/background pause the timer.
Alt+G focuses the action/close; Escape only dismisses when focus is in the snackbar.

Measured a 48px resting height, 4px radius and 346.31px width. Actual click samples
include scale .8, .818, .878, .947, .992 and a small token spring overshoot before
rest. Sampled exit reaches hidden. Opening preserves trigger focus. Alt+G reached
Undo; Escape restored Save with undo after correcting an inert/focus race.
A 6000ms plain toast stayed open through 6500ms of focus, then hid after Tab left
and the remaining clock elapsed. Equivalent inline confirmation remained visible.
Actual OS Reduce Motion finished entry without a scale sequence. An actionable
message remained open after a 7000ms observation; Enter on Undo invoked the action.

### Remaining scope

The app must mount one host and own queues. There is no imperative toast store,
promise helper, stacked-toast port or exit-complete queue callback. Message/revision
replacement is supported but rapid timer/exit interruption needs broader runtime
checks. Long-action/new-line and multi-line geometry need dedicated checks;
wrapped text currently follows upstream's 14px vertical padding (68px for two lines).
An isolated hover test kept the pointer inside and keyboard focus outside: the
6000ms toast stayed open through 6500ms, then hid after pointer leave. Background
visibility pause is implemented but was not timed in this audit.


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
