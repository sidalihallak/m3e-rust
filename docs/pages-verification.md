# Full page templates verification

Date: 2026-11-13. Scope: `AppPage`, `DashboardPage`, `SignInPage`, `InboxPage`,
`SettingsPage` and `NotFoundPage`, with the supporting types `StatCard`,
`ActivityEntry` and `MailMessage`.

Source files: `src/components/pages.rs`, `assets/pages.css` (bundled through
`src/styles.rs`), `src/pages_gallery.rs` (preview routes `dashboard-page`,
`sign-in-page`, `inbox-page`, `settings-page`, `not-found-page`).

## 1. Reference contract

- The layout and shell pieces are the ones in `docs/templates-verification.md`
  (breakpoints 600dp and 840dp; those values are unverified against m3.material.io).
- Navigation: a rail from 600dp and a bottom navigation bar below it, using the
  kit's `NavigationRail` and `NavigationBar`.
- Sign-in, inbox and settings compose the kit's `FormLayout`, `ListDetailLayout`,
  `SettingsSection` and `SettingsItem`. Status copy uses the Material type scale.
- No new motion. The pages reuse the motion of their components.

## 2. Verification matrix

Tool: Playwright (Chromium) against the `dx serve` build on `http://127.0.0.1:8080/`.
Script: `docs/pages-verify.js.txt`. Raw results: `docs/pages-verification-samples.json`.
Real pointer clicks, typing and keyboard input were used.

| Page | Check | Viewport | Result |
| --- | --- | --- | --- |
| Dashboard | Rail visible, bar hidden; four stat tiles; chart rendered | 1280 | Passed |
| Dashboard | Rail click changes the selected destination | 1280 | Passed |
| Dashboard | Bar click changes the selected destination | 390 | Passed |
| Dashboard | Rail hidden, bar visible; no horizontal overflow in frame | 390 | Passed |
| Sign-in | Empty email shows validation; short password shows the error alert | 1280 | Passed |
| Sign-in | Show/hide password toggle changes input type and pressed state | 1280 | Passed |
| Sign-in | Valid credentials update status; alert clears | 1280 | Passed |
| Sign-in | Form panel fits the viewport; no overflow | 390 | Passed |
| Inbox | List and detail both visible; back control hidden | 1280 | Passed |
| Inbox | Search filters by subject, empty state shown, search cleared restores | 1280 | Passed |
| Inbox | Selecting a message shows its subject | 1280 | Passed |
| Inbox | List only at start; tap opens detail; back returns; row ≥ 48px | 390 | Passed |
| Settings | Profile name shown; switch toggles on click | 1280 | Passed |
| Not found | Status code shown; action fires once per click | 1280 | Passed |
| All | No horizontal overflow in the frame | 390 | Passed |
| All | No console errors or page errors | both | Passed |

Result: 43 of 43 checks passed.

Usage snippets: all 11 copyable usage blocks (six templates, five pages) were
extracted from the galleries and compiled against this crate in a scratch crate
(`cargo check`, host). All compile. The six template snippets had not been compiled
before this change.

Build: `cargo check --locked` (host) passed with no warnings after the change. The
wasm32 check and the `dx serve` build (12.7s incremental) also succeed.

## 3. Screenshots

- `docs/pages-desktop-dashboard.png`: dashboard at 1280px inside the preview frame.
- `docs/pages-desktop-sign-in.png`: sign-in after a failed attempt and success.
- `docs/pages-desktop-inbox.png`: inbox with both panes at 1280px.
- `docs/pages-phone-inbox.png`: inbox at 390px, bottom navigation bar.

## 4. Gaps

- **Two `h1` elements per app page.** `AppBar` renders its title as an `h1`, and the
  page title is also an `h1`. Changing `AppBar` is a public-API decision and is left
  for its own change.
- **Android and touch:** the phone results are an emulated 390×844 viewport in desktop
  Chromium, not a device.
- **Screen readers:** not run. Labels, roles and live status were checked in the DOM.
- **Preview frames:** each page sits in a fixed-height frame. The layout follows the
  browser window's width, not the frame's, so a frame narrower than the window does not
  show the layout it would have at its own width.
- **Data:** the pages take their content as props. No real backend or auth flow is
  included; the sign-in page only validates and reports credentials.
- **Reference comparison:** shadcn-m3e was not compared for these pages.

## 5. Reproduce

1. Run `./scripts/dev.sh` and wait for "Build completed successfully".
2. Copy `docs/pages-verify.js.txt` to a `.js` file and run it with Playwright:
   `node <file> <output-dir>`.
3. Compare the output with the table above.
