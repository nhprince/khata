#!/usr/bin/env python3
"""Sanity-checks the website in site/ before it is deployed.

  - every local link, script, stylesheet, image and font that the HTML/CSS mentions exists,
  - every #anchor points at an id on the page,
  - nothing the Content-Security-Policy in site/_headers would block (inline <script>, inline <style>,
    style="..." attributes, inline event handlers),
  - the files the host needs (404 page, robots.txt, sitemap.xml, _headers) are present.

    python3 packaging/check-site.py
"""
import os
import re
import sys
from html.parser import HTMLParser

SITE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "site"))
problems = []


class Page(HTMLParser):
    def __init__(self, name):
        super().__init__()
        self.name, self.refs, self.ids, self.in_script = name, [], set(), False

    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if "id" in a:
            self.ids.add(a["id"])
        if "style" in a:
            problems.append(f"{self.name}: inline style attribute on <{tag}> (blocked by the CSP)")
        for k in a:
            if k.startswith("on"):
                problems.append(f"{self.name}: inline event handler {k} on <{tag}>")
        if tag == "style":
            problems.append(f"{self.name}: inline <style> block (blocked by the CSP)")
        if tag == "script":
            if "src" not in a and a.get("type") != "application/ld+json":
                problems.append(f"{self.name}: inline <script> (blocked by the CSP)")
        for k in ("href", "src"):
            if k in a and tag in ("a", "link", "script", "img", "source"):
                self.refs.append(a[k])


def local_path(ref):
    ref = ref.split("#")[0].split("?")[0]
    if not ref.startswith("/") or ref.startswith("//"):
        return None
    rel = ref.lstrip("/") or "index.html"
    return os.path.join(SITE, rel)


def check_ref(source, ref, ids=None):
    if ref.startswith("#"):
        if ids is not None and ref[1:] and ref[1:] not in ids:
            problems.append(f"{source}: link {ref} has no matching id")
        return
    p = local_path(ref)
    if p and not os.path.exists(p):
        problems.append(f"{source}: {ref} does not exist in site/")


for required in ("index.html", "404.html", "robots.txt", "sitemap.xml", "_headers", "css/style.css", "js/main.js"):
    if not os.path.exists(os.path.join(SITE, required)):
        problems.append(f"site/{required} is missing")

for page_name in ("index.html", "404.html"):
    path = os.path.join(SITE, page_name)
    if not os.path.exists(path):
        continue
    parser = Page(page_name)
    parser.feed(open(path, encoding="utf-8").read())
    for ref in parser.refs:
        check_ref(page_name, ref, parser.ids)

css = open(os.path.join(SITE, "css", "style.css"), encoding="utf-8").read()
for ref in re.findall(r'url\("?([^")]+)"?\)', css):
    if not ref.startswith("data:"):
        check_ref("css/style.css", ref)

if problems:
    print("SITE PROBLEMS:")
    for p in problems:
        print("  " + p)
    sys.exit(1)
print("site OK:", sum(len(f) for _, _, f in os.walk(SITE)), "files, all references resolve")
