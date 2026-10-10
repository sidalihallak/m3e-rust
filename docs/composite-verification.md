# Composite component verification — 2026-10-10

## Current post-restart browser check — 2026-10-10

The permission block is resolved. Reloaded the successful live build and measured
one `.app-shell`; no error/warning console entries were captured. Width/notch,
full-screen dialog, card and divider follow-ups now have desktop runtime evidence
in their reports and [the complete new sample buffer](resumed-browser-verification.json).

All nine copy controls for standard group, split, dialog, alert, full-screen,
menu, context menu, card and divider showed copied success. The last Divider
snippet was then pasted with the native clipboard shortcut into the Note field:
**509 characters, exact match**. Test text was cleared. This establishes real
clipboard output for the shared workflow, not runtime checks of every example.

Actual OS Reduce Motion was enabled under prior user authorization; cards retained
activation with immediate final ripple geometry, and full-screen dialog retained open/close behavior with settled opacity 1 and
transform none. The preference was restored OFF. Detailed raw frames
are retained. The failed 360×800 override remained 1094×980 and was reset; those
frames are explicitly desktop data. Earlier compact evidence is historical.

Current corrections: 16px card body/action padding per freshly rendered specs,
8px stacked-card gap, and dialog scroll-body paint space to prevent clipped focus
rings/floating labels. Android remains paused; audible screen-reader, extended
held-pointer, final compact and consuming-app runtime checks remain open.

## Delivered scope

- [Standard button groups](standard-button-group-verification.md): actions,
  controlled single/multiple/required selection, five sizes and shared width motion.
- [Split buttons](split-button-verification.md): four styles, five sizes,
  independent actions and related menus with inward Standard-scheme rotation.
- [Dialogs](dialog-verification.md): basic forms, urgent alerts, scrollable content,
  responsive full-screen presentation, modal focus and dismissal.
- [Menus](menu-verification.md): standard/vibrant/baseline, dropdown/context,
  cascading actions, checkbox/radio items, disabled focus and keyboard navigation.

Each report links the actual official overview, specs, guidelines/accessibility
and inspected upstream source. Seven new real Rust examples appear in the gallery.
The existing button motion baseline is preserved; shared composites use their own
copyable action helper. Theme generation now supplies explicit scrim/shadow roles.

## Final checks after restored browser access

- Final `./scripts/dev.sh` build: **13.68s**, successful; browser reloaded after
  completion, one app shell, Reduce Motion false, 8px dialog paint gutter present.
- Locked/offline Wasm source check: **0.81s**, passed.
- Refreshed scratch export sources and CSS; all **27** examples compiled for
  Wasm in **3.08s**. Existing unused ChipSize import warning remains.
- `git diff --check`: passed. No parallel standalone Dioxus build was run.
- Preview left running on `http://127.0.0.1:8080/`.

## Earlier build and source-copy checks

- Final live `./scripts/dev.sh`: Dioxus 0.7.10, Rust 1.99, successful Wasm build
  in **33.52s** after restoring cleared temporary tools, then **7.87s** after
  formatting. Server left running on
  `http://127.0.0.1:8080/`.
- Final `cargo check --locked --offline --target wasm32-unknown-unknown`: passed,
  **2.28s** after formatting (earlier 3.24s). JavaScript syntax checks for menu/dialog runtimes passed earlier.
- Exported the complete kit to `/private/tmp/m3e-composites-export-final` and
  independently compiled all **27** gallery examples for Wasm: passed, **0.49s**
  after formatting (earlier 0.83s).
  The remaining warning is an existing unused ChipSize import.
  Badge's required `unread` prop was supplied by the scratch launch wrapper.
- Independent checks found missing explicit `MenuSelection` closure annotations
  in the new split/menu/context snippets; the gallery examples are corrected.
- No standalone `dx build` was run alongside serve.

The source export includes both popup `.js` files and component CSS; exported
guide links point to the repository's reports. Compilation
establishes API/dependency completeness. Runtime style/interaction evidence comes
from the original preview, not a separately launched consumer application.

## Historical runtime evidence and interruption

Before the session interruption, the successful live build was reloaded and
desktop pointer/keyboard checks, computed motion samples and 360×800 responsive
checks were performed. Actual macOS Reduce Motion was temporarily enabled with
user approval and restored to off. Native modal state and keyboard containment
were observed; Android remains paused and audible screen-reader output unverified.

[Retained sample excerpts](composite-samples.json) are transcribed from browser
tool output. The interruption cleared the in-memory full sample buffers and the
temporary toolchain. The retained JSON explicitly distinguishes measured frames,
observed summaries and configured reference values; it is not a complete trace.
The compact screenshot was saved before the interruption.

After tools were restored and the final build succeeded, browser automation
rejected reopening the localhost tab under its URL security policy. No alternative
browser surface was used to bypass that refusal. At that point the post-restart reload, copy controls and screenshot were unverified.
The restored-access check at the top of this report closes those specific gaps. Earlier checks apply to the same implementation;
the subsequent source change corrected usage-example type annotations.

## Reproduce and extend

Run `./scripts/dev.sh`, wait for the successful build, reload and verify one
`.app-shell`. Visit the four component anchors. Follow each report's keyboard,
pointer and geometry checks. Sample computed styles concurrently with real input;
do not inject synthetic input from a page evaluator. Wait for completed popup
closure before activating another modal opener. Save raw samples immediately to
avoid losing in-memory evidence across a restart.

Remaining coverage: extended held/cancelled pointer gestures, audible assistive
technology, Android touch/keyboard, native packaging, forced colors, large text,
dynamic prop/unmount permutations and a consuming-app runtime. These gaps are not
represented as completed fidelity checks. See [remaining work](remaining-work.md).
