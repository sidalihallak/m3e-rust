# Scroll area — verification, 2026-10-12

## Scope and reference contract

`src/components/scroll_area.rs` + `scroll_area.js`, `assets/layout-helpers.css`,
`src/layout_helpers_gallery.rs`: named native viewport, vertical/horizontal/both
axes and decorative pointer-operated ScrollBar. Viewport remains the keyboard
scroll target. Height is configurable CSS; width belongs to its container.

This is an upstream helper without a dedicated Material component specification.
Read the rendered [canonical layout overview](https://m3.material.io/foundations/layout/canonical-examples/overview),
[supporting pane](https://m3.material.io/foundations/layout/canonical-examples/supporting-pane),
[usability](https://m3.material.io/foundations/usability/overview),
[accessibility principles](https://m3.material.io/foundations/overview/principles)
and [RTL guidance](https://m3.material.io/foundations/layout/bidirectionality-rtl)
on2026-10-12. Applicable guidance is adaptable containment, clear hierarchy,
keyboard access and appropriate mirroring; no Material scrollbar token is invented.

Inspected actual upstream [scroll-area.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/scroll-area.tsx)
at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`, plus its Base UI shared engine:
[viewport](https://github.com/mui/base-ui/blob/master/packages/react/src/scroll-area/viewport/ScrollAreaViewport.tsx),
[scrollbar](https://github.com/mui/base-ui/blob/master/packages/react/src/scroll-area/scrollbar/ScrollAreaScrollbar.tsx),
[root](https://github.com/mui/base-ui/blob/master/packages/react/src/scroll-area/root/ScrollAreaRoot.tsx),
[constants](https://github.com/mui/base-ui/blob/master/packages/react/src/scroll-area/constants.ts).
Shared sources are master as retrieved2026-10-12, not a version-pinned dependency port.

| Reference | Local contract |
| --- | --- |
| Track10px,1px padding |8px visual thumb within10px track |
| Thumb Outline, rounded |Semantic Outline; finite thumb geometry |
| Opacity transition150ms |Source fade, no spatial spring added |
| Engine minimum thumb16px |16px minimum, bounded by available track |
| Engine scrolling flag500ms |Latest scroll owns a cleared/restarted timer |
| Native viewport |Named region, tabindex0, native wheel/keyboard/touch scrolling |
| Decorative scrollbar |aria-hidden, no extra keyboard stop; pointer capture/drag/track jump |
| Both axes |Corner accounted for in both tracks |

Local additions: focus keeps bars visible, coarse pointer tracks widen to24px,
reduced motion removes fading, forced colors use CanvasText/Highlight. These are
local accessibility choices, not claims about new Material scrollbar specs.

## Runtime evidence and diagnosis

Codex in-app browser, current Wasm, desktop native input:

- PageDown moved the vertical viewport220px; Home/End work natively. Settled End
  reached760px, equal to the maximum. Initial intermediate592.5px was native
  ongoing scrolling, not the endpoint.
- Entry samples show opacity0.168→0.776→0.969→1 while scrolling. Exit samples
  show1→0.824→0.079→0 after the500ms idle flag. Native scroll movement is sampled,
  not replaced by custom easing.
- Found and fixed initialization: hidden tracks were measured before being made
  visible. They now become visible before measurement; first-focus/resting thumb
  is57.12px on a240px track with1000px content, without needing an initial scroll.
- Pointer thumb drag100px moved scrollTop420px. Track click near the bottom reached
  exactly760/760. Native wheel-up returned to the start.
- Horizontal keyboard scroll moves40px once settled. Both-axis content remains
  inside its viewport; nonoverflow content hides the custom bar.
-390px: body/client375px, viewport325px; long text wraps and thumb adapts to36.61px.
  Native horizontal gallery has1432px content inside325px viewport; board1000px
  remains internally scrollable. Compact layout is not Android evidence.
- Both usage copies match clipboard exactly,236 and390 characters. Early reads
  captured the previous clipboard value; rechecks after completion passed.

[Raw samples](layout-helpers-samples.json),
[desktop](evidence/layout-helpers/scroll-desktop.png),
[390px](evidence/layout-helpers/scroll-390.png).
The final RTL follow-up and build timings are recorded below.

## Build and unresolved scope

Actual dx serve succeeded after the initialization fix29.48s. The refreshed
independent export and all91 snippets passed7.64s. The unused ChipSize import is
pre-existing. No separate dx build ran into the live serve output.

Android remains paused; native touch scrolling/coarse hit areas, audible screen
readers, actual OS Reduce Motion, forced colors, zoom, nested scroll chaining,
scroll snapping, overscroll thumb compression and dynamic axis changes remain
unverified. Full Base UI render/ref/imperative API parity is not claimed. ScrollBar
is associated with the enclosing ScrollArea runtime; use the automatic bars for
ordinary composition. Dynamic additions/removals of bars require remounting.

## Reproduce

Run `./scripts/dev.sh`, wait for successful build, reload Content → Scroll areas.
Focus Recent activity before scrolling: thumb must already have positive height.
Use PageDown/Home/End, drag the thumb, jump using the track and use a wheel. Check
horizontal/both/no-overflow examples and390px bounds. Move focus/pointer out and
sample opacity during the fade. Copy both snippets and include the runtime/CSS.

## Final follow-up

Final Dioxus build22.68s, CSS hotreload success, one app-shell after reload.
At1280px the RTL viewport starts scrollLeft0, thumb aligned to the right; native
ArrowLeft reaches−40. Dragging the mirrored thumb50px left reaches−134.5; a native
wheel gesture on the track reaches−674 (maximum674), and ArrowRight returns−634.
The custom-track wheel delta follows the physical RTL sign. Saved
[RTL screenshot](evidence/layout-helpers/scroll-rtl.png) and raw values establish
this specific desktop case; broader RTL/touch/permutation coverage remains open.
The final refreshed91-example Wasm check passed8.04s; only the existing unused
ChipSize import warning.
