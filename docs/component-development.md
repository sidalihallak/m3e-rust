# Component development protocol for agents

This is the repeatable process for every component in this UI kit. The user
accepted the corrected button desktop motion on 2026-10-09 as the fidelity
baseline. Preserve that result while expanding the kit.

## 1. Establish the reference contract

Read the component's current pages at
[Material 3 components](https://m3.material.io/components): overview, specs,
guidelines and accessibility. When a page requires JavaScript, inspect it in a
browser instead of treating an empty crawler response as its contents.

Inspect the actual component, shared behavior and theme files in
[shadcn-m3e](https://github.com/Crysta1221/shadcn-m3e). Visual comparison alone
does not reveal input timing, cancellation or easing. Record URLs, retrieval
date, and the upstream revision when available.

For each component list the supported variants, sizes, shapes, roles, states,
input methods, tokens and motion. Where Material and the reference library
differ, explain the chosen behavior. A source-specific timing value must be
attributed to that source, rather than presented as a universal Material rule.

## 2. Implement the copyable Dioxus component

- Keep the Rust API idiomatic and consistent with existing kit components.
- Use semantic Material color roles from the seed-generated theme.
- Use native HTML semantics where available, and correct accessible roles,
  labels, focus and keyboard activation.
- Keep app-specific state and preview layout outside the reusable component.
- Include required CSS and any runtime assets in the copy/install instructions.
- Show a real, copyable Rust usage example in the component preview. The example
  must use the current public API, and its copy control must work.
- Implement all relevant enabled, disabled, focus, hover, pressed, selected,
  loading, error, expanded and dismissal states.
- Respect reduced motion and input cancellation. A superseded animation or
  timer must not clear a newer interaction.

## 3. Verify runtime behavior

Build the actual target and confirm the preview serves the new build. A cached
static preview and a host-only Rust check cannot establish that the current
Wasm component works.

Use actual pointer clicks/taps and keyboard actions. Check quick activation,
held activation, repeated activation, cancellation, focus changes and disabled
input as applicable. On mobile, also check scrolling that begins on the
component, layout at phone width, touch targets, and the on-screen keyboard
where relevant.

For motion, sample computed styles or animation state **while the input is
happening**, through both press/enter and release/exit. Record dimensions,
corner radii, transforms, opacity and timing as applicable. Do not claim motion
is correct because an animation declaration exists or because a screenshot
taken after the interaction shows the resting state.

Use concise interaction checks that detect real regressions. Do not create
tests that merely duplicate implementation constants. Check the relevant
target, and widen validation only when failures or uncovered concerns justify
it.

## 4. Compare and diagnose

Compare the runtime result with the reference's geometry, timings, easing,
opacity, layering and input state transitions. Fix the cause before adjusting
timing to compensate.

The button pilot exposed a useful failure: interpolating `border-radius` from
999px to 8px on a 40px-high button runs a CSS transition but produces no visible
outline change for most of it. Use a finite resting radius of half the height
for the round shape. The accepted button morph changes corners without changing
the button's width or height.

Measure pointer origin relative to the control, not a nested label or icon.
Check that repeating input restarts the feedback and that the latest press
generation owns its release timer.

## 5. Save component evidence

Create `docs/<component>-verification.md`. For an existing report such as
`docs/button-motion-verification.md`, update it in place and link it from the
README. Use this structure:

1. Scope, date, component and source files.
2. Official Material and upstream source links/revisions.
3. Reference contract and implemented values.
4. Input/state/platform verification matrix with measured results.
5. Build and check results.
6. Raw measurements, screenshots or other evidence paths.
7. Concrete unresolved differences and untested behavior.
8. Steps to reproduce the verification.

Save raw motion samples when useful. Screenshots show appearance; time samples
show motion. Do not label an image as a mid-animation capture unless the
captured state establishes that.

## Completion criteria

The component API and usage example are copyable, the requested preview is
running, relevant checks pass, and evidence demonstrates the intended
appearance and interactions. Describe the verified platforms precisely.

If a platform or behavior remains unavailable, document the reason and scope
instead of claiming complete parity. Android Chrome verification covers the
web component on Android; a native Dioxus Android package requires its own
build and device verification.

## Button baseline

Read [the button report](button-motion-verification.md) before changing shared
press or ripple behavior. The accepted desktop contract includes:

| Behavior | Reference value |
| --- | --- |
| Round resting corner | Half the button height |
| Pressed XS/S/M/L/XL corners | 8 / 8 / 12 / 16 / 16px |
| Square XS/S/M/L/XL corners | 12 / 12 / 16 / 28 / 28px |
| Minimum visible press | 225ms |
| Shape interpolation | 240ms Material DefaultEffects spring |
| Ripple growth | 450ms standard easing |
| Ripple fade in/out | 105ms / 375ms linear |
| Pressed ripple opacity | 0.10 |
| Ripple path | Pointer origin to control centre |

Touch scrolling and the upstream 150ms touch ripple delay are a known mobile
verification task, documented in the button report. Desktop acceptance does
not close that task.
