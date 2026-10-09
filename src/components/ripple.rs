use dioxus::prelude::*;
use super::motion::next_id;

/// Shared browser ripple for a native action or its input label. Copy with
/// `motion.rs` and `assets/ripple.css`. The accepted Button keeps its own lifecycle.
#[component]
pub(crate) fn Ripple() -> Element {
    let id = use_hook(|| next_id("m3-ripple"));
    use_effect({
        let id = id.clone();
        move || {
            let script = SCRIPT.replace("__ID__", &format!("{id:?}"));
            spawn(async move { let _ = document::eval(&script).join::<bool>().await; });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            let script = format!("window.__m3RippleCleanups?.[{id:?}]?.(); if (window.__m3RippleCleanups) delete window.__m3RippleCleanups[{id:?}];");
            spawn(async move { let _ = document::eval(&script).await; });
        }
    });
    rsx! { span { id, class: "m3-ripple", aria_hidden: "true" } }
}

const SCRIPT: &str = r#"
const id = __ID__;
const layer = document.getElementById(id);
if (!layer || layer.dataset.ready) return true;
layer.dataset.ready = 'true';
const host = layer.parentElement;
const input = host.querySelector('input');
const enabled = () => !host.disabled && !input?.disabled && host.getAttribute('aria-disabled') !== 'true';
let generation = 0, held = false, suppress = false, timer, pending, current;
const listeners = [];
const listen = (name, handler, capture = false) => {
  host.addEventListener(name, handler, capture);
  listeners.push(() => host.removeEventListener(name, handler, capture));
};
const media = matchMedia('(prefers-reduced-motion: reduce)');
const start = (point) => {
  if (!enabled()) return;
  clearTimeout(timer);
  if (current) { current.grow.cancel(); current.fade.cancel(); current.node.remove(); }
  const token = ++generation;
  const rect = layer.getBoundingClientRect();
  const size = Math.max(1, Math.floor(Math.max(rect.width, rect.height) * .2));
  const x = point ? point.clientX - rect.left : rect.width / 2;
  const y = point ? point.clientY - rect.top : rect.height / 2;
  const scale = (Math.hypot(rect.width, rect.height) + 10 + Math.max(Math.max(rect.width, rect.height) * .35, 75)) / size;
  const node = document.createElement('span'); node.className = 'm3-ripple__wave';
  node.style.width = node.style.height = size + 'px';
  layer.append(node); host.dataset.ripplePressed = 'true';
  const reduced = matchMedia('(prefers-reduced-motion: reduce)').matches;
  const grow = node.animate([
    {transform: `translate(${x-size/2}px,${y-size/2}px) scale(1)`},
    {transform: `translate(${(rect.width-size)/2}px,${(rect.height-size)/2}px) scale(${scale})`}
  ], {duration: reduced ? 0 : 450, easing: 'cubic-bezier(.2,0,0,1)', fill: 'both'});
  const fade = node.animate([{opacity: 0}, {opacity: .1}], {duration: reduced ? 0 : 105, fill: 'both'});
  current = {node, grow, fade, token, at: performance.now()};
};
const release = () => {
  clearTimeout(timer);
  window.removeEventListener('pointerup', release);
  window.removeEventListener('pointercancel', release);
  held = false;
  const run = current; if (!run) return;
  const finish = () => {
    if (generation !== run.token || held) return;
    delete host.dataset.ripplePressed;
    const opacity = getComputedStyle(run.node).opacity;
    run.fade.cancel();
    run.fade = run.node.animate([{opacity}, {opacity: 0}], {duration: matchMedia('(prefers-reduced-motion: reduce)').matches ? 0 : 375, fill: 'both'});
    run.fade.finished.then(() => { run.grow.cancel(); run.node.remove(); if (current === run) current = null; }).catch(() => {});
  };
  timer = setTimeout(finish, Math.max(0, 225 - (performance.now() - run.at)));
};
listen('pointerdown', e => {
  if (!enabled() || !e.isPrimary || e.button !== 0) return;
  held = true; suppress = true;
  window.addEventListener('pointerup', release);
  window.addEventListener('pointercancel', release);
  if (e.pointerType === 'touch') pending = setTimeout(() => { pending = null; start(e); }, 150);
  else start(e);
});
listen('pointerup', e => { if (pending) { clearTimeout(pending); pending = null; start(e); } release(); });
for (const name of ['pointercancel', 'pointerleave', 'focusout']) listen(name, () => {
  clearTimeout(pending); pending = null; suppress = false; release();
});
listen('keydown', e => {
  if (!enabled() || e.repeat) return;
  if (e.code === 'Space') { held = true; suppress = true; start(); }
});
listen('keyup', e => { if (e.code === 'Space') release(); });
listen('click', e => {
  if (!enabled() || (input && e.target !== input)) return;
  if (suppress) suppress = false;
  else { held = false; start(); release(); }
});
const reduce = () => {
  if (media.matches && current) { current.grow.finish(); current.fade.finish(); }
};
media.addEventListener('change', reduce);
window.__m3RippleCleanups = window.__m3RippleCleanups || {};
window.__m3RippleCleanups[id] = () => {
  clearTimeout(timer); clearTimeout(pending);
  listeners.forEach(remove => remove());
  media.removeEventListener('change', reduce);
  window.removeEventListener('pointerup', release);
  window.removeEventListener('pointercancel', release);
  if (current) { current.grow.cancel(); current.fade.cancel(); current.node.remove(); }
  delete host.dataset.ripplePressed;
};
return true;
"#;
