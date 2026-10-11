# Date picker verification — 2026-10-11

## Scope and references

Local web implementation: `src/components/date_picker.rs`, `date_input.js`,
`date.rs`, `assets/date-picker.css`, and `src/scheduling_gallery.rs`.
Calendar is shared; see [its report](calendar-verification.md).

Read the current official [overview](https://m3.material.io/components/date-pickers/overview),
[specs](https://m3.material.io/components/date-pickers/specs),
[guidelines](https://m3.material.io/components/date-pickers/guidelines), and
[accessibility](https://m3.material.io/components/date-pickers/accessibility)
in the browser on October 11. Inspected the day/year measurement illustrations.
Read actual upstream [date-picker.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/date-picker.tsx)
and [calendar.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/calendar.tsx).

## Contract and implementation

- Docked outlined field, modal single/range selection and modal text input.
  Adaptive pickers become full-screen below 600px, with Close and Save.
- Calendar action is a separate native 48px button; typing does not open it.
  An actual pointer check found the input above this sibling action. A scoped
  z-index fixes the hit target; both mouse and keyboard activation were retested.
- Strict Gregorian date-only values avoid timezone conversions. Text supports
  slash, dash, dot, spaces and eight compact digits; format is configurable.
  No mask runs while typing. Enter/blur normalizes valid input. Empty docked
  input clears the controlled value; invalid text retains the committed value.
- Modal draft commits only with OK/Save. Cancel and Escape discard it.
  Invalid/incomplete/reversed ranges cannot confirm. Constraints cover bounds,
  years, dates and weekdays; optional range-interior exclusion is explicit.
- Surface container high, on-surface text, primary/on-primary selection,
  secondary-container/on-secondary-container range. Modal width 360px,
  corners 28px, header 120px single / 128px range; docked corners 16px.
  Single headline 32/40; range 22/28; label 14/20.
- Official overview specifies no shadow; this overrides upstream elevation 3.
  Date targets are 48px around 40px circles. Footer actions use the existing
  small Button with a scoped 48px height token, preserving finite half-height
  corners. This is a larger visual than the illustration's 40px action.
- Reuses owned native Dialog/anchored popup runtime. Dialog entry uses existing
  DefaultEffects 240ms opacity and DefaultSpatial 440ms transform; popup entry
  uses DefaultEffects 240ms / FastSpatial 360ms. Exit uses the existing 150ms
  easing. These runtime/reference choices are not universal date-picker timing
  requirements. Shared Button/Ripple implementations were not changed.

## Actual input/state checks

| Scope | Observed result |
| --- | --- |
| Desktop pointer | Modal date changes draft; Cancel restores October 11 on reopening; calendar icon click opens docked popup after stacking fix |
| Keyboard | ArrowRight October 11 → 12; PageDown November 12; End October 31 then PageDown clamps November 30; Home November 1 |
| Year selector | Shift+Y opens; ArrowDown 2026 → 2029; Enter returns to calendar |
| Text | February 30 invalid, OK disabled; 02292024 remains raw before commit, then 02/29/2024; OK confirms leap day |
| Docked text/action | 10.14.2026 remains raw while editing and does not open popup; blur formats it; Tab reaches calendar action |
| Docked selection | Space selects October 15, closes popup and returns focus to named opener; Escape also closes |
| Range | One endpoint disables OK; October 11–15 paints five contiguous selected cells; reversed text range shows error; corrected October 15–19 confirms |
| OS Reduce Motion | Temporarily enabled with prior user permission; browser media query true; sampled modal entry transform remained none; preference restored off |
| 390×844 desktop viewport | Full-screen surface, zero corners, Close/Save, 48px date targets; Save confirms and closes |
| Copy/export | All three date-picker copy controls exactly matched their snippets; calendar copy returned its source; independent library plus all 57 gallery examples compile |

## Build and evidence

Wasm cargo check passes. Dioxus serve initial build succeeded in 22.05s;
subsequent successful builds include 12.42s, 3.98s, 10.17s and final 14.72s.
Final reloaded geometry: 360×524px modal, 48px footer actions and 24px corners on those actions. Reloaded before
measurement; one `.app-shell` confirmed. JavaScript syntax checks pass.
Three focused Rust date tests pass: boundaries/leap rules, input parsing, and
range exclusion/restart. Independent exported library and 57 examples pass
Wasm check (one existing unused ChipSize import warning).

[Raw motion and interaction samples](date-picker-samples.json).
[Desktop](evidence/date-picker/desktop.png),
[compact desktop viewport](evidence/date-picker/compact.png).
Samples show real entry interpolation from scale ~0.80 to 1, opacity growth,
and exit interpolation toward 0.90 before native closure. Day ripple grew and
faded after a quick click. Read sample timestamps as tool-observed timing,
not a precise input-event timestamp.

## Gaps

Android remains paused. Compact desktop evidence does not establish touch,
swipe, scrolling cancellation or on-screen keyboard behavior. Horizontal month
swipes and an indefinitely scrolling range calendar are not implemented; compact
range currently presents two navigable stacked months. Full locale sentence
ordering, non-Gregorian calendars, forced colors, text enlargement and audible
screen-reader output are not verified. Outside dismissal inherits Dialog but
was not independently exercised here. Held/repeated day presses remain unsampled.
This is core implementation evidence, not a claim of complete pixel parity.

## Reproduce

Run `./scripts/dev.sh`, wait for successful build, reload `/#date-pickers`.
Open the modal, change a date, Cancel, reopen and inspect the original selection.
Use arrows, PageUp/Down, Shift+Y and Enter. Switch to text input, enter an invalid
then leap date; confirm. Test a reversed range. At 390px width inspect Close/Save.
Use `/#calendars` for disabled weekends and RTL. Copy code using the gallery
controls; export with `scripts/export-kit.py`.

## Follow-up: date field background — 2026-10-11

Re-read the current official four date-picker pages and compared actual upstream
date-picker.tsx and text-field.tsx at the pinned revision above. Date entry is
**outlined**, with a transparent field fill; it is not a filled TextField.
The official modal-input color diagram maps the surrounding container to
surface-container-high. Upstream outlined variant adds no fill and its input
uses bg-transparent; only its filled variant uses surface-container-highest.

Measured actual docked and modal text inputs after a successful build/reload.
The input and field container are transparent in both themes. The docked field
shows the app's surface (#fdf7ff for the default light scheme). Modal entry shows
the native dialog's surface-container-high: light #ece6ee, dark #2b292f, exactly
matching the inherited semantic token. No additional white/filled background was
added. Restored light mode after checking dark.

[Raw rendered colors](date-field-background-samples.json).
Upstream [text-field.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/text-field.tsx).
