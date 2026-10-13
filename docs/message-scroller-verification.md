# Message scroller verification —2026-10-12

## Scope and reference

Read rendered official [component catalogue](https://m3.material.io/components),
[accessibility principles](https://m3.material.io/foundations/overview/principles)
and [assistive technology](https://m3.material.io/foundations/overview/assistive-technology)
on2026-10-12. None of these application blocks has a dedicated official Material
component page/spec. Geometry comes from upstream; Material guidance informs
semantic roles, keyboard access, named controls, targets and readable text.
Existing button/icon-button/TextField reports cover reused primitive contracts.

Actual [upstream source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/message-scroller.tsx), revision
c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972, retrieved2026-10-12.

## Reference contract and implementation

Sources: message_scroller.rs, message_scroller.js and application-blocks.css.
Inspected styled source and actual @shadcn/react0.3.1 registry JS/declarations:
https://unpkg.com/@shadcn/react@0.3.1/dist/message-scroller/index.js
Primitive defaults: autoScroll false, default position end, edge threshold8px,
previous-item peek64px and margin0. Local defaults preserve auto-scroll opt-in,
end and8px edge threshold; supports Start/End/LastAnchor initial position,
preserve-prepend true, typed handle start/end/message commands and directional
jump control. Content gap32px, native thin/gutter-stable scrollbar, contained
overscroll. A named log viewport is keyboard-focusable, polite additions/text,
non-atomic. Automatic scroll does not focus messages or jump away from history.

Resize/mutation observers initialize after visibility, follow at end when enabled,
preserve a visible anchor during prepend, and leave reading position during append.
Wheel/touch/viewport keys cancel programmatic scrolling. Browser smooth scrolling
uses native timing; reduced motion uses instant scrolling. Settling follows stable
scroll offsets with cancellation generation ownership, not a guessed delay.
Observers/listeners/frames are removed on unmount. A hidden jump control is inert
and not tabbable; focus returns to the viewport when it hides.

The styled jump follows exact source200ms entry cubic-bezier(.23,1,.32,1),400ms
exit cubic-bezier(.7,0,.84,0), opacity,±100% translation and .95→1 scale.
Complete primitive streaming-turn spacer/64px peek behavior, virtualization,
visibility/current-anchor hooks, dynamic initial-position reapplication and nested
provider/viewport composition are extensions; literal React primitive parity is
not claimed. Message IDs/direct MessageScrollerItem children must be stable/unique.

## Runtime/build evidence

Desktop raw measurements and tested actions are saved in
[application-blocks-samples.json](application-blocks-samples.json). Screenshots
are in docs/evidence/application-blocks. Results below describe sampled states,
not every permutation. The live Wasm build and fresh independent export are
checked before completion; five copyable usages are included in the gallery.

## Platform scope and gaps

Android remains paused. Desktop/390px geometry does not establish mobile touch,
on-screen keyboard, native Android package behavior or audible screen-reader
announcements. Reduced-motion permutations beyond the sampled states, forced colors, text zoom, full RTL and
all dynamic/unmount/interruption permutations require wider evidence. No claim
of full upstream engine/API parity follows from the sampled core behavior.

## Reproduce

Run ./scripts/dev.sh, wait for successful build, reload the appropriate catalogue
route. Exercise pointer and native keyboard actions, inspect measured state and
focus, and use390px width separately for bounds. Copy the usage and compile a
fresh exported library. Compare the source contract before modifying behavior.

## Final build and evidence

Live Wasm serve: initial42.08s success; final Rust rebuild32.64s success, then
reload before the final inspection. One .app-shell measured. Fresh editable
export /private/tmp/m3e-application-export-1012 compiles its library and all103
examples in12.81s. The only warning is a pre-existing unused ChipSize import.
Five clipboard comparisons are exact: attachment615, bubble512, message417,
scroller527 and questionnaire673 characters. Rust/UI copy instructions and all
required JS/CSS/dependencies are in docs/copy-components.md.

Pilot typography remains Inter; upstream uses Roboto Flex. Full font/pixel
parity is not claimed. Android and audible screen readers remain open. Actual
OS Reduce Motion was sampled for shimmer/jump and restored; other reduced-motion
permutations and forced-colors/zoom/full RTL checks remain.

## Measured desktop and compact results

Initial12 items settle at end1169/1169px; appending makes1298/1298px and remains
following. After settled Home/PageDown, reading position340px stays340px when
appending. Prepending three older items preserves visible demo--1 at−74.50px
versus−74.75px after (0.25px rounding difference); scrollTop adjusts340→726.5px.
The earlier home/reading arrays are in-flight native keyboard samples; use the
settledReading* records to compare preservation.

Normal motion: opacity0/.95 scale/100% translate at rest, entry .338 at47ms,
.769 at78ms, .996 at174ms and1 at rest, computed200ms source duration. Exit
uses400ms and its source acceleration curve.55 samples per direction include
actual scroll offsets and masks. Smooth-scroll timing belongs to the browser,
not a universal Material duration. Interrupting at435px with Home settles at0,
followingfalse; the prior programmatic motion does not resume. End settles
1715/1715px and hides/inerts the control, returning focus to the named log.

Actual Reduce Motion on: querytrue, shimmer none, jump transitionnone, Enter
immediately reaches end1715px. OS off and prior settings pane restored; normal
transition list returns.390px scroller/body have no page overflow. Screenshots:
scroller-desktop.png/scroller-390.png. Wheel/touch cancellation, initialStart/
LastAnchor, public message-target command, prop changes, streaming and all RTL
permutations remain outside the sampled runtime matrix.


## Advanced behavior follow-up — 2026-10-13

Scope `#advanced-messages`. Current rendered reference: No dedicated current M3 component page; relevant baseline references retained above..
Actual [upstream](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/message-scroller.tsx) at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

Public status exposes following/scrollable/start/end, locally visible IDs and
current anchor; controller adds pause/resume/peek. `stream_anchor`, `streaming`,
scroll margin, 64px previous-item peek and optional controls slot. Uses stable
message IDs for prepend preservation (initial runtime check found DOM-node reuse
broke preservation and prompted this correction). Short turns align to anchor;
responses larger than remaining viewport follow end. Pausing keeps scrollTop160
while appending; resume follows; peek pauses. A long response ended at236 with
scrollHeight632 and viewport396, i.e. the true end. Final corrected prepend passes: top160→216 after56px insertion, with the
existing h0 screen position unchanged at467.992px.

No dedicated M3 MessageScroller component contract. Compared actual upstream
wrapper and `@shadcn/react` 0.3.1 implementation/types: edge8, previousPeek64,
margin0, anchor epsilon0.5. This is an application pattern with Material surfaces,
not an official M3 component. No full virtualization or upstream provider engine
parity; audible streaming/live-region UX and Android remain open.

### Verification and reproduction

Run `./scripts/dev.sh`, wait for a successful Wasm build, reload, then open the
listed hash route. Use actual native pointer and keyboard input. Fresh desktop
results and sample arrays are in [advanced-behavior-samples.json](advanced-behavior-samples.json).
All seven new copy controls matched their displayed Rust snippets exactly.
The exported editable library and **123 usage examples** passed an offline
`cargo check` for `wasm32-unknown-unknown` (2026-10-13, 11.99s); an existing
unused `ChipSize` import warning remains. Final anchoring/announcement fixes were rebuilt and rechecked; see the family report.

Android remains paused. Compact browser geometry is desktop responsive evidence,
not Android touch evidence. Audible screen-reader, forced-colors, large-font,
live-prop interruption and exhaustive variant coverage remain open. Source
settings alone do not establish runtime behavior.
