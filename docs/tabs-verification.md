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
| Primary indicator | 3px, top radii 3px, width max(24px, tab − 32px), centred in the tab | Tokens and upstream |
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

Raw data: [`tabs-samples.json`](tabs-samples.json). Screenshot: [`tabs-primary.png`](tabs-primary.png).

| Check | Result |
| --- | --- |
| Tab widths, 3 tabs in a 504px bar | 168px each, equal |
| Bar height, plain / icon tabs | 48px / 64px |
| Icon size and tab height (icon tabs) | 24px; 56px |
| Primary indicator, initial | left 16px, width 136px; offset 0px from the expected position |
| Indicator mid-slide (90ms after click) | 185.6px, between 16px and 184px: it moves rather than jumping |
| Indicator after click on Hotels (790ms) | left 184px, width 136px; offset 0px |
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
