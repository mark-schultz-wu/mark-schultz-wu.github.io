#!/usr/bin/env python3
"""Generate content/ from content-src/ by wrapping math in shortcodes.

Authoring source lives in content-src/ with plain `$..$` / `$$..$$` math.
This copies the tree to content/ (which Zola builds), rewriting every math
span into a raw-passthrough shortcode:

    $x$    ->  {% mi() %}x{% end %}      (inline)
    $$x$$  ->  {% md() %}x{% end %}      (display)

Because shortcode bodies bypass Markdown, this makes math immune to BOTH
emphasis-mangling (_ * ) and smart_punctuation (' " -- ...), so those can be
enabled globally. Code (fenced + inline), template tags, and link targets are
left untouched.

The mi()/md() shortcodes are treated as terminal (their bodies are already
raw math and are emitted verbatim, which keeps this idempotent). Every OTHER
paired shortcode (aside, warning, figure, demo, ...) has its body recursed
into, so `$x$` written inside an {% aside() %} block is still wrapped -- those
templates re-render the body through the Markdown filter, so unwrapped math
there would be mangled by smart_punctuation. Idempotent.
"""
import re, os, sys, shutil

SRC = "content-src"
DST = "content"

TERMINAL = {"mi", "md"}  # bodies are already-raw math: emit verbatim

# Ordered alternation. `fence`, `tag`, `code` are PROTECTED (emitted verbatim);
# `pair` is a paired shortcode (recurse into its body unless terminal);
# `disp`/`inl` are math spans that get WRAPPED.
TOKEN = re.compile(
    r'(?P<fence>```[\s\S]*?```)'                                              # fenced code
    r'|(?P<pair>\{%\s*(?P<pairname>\w+)\([^)]*\)\s*%\}[\s\S]*?\{%\s*end\s*%\})'  # paired shortcode
    r'|(?P<tag>\{%[\s\S]*?%\}|\{\{[\s\S]*?\}\})'                              # template tag
    r'|(?P<code>`[^`\n]*`)'                                                   # inline code
    r'|(?P<disp>\$\$[\s\S]*?\$\$)'                                            # display math  -> md()
    r'|(?P<inl>\$[^$\n]+?\$)'                                                 # inline math   -> mi()
)

# Split a paired shortcode into (open-tag, body, close-tag) so we can recurse
# the body while leaving the tags untouched.
PAIR_SPLIT = re.compile(r'^(\{%\s*\w+\([^)]*\)\s*%\})([\s\S]*)(\{%\s*end\s*%\})$')

def wrap(body):
    """Wrap math spans in a chunk of Markdown body (no frontmatter)."""
    out, pos = [], 0
    for tok in TOKEN.finditer(body):
        out.append(body[pos:tok.start()])
        d = tok.groupdict()
        if d['disp'] is not None:
            out.append('{% md() %}' + d['disp'][2:-2] + '{% end %}')
        elif d['inl'] is not None:
            out.append('{% mi() %}' + d['inl'][1:-1] + '{% end %}')
        elif d['pair'] is not None and d['pairname'] not in TERMINAL:
            sp = PAIR_SPLIT.match(d['pair'])
            open_t, inner, close_t = sp.group(1, 2, 3)
            out.append(open_t + wrap(inner) + close_t)  # recurse into the body
        else:
            out.append(tok.group(0))  # protected / terminal, verbatim
        pos = tok.end()
    out.append(body[pos:])
    return ''.join(out)

def wrap_doc(text):
    m = re.match(r'^\+\+\+\n[\s\S]*?\n\+\+\+\n', text)
    head, body = (text[:m.end()], text[m.end():]) if m else ('', text)
    return head + wrap(body)

def main():
    if os.path.isdir(DST):
        shutil.rmtree(DST)
    n_md = 0
    for root, _, files in os.walk(SRC):
        rel = os.path.relpath(root, SRC)
        dstroot = os.path.join(DST, rel) if rel != '.' else DST
        os.makedirs(dstroot, exist_ok=True)
        for f in files:
            s, d = os.path.join(root, f), os.path.join(dstroot, f)
            if f.endswith('.md'):
                open(d, 'w').write(wrap_doc(open(s).read()))
                n_md += 1
            else:
                shutil.copy2(s, d)
    return n_md

if __name__ == '__main__':
    print(f"wrapped math in {main()} markdown files: {SRC}/ -> {DST}/")
