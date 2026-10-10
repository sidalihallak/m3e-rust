# Plain tooltip verification — 2026-10-10

## Scope and sources

`Tooltip` in `src/components/tooltip.rs`, with `anchored.rs`/`anchored.js` and `assets/help.css`. A supplementary description for an existing named control.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[overview](https://m3.material.io/components/tooltips/overview) · [specs](https://m3.material.io/components/tooltips/specs) · [guidelines](https://m3.material.io/components/tooltips/guidelines) · [accessibility](https://m3.material.io/components/tooltips/accessibility)

[Actual upstream Tooltip source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/tooltip.tsx) · [Base UI tooltip behavior](https://base-ui.com/react/components/tooltip)

## Reference contract and implementation

| Property | Implemented reference value |
| --- | --- |
| Appearance | Inverse surface/on-inverse-surface; 4px corners; 12/16px text |
| Size/insets | At least 24px; 4px vertical and 8px horizontal |
| Placement | Top/center, 4px gap; collisions may flip or shift |
| Mouse entry | Upstream provider 500ms; adjacent warm tooltips open immediately |
| Mouse leave | Material 1500ms, deliberately longer than Base UI's default |
| Focus | Immediate visual open; accessible name remains on the control |
| Motion | Popup spring, .85→1, opacity 0→1; see shared measured contract |

One plain tooltip is visible at a time. Escape and activation dismiss it;
ordinary pointer clicks do not open it. The runtime merges its ID into the
anchor's `aria-describedby`, retains earlier descriptions and cleans up its
listeners on unmount. Supply brief text, not essential information or actions.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Mouse enter using a native drag released over the icon | Delayed open observed; .85 intermediate scale and fade sampled |
| Mouse leave | Remained visible, then accelerated exit and closed |
| Tab from Save to Information | Information tooltip opened; only one plain tooltip visible |
| Escape | Closed, focus retained on Information |
| Pointer click on Save | Initially exposed a focus-origin bug; corrected and rechecked: no tooltip |
| Resting appearance | 24px high, 4px radius, 4/8px padding, 12/16px text measured |
| OS Reduce Motion ON | First visible sample settled at opacity 1 / transform none |

The hover/leave sample clocks include the native drag and AX observation delay.
The 500ms/1500ms values come from inspected references and implementation, not
an exact reconstructed input-event timestamp.

## Differences and remaining scope

Long-press open after 500ms, >10px movement cancellation and consumed-click
suppression are implemented but unverified on Android. The provider API is
translated into an internal shared warm period (400ms), not a React context.
Keyboard hints and arbitrary interactive tooltip contents are outside this
plain-text API. Wrap long or essential content in another appropriate surface.


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
