# Authoring source lives in content-src/ (plain $..$ math).
# `just build` / `just serve` regenerate content/ (math wrapped in shortcodes)
# before invoking zola, so smart_punctuation & robust math work automatically.

# List available recipes.
default:
    @just --list

# Wrap content-src/ math into shortcodes -> content/.
wrap:
    python3 scripts/wrap_math.py

# Build the site into public/.
build: wrap
    zola build

# Live-reloading dev server (regenerates content/ on content-src/ change).
serve:
    python3 scripts/dev.py

# Validate internal links and markup.
check: wrap
    zola check

# Kept separate from `build` so the site builds without the wasm toolchain.
# Compile every Rust crate in demos/ to wasm under static/demos/<name>/.
demos:
    #!/usr/bin/env bash
    set -euo pipefail
    command -v wasm-pack >/dev/null || { echo "wasm-pack not found: cargo install wasm-pack"; exit 1; }
    for d in demos/*/; do
      name=$(basename "$d")
      [ -f "$d/Cargo.toml" ] || continue
      echo "building demo $name"
      wasm-pack build "$d" --release --target web --no-typescript \
        --out-dir "../../static/demos/$name" --out-name "$name"
    done

# Remove generated output.
clean:
    rm -rf public content
