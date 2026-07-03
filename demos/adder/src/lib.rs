//! Placeholder interactive demo: two number inputs whose sum updates live.
//!
//! This exists to prove the Rust -> wasm -> `{{ demo(name=...) }}` pipeline
//! end to end. Replace it with real cryptography demos (LWE sampling, an FHE
//! toy, lattice reduction visualizers, ...). The contract the `demo` shortcode
//! expects: a `default` export (the wasm-bindgen init) and a `mount(el)`
//! function that attaches the UI to the given element.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, HtmlInputElement};

/// Attach the demo UI into `root` (the `<div class="demo">` mount point).
#[wasm_bindgen]
pub fn mount(root: HtmlElement) -> Result<(), JsValue> {
    let doc = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| JsValue::from_str("no document"))?;

    root.set_inner_html(""); // clear the fallback content

    let a = number_input(&doc, 2.0)?;
    let b = number_input(&doc, 3.0)?;
    let out = doc.create_element("output")?.dyn_into::<HtmlElement>()?;
    out.set_class_name("demo-output");

    let render = {
        let (a, b, out) = (a.clone(), b.clone(), out.clone());
        Closure::<dyn Fn()>::new(move || {
            let sum = a.value_as_number() + b.value_as_number();
            out.set_text_content(Some(&format!("sum = {sum}")));
        })
    };
    let cb = render.as_ref().unchecked_ref();
    a.set_oninput(Some(cb));
    b.set_oninput(Some(cb));

    root.append_child(&a)?;
    root.append_child(&doc.create_text_node(" + "))?;
    root.append_child(&b)?;
    root.append_child(&doc.create_text_node(" "))?;
    root.append_child(&out)?;

    render.forget(); // keep the closure alive for the page's lifetime
    // trigger an initial render
    let sum = a.value_as_number() + b.value_as_number();
    out.set_text_content(Some(&format!("sum = {sum}")));
    Ok(())
}

fn number_input(doc: &Document, value: f64) -> Result<HtmlInputElement, JsValue> {
    let el = doc.create_element("input")?.dyn_into::<HtmlInputElement>()?;
    el.set_type("number");
    el.set_value_as_number(value);
    Ok(el)
}
