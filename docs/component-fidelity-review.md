# Component fidelity review — 2026-10-09

## Current resolution

The R1–R13 implementation fixes and additional desktop changes are recorded in
[the fix report](component-fidelity-fixes.md). The findings below are the original
pre-fix review; see the fix report for current measured scope and remaining gaps.

## Scope and result

Reviewed the added Dioxus components at `ce26469`, their CSS, gallery usage and existing verification reports. The pre-existing local change to `assets/tailwind.css` was included in the running preview and left untouched. No component implementation was changed by this review.

**The kit is not yet verified for high Material 3 interaction fidelity.** Many base dimensions and semantic color roles match the sources, but the findings below include reproducible keyboard failures, invisible press feedback and insufficient interactive targets. These findings concern the new components; the accepted Button motion baseline was not re-certified here.

## Sources

Read the Overview, Specs, Guidelines and Accessibility tabs of each of the official component pages linked in the coverage table. The official site was accessible through the browser during this review. Some older reports describe it as inaccessible; those reports are historical evidence, not current source readings.

Inspected actual source from:

- [shadcn-m3e components at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/tree/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components). Earlier reports used `8f1b3fb`; this review uses the current source, so revisions must be distinguished.
- [Material Web token source at 47adb65](https://github.com/material-components/material-web/tree/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass).
- The repository's [component protocol](component-development.md) and [accepted Button contract](button-motion-verification.md).

Where official guidance and upstream differ, the distinction is recorded below. Matching an upstream limitation does not establish compliance with the official guidance.

## Findings

Follow-up on 2026-10-09: the user also identified the focused text-field notch's residual stroke
and native numeric arrows in Price. Both were corrected in the component CSS and checked in the
desktop preview. See the [current text-field report](text-field-verification.md) and
[notch samples](text-field-notch-samples.json). R9's surface focus issue remains open.

### R1 — P1: Keyboard navigation selects disabled tabs

`src/components/tabs.rs:91–107` computes the next index without checking `TabItem.disabled`.

**Reproduced:** select Specs in Product, then press ArrowRight. Reviews becomes selected even though it is disabled. Both enabled tabs have `tabindex=-1`; the disabled Reviews tab has `tabindex=0`, and focus remains on Specs. The gallery changes to the disabled panel. End has the same code path problem when the last tab is disabled.

Skip disabled items for arrows, Home and End, and retain an enabled tab in the tab order. [Material tab accessibility](https://m3.material.io/components/tabs/accessibility) describes navigation through available interactive destinations. Upstream delegates tab navigation to Base UI.

### R2 — P1: Range handles can invert the selected interval

`src/components/slider.rs:107–139` gives both inputs the full `min..max` bounds and emits each new value without constraining it against the other handle.

**Reproduced:** Price starts at 20–70. Focus Price start and press End. It becomes **100–70**. After the CSS transition settles, the active track is **0px** wide. The consumer in the supplied usage example accepts this invalid pair unchanged.

Enforce ordered range values and expose corresponding accessible bounds. [Material slider guidelines](https://m3.material.io/components/sliders/guidelines) define the handles as the interval's minimum and maximum. The upstream uses a coordinated multi-thumb Base UI slider rather than two independent inputs.

### R3 — P2: Several visible sizes are also undersized interactive targets

The implementation conflates visual dimensions with hit area dimensions. Browser measurements, corroborated by the CSS:

| Control | Measured target | Relevant source |
| --- | --- | --- |
| IconButton XS / S | 32×32 / 40×40px, no extended pseudo-element | `assets/icon-button.css:32` |
| Checkbox | 40×40px | `assets/checkbox.css:18` |
| Radio | 40×40px native input | `assets/radio.css:29` |
| Chip main action | 30px high inside a 32px bordered chip | `assets/chip.css` |
| Input chip remove | 30×30px | `assets/chip.css` |
| Segmented item | 38px high inside a 40px bordered set | `assets/segmented-button.css` |
| Label-only tab | 40px high inside a 48px bar | `assets/tabs.css` |
| Small FAB | 40×40px | `assets/fab.css` |
| Range slider | Only the transparent 4×44px native thumbs accept pointer events | `assets/slider.css:114` |

[Icon buttons explicitly require 48×48dp targets for XS/S](https://m3.material.io/components/icon-buttons/specs). [Checkbox specs](https://m3.material.io/components/checkbox/specs), [radio specs](https://m3.material.io/components/radio-button/specs), [segmented button specs](https://m3.material.io/components/segmented-buttons/specs), [chip accessibility](https://m3.material.io/components/chips/accessibility) and [tab accessibility](https://m3.material.io/components/tabs/accessibility) also specify or recommend 48dp targets. The upstream checkbox and radio deliberately extend targets with pseudo-elements. Its tab trigger is also only 40px high, so copying that value alone does not satisfy current official guidance.

Keep the specified visual containers, but expand actual hit areas without overlapping neighboring controls. The Switch already has an extended hit area; its 32px visual track is not itself a defect. No Android touch result is claimed from these desktop measurements.

### R4 — P2: Quick click and Enter feedback are lost on IconButton and FAB

`src/components/icon_button.rs:118–137` clears pressed state on pointer-up and handles only Space in its key handlers. There is no click feedback fallback or ripple. `src/components/fab.rs:177–225` likewise clears the held flag immediately, including immediately after starting the Enter ripple. Its asynchronously measured ripple is often rendered after the pressed state has ended.

**Measured in the browser:** for a 700ms observation window spanning each real activation:

| Component/input | Observed result |
| --- | --- |
| XS IconButton / Enter | All 26 samples: resting radius 16px; pressed=false |
| XS IconButton / quick pointer click | All 26 samples: radius 16px; pressed=false |
| Standard FAB / Enter | All 27 samples: ripple exists, opacity 0; pressed=false |
| Standard FAB / quick pointer click | All 26 samples: ripple exists, opacity 0; pressed=false |

See [motion samples](component-fidelity-review-samples.json). These are computed-style polls approximately 25–39ms apart, not frame-perfect event traces. They cannot exclude an instantaneous state between polls; they do show that the intended sustained visible feedback did not occur for these activations.

Use the accepted Button press/release lifecycle, with appropriate component tokens. Upstream Button/icon sizes and FAB include Ripple. [Material icon-button specs](https://m3.material.io/components/icon-buttons/specs) show press shape changes. Other new components (Chip, Tabs, Radio, Checkbox, Switch, interactive Card and FabMenu) also omit the upstream expanding ripple; their selection transitions or fading state layers should not be reported as ripple parity. [Cards explicitly require click/touch ripple](https://m3.material.io/components/cards/accessibility).

### R5 — P2: Selected chips and labeled segments keep the wrong leading icon

`src/components/segmented_button.rs:48–51` and `src/components/chip.rs:67–69` prioritize a supplied icon over the selected checkmark. The selected Favourite and Add segments in the gallery retain their original icons.

[Material segmented-button guidelines](https://m3.material.io/components/segmented-buttons/guidelines) require an icon to become a checkmark when an icon-and-label segment is selected. [Filter-chip guidance](https://m3.material.io/components/chips/guidelines) uses a leading checkmark for selection. The upstream FilterChip hides the original icon and displays the check while selected. Preserve the supplied icon for the unselected state and use the selected checkmark in these labeled controls.

### R6 — P2: Filled toggle has default-action colors while unselected

`assets/icon-button.css:95–99` applies primary/on-primary to every filled button. There is no filled-toggle-unselected override.

**Measured:** Filled toggle has `aria-pressed=false` but background `rgb(101,85,143)` (primary) and a white icon. The current [official icon-button specs](https://m3.material.io/components/icon-buttons/specs) distinguish a default filled action from an unselected filled toggle: the latter uses **surface-container / on-surface-variant**, then primary / on-primary when selected. The current `_md-comp-icon-button-filled.scss` also specifies surface-container. Older deprecated filled-icon-button tokens use surface-container-highest; do not mix token generations.

### R7 — P2: Input-chip keyboard removal is missing

`src/components/chip.rs:94–117` supports Space press state and a separate remove click but never handles Backspace or Delete.

**Reproduced:** focus Design review and press Delete; the chip remains. [Material chip accessibility](https://m3.material.io/components/chips/accessibility) assigns these keys to removing the focused input chip. Call `onremove` from the appropriate enabled input-chip keyboard path and preserve focus after removal. Upstream's current InputChip also omits this behavior; official compliance needs an additional implementation.

### R8 — P2: Opening a FAB menu does not establish usable forward focus order

`src/components/fab_menu.rs:118–145` renders menu items before the trigger in DOM order. Opening leaves focus on the trigger, and Tab then moves past that menu's items. No arrow navigation or first-item focus behavior is supplied despite `role=menu`/`menuitem`.

**Reproduced with both gallery menus open:** open Actions with Enter, then Tab. Focus goes to Favourite in the *other* menu, not the first item of the opened menu. Recorded IDs differ (`m3-fab-menu-1` → `m3-fab-menu-0`). [FAB menu accessibility](https://m3.material.io/components/fab-menu/accessibility) describes navigation from the close button through the items and requires web menu accessibility behavior. Implement a coherent menu focus model and return focus on dismissal/selection.

Related visual difference: `.m3-fab-menu__items` stays in normal flow while hidden, so a three-item closed menu still reserves **240px** total height. The upstream positions the item stack absolutely above the trigger. This affects placement when used as a floating action in a real project. Its state-specific elevation also needs comparison: current Material tokens set item focus/pressed elevation 3 and hover 4; local items remain shadowless. Upstream uses item elevation 3 at rest whereas the common Material token says 0 — document the chosen reference explicitly.

### R9 — P2: Text-field surface is 56px, but most of it does not focus the input

`assets/text-field.css:118–127` makes the input 24px high with no padding; the surrounding row/container has no focus forwarding. The label has `pointer-events:none`.

**Reproduced:** click the floated Email label through an actual pointer click. The Email input does not focus. The rendered container is 56px high, but the native input is only 24px high. Clicking the center of the Name field does focus it, which does not establish full target coverage.

[Material text-field specs](https://m3.material.io/components/text-fields/specs) specify a 56dp target. Expand the interactive input area or forward appropriate surface clicks to it. The upstream input includes vertical padding, unlike this implementation. Also track the already-documented once-only notch measurement when labels/fonts change; that case was not exercised here.

### R10 — P2: Reduced motion does not stop the wavy circular sweep

This is a **source-confirmed gap, not an OS-preference runtime test**. `src/components/progress.rs:313–353` rewrites the arc on every animation frame. `src/components/motion.rs` schedules frames without checking reduced motion. The media query in `assets/progress.css` only pauses CSS animations; it cannot stop those JavaScript path updates. Thus the sweep continues while the CSS rotation is paused.

Gate or stop the sweep for the preference and respond when it changes. LoadingIndicator already checks `matchMedia` in its loop. The upstream circular sweep currently has the same omission, so copying it does not meet the repository's reduced-motion requirement.

### R11 — P2: Indeterminate progress differs from the actual reference rendering

`src/components/progress.rs:51–65` renders moving linear bars but no inactive track. **Measured:** Flat indeterminate has a transparent background and only two bar children. Both upstream linear variants retain a full inactive track; the local `--linear-track` variable is not painted in indeterminate mode.

Separately, `src/components/progress.rs:158–165` uses a fixed 270° circular arc with a single 1.4s rotation. Upstream flat CircularProgress uses a two-half spinner with independent expansion/contraction and rotation. A spinning fixed arc is not motion parity. See [official progress guidance](https://m3.material.io/components/progress-indicators/guidelines).

The local linear track's surface-container-highest role matches the checked Material Web token, while current upstream uses secondary-container. This color difference needs a declared reference choice; it is not classified here as an unequivocal official token error.

### R12 — P2: Verification claims and copy instructions are stale or incomplete

README repeatedly calls components verified, but reports have known untested behavior and no current evidence for several added variants. `docs/progress-verification.md` and `src/main.rs:774` still say wavy circular is unimplemented; it is rendered now. The FAB report says branded/toolbar/menu are absent; the card report says media/headline slots are absent. LoadingIndicator has no dedicated component verification report.

The CSS copy comments for Progress and LoadingIndicator also omit `src/components/motion.rs`, whose `next_id`/`frame_loop` they use. LoadingIndicator additionally requires `loading_shapes.rs`; several components require Icon and generated icon data. Publish complete source/CSS/helper dependencies and verify a copied consumer example before marking the Rust/UI workflow complete. No isolated copy-consumer build was performed in this review.

### R13 — P2: Tick positions are wrong when step does not divide the range

Source analysis at `src/components/slider.rs:63–68`: with `min=0`, `max=100`, `step=30`, rounding the number of steps to 3 and dividing `i/n` puts ticks at 0%, 33.33%, 66.67%, 100%. Native valid values occur at 0, 30, 60, 90. Compute each tick from `min + i*step` and normalize that value. This configuration was not mounted in the gallery; the arithmetic mismatch follows directly from the implementation.

## Coverage and remaining scope

“No additional finding” means none identified in this review's scope, not full component certification.

| Component / official source | Upstream source inspected | Result / remaining scope |
| --- | --- | --- |
| Button | `button.tsx` | Existing accepted baseline retained; no new runtime certification |
| [Icon buttons](https://m3.material.io/components/icon-buttons/overview) | Icon sizes in `button.tsx` | R3/R4/R6; narrow/wide options and outlined/filled glyph toggles are incomplete |
| [Checkbox](https://m3.material.io/components/checkbox/overview) | `checkbox.tsx` | Space toggles the unchecked example; R3 and ripple gap |
| [Switch](https://m3.material.io/components/switch/overview) | `switch.tsx` | Space toggles Wi-Fi; extended target exists; ripple/drag parity not established |
| [Radio](https://m3.material.io/components/radio-button/overview) | `radio-group.tsx` | ArrowRight moves Medium → Large and selects it; R3 and ripple gap |
| [Chips](https://m3.material.io/components/chips/overview) | `chip.tsx` | R3/R5/R7; only base visual size implemented |
| [Segmented buttons](https://m3.material.io/components/segmented-buttons/overview) | No dedicated upstream file found in this revision; native inputs compared with official guidance | ArrowRight changes Week → Month; R3/R5; all-disabled outer outline also stays full opacity |
| [Sliders](https://m3.material.io/components/sliders/overview) | `slider.tsx` | R2/R3/R13; only XS 16px track implemented, centered/inset-icon/larger sizes absent; local value bubble is 28px minimum width versus current 48dp spec |
| [Tabs](https://m3.material.io/components/tabs/overview) | `tabs.tsx` | R1/R3; ripple and panel association API absent; current official primary indicator is fully rounded/inset, local and upstream use bottom-flush rounded-top form |
| [FABs](https://m3.material.io/components/floating-action-button/overview) / [extended FABs](https://m3.material.io/components/extended-fab/overview) | `fab.tsx` | R3/R4; geometry/roles largely token based; toggling/extended transitions not certified |
| [FAB menu](https://m3.material.io/components/fab-menu/overview) | `fab-menu.tsx` | R8; trigger close glyph swaps immediately rather than upstream's crossfade/rotation |
| [Progress indicators](https://m3.material.io/components/progress-indicators/overview) | `progress.tsx`, `circular-progress.tsx` | R10/R11; geometry follows upstream, but motion and dynamic determinate/indeterminate switching require verification |
| [Loading indicator](https://m3.material.io/components/loading-indicator/overview) | `loading-indicator.tsx` | 48px container, 38/48 shape scale and contained color roles align in source; seven-shape motion/frame matching and reduced motion not exercised |
| [Badges](https://m3.material.io/components/badges/overview) | `badge.tsx` NotificationBadge | 6px dot/16px pill, error/on-error, 999+ convention align; no additional functional finding; screen-reader announcements/context not tested |
| [Cards](https://m3.material.io/components/cards/overview) | `card.tsx` | Base 12px corners, roles and native interactive button are appropriate; expanding ripple absent; media/disabled visual comparison and drag behavior incomplete |
| [Divider](https://m3.material.io/components/divider/overview) | `separator.tsx` | 1px outline-variant line and separator orientation align; RTL start inset uses physical margin-left and needs correction/verification |
| [Text fields](https://m3.material.io/components/text-fields/overview) | `text-field.tsx` | Native inputs, label/support association and error roles present; R9; read-only API, changing-label notch and reduced-motion notch transition need work |
| Material Symbols | Generated icon paths and Icon component usage | Existing generated rounded glyph system retained; no exhaustive glyph audit |

## Build, platform and method

- `cargo check --locked --offline --target wasm32-unknown-unknown` with the isolated tools: **passed**.
- `dx build --platform web --locked --offline`: **passed**.
- Restarted `./scripts/dev.sh`: Dioxus reported a successful build. Opened a fresh local preview after an old tab had retained a connection-error page. Preview was left running at `http://127.0.0.1:8080/`.
- Runtime checks: desktop Codex in-app browser; real pointer clicks and keyboard input, DOM/accessibility inspection, computed sizes/styles and motion polling. Default light scheme in the gallery. No component code or synthetic event injection used to manufacture test states.
- Source review: CSS/native semantics, official four-section readings, pinned upstream files/tokens, current docs.
- Not tested: Android/emulator/touch, native Dioxus Android, assistive-technology announcements, full dark/seed contrast matrix, RTL, exhaustive states/variants, screenshot pixel comparison, OS reduced-motion preference, isolated copy-consumer build. Android work remains paused.

## Reproduction order

1. Run `./scripts/dev.sh`, wait for successful build, then reload/open the local page.
2. Product: click Specs, ArrowRight; inspect selected/disabled/tabindex and panel.
3. Price start: End; inspect values and active track after its transition settles.
4. Design review: Delete; verify the chip persists.
5. XS IconButton: Enter and quick click; poll computed radius for 700ms. Standard FAB: repeat and poll ripple opacity.
6. Inspect Filled toggle while unselected and compare primary with surface-container.
7. Open Actions with Enter; Tab and inspect which menu owns focus.
8. Click the floated Email label; inspect whether the Email input focused.
9. Check native Space/Arrow selection examples as recorded above.

Fix R1/R2 first, then shared hit areas/feedback and selection indicators. Re-run each affected component's protocol, update its individual report and replace blanket verification claims with the precise tested scope.
