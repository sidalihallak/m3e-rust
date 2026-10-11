# Chart verification — 2026-10-11

## Scope

`src/components/chart.rs`, `chart.js`, `assets/chart.css` and
`src/visual_gallery.rs`. Public Chart/ChartContainer/ChartLegend/
ChartTooltipContent, ChartSeries, ChartPoint, ChartKind and tooltip helpers.
Bar, line, area and donut are implemented as native SVG compositions.

## References

The [current Material 3 catalog](https://m3.material.io/components) has no
standalone chart component or four-page chart specification. Read the actual
[archived official data-visualization guidance](https://m2.material.io/design/communication/data-visualization.html)
for principles, chart types, baseline, labels, color, legends, motion and
accessibility. It is archived M2 guidance, not a newly claimed M3 chart contract.
The [WAI complex-image guidance](https://www.w3.org/WAI/tutorials/images/complex/)
informs the named figure, accompanying description and underlying data table.

Inspected the complete [upstream chart source](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/components/chart.tsx)
and [shared style tokens](https://github.com/Crysta1221/shadcn-m3e/blob/c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972/packages/m3e/src/styles/m3e.css)
on 2026-10-11. Confirmed revision c37c0d2f6aa3a8ab0b3f195c3ce0f6a568064972.

## Reference contract and implemented values

- Upstream is a Recharts wrapper/configuration/tooltip/legend system, not a
  chart renderer. This Rust port preserves its roles/styling using native SVG;
  it does not claim Recharts API, chart-type or animation parity.
- Aspect-video plot; initial320×200 dimensions and 12/16 body-small text.
  A ResizeObserver drives SVG viewBox from actual CSS dimensions, avoiding
  scaled-down axis text on narrow layouts. Min plot height200px is a kit choice.
- Axis text on-surface-variant; grids/cursor outline-variant; semantic series
  colors by default. Caller-supplied CSS color expressions can use theme roles.
- Tooltip: minimum128px, 16px corners, outline-variant1px border, surface fill,
  padding10px horizontal/6px vertical, 6px row gap, elevation2 matching source
  (0 1px 2px .3 plus0 2px 6px 2px .15), medium monospace tabular values.
- Tooltip indicators: dot10×10px/2px corners; line4px; dashed1.5px borders.
  Source icon/custom formatter/hide variants are not all equivalent APIs.
- Legend gap16px and12px top spacing, 8px swatches with2px corners. The
  interactive chart legend adds48px targets and preserves one visible series.
  Compact layouts place the legend above the plot, following archived guidance.
- Bar/area domains include zero, with explicit zero baseline and correct signed
  bars. Line/area preserve missing-value gaps rather than converting them to0.
  Multiple overlaid areas are limited to two; stacked area is an extension.
- Donut takes one datum with non-negative category values. Zero totals show the
  neutral ring/zero, negative values or unsupported totals display status.
- Chart points have native48px inspection buttons; arrows/Home/End focus points;
  focus/hover expose tooltip values, Escape dismisses. Legend buttons have
  actual toggle semantics; inspection buttons are ordinary commands.
- Figure/caption and a named raw-data table expose every original value. SVG is
  decorative to assistive technology, with text/button alternatives. Colors
  are supplemented by circle/square markers and optional dashed lines.
- Data redraws immediately; no decorative entrance or guessed duration. General
  Recharts interpolation/animation is not ported. No plot animation is present; the actual OS setting was checked with the
  shared carousel family. No Recharts animation equivalence is claimed.

## Verification matrix

| Check | Result |
| --- | --- |
| Four chart types | Rendered in the gallery |
| Pointer inspection | Mon tooltip shows Desktop180 / Mobile95 |
| Keyboard | Tab/arrows focus points; Thu280/155; Escape dismisses tooltip |
| Legend | Mobile hides; final Desktop toggle disabled; original raw data retained |
| Edge cases | Empty status; signed bars respect zero; missing line splits; negative donut rejected |
| Tooltip geometry | 128px minimum,16px corners,6px/10px padding,12px text, source elevation2 |
| Themes | Light surface rgb(253,247,255); dark surface rgb(32,24,36), text rgb(230,224,233) |
| 390px | SVG295×200 and identical viewBox;12px axis text;48px controls; body/client375px |
| Copy | All four snippets match clipboard exactly |

## Build and evidence

Dioxus 0.7.10 / Rust 1.99 built the final preview successfully in 19.35s;
reloaded before final checks. The refreshed independent library and all **75**
usage examples passed the locked offline Wasm check in **8.45s**. One existing
unused ChipSize import warning remains. No chart/carousel dependency was added.

Actual Codex in-app-browser inputs and read-only DOM samples are saved in
[visual-family-samples.json](visual-family-samples.json), with screenshots in
[evidence/visual](evidence/visual/). Earlier local access failed from an internal
data error page; the user returned the tab to the HTTP preview and inspection
resumed. No alternate browser or policy bypass was used. A stale rebuild toast
prompted a preview restart; the mounted application and final screenshots are
healthy. The preview remains running.

All **nine** new copy controls produced clipboard text exactly matching the
displayed code. The first read preceded clipboard completion; repeating after
completion passed. OS Reduce Motion was temporarily enabled under prior user
approval, its browser media query verified, then restored to its original off
state. Android remains paused; 390px is desktop responsive evidence only.

## Remaining differences and gaps

Stacked charts, scatter/radar/radial charts, brushing, zoom/pan, date axes,
custom per-point renderers, Recharts tooltip payload APIs, full formatter/icon
parity, animation and a large-data engine are not implemented. Long labels,
custom seeds, text enlargement, forced colors, RTL, very large numeric values,
series/data replacement, screen-reader speech and Android remain unverified.

## Reproduce

Open `/#charts` after a successful build and reload. Click a bar/line marker and
move away; Tab through Inspect controls, use arrows/Home/End/Escape, toggle
series and open the raw table. Open the edge-case disclosure. Repeat at390px
and in dark mode. Copy all four chart examples and compare the clipboard to
the displayed code. Record geometry, color and any observed motion before
extending the recorded evidence.
