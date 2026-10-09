use dioxus::prelude::*;

use super::motion::{frame_loop, next_id};
use crate::loading_shapes::SHAPES;

/// Loading indicator style. `Contained` sits on a primary-container disc.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadingIndicatorVariant {
    #[default]
    Default,
    Contained,
}

impl LoadingIndicatorVariant {
    const fn class(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Contained => "contained",
        }
    }
}

// Shapes are normalised to radius 1 and drawn at 38/48 of a 48dp container.
// Contained scales the indicator by 0.84, as the reference does.
const INDICATOR_SCALE: f64 = 0.5 * 38.0 / 48.0;

/// Seven Material shapes that morph into one another while the indicator turns.
///
/// The shapes are baked in from the official shape data (see
/// `scripts/gen-loading-shapes.mjs`). The morph follows Android's loading
/// indicator: 650ms per shape, a spring (stiffness 200, damping 0.6) drives the
/// morph, and the rotation adds 50° a shape plus 90° per spring step. The
/// browser runs the animation; reduced motion draws one static frame.
#[component]
pub fn LoadingIndicator(
    #[props(default)] variant: LoadingIndicatorVariant,
    #[props(default = 48.0)] size: f64,
    #[props(default)] class: String,
) -> Element {
    let id = use_hook(|| next_id("m3-loading"));
    let shapes_json = use_hook(shapes_json);
    let scale = match variant {
        LoadingIndicatorVariant::Default => INDICATOR_SCALE,
        LoadingIndicatorVariant::Contained => INDICATOR_SCALE * 0.84,
    };
    let initial_d = shape_path(0);
    let class = format!(
        "m3-loading m3-loading--{} {class}",
        variant.class()
    );

    use_effect({
        let id = id.clone();
        move || {
            let body = LOOP_BODY.replace("__SHAPES__", &shapes_json);
            let script = frame_loop(&id, &body);
            // Keep the eval handle alive: dropping it cancels the script.
            spawn(async move {
                if let Err(err) = document::eval(&script).join::<bool>().await {
                    let _ = document::eval(&format!("console.error({:?})", format!("loading indicator: {err}"))).await;
                }
            });
        }
    });

    rsx! {
        div {
            id: "{id}",
            "data-m3-frame-loop": "true",
            class,
            role: "progressbar",
            "aria-label": "Loading",
            style: "width: {size}px; height: {size}px;",
            svg { class: "m3-loading__svg", view_box: "-0.5 -0.5 1 1",
                g { id: "{id}-rot",
                    path {
                        id: "{id}-path",
                        d: initial_d,
                        fill: "currentColor",
                        transform: "scale({scale})",
                    }
                }
            }
        }
    }
}

/// Per-frame update, run in the browser. It reads the shape data from `__SHAPES__`
/// and keeps the spring state on `window` so the animation survives re-renders.
const LOOP_BODY: &str = r#"
const shapes = __SHAPES__;
const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;
const store = (window.__m3LoadingState = window.__m3LoadingState || {});
const S = (store[id] = store[id] || { pos: 0, vel: 0, target: 1, morphTarget: 1, elapsed: 0, lastTs: 0, prevCycle: 0 });
const k = 200, c = 0.6 * 2 * Math.sqrt(k);
let dt = 0;
if (S.lastTs === 0) S.lastTs = ts;
if (!reduced) dt = Math.min((ts - S.lastTs) / 1000, 0.1);
S.lastTs = ts;
let rotation = 0;
if (dt > 0) {
  S.elapsed += dt * 1000;
  const cycle = Math.floor(S.elapsed / 650);
  if (cycle > S.prevCycle) {
    S.morphTarget += cycle - S.prevCycle;
    S.target = S.morphTarget;
    S.prevCycle = cycle;
  }
  const fraction = (S.elapsed % 650) / 650;
  const sub = dt / 12;
  for (let i = 0; i < 12; i++) {
    const accel = -k * (S.pos - S.target) - c * S.vel;
    S.vel += accel * sub;
    S.pos += S.vel * sub;
  }
  const base = S.morphTarget - 1;
  const perShape = S.pos - base;
  rotation = ((50 + 90) * base + 50 * fraction + 90 * perShape) % 360;
}
const n = shapes.length;
const idx = Math.floor(S.pos);
const from = ((idx % n) + n) % n;
const to = (from + 1) % n;
const t = Math.max(0, Math.min(1, S.pos - idx));
const a = shapes[from], b = shapes[to];
let d = "";
for (let i = 0; i < a.length; i++) {
  const x = a[i][0] + (b[i][0] - a[i][0]) * t;
  const y = a[i][1] + (b[i][1] - a[i][1]) * t;
  d += (i ? "L" : "M") + x.toFixed(4) + " " + y.toFixed(4);
}
d += "Z";
const path = document.getElementById(id + "-path");
const rot = document.getElementById(id + "-rot");
if (path) path.setAttribute("d", d);
if (rot) rot.setAttribute("transform", "rotate(" + rotation + ")");
"#;

/// The seven shapes as a JSON array of `[x, y]` pairs.
fn shapes_json() -> String {
    let mut out = String::from("[");
    for (si, shape) in SHAPES.iter().enumerate() {
        if si > 0 {
            out.push(',');
        }
        out.push('[');
        for (pi, [x, y]) in shape.iter().enumerate() {
            if pi > 0 {
                out.push(',');
            }
            out.push_str(&format!("[{x:.4},{y:.4}]"));
        }
        out.push(']');
    }
    out.push(']');
    out
}

/// Path data for one shape, used as the first frame before the browser loop starts.
fn shape_path(index: usize) -> String {
    let mut d = String::new();
    for (i, [x, y]) in SHAPES[index].iter().enumerate() {
        d.push_str(&format!(
            "{}{:.4} {:.4}",
            if i == 0 { "M" } else { "L" },
            x,
            y
        ));
    }
    d.push('Z');
    d
}
