#!/usr/bin/env python3
"""Dev server: regenerate content/ from content-src/ on change, run `zola serve`.

Zola watches content/ and live-reloads. This poller watches content-src/ and
re-runs wrap_math.py whenever a source file changes, so editing content-src/*.md
flows through the shortcode wrap and into the browser automatically.
"""
import os, sys, time, subprocess, signal

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
SRC = os.path.join(ROOT, "content-src")
WRAP = [sys.executable, os.path.join(HERE, "wrap_math.py")]

def snapshot():
    sig = {}
    for root, _, files in os.walk(SRC):
        for f in files:
            p = os.path.join(root, f)
            try: sig[p] = os.stat(p).st_mtime
            except OSError: pass
    return sig

def regen():
    subprocess.run(WRAP, cwd=ROOT, check=True)

def main():
    regen()
    zola = subprocess.Popen(["zola", "serve"], cwd=ROOT)
    def stop(*_):
        zola.terminate(); sys.exit(0)
    signal.signal(signal.SIGINT, stop)
    signal.signal(signal.SIGTERM, stop)
    last = snapshot()
    try:
        while True:
            time.sleep(0.6)
            if zola.poll() is not None:
                break
            cur = snapshot()
            if cur != last:
                print("content-src changed -> regenerating content/")
                try: regen()
                except subprocess.CalledProcessError as e:
                    print("wrap_math failed:", e)
                last = cur
    finally:
        if zola.poll() is None:
            zola.terminate()

if __name__ == "__main__":
    main()
