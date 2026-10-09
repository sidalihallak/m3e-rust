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
  if (window.__m3Frames[id]) cancelAnimationFrame(window.__m3Frames[id]);
  const tick = (ts) => {{
    if (!document.getElementById(id)) {{ delete window.__m3Frames[id]; return; }}
    {body}
    window.__m3Frames[id] = requestAnimationFrame(tick);
  }};
  window.__m3Frames[id] = requestAnimationFrame(tick);
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
