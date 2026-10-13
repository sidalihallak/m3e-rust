# Independent consuming-app verification — 2026-10-13

Fixture: [examples/consumer](../examples/consumer/README.md). Runtime app:
/private/tmp/m3e-consumer, preview http://127.0.0.1:8081/. Thirteen requested
entries resolve to 32 modules. Application CSS controls page layout only.
Registry-generated UiStyles supplies component CSS/assets; no gallery or
Tailwind CSS. The app sets seed-generated light/dark semantic color roles.

## Actual desktop checks

- Pointer button activation, Enter activation, disabled skip during Tab.
- Button corner interpolation sampled during input; dimensions remain stable.
- Three icon styles and dynamic local sprite; labeled semantic SVGs.
- Card action, surface/elevation/body padding.
- Text entry and keyboard combobox option selection.
- Dialog initial input focus, Shift-Tab wrap to action, Escape dismissal and
  focus restoration. Field/notch geometry recorded in content area.
- Tabs Right-arrow selection, Switch Space, sidebar collapse, controlled table
  editing (10 to 27), and installed MessageScroller rendering.
- Light/dark colors; 390px responsive bounds without document overflow.
- Actual macOS Reduce Motion enabled temporarily: transition 0.00001s,
  height40px; original off setting restored.

[Raw observations and motion samples](consumer-browser-samples.json),
[desktop screenshot](consumer-desktop.png). Dioxus Web builds succeeded;
independent Wasm check passed. Component reference contracts are linked from
README and remain authoritative; this smoke scenario does not retest every
behavior of every installed component.

## Reproduction and gaps

Follow the fixture README from a fresh checkout, serve port8081, and operate
controls with pointer, Tab, Enter, Space and arrows. Verify the stylesheet graph
contains app.css and copied kit styles.

390px is a desktop responsive check, not an Android touch result. Android is
paused. Audible screen reader, full RTL, native Dioxus targets, all integration
permutations and deployment/CSP variations remain unverified.

Final manifest rebuild:13.40s, successful. Reloaded app accepts pointer click
and text entry. At390px the dialog opens, field geometry is recorded in the
raw observations, Escape closes it and returns focus to Create project.
[Compact screenshot](consumer-compact.png).
