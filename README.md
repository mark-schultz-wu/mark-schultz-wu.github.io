# mark.schultz-wu.com

Personal site — lattice & FHE cryptography and assorted TCS. Built with
[Zola](https://www.getzola.org/) (Rust static site generator), with
author-written Rust → WebAssembly interactive demos.

## Layout

```
content-src/     Authoring source. Plain $..$ / $$..$$ math. EDIT HERE.
content/         Generated from content-src/ by scripts/wrap_math.py. (gitignored)
templates/       Tera templates + shortcodes/ (mi, md, aside, warning, figure, demo).
sass/            Styles (compiled by Zola).
static/          Verbatim assets (files/, images/).
demos/           Rust→wasm crates, one per interactive demo. (see demos/README.md)
scripts/         wrap_math.py (math→shortcodes), dev.py (live-reload poller).
zola.toml        Zola config.
justfile         Task runner.
```

## Why `content-src/` → `content/`

Zola's Markdown engine (pulldown-cmark) mangles `_`/`*` inside `$math$` as
emphasis and (with `smart_punctuation`) rewrites `'`/`"` into curly forms that
break KaTeX (a curly prime loses its glyph). `scripts/wrap_math.py` copies
`content-src/` to `content/`, wrapping every math span in a raw-passthrough
shortcode (`{% mi() %}` / `{% md() %}`) whose body bypasses Markdown — so math
is immune and `smart_punctuation` can stay on globally. **Always edit
`content-src/`; never edit `content/`.**

## Developing

Requires [`just`](https://github.com/casey/just) and a `zola` build that reads
`zola.toml` (this repo uses a from-source build, not a stock release).

```
just serve     # regenerate content/ on change + live-reloading zola server
just build     # wrap math, then `zola build` into public/
just check     # validate internal links (@/blog/*.md) and markup
just demos     # compile demos/*/ to wasm under static/demos/  (needs wasm-pack)
just --list    # all recipes
```

## Deploying

Not yet wired up. GitHub Pages cannot build Zola natively, so go-live needs a
GitHub Actions workflow (build → deploy `public/`) plus switching the Pages
source to "GitHub Actions", and — for the `mark.schultz-wu.com` custom domain —
a populated `CNAME` + DNS. Deferred to cutover.
