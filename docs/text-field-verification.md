# Text field verification — current and historical evidence

## Current dialog field verification — 2026-10-10

Browser access is restored; the blocked width/notch notes below are historical.
Source: `text_field.rs`/`text-field.css`, reused in `dialog.rs`/`dialog.css`.
Pinned upstream and the previously rendered four official field sections remain
linked below. Width follows upstream `w-full`, not a universal fixed M3 field width.

Opening Create project was sampled in **32 actual frames**. Field layout width
stays 400px while its viewport width grows from 321.06px to 400px. The hidden
label has layout width 79px rounded (observer fractional value **79.375px**) while
its early viewport width is 63.71px. The notch stores **79.375px**, proving the
new layout measurement ignores the ancestor's .8→1 transform. The former
`getBoundingClientRect()` value would have stored the scaled width without a new
ResizeObserver event on transform completion.

Settled: field/body content 400px; label font 12px; mask **87.375px×3px** focused,
**87.375px×1px** after typing `Fidelity check` and Tab. Label padding adds 4px on
each side. The visible label fits the opening in the screenshot. The full-screen
field is 1046px at viewport 1094px, matching the 24px body side insets. The dialog
paint gutter protects a first-child floating label and neighboring focus rings
without changing visible field width; see [dialog verification](dialog-verification.md).

[Full raw trace](resumed-browser-verification.json),
[settled field screenshot](dialog-field-verified-preview.png).
Successful final build, source-copy compilation and copy/paste are recorded in
[the composite index](composite-verification.md).
This targeted run verifies normal outlined Project name under the actual dialog
entry transform. It does not re-certify every disabled/filled/multiline/autofill
state. Android stays paused; compact override failed in this run, so final phone
width is not claimed. Audible assistive technology and live font/prop permutations
remain open.

Reproduce: reload, open Create project while sampling field layout/viewport bounds,
notch custom property and mask; type, Tab and compare settled focused/blurred depths.
Confirm focus returns on dismissal, then inspect the desktop FullScreen field.

## Historical width and notch follow-ups before restored browser access

## Parent-width follow-up — 2026-10-10

The dialog field stopped at 280px because `.m3-field` had a fixed `width: 280px`.
Its native input already filled the field row; the outer component was the
constraint. The reusable field now uses `width: 100%`, `min-width: 0` and its
existing `max-width: 100%`/border-box sizing. Layout containers determine the
width, including nested form rows and compact dialogs. To request a narrow field,
constrain its parent. The Rust API and copyable usage snippets are unchanged.

Re-read the actual pinned upstream
[text-field.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/text-field.tsx)
and [dialog.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/dialog.tsx):
the field wrapper and surface use `w-full`. This width behavior is attributed to
upstream, rather than a fixed universal Material field width. The official four
sections previously read in this conversation remain linked below. A fresh
request for the official pages returned JavaScript-only shells; it is not a new
rendered guideline reading.

`cargo check --locked --offline --target wasm32-unknown-unknown` passed in 0.38s.
The live server reported `text-field.css` hot reload, then a fresh preview build
succeeded in **9.94s**. Automatic approval review rejected browser
inspection because of the existing localhost URL security block. No substitute
browser access was used. Final rendered width, typing/notch regression checks and
responsive measurements are therefore pending; source arithmetic is not runtime
evidence. Android remains paused.

The user manually refreshed Create project and confirmed that the field now fills
the content width. They then reported overflowing Project name notch text. Source
inspection identified a transform-sensitive measurement: `getBoundingClientRect()`
includes the dialog's .8→1 entry scale, whereas the notch needs the label's layout
width. ResizeObserver does not issue another size change for an ancestor transform,
so a scaled-down width could remain after entry. This is the likely cause, pending
the requested screenshot/rendered confirmation.

The notch now reads the observer's fractional `borderBoxSize.inlineSize`, falling
back to `offsetWidth` for initial/font-ready measurements. Both ignore ancestor
transforms. This preserves the existing label-plus-8px notch, focus-depth mask and
200ms label/notch motion. The fallback rounds to whole CSS pixels; the observer
supplies fractional layout width when available. Fresh Dioxus build passed in
**5.36s**, Wasm check in **1.94s**, and the generated notch JavaScript passes
`node --check`. Post-fix overflow, transformed-ancestor geometry and input checks
still require rendered verification; no browser/device result is claimed.
The independently exported TextField usage example also compiles with the updated
source and stylesheet. The source-copy dependencies and API remain unchanged.

Reproduce: reload, open Create project, compare field and dialog-body bounds, type
a name, Tab out and verify the label/notch; repeat the Adaptive editor at a compact
desktop viewport. Check normal, disabled, filled and multiline gallery fields
against their own containers. Save settled measurements before marking verified.

## Accessibility follow-up — 2026-10-10

See [the accessibility audit](accessibility-verification.md) and
[raw samples](accessibility-samples.json) for tested behavior and remaining gaps.


R9: the native label and 56px surface focus the input; disabled surface does
not forward focus. Added native read_only and ResizeObserver notch tracking.
Email label click focuses Email. Readonly REF-123 remains unchanged after x;
changing Reference to Longer account reference increases measured floated text
width 61.492 → 156.711px. Previous full-depth notch and hidden number steppers
remain in place. Current build/check pass and copy dependencies are in the
[fix report](component-fidelity-fixes.md), with
[raw evidence](component-fidelity-fixes-samples.json).

Official four-section source readings and pinned upstream text-field.tsx are
listed below. Desktop checks establish the named behavior; Android, autofill,
screen readers and OS reduced-motion true remain untested.

## Earlier notch/spinner evidence

# Text field verification

Scope: `TextField` and `TextFieldVariant` in
[`src/components/text_field.rs`](../src/components/text_field.rs) and
[`assets/text-field.css`](../assets/text-field.css). Verified in the Dioxus preview on
2026-10-09 with headless Chromium 1194, using real pointer and keyboard input, at device scale 2.
Desktop only. Those measurements are historical. The most recent targeted follow-up below was
performed in the desktop Codex in-app browser on 2026-10-09; it supersedes the old notch rendering
and blocked-source notes. It does not re-certify all of the historical matrix.

## Latest follow-up: full-depth notch and numeric appearance

The user identified a line crossing the outlined label and native stepper arrows on Price.
The focused outline combined a 1px border with a 2px inset shadow, but its mask removed only a
2px band. The cut did not remove the full visible edge. `assets/text-field.css` now draws the
focused outline as a single 3px border and derives the cut's depth from that border width.
The cut is 1px deep at rest and 3px deep on focus; no surface-colored patch is painted behind
the label. Container dimensions and the existing 200ms float timing are preserved.

Numeric inputs keep `type="number"`, native numeric editing/validation and accessible spinbutton
semantics. The component stylesheet uses `appearance: textfield` and suppresses WebKit spin-button
appearance to remove browser chrome from this Material field. This is a kit presentation choice
requested by the user; the current upstream TextField does not explicitly suppress native steppers.
See [MDN appearance](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/appearance).

| Current desktop check | Result |
| --- | --- |
| Focused Price | Border 3px, no inset shadow, notch `40.875px 3px`, container 56px |
| Populated Price after blur | Border 1px, no inset shadow, notch `40.875px 1px`; value retained |
| Price typing | Typed 125; value 125 |
| Price ArrowUp / ArrowDown | 125 → 126 → 125 with the visible steppers suppressed |
| Numeric appearance | Computed `appearance: textfield`; screenshot showed no arrows |
| Focused Name | Border 3px; settled notch width 46.2031px; label 12px |
| Name filled then blurred | Ada retained; label stays floated; outline returns to 1px |
| Name cleared then blurred | Screenshot confirmed closed notch and resting label |
| Disabled Account number | Pointer click did not focus the disabled input |
| Light and dark surfaces | Visually inspected the focused Price cut and absence of arrows in both |
| Motion while Name focused | Samples from 0–530ms; first focused sample 124ms, font/notch settled by 327ms; border/cut depth 3px throughout focus |
| Reduced motion | Rule now shortens both label and notch transitions; OS preference not exercised |
| Rust/Wasm check | `cargo check --locked --offline --target wasm32-unknown-unknown` passed |
| Dioxus web build | `dx build --platform web --locked --offline` passed; live server reported CSS hot reload, then browser reloaded |

Current screenshot: [text-field-price-fixed.jpg](text-field-price-fixed.jpg), showing the focused
Price field without browser arrows and with a full-depth label cut on the light surface.

Motion timestamps are relative to polling start, including the pointer action's setup; they are
not exact event timestamps. Raw samples: [text-field-notch-samples.json](text-field-notch-samples.json).
No Android, Firefox, screen-reader or numeric-autofill result is claimed. The native number step
remains the browser default; decimal currency policy is a separate consumer/API decision.

Copy dependencies: `src/components/text_field.rs`, `assets/text-field.css`, the `next_id` helper
from `src/components/motion.rs`, and `Icon`/`IconData` plus `assets/icon.css` when using icons.
Consumers also need the existing semantic color variables. The Rust API and gallery snippet
were not changed in this follow-up.

## 1. References

- Material Web filled text field tokens, revision `47adb65`
  ([`_md-comp-filled-text-field.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-filled-text-field.scss)):
  56dp container, surface-container-highest, 1dp on-surface-variant active indicator, 2dp primary
  focused indicator, error colours, disabled 38% text and 4% container.
- Material Web outlined text field tokens, revision `47adb65`
  ([`_md-comp-outlined-text-field.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-outlined-text-field.scss)):
  56dp container, 1dp outline, hover on-surface, focus primary, 4dp corner (corner-extra-small),
  body-large input and label text, body-small floated label, 12% disabled outline.
- **Note on focus width.** The outlined token file lists a 3dp focus outline. It is now drawn as
  one 3px border, with the notch cut through the full border thickness. Current upstream uses 2px
  on focus; this kit retains its existing official Material Web 3px token contract.
- Official text field [overview](https://m3.material.io/components/text-fields/overview),
  [specs](https://m3.material.io/components/text-fields/specs),
  [guidelines](https://m3.material.io/components/text-fields/guidelines) and
  [accessibility](https://m3.material.io/components/text-fields/accessibility) were read in the
  browser on 2026-10-09. The specs describe 56dp containers/targets, 4dp padding beside the populated
  outlined label, and floating label/input/support roles. The former blocked-host note is superseded.
- Actual [shadcn-m3e text-field.tsx at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/text-field.tsx)
  was inspected. It uses three outline pieces with no top edge over the floated label. The kit uses
  a transparent CSS mask to produce the same cut, including on non-page-colored surfaces.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Container height | 56px | Tokens |
| Label at rest | body-large, 16/24, centred vertically, 16px inset | Tokens |
| Label floated | body-small, 12/16, on the outline (outlined) or 8px from the top (filled) | Tokens and Material layout |
| Float transition | 200ms standard easing | Choice; the token file gives no duration |
| Outlined outline | 1dp outline; hover on-surface; focus primary, one 3dp border | Tokens |
| Outlined notch | A cut through the full top border, the floated label's width plus 4dp either side; no painted colour | Material layout; the cut is drawn with a mask |
| Filled container | surface-container-highest, 4dp top corners | Tokens |
| Filled indicator | 1dp on-surface-variant; hover on-surface; focus 2dp primary | Tokens |
| Error | error outline or indicator, error label and supporting text, `aria-invalid="true"` | Tokens |
| Disabled | 38% text and label; 12% outline; 4% filled container; not focusable | Tokens |
| Supporting text | body-small, 16px inset, `aria-describedby` on the input | Tokens and ARIA |
| Label association | `<label for>` linked to the input | ARIA |
| Keyboard | Native input: Tab moves focus; typing edits the value | Native |
| Leading icon | 24dp icon 12dp from the start edge; text starts 16dp after the icon (52dp) | Tokens (24dp icon) and Material layout |
| Trailing icon | 24dp icon 12dp from the end edge | Tokens and Material layout |
| Prefix and suffix | Body-large, on-surface-variant; shown only while the label is floated, as in Material | Material layout |
| Counter | `count/max` on the trailing edge of the supporting row; over the limit, the field shows the error state and `aria-invalid="true"` | Material layout |
| Multi-line | Three lines at rest, growing with the text (`field-sizing: content`); label beside the first line at rest, floated to the top edge | Material layout; growth is a choice |

## 3. Input, state and platform matrix

Raw data: [`field-samples.json`](field-samples.json). Screenshot: [`field-section.png`](field-section.png).

| Check | Result |
| --- | --- |
| Container height, all fields | 56px |
| Outlined label at rest | 16px, top 16px inside the container |
| Click into the outlined field | Label 12px, primary colour, floated to the outline; outline primary |
| Typing "Ada" | Value "Ada"; label stays floated |
| Tab out with a value | Label stays floated; focus moves to the next field |
| Clear and blur | Label returns to 16px at rest |
| Filled error field | Label and supporting text error colour; `aria-invalid="true"`; 2px error indicator when focused |
| Error supporting text | Linked with `aria-describedby` |
| Label association | `for` matches the input id on every field |
| Disabled field, click | Not focused; text and label 38% |
| Leading icon, input and label positions | Icon at 12px; input and floated label at 52px (measured) |
| Prefix and suffix | Hidden at rest; shown on focus or with a value (measured) |
| Counter | `0/60` at start; `10/60` after 10 characters; `70/60` with the error colour and `aria-invalid="true"` over 60 |
| Multi-line | 104px container with three lines; 176px with six lines |
| Page errors | 0 |

## 4. Diagnosed defects

- **Floated labels stopped floating (regression).** Moving the input into a row element broke the
  `input:focus ~ label` sibling selectors. The outline, label and indicator now sit inside the row,
  after the input. Measured again: the outlined label floats to 12px on focus and when filled,
  and the multi-line note floats too.
- **Filled floated label overlapped the text.** The filled row's top padding was 16px, which is
  not enough for a floated label. It is now 24px at the top and 8px at the bottom. Measured: the
  label and the input text no longer overlap.
- **Outlined notch was a painted background.** The label's background was the page surface. It
  showed as a box on any other surface. The notch is now a real cut in the outline: a CSS mask removes
  a band from the top edge, the width of the floated label plus 4dp either side. No colour is
  painted, so the notch works on any surface. Measured: the notch is 42.9px for the 34.9px "Name"
  label, and the outline is cut on the light card and the dark card alike (`notch-light.png`,
  `notch-dark.png`).

- **Focused outline never applied on outlined fields.** The outline element came before the input in
  the DOM, so `input:focus ~ outline` never matched. The outline now follows the input. Measured:
  the outline is primary on focus.
- **Focused notch left a residual stroke.** The old 2px mask did not remove the complete 3px
  focused edge. The current border and mask use the same thickness, with no inset shadow.
- **Native numeric steppers appeared in Price.** The component CSS now suppresses their visual
  appearance while retaining number semantics and the tested ArrowUp/ArrowDown behavior.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Preview served from a fresh build | Passed |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's cards, which compile |

## 6. Unresolved differences and untested behaviour

- Multi-line growth needs `field-sizing: content`. Chromium 1194 supports it; browsers without it keep
  the three-line height and scroll.
- The counter does not stop input at the limit: it counts and shows the error state, as Material does.
- The label float duration (200ms) is a kit choice, not a universal Material timing requirement.
- The notch width is measured once, when the field mounts, from a hidden copy of the label. If the
  label text changes later, the notch keeps the old width. Browsers without CSS mask composition
  (`mask-composite`) show the full outline.
- Touch input, Android, the on-screen keyboard, reduced motion and screen-reader announcements are not measured.
- Autofill and validation timing (when the error appears) are left to the consumer.
- Surface/label click forwarding (the 56px surface versus the 24px native input) remains the
  separate R9 finding in [the fidelity review](component-fidelity-review.md); it was not changed here.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Focus Name and Price. Inspect the full notch on focus, then type and Tab out. Verify that the
   populated label stays floated and the outline becomes 1px.
3. In Price type 125, press ArrowUp and ArrowDown, and verify 126 then 125 without visible steppers.
4. Clear Name and blur it; verify the notch closes. Check disabled Account number does not focus.
5. Repeat focused Price in dark mode. Compare current motion with
   [text-field-notch-samples.json](text-field-notch-samples.json). Older
   [field-samples.json](field-samples.json) and screenshots are historical evidence.
