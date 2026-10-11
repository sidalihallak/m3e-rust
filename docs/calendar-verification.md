# Calendar verification — 2026-10-11

## Scope and references

`src/components/calendar.rs`, `calendar.js`, `date.rs` and
`assets/date-picker.css`. Official date-picker overview, specs, guidelines
and accessibility and actual upstream calendar source were read; full links,
revision, measured popup motion and build results are in
[the date-picker report](date-picker-verification.md).

## Contract and observed values

Controlled `month/onmonthchange` and `value/onselect`; single or range
selection, configurable locale/week start, min/max and disabled dates/weekdays.
Native buttons and grid/columnheader/selected-cell semantics; full date names,
today distinct from selection. Width 360px, grid 336px, seven 48px columns,
40px selected circles with a 4px inset. Year visuals 72×36 within 72×48 targets,
three columns, 288px scroll area. Official year diagram and upstream use 72×36;
official accessibility requires at least 48px targets.

Dates use one roving Tab stop. Logical horizontal arrows support RTL,
vertical arrows move seven days; PageUp/Down move months and Shift adds years.
Home/End use first/last month date (the official accessibility page repeats
“first” for End; implementation uses conventional last-date behavior).
Shift+M/Y opens month/year lists; arrows and native Enter navigate choices.
Calendar does not commit application state itself.

## Actual verification

- Pointer selects a modal draft without immediately committing.
- ArrowRight October 11 → October 12; PageDown November 12.
- End October 31 → PageDown November 30 (day clamped); Home November 1.
- Shift+Y, ArrowDown 2026 → 2029 and Enter return to date view.
- RTL ArrowLeft October 11 → October 12.
- Disabled-weekend example: ArrowRight Friday October 9 skips Saturday/Sunday
  and focuses Monday October 12; disabled native Saturday confirmed.
- Range October 11–15 marks five contiguous selected cells and displays a band.
- Quick pointer click samples demonstrate real ripple growth and fade; shared
  ripple timing is preserved. Reduced-motion modal container sampled separately.
- Calendar copy control returned its source. Exported API/usage compiles.
- Wasm check, Dioxus serve and three focused date-model tests pass.

[Raw data](date-picker-samples.json) and
[inline LTR/RTL screenshot](evidence/date-picker/calendars.png).

## Differences and gaps

Blank adjacent-month cells follow the official modal illustration; upstream
Calendar defaults to showing outside days. Modal uses separate month/year
selectors (docked contract) rather than a single combined caption. Month heights
are dynamic; all six/four/five-row and date-constraint permutations are not
exhaustively measured. Today is resolved from local browser date, or a supplied
date override. No non-Gregorian calendars, week numbers or multi-month desktop
layout. Touch swipes and Android remain unverified/paused. Audible screen-reader
announcements, forced colors and 200% text enlargement remain open.

Run the preview, reload `/#calendars`, use each calendar's named buttons and
arrow keys, then `/#date-pickers` for modal/range integration.
