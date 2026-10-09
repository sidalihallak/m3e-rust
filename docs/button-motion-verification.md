# Button motion verification

Verified in the Dioxus web preview on 2026-10-09 using real browser pointer
clicks and keyboard activation. Computed styles were sampled concurrently with
the input at approximately 30ms intervals. These samples measure the actual
rendered component, rather than assuming its CSS declarations are working.

## References

- [Material 3 button specs: Shape morph and Corner sizes](https://m3.material.io/components/buttons/specs)
- [shadcn-m3e button implementation](https://github.com/Crysta1221/shadcn-m3e/blob/main/packages/m3e/src/components/button.tsx)
- [shadcn-m3e press and ripple implementation](https://github.com/Crysta1221/shadcn-m3e/blob/main/packages/m3e/src/components/ripple.tsx)
- [shadcn-m3e shape and ripple CSS](https://github.com/Crysta1221/shadcn-m3e/blob/main/packages/m3e/src/styles/m3e.css)
- [shadcn-m3e generated motion tokens](https://github.com/Crysta1221/shadcn-m3e/blob/main/packages/m3e/src/styles/m3e.generated.css)
- [Google Material Web ripple implementation](https://github.com/material-components/material-web/blob/main/ripple/internal/ripple.ts)

## Cause of invisible motion

The previous round button interpolated from a `999px` radius toward its pressed
radius. On a 40px-high button, every radius above 20px produces the same visible
pill. A quick press ended before the computed radius crossed that threshold.

Observed before the fix: 999px at rest, 316.9px at 141ms, 125.8px at 207ms,
79.3px at 246ms, then back toward 999px. The CSS transition ran, but the button
outline stayed visibly round throughout the quick click.

The resting radius is now half the actual button height, as in the reference.
It interpolates through visible corner sizes immediately. Width and height do
not animate.

## Motion values

| Behavior | Current implementation | Reference |
| --- | --- | --- |
| Minimum visible press | 225ms | 225ms |
| Shape morph | 240ms Material DefaultEffects spring | Same generated spring and duration |
| Ripple growth | 450ms, cubic-bezier(.2, 0, 0, 1) | Same |
| Ripple fade in | 105ms linear | Same |
| Ripple fade out after release | 375ms linear | Same |
| Ripple opacity | 0.10 | Material pressed state opacity |
| Ripple origin | Pointer position relative to button; centre for keyboard | Same |
| Ripple destination | Button centre | Same |
| Ripple geometry | Initial diameter 20% of the larger side; diagonal + 10px padding + soft edge | Same formula |
| Ripple edge | Radial gradient | Same gradient |

The ripple growth and opacity are separate. A long press retains the pressed
shape and ripple opacity until release. Repeated presses replace the ripple
element and invalidate timers from older press generations.

## Browser measurements after the fix

| Sample | Rest radius | Smallest observed radius during quick click | Material pressed target |
| --- | ---: | ---: | ---: |
| XS / Create | 16px | 8.010px | 8px |
| S / Filled | 20px | 8.015px | 8px |
| M / Continue | 28px | 12.048px | 12px |
| L / Confirm | 48px | 16.095px | 16px |
| XL / Get started | 68px | 16.089px | 16px |
| Square / S | 12px | 8.005px | 8px |

The Filled button remained 71.0781px wide and 40px high throughout every sample.
All other tested sizes also retained their dimensions. The small differences
from target radii occur because sampling can miss the exact final frame before
the minimum press ends.

| Input | Observed minimum radius | Peak ripple opacity | Result |
| --- | ---: | ---: | --- |
| First mouse click | 8.015px | 0.10 | Morph, ripple, return to 20px |
| Second mouse click | 8.037px | 0.10 | Replays independently |
| Enter | 8.020px | 0.10 | Centred ripple and morph |
| Space | 8.021px | 0.10 | Centred ripple and morph |
| Toggle selected → unselected | 8.012px | 0.10 | Returns to 20px round shape |
| Toggle unselected → selected | 8.036px | 0.10 | Returns to 12px square shape |
| Disabled pointer click | — | — | No pressed state and no ripple |

## Validation and remaining scope

- `cargo check --locked --offline --target wasm32-unknown-unknown` passed with the pilot's existing isolated toolchain.
- Dioxus completed a fresh web build and is serving with live rebuilds on localhost:8080.
- Browser reduced-motion preference was false during measurement. The reduced-motion CSS disables animation and transition durations; an enabled OS preference has not been exercised here.
- Mobile touch scrolling is not covered by these desktop checks. The reference's 150ms touch ripple delay is still a porting gap; this report does not claim complete touch parity or full component accessibility conformance.

## Repeat this check

1. Start `./scripts/dev.sh` and confirm Dioxus reports a successful build.
2. Reload the browser so it uses the current Rust/Wasm build.
3. Start sampling computed border radius, dimensions, pressed class, ripple
   opacity and transform before issuing a real pointer click.
4. Continue sampling through the press and release, then repeat once to confirm
   the ripple restarts. Check Enter, Space, square shape, toggle selection and
   disabled input.
5. Compare measured values with the linked source and current Material specs.

A screenshot taken after a click has completed cannot establish whether motion
ran. Keep measurements or capture during the press when diagnosing animation.

## Regression check during the component fidelity fixes — 2026-10-09

Button source/CSS remained unchanged. A real quick click in the current desktop
preview measured Filled at minimum radius 8.0367px, peak ripple opacity .10,
and constant 71.078125×40px dimensions. Raw samples are in
[the shared fidelity fix evidence](component-fidelity-fixes-samples.json).
This is a focused regression check; the original platform limitations remain.
