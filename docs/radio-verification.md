# Radio — current fidelity update

## Accessibility follow-up — 2026-10-10

See [the accessibility audit](accessibility-verification.md) and
[raw samples](accessibility-samples.json) for tested behavior and remaining gaps.


Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/radio-button/overview)
- [Material specs](https://m3.material.io/components/radio-button/specs)
- [Material guidelines](https://m3.material.io/components/radio-button/guidelines)
- [Material accessibility](https://m3.material.io/components/radio-button/accessibility)
- [Actual upstream source at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/radio-group.tsx)
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R3/R4; native input 48×48 around unchanged 40px state layer/20px ring; expanding ripple for click/Space. ArrowRight Medium → Large selects and focuses Large. Native arrows keep browser selection semantics.

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


## 7. Centring across viewport and zoom (2026-10-09, update)

Reported: the radio looked centred at first, then off-centre after resizing.

- **Viewport width:** the ring is measured at widths from 320 to 1280px. The layout
  offset is 0 at every width, so resizing the window does not move the layout.
- **Browser zoom:** the first version drew the ring and dot as boxes. Their edges
  rounded to device pixels, so the drawn centre could shift by up to one device pixel
  at 80% and 90% zoom.
- **Fix:** the ring and dot are now SVG circles on a 20-unit grid, and the focus ring
  sits on the 40dp state layer.
- **After the fix:** the ring's ink centre is within 0.8 device pixels of the expected
  centre at 67%, 80%, 90%, 100%, 110%, 125% and 150% zoom. Measurement uses integer
  pixel bounds, which resolve about 0.5 pixel, so most of the residual is measurement
  noise. Raw data: [`radio-zoom-samples.json`](radio-zoom-samples.json).
- **Not verified:** Safari and Firefox, and the visual result on the user's device.
