# Navigation verification — 2026-10-11

## Scope and references

Core Dioxus AppBar, NavigationBar, NavigationRail, ModalNavigationRail and Toolbar,
plus the showcase catalogue reorganization. Android remains paused.
Official overview, specs, guidelines and accessibility pages were read in the
rendered browser on 2026-10-10 for [app bars](https://m3.material.io/components/app-bars/overview),
[navigation bars](https://m3.material.io/components/navigation-bar/overview),
[navigation rails](https://m3.material.io/components/navigation-rail/overview) and
[toolbars](https://m3.material.io/components/toolbars/overview).
The current Expressive expanded rail replaces the legacy navigation drawer.

Actual upstream source inspected at revision
`c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972`:
[app-bar.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/app-bar.tsx),
[navigation.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/navigation.tsx),
[toolbar.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/toolbar.tsx).

## Reference contract and recorded desktop results

| Component/state | Contract and observed result |
| --- | --- |
| App bars | Small 64px; medium 112px/plain, 136px/subtitle; large 120px/plain, 152px/subtitle. All six gallery examples measured these heights. Centered small title had zero horizontal offset with unequal action widths. Scrolled state changed surface/elevation. |
| Navigation bar | Default 64px, horizontal layout has a 40px indicator. Vertical indicator 56×32px; icon 24px. Active icon filled, inactive outlined; active vertical label uses current official secondary role. Tall 80px variant implemented but final new demo was not remeasured. |
| Navigation rail | Collapsed 96px (optional 80px); expanded width clamped 220–360px. Measured 96→220px. Expanded targets 220×56px, icon start 36px and indicator inset 16px, matching the inspected official diagram. |
| Expansion | An initial wrapping defect grew height from 390 to 552px during transition. Keeping expanded content at its target width fixed it: repeat samples held height at 390px while width interpolated. |
| Selection | Arrow focus moved Home→Library without selecting; Enter selected Library and changed to filled glyph. Disabled Explore ignored pointer activation. RTL ArrowLeft skipped disabled Explore and focused Library. |
| Toolbar | Floating/docked horizontal examples measured 64px high with actual 48×48px controls and 4px gaps. Arrow focus and Space toggle worked. Vertical variant implemented; final added example not separately exercised. |
| Modal rail | Native dialog 320px wide and viewport height, initial Close navigation focus, background scroll locked. Escape, outside scrim and destination selection closed and restored opener/overflow. |
| Modal boundaries | Initial reverse Tab escaped the expected controls. Explicit boundary wrapping fixed Shift+Tab first→last and Tab last→first in the repeated browser check. |
| Motion | Existing token curves: FastSpatial 360ms indicator, DefaultSpatial 440ms rail/modal, DefaultEffects 240ms app-bar surface. Computed transforms were sampled during actual entry/exit and input, not inferred solely from declarations. |
| Reduced motion | macOS Reduce Motion temporarily enabled with user authorization; actual computed transitions were 0.00001s from the global override, width settled directly at endpoints, modal opened without transform and Escape restored focus. Original OFF setting restored. |
| Copying | All five navigation copy controls returned the exact examples: 409/800/629/837/612 characters. All 40 gallery snippets compiled as independent exported examples. |

The expanded rail deliberately uses the officially permitted full-width pill
rather than the default text-hugging pill. Native destination buttons retain Tab,
Enter/Space, disabled behavior and `aria-current="page"`; navigation is not a tablist.
Consumer callbacks own route changes. The rail's default top padding is the
upstream 44px; the showcase uses application-specific 12px padding.

## Showcase verification

Six primary families and 28 stable hash destinations replace the long single page.
The desktop shell combines an app bar, collapsible rail, component catalogue,
search and one visible component page. Sections remain mounted to preserve state.
Compact layout hides both desktop sidebars, uses a modal expanded rail for six
families, and uses the existing NativeSelect for the component picker.
The catalogue search is application-specific, not a completed reusable Search port.

Actual cross-family search originally failed because clearing the query removed
the clicked anchor before default navigation. Keeping the query until explicit
clear fixed selection of Sliders; browser Back restored Navigation. Individual
App bars and Navigation rails pages were inspected. Centered/scrolled app-bar
checks and route changes were exercised in the live browser.

## Builds and evidence limits

Live Dioxus serve reported successful builds, including the final NativeSelect
integration (7.62s); a single `.app-shell` was observed after the clean restart.
The independent exported library plus all 40 examples passed offline locked
Wasm cargo check (1.18s). JavaScript syntax checks and diff whitespace checks passed.
The final locked offline Wasm check passed on 2026-10-11 (5.41s); JavaScript syntax and `git diff --check` also passed before publishing.

These are retained observation notes from the working session. Raw frame arrays
were held in the browser tool session, which was no longer available at publish
time; they were not persisted and are not represented as an attached dataset.
A final browser recheck was rejected by the browser URL security policy. No
workaround was attempted. Consequently final compact NativeSelect routing, skip
link, the newly added tall/vertical demos and state preservation are not claimed
as independently rechecked after their last edits.

No Android touch/native-package verification or audible VoiceOver check is claimed.
Also open: enlarged text/forced colors, nested modals, dynamic removal during
animation, broader browser compatibility and exact font/pixel parity. This is core
navigation scope, not all upstream navigation menus/sidebar helpers.

## Reproduce

Run `./scripts/dev.sh`, wait for a successful build and reload. Open `#navigation`,
then each navigation subpage. Check pointer, arrows, Home/End, Enter/Space,
disabled and RTL demos. Expand/collapse the rail while sampling width/height.
Open the modal, test both boundary Tab directions, Escape, scrim and selection;
confirm opener focus and scroll restoration. Repeat with reduced motion and
restore the system setting. Resize to phone width and exercise the compact
family menu/component picker separately; resume Android checks when requested.
