# Page templates verification

Date: 2026-11-12. Scope: `PageLayout` (with `PageWidth`), `ListDetailLayout`,
`SupportingPaneLayout`, `FeedLayout`, `FormLayout`, `SettingsSection` and
`SettingsItem`.

Source files: `src/components/templates.rs`, `assets/templates.css` (bundled through
`src/styles.rs`), `src/templates_gallery.rs` (preview, routed as `page-layouts`,
`list-detail-layouts` and `feed-layouts`).

## 1. Reference contract

- Material canonical layouts (list-detail, supporting pane, feed) and window size
  classes (compact < 600dp, medium 600–839dp, expanded ≥ 840dp).
  **Not verified against m3.material.io**: the sandbox could not resolve that host.
  The 600dp and 840dp breakpoints and the 70/30 supporting-pane suggestion come from
  a secondary search summary of the Android canonical-layout guidance
  (https://developer.android.com/guide/topics/large-screens/large-screen-canonical-layouts).
- Margins (16dp compact, 24dp from medium), the 280dp feed column minimum, 360dp list
  pane and 480dp / 640dp / 1040dp widths are this kit's choices, not values taken from
  a Material page. They are unverified.
- Upstream shadcn-m3e was not inspected for these templates in this session.
- Typography: Material type-scale sizes (headline 24–28px, body 14–16px), in the Inter
  stack used by `card.css`. Colours are semantic `--md-sys-color-*` roles. The templates
  have no motion, so reduced-motion behaviour is not applicable.

## 2. Implemented behaviour

| Template | Compact (< 600dp) | Medium (600–839dp) | Expanded (≥ 840dp) |
| --- | --- | --- | --- |
| `PageLayout` | 16dp margin | 24dp margin, 28px title | Max width per `PageWidth`: Narrow 640px, Standard 1040px, Full none |
| `ListDetailLayout` | One pane: list, or detail when `show_detail` is true | One pane (as compact) | Both panes: 360px list, detail fills the rest |
| `SupportingPaneLayout` | Supporting pane stacks below main | Stacks below main | 7fr / 3fr side by side |
| `FeedLayout` | 1 column at phone width | Auto-fill of ≥ 280px columns, 16px gap | Auto-fill, 24px gap |
| `FormLayout` | Centred panel, 16dp padding | Centred panel, 24dp padding, max 480px | Same |

Each template renders a labelled landmark (`section` with `aria-labelledby` to its
`h1`/`h2`, or `aside`/`section` with `aria-label`). `PageLayout` and `FormLayout` put their
title in an `h1`; settings groups use an `h2`.

## 3. Verification matrix

Tool: Playwright (Chromium, pre-installed) against the `dx serve` build on
`http://127.0.0.1:8080/`. Script: `docs/templates-verify.js.txt` (saved as text;
copy to a `.js` file to run). Raw results: `docs/templates-verification-samples.json`.

| Area | Input / check | Result |
| --- | --- | --- |
| Build | `dx serve` web build, one `.app-shell` | Passed |
| Desktop 1280×900 | Standard page: h1 labels its section, max width 1040px | Passed |
| Desktop | Feed columns match the measured container width (3 cards, 2 columns at 792px) | Passed |
| Desktop | Form: real typing and a click on Submit shows the entered email | Passed |
| Desktop | Settings switch toggles on click (`aria-checked` true → false) | Passed |
| Desktop | List-detail: both panes side by side, list 360px wide | Passed |
| Desktop | Supporting pane beside main, split ≈ 7:3 | Passed |
| Desktop | Back control hidden in expanded | Passed |
| Desktop | Dashboard feed columns match container (2 at 760px); full-width page has no max width | Passed |
| 390×844 (emulated) | List only at start; tap on an item opens detail and hides the list | Passed |
| 390×844 | Back control returns to list; `aria-current` kept on the selected item | Passed |
| 390×844 | Keyboard: focus item and press Enter, detail opens | Passed |
| 390×844 | Supporting pane stacks below main; feed is one column; form fits 390px | Passed |
| 390×844 | No horizontal page overflow on the three sections | Passed |
| 390×844 | List item height 68px (≥ 48px target) | Passed |
| Console | No console errors or page errors during the run | Passed |

Result: 33 of 33 checks passed. A wide-window measurement (1920px viewport) gave a feed
of 990px wide with 3 columns, because the preview's catalogue sidebar takes about 280px of
the window.

Note on the preview: the showcase's catalogue sidebar narrows the content column, so
on a 1280px window the templates render at about 840px, not at their Standard maximum.

## 4. Screenshots

- `docs/templates-desktop-page-layouts.png`: desktop page, settings and form (device scale 2).
- `docs/templates-desktop-list-detail.png`: desktop list-detail with both panes (device scale 2).
- `docs/templates-phone-list-detail.png`: 390px detail pane with back control (device scale 2).

## 5. Gaps

- **Android and touch:** the phone results are an emulated 390×844 viewport in desktop
  Chromium. They do not establish Android touch behaviour, scrolling beginning on a
  pane, or the on-screen keyboard. Not tested on a device.
- **Screen readers:** landmarks and labels were checked in the DOM only. No screen reader
  or accessibility-tree run was done.
- **Material values:** margins, gutters, and the breakpoint and split values need checking
  against the Material pages once `m3.material.io` is reachable from this environment.
- **Reference comparison:** shadcn-m3e was not compared for these templates.
- **Forced colours:** not checked for the new surfaces.

## 6. Reproduce

1. Install dioxus-cli 0.7.10 and run `./scripts/dev.sh`. Wait for "Build completed successfully".
2. `cp docs/templates-verify.js.txt /tmp/verify.js`, then run
   `node /tmp/verify.js <output-dir>` with Playwright available.
3. Compare the printed result with the table above.

Compile checks run in this session: `cargo check --locked` (host) and
`cargo check --locked --target wasm32-unknown-unknown`. Both passed with no warnings.
