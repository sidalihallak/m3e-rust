# Accessibility check — 2026-10-10

## Scope and sources

Desktop Dioxus web preview in the Codex in-app browser. This is a targeted audit
of the existing kit and the connected-group pilot, not a WCAG certification.
Android remains paused. Relevant component source references and earlier
interaction evidence remain in each component's verification report.

Read the rendered official overview, specs, guidelines and accessibility pages
for [tabs](https://m3.material.io/components/tabs/overview),
[sliders](https://m3.material.io/components/sliders/overview),
[badges](https://m3.material.io/components/badges/overview), and
[button groups](https://m3.material.io/components/button-groups/overview), including
their `/specs`, `/guidelines` and `/accessibility` sections. See the concise
[reference observations](accessibility-reference-notes.json).

Inspected actual upstream `tabs.tsx`, `slider.tsx`, `badge.tsx`, `button-group.tsx`,
`toggle.tsx` and shared ripple/styles at shadcn-m3e revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`.

Web semantics also follow the [WAI-ARIA tabs pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tabs/),
[button pattern](https://www.w3.org/WAI/ARIA/apg/patterns/button/) and
[slider pattern](https://www.w3.org/WAI/ARIA/apg/patterns/slider/).
Text contrast calculations use [WCAG relative luminance](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html).

## Findings and corrections

1. Text-only tab panels lacked a tab stop. Tab skipped directly to the next tab
   list. Gallery panels and the copyable example now have `tabindex="0"`; the
   gallery includes a visible panel focus outline. Consumers must include a
   focusable panel when it has no focusable content.
2. Tab arrows used left-to-right index order inside RTL ancestors. They now
   read the rendered direction, mirror Left/Right, skip disabled tabs and retain
   automatic activation. Home/End use the first/last enabled item.
3. CSS-rotated vertical native sliders did not declare orientation. Both handles
   now expose `aria-orientation="vertical"`; horizontal inputs explicitly expose
   horizontal orientation. The copied range example now names its price handles.
4. Badge live content only contained the visual number; notification context was
   an accessible-name attribute. Numeric badges now expose complete text in a
   polite, atomic status region and hide the duplicate visual number from the
   accessibility tree. Optional `aria_label` supports application context and
   localization. The default text remains notification-oriented.
5. The gallery mode button's decorative glyph polluted its name. It is now
   hidden from assistive technology.

## Actual checks

| Check | Observed result |
| --- | --- |
| Names | Initial visible button/input/textarea/progress scan found no unnamed controls; native AX tree exposed names and selection states |
| Relationships | No duplicate IDs or missing `aria-controls`, `aria-describedby` or `aria-labelledby` targets in sampled DOM |
| Switch / mixed checkbox | Space changed Notifications to off and mixed Select all to checked |
| Native radio | Right from Medium focused and selected Large |
| Native slider | Arrow adjustment worked; vertical Level and both range handles expose vertical orientation |
| Tabs | Right selected/focused Hotels; after correction Tab reached trip-panel-0; RTL Left from One selected/focused Two |
| Menu | Down opened Actions and focused Favourite; Escape closed and returned focus to Actions |
| Input chip | Delete removed Design review and moved focus to Medium chip |
| Field error | Email exposed invalid=true and the linked description Enter a valid email address; Reference remained read-only |
| Live badge | Real Add notification click changed visual 3 to 4; accessible snapshot exposed 4 unread messages in Inbox without duplicate visual text |
| Color roles | All 138 sampled semantic text pairs passed 4.5:1 across Violet/Teal/Rose × light/dark; lowest ratio 4.9844:1 |
| Preview integrity | One app shell; no captured console errors |

The contrast matrix covers normal semantic text roles against relevant container
roles. It does not establish every rendered hover overlay, focus ring, decorative
outline, media background, disabled state or arbitrary custom seed. Disabled
controls are outside the enabled text contrast check.

## Actual OS reduced motion

With explicit user authorization, macOS Reduce Motion was changed from off to on
and restored to off. VoiceOver was also changed from off to on and restored to
off. VoiceOver caption-panel configuration was inspected but not changed.

`matchMedia('(prefers-reduced-motion: reduce)').matches` was observed true in the
live preview. Loading and circular indicator content stayed stable; sampled CSS
spinner/bar animation play states were paused. Accepted Button transition and
ripple CSS durations became 0.01ms. Turning the preference off resumed loading
shape updates. A second check toggled the preference while indicators were
already running: different shapes before, stable shapes while enabled, different
shapes again after restoring off. Connected-group selection still worked with
its transition duration reduced to 0.01ms.

This closes the previous actual-OS-preference gap for these sampled animations.
It does not certify every animation, a held touch gesture, or every possible
preference change during an in-flight press.

## Screen-reader limitation

VoiceOver was actually enabled, but spoken/caption output could not be reliably
captured. Native access to the Codex app was disallowed by the computer-use tool;
Chrome's localhost showed a different app; an attempt to inspect VoiceOver's
own output timed out. A temporary Chrome reference tab was closed. Therefore,
accessible-tree checks and semantic improvements are verified, while actual
VoiceOver announcement wording/order, live-region delivery and full screen-reader
navigation remain unverified. NVDA/TalkBack are also untested.

## Build and reproducibility

Rust/Wasm checks passed; successful Dioxus serve builds were observed before
reload and measurement. The independent source export and all 20 usage examples
compile for `wasm32-unknown-unknown`. See the connected-group report for the final
build timings. No parallel standalone `dx build` was used.

Raw evidence: [accessibility-samples.json](accessibility-samples.json). Repeated
indicator DOM snapshots are stored as SHA-256 hashes to keep the file compact;
equal hashes establish stable DOM, while CSS animation state was checked separately.
To reproduce, run `./scripts/dev.sh`, wait for a successful build and reload.
Exercise the keyboard actions above, use the mode/seed controls for all six
schemes, and temporarily toggle Accessibility → Motion → Reduce Motion while
progress runs. Restore the original preference afterward. The live badge example
and RTL tab example are included in the gallery.

Remaining: audible screen-reader checks, zoom/text enlargement and forced colors,
full RTL behavior outside the sampled components, custom-seed/state contrast,
and Android/touch verification.
