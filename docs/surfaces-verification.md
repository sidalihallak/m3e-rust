# Surfaces verification — 2026-10-11

## Scope

Public Dioxus `BottomSheet`, `SideSheet`, `Sheet`, `SheetVariant` and `SheetSide`.
Sources: `src/components/sheet.rs`, `sheet.js`, `assets/sheet.css`.
Application examples: `src/surfaces_gallery.rs`; new Content catalogue routes
`#bottom-sheets`, `#side-sheets`, `#sheets`. This adds five copyable usage examples.

The family covers standard/modal bottom sheets, partial/expanded bottom-sheet
heights, handle dragging, docked/detached standard side sheets, modal side sheets,
and the upstream generic sheet's top/bottom/logical-start/logical-end placement.
Top placement is a library extension, not an official Material component variant.
The existing rskit-ui project and accepted button motion were not changed.

## Official and upstream references

Read rendered official pages on 2026-10-11:

- Bottom sheets: [overview](https://m3.material.io/components/bottom-sheets/overview),
  [specs](https://m3.material.io/components/bottom-sheets/specs),
  [guidelines](https://m3.material.io/components/bottom-sheets/guidelines),
  [accessibility](https://m3.material.io/components/bottom-sheets/accessibility).
- Side sheets: [overview](https://m3.material.io/components/side-sheets/overview),
  [specs](https://m3.material.io/components/side-sheets/specs),
  [guidelines](https://m3.material.io/components/side-sheets/guidelines),
  [accessibility](https://m3.material.io/components/side-sheets/accessibility).
- [Motion physics](https://m3.material.io/styles/motion/overview/how-it-works)
  assigns DefaultSpatial to supplementary panels and DefaultEffects to opacity.

Actual upstream source read through GitHub, revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`:
[drawer.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/drawer.tsx),
[sheet.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/sheet.tsx),
[side-sheet.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/side-sheet.tsx).
Upstream uses Base UI for modal/gesture behavior. This port supplies native dialog
and pointer runtime rather than translating React primitives literally.

## Contract and choices

| Element | Implemented value / reference |
| --- | --- |
| Bottom sheet surface | Surface container low; on surface variant; 28px top corners. Official maximum width 640px; full width at compact sizes. |
| Initial modal bottom height | 50% of viewport, matching the official initial-height cap. Expanded height leaves 72px above at compact widths and 56px above at widths over 640px. |
| Wider bottom margins | Width min(640px, viewport−112px), giving at least 56px side margins. |
| Drag handle | 32×4px mark from upstream, 48px high interactive area from official accessibility guidance. Named native button, focus ring and aria-expanded. |
| Standard bottom | Positioned in a supplied application pane; no scrim/scroll lock. Partial height 50%; expanded leaves 16px in that pane (application adaptation, not a viewport spec). |
| Standard side | Width clamped 256–400px, default 360px from upstream; surface background, inner divider, square docked corners. Fixed-width inner content while outer width animates. |
| Detached side | 16px margin and corners; surface container low, no elevation, matching upstream configuration. |
| Modal side | Logical trailing edge by default; 16px exposed corners; max 400px, default 360px, leaves at least 32px of viewport visible. RTL reverses the logical edge. |
| Content | Independent vertical scrolling; 24px inline body padding. Long titles wrap. No horizontal body scrolling. |
| Actions | Footer optional, minimum 72px; 16px top / 24px bottom / 24px inline padding. Height grows to fit supplied controls, rather than forcing 40px actions into a 72px box. |
| Motion | Existing 440ms DefaultSpatial and 240ms DefaultEffects approximations. CSS s/ms duration conversion is preserved. Edge-attached spatial output bounded to [0,1], following the rail gap fix. Scrim and elevation fade independently of position. |
| Semantics | Modal native dialog with title/description association, inert background, initial first-control focus and boundary Tab wrapping. Standard side role dialog without modality; standard bottom named region. Closed standard surfaces use omitted/present inert attribute correctly. |

Modal bottom sheets are primarily a compact/mobile pattern; Material recommends
side sheets for expanded layouts. The desktop gallery exposes the modal bottom
example for implementation comparison. Consumers choose the responsive variant;
this API does not automatically convert a bottom sheet into a side sheet.

Compared with upstream generic sheets, this port prefers official bottom-sheet
640px/50%-initial geometry and official side-sheet max-width rules over generic
75%/24rem side sizing. Logical Start/End replaces physical left/right props.
Drag is initiated **on the handle**, leaving body content available for scrolling.
The release threshold is min(96px, 20% of initial panel height), a port-specific
interaction choice. It is not claimed as a Material-specified distance. No custom
spring durations or exaggerated scaling were introduced.

## Actual desktop verification

| Input/state | Result |
| --- | --- |
| Pointer entry/exit | Sampled transforms, geometry, opacity, shadow and phases during actual open and Escape. Intermediate states observed, then native close, opener focus and overflow restoration. One app shell. |
| Desktop geometry 1329×980 | Modal bottom 640×490px at x344.5/y490. Expanded 640×924px at y56. Handle 48px; corner radius 28px. Partial body client height297px vs scrollHeight392px. |
| Keyboard expansion | Native Space on handle changed partial→expanded. Enter worked with actual reduced motion. Arrow keys implemented; not separately exercised in this audit. |
| Focus boundaries | Tab from Done returned to handle; Shift+Tab from handle went to Done. Escape dismissed and returned focus to Open share sheet. |
| Dragging | Desktop pointer handle drag up expanded; at resized 390×844, upward drag tracked height and ended expanded, downward drag collapsed, second downward drag dismissed. Controls did not also toggle from the synthesized click. |
| Compact geometry | 390×422px initial; expanded 390×772px at y72; 28px corners and 48px handle. These are resized desktop results, not Android touch evidence. |
| Body scroll / scrim | Native scroll input moved the compact modal body independently; pointer on exposed scrim dismissed and restored page state. |
| Standard side | Measured320×400px with square docked corners, no scrim/lock. Main action remained usable; edited Workspace name remained after close/reopen. An open-during-close sequence completed without stale closure. |
| Detached standard side | Measured320×368px in a 400px stage, 16px margins/corners; primary content remains interactive. |
| Modal side | Measured360×980px at x969 on desktop. Initial Close Project filters focus. Tab skipped the disabled action to Apply filters. Apply and Escape dismissed/restored opener. |
| RTL modal side | Measured360px at x0, corners0/16/16/0. Entry sampled, then Escape restored opener. |
| Generic Sheet | Pointer-open and Escape for Start, End, Top and Bottom. Each settled with transform none and preserved draft. Top is explicitly an extension. |
| Actual Reduce Motion | macOS preference temporarily ON under prior user authorization; browser matchMedia true. First shown sample at 33ms already phaseopen/transformnone/opacity1. Expansion and Escape worked. Original OFF restored. Only bottom-sheet reduced state directly sampled. |
| Copy controls | All five returned exact visible examples: 609,706,583,528,441 characters. All 45 exported usage examples compile independently. |

Observed during development and corrected: Rust moved-ID compilation error;
docked standard side inherited modal corners; hidden initial standard panels
need not start invisible animations. Drag-release sync must distinguish a resting `none` transform from a fresh opening, so releasing an expanded sheet does not restart entry. Verification reloaded the successful final
build after the corrections. The catalogue also now exposes all three new pages.

## Build and evidence

Clean live Dioxus preview build passed (22.95s), followed by successful updates
including 7.39s/6.98s builds. Locked offline Wasm check passed (7.69s). Independent
source export with all 45 examples passed (4.53s), with one existing unused ChipSize
import warning. JavaScript syntax and whitespace checks passed. Final checks are
recorded in the completion note below.

[Raw desktop and compact samples](surfaces-samples.json),
[compact bottom sheet](bottom-sheet-compact.png),
[desktop side sheet](side-sheet-desktop.png). External sampler times include tool
latency and do not assert DOM event timestamps. Screenshots show resting layout,
not animation timing. Early frames collected before a live reload remain in the
raw dataset; `finalDocked`, `compactEntry`, `liveDragUp`, `dragCollapse`,
`dragDismiss`, `reducedEntry`, `rtlEntry` and copy results describe later checks.

## Gaps and extension scope

Android remains paused: no touch-scroll arbitration, mobile pointer cancellation,
on-screen keyboard, native Dioxus package or predictive back result is claimed.
Audible VoiceOver, enlarged text, forced colors, alternate browsers, side-sheet
reduced motion and preference changes during animation remain untested.
Nested/concurrent modal scroll-lock stacks, additional arbitrary snap points,
velocity-based fling, whole-surface swipe-to-dismiss and nested drawer stacking
from upstream are not ported. Consumer-provided layout owns standard pane size,
responsive adaptation and unsaved-data confirmation. Reusable opening/closing
supports retargeting, but broader rapid interruption/unmount cases need checks.
Do not treat this core family as exhaustive Base UI Drawer feature parity.

## Reproduce

Run `./scripts/dev.sh`, wait for a successful build and reload. Open Content →
Bottom sheets. Exercise pointer, Space/Enter, both Tab boundaries, Escape, scrim,
handle drags and body scrolling. Sample actual input while resizing between
partial/expanded heights. Repeat at 390×844 and reset viewport. Open Side sheets,
use primary controls while docked, edit/close/reopen, toggle detached and try modal
LTR/RTL/disabled control flow. Check all four Sheet placements and five clipboard
examples. Repeat with actual reduced motion and restore its original setting.

## Final completion checks

After the resting-transform correction, live rebuild passed (7.77s), the browser
was reloaded, and the compact drag sequence was repeated from settled-open state.
`finalDragUp` ended at 772px height and `finalDragDown` ended at 422px. The top edge
never travelled below y422px during either sequence: it did not restart entry.
Earlier `liveDragUp`/`dragCollapse` frames retain the pre-fix defect for diagnosis.
The updated independent export with all 45 examples passed again. Five clipboard
checks remain exact; API snippets were unchanged by the runtime correction.
Reduce Motion was restored OFF and the temporary viewport reset. Preview remains
running. Android remains paused and broader runtime gaps listed above remain.
