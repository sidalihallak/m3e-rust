# Notification provider and queue verification

## Scope and date

2026-10-12. New `src/components/toaster.rs`, embedded `toaster.js`,
`assets/toaster.css`, and `src/notification_gallery.rs`. The reusable API covers
ToastProvider, scoped ToastManager, Toaster, options/actions/IDs/dismissal reasons,
reactive snapshots and a Rust future helper. Existing Snackbar/Toast primitives
remain available. This report covers the new single-visible-message queue.

## References inspected

Official rendered Material pages, read in the Codex in-app browser on2026-10-12:

- [Overview](https://m3.material.io/components/snackbar/overview)
- [Specs](https://m3.material.io/components/snackbar/specs)
- [Guidelines](https://m3.material.io/components/snackbar/guidelines)
- [Accessibility](https://m3.material.io/components/snackbar/accessibility)

Actual upstream sources at revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`:

- [sonner.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sonner.tsx)
- [toast.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/toast.tsx)
- [Shared theme/motion CSS](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css)

The inspected Sonner adapter deliberately presents one Material snackbar and
queues the rest. The separate Base UI toast implementation has stack/peek/swipe
behavior. This port follows the Sonner adapter; it does not claim both engines.

## Reference contract and chosen values

| Contract | Implementation and source comparison |
| --- | --- |
| Visibility | One snackbar, FIFO; promote only after exit completion. Same-ID updates preserve position. |
| Colors | inverse-surface container, inverse-on-surface text, inverse-primary action. Kinds have no automatic icons or semantic color substitutions, matching Sonner. |
| Geometry | 4px corners, elevation3, BodyMedium14/20,16px leading inset; desktop344–600px, compact available width,12px edges plus safe area/offset. |
| One/two lines |48px single line;64px two lines. Source py3.5 produces68px for two lines; choose official Material64px with measured line-count padding. |
| Long action | Optional separate action row; zero inter-row gap,8px trailing inset,4px bottom inset, native48px action/close targets. |
| Duration |4000ms default; timed inputs clamp4000–10000ms;0 persists. Actions and loading persist regardless of duration. Official guidance requires actionable messages to remain and timed information to have equivalent inline feedback. |
| Motion | Upstream FastSpatial360ms linear spring, scale .8→1 /1→.8; FastEffects150ms opacity. Existing shared tokens supply both easings. Exit awaits actual animations rather than700ms fallback. |
| Reduced motion | Spatial/effect durations0; queue advances without animation. Upstream retains opacity under reduced motion; instantaneous feedback is this port's explicit choice. |
| Keyboard/focus | Native buttons, Alt+G notification shortcut, Escape inside host, no trap/no initial focus grab, restore trigger when appropriate. Closing content is inert/aria-hidden. |
| Announcement | Persistent polite atomic status region, including error kind; clear when idle. No simultaneous assertive error announcement. |
| Pause | Independent hover/focus/hidden holds preserve remaining time. Same-ID revisions reset their own exposure timer. |
| Futures | Loading→success/error updates same instance. Return original Result. Dismissed or superseded results cannot resurrect a stale notification. This strengthens upstream's late-result behavior. |
| Scope | Dioxus provider context rather than process-wide singleton. One host per provider near root; no portal/container adapter. |

Keep messages brief, with one concise text action. Consumers provide persistent
inline feedback for timed results and enough offset to clear their navigation or
FAB. Demo/examples now show that inline feedback. The host does not cancel
application work when its notification is dismissed.

## Runtime verification matrix

Desktop macOS Codex IAB, actual pointer and native keyboard input. Screenshots
and390px viewport checks are desktop web evidence, not Android touch evidence.

| Check | Observed result |
| --- | --- |
| Queue3, pointer close | First→Second→Third FIFO, one card in every sample; closing inert="", active inert absent. No overlap. Close restored Queue three messages focus. |
| Current ID update | Same UID, revision increments; Updated message and description, persistent0 duration;344×64px rest; polite text updates. |
| Queued ID update/remove | Current First unchanged, queued target updated without duplication; dismiss target removes only target, preserves Second/Third. |
| Dismiss all | Clears previous current/queue, callbacks report Api, live text clears after exit; inline feedback retained. |
| Alt+G / Escape | Shortcut focuses Dismiss notification; Escape closes and returns Show notification focus. |
| Native action Space | Undo executes once, action count1, dismissal Action, focus returns action trigger. |
| Real Rust future success | Saving project…→Project saved, same UID, revision1; duration0→4000; Result increments task count. |
| Real Rust future failure | Loading→Could not save project/error, same UID; original failure delivered to caller. |
| Dismiss pending future | Completing dismissed work increments task count/inline result, leaves no current/queue/live notification. No resurrection. |
| Stale timer | Persistent same-ID update remained after>5s, beyond original4s deadline. |
| Timed expiry |4000ms notification removed; first empty sample4546ms from sampling start, including remaining entry, exposure and exit; Timeout callback. Last update: Changes saved remains inline. |
| Hover/focus holds | Combined hold, then hover alone with focus on Copy Rust code, then focus alone after pointer moved to header: each survived>5s. Releasing both allowed expiry, first empty sample4624ms after release sampling start. |
| Reduced Motion | Actual macOS off→on; browser media true. Sampled cards opacity1/transform none; successor visible by47ms sample after dismissal. Original off restored, media false, original Settings panel restored. |
| Light/dark roles | Light container rgb(50,47,53), text rgb(245,239,247); dark container rgb(230,224,233), text rgb(50,47,53), each matches theme inverse roles. |
| Positions/offset | Desktop start x12; end right27 including15px scrollbar (12px content edge). Center centered in usable width.80px offset→92px bottom clearance. |
| Compact390×844 |351px card in375px usable width,12px content edges; no document overflow. Long action100px high; two-line update≈64px. Action and close48px targets. |
| Copy controls | All4 clipboard texts exactly match displayed Rust. First provider read was stale; repeat stable button activation/next-roundtrip read passed. Raw evidence retains failed first attempt. |
| Final mount | Reloaded successful serve build: one app-shell, Reduced Motion false. |

The disabled Start task trigger loses native focus to BODY on pending state;
this is application trigger disabling, not host focus theft. BODY text was omitted
from raw evidence. IDs restart after preview reload/provider remount; separate
runs are not a single uninterrupted ID sequence.

## Motion measurements

`toaster-samples.json` contains42-frame normal entry and42-frame exit+handoff
samples, independent timer/pause/async/keyboard snapshots, reduced samples and
geometry/copy results. External sampling reads computed styles while real input
is dispatched; time values include browser input/inspection latency.

Entry first visible sample173ms: opacity0, scale.8. At354ms scale1.01407;
at380ms1.01886, consistent with the upstream FastSpatial spring overshoot.
Rest open observed626ms from sampling start. Exit begins by69ms, opacity≈.0847
at136ms; scale.781818 at220ms and.801786 at341ms. The next UID enters after
exit, and is open by818ms. These timestamps are sampled observations, not exact
animation-start latency guarantees. Runtime token reads are.36s spatial/.15s
effects with the full source linear() easings saved in `finalRuntime`.

The original desktop long-action sample108px preceded the zero-row-gap fix;
compact100px and final desktop450.58×100px are after that fix. The final CSS/export contains zero row gap.

## Build and export

`./scripts/dev.sh` with isolated tools/CARGO_INCREMENTAL=0/jobs2 reported successful
Dioxus Web builds (including28.32s and46.25s after latest runtime edits). CSS
follow-up hotpatch succeeded, then browser reloaded. No separate dx build ran
concurrently with serve. Preview remains running on8080.

Fresh editable kit exported to `/private/tmp/m3e-toaster-final-1012`; all116
independent usage examples and library passed locked/offline wasm32 Cargo check
in32.41s. Only existing unused ChipSize import warning in older chip example.
The four new snippets include equivalent inline result feedback.

## Evidence and gaps

- [Raw samples](toaster-samples.json)
- [Desktop preview](toaster-desktop.png)
- [Compact long action + offset](toaster-compact.png)

Not yet verified: Android/native Dioxus touch, RTL, actual audible screen-reader
announcement ordering, document-hidden pause via app switching, multi-provider
shortcut arbitration, every reentrant callback/race permutation, queued future
resolution/superseding in runtime, arbitrary long/localized messages and external
consuming-app runtime. Code guards exist for revisions, callbacks and late results;
those are not substitutes for missing runtime checks.

Remaining upstream API differences: no Base UI stack/swipe engine, portal/container
adapter, arbitrary React render slots, result-to-message mapper/finally helper or
separate onAutoClose hook. One ondismiss reason channel is exposed. Reference
source comparison is source/contract based; no synchronized side-by-side upstream
runtime motion capture was made for this family. Accessibility semantics and
keyboard checks do not establish audible VoiceOver behavior.

## Reproduce

1. Run `./scripts/dev.sh`, wait successful build, reload `/#notification-host`.
2. Queue3; close repeatedly and inspect FIFO/no overlap. Add/update/remove target
   while queued, then update current by ID and wait beyond previous deadline.
3. Start task; Complete/Fail. Dismiss pending, then complete; check no resurrection
   and retained inline result. Show action, Alt+G, activate Undo with Space; Escape
   another notification and inspect trigger focus.
4. Show timed message, hover/focus for>4s, release holds independently, then both;
   inspect expiry/inline result. Sample transform/opacity through enter and exit.
5. Check start/center/end,80px offset, both themes,390px layout, actual Reduce Motion
   with user consent; restore setting/viewport. Copy all4 snippets and compare exact
   clipboard text on the next roundtrip.
6. Export to a new directory with `scripts/export-kit.py`, compile usage examples
   against that library for wasm32 with isolated tools.
