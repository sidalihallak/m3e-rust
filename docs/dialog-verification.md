# Dialogs and alert dialogs — 2026-10-10

## Current desktop verification — 2026-10-10

Browser access is restored. The earlier blocked follow-ups below are historical.
The current basic field is **400px** wide inside a **448px** dialog with visible
24px side insets. Entry frames establish that layout width remains 400px while
the ancestor scales; the label notch no longer stores scaled viewport widths.
After typing `Fidelity check` and tabbing out, the 12px floated label fits its
87.375px notch. Escape closes the native modal and returns focus to Create project.
See [the field report](text-field-verification.md) for the measured mask depths.

A new rendered defect was found: body overflow clipped the Project template
button's outer focus ring. `dialog.css` now reserves an 8px paint gutter with
negative margins, preserving the 24px visible content inset. Measured body width
416px, field width 400px; the 3px ring plus 3px offset fits in the gutter. Body
clientHeight equals scrollHeight (164px), so this example gains no false scrollbar.
The long-content example still scrolls internally (clientHeight 778px,
scrollHeight 1252px); actions start at y884 after body bottom y868, within the
980px viewport. The gutter also protects the first field's label, which extends 8px above it.
Full-screen body padding remains 16px 24px with zero margins.

Explicit FullScreen now measures **1094×980**, x/y 0, radius 0, header 56px,
field width 1046px, and `:modal=true` at the desktop viewport 1094×980. Close and
Save close the native modal; after the final rebuild Escape also closed it and
returned focus to Open full-screen editor. This is an explicit variant demo; use Adaptive for
production compact/wide adaptation as described below. With actual OS Reduce
Motion enabled, the modal reaches open with opacity 1/transform none and Close
still works. The original OFF setting was restored.

[Actual desktop frames](resumed-browser-verification.json),
[field and focus-ring screenshot](dialog-field-verified-preview.png),
[full-screen screenshot](dialog-fullscreen-verified-preview.png).
Final live build and exported examples are in [the composite index](composite-verification.md).
The pinned upstream and official four-section readings below remain the reference
contract; the text-field and dialog sources were re-read for these fixes.

A 360×800 viewport override did not apply to the preview in this run (actual
1094×980); it was reset. That attempt is recorded as desktop data, not compact
verification. Earlier valid compact evidence below predates the latest gutter
correction. Android, audible screen readers and final compact regression remain
open. Existing menu/modal containment checks below were not all repeated here.

Reproduce: reload after a successful live build, open Create project, watch the
notch during entry, type and Tab to Project template, inspect the full ring,
Escape and check focus return. Open full-screen editor at desktop width; compare
all four edges, header/field bounds and Close/Save dismissal. Follow previous
keyboard/alert/scroll checks when expanding coverage.

## Historical follow-ups before browser access was restored

## Explicit full-screen demo follow-up — 2026-10-10

The button labelled Open full-screen editor used `DialogVariant::Adaptive`, so
its desktop presentation was correctly basic but did not match the demo label.
The live example and its copyable FullScreenEditor snippet now explicitly use
`DialogVariant::FullScreen`. This bypasses the compact breakpoint and selects the
existing viewport-filling surface, 56px header and close action at every width.
The reusable Adaptive behavior remains compact full-screen/wider basic.

This is a variant showcase. Material's production guidance still recommends
full-screen dialogs on compact windows; responsive applications should use
Adaptive. The inspected pinned upstream `DialogContent` likewise selects the
full-screen layout when its explicit `variant` is `fullscreen`, without a width
condition. Sources and the earlier four-section guideline readings are below.

Browser inspection remains blocked by the existing URL security policy. The
earlier compact measurements establish the full-screen implementation at 360px;
they do not verify this updated desktop demo. To recheck, reload, click Open
full-screen editor at desktop width, verify the surface reaches all viewport
edges, then exercise Close, Save, Escape and focus return. Check the copied snippet
uses FullScreen too. Fresh live Dioxus build: **3.41s**, successful. Wasm source
check: **1.00s**, passed. Independently copied FullScreenEditor snippet: **0.24s**,
passed. The preview remains running; desktop rendered/input confirmation is pending.

## Text-field width follow-up — 2026-10-10

The field's reusable CSS capped it at a fixed 280px, leaving unused space in the
dialog body. `assets/text-field.css` now sets the outer field to 100% of its parent,
matching upstream `w-full`; no dialog-specific field override is needed. With
the default 448px card and 24px side padding, the expected content/field width is
400px. In a 360px full-screen editor with 24px body side padding it is 312px.
These are expected layout values, **not measured post-fix widths**.

The pinned upstream dialog and text-field sources were re-read. Wasm source
check passed in 0.38s; a fresh `./scripts/dev.sh` build succeeded in **9.94s**
and the preview remains running. See the
[text-field follow-up](text-field-verification.md) for provenance and reproduction.
Browser inspection was rejected by automatic approval review because the earlier
localhost access block remains active; final visual/interaction confirmation is
pending. Previous dialog runtime checks below predate this CSS correction.

User confirmation: after refreshing, the Create project field fills the content
width. A subsequently reported notch/label overflow revealed a likely measurement
issue: viewport label bounds included the dialog's entry scale. The field now
measures layout dimensions instead; the fresh **5.36s** build passes. Rendered
notch confirmation is pending; see the text-field follow-up for details.

## Scope and references

Files: `src/components/dialog.rs`, `dialog.js`, `compact.rs`, shared action/ripple
helpers, `assets/dialog.css`, `action-control.css`, `composite-motion.css`.
The API includes basic, explicit full-screen and responsive Adaptive dialogs,
urgent AlertDialog, optional icons, scrollable bodies and native text actions.

Read the rendered Material [overview](https://m3.material.io/components/dialogs/overview),
[specs](https://m3.material.io/components/dialogs/specs),
[guidelines](https://m3.material.io/components/dialogs/guidelines), and
[accessibility](https://m3.material.io/components/dialogs/accessibility).
Inspected upstream [dialog.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/dialog.tsx),
[alert-dialog.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/alert-dialog.tsx),
and [WAI dialog pattern](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/).
Upstream revision: `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`; date: 2026-10-10.

## Contract and implementation

| Feature | Value/behavior |
| --- | --- |
| Basic geometry | Default 448px (upstream), minimum 280, maximum 560, viewport margin 24 |
| Surface | `surface-container-high`, 28px corners, 24px padding, elevation 3 |
| Scrim | `scrim` role at 32%; a child surface inherits the consuming theme |
| Type/icon | Headline 24/32, body 14/20, optional secondary-color icon 24px |
| Spacing | Header/body 16px, body/actions 24px, trailing actions gap 8px |
| Compact editor | Adaptive uses full screen below 600px; 56px header, 0 corners |
| Motion | Upstream scale .8→1 with Expressive DefaultSpatial 440ms; opacity 240ms; scrim 330ms |
| Exit | Surface 150ms acceleration to scale .9/opacity 0; scrim 330ms |

Native `showModal()` supplies background inertness. The runtime traps Tab/Shift+Tab,
locks page scrolling, dismisses Escape according to policy, and restores the
opener after closing. AlertDialog uses `alertdialog`, ignores outside activation
and permits safe cancellation. General forms use `dialog`. Initial focus defaults
to the first interactive control; callers can supply `initial_focus_id`.

Full-screen is recommended only on compact windows. Use Adaptive for responsive
apps; its header action becomes a footer action on larger widths. Supply no more
than two basic actions, with cancellation first and confirmation last. Application
state must validate confirmation and intercept dismissal if unsaved data would
be discarded. The preview preserves its editor draft when dismissed.

## Runtime evidence

Checked input initial focus, form validation, Tab and Shift+Tab containment,
confirmation, Escape, outside pointer dismissal, and focus return. A menu inside
the modal opens in the top layer; Tab closes it and moves to Cancel. The alert
keeps the draft until explicit confirmation. Scrollable content exposes focus and
top/bottom dividers only where content remains; the 770px body scrolls 466px over
1236px content, ending with `more-above=true`, `more-below=false`.

The 360×800 **desktop viewport** produced a 360×800 full-screen Adaptive surface,
0px corners and 56px header; this is not Android evidence.
[Compact screenshot](dialog-compact-preview.png) captures an early entry frame;
its faint scaled content is not evidence of the settled appearance.

Sampling caught seconds being parsed as milliseconds in the JS runtime. After
fixing unit conversion, actual open frames progressed through scale .818, .883,
.932, .966 and settled at 1. Close frames decreased opacity/scale before native
closure. A 4px target overflow causing a false scrollbar was also fixed: the form
body now has equal client/scroll heights of 148px and initial focus on the input.
Actual OS Reduce Motion skips motion; its original setting was restored.

## Build, evidence, reproduction and gaps

See [build/copy index](composite-verification.md), [raw samples](composite-samples.json)
and [copy guide](copy-components.md). Run the preview, wait for successful build,
reload, visit `/#dialogs`, and exercise each dialog. Wait for completed native
closure before clicking another opener. Inspect scroll borders at both limits.

Android touch and keyboard behavior, native Dioxus packaging, audible screen-reader
output and forced colors remain unverified. Arbitrary unmount/reopen races and
multiple simultaneous modals have not received a dedicated application fixture.
Unsaved-data confirmation is a consuming-app responsibility, not inferred by the kit.
