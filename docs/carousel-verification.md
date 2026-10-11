# Carousel verification — 2026-10-11

## Scope

`src/components/carousel.rs`, `carousel.js`, `assets/carousel.css` and
`src/visual_gallery.rs`. Standard Carousel with horizontal/vertical orientation,
per-view sizing, disabled state/items, stable-key presentation entries and
native item activation.

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

The upstream standard carousel delegates scrolling, selection and drag physics
to Embla. This port uses native scroll/snap plus a small owned runtime; it does
not claim identical Embla drag physics, plugin APIs or loop behavior.

- Standard slides span the available track (or per-view fraction);16px spacing,
  horizontal/vertical layouts, scroll-snap and overflow clipping.
- Native button item labels include the original label and index/count. Content
  inside an entry must be presentational, with no nested interactive controls.
- Controls are below content with48px targets and boundary disabling, rather
  than upstream's absolute arrows outside the viewport. This also follows the
  current Material advice against overlay/beside controls for its carousels.
- Arrow keys/Home/End navigate items; Tab remains native. Disabled items are
  skipped; offscreen items are inert until brought into view. No autoplay.
- Mouse drag begins only after a6px threshold, captures its pointer, suppresses
  the drag's click and snaps on release. Cancellation restores its starting
  offset. Touch uses native scrolling; actual Android remains untested.
- Reduced motion uses instant programmatic scroll. Native UA smooth scrolling
  supplies normal movement; no guessed duration is claimed as a Material token.
- Item Ripple reuses the accepted shared runtime. Nav finite corners24/8px use
  DefaultEffects240ms. Shared standalone Button/Ripple behavior is unchanged.

## Verification matrix

| Check | Result |
| --- | --- |
| Horizontal | 607px slides,16px gaps; Next settles at623px |
| Vertical | ArrowDown focuses Beacon and settles at276px (260+16) |
| Boundaries | End reaches item8; final Next disabled |
| Activation | Native click/Enter update Selected; real drag does not activate |
| Drag focus | Final mouse drag snaps to Beacon with focus retained; previous item inert |
| Disabled | Restricted item and both nav controls disabled |
| Copy | Standard snippet matches clipboard exactly |
| Other branches | Empty/dynamic/per-view, RTL and reduced standard scroll remain untested |

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

Embla loop/plugin/API compatibility and identical momentum physics are not
implemented. Native smooth-scroll timing is browser-dependent. Wheel, touch,
held/repeated activation, cancellation and dynamic
items/fonts/size changes need runtime evidence. Standard390px and reduced-motion
scroll have not been separately sampled. Audible screen-reader checks,
RTL, text zoom and forced colors remain open.

## Reproduce

Open `/#carousels`. Activate a slide, use Previous/Next until boundaries, then
navigate with ArrowLeft/Right/Home/End. Repeat the vertical example with
ArrowUp/Down. Drag the track and verify that a drag does not activate an item.
Check the disabled example, resize to390px, enable Reduce Motion and compare
scroll samples. Restore the preference and copy the Rust snippet.
