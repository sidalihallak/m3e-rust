# Attachment verification —2026-10-12

## Scope and reference

Read rendered official [component catalogue](https://m3.material.io/components),
[accessibility principles](https://m3.material.io/foundations/overview/principles)
and [assistive technology](https://m3.material.io/foundations/overview/assistive-technology)
on2026-10-12. None of these application blocks has a dedicated official Material
component page/spec. Geometry comes from upstream; Material guidance informs
semantic roles, keyboard access, named controls, targets and readable text.
Existing button/icon-button/TextField reports cover reused primitive contracts.

Actual [upstream source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/attachment.tsx), revision
c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972, retrieved2026-10-12.

## Reference contract and implementation

Sources: attachment.rs, application-blocks.css and ApplicationBlocksGallery.
Upstream states idle/uploading/processing/error/done, sizes default/sm/xs,
horizontal/vertical layout, media/content/title/description/actions/trigger/group.
Local typed API implements those compositions. Default/small/xs gaps8/10/6px,
media40/32/28px, radii28/28/20px; horizontal min160px, vertical content120px.
Primary surface role: SurfaceContainerLow/OnSurface; media uses highest surface;
error uses ErrorContainer/Error. Image opacity .6 while pending,1 at idle/done.
Titles/descriptions ellipsize. The open trigger overlays the body, while separate
native icon actions stay above it. Native XS IconButton32px and its reserved48px
hit area replace upstream Button icon-xs. Focus-within uses3px Secondary.
Group gap12px, native horizontal scroll and snap-start.

Uploading/processing titles adapt shadcn4.21.0's shimmer:110-degree gradient,
spread3ch+40px,2s linear infinite, RTL reverse. Source:
https://unpkg.com/shadcn@4.21.0/dist/tailwind.css. Reduced motion stops shimmer and
restores solid text; forced colors also removes it. The gallery upload state is
local example state, not a real uploader. No file dialog/network upload is installed.

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

Token-resolution correction: source `m3e.css` overrides Tailwind radii.
`rounded-xl/2xl/3xl/lg` resolve to20/28/32/16px, not stock Tailwind
12/16/24/8px. The final CSS uses the inspected Material token values. Source:
https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css

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

Pointer open, Enter remove and Space retry produce independent named callback
results; disabled Assets.zip remove remains disabled. Uploading→Done clears
aria-busy and removes shimmer. Eight live samples move background-position from
75.83% to42.50% while computed duration is2s; this establishes live motion.
Actual OS Reduce Motion off→on makes animation none and text fill the same
rgb(29,27,32) as its solid text; off is restored.

Final default/small/xs corners28/28/20px and media20px match source token aliases.
Default media40px, small32px, xs28px; vertical card120px/media102px. Default
document237.63×66px; small184.30×47px; processing160×43px. At390px, error card
shrinks to277px; body375px has no horizontal overflow. Group277px wide with524px
content; ArrowRight settles at132px native snap, maximum247px. Edge fades follow
source min(12%,40px),96px reveal; masks measured in raw group samples.

Screenshots: attachments-desktop.png and attachments-390.png. Actual upload/file
selection/download/network, image-pending opacity permutations, touch swipe and
all action/size variants still need consuming-app/platform evidence.
