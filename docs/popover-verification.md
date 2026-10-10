# Popover verification — 2026-10-10

## Scope and sources

Controlled `Popover` in `popover.rs`; interactive anchored content, title and optional description. Includes a form example.

The current official pages below were read as rendered pages on 2026-10-10.
Upstream components and shared motion styles were inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

[overview](https://m3.material.io/components/tooltips/overview) · [specs](https://m3.material.io/components/tooltips/specs) · [guidelines](https://m3.material.io/components/tooltips/guidelines) · [accessibility](https://m3.material.io/components/tooltips/accessibility)

[Actual upstream Popover source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/popover.tsx) · [Base UI behavior](https://base-ui.com/react/components/popover)

## Reference contract and implementation

Popover is an upstream application primitive, with no separate official M3
Popover page. Material tooltip guidance establishes the distinction between
supplementary help and interactive context; the visual contract comes from the
actual upstream surface.

288px width, 16px padding and content gap, 16px radius, surface-container,
on-surface-variant, elevation 2, body 14/20px and title 16/24px. Bottom/center,
4px offset by default. Placement is bounded by an 8px viewport gutter and may
flip. Anchor-relative origins follow inspected Base UI positioning.

A named `role=dialog` nonmodal surface uses native Popover API top-layer
placement. Focus enters its first eligible control. Escape restores the anchor;
outside activation dismisses; Tab exits in document order.

## Runtime checks

| Actual input/state | Result |
| --- | --- |
| Enter on View preferences | Opened, Display name focused |
| Tab through form | Text field → checkbox → Done → link outside; popup closed |
| Shift+Tab from first input | Closed and focused trigger |
| Escape | Closed and restored trigger focus |
| Outside pointer | Closed without stealing focus back |
| Collision-origin | Popup x=8; anchor center 125.59; local origin 117.59px / −4px |
| Geometry/screenshot | 288px wide; 16px radius/padding; 256px field outer width, 224px input content |
| Raised-surface notch | Display name rendered without clipped or painted-background notch |
| 390×844 | 288×280px popup, x=8; bottom 718.41 within viewport |
| OS Reduce Motion ON | Visible sampled states opacity 1 / transform none |
| Copy | Exact 683-character snippet and native paste workflow passed |


## Differences and remaining scope

No modal variant, virtual anchor, arrow, custom collision boundary or full
Floating UI API is claimed. Place the component after its trigger in DOM order
and use a stable anchor ID. Caller content owns validation and actions; include
each embedded component's CSS. Nested popup/modal combinations remain an
additional verification task.

Screenshots: [desktop form](help-popover-desktop.png), [compact form](help-popover-compact.png).

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
