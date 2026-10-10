# Connected button group verification — 2026-10-10

## Scope and files

A horizontal, controlled Material 3 Expressive connected selection group for
Dioxus web. Source: `src/components/button_group.rs`, `assets/button-group.css`,
shared `ripple.rs`/`motion.rs`/`ripple.css`, and icon source/CSS. Gallery usage is
copyable Rust. `ConnectedButtonItem` supports labels, icons, icon-only options and
native disabled buttons. `ConnectedButtonGroup` supports single/multiple,
optional/required selection, group disabled state, five sizes, round/square
shapes and filled/tonal/outlined/elevated styles.

Standard groups, split buttons and vertical groups are separate future ports.

## Reference contract

Read all four rendered official pages on 2026-10-10:
[overview](https://m3.material.io/components/button-groups/overview),
[specs](https://m3.material.io/components/button-groups/specs),
[guidelines](https://m3.material.io/components/button-groups/guidelines),
[accessibility](https://m3.material.io/components/button-groups/accessibility).

Inspected [upstream button-group.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/button-group.tsx),
`toggle.tsx`, `button.tsx`, `split-button.tsx`, shared ripple and generated motion
CSS at that revision. Also checked the official Compose
[connected small tokens](https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/ConnectedButtonGroupSmallTokens.kt)
and [ButtonGroup defaults](https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ButtonGroup.kt).

| Property | Implementation and attribution |
| --- | --- |
| Gap | 2px at every size, official connected-group specs |
| Height XS/S/M/L/XL | 32/40/56/96/136px, existing expressive button sizes |
| Unselected inner corners | 4/8/8/16/20px, official measurements |
| Outer round corners | Half visual height; square outer corners use the inner-corner scale |
| Selected shape | All corners half visual height, official Compose checked-shape token |
| Pressed inner corners | 4px; official small token and upstream connected-group extra-small press shape. Other-size press-token parity is not independently established |
| Pressed outer corners | Retain the group's round/square end shape, as upstream and official small leading/trailing press shapes do |
| Label typography | Upstream label-large 14/20, title-medium 16/24, headline-small 24/32, headline-large 32/40; weights 500/500/400/400. Inter matches the existing kit; font-family parity remains a gap |
| Targets | Width ≥48px; XS/S reserve and extend vertically to 48px without widening the 2px gap |
| Layout | Label groups fill the parent; icon-only options keep default widths with 48px minimum. One line, no wrap |
| Motion | 240ms DefaultEffects generated spring; shared 225ms minimum press, 450ms growth, 105ms fade-in, 375ms fade-out, opacity .10 |
| Keyboard | Tab reaches enabled buttons; Left/Right moves focus, RTL mirrored; Home/End move to edges; Space/Enter activate |
| Semantics | Native type=button, aria-pressed; nonfocusable role=group. Labels remain stable on selection |

### Declared choices

The upstream container uses a fixed 8px inner corner and content width. It also
overrides toggle shape at the ends. This port follows current official size
measurements, flexible label-group layout and fully round selected shape instead.
The existing classic segmented buttons remain available. Connected selection uses
native toggle buttons and manual activation on arrows; it does not borrow the
classic segmented radio group's automatic selection behavior.

Filled/elevated/tonal/outlined selected colors follow actual upstream toggle
roles. Selecting retains the supplied icon; an optional check icon is not forced.
Required selection rejects removing the last selected item. Initialize controlled
state with at least one valid item when required; the component does not silently
write the caller's state. Out-of-range indices and duplicates are filtered for
rendering; single selection renders the first valid index after sorting.

## Runtime desktop results

- All five heights and corner scales measured correctly; icon-only widths were
  48/48/56/96/136px. Gaps measured 2px.
- Initial Tab landed on Day, not the container. Tab from Day reached Week.
  Arrows moved focus without selecting. Enter selected Month. End focused Month.
- Space on required Week retained Week. Multiple selection added Info while
  preserving Save/Add, then Enter removed Save alone. Optional RTL Weekly could
  clear its selection.
- RTL Left moved Daily → Weekly and wrapped Weekly → Daily, skipping disabled
  Monthly. Fully disabled List/Grid use native disabled semantics.
- A real click 6px above the XS Details visual button selected it, proving its
  transparent native target extension responds.
- 35 samples captured a real quick click: pressed inner radius reached 4.00684px
  from 20px and returned; ripple opacity reached .10. All button widths remained
  exactly 108.3359375 / 120.15625 / 125.5078125px throughout. No scale shrink.
- Repeated clicks restarted visible feedback. A real drag from Day to outside
  the group did not select Day, and feedback released.
- Actual macOS Reduce Motion made transitions 0.01ms; Enter still selected Day.
  Original system preferences were restored. See [accessibility results](accessibility-verification.md).

These are sampled desktop results, not frame-perfect pixel or screen-reader
certification. A separate sustained held-input measurement, live item-list
replacement, empty/invalid caller state, font changes, font-family parity and Android touch remain
untested. Do not claim those from shared-helper source alone.

## Build, evidence and reproduction

Final successful Dioxus serve build: **7.22s**, followed by reload and final
geometry/motion sampling. Final Rust/Wasm check passed (**4.37s**). The independent
source export plus all **20** usage examples passed a locked offline Wasm check
(**2.95s**). The new group's Copy Rust code action displayed Copied. Actual
end-to-end paste remains the clipboard-adapter gap documented in remaining work.

Evidence: [raw samples](button-group-samples.json),
[preview](button-group-preview.png), [copy guide](copy-components.md).

Run `./scripts/dev.sh`, wait for the successful build and reload. Find Connected
button groups after Segmented buttons. Repeat Tab/arrows/Home/End/Space/Enter,
required and optional activation, and the XS target click. Sample computed corner
radii and ripple styles during the event; a resting screenshot cannot establish
motion fidelity. Include `button-group.css`, `ripple.css`, and `icon.css` when
copying, and retain the listed Rust helpers and Material color roles.
