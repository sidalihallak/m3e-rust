# Text field verification

Scope: `TextField` and `TextFieldVariant` in
[`src/components/text_field.rs`](../src/components/text_field.rs) and
[`assets/text-field.css`](../assets/text-field.css). Verified in the Dioxus preview on
2026-10-09 with headless Chromium 1194, using real pointer and keyboard input, at device scale 2.
Desktop only.

## 1. References

- Material Web filled text field tokens, revision `47adb65`
  ([`_md-comp-filled-text-field.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-filled-text-field.scss)):
  56dp container, surface-container-highest, 1dp on-surface-variant active indicator, 2dp primary
  focused indicator, error colours, disabled 38% text and 4% container.
- Material Web outlined text field tokens, revision `47adb65`
  ([`_md-comp-outlined-text-field.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-outlined-text-field.scss)):
  56dp container, 1dp outline, hover on-surface, focus primary, 4dp corner (corner-extra-small),
  body-large input and label text, body-small floated label, 12% disabled outline.
- **Note on focus width.** The outlined token file lists a 3dp focus outline. It is drawn here as a
  1dp outline plus a 2dp inset ring, which totals 3dp.
- **Not read:** the official `m3.material.io` text field page. The egress proxy blocks that host.
- shadcn-m3e `input.tsx` and `text-field.tsx` (revision `8f1b3fb`) were checked for structure only.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Container height | 56px | Tokens |
| Label at rest | body-large, 16/24, centred vertically, 16px inset | Tokens |
| Label floated | body-small, 12/16, on the outline (outlined) or 8px from the top (filled) | Tokens and Material layout |
| Float transition | 200ms standard easing | Choice; the token file gives no duration |
| Outlined outline | 1dp outline; hover on-surface; focus primary, 3dp total | Tokens |
| Filled container | surface-container-highest, 4dp top corners | Tokens |
| Filled indicator | 1dp on-surface-variant; hover on-surface; focus 2dp primary | Tokens |
| Error | error outline or indicator, error label and supporting text, `aria-invalid="true"` | Tokens |
| Disabled | 38% text and label; 12% outline; 4% filled container; not focusable | Tokens |
| Supporting text | body-small, 16px inset, `aria-describedby` on the input | Tokens and ARIA |
| Label association | `<label for>` linked to the input | ARIA |
| Keyboard | Native input: Tab moves focus; typing edits the value | Native |

Not implemented: leading and trailing icons, prefix and suffix text, character counters, and
multi-line text areas.

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
| Page errors | 0 |

## 4. Diagnosed defects

- **Focused outline never applied on outlined fields.** The outline element came before the input in
  the DOM, so `input:focus ~ outline` never matched. The outline now follows the input. Measured:
  the outline is primary on focus.
- **Floated label crossed the outline.** The label background was transparent, so the outline ran
  through the floated label. The label now takes the page surface as its background, which masks
  the outline behind it.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Preview served from a fresh build | Passed |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's cards, which compile |

## 6. Unresolved differences and untested behaviour

- Leading and trailing icons, prefix and suffix text, counters and multi-line fields are not implemented.
- The label float duration (200ms) is a choice. The Material spec was not read (blocked host).
- The label notch on the outline is an approximation: the label takes the surface colour rather than
  being cut from the outline path, so it looks right only on the surface colour it is placed on.
- Touch input, Android, the on-screen keyboard, reduced motion and screen-reader announcements are not measured.
- Autofill and validation timing (when the error appears) are left to the consumer.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script against the text field section with the Chromium at
   `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`. It clicks into and types in the outlined
   field, tabs out, clears it, checks the error field, and clicks the disabled field.
3. Compare the output with [`field-samples.json`](field-samples.json).
