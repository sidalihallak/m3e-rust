# ModalNavigationRail verification — 2026-10-11

## Scope and source

Implementation: `src/components/navigation.rs and modal_navigation.js`, `assets/navigation.css`. Controlled native dialog, inert background, dismissal, focus restoration and boundary Tab wrapping.

Official reference read on 2026-10-10: [overview](https://m3.material.io/components/navigation-rail/overview) · [specs](https://m3.material.io/components/navigation-rail/specs) · [guidelines](https://m3.material.io/components/navigation-rail/guidelines) · [accessibility](https://m3.material.io/components/navigation-rail/accessibility).
Actual upstream revision, implementation values, deliberate differences and
measured results are in [the navigation report](navigation-verification.md).

## Verification and reproduction

The family report records per-state pointer/keyboard checks, motion observations,
actual OS reduced motion, build/export results and steps to reproduce. This report
inherits those results only for the states explicitly measured there. Keep this
component's Rust/CSS and required runtime dependencies together using
[the copy guide](copy-components.md). Each component has a real copyable gallery
example in `src/navigation_gallery.rs`.

## Remaining gaps

Android remains paused; audible screen-reader, text enlargement and full pixel
parity are unverified. Final browser recheck was blocked by URL policy, and raw
sample arrays from the earlier session were not retained. See the family report
for component-specific untested variants and final-edit limitations.

## Drawer motion follow-up — 2026-10-11

Official rail overview/specs/guidelines/accessibility were reread in the rendered
browser. [Motion physics](https://m3.material.io/styles/motion/overview/how-it-works)
explicitly assigns **default spatial** to an expanded navigation rail's movement
and **default effects** to rail content opacity. This supports the existing
upstream-derived 440ms/240ms spring approximations; it does not justify guessing
a longer duration. Current upstream `navigation.tsx` was fetched through GitHub
and matched the previously inspected c37c0d2 source byte-for-byte. Upstream styles
supply rail geometry/width transitions, not this port's native modal lifecycle.

The prior native `::backdrop` appeared immediately while only the narrow dialog
translated. The new native dialog covers the viewport with separate explicit
scrim and panel children. The panel uses DefaultSpatial; the scrim opacity uses
DefaultEffects. Both entry and exit animate. Interruptions capture interpolated
transform/opacity before cancellation; the generation check owns final close.
RTL moves from the logical leading edge. Reduced motion uses zero duration,
including when the preference changes during animation. Native modality, focus
wrapping, opener restoration and saved document overflow behavior are retained
in implementation. No shared button/ripple token changes were made.

Source/CSS: `src/components/navigation.rs`, `modal_navigation.js`,
`assets/navigation.css`. Public component API and copy dependencies are unchanged.
The `.m3-modal-rail` element is now the viewport-sized semantic dialog;
`.m3-modal-rail__surface` is the 320px visual panel. Future geometry/motion samples
must measure the **surface**, not the enclosing dialog.

Build evidence: locked offline Wasm check passed (8.96s); live preview reported
successful build (5.55s), and JavaScript syntax/whitespace checks passed.
All 40 independent exported snippets passed with the updated component (4.19s; one existing unused ChipSize import warning).

**Runtime gap:** local browser access was rejected by URL security policy on this
follow-up. No new pointer/keyboard or frame sampling result is claimed. Earlier
checks in the family report apply to the previous modal structure and do not
verify the new one. Android remains paused. Fresh checks must cover quick and
repeated opening, dismissal during entry, entry during exit, both Tab boundaries,
Escape/scrim/selection, focus and scroll restoration, RTL and reduced motion.
Save panel-transform and scrim-opacity samples during those actual inputs.

## In-app browser recheck — 2026-10-11

Opening a fresh `http://127.0.0.1:8080/` tab directly in Codex's in-app browser
succeeded. This supersedes the earlier access-block status. One app shell and
`prefers-reduced-motion: false` were observed.

The actual served duration tokens were `.44s` and `.24s`. The modal runtime's
`parseFloat` treated those as 0.44/0.24 milliseconds, causing the sudden motion.
It now converts seconds to milliseconds and also accepts explicit ms values.
The live rebuilt version passed (8.57s). Actual pointer-open and keyboard-Escape
samples show panel movement through -320, -291.5, -232.2, -186.6 and -126.3px,
then a small spring overshoot before resting; the scrim independently fades
0→0.255→0.586→0.743→0.880→1. Exit has multiple interpolated frames, followed by
native close, scroll restoration and focus back on Open navigation.

Desktop catalogue width interpolated 0→23.5→69.0→117.5→171.5→240px, with a slight
spring overshoot. Inner content remained 240px wide throughout. Closing also
interpolated and removed its focus targets. A newly found `inert="false"` bug
made the open catalogue inaccessible; the attribute is now omitted while open.
The open catalogue/search/links are visible in the accessibility tree.

Both modal Tab boundaries were checked. A 390×844 resized desktop check exercised
family Selection and native picker navigation to `#selection-fields`. This is
responsive desktop evidence, not Android touch coverage. The viewport was reset.
Raw observations: [navigation-followup-samples.json](navigation-followup-samples.json).
Times are external sampler elapsed times, not DOM event timestamps. Historical
`modalOpen`/`modalClose` capture the unit bug; `modalOpenFixed`/`modalCloseFixed`
capture the corrected version. [Desktop screenshot](showcase-navigation-desktop.png).

Still untested after the correction: reduced-motion preference changes, interrupted
entry/exit, RTL modal motion, enlarged text and Android. Earlier reduced-motion
results must not be treated as verification of this changed runtime.

## End bounce correction — 2026-10-11

The user reported a final bounce on the desktop modal rail demo. Actual frame
samples confirmed the panel's leading edge reached **x = +4.73px** before settling
at zero. The shared DefaultSpatial spring peaks at 1.0151; over a 320px translation
this produces about 4.83px of overshoot, exposing a gap at the viewport edge.
This is geometry resulting from the spring, not an event or release timer defect.

The modal now uses `--m3-modal-rail-spatial` in `navigation.css`: the same 49
DefaultSpatial sample positions, with output bounded to [0,1]. Its 440ms duration
and approach to the edge are preserved; excess travel after first arrival is
removed. Effects/scrim still use the original 240ms curve. Shared spatial tokens,
button behavior and standard rail motion are unchanged. This is an intentional
edge-attached-panel customization, **not** a claim that Material forbids spring
bounce. [Official motion guidance](https://m3.material.io/styles/motion/overview/how-it-works)
permits per-element customization and assigns DefaultSpatial to expanded rails.
The official rail pages and upstream source inspected earlier in this report
remain the reference contract; upstream does not implement our native modal lifecycle.

Actual pointer-open samples after rebuilding showed **maximum x = 0px**, intermediate
negative positions and constant 320px width; no positive edge travel remained.
Escape closed the modal, restored empty overflow and focused Open modal rail.
One gallery shell was retained; live build passed (8.54s), locked offline Wasm
check passed, and JS syntax/whitespace checks passed.

[Before/after raw samples](modal-rail-edge-samples.json) retain the comparison;
[resting-state screenshot](modal-rail-edge.png) documents appearance, not motion.
Reproduce at `#navigation-rails`: open the modal and sample its surface transform
through the final half of entry. Android, RTL and reduced-motion rechecks remain
open; this turn establishes desktop LTR pointer entry and keyboard dismissal only.

## Residual surface/shadow correction — 2026-10-11

A further user report described a momentary surface at animation end. Actual
closing frames showed the panel at x=-320px and scrim opacity zero while its
level-2 box shadow retained 0.2/0.133 alpha. The blurred shadow extended beyond
the off-screen panel and remained until native close, exposing a thin leading-edge
strip. The before screenshot captures this state while the dialog is still open.

Panel elevation now fades using the existing DefaultEffects 240ms token, together
with the scrim. Opening fades elevation in; closing fades it to transparent.
Interpolated shadow is captured before cancellation for interrupted motion, and
final native close still belongs to the current animation generation. No duration,
geometry, shared spring or component API changes were needed. This is layering
cleanup; it does not change the official/upstream resting elevation contract.
The references established earlier in this report remain applicable.

After the successful live build (7.66s), the browser was reloaded. Actual pointer
open/close frame samples showed transparent shadow and zero scrim opacity while
x=-320px and the dialog was still closing. A screenshot of that state no longer
shows the leading-edge strip. Native close restored focus to Open modal rail.
Locked offline Wasm, JS syntax and whitespace checks passed.

Evidence: [raw states](modal-rail-shadow-samples.json),
[before](modal-rail-shadow-before.png), [after](modal-rail-shadow-after.png).
Elapsed sampler times include browser screenshot latency and are not DOM event
timestamps. This establishes desktop LTR motion/paint for the sampled pointer
sequence; Android, reduced motion and RTL rechecks remain open.
