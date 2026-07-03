+++
title = "Authoring kit reference"
date = 2026-07-02
[taxonomies]
tags = ["meta"]
[extra]
hidden = true
+++
A hidden reference page exercising every shortcode in the authoring kit. Not
listed anywhere; reachable only by URL. Use it as a smoke test after template
changes.

# Math

Inline math like $e = mc^2$ and a prime $f'(x)$ (curly-quote-safe), plus a
display block:

$$
\Pr[\,\|e\| > t\,] \le 2\exp\!\left(-\frac{t^2}{2\sigma^2}\right).
$$

Math is authored as plain `$...$` / `$$...$$` in `content-src/` and wrapped into
raw shortcodes by `scripts/wrap_math.py`, so `smart_punctuation` never touches
it.

# Callouts

{% aside(title="Aside") %}
Asides hold tangential remarks. They render **Markdown**, including math such as
$\sigma^2$ and links like [the LWE post](@/blog/lattices-for-programmers-pt1.md).
{% end %}

{% warning() %}
Warnings flag caveats. Math here is also safe: $\|x\|_\infty \le q/2$.
{% end %}

# Figure

{{ figure(src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/93/Lattice_of_the_divisibility_of_60.svg/320px-Lattice_of_the_divisibility_of_60.svg.png", caption="A *figure* with a Markdown caption.", width="240px") }}

# Interactive demo

The demo below is a real Rust → wasm demo (`demos/noise-budget/`) — an LWE
noise-budget visualizer. Build it with `just demos`; until then it degrades
gracefully to the fallback text.

{% demo(name="noise-budget") %}
LWE noise-budget demo (build with `just demos`).
{% end %}

# Footnotes

Footnotes collect at the bottom.[^ex]

[^ex]: Like this one.
