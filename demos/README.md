# Interactive Rust → wasm demos

Each subdirectory is a self-contained Rust crate compiled to WebAssembly and
embedded in a post via the `demo` shortcode:

```
{{ demo(name="adder") }}
```

## Convention

```
demos/<name>/            Rust crate: cdylib + wasm-bindgen
  Cargo.toml
  src/lib.rs             must export `mount(root: HtmlElement)`
```

`just demos` compiles every crate here into `static/demos/<name>/`, producing
`<name>.js` (ES-module glue) and `<name>_bg.wasm`. The `demo` shortcode lazily
`import()`s that module when the embed scrolls into view, calls its default
export (wasm init), then calls `mount(el)` with the embed element.

## Building

Requires [`wasm-pack`](https://rustwasm.github.io/wasm-pack/):

```
cargo install wasm-pack   # once
just demos                # builds all demos into static/demos/
```

`static/demos/` is generated — it is gitignored. `just build` does **not**
build the demos (so the site builds without the wasm toolchain); run
`just demos` explicitly, then `just build`.
