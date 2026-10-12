# Input groups — verification, 2026-10-12

## Scope and sources

`src/components/input_group.rs`, `assets/input-extensions.css` and
`src/input_extensions_gallery.rs` implement InputGroup, Addon, Text, Button,
Input and Textarea. One group contains one native control. The app owns its value
and action callbacks. These are upstream compositions, not a separately specified
Material component.

Read all four rendered official pages on 2026-10-12:
[overview](https://m3.material.io/components/text-fields/overview),
[specs](https://m3.material.io/components/text-fields/specs),
[guidelines](https://m3.material.io/components/text-fields/guidelines),
[accessibility](https://m3.material.io/components/text-fields/accessibility).
Material permits a persistent adjacent label aligned with the leading edge;
labels must remain visible, supporting/error text must be associated with the
control, and actions need descriptive names. Default fields are56dp high.

Inspected actual upstream sources at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`:
[input-group.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/input-group.tsx),
[input.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/input.tsx),
[textarea.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/textarea.tsx).
The previously retrieved upstream styles map rounded-xs to4px and rounded-md to12px.

## Contract and decisions

| Part | Implemented contract |
| --- | --- |
| Container | Full parent width, minimum56px, transparent,1px Outline,4px corners |
| Focus | Primary outline plus1px inset shadow; geometry does not shift |
| Error | Error outline/shadow, aria-invalid on input, linked error alert |
| Text | Body large16/24; adjacent label14/20; support12/16 |
| Addons | Inline start/end and block start/end;12px leading/trailing padding;8px gap |
| Block composition | Column layout,12px corners, textarea minimum96px, native scrolling |
| Action | Existing Button, default Text variant, native button/submit/reset;48px high target |
| Disabled | Native disabled fieldset disables control and all nested actions |
| Read-only | Native readonly; normal field styling and explicit example label |
| Price example | Text input with decimal inputmode; no number spinbutton |

Differences from upstream are intentional: native fieldset and linked persistent
label replace an unlabelled div group; block/textarea content has a fixed initial
height and scrolls, as recommended for web text areas. Group actions use48px
targets rather than upstream32/40px. Existing accepted Button motion is reused;
no shared motion token was changed. Input-group outlines change immediately;
there is no new geometry animation to measure.

Group error propagates through a reactive context to its native control. Input
can also declare its own error. Keep error supporting text on the enclosing
group. Supply one InputGroupInput or InputGroupTextarea per InputGroup; nested
independent fields should use independent groups.

## Runtime evidence

Live Wasm in Codex in-app browser, one `.app-shell`:

- Entered24.50 as decimal text, no native numeric counter.
- Typed a project query; Clear emptied it and became disabled.
- Entered two lines; Send message incremented the app status to1 and cleared the textarea.
- Tab from Price focused Find a project. Disabled controls/actions appear disabled
  in the browser accessibility tree and match native `:disabled`.
- A key press in the read-only workspace ID left atlas-2026 unchanged.
- Postal input reports aria-invalid=true and links the error message; alert appears
  in the accessibility tree. Focus measured Primary border and inset1px shadow.
- Desktop groups measure560×56px/4px; the block textarea group560×214px/12px.
- At390px viewport, client/body widths375px, no page horizontal overflow;
  groups fit327px content width; the action has a48px-high target.
- Both Copy Rust code controls matched clipboard exactly:567 and603 characters.

[Raw data](input-extensions-samples.json), screenshots:
[evidence/input-extensions/groups-desktop.png](evidence/input-extensions/groups-desktop.png),
[evidence/input-extensions/groups-390.png](evidence/input-extensions/groups-390.png).

## Build

Actual Dioxus serve succeeded:35.78s after native-group fixes; final gallery rebuild
24.70s. CSS compact correction hot-reloaded successfully. Refreshed source export
and all86 usages passed offline wasm32 cargo check in1.12s after the final snippet refresh; only pre-existing
unused ChipSize import warning. Initial RSX children/moved-ID and final snippet closure-inference errors were fixed
before runtime checks. Preview remains running.

## Gaps and reproduction

Android is paused;390px desktop emulation does not prove touch or keyboard behavior.
Audible screen reader, RTL, forced colors, zoom, live group-error changes, form
submission/reset and arbitrary custom addons remain untested. System Settings
was unavailable during this family's Reduce Motion attempt; no preference changed.
Action quick/held/repeated/interrupted motion relies on the accepted Button
baseline and was not independently sampled in this composition. Upstream compact
Input/file controls and automatic textarea growth are not ported here.

Run `./scripts/dev.sh`, wait for successful build, reload, open Selection → Input
groups. Repeat native typing, Clear, multi-line Send, read-only/disabled/error
checks; resize to390px and compare saved geometry. Copy both examples and compile
with the source exporter and required styles listed in copy-components.md.
