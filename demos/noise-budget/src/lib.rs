//! Interactive LWE "noise budget" demo.
//!
//! We encrypt a single bit under symmetric-key LWE and let the reader dial the
//! error term `e`. Decryption recovers the bit only while `|e| < q/4` — the
//! "noise budget". Push past it and the bit flips. This is the whole reason FHE
//! is hard: every homomorphic operation grows `e`, and you must bootstrap
//! before it eats the budget.
//!
//! Params are deliberately tiny (n = 4, q = 64) so every number on screen is
//! legible. The `demo` shortcode calls `default()` (wasm init) then `mount(el)`.

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};

const N: usize = 4;
const Q: i32 = 64;
const HALF: i32 = Q / 2; // encodes the message bit: μ·⌊q/2⌋
const QUARTER: i32 = Q / 4; // decode succeeds iff |e| < q/4

/// Attach the demo UI into `root` (the `<div class="demo">` mount point).
#[wasm_bindgen]
pub fn mount(root: HtmlElement) -> Result<(), JsValue> {
    let doc = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| JsValue::from_str("no document"))?;
    root.set_inner_html("");

    // --- keygen (deterministic so the on-screen numbers are stable) ---------
    let mut rng = Lcg::new(0x51ED_2701);
    let s: [i32; N] = std::array::from_fn(|_| rng.next_mod(Q));
    let a: [i32; N] = std::array::from_fn(|_| rng.next_mod(Q));
    let as_dot: i32 = (0..N).map(|i| a[i] * s[i]).sum::<i32>().rem_euclid(Q);

    // --- static explanation -------------------------------------------------
    let params = doc.create_element("p")?;
    params.set_inner_html(&format!(
        "n = {N}, q = {Q}. Encrypt μ = 1 as \
         b = ⟨a,s⟩ + e + ⌊q/2⌋ = {as_dot} + e + {HALF} (mod {Q}). \
         Decryption rounds the phase to the nearest multiple of q/2, \
         so it survives iff |e| &lt; q/4 = {QUARTER}."
    ));
    root.append_child(&params)?;

    // --- slider for the error term -----------------------------------------
    let controls = doc.create_element("p")?;
    let label = doc.create_element("label")?;
    label.set_inner_html("error&nbsp;e&nbsp;");
    let slider = doc.create_element("input")?.dyn_into::<HtmlInputElement>()?;
    slider.set_type("range");
    slider.set_min(&(-HALF).to_string());
    slider.set_max(&HALF.to_string());
    slider.set_value("8");
    slider.set_attribute("style", "vertical-align: middle; width: 16rem;")?;
    label.append_child(&slider)?;
    controls.append_child(&label)?;
    root.append_child(&controls)?;

    // --- error-axis bar with the safe zone shaded --------------------------
    let track = doc.create_element("div")?;
    track.set_attribute(
        "style",
        "position: relative; height: 12px; border-radius: 6px; margin: 0.6rem 0 0.2rem; \
         background: var(--surface); border: 1px solid var(--border);",
    )?;
    let safe = doc.create_element("div")?;
    // safe zone |e| < q/4 spans the middle q/2 of the ±q/2 axis => 50% wide
    safe.set_attribute(
        "style",
        "position: absolute; left: 25%; width: 50%; top: 0; bottom: 0; \
         background: var(--note-bg); border-left: 1px solid var(--note-border); \
         border-right: 1px solid var(--note-border);",
    )?;
    track.append_child(&safe)?;
    let marker = doc.create_element("div")?.dyn_into::<HtmlElement>()?;
    marker.set_attribute(
        "style",
        "position: absolute; top: -3px; width: 3px; height: 18px; border-radius: 2px; \
         transform: translateX(-50%);",
    )?;
    track.append_child(&marker)?;
    root.append_child(&track)?;

    let axis = doc.create_element("div")?;
    axis.set_attribute(
        "style",
        "display: flex; justify-content: space-between; font-size: 0.72rem; color: var(--muted);",
    )?;
    axis.set_inner_html(&format!("<span>−{HALF}</span><span>0</span><span>+{HALF}</span>"));
    root.append_child(&axis)?;

    // --- live readout -------------------------------------------------------
    let out = doc.create_element("div")?.dyn_into::<HtmlElement>()?;
    out.set_attribute("style", "margin-top: 0.7rem; line-height: 1.5;")?;
    root.append_child(&out)?;

    // --- render: recompute from the slider and repaint ----------------------
    let render: Rc<dyn Fn()> = Rc::new({
        let slider = slider.clone();
        move || {
            let e = slider.value_as_number() as i32;
            let b = (as_dot + e + HALF).rem_euclid(Q);
            let phase = (b - as_dot).rem_euclid(Q); // = e + q/2 (mod q)
            // round phase to nearest multiple of q/2, mod 2 -> recovered bit
            let bit = (((phase as f64) / (HALF as f64)).round() as i32).rem_euclid(2);
            let ok = bit == 1;

            let pct = (e + HALF) as f64 / Q as f64 * 100.0;
            let color = if ok { "var(--accent)" } else { "var(--warn-border)" };
            let _ = marker.set_attribute(
                "style",
                &format!(
                    "position: absolute; top: -3px; width: 3px; height: 18px; border-radius: 2px; \
                     transform: translateX(-50%); left: {pct:.1}%; background: {color};"
                ),
            );

            let verdict = if ok {
                "<strong style=\"color: var(--accent)\">decrypts to μ̂ = 1 ✓</strong>".to_string()
            } else {
                format!(
                    "<strong style=\"color: var(--warn-border)\">decrypts to μ̂ = {bit} ✗ \
                     — noise exceeded the budget</strong>"
                )
            };
            out.set_inner_html(&format!(
                "e = {e}, &nbsp; b = {b}, &nbsp; phase = b − ⟨a,s⟩ = {phase}<br>{verdict}"
            ));
        }
    });

    // Wire the slider, then paint once for the initial state.
    let cb = {
        let render = render.clone();
        Closure::<dyn Fn()>::new(move || render())
    };
    slider.set_oninput(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    render();
    Ok(())
}

// Minimal linear congruential generator — no external RNG crate needed.
struct Lcg(u32);
impl Lcg {
    fn new(seed: u32) -> Self {
        Lcg(seed)
    }
    fn next_mod(&mut self, m: i32) -> i32 {
        self.0 = self.0.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        ((self.0 >> 16) as i32).rem_euclid(m)
    }
}
