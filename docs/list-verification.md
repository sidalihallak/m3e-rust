# List / ListItem verification — 2026-10-11

## Scope and references

`src/components/list.rs, list.js; assets/list.css`. Retrieved official rendered pages and actual upstream
source on 2026-10-11; upstream revision `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[Overview](https://m3.material.io/components/lists/overview) · [Specs](https://m3.material.io/components/lists/specs) · [Guidelines](https://m3.material.io/components/lists/guidelines) · [Accessibility](https://m3.material.io/components/lists/accessibility)

[Upstream item](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/item.tsx) · [WAI listbox](https://www.w3.org/WAI/ARIA/apg/patterns/listbox/)

## Contract and runtime checks

Standard and segmented lists, one/two/three-line content, static, primary action,
independent secondary action, single and multiple selection. Native primary
buttons become options in a named listbox for selection, with aria-selected and
an extra check cue. Selection mode requires selected:Some(bool) on each row.
Secondary controls are siblings and are excluded from selection rows.

Observed standard row heights 56/72/88px. Text 16/24, supporting 14/20, metadata 12/16,
leading 16px, slot gap 16px. Segmented gap 2px, inner 4px and outer 16px corners.
Selected primary-container/on-primary-container matches the latest official
Expressive update, overriding upstream secondary-container. Shape uses existing
DefaultEffects 240ms and accepted shared ripple. Actual selection samples record
corner changes without scaling the row; selected radius settles 16px.

ArrowDown moved Personal→Team; Space selected Team exclusively. Next ArrowDown
wrapped to Personal and skipped disabled Archive. Native multiple selection kept
Design selected while Travel/Notes were activated by click/Enter. Secondary details
updated its own status; zero nested buttons. Compact390px showed no overflow.

### Remaining scope

Expandable/swipe rows, typeahead, drag/reorder, virtualization and range selection
are not included. Consumers must keep decorative slots noninteractive, use a
nonselection list for multiple actions, and own exclusivity. RTL arrows, reduced
motion and held press are implemented via shared primitives but were not separately
sampled for this list in the audit. Audible selection announcements remain open.


## Build, evidence and reproduction

Rust/Wasm check and the live Dioxus build passed. The independent exported library
and all 53 usage examples passed `cargo check --locked --offline --target
wasm32-unknown-unknown --lib --examples`. Eight new gallery copy controls matched
the displayed Rust code exactly. See [family verification](feedback-content-verification.md)
and [raw samples](feedback-content-samples.json). Sample timestamps are external
wall-clock times from a concurrent observation loop; they are not event-relative
animation timestamps. Bounding rectangles include transforms.

Run `./scripts/dev.sh`, wait for a successful Dioxus build, reload the preview,
and open the component's catalogue page. Repeat the inputs above. Include the
styles/runtime dependencies in [copy instructions](copy-components.md).

Desktop evidence does not establish Android touch or native Dioxus Android.
Android remains paused. Audible VoiceOver, forced colors, custom seed contrast,
200% text, broad RTL, interrupted animation and held/touch-cancellation matrices
remain unverified unless explicitly listed below. Do not treat this report as
exhaustive pixel or platform parity.


## Advanced behavior follow-up — 2026-10-13

Scope `#advanced-lists`. Current rendered reference: [overview](https://m3.material.io/components/lists/overview), [specs](https://m3.material.io/components/lists/specs), [guidelines](https://m3.material.io/components/lists/guidelines), [accessibility](https://m3.material.io/components/lists/accessibility).
Actual [upstream](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/item.tsx) at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

ListItem optional expanded/content/callback uses measured disclosure runtime;
selection rows do not combine expansion with selection. Prefix typeahead moves
Design→Themes; Enter expands, Open review adds one activation. Archive native
Space and real horizontal swipe both archive Rust, and Undo restores it without
an accidental extra primary activation. The 48px visible action is the accessible
alternative to swipe. Source upstream item is a styling/composition primitive.

SwipeAction commits a reversible action on release, not the official Android
multi-button swipe-to-reveal animation. That visual reveal is explicitly still
open. Expansion uses existing measured-height effects rather than a full-screen
container-transform pattern. Long-label typeahead/locales and gesture cancellation
need broader evidence.

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
