#!/usr/bin/env python3
"""
harness/clip.py - Render side-by-side diff clip between reference and port.

Three side-by-side panels:
  1. Reference frame (320x200 scaled 2x nearest-neighbor)
  2. Port frame      (320x200 scaled 2x nearest-neighbor)
  3. Diff heatmap    (matching pixels dark grey, mismatched pixels bright red, scaled 2x)

Usage:
    python harness/clip.py <ref_dir> <port_dir> [<frame>] [--start <frame>] [--seconds <secs>] [--out <output.mp4>]
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

DEFAULT_CLIP_SECONDS = 2.0
FPS = 35

PANEL_SCALE = 2
PANEL_WIDTH = SCREEN_WIDTH * PANEL_SCALE    # 640
PANEL_HEIGHT = SCREEN_HEIGHT * PANEL_SCALE  # 400
HEADER_HEIGHT = 48                          # 400 + 48 = 448 (multiple of 16 for H.264)

TOTAL_WIDTH = PANEL_WIDTH * 3               # 1920 (multiple of 16)
TOTAL_HEIGHT = PANEL_HEIGHT + HEADER_HEIGHT # 448


def load_raw_frame(raw_path: Path) -> tuple[np.ndarray | None, bytes | None, bool]:
    """
    Loads raw frame file.
    Returns (screen_indices (200, 320), palette (768 bytes), exists_and_valid).
    If missing or invalid, returns (None, None, False).
    """
    if not raw_path.exists():
        return None, None, False

    try:
        data = raw_path.read_bytes()
        if len(data) < SCREEN_SIZE:
            return None, None, False

        screen_bytes = data[:SCREEN_SIZE]
        palette = data[SCREEN_SIZE:EXPECTED_FRAME_SIZE]

        if len(palette) < PALETTE_SIZE:
            palette = palette + bytes(PALETTE_SIZE - len(palette))

        # Convert 0-63 VGA DAC palette to 0-255 if needed
        if palette and max(palette) <= 63:
            palette = bytes(min(255, b * 4) for b in palette)

        indices = np.frombuffer(screen_bytes, dtype=np.uint8).reshape((SCREEN_HEIGHT, SCREEN_WIDTH))
        return indices, palette, True
    except Exception:
        return None, None, False


def indices_to_rgb(indices: np.ndarray | None, palette: bytes | None) -> Image.Image:
    """Converts 8-bit index array with 768-byte palette into PIL RGB Image (or black image if None)."""
    if indices is None or palette is None:
        return Image.new("RGB", (SCREEN_WIDTH, SCREEN_HEIGHT), (0, 0, 0))

    img = Image.frombytes("P", (SCREEN_WIDTH, SCREEN_HEIGHT), indices.tobytes())
    img.putpalette(palette)
    return img.convert("RGB")


def create_diff_heatmap(
    ref_indices: np.ndarray | None,
    port_indices: np.ndarray | None,
    port_valid: bool,
) -> Image.Image:
    """
    Creates a diff heatmap image:
    Matching pixels: dark grey (30, 30, 30)
    Mismatched pixels: bright red (255, 0, 0)
    If port frame is missing, all pixels are mismatched (bright red).
    """
    # Base dark grey for matching pixels
    diff_arr = np.full((SCREEN_HEIGHT, SCREEN_WIDTH, 3), 30, dtype=np.uint8)

    if not port_valid or port_indices is None:
        # Port missing: all pixels mismatched (bright red)
        diff_arr[:, :] = [255, 0, 0]
    elif ref_indices is not None:
        mismatch = (ref_indices != port_indices)
        diff_arr[mismatch] = [255, 0, 0]

    return Image.fromarray(diff_arr, "RGB")


def render_frame_composite(
    ref_img: Image.Image,
    port_img: Image.Image,
    diff_img: Image.Image,
    ref_label: str,
    port_label: str,
    frame_num: int,
    port_valid: bool,
    font: ImageFont.ImageFont,
) -> Image.Image:
    """Composites reference, port, and diff panels with header labels."""
    # 2x nearest-neighbor scaling
    scaled_ref = ref_img.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)
    scaled_port = port_img.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)
    scaled_diff = diff_img.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)

    composite = Image.new("RGB", (TOTAL_WIDTH, TOTAL_HEIGHT), (15, 15, 18))
    composite.paste(scaled_ref, (0, HEADER_HEIGHT))
    composite.paste(scaled_port, (PANEL_WIDTH, HEADER_HEIGHT))
    composite.paste(scaled_diff, (PANEL_WIDTH * 2, HEADER_HEIGHT))

    draw = ImageDraw.Draw(composite)

    # Header labels above each panel
    port_status = f"PORT ({port_label})" if port_valid else f"PORT ({port_label}) - MISSING"
    draw.text((20, 14), f"REFERENCE ({ref_label}) — Frame {frame_num:05d}", fill=(240, 240, 240), font=font)
    draw.text((PANEL_WIDTH + 20, 14), f"{port_status} — Frame {frame_num:05d}", fill=(240, 240, 240) if port_valid else (255, 120, 120), font=font)
    draw.text((PANEL_WIDTH * 2 + 20, 14), "DIFF HEATMAP (Dark Grey = Match, Red = Diff)", fill=(255, 110, 110), font=font)

    # Separator borders
    draw.line([(0, HEADER_HEIGHT - 1), (TOTAL_WIDTH, HEADER_HEIGHT - 1)], fill=(50, 50, 55), width=1)
    draw.line([(PANEL_WIDTH, 0), (PANEL_WIDTH, TOTAL_HEIGHT)], fill=(45, 45, 50), width=1)
    draw.line([(PANEL_WIDTH * 2, 0), (PANEL_WIDTH * 2, TOTAL_HEIGHT)], fill=(45, 45, 50), width=1)

    return composite


def render_clip(
    ref_dir: Path,
    port_dir: Path,
    target_frame: int | None = None,
    output_path: Path | None = None,
    start_frame: int | None = None,
    seconds: float = DEFAULT_CLIP_SECONDS,
) -> Path:
    ref_dir = Path(ref_dir)
    port_dir = Path(port_dir)

    # Discover available frame numbers from reference directory
    ref_raw_files = sorted(ref_dir.glob("*.raw"))
    if not ref_raw_files:
        raise FileNotFoundError(f"No .raw frame files found in reference directory: {ref_dir}")

    frame_nums = [int(p.stem) for p in ref_raw_files if p.stem.isdigit()]
    min_frame = min(frame_nums) if frame_nums else 0
    max_frame = max(frame_nums) if frame_nums else 0
    total_available = max_frame - min_frame + 1

    clip_frames = max(1, int(round(seconds * FPS)))

    if start_frame is not None:
        start = max(min_frame, start_frame)
        end = min(max_frame + 1, start + clip_frames)
    elif target_frame is not None:
        if total_available <= clip_frames:
            start = min_frame
            end = max_frame + 1
        else:
            half_clip = clip_frames // 2
            start = target_frame - half_clip
            end = start + clip_frames

            if start < min_frame:
                start = min_frame
                end = min(max_frame + 1, min_frame + clip_frames)
            elif end > max_frame + 1:
                end = max_frame + 1
                start = max(min_frame, end - clip_frames)
    else:
        start = min_frame
        end = min(max_frame + 1, start + clip_frames)

    actual_frames_count = end - start

    ref_name = Path(str(ref_dir).rstrip("/\\")).name or "ref"
    port_name = Path(str(port_dir).rstrip("/\\")).name or "port"

    if output_path is None:
        runs_dir = Path("runs")
        runs_dir.mkdir(parents=True, exist_ok=True)
        if start_frame is not None or target_frame is None:
            output_path = runs_dir / f"clip_{ref_name}_vs_{port_name}_frame{start:05d}_{int(round(seconds))}s.mp4"
        else:
            output_path = runs_dir / f"clip_{ref_name}_vs_{port_name}_{target_frame:05d}.mp4"
    else:
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

    desc = f"from {start:05d}" if start_frame is not None or target_frame is None else f"centered on {target_frame}"
    print(
        f"Rendering {actual_frames_count}-frame clip (frames {start:05d} to {end - 1:05d}, {actual_frames_count / FPS:.1f}s) "
        f"{desc} at {FPS} fps...",
        file=sys.stderr,
    )
    print(f"Output: {output_path.as_posix()}", file=sys.stderr)

    # Initialize font
    try:
        font = ImageFont.load_default(size=18)
    except TypeError:
        font = ImageFont.load_default()

    # Launch FFmpeg to encode H.264 MP4
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
        for idx, fnum in enumerate(range(start, end)):
            if (idx + 1) % 350 == 0 or (idx + 1) == actual_frames_count:
                print(f"  Processed {idx + 1}/{actual_frames_count} frames...", file=sys.stderr)

            ref_file = ref_dir / f"{fnum:05d}.raw"
            port_file = port_dir / f"{fnum:05d}.raw"

            ref_indices, ref_palette, ref_valid = load_raw_frame(ref_file)
            port_indices, port_palette, port_valid = load_raw_frame(port_file)

            img_ref = indices_to_rgb(ref_indices, ref_palette)
            img_port = indices_to_rgb(port_indices, port_palette)
            img_diff = create_diff_heatmap(ref_indices, port_indices, port_valid)

            composite = render_frame_composite(
                img_ref,
                img_port,
                img_diff,
                ref_name,
                port_name,
                fnum,
                port_valid,
                font,
            )

            proc.stdin.write(composite.tobytes())

        proc.stdin.close()
        stderr_output = proc.stderr.read().decode("utf-8", errors="replace")
        proc.wait()

        if proc.returncode != 0:
            raise RuntimeError(f"FFmpeg failed with code {proc.returncode}:\n{stderr_output}")

    except Exception:
        proc.kill()
        raise

    print(
        f"Rendered: {output_path.as_posix()} ({os.path.getsize(output_path):,} bytes)",
        file=sys.stderr,
    )
    return output_path


def main():
    parser = argparse.ArgumentParser(description="Render side-by-side diff clip between reference and port.")
    parser.add_argument("ref_dir", type=Path, help="Reference frames directory (e.g. frames/c)")
    parser.add_argument("port_dir", type=Path, help="Port frames directory (e.g. frames/odin)")
    parser.add_argument("frame", type=int, nargs="?", default=None, help="Target frame number to center clip on")
    parser.add_argument("--start", type=int, default=None, help="Starting frame number (overrides centered frame)")
    parser.add_argument("--seconds", type=float, default=DEFAULT_CLIP_SECONDS, help="Duration in seconds (default: 2.0)")
    parser.add_argument("--out", type=Path, default=None, help="Output mp4 path (defaults to runs/clip_*.mp4)")

    args = parser.parse_args()
    if args.frame is None and args.start is None:
        args.start = 0

    out = render_clip(
        ref_dir=args.ref_dir,
        port_dir=args.port_dir,
        target_frame=args.frame,
        output_path=args.out,
        start_frame=args.start,
        seconds=args.seconds,
    )
    # Output path to stdout
    print(out.as_posix())


if __name__ == "__main__":
    main()
