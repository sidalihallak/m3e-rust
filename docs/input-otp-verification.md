# OTP input — verification, 2026-10-12

## Scope and references

`src/components/input_otp.rs`, its `input_otp.js` runtime and
`assets/input-extensions.css` provide controlled InputOTP, ASCII digits or
alphanumeric patterns, configurable length1–12, optional separator, named form
input, supporting/error text, disabled/read-only/required and completion callback.
This is an upstream extension; Material has no dedicated OTP component page.

Read the rendered text-field [overview](https://m3.material.io/components/text-fields/overview),
[specs](https://m3.material.io/components/text-fields/specs),
[guidelines](https://m3.material.io/components/text-fields/guidelines) and
[accessibility](https://m3.material.io/components/text-fields/accessibility) on
2026-10-12. Applied adjacent labels, visible focus/error, semantic color roles,
linked supporting text and56px field height.

Inspected shadcn-m3e
[input-otp.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/input-otp.tsx)
at revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972` and the actual
[input-otp engine](https://github.com/guilhermerodz/input-otp/blob/master/packages/input-otp/src/input.tsx)
(master as retrieved2026-10-12; not a claim of a dependency-version-pinned port).
Its [maintainer documentation](https://github.com/guilhermerodz/input-otp)
explains the single-input design. The source selection algorithm, focus and
completion rules were read; this port uses one native text input and decorative
slots instead of six input elements.

## Reference contract

- Desktop slots48×56px, rounded-md12px,8px gaps, separator16px, headline-small24/32.
- Transparent container, Outline borders, Primary active border+1px inset shadow,
  Error border/shadow and supporting alert, disabled38% opacity.
- One labelled native input, `autocomplete=one-time-code`, numeric or text
  inputmode, native maxlength/name/readonly/disabled/required. Slots are aria-hidden.
- Click a populated slot selects its character for replacement. Selectionchange
  normalizes a collapsed caret to a character range except append position;
  directional history follows the inspected source algorithm. Native Shift
  selection remains available.
- Completion fires when a user edit changes an incomplete value to full length;
  no automatic submission or verification side effect.
- Native exact-length pattern makes partial input invalid and complete input valid.
- Numeric paste removes formatting; alphanumeric pattern preserves ASCII case.
  Programmatic value rendering is normalized; consumers should store onchange
  output. Controlled props must be updated in oninput.
- Fake caret24×2px,1s blink. Reduced-motion media rule makes it steady. Focus border
  is immediate and changes no geometry.

The single input fills the entire field target. At compact widths, decorative
slots shrink while field height remains56px. Exposing separately focusable Slot/
Group primitives would duplicate input state, so the Dioxus API configures slots
through the parent instead of React context children.

## Actual runtime checks

Live Dioxus Wasm, Codex in-app browser, desktop keyboard/pointer:

| Check | Observed |
| --- | --- |
| Sequential123456 | All six characters visible, completion status ready, final range5–6 |
| Click slot2, type9 | Slots show129456; final demo status remains ready |
| ArrowLeft | Range2–3 after replacement, no extra focus stop |
| Backspace | Selected digit removed, remaining characters shift |
| Select-all + paste12-34 56 | Slots show123456, completion ready |
| Alphanumeric A!b2Z | Slots showAb2Z; punctuation filtered |
| Tab from verification | Focus Invitation code |
| Tab from invitation | Focus Expired code |
| Tab from expired | Skips Disabled code, focuses Saved code |
| Read-only key | Saved654321 remains unchanged |
| Empty focused caret |12 samples over1.26s include opacity0 and1; see raw frames |
| Desktop geometry | Field352×56px, slot48×56px, radius12px, gap8px |
|390px layout | After correction field327×56px at x24..351; slots43.84px; client/body375px |
| Native validity | Three digits patternMismatch=true; six digits valid=true |
| Dark mode | Outline148,143,153; text230,224,233; label202,196,207; light restored |
| Code copy | Both controls exact,529 and401 characters |

The first compact check found fixed slots overflowing: body376 vs client375px.
Grid tracks now use minmax(0,1fr) and the field allows shrinking; second measurement
shows no page overflow. A stale older preview tab timed out; checks used a fresh
normal localhost tab and one mounted gallery.

[Raw measurements](input-extensions-samples.json), screenshots:
[evidence/input-extensions/otp-desktop.png](evidence/input-extensions/otp-desktop.png),
[evidence/input-extensions/otp-390.png](evidence/input-extensions/otp-390.png).
Only fictional demo codes were entered.

## Build and gaps

Final Dioxus build completed24.70s after the complete-value demo-status correction. Compact
CSS hot-reload reported success. The refreshed independent library and all86
examples pass offline Wasm check1.12s after the final snippet refresh (existing unused ChipSize warning only).

Android remains paused. Native SMS autofill, password-manager badge displacement,
iOS long-press/paste, actual reduced motion, audible screen readers, RTL, zoom,
forced colors, IME composition, undo after transformed paste, dynamic lengths and
interruptions are unverified. System Settings process was unavailable; no OS
preference changed. The CSS reduced-motion rule alone is not runtime evidence.
Password-manager detection, no-script fallback, arbitrary regex/transform callbacks,
React render/context composition and complete browser workaround parity remain
upstream extensions. Controlled full-value replacement does not trigger completion;
the consumer owns programmatic verification.

## Reproduce

Run `./scripts/dev.sh`, confirm successful build and reload. Open Selection → OTP
input. Type six demo digits, click a slot and replace it, use arrows/Backspace,
select-all and paste a formatted demo code. Traverse each field with Tab; verify
disabled/read-only and error states. Sample active slot/caret while focused,
resize to390px and inspect overflow. Copy both examples, retaining the JavaScript
runtime next to the Rust source and required CSS/theme roles.
