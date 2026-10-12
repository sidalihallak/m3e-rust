# Resizable panels — verification, 2026-10-12

## Scope and references

`src/components/resizable.rs` + `resizable.js`, `assets/layout-helpers.css`,
`src/layout_helpers_gallery.rs`: percentage-based adjacent panel groups,
constraints, orientation, optional compact/medium stacking, separators,
collapse/restore, disabled resize, pointer cancellation and committed layout callback.

There are no standalone Material resizable-panel overview/spec/accessibility
pages. Read the rendered [canonical overview](https://m3.material.io/foundations/layout/canonical-examples/overview),
[supporting-pane usage/specs](https://m3.material.io/foundations/layout/canonical-examples/supporting-pane),
[usability](https://m3.material.io/foundations/usability/overview),
[accessibility principles](https://m3.material.io/foundations/overview/principles)
and [RTL](https://m3.material.io/foundations/layout/bidirectionality-rtl) on2026-10-12.
Supporting content moves below primary content at compact/medium sizes; the optional
adaptive helper stacks below840px. A generic group is not a complete canonical
scaffold or a claim of the canonical360dp supporting-pane width.

Inspected [resizable.tsx](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/resizable.tsx)
at `c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972` and
[react-resizable-panels maintainer API](https://github.com/bvaughn/react-resizable-panels).
Applied the [WAI window splitter pattern](https://www.w3.org/WAI/ARIA/apg/patterns/windowsplitter/).

## Contract

- Source geometry:1px OutlineVariant line, optional4×48px Outline grip,3px
  Secondary focus outline. Local fine-pointer handle region24px; coarse48px.
- Direct children alternate Panel/Handle/Panel. Defaults initialize once and
  normalize to100%; native runtime retains live sizes. Min/max are percentages
  of available panel space, excluding handles. Impossible initial constraints
  disable resize through data-invalid.
- Pointer capture updates adjacent panes immediately, without a width/height
  transition. Layout callback publishes initial layout and committed drag/key changes.
- Native focusable separator has name, orientation, controls, min/max/current value
  and percentage value text. Horizontal groups use vertical separators.
- Arrow step1%, Shift step10%; Home/End bounds; Enter collapse/restore when primary
  pane is collapsible. Double-click restores that pair's default ratio.
- Escape during drag restores start layout; pointercancel/lost capture share the
  cancel path. Superseded disabled/orientation state cancels the active drag.
- Collapsed pane has0 size and native inert, removing its descendants from focus.
  Group/handle/panel disabled state prevents adjacent resize and removes handle
  from tab order. Content remains operable when resizing is disabled.

The24px hit area and adaptive stacking are deliberate additions; upstream wrapper
has a1px line and4px pseudo hit strip. No arbitrary drag spring or exaggerated
motion was introduced.

## Actual desktop/compact checks

- Initial65/35 sizes; ArrowRight66, ShiftLeft56, Home25, End80. Bounds include the
  adjacent panel's minimum20%. ARIA orientation/controls/values match runtime.
- Native pointer drag100px moved80% to63.997%;30 geometry frames saved. Sizes update
  directly and sum100. Content scrolls if a panel becomes narrower than its content.
- Lock sets aria-disabled and leaves63.997 unchanged. A browser locator refused
  disabled activation; disabled native pointer/keyboard permutations remain a gap.
- Escape sent while data-resizing=true during a native drag: before/after63.997,
  demonstrating cancellation rather than an already-ended drag.
- Vertical Enter collapsed Editor to0px/inert, focus stayed on separator. ShiftTab
  skipped Editor action and reached Comment action. Enter restored70%; ArrowDown71.
- Double-click restored65/35 in the document group.
-390px: body/client375px, full panel width325px, vertical stacking; separator becomes
  aria-orientation=horizontal. ArrowDown changes65→66. Native touch remains untested.
- Two copy controls match exactly650 and547 characters; early clipboard reads were
  retried after the copy operation completed.

[Raw measurements](layout-helpers-samples.json),
[desktop](evidence/layout-helpers/resizable-desktop.png),
[390px](evidence/layout-helpers/resizable-390.png).
Resizing is direct manipulation, so there is no enter/release interpolation to time.

## Build, gaps and reproduction

Actual preview build succeeded; independent library plus all91 usages compile.
Final timings are recorded with the raw family evidence. Only existing unused
ChipSize import warning. All JavaScript is retained by include_str in the source export.

Android is paused. Audible screen readers, RTL resize, actual reduced motion,
forced colors/zoom, pointercancel on real touch, multi-panel/nested permutations,
changing min/max/default props and constraint failures remain untested. No px/rem
constraints, persistence/provider, imperative resize API, grid resizing, separator
preview mode or complete react-resizable-panels behavior is claimed. Defaults are
initial values; remount for new initial layouts. Separate bounded content/adaptation
belongs to the app when this generic helper is used outside the gallery.

Run `./scripts/dev.sh`, wait for success and reload Content → Resizable panels.
Try arrows/Shift/Home/End, drag, Escape while dragging, Lock, double-click reset,
vertical Enter collapse/restore and ShiftTab. Resize to390px and verify orientation.
Copy both examples, retaining resizable.js and layout-helpers.css.

Final rebuild22.68s and one mounted app-shell. On the final build ArrowRight updates
65→66. Dark theme grip uses Outline rgb148,143,153; surface uses
SurfaceContainerLow rgb29,27,32; light theme restored. Temporary viewport override
was reset, and the review preview remains running. This is rendered role evidence,
not a complete custom-seed contrast or forced-color audit.
