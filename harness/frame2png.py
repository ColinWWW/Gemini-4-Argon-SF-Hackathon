#!/usr/bin/env python3
"""
Convert raw DOOM frame dumps to PNG.
Raw frame format:
  - 64,000 bytes: 320x200 8-bit indexed framebuffer (screens[0])
  - 768 bytes: 256-color palette (RGB triples, 0-255)
Total size: 64,768 bytes.
"""

import sys
import os
from pathlib import Path
from PIL import Image

SCREEN_WIDTH = 320
SCREEN_HEIGHT = 200
SCREEN_SIZE = SCREEN_WIDTH * SCREEN_HEIGHT  # 64,000
PALETTE_SIZE = 256 * 3                       # 768
EXPECTED_SIZE = SCREEN_SIZE + PALETTE_SIZE    # 64,768

def raw_to_png(raw_path: str | Path, output_path: str | Path = None) -> Path:
    raw_path = Path(raw_path)
    if not raw_path.exists():
        raise FileNotFoundError(f"Raw frame not found: {raw_path}")

    data = raw_path.read_bytes()
    if len(data) != EXPECTED_SIZE:
        raise ValueError(
            f"Invalid raw frame size {len(data)} in {raw_path} (expected {EXPECTED_SIZE})"
        )

    screen_data = data[:SCREEN_SIZE]
    palette_data = data[SCREEN_SIZE:EXPECTED_SIZE]

    # Create 8-bit palette image
    img = Image.frombytes("P", (SCREEN_WIDTH, SCREEN_HEIGHT), screen_data)
    img.putpalette(palette_data)

    # Convert to RGB for universal compatibility
    rgb_img = img.convert("RGB")

    if output_path is None:
        output_path = raw_path.with_suffix(".png")
    else:
        output_path = Path(output_path)

    output_path.parent.mkdir(parents=True, exist_ok=True)
    rgb_img.save(output_path, "PNG")
    return output_path

def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <input.raw> [output.png]")
        sys.exit(1)

    raw_input = sys.argv[1]
    out_input = sys.argv[2] if len(sys.argv) > 2 else None

    # Handle numeric frame index or full path
    if raw_input.isdigit():
        frame_num = int(raw_input)
        raw_input = f"frames/c/{frame_num:05d}.raw"

    out = raw_to_png(raw_input, out_input)
    print(f"Converted: {raw_input} -> {out}")

if __name__ == "__main__":
    main()
