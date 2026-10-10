# Checkbox — current fidelity update

## Accessibility follow-up — 2026-10-10

See [the accessibility audit](accessibility-verification.md) and
[raw samples](accessibility-samples.json) for tested behavior and remaining gaps.


Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/checkbox/overview)
- [Material specs](https://m3.material.io/components/checkbox/specs)
- [Material guidelines](https://m3.material.io/components/checkbox/guidelines)
- [Material accessibility](https://m3.material.io/components/checkbox/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/checkbox.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R3/R4; reserve 48px target around 40px circular state layer and 18px box; expanding ripple. Space checks the initially unchecked example and reaches ripple .10. Disabled pointer remains unchecked with no ripple.

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

# Checkbox verification

Scope: `Checkbox` in [`src/components/checkbox.rs`](../src/components/checkbox.rs)
and [`assets/checkbox.css`](../assets/checkbox.css). Verified in the Dioxus web
preview on 2026-10-09 with headless Chromium 1194 driven by real pointer and
keyboard input. Desktop only. Android Chrome has **not** been tested.

## 1. References

- Material Web checkbox tokens, read from the official repository at
  [`tokens/versions/latest/sass/_md-comp-checkbox.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-checkbox.scss)
  (revision `47adb65`).
- Material Web state-layer tokens:
  [`_md-sys-state.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-sys-state.scss).
- shadcn-m3e checkbox implementation:
  [`checkbox.tsx`](https://github.com/Crysta1221/shadcn-m3e/blob/8f1b3fb/packages/m3e/src/components/checkbox.tsx)
  (revision `8f1b3fb`), used for the stroke-draw check mark and timing.
- **Not read:** the official Material checkbox overview, specs, guidelines and
  accessibility pages at `m3.material.io`, which the egress proxy blocks for
  this session.

## 2. Reference contract and implemented values

| Property | Implemented | Source |
| --- | --- | --- |
| Container | 18 × 18px, 2px corner radius | tokens: `container-size`, `container-shape` |
| Outline | 2px unselected; 0px when selected (container fills the same 18px) | tokens: `unselected-outline-width`, `selected-outline-width` |
| State layer | 40px circle, centred | tokens: `state-layer-size`, `state-layer-shape` |
| Icon | 18px check or dash, `currentColor` | tokens: `icon-size` |
| Unselected outline | `on-surface-variant` | `unselected-outline-color` |
| Unselected hover / focus outline | `on-surface` | `unselected-hover-outline-color`, `unselected-focus-outline-color` |
| Unselected pressed outline | `on-surface` | `unselected-pressed-outline-color` |
| Unselected pressed state layer | `primary` | `unselected-pressed-state-layer-color` |
| Selected container and outline | `primary`, icon `on-primary` | `selected-container-color`, `selected-icon-color` |
| Selected pressed state layer | `on-surface` | `selected-pressed-state-layer-color` |
| Hover state layer | 0.08 | `hover-state-layer-opacity` |
| Focus state layer | 0.10 | `focus-state-layer-opacity` |
| Pressed state layer | 0.10 | `pressed-state-layer-opacity` |
| Disabled | 38% outline (unselected); 38% container (selected) | `disabled-*` tokens |
| Error | outline and state layer `error`; selected container `error`, icon `on-error` | `*-error-*` tokens |
| Check mark draw | `stroke-dashoffset` 1 → 0 over 240ms `cubic-bezier(.05, .7, .1, 1)` | upstream `checkbox.tsx` |
| Container colour | 200ms `cubic-bezier(.2, 0, 0, 1)` | Button pilot convention |
| Touch target | 40 × 40px button | Material state layer size |
| Focus ring | 3px `secondary`, 11px offset from the box | upstream `checkbox.tsx`. **Button pilot uses `primary`; see section 7.** |

Deliberate differences:

- **Native `<button role="checkbox">`, not `<input type="checkbox">`.** It matches
  the Button pilot and avoids the separate `indeterminate` DOM property. Form
  submission is not supported. A native input would be needed for that.
- **Indeterminate resolves to checked** when activated. The reference
  description and upstream both use this behaviour.
- **No ripple.** The state layer carries the hover, focus and pressed feedback.

## 3. Input, state and platform matrix

Measured in headless Chromium at 1280×900. Raw data is in
[`control-motion-samples.json`](control-motion-samples.json).

| Input | Starting state | Result | Measured |
| --- | --- | --- | --- |
| Pointer press, held 300ms | unselected | → selected | Pressed class on 18 samples. State layer reached 0.10 while held. Box stayed 18px. Check dash offset 1px → 0px |
| Indeterminate click | indeterminate | → selected | `aria-checked` `mixed` → `true` |
| Space key | selected | → unselected | Toggled |
| Pointer click, disabled selected | selected | unchanged | No change |
| Hover (pointer resting) | unselected | outline `on-surface`, state layer 0.08 | Confirmed through `:hover` |
| Enter key | — | Not tested | |
| Focus via Tab | — | Not tested beyond `focus()` | |
| Reduced motion | — | Not tested | Stylesheet rule only |
| Screen reader | — | Not tested | |
| Touch, Android Chrome | — | **Not tested** | |

## 4. Diagnosed defects and fixes

Two specificity bugs were found by live computed-style checks while the press
was held. The pressed state layer stayed at 0.08 rather than 0.10.

1. `.m3-checkbox--pressed:not(:disabled)` (0,2,0) lost to
   `.m3-checkbox:not(:disabled):hover` (0,3,0). Fixed by qualifying the pressed
   selector with the root class, giving 0,3,0 and placing it after the hover rule.
2. The unselected hover rule set `--checkbox-state-opacity: .08` at 0,4,0, so it
   still beat the fixed pressed rule. That duplicate was removed. The general
   hover rule already sets 0.08.

Re-probing after both fixes showed 0.10 while held.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed, no warnings |
| Usage snippet in `main.rs` compiled in a separate crate against `m3e-rust-ui` | Passed |
| `dx serve` (Dioxus CLI 0.7.10, esbuild 0.27.3, wasm-bindgen 0.2.129) | Built and serving on `127.0.0.1:8080` |
| Browser load: 0 page errors, 6 checkboxes rendered | Passed |

## 6. Evidence

- Raw motion samples: [`control-motion-samples.json`](control-motion-samples.json)
- Preview screenshot: [`controls-preview.png`](controls-preview.png). A resting
  state, not a mid-animation capture.

## 7. Unresolved differences and untested behaviour

- **Official Material pages not read.** Re-check the spec and guidelines text
  once `m3.material.io` is reachable.
- **Android touch not tested.** The Android verification in the protocol still
  needs doing for this component.
- **Focus ring differs from the Button pilot.** One colour should be chosen for
  both components.
- **Error state colour** is taken from the tokens but not visually compared with
  a reference render.
- **Reduced motion** has not been exercised with an OS-level preference.

## 8. Steps to reproduce

1. Start the dev server as in the [switch report](switch-verification.md#8-steps-to-reproduce).
2. Load `http://127.0.0.1:8080/`, then drive the checkbox: a 300ms pointer press
   on `Unchecked`, a click on `Select all` (indeterminate), a Space press on
   `Checked`, and a click on `Disabled checked`.
3. Sample `.m3-checkbox__box` width, `.m3-checkbox__state` opacity and
   `.m3-checkbox__check` `stroke-dashoffset` from `requestAnimationFrame`.
4. Compare against the table in section 2.
