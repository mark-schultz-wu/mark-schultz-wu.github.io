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
config.toml      Zola config.
justfile         Task runner.
.github/workflows/deploy.yml   Build + publish to GitHub Pages on push to master.
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

Requires [`just`](https://github.com/casey/just) and official
[`zola`](https://www.getzola.org/) **v0.22.1** (any install; a stock release is
fine — the config is standard `config.toml`).

```
just serve     # regenerate content/ on change + live-reloading zola server
just build     # wrap math, then `zola build` into public/
just check     # validate internal links (@/blog/*.md) and markup
just demos     # compile demos/*/ to wasm under static/demos/  (needs wasm-pack)
just --list    # all recipes
```

## Deploying

`.github/workflows/deploy.yml` builds on every push to `master` (installs
official zola v0.22.1, runs `wrap_math.py` + `zola build`) and publishes
`public/` to GitHub Pages. **One-time manual step:** in repo Settings → Pages,
set Source = "GitHub Actions".

Currently soft-launched on the `github.io` URL (`base_url` in `config.toml`).
To move to the `mark.schultz-wu.com` custom domain later: point DNS at GitHub
Pages, set `base_url` to the custom domain, add a `CNAME` file (in `static/`, so
it lands in `public/`), and set the domain in Settings → Pages.

The wasm demos (`just demos`) are **not** built in CI yet — they only appear on
a hidden page, so they're deferred. Add a `wasm-pack` build step before
`zola build` when a demo goes on a public post.
