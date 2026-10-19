#!/usr/bin/env python3
"""Audits the licenses of every dependency and writes THIRD_PARTY_LICENSES.txt.

Open-source licenses (MIT, Apache-2.0, BSD, ...) require the copyright notice and license text to
travel with binaries. This script walks the locked dependency graph (`cargo metadata --locked`),
collects each crate's license files, adds the bundled fonts' licenses, and writes one combined file.
It exits non-zero if a dependency uses a license outside the allow-list (so a new copyleft or
unknown license is noticed in CI instead of in a lawsuit).

    python3 packaging/third-party-licenses.py            # write THIRD_PARTY_LICENSES.txt
    python3 packaging/third-party-licenses.py --check    # audit only, write nothing
"""
import glob
import hashlib
import json
import os
import re
import subprocess
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
OUT = os.path.join(ROOT, "THIRD_PARTY_LICENSES.txt")

# Permissive licenses (and MPL-2.0, a file-level weak copyleft, fine for unmodified use of the crate).
ALLOWED = {
    "MIT", "MIT-0", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Zlib", "0BSD", "BSL-1.0",
    "Unicode-3.0", "Unicode-DFS-2016", "CC0-1.0", "Unlicense", "MPL-2.0", "OFL-1.1", "NCSA",
    "LicenseRef-UFL-1.0", "Ubuntu-font-1.0", "CDLA-Permissive-2.0", "Apache-2.0 WITH LLVM-exception",
}
# Reviewed by hand: crate name -> reason.
REVIEWED = {}


def allowed(expr):
    """True if the SPDX expression can be satisfied with allowed licenses only."""
    tokens = re.findall(r"\(|\)|[A-Za-z0-9.+\-:]+", expr.replace("/", " OR "))
    pos = 0

    def peek():
        return tokens[pos] if pos < len(tokens) else None

    def eat():
        nonlocal pos
        pos += 1
        return tokens[pos - 1]

    def atom():
        if peek() == "(":
            eat()
            v = or_expr()
            if peek() == ")":
                eat()
            return v
        ident = eat().rstrip("+")
        if peek() == "WITH":
            eat()
            eat()
        return ident in ALLOWED

    def and_expr():
        v = atom()
        while peek() == "AND":
            eat()
            v = atom() and v
        return v

    def or_expr():
        v = and_expr()
        while peek() == "OR":
            eat()
            v = and_expr() or v
        return v

    return or_expr()


def main():
    check_only = "--check" in sys.argv
    meta = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--format-version", "1"], cwd=ROOT, text=True))
    packages = [p for p in meta["packages"] if p["source"] is not None]  # skip the workspace's own crates
    problems, entries = [], []
    for p in sorted(packages, key=lambda p: (p["name"], p["version"])):
        expr = p.get("license") or ""
        if not expr:
            if p["name"] not in REVIEWED:
                problems.append(f"{p['name']} {p['version']}: no license field")
        elif not allowed(expr) and p["name"] not in REVIEWED:
            problems.append(f"{p['name']} {p['version']}: {expr}")
        entries.append(p)

    print(f"{len(entries)} third-party crates audited")
    if problems:
        print("LICENSES NEEDING REVIEW:")
        for line in problems:
            print("  " + line)
    else:
        print("all licenses are on the allow-list")
    if check_only:
        return 1 if problems else 0

    texts = {}  # sha -> (text, [crates])
    names = {}
    for p in entries:
        base = os.path.dirname(p["manifest_path"])
        files = sorted(f for f in glob.glob(os.path.join(base, "*"))
                       if re.match(r"(?i)(license|licence|copying|notice|unlicense)", os.path.basename(f)) and os.path.isfile(f))
        label = f"{p['name']} {p['version']}"
        names[label] = p.get("license") or "see crate"
        for f in files:
            body = open(f, encoding="utf-8", errors="replace").read().strip()
            if body:
                key = hashlib.sha256(body.encode()).hexdigest()
                texts.setdefault(key, (body, []))[1].append(label)
    fonts = sorted(glob.glob(os.path.join(ROOT, "assets", "fonts", "OFL-*.txt")))

    with open(OUT, "w", encoding="utf-8") as out:
        out.write("THIRD-PARTY LICENSES FOR KHATA\n==============================\n\n")
        out.write("Khata is distributed under MIT OR Apache-2.0 (see LICENSE-MIT, LICENSE-APACHE, NOTICE).\n")
        out.write("It includes the open-source components listed below. Each component's license text follows.\n\n")
        out.write("COMPONENTS\n----------\n")
        for label, expr in names.items():
            out.write(f"{label}  [{expr}]\n")
        out.write("\nBUNDLED FONTS (SIL Open Font License 1.1; texts below)\n------------------------------------------------------\n")
        out.write("Inter, JetBrains Mono, Source Sans 3, Source Serif 4, Patrick Hand, Kalam, Caveat Brush, Atma\n")
        for f in fonts:
            out.write("\n" + "=" * 78 + f"\n{os.path.basename(f)}\n" + "=" * 78 + "\n")
            out.write(open(f, encoding="utf-8", errors="replace").read().strip() + "\n")
        out.write("\n\nLICENSE TEXTS OF THE COMPONENTS\n===============================\n")
        for _, (body, crates) in sorted(texts.items(), key=lambda kv: kv[1][1][0]):
            out.write("\n" + "=" * 78 + "\nUsed by: " + ", ".join(crates) + "\n" + "=" * 78 + "\n" + body + "\n")
    print(f"wrote {os.path.relpath(OUT, ROOT)} ({os.path.getsize(OUT) // 1024} KiB, {len(texts)} distinct license texts)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
