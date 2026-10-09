# Primary tab indicator correction — 2026-10-09

## Scope and source reading

Corrected the primary indicator in `assets/tabs.css` and its geometry observer
in `src/components/tabs.rs`. Read the current official
[overview](https://m3.material.io/components/tabs/overview),
[specs](https://m3.material.io/components/tabs/specs),
[guidelines](https://m3.material.io/components/tabs/guidelines), and
[accessibility](https://m3.material.io/components/tabs/accessibility) in the
browser on this date. Inspected the loaded **Primary tab active indicator
measurements** diagram, not just its caption.

Sources inspected alongside the site:

- [Actual shadcn-m3e tabs.tsx at c37c0d2](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/tabs.tsx)
- [Generated upstream motion tokens](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.generated.css)
- [Material Web primary tab tokens at 47adb65](https://github.com/material-components/material-web/blob/47adb655bd7a88c4d62e8faac2873084eed555dc/tokens/versions/latest/sass/_md-comp-primary-navigation-tab.scss)

## Diagnosis and reference choice

The previous follow-up misread the 2dp inset as a gap **above the divider**.
It also kept the full label width and rounded all four corners. Actual prior
measurements: 2px divider gap, 45px indicator over a 45.281px Flights label,
and rounded bottom corners.

The official diagram's Search example insets the indicator horizontally from
the label edges; its All example demonstrates the 24dp minimum. The width is
therefore max(24px, label width − 4px). The line meets the divider. The prose
caption calls the corners fully rounded, while the same page's explicit shape
table, its drawn profile and Material Web tokens specify **3,3,0,0**. This fix
follows that explicit table and diagram: rounded top corners and square bottom
corners. The earlier documentation's claimed official 2px bottom inset was wrong.

Upstream uses a 16px inset from the **tab container**, so that width rule is
not copied. Current official label-relative geometry takes precedence. Motion
uses the actual upstream 440ms expressive DefaultSpatial curve; the previous
360ms FastSpatial attribution was inaccurate.

## Implemented and measured contract

| Property / check | Actual result |
| --- | --- |
| Primary height | 3px |
| Flights label / indicator | 45.28125 / 41.28125px; 2px each horizontal inset |
| Hotels label / indicator | 43.3984375 / 39.3984375px; 2px each inset |
| Cars label / indicator | 31.578125 / 27.578125px; 2px each inset |
| Stacked-icon Details | 46.40625 / 42.40625px; 2px each inset |
| Stacked-icon Favourites | 70.78125 / 66.78125px; 2px each inset |
| Short Add label | 27.4296875px label; centered 24px indicator (minimum overrides full inset) |
| Indicator center error | 0px for above full-inset examples; ≤0.00390625px at minimum |
| Vertical placement | Indicator bottom meets divider top: 0px gap |
| Corners | 3px 3px 0px 0px |
| Secondary | 2px high; full 112.5px selected tab width; square corners; flush divider |
| Motion | 440ms DefaultSpatial; 13 distinct positions in actual pointer-click sampling |
| Minimum during Add animation | 24px throughout samples; explicit min-width prevents spring undershoot |
| Keyboard | Hotels ArrowRight → Cars; Add Home → Favourites; Specs ArrowRight → Overview; Reviews stays disabled/unselected/tabindex −1 |
| Dark scheme | Same geometry and alignment |
| Initial load | 16 polling samples spanning mount; every observed visible/ready indicator had ≤0.00390625px center error |

Measurements retain fractional CSS pixels rather than rounded offsetLeft and
offsetWidth. ResizeObserver tracks tabs and labels; character/child changes
trigger new measurements. Font readiness is handled. Motion is enabled only
after the initial coordinates are painted, preventing an initial slide from
x=0. Observers and the pending frame are cleaned up on unmount.

## Build, evidence, copyability and gaps

- Locked offline Wasm check: passed.
- Updated tab sources in the independent exported kit: all 19 existing usage
  examples passed the locked offline Wasm check (1.85s).
- ./scripts/dev.sh reported a fresh successful 12.57s build; subsequent component
  rebuild passed in 6.52s. Reloaded the actual preview before measurement.
- [Raw measurements and motion polls](tabs-indicator-verification-samples.json).
  Poll times are not exact event/frame timestamps. The earlier initial-slide
  observation is retained separately from the final mount samples.
- [Corrected desktop screenshot](tabs-indicator-corrected.png).
- Existing Rust usage and [copy dependencies](copy-components.md) remain valid;
  no public API changed in this fix.
- Desktop pointer, keyboard and light/dark geometry were checked. Android
  remains paused. OS reduced-motion true, screen readers, dynamic font/label
  replacement, transformed ancestors and touch/swipe behavior were not tested.
  Reduced-motion CSS remains in place; no runtime certification is inferred.
- This API does not currently expose inline tab badges; the official badge+
  label width example is a future API scope, not a tested configuration here.

## Reproduce

Run ./scripts/dev.sh, confirm build success, reload. Compare selected primary
label and indicator rectangles: width max(24, label width −4), common center,
3px height and zero gap to the divider. Click Hotels, use arrows to Cars, and
select Add in the icon example to exercise the minimum. Check first load,
secondary tabs, disabled Reviews and dark mode. Keep primary and secondary
width rules distinct when comparing the source.

## Historical reports before this correction

# Tabs — current fidelity update

2026-10-09, desktop Codex in-app browser. R1/R3/R4: enabled-only navigation,
48px/64px native target height, shared ripple, optional stable tab_ids/panel_ids
and associated gallery panels. That update incorrectly interpreted the horizontal 2dp label inset as a
vertical bottom inset. The correction above supersedes this claim.

Read official [overview](https://m3.material.io/components/tabs/overview),
[specs](https://m3.material.io/components/tabs/specs),
[guidelines](https://m3.material.io/components/tabs/guidelines), and
[accessibility](https://m3.material.io/components/tabs/accessibility) in the
preceding review on this date; inspected actual tabs.tsx at upstream c37c0d2.
Specs ArrowRight selects/focuses Overview; End selects/focuses Specs; disabled
Reviews remains unselected/tabindex −1. aria-controls targets exist. Fresh
web build/Wasm check and isolated usage compile passed.

See [fix evidence/reproduction/gaps](component-fidelity-fixes.md),
[raw samples](component-fidelity-fixes-samples.json), and
[copy dependencies](copy-components.md). Android, touch, assistive technology,
OS reduced-motion true and full RTL key navigation remain untested.

## Historical report before this fix

# Tabs verification

Scope: `Tabs`, `TabItem` and `TabsVariant` in
[`src/components/tabs.rs`](../src/components/tabs.rs) and
[`assets/tabs.css`](../assets/tabs.css). Verified in the Dioxus preview on 2026-10-09 with
headless Chromium 1194, using real mouse and keyboard input, at device scale 2. Desktop only.
Android Chrome has not been tested.

## 1. References

- Material Web primary navigation tab tokens, revision `47adb65`
  ([`_md-comp-primary-navigation-tab.scss`](https://github.com/material-components/material-web/blob/47adb65/tokens/versions/latest/sass/_md-comp-primary-navigation-tab.scss)):
  48dp container, 1dp divider in surface-variant, 3dp active indicator with 3dp top radii,
  title-small text, 24dp icon, 64dp icon-and-label container, active label primary, inactive
  on-surface-variant, state layers hover 0.08 and focus/pressed 0.10, disabled 38%.
- Material Web secondary navigation tab tokens, revision `47adb65`
  (`_md-comp-secondary-navigation-tab.scss`): the 2dp full-width indicator and the on-surface
  active label, used for the secondary variant.
- shadcn-m3e `tabs.tsx`, revision `8f1b3fb`: equal-width tabs (`flex-1`), the 16dp indicator
  inset from each tab edge with a 24dp minimum width, the 16dp state-layer shape, and the
  transition used for the indicator.
- **Not read:** the official `m3.material.io` tabs page. The egress proxy blocks that host.
  The token files and upstream source are the sources used.

## 2. Values and behaviour

| Property | Implemented | Source |
| --- | --- | --- |
| Container height | 48px (64px when any tab has an icon) | Tokens |
| Divider | 1px bottom border, surface-variant | Tokens |
| Tab width | Equal share of the bar (`flex: 1 1 0`) | Upstream |
| Tab height and inset | 40px with 4px vertical margin (56px with icon) | Tokens |
| Label | title-small, 14/20, weight 500 | Tokens |
| Icon | 24px, above the label when the tabs have icons | Tokens |
| Primary indicator | 3px, top radii 3px, the width of the selected label (minimum 24px), centred under the label | Material primary tab reference (supplied screenshot) |
| Secondary indicator | 2px, full tab width | Secondary tokens |
| Indicator motion | Left and width, 360ms expressive fast spatial spring | Upstream duration; kit curve |
| Selected label | Primary (primary variant); on-surface (secondary variant) | Tokens |
| Inactive label | on-surface-variant | Tokens |
| State layer | Hover 0.08; focus and pressed 0.10; 16dp shape | Tokens and upstream |
| Focus indicator | 3px secondary outline, inset | Tokens (inner offset) |
| Disabled | 38% label and icon, no state layer | Tokens |
| Keyboard | Arrows wrap; Home and End jump; selection follows focus | Automatic activation pattern |
| Roles | `tablist`, `tab`, `aria-selected`; roving `tabindex` (selected tab = 0) | ARIA |

## 3. Input, state and platform matrix

Raw data: [`tabs-samples.json`](tabs-samples.json). Screenshot: [`tabs-primary.png`](tabs-primary.png), taken after the fix.

| Check | Result |
| --- | --- |
| Tab widths, 3 tabs in a 504px bar | 168px each, equal |
| Bar height, plain / icon tabs | 48px / 64px |
| Icon size and tab height (icon tabs) | 24px; 56px |
| Primary indicator, initial | left 158px, width 45px; within 0.4px of the label's left edge and width |
| Indicator mid-slide (90ms after click) | 524.6px, moving from 158px toward 521px; it overshoots by about 3px from the expressive spring before it settles |
| Indicator after click on Hotels (790ms) | left 521px, width 43px; within 0.4px of the label |
| Mouse click selects | Hotels selected, Flights unselected |
| Press and hold (150ms) on an unselected tab | State layer 0.10; not selected while held |
| Release after press | Selected |
| Hover state layer | 0.08 |
| ArrowRight from Hotels | Cars selected; focus on Cars |
| ArrowRight from Cars (wrap) | Flights selected; focus on Flights |
| Home | Flights selected and focused |
| End | Cars selected and focused |
| ArrowLeft from Cars | Hotels selected and focused |
| Roving tab order | Selected tab `tabindex=0`; others `-1` |
| Label colours | Selected rgb(101, 85, 143) (primary); unselected rgb(73, 69, 78) (on-surface-variant) |
| Secondary indicator | 2px, left 168px, width 168px after selecting Specs, matching the tab |
| Secondary selected label | on-surface |
| Disabled tab, pointer click | Selection unchanged |
| Disabled label | 38% on-surface |
| Page errors | 0 |

## 4. Diagnosed defects

- **Primary indicator spanned the tab.** The first version used the upstream shadcn-m3e
  rule (tab width minus 32dp). The Material primary tab reference shows the indicator under
  the label only. The indicator now uses the label's measured width, with the 24dp minimum.
  The secondary indicator still spans the full tab, as the reference shows.

- **Indicator slid in from the left edge on mount.** The indicator is hidden until the script
  has placed it. The list's `data-ready` attribute then enables the transition, so the first
  placement is not animated.
- **Indicator position depended on Dioxus re-running an effect.** An effect that depends on a
  prop does not re-run on each change. The script instead observes `aria-selected`, so it
  follows the DOM whatever re-renders the list.

## 5. Build and checks

| Check | Result |
| --- | --- |
| `cargo check --locked --offline --target wasm32-unknown-unknown` | Passed |
| Preview served from a rebuilt bundle | Passed (build completed after the final edit) |
| Usage snippet compiled separately | Not compiled separately. It uses the same calls as the preview's primary card, which compiles |

## 6. Unresolved differences and untested behaviour

- The Material ripple is not implemented for tabs. The button, FAB and icon-button ripple port
  is not reused here, so press feedback is the state layer only.
- Scrollable tabs (more tabs than fit) are not implemented. Tabs share the bar equally and do not
  scroll.
- Vertical tabs are not implemented.
- Touch input, Android Chrome and the on-screen keyboard are not measured.
- Reduced motion sets the indicator transition to 0.01ms. It is not measured.
- Screen-reader output for the tab list and tabs is not measured.
- Panels are not provided: the consumer renders the panel for the selected index.
- The official Material tabs page was not read (blocked host). Values come from the token files
  and upstream source.

## 7. Steps to reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/` and wait for the build to report success.
2. Run a Playwright script against the preview with the Chromium at
   `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`. It reads the tab and indicator geometry,
   dispatches real mouse and keyboard actions, and samples computed styles while the input is
   held. The script is kept in the scratchpad and is not committed.
3. Compare the output with [`tabs-samples.json`](tabs-samples.json).
