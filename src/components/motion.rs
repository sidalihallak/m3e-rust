/// Builds a browser script that runs `body` once per animation frame for the
/// element with `id`. `ts` is the frame timestamp inside `body`. The loop stops
/// itself when the element leaves the page, so unmounted components do not keep
/// running. Starting a new loop with the same id cancels the old one.
pub(crate) fn frame_loop(id: &str, body: &str) -> String {
    format!(
        r#"
(() => {{
  const id = {id:?};
  window.__m3Frames = window.__m3Frames || {{}};
  const stale = window.__m3Frames[id];
  if (stale) {{ cancelAnimationFrame(stale.raf); clearTimeout(stale.timer); }}
  const media = matchMedia('(prefers-reduced-motion: reduce)');
  const state = {{ raf: 0, timer: 0 }};
  window.__m3Frames[id] = state;
  let previous = null, elapsed = 0;
  const tick = (now) => {{
    const root = document.getElementById(id);
    if (!root || root.dataset.m3FrameLoop !== 'true') {{
      delete window.__m3Frames[id];
      if (window.__m3SweepT0) delete window.__m3SweepT0[id];
      if (!root && window.__m3LoadingState) delete window.__m3LoadingState[id];
      return;
    }}
    // A hidden component (display: none, or a hidden route) does no work and wakes
    // slowly. Its clock is reset, so the time it was hidden does not advance the animation.
    if (root.getClientRects().length === 0) {{
      previous = null;
      state.timer = setTimeout(() => {{ state.raf = requestAnimationFrame(tick); }}, 250);
      return;
    }}
    if (previous !== null && !media.matches) elapsed += now - previous;
    previous = now;
    if (!media.matches) {{
      const ts = elapsed;
      {body}
    }}
    state.raf = requestAnimationFrame(tick);
  }};
  state.raf = requestAnimationFrame(tick);
}})();
return true;
"#
    )
}

/// A unique element id for one component instance.
pub(crate) fn next_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    format!("{prefix}-{}", NEXT.fetch_add(1, Ordering::Relaxed))
}
