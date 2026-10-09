# Fab — current fidelity update

Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/floating-action-button/overview)
- [Material specs](https://m3.material.io/components/floating-action-button/specs)
- [Material guidelines](https://m3.material.io/components/floating-action-button/guidelines)
- [Material accessibility](https://m3.material.io/components/floating-action-button/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/fab.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R3/R4; small target extended/reserved to 48px, common minimum press/ripple lifecycle. Standard Enter and quick click produce ripple .10 at unchanged 56×56px. Branded/toolbar/extended options remain implemented; no native Android verification.

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

# Floating action button verification

Scope: `Fab`, `FabSize`, `FabColor` in [`src/components/fab.rs`](../src/components/fab.rs)
and [`assets/fab.css`](../assets/fab.css), using the icon set in
[`src/icons.rs`](../src/icons.rs). Verified in the Dioxus web preview on
2026-10-09 with headless Chromium 1194, using real pointer and keyboard input.
Desktop only. Android Chrome has **not** been tested.

## 1. References

- Material Web FAB tokens, read from the official repository
  ([`_md-comp-fab*.scss`](https://github.com/material-components/material-web/tree/47adb65/tokens/versions/latest/sass),
  revision `47adb65`). These give container sizes (40/56/80/96dp), corner radii
  (12/16/20/28dp), icon sizes (24/24/28/36dp), colour roles, and state-layer opacities.
- Material elevation levels from Material Web's elevation implementation
  ([`_elevation.scss`](https://github.com/material-components/material-web/blob/47adb65/elevation/internal/_elevation.scss)):
  level 1 at rest when lowered, level 3 at rest, level 4 on hover.
- shadcn-m3e FAB implementation
  ([`fab.tsx`](https://github.com/Crysta1221/shadcn-m3e/blob/8f1b3fb/packages/m3e/src/components/fab.tsx),
  revision `8f1b3fb`), and its shadow tokens
  ([`m3e.css`](https://github.com/Crysta1221/shadcn-m3e/blob/8f1b3fb/packages/m3e/src/styles/m3e.css)).
  The ambient shadow component comes from here.
- **Not read:** the official Material FAB overview, specs, guidelines and
  accessibility pages at `m3.material.io`. The egress proxy blocks that host.

## 2. Reference contract and implemented values

| Property | Implemented | Source |
| --- | --- | --- |
| Size S / standard / M / L | 40 / 56 / 80 / 96px | Material Web tokens |
| Corner radius | 12 / 16 / 20 / 28px | Material Web tokens |
| Icon size | 24 / 24 / 28 / 36px | Material Web tokens |
| Container and icon colour | primary, primary-container, secondary, secondary-container, tertiary, tertiary-container, surface (`surface-container-high` with `primary` icon) | Material Web tokens |
| State layer colour | Icon colour (`currentColor`) | Material Web tokens |
| State layer | hover 0.08, focus 0.10, pressed 0.10 | Material Web state tokens |
| Elevation at rest | level 3: `0 1px 3px 0 .3, 0 4px 8px 3px .15` | elevation spec and upstream |
| Elevation on hover | level 4: `0 2px 3px 0 .3, 0 6px 10px 4px .15` | elevation spec and upstream |
| Elevation pressed | level 3 (back from hover) | Material Web `pressed-container-elevation` |
| Lowered rest / hover / pressed | level 1 / level 2 / level 1 | Material Web `lowered-*-elevation` |
| Disabled | container 12% on-surface, icon 38% on-surface, no shadow | **Upstream fab.tsx.** The Material Web FAB token files do not define disabled values. |
| Extended FAB S / standard / M / L | Heights 56 / 56 / 80 / 96px. Corners 16 / 16 / 20 / 28px. Icons 24 / 24 / 28 / 36px. Leading and trailing space 16 / 20 / 26 / 28px. Icon-label gap 8 / 12 / 12 / 16px. Labels: label-large 14/20 (500), title-medium 16/24 (500), title-large 22/28 (400), headline-small 24/32 (400). | Material Web extended FAB tokens and the Material Web type-scale tokens. The label typography in the upstream reference differs; the official values are used. |
| Focus ring | 3px `secondary`, 2px offset | Upstream `focus-ring`. The Button uses `primary`. |
| Press model | Pointer and Space set the pressed class. No minimum press time. | Same as switch and checkbox |
| Touch target | Container size (40dp minimum for small) | Material size |

Deliberate differences from the upstream reference:

- **Icon size at L is 36px**, from the official tokens. Upstream uses 32px.
- **Corner radius at M is 20px and at L is 28px**, from the official tokens.
  Upstream uses its own rounded scale.
- **No ripple.** Feedback is the state layer and elevation.
- **No pressed shape morph.** The FAB keeps its corner radius while pressed.
- **Extended heights.** An extended small FAB is 56px tall, not 40px. Upstream
  does not specify a separate extended small size.

## 3. Input, state and platform matrix

Raw data: [`fab-motion-samples.json`](fab-motion-samples.json).

| Check | Result |
| --- | --- |
| Sizes S, standard, M, L | 40, 56, 80, 96px. Radii 12, 16, 20, 28px. Icons 24, 24, 28, 36px |
| Colour roles (7) | Each container matches its theme token exactly, as hex |
| Elevation at rest (standard) | level 3 shadow |
| Hover | state layer 0.08, shadow level 4 |
| Pointer held 350ms | state layer 0.10, pressed class on, shadow level 3 |
| Pointer released (still hovered) | state layer 0.08, shadow level 4 |
| Space held 300ms | pressed class on, state layer 0.10 |
| Space released | pressed class off |
| Lowered at rest | shadow level 1 |
| Lowered hover | shadow level 2 |
| Disabled | no shadow; pointer press does not set the pressed class |
| Extended S / standard / M / L | Measured in the browser: heights, corners, icon sizes, paddings, icon-label gaps and label type all match the tokens. Raw data: [`fab-extended-samples.json`](fab-extended-samples.json). |
| Pointer click calling `onclick` | **Not tested.** The preview has no handler, and the handler wiring is not asserted. |
| Enter key | Not tested |
| Focus via Tab | Not tested beyond `focus()` |
| Reduced motion | Stylesheet rule only |
| Screen reader | Not tested |
| Touch, Android Chrome | **Not tested** |

## 4. Diagnosed defects

None in this round. The pressed and lowered selectors are qualified
(`.m3-fab.m3-fab--pressed:not(:disabled)`, `.m3-fab.m3-fab--lowered:not(:disabled):hover`)
so they outrank the hover rules, the same fix that the switch and checkbox
needed. Each state was checked with live computed styles.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --target wasm32-unknown-unknown` | Passed, no warnings |
| FAB usage snippet compiled in a separate crate against `m3e-rust-ui` | Passed |
| `dx serve` | Built and serving on `127.0.0.1:8080` |
| Browser load | 0 page errors, 14 FABs rendered |

## 6. Evidence

- Raw measurements: [`fab-motion-samples.json`](fab-motion-samples.json)
- Extended measurements: [`fab-extended-samples.json`](fab-extended-samples.json)
- Section screenshot: [`fab-section.png`](fab-section.png). A resting state, not
  a mid-animation capture.

## 7. Unresolved differences and untested behaviour

- **Official Material pages not read.** Check the FAB spec and guidelines
  text once `m3.material.io` is reachable.
- **Click handler not tested.** The `onclick` path needs an assertion.
- Branded, toolbar and menu variants are implemented; see the current update above and the FAB family report.
- **Shadow transition not sampled mid-animation.** The transition is 200ms, but the
  intermediate shadow values were not captured.
- **Android touch and reduced motion** have not been tested.
- **Focus ring colour** differs from the Button pilot, as for the other new components.

## 8. Steps to reproduce

1. Start the dev server as in the [switch report](switch-verification.md#8-steps-to-reproduce).
2. Load `http://127.0.0.1:8080/`. In the FAB section, hover, press and hold a
   standard FAB, release, then use Space on the same button.
3. Read `box-shadow` and the `::before` opacity during each step.
4. Compare against the table in section 2.
