# Accessible card and chip reordering — 2026-10-13

## Scope and references

Preview `#advanced-gestures`. Current official rendered pages read on Oct12–13:
[overview](https://m3.material.io/components/cards/overview), [specs](https://m3.material.io/components/cards/specs), [guidelines](https://m3.material.io/components/cards/guidelines), [accessibility](https://m3.material.io/components/cards/accessibility); [overview](https://m3.material.io/components/chips/overview), [specs](https://m3.material.io/components/chips/specs), [guidelines](https://m3.material.io/components/chips/guidelines), [accessibility](https://m3.material.io/components/chips/accessibility).
Actual [upstream source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/card.tsx), pinned `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`;
also chip, switch, tabs, item, combobox, table, pagination and message-scroller.

## Contract, checks and limits

`reorder.rs`/`.js`, `advanced.css`: controlled `ReorderItem` values and
`ReorderGroup`, Vertical/Horizontal axes. 48px native drag handles and Move
earlier/later buttons. Space picks up; arrows/Home/End move; Enter/Space commits;
Escape cancels. Polite position announcements; focus restored to the moved item.
Move buttons provide the official single-pointer alternative to dragging.

Dragged card/chip level4 shadow and 0.16 state layer; card gap 8px. These
values are grounded in Material Web v0_192 card/input-chip dragged token files
and shared Material state tokens. Neighbor FLIP uses FastSpatial 360ms easing;
controls preserve scrolling begun on content. Chip groups use an inline
horizontal scroll layout. Official card/chip contracts permit dragging, but
upstream `card.tsx`/`chip.tsx` supply styled primitives, not a DnD engine.

Actual pointer drag across the next row centre changes Draft/Review/Publish to
Review/Draft/Publish. Keyboard commit matches that order; Escape restores it.
Single-pointer Move earlier returns Draft to first and focuses Move Draft later
at the boundary. Saved samples show nonzero neighbor transforms during reorder;
the initial sample interval is shorter than the full360ms interpolation. Final
1.538s samples cover the transition: neighbor−108px through spring overshoot
to no transform. Native Move now announces “Draft moved to position2.”
Actual OS Reduce Motion yields no neighbor transform and still commits order.

No cross-group dragging, edge autoscroll, virtualized drag engine or exhaustive
interrupted motion comparison is claimed. Disabled handles cannot initiate a
move; other items may move around them. IDs must be unique; parent must apply
`onchange` order. Larger-engine and Android behavior remain open.

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
