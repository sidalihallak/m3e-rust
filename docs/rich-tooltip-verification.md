# Rich tooltip verification — 2026-10-10

## Scope and sources

Persistent `RichTooltip` in `tooltip.rs`; it opens through explicit controlled state and accepts body content and up to two caller-supplied actions.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[overview](https://m3.material.io/components/tooltips/overview) · [specs](https://m3.material.io/components/tooltips/specs) · [guidelines](https://m3.material.io/components/tooltips/guidelines) · [accessibility](https://m3.material.io/components/tooltips/accessibility)

[Upstream plain-tooltip source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/tooltip.tsx) · [Base UI nonmodal popover](https://base-ui.com/react/components/popover)

## Reference contract and implementation

Material distinguishes plain and rich guidance and permits persistent rich
help opened by explicit activation. The current upstream file exposes the plain
variant; this rich variant is a Material-guided addition.

Rich appearance: 288px default width, surface-container/on-surface-variant,
12px corners, 12px top / 16px horizontal / 8px bottom padding, 16/24px headline
and 14/20px body. Actions use the existing text action style. It uses the shared
popup spring. Because its content can be interactive, it is a named **nonmodal
dialog**, rather than putting buttons inside an ARIA tooltip. It never traps
focus. Keep essential information on the main page.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Pointer open | Named “Share with your team” surface opened; Got it received focus |
| Escape from action | Closed and restored How sharing works trigger focus |
| Motion | Intermediate scale .87684 and opacity .256127 observed; settled at 288×144px |
| OS Reduce Motion ON | Visible frames already at opacity 1, transform none |
| Copy | Exact 619-character snippet copied |


## Differences and remaining scope

This API delivers the persistent rich variant. Transient rich hover/focus help
and enforcement of the two-action content limit are not included; the caller
must keep the action count and content suitable. It shares the nonmodal Tab and
outside-dismissal runtime with Popover; those paths were directly tested on
Popover, not separately for every rich-help action combination.


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
