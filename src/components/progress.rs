use dioxus::prelude::*;

/// Linear progress: `value` is 0.0 to 1.0, or `None` for indeterminate.
///
/// Wavy tracks use a sine wave, 3dp amplitude and 40dp wavelength (20dp while
/// indeterminate). Thick tracks are 8dp, and flat tracks 4dp. Determinate
/// tracks show a 4dp stop dot at the end of the track.
#[component]
pub fn LinearProgress(
    #[props(default)] value: Option<f64>,
    #[props(default)] wavy: bool,
    #[props(default)] thick: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let wavy_class = if wavy { " m3-linear--wavy" } else { "" };
    let thick_class = if thick { " m3-linear--thick" } else { "" };
    let class = format!("m3-linear{wavy_class}{thick_class} {class}");
    let value = value.map(|v| v.clamp(0.0, 1.0));

    match value {
        Some(v) => {
            let percent = format!("{:.2}%", v * 100.0);
            let now = format!("{:.2}", v);
            rsx! {
                div {
                    class: "{class} m3-linear--determinate",
                    role: "progressbar",
                    "aria-label": aria_label,
                    "aria-valuemin": "0",
                    "aria-valuemax": "1",
                    "aria-valuenow": now,
                    div { class: "m3-linear__active", style: "width: {percent};",
                        div { class: "m3-linear__wave" }
                    }
                    div { class: "m3-linear__track" }
                    if v < 1.0 {
                        span { class: "m3-linear__stop", aria_hidden: "true" }
                    }
                }
            }
        }
        None => rsx! {
            div {
                class: "{class} m3-linear--indeterminate",
                role: "progressbar",
                "aria-label": aria_label,
                div { class: "m3-linear__bar m3-linear__bar--1",
                    div { class: "m3-linear__wave" }
                }
                div { class: "m3-linear__bar m3-linear__bar--2",
                    div { class: "m3-linear__wave" }
                }
            }
        },
    }
}

/// Circular progress. Flat rings are 40dp with a 4dp stroke; wavy rings are 48dp
/// with a 1.6dp-amplitude, 15dp-wavelength wave. Thick rings use an 8dp stroke.
///
/// `value` is 0.0 to 1.0, or `None` for indeterminate. Flat indeterminate is a
/// rotating arc. Wavy indeterminate sweeps a wave arc (hold, grow, hold, shrink)
/// in the browser, and wavy determinate is a full wave masked to the active arc.
#[component]
pub fn CircularProgress(
    #[props(default)] value: Option<f64>,
    #[props(default)] wavy: bool,
    #[props(default)] thick: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let geo = CircleGeometry::new(wavy, thick);
    let indeterminate = value.is_none();
    let v = value.unwrap_or(0.0).clamp(0.0, 1.0);
    let wavy_class = if wavy { " m3-circular--wavy" } else { "" };
    let thick_class = if thick { " m3-circular--thick" } else { "" };
    let indeterminate_class = if indeterminate { " m3-circular--indeterminate" } else { "" };
    let class = format!("m3-circular{wavy_class}{thick_class}{indeterminate_class} {class}");
    let now = value.map(|v| format!("{:.2}", v.clamp(0.0, 1.0)));
    let vb = geo.view_box();
    let id = use_hook(|| super::motion::next_id("m3-circular"));
    let mask_id = format!("{id}-mask");

    // Determinate: spanned degrees with the minimum visible arc, as the reference does.
    let min_deg = geo.degrees_for(geo.stroke * 2.0);
    let mut degrees = if indeterminate { 0.0 } else { v * 360.0 };
    if degrees > 0.0 {
        degrees = degrees.max(min_deg);
    }
    let amplitude = if !wavy || degrees <= min_deg * 1.5 || degrees >= 360.0 {
        0.0
    } else {
        geo.amplitude
    };
    let active = geo.arc(0.0, degrees, if degrees < 360.0 { geo.stroke } else { 0.0 });
    let track = geo.arc(degrees, 360.0, if degrees > 0.0 { geo.stroke } else { 0.0 });
    let track_visible = 360.0 - degrees >= min_deg;

    // Indeterminate wavy: a sweep driven in the browser, on the reference's timing.
    let sweep_min = 18.0 + geo.degrees_for(geo.stroke) * 2.0;
    let sweep_max = 280.0 - geo.degrees_for(geo.stroke) * 2.0;
    let sweep_script = if indeterminate && wavy {
        Some(super::motion::frame_loop(&id, &SWEEP_BODY
            .replace("__CX__", &format!("{:.4}", geo.cx()))
            .replace("__CY__", &format!("{:.4}", geo.cy()))
            .replace("__R__", &format!("{:.4}", geo.r))
            .replace("__AMP__", &format!("{:.4}", geo.amplitude))
            .replace("__STROKE__", &format!("{:.4}", geo.stroke))
            .replace("__MIN__", &format!("{:.4}", sweep_min))
            .replace("__MAX__", &format!("{:.4}", sweep_max))))
    } else {
        None
    };
    use_effect({
        let script = sweep_script.clone();
        move || {
            if let Some(script) = script.clone() {
                // Keep the eval handle alive: dropping it cancels the script.
                spawn(async move {
                    if let Err(err) = document::eval(&script).join::<bool>().await {
                        let _ = document::eval(&format!("console.error({:?})", format!("progress: {err}"))).await;
                    }
                });
            }
        }
    });

    rsx! {
        div {
            id: "{id}",
            class,
            role: "progressbar",
            "aria-label": aria_label,
            "aria-valuemin": "0",
            "aria-valuemax": "1",
            "aria-valuenow": now,
            svg { class: "m3-circular__svg", view_box: "{vb}",
                if indeterminate && wavy {
                    path {
                        id: "{id}-active",
                        class: "m3-circular__wave",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "{geo.stroke}",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: geo.wavy_arc(0.0, sweep_min, geo.amplitude),
                    }
                    path {
                        id: "{id}-track",
                        class: "m3-circular__track-arc",
                        fill: "none",
                        stroke_width: "{geo.stroke}",
                        stroke_linecap: "round",
                        d: geo.arc(sweep_min, 360.0, geo.stroke),
                    }
                } else if indeterminate {
                    path {
                        class: "m3-circular__active",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "{geo.stroke}",
                        stroke_linecap: "round",
                        d: geo.arc(0.0, 270.0, 0.0),
                    }
                } else if wavy && amplitude > 0.0 {
                    defs {
                        mask { id: "{mask_id}",
                            path {
                                d: active.clone(),
                                stroke: "white",
                                stroke_width: "{geo.stroke + amplitude + geo.stroke / 2.0}",
                                fill: "none",
                                stroke_linecap: "round",
                            }
                        }
                    }
                    if degrees > 0.0 {
                        g { mask: "url(#{mask_id})",
                            path {
                                class: "m3-circular__wave",
                                d: geo.wavy_arc(0.0, 360.0, amplitude),
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "{geo.stroke}",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                            }
                        }
                    }
                } else if degrees > 0.0 {
                    path {
                        class: "m3-circular__active",
                        d: active.clone(),
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "{geo.stroke}",
                        stroke_linecap: "round",
                    }
                }
                if track_visible && !indeterminate {
                    path {
                        class: "m3-circular__track-arc",
                        d: track,
                        fill: "none",
                        stroke_width: "{geo.stroke}",
                        stroke_linecap: "round",
                    }
                }
            }
        }
    }
}

/// Geometry of a circular indicator, in its own SVG units. The ring radius is
/// half the diameter; the wave and stroke extend outside it, so the view box is
/// padded by amplitude plus half the stroke.
#[derive(Clone, Copy)]
struct CircleGeometry {
    diameter: f64,
    stroke: f64,
    amplitude: f64,
    r: f64,
    pad: f64,
}

impl CircleGeometry {
    fn new(wavy: bool, thick: bool) -> Self {
        let diameter = if thick { 52.0 } else if wavy { 48.0 } else { 40.0 };
        let stroke = if thick { 8.0 } else { 4.0 };
        let amplitude = if wavy { 1.6 * (stroke / 4.0) } else { 0.0 };
        let r = diameter / 2.0;
        let pad = amplitude + stroke / 2.0;
        Self { diameter, stroke, amplitude, r, pad }
    }

    fn cx(&self) -> f64 {
        self.r + self.pad
    }

    fn cy(&self) -> f64 {
        self.r + self.pad
    }

    fn view_box(&self) -> String {
        let size = self.diameter + self.pad * 2.0;
        format!("0 0 {size:.3} {size:.3}")
    }

    /// Degrees that span `size` units along the ring.
    fn degrees_for(&self, size: f64) -> f64 {
        size * (360.0 / (2.0 * std::f64::consts::PI * self.r))
    }

    /// An arc from `start` to `end` degrees, starting at 12 o'clock and running
    /// clockwise, with `gap` units cut from both ends.
    fn arc(&self, start: f64, end: f64, gap: f64) -> String {
        let (mut start, mut end) = (start, end);
        if gap > 0.0 {
            start += self.degrees_for(gap);
            end -= self.degrees_for(gap);
        }
        if end - start >= 360.0 {
            end = start + 359.999;
        }
        let a = self.point(end);
        let b = self.point(start);
        let large = if end - start <= 180.0 { 0 } else { 1 };
        format!(
            "M {:.4} {:.4} A {:.4} {:.4} 0 {} 0 {:.4} {:.4}",
            a.0, a.1, self.r, self.r, large, b.0, b.1
        )
    }

    fn point(&self, deg: f64) -> (f64, f64) {
        let rad = (deg - 90.0).to_radians();
        (self.cx() + self.r * rad.cos(), self.cy() + self.r * rad.sin())
    }

    /// A sine-modulated arc: the radius swings by `amplitude` over a 15-unit
    /// wavelength, as the reference's wavy arc does.
    fn wavy_arc(&self, start: f64, end: f64, amplitude: f64) -> String {
        const STEPS: usize = 200;
        const WAVELENGTH: f64 = 15.0;
        let start_rad = (start - 90.0).to_radians();
        let mut end_rad = (end - 90.0).to_radians();
        if start == end {
            end_rad = start_rad;
        } else if end_rad < start_rad {
            end_rad += std::f64::consts::TAU;
        }
        let total = end_rad - start_rad;
        let wave_count = std::f64::consts::TAU * self.r / WAVELENGTH;
        let phase = std::f64::consts::FRAC_PI_2 * (wave_count - 1.0);
        let mut d = String::new();
        for i in 0..=STEPS {
            let angle = start_rad + (i as f64 / STEPS as f64) * total;
            let radius = self.r - amplitude * (angle * wave_count + phase).sin();
            d.push_str(&format!(
                "{} {:.4},{:.4} ",
                if i == 0 { "M" } else { "L" },
                radius * angle.cos() + self.cx(),
                radius * angle.sin() + self.cy()
            ));
        }
        d
    }
}

/// Per-frame sweep for the wavy indeterminate ring. The arc holds at the minimum,
/// grows over 1575ms, holds at the maximum, and shrinks, on a 4-phase loop.
const SWEEP_BODY: &str = r#"
const t0 = (window.__m3SweepT0 = window.__m3SweepT0 || {})[id] ?? ts;
window.__m3SweepT0[id] = t0;
const dur = 1575, min = __MIN__, max = __MAX__;
const u = (ts - t0) % (dur * 4);
const ease = (p) => p * p * (3 - 2 * p);
let sweep;
if (u < dur) sweep = min;
else if (u < dur * 2) sweep = min + (max - min) * ease((u - dur) / dur);
else if (u < dur * 3) sweep = max;
else sweep = max - (max - min) * ease((u - dur * 3) / dur);
const cx = __CX__, cy = __CY__, r = __R__, amp = __AMP__, stroke = __STROKE__;
const toRad = (deg) => (deg - 90) * Math.PI / 180;
const pointAt = (deg) => [cx + r * Math.cos(toRad(deg)), cy + r * Math.sin(toRad(deg))];
const degreesFor = (size) => size * (360 / (2 * Math.PI * r));
const wavy = (start, end) => {
  const sr = toRad(start);
  let er = toRad(end);
  if (end < start) er += Math.PI * 2;
  const total = er - sr;
  const waves = (2 * Math.PI * r) / 15;
  const phase = (Math.PI / 2) * (waves - 1);
  let d = "";
  for (let i = 0; i <= 200; i++) {
    const angle = sr + (i / 200) * total;
    const radius = r - amp * Math.sin(angle * waves + phase);
    d += (i ? "L" : "M") + " " + (radius * Math.cos(angle) + cx).toFixed(4) + "," + (radius * Math.sin(angle) + cy).toFixed(4) + " ";
  }
  return d;
};
const arcPath = (start, end, gap) => {
  start += degreesFor(gap);
  end -= degreesFor(gap);
  if (end - start >= 360) end = start + 359.999;
  const a = pointAt(end), b = pointAt(start);
  return "M " + a[0].toFixed(4) + " " + a[1].toFixed(4) + " A " + r + " " + r + " 0 " + (end - start <= 180 ? 0 : 1) + " 0 " + b[0].toFixed(4) + " " + b[1].toFixed(4);
};
const active = document.getElementById(id + "-active");
const track = document.getElementById(id + "-track");
if (active) active.setAttribute("d", wavy(0, sweep));
if (track) track.setAttribute("d", arcPath(sweep, 360, stroke));
"#;
