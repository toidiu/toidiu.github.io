"""
Generate BRAG icon PNGs: thick black ring, color fill, black "BRAG" text.
Outputs: brag_white.png, brag_red.png, brag_yellow.png, brag_blue.png

Requirements:
    pip install pillow

Usage:
    python make_brag_icons.py
"""

from PIL import Image, ImageDraw, ImageFont

SIZE = 300
CENTER = SIZE / 2

# Ring/fill proportions (scaled from a 150px design, so they scale with SIZE)
R_OUTER = SIZE * 0.50   # outer edge of black ring (= full circle)
R_INNER = SIZE * 0.40   # inner edge of black ring / start of color fill

COLORS = {
    "white": (255, 255, 255),
    "red": (225, 35, 43),
    "yellow": (242, 193, 46),
    "blue": (46, 107, 242),
}

# Common bold font locations (Linux / macOS / Windows). Falls back to default if none found.
FONT_PATHS = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
    "/System/Library/Fonts/Supplemental/Arial Bold.ttf",
    "C:\\Windows\\Fonts\\arialbd.ttf",
]


def get_font(size):
    for path in FONT_PATHS:
        try:
            return ImageFont.truetype(path, size)
        except Exception:
            continue
    return ImageFont.load_default()


def make_icon(fill_color, filename):
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    def circle(r, fill):
        bbox = [CENTER - r, CENTER - r, CENTER + r, CENTER + r]
        draw.ellipse(bbox, fill=fill)

    circle(R_OUTER, (0, 0, 0, 255))        # thick black ring
    circle(R_INNER, fill_color + (255,))   # color fill inside

    font = get_font(int(SIZE * 0.16))      # scales with icon size
    text = "BRAG"
    spacing = SIZE * 0.013

    widths = [draw.textlength(ch, font=font) for ch in text]
    total_width = sum(widths) + spacing * (len(text) - 1)
    bbox = font.getbbox(text)
    text_height = bbox[3] - bbox[1]

    x = CENTER - total_width / 2
    y = CENTER - text_height / 2 - bbox[1]

    for ch, w in zip(text, widths):
        draw.text((x, y), ch, font=font, fill=(0, 0, 0, 255))
        x += w + spacing

    img.save(filename)


if __name__ == "__main__":
    for name, rgb in COLORS.items():
        out = f"brag_{name}.png"
        make_icon(rgb, out)
        print(f"saved {out}")
