# Hover card verification — 2026-10-10

## Scope and sources

`HoverCard` in `hover_card.rs`: supplementary link preview, not an interactive form or required information.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[overview](https://m3.material.io/components/tooltips/overview) · [specs](https://m3.material.io/components/tooltips/specs) · [guidelines](https://m3.material.io/components/tooltips/guidelines) · [accessibility](https://m3.material.io/components/tooltips/accessibility)

[Actual upstream HoverCard source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/hover-card.tsx) · [Base UI PreviewCard](https://base-ui.com/react/components/preview-card)

## Reference contract and implementation

There is no dedicated M3 HoverCard page. This upstream addition takes its
visual surface from shadcn-m3e and its link semantics from Base UI PreviewCard.
288px width, 16px padding, 12px corners, surface-container/on-surface-variant,
elevation 2, 14/20px body. Bottom/center with 4px gap and 4px alignment offset.
Base UI defaults: mouse delay 600ms, close delay 300ms. Uses popup spring motion.

The link remains the accessible interface. The preview is `aria-hidden=true`,
its contents are inert, and its information is repeated at the linked
destination. Focus may reveal the visual enhancement, but does not add another
screen-reader destination. Use Popover for interactive content.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Mouse entry via a released native drag | Delayed open; scale .85 and opacity 0 sampled, then settled at 288×96px |
| Pointer leaves | Delay, exit interpolation and closure observed |
| Tab to Material project | Link focused, visual preview opened |
| Escape | Preview closed; link retained focus |
| Semantics | aria-hidden true, inert content, actual link to full equivalent page content |
| Geometry | 288px width and 12px radius measured |
| Copy | Exact 452-character example copied |


## Differences and remaining scope

Touch/mobile preview behavior is not verified. The hover delay is a Base UI
value; it is not a universal Material timing. The caller is responsible for
repeating useful content at the destination. OS Reduce Motion coverage sampled
the same shared popup runtime on Tooltip/Popover/RichTooltip; HoverCard did not
receive a separate reduced-motion stream.


## Builds, evidence and reproduction

The live Dioxus Wasm build succeeds; source export and all **35** independent
usage examples compile for `wasm32-unknown-unknown`. The existing ChipSize
unused-import warning remains. See [family build and motion details](help-selection-verification.md).

[Raw browser samples](help-selection-samples.json) include actual input, focus,
expanded/selected states, computed geometry and motion. Each sample stream uses
browser round trips; its clock is relative to the sampling request, **not** the
DOM input event. Do not interpret RPC latency as a component animation duration.

Run `./scripts/dev.sh`, wait for successful completion, reload and check one
`.app-shell`. Open the family anchors, perform the matrix above through real
pointer/keyboard input, and sample computed styles concurrently. Read the
[copy guide](copy-components.md) for dependencies and controlled state.

Desktop IAB and a desktop browser at **390×844** are the verified platforms.
Android touch/on-screen keyboard and native Dioxus Android remain paused.
Audible screen-reader announcements, forced colors, text enlargement,
transformed/scrolled ancestors, dynamic anchor replacement and unmount stress
remain unverified. A separate consuming-app runtime is also still open.
