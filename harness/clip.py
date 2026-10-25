#!/usr/bin/env python3
"""
harness/clip.py - Render a 2-second clip (70 frames at 35 fps) centered on a divergent frame.

Three side-by-side panels:
  1. Reference frame (320x200 scaled 2x to 640x400)
  2. Port frame      (320x200 scaled 2x to 640x400)
  3. Diff heatmap    (matching pixels dark, mismatched pixels bright red, scaled 2x to 640x400)

Usage:
    python harness/clip.py <ref_dir> <port_dir> <frame> [--out <output.mp4>]
"""

import argparse
import os
from pathlib import Path
import subprocess
import sys
import numpy as np
from PIL import Image, ImageDraw, ImageFont

SCREEN_WIDTH = 320
SCREEN_HEIGHT = 200
SCREEN_SIZE = SCREEN_WIDTH * SCREEN_HEIGHT  # 64,000
PALETTE_SIZE = 768
EXPECTED_FRAME_SIZE = SCREEN_SIZE + PALETTE_SIZE  # 64,768
CLIP_FRAMES = 70  # 2 seconds at 35 fps
FPS = 35
PANEL_SCALE = 2
PANEL_WIDTH = SCREEN_WIDTH * PANEL_SCALE    # 640
PANEL_HEIGHT = SCREEN_HEIGHT * PANEL_SCALE  # 400
TOTAL_WIDTH = PANEL_WIDTH * 3               # 1920
TOTAL_HEIGHT = PANEL_HEIGHT                 # 400


def load_frame(raw_path: Path) -> tuple[np.ndarray, bytes]:
    """
    Loads raw frame file.
    Returns (screen_indices as np.ndarray (200, 320), palette as bytes of 768 elements).
    """
    if not raw_path.exists():
        empty_indices = np.zeros((SCREEN_HEIGHT, SCREEN_WIDTH), dtype=np.uint8)
        empty_palette = bytes(768)
        return empty_indices, empty_palette

    data = raw_path.read_bytes()
    screen_bytes = data[:SCREEN_SIZE]
    palette = data[SCREEN_SIZE:EXPECTED_FRAME_SIZE]

    if len(palette) < 768:
        palette = palette + bytes(768 - len(palette))

    # Convert 0-63 palette if needed (VGA DAC format)
    if max(palette) <= 63:
        palette = bytes(min(255, b * 4) for b in palette)

    indices = np.frombuffer(screen_bytes, dtype=np.uint8).reshape((SCREEN_HEIGHT, SCREEN_WIDTH))
    return indices, palette


def indices_to_rgb(indices: np.ndarray, palette: bytes) -> Image.Image:
    """Converts 8-bit index array with 768-byte palette into PIL RGB Image."""
    img = Image.frombytes("P", (SCREEN_WIDTH, SCREEN_HEIGHT), indices.tobytes())
    img.putpalette(palette)
    return img.convert("RGB")


def create_diff_heatmap(ref_indices: np.ndarray, port_indices: np.ndarray) -> Image.Image:
    """
    Creates a diff heatmap image:
    Matching pixels: dark gray (20, 20, 20)
    Mismatched pixels: bright red (255, 0, 0)
    """
    diff_arr = np.full((SCREEN_HEIGHT, SCREEN_WIDTH, 3), 20, dtype=np.uint8)
    mismatch = (ref_indices != port_indices)
    diff_arr[mismatch] = [255, 0, 0]
    return Image.fromarray(diff_arr, "RGB")


def add_panel_labels(img: Image.Image, ref_label: str, port_label: str, frame_num: int) -> Image.Image:
    """Draws clear panel headers and frame counter."""
    draw = ImageDraw.Draw(img)
    # Simple clear headers
    draw.text((10, 8), f"REFERENCE ({ref_label}) - Frame {frame_num:05d}", fill=(240, 240, 240))
    draw.text((PANEL_WIDTH + 10, 8), f"PORT ({port_label})", fill=(240, 240, 240))
    draw.text((PANEL_WIDTH * 2 + 10, 8), "DIFF HEATMAP (Mismatch = Red)", fill=(255, 80, 80))
    return img


def render_clip(ref_dir: Path, port_dir: Path, target_frame: int, output_path: Path | None = None) -> Path:
    ref_dir = Path(ref_dir)
    port_dir = Path(port_dir)

    # Determine frame range (70 frames centered on target_frame)
    half_clip = CLIP_FRAMES // 2  # 35
    start_frame = max(0, target_frame - half_clip)
    end_frame = start_frame + CLIP_FRAMES

    if output_path is None:
        runs_dir = Path("runs")
        runs_dir.mkdir(parents=True, exist_ok=True)
        ref_name = ref_dir.name or "ref"
        port_name = port_dir.name or "port"
        output_path = runs_dir / f"clip_{ref_name}_vs_{port_name}_{target_frame:05d}.mp4"
    else:
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

    print(f"Rendering {CLIP_FRAMES}-frame clip (frames {start_frame:05d} to {end_frame - 1:05d}) at 35 fps...")
    print(f"Output target: {output_path.as_posix()}")

    # FFmpeg subprocess piping rawvideo rgb24 directly into h264 encoder
    ffmpeg_cmd = [
        "ffmpeg",
        "-y",
        "-f", "rawvideo",
        "-vcodec", "rawvideo",
        "-s", f"{TOTAL_WIDTH}x{TOTAL_HEIGHT}",
        "-pix_fmt", "rgb24",
        "-r", str(FPS),
        "-i", "-",
        "-c:v", "libx264",
        "-preset", "fast",
        "-crf", "18",
        "-pix_fmt", "yuv420p",
        str(output_path),
    ]

    proc = subprocess.Popen(
        ffmpeg_cmd,
        stdin=subprocess.PIPE,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
    )

    try:
        for fnum in range(start_frame, end_frame):
            ref_file = ref_dir / f"{fnum:05d}.raw"
            port_file = port_dir / f"{fnum:05d}.raw"

            ref_indices, ref_palette = load_frame(ref_file)
            port_indices, port_palette = load_frame(port_file)

            # Render 3 panels
            img_ref = indices_to_rgb(ref_indices, ref_palette)
            img_port = indices_to_rgb(port_indices, port_palette)
            img_diff = create_diff_heatmap(ref_indices, port_indices)

            # 2x nearest-neighbor scaling
            scaled_ref = img_ref.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)
            scaled_port = img_port.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)
            scaled_diff = img_diff.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)

            # Composite side-by-side
            composite = Image.new("RGB", (TOTAL_WIDTH, TOTAL_HEIGHT))
            composite.paste(scaled_ref, (0, 0))
            composite.paste(scaled_port, (PANEL_WIDTH, 0))
            composite.paste(scaled_diff, (PANEL_WIDTH * 2, 0))

            # Add labels
            composite = add_panel_labels(composite, ref_dir.name, port_dir.name, fnum)

            proc.stdin.write(composite.tobytes())

        proc.stdin.close()
        stderr_output = proc.stderr.read().decode("utf-8", errors="replace")
        proc.wait()

        if proc.returncode != 0:
            raise RuntimeError(f"FFmpeg failed with code {proc.returncode}:\n{stderr_output}")

    except Exception:
        proc.kill()
        raise

    print(f"Successfully rendered clip: {output_path.as_posix()} ({os.path.getsize(output_path):,} bytes)")
    return output_path


def main():
    parser = argparse.ArgumentParser(description="Render 2-second side-by-side diff clip around a target frame.")
    parser.add_argument("ref_dir", type=Path, help="Reference frames directory (e.g. frames/c)")
    parser.add_argument("port_dir", type=Path, help="Port frames directory (e.g. frames/c_run2)")
    parser.add_argument("frame", type=int, help="Target frame number (e.g. 500)")
    parser.add_argument("--out", type=Path, default=None, help="Output mp4 path (defaults to runs/clip_*.mp4)")

    args = parser.parse_args()
    out = render_clip(args.ref_dir, args.port_dir, args.frame, args.out)
    print(out.as_posix())


if __name__ == "__main__":
    main()
