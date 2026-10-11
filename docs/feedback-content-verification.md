# Feedback and content — 2026-10-11

## Scope and references

Core local ports: Snackbar/Toast, Alert banner, SearchBar/SearchView,
List/ListItem, Avatar/AvatarGroup, Skeleton/LoadingPlaceholder and EmptyState.
Seven new catalogue routes bring the showcase to 38 pages; eight new usage examples
bring the independent example set to 53. Official rendered overview, specs,
guidelines and accessibility pages were read for Snackbar, Search and Lists.
Actual upstream sources were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

| Component report | Scope |
| --- | --- |
| [Snackbar](snackbar-verification.md) / [Toast](toast-verification.md) | Persistent action, optional 4–10s timeout, one host, hover/focus pause, fast scale/fade |
| [Alert](alert-verification.md) | Inline semantic banner and independent actions |
| [Search](search-verification.md) | Contained docked/full-screen search, query/suggestion keyboard behavior |
| [List](list-verification.md) | Standard/segmented content, actions, single/multiple selection |
| [Avatar](avatar-verification.md) | Sizes, image/error fallback, status and group |
| [Skeleton](skeleton-verification.md) | Decorative pulse and loading container |
| [EmptyState](empty-state-verification.md) | Semantic message and native action |

Alert, Avatar, Skeleton and EmptyState are upstream helpers/extensions; there are
no standalone official M3 component pages for them. Their reports separate
upstream appearance from applicable Material guidance. Latest official contained
search and primary-container list selection take precedence over older upstream
baseline styles. Material-oriented `sonner.tsx` supplies snackbar motion; the
stacked Base UI toast is a separate upstream implementation.

## Input/state/platform matrix

| Check | Actual result |
| --- | --- |
| Desktop pointer | Snackbar, alert dismissal, selection, secondary list action, avatar URL toggle and loading toggle worked |
| Snackbar keyboard | Alt+G reached Undo; Escape restored Save with undo; Enter invoked Undo |
| Timer focus pause | 6000ms message stayed open through 6500ms with focus, then closed after focus left |
| Timer hover pause | Pointer remained inside and focus outside; message stayed open through 6500ms, then closed after pointer left |
| Action persistence | Action message remained open after 7000ms; no auto timer |
| Search keyboard | Query filtered, disabled result skipped, input kept focus during preview, Enter committed, reopening reset selection |
| Submitted search | Native input released focus to results; ArrowDown/Enter selected a result; actual opener regained focus |
| Search Tab | Empty-query input Tab wrapped to Back; Shift+Tab returned to input |
| List keyboard | Arrow skip/wrap, Space exclusive selection, Enter multiple selection; no nested buttons |
| Avatar | 32/40/56px; loaded local image→failed URL initials→restored local image |
| Loading | Native toggle removed skeletons and set aria-busy=false |
| Empty state | Enter on Create project displayed Project created |
| Clipboard | All eight new copy controls exactly matched Rust snippets |
| Responsive desktop 390px | All seven final routes had no horizontal overflow; search used 390×844px full-screen and 48px back control |
| Actual OS reduced motion | Search/snackbar entry completed immediately; skeleton animation none with opacity 1; macOS preference restored off |
| Android/native Android | Paused at user request; no new claim |
| Audible screen reader | Not established by DOM/AX names; remains open |

## Motion and geometry

[Raw evidence](feedback-content-samples.json) contains checks and concurrently
sampled entry/exit, selection, pulse and timer states. Timestamps are external
wall-clock offsets, not exact DOM event timestamps. Generic `open` fields on
surface/div samples are not dialog visibility assertions. Use actual dialog checks,
phase, hidden state and bounding rectangles as applicable.

- Snackbar: 48px resting height, 4px corners, fast spatial 360ms and fast effects 150ms.
  Samples show 0.8 scale, intermediate growth and a small reference-token overshoot,
  followed by rest; exit ends hidden.
- Search: closed pill 592×56px and focused pill 616×56px in a 640px container.
  Margins 24→12px; docked width grows 616→640 with DefaultSpatial 440ms and opacity
  with DefaultEffects 240ms. Final two-line rows measured 72px. Empty/short results
  retain the 240px minimum; four results measured 420px container height.
- List: standard 56/72/88px row geometry; segmented selected corners 16px with
  DefaultEffects 240ms; the accepted shared ripple is unchanged.
- Skeleton: upstream 2s opacity pulse; 2100ms sample records the cycle. Actual
  reduced-motion 600ms sample stays opaque with no animation.

Screenshots are resting/layout evidence, not claimed mid-animation frames:
[desktop search](feedback-search-desktop.png),
[compact search](feedback-search-compact.png),
[compact lists](feedback-lists-compact.png).

## Builds and copy workflow

Live `./scripts/dev.sh` completed the final Rust build successfully (6.16s after
the last gallery change; earlier core builds 6.79/8.72/10.08s). Reload confirmed
one `.app-shell`, final gallery spacing 24px and current runtime behavior.
The preview remains running on localhost:8080.

The final independent source export, library and all 53 usage examples passed
`cargo check --locked --offline --target wasm32-unknown-unknown --lib --examples`.
Only the pre-existing unused ChipSize import warning remains. All three new
JavaScript runtimes pass `node --check`. See [copy dependencies](copy-components.md).
Consumer runtime styling still needs verification in a separate consuming app.

## Remaining fidelity scope

- Imperative toast queue/promise API and automatic host exclusivity across
  separate instances are unported. Mount one controlled host and own queues.
- Full-screen search source morph, search app bar, baseline divided style,
  virtual/async result integration, anchor movement and query-dependent Tab cases.
- Expandable/swipe/drag lists, typeahead/range selection and virtualization.
- Broad RTL, held/repeated/cancelled input, forced colors, text enlargement,
  custom seed/state contrast and audible live/selection announcements.
- Long snackbar actions/multiple lines, all alert tones and custom empty-state media.
- Android web touch, on-screen keyboard, predictive back and native packages remain paused.

## Reproduction

Run the dev script; wait for success and reload. Use Feedback catalogue pages for
Snackbars, Alert banners and Search, and Content for Lists, Avatars, Loading
placeholders and Empty states. Try the named controls in the matrix and their
copy buttons. For reduced motion, temporarily change the OS preference only with
user authorization, then restore it. Compact evidence is a 390px desktop viewport.
Next planned family: Scheduling (date picker, time picker and calendar).
