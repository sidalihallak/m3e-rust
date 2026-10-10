# Switch — current fidelity update

## Accessibility follow-up — 2026-10-10

See [the accessibility audit](accessibility-verification.md) and
[raw samples](accessibility-samples.json) for tested behavior and remaining gaps.


Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/switch/overview)
- [Material specs](https://m3.material.io/components/switch/specs)
- [Material guidelines](https://m3.material.io/components/switch/guidelines)
- [Material accessibility](https://m3.material.io/components/switch/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/switch.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R4; native switch semantics and existing extended target retained; expanding ripple. Wi-Fi pointer toggles to checked with peak ripple .10. Drag selection has not been implemented or verified.

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

# Switch verification

Scope: `Switch` in [`src/components/switch.rs`](../src/components/switch.rs) and
[`assets/switch.css`](../assets/switch.css). Verified in the Dioxus web preview
on 2026-10-09 with headless Chromium 1194 driven by real pointer and keyboard
input. Desktop only. Android Chrome has **not** been tested.

## 1. References

- Material Web switch tokens, read from the official repository at
  [`tokens/versions/latest/sass/_md-comp-switch.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-switch.scss)
  (revision `47adb65`).
- Material Web state-layer tokens:
  [`_md-sys-state.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-sys-state.scss)
  (hover 0.08, focus 0.10, pressed 0.10).
- shadcn-m3e switch implementation:
  [`switch.tsx`](https://github.com/Crysta1221/shadcn-m3e/blob/8f1b3fb/packages/m3e/src/components/switch.tsx)
  (revision `8f1b3fb`), used for the spatial spring and handle behaviour.
- **Not read:** the official Material switch overview, specs, guidelines and
  accessibility pages at `m3.material.io`. The egress proxy blocks that host
  for this session, so the values above come from the token files and the
  upstream implementation. The values are consistent between the two, but the
  official pages still need checking.

## 2. Reference contract and implemented values

| Property | Implemented | Source |
| --- | --- | --- |
| Track | 52 × 32px, 2px outline, full radius | tokens: `track-width`, `track-height`, `track-outline-width`, `track-shape` |
| Unselected handle | 16px, left 6px in padding box (8px from outer edge) | tokens: `unselected-handle-*` |
| Selected handle | 24px, 22px left (4px from outer right edge) | tokens: `selected-handle-*` |
| Pressed handle | 28px; 0px left unselected, 20px left selected | tokens: `pressed-handle-*` |
| Unselected with icon | 24px handle | tokens: `with-icon-handle-*` |
| Icons | 16px | tokens: `*-icon-size` |
| State layer | 40px, centred on handle, full radius | tokens: `state-layer-size`, `state-layer-shape` |
| Hover state layer | 0.08 | `hover-state-layer-opacity` |
| Focus state layer | 0.10 | `focus-state-layer-opacity` |
| Pressed state layer | 0.10 | `pressed-state-layer-opacity` |
| Disabled | 12% track and outline, 38% handle and icon | `disabled-*-opacity` tokens |
| Handle motion | 240ms spring (`linear()` curve from the Material DefaultEffects spring) | upstream `transition-shape`; the same curve as the Button |
| Colour transitions | 200ms `cubic-bezier(.2, 0, 0, 1)` | Button pilot convention |
| Touch target | 48px tall, via `::after` | Material minimum; upstream `after:-inset` |
| Focus ring | 3px `secondary`, 2px offset | upstream `focus-ring`. **Button pilot uses `primary`; see section 7.** |

Deliberate differences from the reference:

- **No minimum visible press.** The Button holds its pressed state for 225ms.
  The switch shows the pressed handle only while the pointer is down.
- **No ripple.** The state layer carries the hover, focus and pressed feedback.
- **Error state is an upstream extension.** Material Web defines no switch error
  tokens. Error draws the outline in the `error` role.
- **No small size.** Upstream's `sm` 40×24 variant is not in the M3 spec and is
  not implemented.

## 3. Input, state and platform matrix

Measured in headless Chromium at 1280×900. The raw data is in
[`control-motion-samples.json`](control-motion-samples.json). Each run samples
the handle with `requestAnimationFrame` while the input is in progress.

| Input | Starting state | Result | Measured |
| --- | --- | --- | --- |
| Pointer press, held 350ms | on | → off | Handle grew 16 → 28px while held, pressed class on 21 samples, state layer reached 0.10 |
| Pointer quick tap | off | → on | Toggled. Pressed state not captured: the press is shorter than one frame sample |
| Space key, held 250ms | off | → on | Pressed from keydown at 23ms; handle reached 27.98px; state layer 0.10 |
| Pointer click, disabled | off | unchanged | No change |
| Hover (pointer resting) | any | state layer 0.08 | Confirmed through `:hover` |
| Enter key | — | Not tested | |
| Focus via Tab | — | Not tested beyond `focus()` | |
| Reduced motion | — | Not tested | Stylesheet rule only |
| Screen reader | — | Not tested | |
| Touch, Android Chrome | — | **Not tested** | |

## 4. Diagnosed defect and fix

Found during this verification: the pressed state layer stayed at 0.08, not
0.10. Live computed-style checks showed that `.m3-switch:not(:disabled):hover`
(specificity 0,3,0) outranked `.m3-switch--pressed:not(:disabled)` (0,2,0).
The pressed selector now carries the class twice, so it scores 0,3,0 and
comes after the hover rule. Re-probing after the fix showed 0.10.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed, no warnings |
| Usage snippet in `main.rs` compiled in a separate crate against `m3e-rust-ui` | Passed |
| `dx serve` (Dioxus CLI 0.7.10, esbuild 0.27.3, wasm-bindgen 0.2.129) | Built and serving on `127.0.0.1:8080` |
| Browser load: 0 page errors, 5 switches rendered, initial `aria-checked` as expected | Passed |

## 6. Evidence

- Raw motion samples: [`control-motion-samples.json`](control-motion-samples.json)
- Preview screenshot: [`controls-preview.png`](controls-preview.png). A resting
  state, not a mid-animation capture.

## 7. Unresolved differences and untested behaviour

- **Official Material pages not read.** Re-check the spec and guidelines text
  once `m3.material.io` is reachable.
- **Android touch not tested.** Protocol section 3 requires scrolling that
  starts on the control, touch targets and the on-screen keyboard. None has been
  checked.
- **Focus ring differs from the Button pilot.** The Button uses `primary`, and
  upstream uses `secondary`. One of them should be chosen and applied to both.
- **Quick taps do not show the pressed shape.** A tap shorter than one frame
  skips the pressed geometry. This is consistent with the no-minimum-press
  decision above, but it is not an M3 requirement.
- **Reduced motion** has not been exercised with an OS-level preference. The
  CSS rule disables transitions, and that is not measured.

## 8. Steps to reproduce

1. Run the dev server with `dx serve --platform web --port 8080`, with
   `esbuild` and `wasm-bindgen` on `PATH` and `NO_DOWNLOADS=1` set.
2. Load `http://127.0.0.1:8080/`, then run the Playwright script that drives
   the Switch: a pointer press with 350ms hold on `Notifications`, a quick tap,
   a Space press on `Wi-Fi`, and a click on `Disabled off`.
3. Sample `.m3-switch__handle` width and `::before` opacity from
   `requestAnimationFrame` during each input.
4. Compare against the table in section 2.

## 9. Icon centring fix (2026-10-09)

The unselected cross icon sat off-centre. The preview passed the `×` text
character as the icon. Its ink is positioned by the font's line metrics, so
it sat 1.9px below the handle centre (measured from a 4× screenshot, ink
bounding box inside the handle disc). The preview now passes SVG icons on a
24-unit grid. After the change the cross centre is 0.00px horizontally and
0.5px vertically. The remaining 0.5px is sub-pixel, because the handle sits
at a half-pixel top edge (y = 437.5).

Text glyphs are not reliably centred in a flex box. Pass SVG icons to
`checked_icon` and `unchecked_icon`. The component's own centring is correct:
the glyph line box is centred on the handle.

The check mark's ink sits about 0.6px low. That comes from the Material check
path itself, whose bounding box spans y 7–19 of 24. It was left unchanged.

Evidence: [`switch-icon-unchecked-handle.png`](switch-icon-unchecked-handle.png)
(4× scale, unselected handle with cross).

## 10. Official icons and sizing (2026-10-09)

The section 9 icons were hand-typed paths. They are now official Material Symbols
Rounded glyphs, generated by `scripts/gen-icons.mjs` from
`@material-symbols/svg-400@0.48.0` (Apache-2.0, Google). The upstream reference
uses the same glyphs; its `check-rounded` and `close-rounded` paths match the
official files exactly.

Sizing: the switch sets `--m3-icon-size: 16px` on its icon. Two cascade
problems came up while doing this. An `.m3-icon` rule that also set the variable
overrode the parent's value, and the icon stylesheet's width rule overrode the
switch's width rule. The icon stylesheet now uses `width: var(--m3-icon-size, 24px)`
with no default on the element.

Measured after the change, from 4× screenshots: the unselected cross ink is
8.5px across and centred at 0.00px horizontally and +0.50px vertically. The
selected check is centred at 0.00px horizontally and +0.50px vertically. The
+0.5px is sub-pixel, from the handle's half-pixel top edge (y = 437.5).

Evidence: [`icon-specimen.png`](icon-specimen.png).
