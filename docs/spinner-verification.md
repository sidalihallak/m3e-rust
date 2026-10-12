# Spinner verification

## Scope and references

2026-10-12. `src/components/spinner.rs`, `assets/spinner.css`, gallery
`src/item_gallery.rs`. This is the compact upstream SVG loading wrapper; use the
existing CircularProgress for determinate/wavy progress and LoadingIndicator for
expressive shape morphing.

Read current rendered official progress [overview](https://m3.material.io/components/progress-indicators/overview),
[specs](https://m3.material.io/components/progress-indicators/specs),
[guidelines](https://m3.material.io/components/progress-indicators/guidelines), and
[accessibility](https://m3.material.io/components/progress-indicators/accessibility).
Inspected actual [spinner.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/spinner.tsx) and
[m3e.css](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css) at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

## Contract and choices

SVG viewBox48×48; center24,24; radius20; stroke5 with round caps; normalized
pathLength100. Default layout20px, configurable by typed size. Primary foreground;
inherit_color uses parent content color for integration into buttons. No track.
Source rotation1.568s linear; arc1.333s standard cubic-bezier(.2,0,0,1). Arc offset
97→25→97 and local rotation0→45→360 degrees exactly follow source keyframes.
These timings are source-specific, not universal Material durations.

Official progress accessibility requires progressbar, so the standalone component
uses a labeled indeterminate progressbar without aria-valuenow; upstream uses
status. decorative=true removes role/name and hides the SVG when surrounding
status/control already communicates the operation. Reduced motion disables both
animations and retains a static75% arc and accessible state. Indeterminate visuals
must not imply a known percentage. Consumer removes/replaces the indicator when
work completes; prefer determinate progress as soon as a measured value exists.

## Runtime checks

-24 live samples showed rotation matrices changing and arc offsets96.943→38.303→
 25.312→80.276px. Computed periods exactly1.568s and1.333s, pathLength100/stroke5.
 Initial samples retained the pre-correction status role; final DOM check confirms
 progressbar. getBoundingClientRect varies as the square SVG rotates; layout
 dimensions remain20×20,32×32 and48×48px.
- Native Complete action replaced the inline loading state with Project ready;
 Space on Restart brought it back. Decorative instances have no role/name and
 aria-hidden=true, including the disabled Saving control. inherit_color resolved exactly to the button content color color(srgb0.6583530.6407840.667686).
- Actual OS Reduce Motion: matchMedia=true; both animation names none and dash
 offset25px. Preference restored to false and normal animation resumed.
-390px viewport: body375px with no page overflow. All three copy examples were
 checked; Spinner220-character clipboard content matched exactly.

## Remaining scope

Audible announcement ordering, all consuming controls, contrast across every
color mode and global busy/application integrations remain. A custom class can
change visual sizing/color, but the API intentionally does not duplicate the
existing determinate progress component. No app or network work is performed by
this display component.

## Build and evidence

Live Dioxus Wasm builds succeeded; the latest clean serve build took14.71s. The
standalone exported source kit and all112 usage examples passed the offline Wasm
check in11.93s, with only the pre-existing ChipSize unused-import warning.

Raw browser evidence: [samples](item-spinner-samples.json). Desktop and390px
screenshots are in `docs/evidence/item-spinner/`. One showcase-shell was confirmed.
The OS Reduce Motion preference was tested off→on→off and the original settings
panel restored. Desktop compact-width evidence does not establish Android touch
behavior. Android remains paused; native packages and audible screen readers are
not verified. Inter differs from upstream Roboto Flex. All theme/contrast, RTL,
held-pointer/cancellation and dynamic nested/unmount permutations remain broader
coverage tasks.

## Reproduce

Run `./scripts/dev.sh`, wait for its successful build and reload the preview.
Use native pointer/Space/Enter and arrow/Tab input, compare copies against the
rendered code, sample computed styles during interaction, then resize to390px
and restore the viewport. For reduced motion, toggle the actual OS preference,
confirm matchMedia and sampled animation/state behavior, and restore it.
Preview: `http://127.0.0.1:8080/#spinners`.
