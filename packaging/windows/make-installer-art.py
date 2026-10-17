#!/usr/bin/env python3
"""Generates the scrapbook artwork of the Windows installer wizard.

  installer-dialog.bmp  493 x 312  (WixUIDialogBmp: welcome, license and finish pages)
  installer-banner.bmp  493 x 58   (WixUIBannerBmp: top strip of the inner pages)

Needs Pillow and rsvg-convert (librsvg). Run from the repository root:
  python3 packaging/windows/make-installer-art.py
"""
import io, os, subprocess
from PIL import Image, ImageDraw, ImageFont

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
FONTS = os.path.join(ROOT, "assets", "fonts")
INK, SUN, PAPER, CORAL, TAPE = "#1A1A1A", "#FFD93D", "#FFFBEA", "#FF6B6B", "#FFF1A8"

def mark(px):
    png = subprocess.run(["rsvg-convert", "-h", str(px), os.path.join(ROOT, "assets", "app-icon", "khata-mark.svg")],
                         check=True, capture_output=True).stdout
    return Image.open(io.BytesIO(png)).convert("RGBA")

def dots(d, box, step=16):
    x0, y0, x1, y1 = box
    for y in range(y0 + step // 2, y1, step):
        for x in range(x0 + step // 2, x1, step):
            d.ellipse((x - 1, y - 1, x + 1, y + 1), fill="#E6DDB5")

def font(name, size):
    return ImageFont.truetype(os.path.join(FONTS, name), size)

# --- dialog: a yellow notebook cover on the left, dotted paper where the text goes -------------
W, H, COVER = 493, 312, 164
img = Image.new("RGB", (W, H), PAPER)
d = ImageDraw.Draw(img)
dots(d, (COVER, 0, W, H))
d.rectangle((0, 0, COVER, H), fill=SUN)
d.line((COVER, 0, COVER, H), fill=INK, width=3)
m = mark(110)
img.paste(m, ((COVER - m.width) // 2, 48), m)
d.text((COVER // 2, 188), "Khata", font=font("CaveatBrush-Regular.ttf", 44), fill=INK, anchor="mm")
d.text((COVER // 2, 228), "খাতা", font=font("Atma-Bold.ttf", 30), fill=CORAL, anchor="mm")
d.rectangle((14, 258, COVER - 14, 288), fill=TAPE, outline=INK)  # a strip of washi tape
d.text((COVER // 2, 273), "write · doodle · keep", font=font("PatrickHand-Regular.ttf", 17), fill=INK, anchor="mm")
img.save(os.path.join(os.path.dirname(__file__), "installer-dialog.bmp"))

# --- banner: dotted paper, the notebook on the right, an ink rule underneath ---------------------
BW, BH = 493, 58
b = Image.new("RGB", (BW, BH), PAPER)
d = ImageDraw.Draw(b)
dots(d, (0, 0, BW, BH))
m = mark(46)
b.paste(m, (BW - m.width - 14, 6), m)
d.line((0, BH - 2, BW, BH - 2), fill=INK, width=2)
b.save(os.path.join(os.path.dirname(__file__), "installer-banner.bmp"))
print("wrote installer-dialog.bmp and installer-banner.bmp")
