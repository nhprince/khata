#!/usr/bin/env python3
"""Draws background.svg for the macOS DMG window (660 x 400): the Khata scrapbook look.

All text is converted to outlines (no fonts are needed to render the SVG), as generate.py expects.
Needs fontTools:  pip install fonttools
Then run generate.py to turn this SVG into background.tiff and the Finder layout.

    python3 packaging/macos/dmg/make-background.py && python3 packaging/macos/dmg/generate.py
"""
import os
import re

from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
FONTS = os.path.join(ROOT, "assets", "fonts")
INK, SUN, PAPER, CORAL, TAPE, DOT = "#1A1A1A", "#FFD93D", "#FFFBEA", "#E8505B", "#FFF1A8", "#E2D8A8"


def outline(text, font_file, size, x, y, fill, anchor="start"):
    """`text` as one filled <path>, baseline at y (simple left-to-right layout, no shaping needed here)."""
    font = TTFont(os.path.join(FONTS, font_file))
    glyphs, cmap, hmtx = font.getGlyphSet(), font.getBestCmap(), font["hmtx"]
    scale = size / font["head"].unitsPerEm
    width = sum(hmtx[cmap[ord(c)]][0] for c in text) * scale
    pen_x = x - (width / 2 if anchor == "middle" else 0)
    d = []
    for ch in text:
        name = cmap[ord(ch)]
        pen = SVGPathPen(glyphs, ntos=lambda v: f"{v:.2f}")
        glyphs[name].draw(TransformPen(pen, (scale, 0, 0, -scale, pen_x, y)))
        d.append(pen.getCommands())
        pen_x += hmtx[name][0] * scale
    return f'<path fill="{fill}" d="{" ".join(d)}"/>'


mark = open(os.path.join(ROOT, "assets", "app-icon", "khata-mark.svg"), encoding="utf-8").read()
inner = re.search(r"<svg[^>]*>(.*)</svg>", mark, re.S).group(1)
inner = re.sub(r"<title>.*?</title>", "", inner, flags=re.S)

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="660" height="400" viewBox="0 0 660 400">
  <defs>
    <pattern id="dots" width="22" height="22" patternUnits="userSpaceOnUse">
      <circle cx="11" cy="11" r="1.4" fill="{DOT}"/>
    </pattern>
  </defs>
  <rect width="660" height="400" fill="{PAPER}"/>
  <rect width="660" height="400" fill="url(#dots)"/>
  <!-- the yellow notebook cover -->
  <rect width="232" height="400" fill="{SUN}"/>
  <rect x="232" width="3" height="400" fill="{INK}"/>
  <svg x="26" y="86" width="180" height="184" viewBox="40 20 460 470">{inner}</svg>
  {outline("write · doodle · keep", "PatrickHand-Regular.ttf", 17, 116, 322, INK, "middle")}
  <!-- washi tape on the corner -->
  <g transform="rotate(-8 40 40)"><rect x="-6" y="24" width="96" height="26" fill="{TAPE}" stroke="{INK}" stroke-width="1.5"/></g>
  <!-- the title -->
  {outline("Khata", "CaveatBrush-Regular.ttf", 76, 268, 108, INK)}
  {outline("খাতা", "Atma-Bold.ttf", 54, 468, 104, CORAL)}
  <path d="M270 128 q30 -9 60 0 t60 0 t60 0 t60 0" fill="none" stroke="{CORAL}" stroke-width="3.5" stroke-linecap="round"/>
  <!-- the hand-drawn arrow between the two icons -->
  <path d="M398 214 q44 -34 90 -4" fill="none" stroke="{INK}" stroke-width="4" stroke-linecap="round" stroke-dasharray="2 11"/>
  <path d="M480 196 l18 12 l-22 7 z" fill="{INK}"/>
  <!-- the install hint -->
  <rect x="268" y="338" width="360" height="1.5" fill="{INK}"/>
  {outline("DRAG KHATA ONTO APPLICATIONS TO INSTALL", "PatrickHand-Regular.ttf", 15, 268, 364, "#5C5640")}
</svg>
'''
open(os.path.join(HERE, "background.svg"), "w", encoding="utf-8").write(svg)
print("wrote background.svg", len(svg), "bytes")
