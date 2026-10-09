# Fab Family — current fidelity update

Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/fab-menu/overview)
- [Material specs](https://m3.material.io/components/fab-menu/specs)
- [Material guidelines](https://m3.material.io/components/fab-menu/guidelines)
- [Material accessibility](https://m3.material.io/components/fab-menu/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/fab-menu.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R8 and ripple; trigger-first DOM, forward Tab to own items, arrows/Home/End, selection/Escape return focus, absolute content with 56px root, official item state elevation. Glyphs crossfade/rotate using 240ms DefaultEffects; items 440ms DefaultSpatial +35ms stagger; trigger 360ms FastSpatial. ArrowDown Favourite → Details; End Add; Escape Actions/closed.

## Checks, dependencies and gaps

- Locked offline Wasm check and fresh Dioxus web build passed; live preview
  was restarted/reloaded and checked for one app-shell.
- [Shared fix matrix, measured results and reproduction steps](component-fidelity-fixes.md)
- [Raw runtime measurements](component-fidelity-fixes-samples.json)
- [Copy dependencies and source export](copy-components.md); exported library
  and all 19 current usage examples compile independently for Wasm.
- Platform scope: desktop pointer/keyboard checks named above. Android remains
  paused; touch, assistive technology and OS reduced-motion true are untested.
  Source gating is not claimed as an OS-preference runtime measurement.
- Historical screenshots/samples below establish only the state and version
  in which they were captured. See the shared fix report for current gaps.

## Historical report before this fix

# FAB family verification

Scope: `Fab` with `branded` and `toolbar` options, and `FabMenu` in
[`src/components/fab_menu.rs`](../src/components/fab_menu.rs), styled by
[`assets/fab.css`](../assets/fab.css) and [`assets/fab-menu.css`](../assets/fab-menu.css).
Verified in the Dioxus preview on 2026-10-09 with headless Chromium 1194, using
real pointer and keyboard input. Desktop only. Android Chrome has not been tested.

## References

- Material Web tokens, revision `47adb65`: `_md-comp-fab-branded*.scss`,
  `_md-comp-toolbar-floating-fab.scss`, `_md-comp-fab-menu*.scss`.
- shadcn-m3e `fab-menu.tsx`, revision `8f1b3fb`: open trigger shape, item layout,
  35ms stagger.
- **Not read:** the official Material pages at `m3.material.io`. The egress
  proxy blocks that host.

## Values

| Variant | Measured | Expected |
| --- | --- | --- |
| Branded standard | 56px, radius 16px, icon 36px, surface-container-high | 56px, 16px, 36px, surface-container-high |
| Branded large | 96px, radius 28px, icon 48px | 96px, 28px, 48px |
| Toolbar standard, rest | elevation 1 | elevation 1 |
| Toolbar standard, hover | elevation 2 | elevation 2 |
| Toolbar medium, rest | 80px, radius 20px, icon 28px, elevation 2 | 80px, 20px, 28px, elevation 2 |
| Toolbar medium, hover | elevation 3 | elevation 3 |
| Toolbar colours | secondary-container and tertiary-container, matching tokens | same |
| Menu closed | items opacity 0, not focusable, trigger radius 16px, primary-container | same |
| Menu open | trigger radius 28px, primary background, 20px close icon | same |
| Menu item | 56px tall, 24px side padding, 28px radius, 8px icon-label gap, title-medium 500 16/24 | same |
| Menu spacing | 4px between items | 4px |
| Stagger | nearest item reaches half opacity first: 113ms, 146ms, 185ms top to bottom | nearest first, 35ms apart |
| Select an item | reports index, closes the menu, updates the status line | same |
| Escape | closes the open menu | same |
| Tertiary menu | trigger and items use the tertiary roles | same |

Raw data: [`fab-family-samples.json`](fab-family-samples.json). Section
screenshot: [`fab-family-section.png`](fab-family-section.png). The screenshot
shows a resting state, not a mid-animation capture.

## Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed |
| FAB menu usage snippet compiled against the public API | Passed |
| Browser load | 0 page errors |

## Unresolved

- Branded icon colour: the token set defines no icon colour for the branded
  container. The kit uses primary. Override `--fab-icon` for a brand colour.
- Trigger glyph crossfade/rotation is implemented in the current update above.
- Medium and large toolbar elevations were measured at rest and on hover only.
- Android touch and reduced motion have not been tested.
- The official Material pages were not read.

## Update: press ripple and outside-click dismissal

Both were added after the first round, and both were checked in the browser on the fresh build.

| Check | Result |
| --- | --- |
| FAB pointer press | Ripple starts at the pointer, grows to full size (scale about 14.9 for the 11px start), opacity 0.10 while held, fades to 0 after release. Raw: [`fab-ripple-samples.json`](fab-ripple-samples.json) |
| FAB Space press | Ripple starts at the centre, opacity 0.10 while held |
| Disabled FAB press | No ripple |
| Menu: press on the top bar while open | Closes |
| Menu: press on the page body while open | Closes |
| Menu: trigger press | Opens, then closes on the next press |
| Menu: item press | Reports the item and closes |
| Page errors | 0 |

Raw outside-click results: [`fab-menu-outside-samples.json`](fab-menu-outside-samples.json).

The FAB menu listens for pointer presses at document level while it is mounted.
The listener is removed when the menu unmounts.

Not yet measured: the expressive shape morph. The upstream reference does not
morph the FAB corner on press. I have not added one, because I could not confirm
it from the official guidelines.
