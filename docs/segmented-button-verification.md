# Segmented-button centering correction — 2026-10-09

## Scope and source contract

Follow-up alignment fix in `src/components/segmented_button.rs` and
`assets/segmented-button.css`. Read the official
[overview](https://m3.material.io/components/segmented-buttons/overview),
[specs](https://m3.material.io/components/segmented-buttons/specs),
[guidelines](https://m3.material.io/components/segmented-buttons/guidelines), and
[accessibility](https://m3.material.io/components/segmented-buttons/accessibility)
in the browser on this date. Inspected the actual Material Web labs
[shared layout source at 47adb65](https://github.com/material-components/material-web/blob/47adb655bd7a88c4d62e8faac2873084eed555dc/labs/segmentedbutton/internal/_shared.scss).
There is no dedicated segmented-button file in shadcn-m3e c37c0d2.

The current official site recommends connected button groups for new M3
Expressive designs. This follow-up corrects the existing classic 40px connected
segmented control. Its icon + 8px gap + label form **one centered group**;
the label by itself is not centered when an icon precedes it.

## Cause and fix

An empty `m3-segmented__state` span remained in normal flex flow even though
state rendering already used ::before. The parent gap inserted 8px before the
visible content, shifting it 4px right. Removed the redundant span and parent
flex gap. The inner icon/label gap stays 8px. Native 48px interaction targets,
40px visual geometry, equal segment widths and selected check replacement
remain intact. Divider placement now uses a logical start inset.

## Actual checks

| Check | Result |
| --- | --- |
| Before, with/without check/icon | Visible content center displaced +3.996..4px horizontally |
| After, every initial example | Absolute center error ≤0.00390625px horizontally; 0px vertically |
| Week ArrowRight | Month selects; content remains centered |
| Favourite Space | Check becomes favourite glyph; content remains centered |
| Details Space | Check appears; content remains centered |
| Restore selection | Week/Favourite/Add again selected, all centered |
| Disabled List/Grid | Centered; native disabled semantics retained |
| Target / visual geometry | 48px native height, 38px item inside 40px outlined set |
| Dark scheme | Same centering through the tested examples |

Raw data: [alignment samples](badge-segmented-alignment-samples.json).
Screenshot: [centered segmented buttons](segmented-button-alignment.png).
Dioxus reported a successful 9.44s live build; the preview was reloaded and
new markup measured. Locked offline Wasm check passed. Copy dependencies
remain in [copy-components.md](copy-components.md).

Android/touch and OS reduced-motion true remain untested. These measurements
establish settled content centering, not new motion certification or complete
keyboard/assistive-technology conformance. No motion timing changed in this fix.

## Reproduce

Run ./scripts/dev.sh, confirm successful build, reload. For each segment,
compare the bounding-box center of .m3-segmented__content with its parent item.
Toggle Favourite/Details with Space and Week/Month with arrows; repeat in dark
mode. Keep the icon and label together for the centering comparison.

## Historical reports before this correction

# Segmented Button — current fidelity update

Date: 2026-10-09. Desktop Codex in-app browser. Scope: component Rust/CSS
changes in this fix, not complete platform certification.

## Sources and implemented contract

- [Material overview](https://m3.material.io/components/segmented-buttons/overview)
- [Material specs](https://m3.material.io/components/segmented-buttons/specs)
- [Material guidelines](https://m3.material.io/components/segmented-buttons/guidelines)
- [Material accessibility](https://m3.material.io/components/segmented-buttons/accessibility)
- No dedicated segmented-button upstream file exists at c37c0d2; native input behavior is implemented against the official contract.
- [Pinned Material Web tokens](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass)

The four official sections were read in the preceding review on this date;
actual upstream files/styles were inspected for these fixes. Current values
and reference choices supersede historical measurements below.

R3/R5 plus ripple; input height 48px around 40px connected set, selected supplied icon becomes check, all-disabled outline reduced. Week ArrowRight selects/focuses Month; selected Month/Favourite/Add glyphs are check. Set spacing reserves target extension.

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

# Segmented button verification

Scope: `SegmentedButtonSet` and `SegmentedButton` in
[`src/components/segmented_button.rs`](../src/components/segmented_button.rs) and
[`assets/segmented-button.css`](../assets/segmented-button.css). Verified in the Dioxus
preview on 2026-10-09, with headless Chromium 1194 using real pointer and keyboard input,
at device scale 2. Desktop only. Android Chrome has not been tested.

## 1. Correction to the first version

The first version used separate pills with 2dp gaps, 8dp inner corners and a corner morph.
The user supplied the Material segmented button reference, which shows one connected pill.
That version did not match the reference, so it was replaced. The old measurements are gone
with it. This report covers only the current version.

## 2. References

- **Material 3 segmented button reference** (supplied by the user as a screenshot of the
  Material documentation): one pill with a 1dp outline, 1dp dividers between items and no
  gaps. Selected items are filled with a flat lavender segment and show a check. Icons and
  labels sit centred in each segment.
- Material Web labs outlined segmented button tokens, revision `47adb65`
  ([`_md-comp-outlined-segmented-button.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-outlined-segmented-button.scss)):
  40dp height, 1dp outline, label-large, 18dp icon, secondary-container when selected, state
  layers 0.08 hover and 0.10 focus/pressed, disabled 38% label and icon, 12% outline and
  selected container.
- **Not read:** the official `m3.material.io` segmented button page. The egress proxy blocks
  that host. The supplied reference and the labs tokens are the sources used.

## 3. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Set height | 40px, including the 1px outline | Tokens |
| Set outline | 1px outline, full pill (20px radius), clipped content | Reference |
| Dividers | 1px between items, drawn as an overlay so widths stay equal | Reference |
| Item widths | Equal: each item takes 1/n of the set, sized by the widest label (`flex: 1 1 0`) | Material guideline (equal-width segments) |
| Item corners | Square; the pill clip gives the rounded ends | Reference |
| Selected | secondary-container fill, on-secondary-container label, check replacing a supplied icon | Reference and tokens |
| Label | label-large, 14/20, weight 500 | Tokens |
| Icon | 18px | Tokens |
| State layer | hover 0.08; focus and pressed 0.10 | Tokens |
| Focus indicator | 3px secondary inset outline, since the set clips the edges | Kit focus style, adapted for the clip |
| Disabled | 38% label and icon; 12% divider to the item's left; 12% container when selected | Tokens |
| Single select | Native radios with one `name`; arrow keys move the selection | Native |
| Multi select | Native checkboxes; Space toggles | Native |

## 4. Input, state and platform matrix

Raw data: [`segmented-spec.json`](segmented-spec.json) (behaviour) and
[`segmented-widths.json`](segmented-widths.json) (widths). Screenshot:
[`segmented-single.png`](segmented-single.png).

| Check | Result |
| --- | --- |
| Set height and outline | 40px, 1px outline, 20px radius, overflow hidden |
| Item height | 38px inside the outline |
| Gap between items | 0px |
| Item widths, single and multi (504px set) | 167.33px each, 3 items; 0px difference |
| Item widths, disabled (504px set) | 251px each, 2 items |
| Labels clipped | None |
| Divider | 1px overlay at the left edge of items 2 and 3 |
| Item corner radius | 0px for every item |
| Hover state layer | 0.08 |
| Held press, unselected item, 200ms | State layer 0.10; corner 0px; not selected while held |
| Release on the item | Selected; fill secondary-container; check shown |
| Previously selected item after a new selection | Fill and check removed; label back to on-surface |
| Arrow Left / Right with focus on a radio | Selection moves to the previous / next item |
| Focus-visible | Inset 3px secondary outline |
| Multi select, click | Toggles the clicked item only |
| Multi select, Space on focused checkbox | Toggles that item |
| Selected item with an icon | Shows the icon instead of the check |
| Press, drag off the set, release | Selection unchanged |
| Disabled item, pointer click | Selection unchanged |
| Disabled label and container | 38% and 12% on-surface |
| Page errors | 0 |

Visual comparison: the screenshot has the same structure as the reference: one outlined pill,
flat 1dp dividers, and a lavender selected segment with a check. Segments are equal width, as
the guideline requires.

## 5. Diagnosed defects

- **Unequal widths.** The first fix sized items to content, so the segments differed in width.
  The guideline requires equal widths. Items now use `flex: 1 1 0`. A border divider had made
  items 2 and 3 1px wider, so dividers are an overlay instead. Measured widths are equal.

- **Wrong shape.** See section 1. The first version used gaps, pills per item and corner
  morphs. The current version follows the reference.
- **Radio change reported unchecked.** A radio's change event only fires for the newly checked
  radio, so reading `event.checked()` never moved the selection. The handler reports `true` for
  radios. Checkboxes still use `event.checked()`, which toggles correctly.
- **Stale preview build.** The dev server stopped rebuilding. Results from before the restart
  were discarded. The current results come from a rebuilt bundle.

## 6. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's single-select card, which compiles |
| Preview served from a rebuilt bundle | Passed after restart |

## 7. Unresolved differences and untested behaviour

- Touch input, Android Chrome and the on-screen keyboard are not measured.
- Reduced motion sets the 200ms colour transitions to 0.01ms. It is not measured.
- Screen-reader output for the group and each radio or checkbox is not measured.
- Only the standard 40dp height is provided.
- The official Material segmented button page was not read (blocked host). The supplied
  reference is a screenshot, so exact segment widths and text positions are not measured against it.
- Disabled outline is applied to the divider on each item's left only. The set's outer outline
  stays at full strength when every item is disabled. The reference does not show a disabled state.
- Equal widths follow the guideline. The longest label sets the width, so a set with long labels
  is wider than one with short labels. Text is not truncated or wrapped.
- Corner clipping relies on `overflow: hidden` on the set. The focus ring is inset for that reason.

## 8. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script with the Chromium at `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`
   against the preview. It sets the section's items, dispatches real mouse and keyboard
   actions, and reads computed styles. The script is kept in the scratchpad and is not committed.
3. Compare the output with [`segmented-spec.json`](segmented-spec.json) and
[`segmented-widths.json`](segmented-widths.json).
