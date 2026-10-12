# Menubar verification — 2026-10-12

## Scope / sources

`menubar.rs`, `menubar.js`, `navigation-extensions.css`, gallery.
Inspected full [pinned upstream menubar](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/menubar.tsx)
and its shared behavior through our existing Menu/Ripple implementation.
Read current rendered Material menus [overview](https://m3.material.io/components/menus/overview),
[specs](https://m3.material.io/components/menus/specs),
[guidelines](https://m3.material.io/components/menus/guidelines),
[accessibility](https://m3.material.io/components/menus/accessibility), and
[WAI menubar](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/).
There is no standalone M3 menubar page; it is a library action-menu composition.

## Contract

Source surface-container pill,4px gap/padding,label-large triggers and secondary
container expanded state. Trigger target is48px (source visual40px); resulting
bar is56px high. Existing expressive Menu is reused for current M3 recommended
selection/shape/submenu behavior instead of recreating upstream's8px shell.
This difference is deliberate; baseline Menu/Ripple was not changed.

Named menubar; native button menuitems with roving tabindex. Disabled top items
remain focusable and cannot activate. Left/Right wrap (RTL-aware); Home/End;
Down/Up open first/last menu item; native Enter/Space activate. Cross-menu arrows
from root popup switch menus; nested submenu arrows retain existing Menu behavior.
Tab exits the whole composite, Escape restores the current opener. Pointer
switching only occurs when an action menu is already open. Checkbox/radio and
submenus reuse controlled MenuSelection; shortcuts belong to the app.

## Verification

Runtime results are recorded below. Android remains paused. Copy MenubarSelection with the API; callbacks inspecting fields
need the explicit Rust event type shown in the usage example.

## Gaps

Source portal/render adapters, compact bottom-sheet menu adaptation, browser/OS
menu integration and global shortcuts are extensions. Audible screen readers,
RTL runtime, forced colors, zoom, dynamic entries and Android are not yet checked.

## Runtime checks and repairs

| Check | Result |
| --- | --- |
| Roving focus | File→Edit via Right; Home/End; one0 tabindex |
| First/last opening | Down→Undo; Up→disabled Export (focusable, not activatable) |
| Cross-menu keys | Undo Right→View Auto save |
| Checkbox | Space toggles true→false, retains popup |
| Tab | Exits entire composite to Focus after the menubar |
| Submenu | Share Right→Copy project link; Left returns to Share |
| Disabled top item | Tools receives focus; Enter does not open a menu |
| Pointer switch | Final File→Edit leaves Edit open with Undo focused |
| Repeated input | Two quick clicks close; three reopen |
| Geometry |56px bar /48px triggers;192px-wide File popup with16px inherited corners |
| 390px |249.99px bar;192px popup at x52 within375px client width; no body overflow |

Pointer hover initially opened a new menu that its click immediately closed.
Moved trigger activation to one JS owner. A second rapid-toggle check exposed
queued Rust state lag; the runtime now tracks requested state until DOM
acknowledgement. Pointer activation explicitly focuses the first action when
hover has already opened that popup. Rust close callbacks are scoped to their
active menu index. Earlier failed samples remain diagnostic; final successful
states are `finalPointerFocus`, `quickToggleClosed` and `tripleOpen`.

27 entry/28 exit frames record actual opacity and scale through the existing
Menu animation. Entry ends opacity1/transform none at192×217px; exit reaches
closed. No shape/animation timing was guessed or standalone Menu changed.
[Desktop](evidence/navigation-extensions/menubar-desktop.png),
[390px](evidence/navigation-extensions/menubar-390.png).
Use `/#menubars`; test arrows, checkbox, disabled items, submenu and rapid clicks.

## Build, export and evidence

Final preview built successfully in13.47s after the last gallery update (earlier
successful31.18s/20.10s builds include runtime repairs). Reloaded before final
checks; one `.app-shell` confirmed. The final refreshed independent Rust/Wasm
library and all **82** usage examples compile in **7.26s**, with one existing
unused ChipSize warning. No dependency was added. Preview remains running.

[Raw desktop/responsive/motion/copy samples](navigation-extensions-samples.json).
Three new copy controls exactly match displayed Rust text: Menubar568,
NavigationMenu427 and Sidebar781 characters. The initial navigation-menu read
preceded clipboard completion; the completed recheck passed. Viewport and light
mode restored. Android remains paused;390px results are desktop responsiveness,
not Android touch evidence. Audible screen-reader, actual OS Reduce Motion,
RTL, forced colors, zoom and held/touch cancellation checks remain open for this
family. Existing shared runtimes support reduced motion but were not newly
verified under the OS setting here.
