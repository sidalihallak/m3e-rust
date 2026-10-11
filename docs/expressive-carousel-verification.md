# Expressive carousel verification — 2026-10-11

## Scope

`ExpressiveCarousel`, `ExpressiveCarouselVariant` and shared CarouselEntry/
CarouselSlide in `src/components/carousel.rs`, `carousel.js`, `assets/carousel.css`,
with showcase in `src/visual_gallery.rs`.

## References

Read the rendered current Material [overview](https://m3.material.io/components/carousel/overview),
[specs](https://m3.material.io/components/carousel/specs),
[guidelines](https://m3.material.io/components/carousel/guidelines), and
[accessibility](https://m3.material.io/components/carousel/accessibility) on
2026-10-11. Also read the [WAI carousel pattern](https://www.w3.org/WAI/ARIA/apg/patterns/carousel/).

Inspected the entire upstream [standard carousel](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/carousel.tsx)
and [expressive carousel](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/expressive-carousel.tsx).
Confirmed revision c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972.

## Reference contract and implemented values

The current official catalog lists six layouts; upstream implements four.
This port implements upstream's multi-browse, hero, uncontained and full-screen.
Center-aligned hero and uncontained multi-aspect ratio remain extensions.

- Multi-browse/hero: container surface,16px leading/trailing and8px top/bottom
  margins;8px gaps;28px corners. Defaults preferred large width186px and item
  height221px match upstream. Small width56px is in the official40–56 range.
- Multi-browse preferred large/medium/small sizing retains the upstream
  arrangement; hero has one large plus56px preview. Current scrolling maps
  evenly spaced logical item centers through size keylines. End states move
  trailing previews before the focal range, with shift intervals determined by
  keyline distance. This supersedes the original `widthAt` layout and subsequent
  per-item boundary blends. Content stays large-sized and centered in its mask.
- Range uses the unmasked content length minus the viewport. The last snap
  lands at that edge; a redundant terminal snap within one inter-item gap is
  merged into it. End focuses the actual last item. Device-pixel precision is
  normalized at the edge to avoid leaving a fractional terminal mask.
- Brief titles hide on small items; supporting text hides on medium/small items;
  native labels still name the full item and index/count.
- Uncontained uses fixed-width items and native free scrolling (official default
  behavior) with leading16px/vertical8px spacing, rather than mandatory snapping.
- Full-screen uses vertical snap,0px exterior padding/0px item corners and16px
  gaps, following current official guidance instead of upstream's horizontal
  rounded/8px-gap interpretation. The gallery is a bounded portrait preview;
  consumers supply height for a real immersive region. Landscape/full-window
  integration is not verified or enforced by this primitive.
- Reduced motion switches dynamic multi-browse/hero to fixed native items,
  removes parallax/masking and uses instant programmatic scroll. Reduced hero
  leaves a partial preview. Source did not implement this required layout change.
- Show all callback is required for non-full-screen variants and appears below
  the content; gallery opens a native full-screen vertically scrolling list of
  all8 items with the title retained. Full-screen omits Show all (official exception).
- Initial focus is on native item buttons, never the track. Arrows/Home/End
  navigate; horizontal Up/Down moves to the nearby Show all control. All-visible
  alternatives and controlled activation remain app state.

## Verification matrix

| Check | Result |
| --- | --- |
| Desktop keylines | 575px track: large186.80, medium121.40, small56px;28px corners |
| Continuous masking | ArrowRight changes widths/transforms continuously; inner width stays186.80px |
| Scroll settle | Native scroll settles near195px after roughly428ms; UA timing, not a Material duration |
| End / focus | End reaches Harbor item8; Home returns; offscreen items inert |
| Activation / navigation | Enter selects Beacon; horizontal ArrowDown focuses Show all |
| Show all | Full-screen dialog lists all8 articles; Escape returns focus to Show all |
| Full-screen variant | Vertical ArrowDown settles496px (480+16), focus Beacon;0px corners |
| Uncontained | Free horizontal scroll; final trailing margin0px |
| Actual Reduce Motion | Dynamic false; multi items186px, hero567px, transforms none; scroll immediate |
| 390px | Multi/hero previews remain visible; titles fit; body/client375px with no overflow |
| Copy | All four snippets match clipboard exactly |

## Build and evidence

Dioxus 0.7.10 / Rust 1.99 built the final preview successfully in 19.35s;
reloaded before final checks. The refreshed independent library and all **75**
usage examples passed the locked offline Wasm check in **8.45s**. One existing
unused ChipSize import warning remains. No chart/carousel dependency was added.

Actual Codex in-app-browser inputs and read-only DOM samples are saved in
[visual-family-samples.json](visual-family-samples.json), with screenshots in
[evidence/visual](evidence/visual/). Earlier local access failed from an internal
data error page; the user returned the tab to the HTTP preview and inspection
resumed. No alternate browser or policy bypass was used. A stale rebuild toast
prompted a preview restart; the mounted application and final screenshots are
healthy. The preview remains running.

All **nine** new copy controls produced clipboard text exactly matching the
displayed code. The first read preceded clipboard completion; repeating after
completion passed. OS Reduce Motion was temporarily enabled under prior user
approval, its browser media query verified, then restored to its original off
state. Android remains paused; 390px is desktop responsive evidence only.

## Remaining differences and gaps

Centered hero, multiple aspect ratios, full Compose keyline equivalence,
Embla/Compose physics equivalence, landscape adaptation and app full-window
composition remain extensions. Desktop samples establish masking,
visibility and offscreen inertness for the tested paths. Touch cancellation,
reversal/rapid interruption and dark carousel screenshots remain open. Screen-reader speech, RTL, text
zoom, disabled transitions, many/very few items and Android remain unverified.

## Reproduce

Open `/#expressive-carousels`. Inspect all four layouts; use click/Tab/arrows,
Home/End and drag. Sample item widths, transforms and content width while
scrolling, including reversal and the final keyline. Open Show all, read all8
items vertically, activate an item and test Escape/focus return. Enable actual
OS Reduce Motion and repeat, verifying equal fixed widths and no centered-mask
transform; restore the preference. Repeat at390px and copy all four snippets.

## Terminal layout correction — 2026-10-11

User observed excessive blank space after the last item. Read the current
rendered official overview/specs/guidelines/accessibility pages linked above
again and inspected the pinned upstream source. Material requires contained
items to remain recognizable and snap into their layout; the docs do not
prescribe this interpolation algorithm. Upstream restricts maxIndex and lacks
our last-item-large path, so copying its scroll limit would remove that path.

Cause: our extended range let the final item occupy the leading keyline after
all following previews disappeared. The measured multi-browse trailing gap was
**598.55px** on a **791.69px** track. `carousel.js` now blends widths and positions
into the mirrored initial hierarchy during the final scroll step. Earlier
scroll geometry, native scroll timing and fixed inner content widths are
preserved; no animation duration or easing was changed. Native/reduced-motion,
uncontained and full-screen branches are unchanged.

Verification after successful Dioxus build (**29.81s**) and reload:

- Desktop multi-browse and hero end at item8, retaining Harbor focus. Trailing
  edge error is **-0.31px** (fractional DOM width versus rounded clientWidth).
- Multi-browse final hierarchy:56px small,124.57px medium, three193.14px large
  items with8px gaps. Hero final hierarchy:56px preview,8px gap,728px large.
- Actual ArrowLeft/ArrowRight inputs across the final step and time samples
  show continuous widths/transforms. End and repeated ArrowRight stay bounded.
  Enter updates `Selected: Harbor` without losing focus.
- At390px, both tracks are263px. Multi-browse trailing error is0.22px; hero0px.
  Reverse/forward input also works at this width. Viewport override restored.
- Independent Wasm export and all75 examples compile after the JS refresh.
  The shared Rust API and copy snippets have not changed.

Raw evidence: [carousel-end-samples.json](carousel-end-samples.json).
Screenshots: [desktop end](evidence/visual/expressive-end-desktop.png),
[390px end](evidence/visual/expressive-end-390.png).
These are desktop browser checks. Android remains paused. Actual reduced-motion
was verified in the earlier family pass; it was not toggled again for this
boundary-only change. Very few items, rapid interruptions and RTL remain open.

## Boundary motion regression correction — 2026-10-11

The user reported that the preceding terminal-gap fix broke motion. This
supersedes the earlier claim that its continuous dimensions established good
boundary motion: continuity alone did not detect direction reversals.

Cause: `normal(s) + (ending - normal(s)) * blend(s)` interpolated an already
moving width/position. It produced quadratic shrink-then-expand paths, and the
multi-browse range extended past the point where all final large items fit.

Correction in `carousel.js`: use a fixed penultimate layout and fixed ending
layout for the last interval. Limit the dynamic range to `count - big` steps;
keyboard End still focuses the actual last item, and the final large group
remains visible. Hero retains its one-item steps. Earlier intervals and UA
smooth-scroll timing are unchanged. No duration/easing token was invented.

After successful Dioxus build (**26.27s**) and reload, actual keyboard inputs
were sampled across End, reverse, and final-step forward navigation:

| Input/layout | Sampled width movement | Direction reversals |
| --- | --- | --- |
| Desktop multi-browse | Echo193.14→124.64; Harbor124.55→193.06px | 0 |
| Desktop hero | Garden728→56; Harbor56→728px | 0 |
| 390px multi-browse | Garden108.55→82.37; Harbor82.45→108.63px | 0 |
| 390px hero | Garden198.65→56; Harbor56.34→199px | 0 |

Desktop multi-browse now stops at scroll1005.5px (previous1408px), while End
focuses Harbor8of8. The final edge error remains below1px. Phone multi-browse
trailing error0.095px, hero0px. Enter activation and focus retained; viewport
override restored. Raw all-item samples are in
[carousel-boundary-motion-samples.json](carousel-boundary-motion-samples.json).
Proof: [desktop](evidence/visual/expressive-boundary-desktop.png),
[390px](evidence/visual/expressive-boundary-390.png).

The Material/source contract was read in the immediately preceding boundary
fix, on the same date. Fixed interpolation is an owned implementation choice,
not a claim of numerical Compose parity. Reduced-motion and native branches
are unchanged and were not retested here. Android and interruption/RTL/few-item
coverage remain open. Refreshed independent library and all75 usage examples
compiled successfully in6.84s; the existing unused ChipSize warning remains.

## Material keyline follow-up — 2026-10-11

User still observed a different size adjustment at the end. Re-read the rendered
[official guidelines](https://m3.material.io/components/carousel/guidelines),
including its multi-browse video, and inspected the official Android
[KeylineStateList implementation](https://github.com/material-components/material-components-android/blob/master/lib/java/com/google/android/material/carousel/KeylineStateList.java)
(`getShiftedState`, end-step generation and distance-based interpolation).
The overview/specs/accessibility and pinned shadcn source were read during the
preceding same-day pass. Material shifts the focal keyline range near boundaries;
our two per-item blends had not implemented that model. This section supersedes
their fidelity conclusions. Their saved measurements remain historical evidence.

Current owned runtime models logical centers at large-item spacing and maps them
through interpolated physical centers/mask widths. End states reposition one
trailing preview keyline at a time before the focal range. Shift distances come
from those state positions, not a guessed duration or extra end animation.
Focal items retain large masks while other items enter/leave that range. The
underlying UA scroll behavior, color roles,28px corners and centered content
remain. This is a browser adaptation, not full Android/Compose physics parity.

Two additional causes were corrected: a redundant edge snap less than an8px
item gap from the preceding snap, and device-pixel quantization leaving masks
slightly short of their terminal widths. The final target replaces that redundant
snap; geometry within one device pixel uses the exact terminal state.

Runtime evidence after final successful Dioxus build (**16.67s**) and reload:

- First-step focal Beacon stayed **185.195px** throughout22 samples.
- Boundary Garden grew **120.695→185.195px**; Harbor **56.094→185.094px** in
  the pre-precision pass. Both had zero width-direction reversals across27
  samples. Final precision pass has Garden/Harbor both **185.195px** at rest.
- Desktop native mouse drag backward/forward reaches the edge with no trailing
  blank region. Final edge discrepancy **-0.105px** reflects DOM/client rounding.
- 390px first/end keyboard motion, Home/End, focus and Enter activation checked.
  Final Harbor **108.664px**, trailing discrepancy **0.174px**. Hero **199px**
  with0px trailing discrepancy. Actual last item remains focusable; offscreen
  items remain inert. Viewport override restored; one `.app-shell` confirmed.
- Refreshed independent Wasm library and all75 examples compile in **6.36s**;
  the existing unused ChipSize warning remains. Preview remains running.

[All-item samples](carousel-keyline-samples.json),
[desktop proof](evidence/visual/expressive-keylines-desktop.png),
[390px proof](evidence/visual/expressive-keylines-390.png).
Earlier boundary-only reports did not measure the right contract: zero reversals
alone does not establish Material fidelity. No screenshot proves timing. Android,
RTL, few-item cases, rapid interruption and audible screen-reader coverage remain
open. Native/reduced-motion branches were unchanged and not retested in this pass.
