# Performance audit

Scope: the component library (`src/components`, `assets/*.css`) as shown in the showcase preview,
measured in headless Chromium 1194 on desktop (1280×900). Desktop only. Android and low-end
devices are not measured.

Method: `docs/performance-route-sweep.json` records, for each of the 80 showcase routes, idle
animation frame requests, script time, layouts and style recalculations over 2 seconds, plus the
section's DOM node count. The browser's `Performance.getMetrics` supplies the counts.

## 1. Findings and changes

### Hidden components kept animating (fixed)

Every showcase section is mounted and hidden, so the animation loops of hidden components kept
running. On `#buttons` three loops ran forever (two loading indicators and one circular progress),
all inside hidden sections.

| `#buttons`, idle over 3 s | Before | After |
| --- | --- | --- |
| Frame callbacks requested | 540 | 36 |
| Script time | 124.8 ms | 4.6 ms |
| Task time | 184.6 ms | 15.2 ms |

Change: `src/components/motion.rs` (`frame_loop`). A loop now checks whether its element has a
layout box. While it has none, the loop wakes every 250 ms instead of every frame, and its clock is
reset so the hidden time does not advance the animation when it becomes visible again. The
remaining 36 frame requests are those 250 ms wake-ups for three hidden loops.

Visible loops are unchanged: on `#progress`, the three visible loops still request about 180
frames per second, and the indicator's path changes between frames.

### Circular progress rebuilt its path while holding still (small)

`SWEEP_BODY` in `src/components/progress.rs` rebuilt the wavy path every frame, including the half
of each cycle when the sweep does not move. It now rebuilds only when the sweep changes. The script
time on `#progress` fell from about 159 ms per 3 s (extrapolated from a 2-second sweep) to 117 ms
per 3 s. The change is small; it does not affect the layout cost below.

### Animated SVG paths cost layouts (open)

On `#progress` the three animated indicators cause about 213 layouts per second (639 per 3 s),
and the task time is about 21% of a core (633 ms per 3 s). Each frame changes an SVG `d` attribute,
and the browser relays out the SVG for each change. The cache above does not remove this, because
the morph changes every frame while it runs. Reducing it would need a different approach: for
example pre-rendered frames, or a frame-rate cap, which changes the motion and needs a design
decision. This is not changed.

### Route sweep (all other routes)

- 78 of the 80 routes show no layouts or style recalculations while idle. Each has at most 12 frame
  requests per second (the hidden-loop wake-ups), and script time of at most 2.4 ms per 2 seconds.
- `progress` has visible morphing indicators; see the layout finding above.
- `spinners` has 121 layouts per 2 seconds (about 60 per second), from its arc animation. The arc's
  CSS keyframes animate `stroke-dashoffset` (in `assets/spinner.css`), which is not a compositor
  property, so the browser relays out the SVG each frame. The fix would change how the spinner looks
  (for example animating a different property), so it needs a design decision. This is not changed.

### CSS bundle

`component_css()` is 275,766 bytes uncompressed. Without comments it is 245,143 bytes. Gzip gives
52,145 bytes with comments and 39,865 without. The bundle is not minified. Removing the comments at
build time would save about 30 KB uncompressed; this is not changed.

### Event listeners and observers

Per-component scripts add observers and global listeners (for example `menu.js`, `menubar.js`,
`toaster.js`, `time_picker.js`, `calendar.js`, `search.js`). Those I checked remove their listeners
and disconnect their observers when the component unmounts (`window.__m3...` cleanup functions,
or `use_drop`). This was checked by reading the code, not by measuring memory after unmount.

## 2. Not measured

- Release Wasm size. The only build measured is the debug build (about 121 MB with symbols).
- Memory after unmounting components (no leak test was run).
- Touch devices, low-end CPUs, and reduced motion (the reduced-motion path skips the loop bodies,
  so its cost should be near zero, but this was not measured).
- Per-component timing for components that have no route of their own.

## 3. Reproduce

1. `./scripts/dev.sh`, then open `http://127.0.0.1:8080/`.
2. Run a Playwright script that loads `#buttons` and reads `window.__m3Frames` and the
   `Performance` metrics before and after a 3-second wait.
3. For the sweep, visit each route with `location.hash` and record the same counters.
