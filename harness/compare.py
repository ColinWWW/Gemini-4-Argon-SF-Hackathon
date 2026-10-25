#!/usr/bin/env python3
"""
harness/compare.py - Render side-by-side comparison video across reference and any number of ports.

Usage:
    python harness/compare.py --ref frames/c --ports frames/odin ..\\hack-rust\\frames\\rust --start 1500 --seconds 60 --out runs/three_way_60s.mp4
"""

import argparse
import os
from pathlib import Path
import subprocess
import sys
import numpy as np
from PIL import Image, ImageDraw, ImageFont

from clip import (
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    SCREEN_SIZE,
    FPS,
    PANEL_SCALE,
    PANEL_WIDTH,
    PANEL_HEIGHT,
    load_raw_frame,
    indices_to_rgb,
)

HEADER_HEIGHT = 48
FOOTER_HEIGHT = 48
TOTAL_HEIGHT = HEADER_HEIGHT + PANEL_HEIGHT + FOOTER_HEIGHT  # 496 (divisible by 16 for H.264)


def derive_label(folder_path: Path, is_ref: bool = False) -> str:
    """Derive human-readable display label from folder name."""
    name = Path(str(folder_path).rstrip("/\\")).name.lower()
    if name == "c" or (is_ref and name == "c"):
        return "Original C (1997)"
    if name == "odin":
        return "Odin (Argon)"
    if name == "rust":
        return "Rust (Argon)"
    if name == "zig":
        return "Zig (Argon)"
    if is_ref:
        return f"Original {name.capitalize()}"
    return f"{name.capitalize()} (Argon)"


def resolve_port_dir(port_dir: Path) -> Path:
    """If a relative port directory like ..\\hack-rust\\frames\\rust does not exist, fallback to frames/<name> if present."""
    if port_dir.exists():
        return port_dir
    fallback = Path("frames") / port_dir.name
    if fallback.exists():
        return fallback
    return port_dir


def render_no_frame_panel(font: ImageFont.ImageFont) -> Image.Image:
    """Renders a 640x400 black panel with 'no frame' centered."""
    panel = Image.new("RGB", (PANEL_WIDTH, PANEL_HEIGHT), (0, 0, 0))
    draw = ImageDraw.Draw(panel)
    text = "no frame"
    bbox = draw.textbbox((0, 0), text, font=font)
    tw = bbox[2] - bbox[0]
    th = bbox[3] - bbox[1]
    draw.text(
        ((PANEL_WIDTH - tw) // 2, (PANEL_HEIGHT - th) // 2),
        text,
        fill=(180, 180, 180),
        font=font,
    )
    return panel


def render_comparison_frame(
    ref_panel: Image.Image,
    port_panels: list[Image.Image],
    ref_label: str,
    port_labels: list[str],
    match_infos: list[tuple[bool, float]],
    frame_num: int,
    font: ImageFont.ImageFont,
    badge_font: ImageFont.ImageFont,
) -> Image.Image:
    """
    Composites reference and N port panels side-by-side:
      - Header above each panel (reference includes Frame NNNNN)
      - 640x400 game panel (2x nearest-neighbor)
      - Footer under each port panel with pixel match % badge (green at 100%, red otherwise)
    """
    num_panels = 1 + len(port_panels)
    total_width = PANEL_WIDTH * num_panels

    composite = Image.new("RGB", (total_width, TOTAL_HEIGHT), (15, 15, 18))
    draw = ImageDraw.Draw(composite)

    # Paste reference panel
    composite.paste(ref_panel, (0, HEADER_HEIGHT))

    # Reference header: label + frame number above reference panel
    ref_header_text = f"{ref_label}   [Frame {frame_num:05d}]"
    draw.text((20, 14), ref_header_text, fill=(240, 240, 240), font=font)

    # Paste port panels, headers, and match % badges
    for i, (panel, label, (is_exact_100, pct)) in enumerate(zip(port_panels, port_labels, match_infos), start=1):
        x0 = i * PANEL_WIDTH
        composite.paste(panel, (x0, HEADER_HEIGHT))

        # Port header label
        draw.text((x0 + 20, 14), label, fill=(240, 240, 240), font=font)

        # Vertical separator between panels
        draw.line([(x0, 0), (x0, TOTAL_HEIGHT)], fill=(45, 45, 50), width=1)

        # Badge under port panel
        badge_bg = (28, 140, 60) if is_exact_100 else (195, 38, 38)
        badge_border = (60, 200, 95) if is_exact_100 else (245, 80, 80)
        pct_str = "100.0%" if is_exact_100 else f"{pct:.2f}%"
        badge_text = f"Match: {pct_str}"

        bbox = draw.textbbox((0, 0), badge_text, font=badge_font)
        tw = bbox[2] - bbox[0]
        th = bbox[3] - bbox[1]

        pad_x, pad_y = 18, 6
        bw = tw + pad_x * 2
        bh = th + pad_y * 2
        bx0 = x0 + (PANEL_WIDTH - bw) // 2
        by0 = HEADER_HEIGHT + PANEL_HEIGHT + (FOOTER_HEIGHT - bh) // 2
        bx1 = bx0 + bw
        by1 = by0 + bh

        draw.rounded_rectangle([bx0, by0, bx1, by1], radius=8, fill=badge_bg, outline=badge_border, width=1)
        draw.text((bx0 + pad_x, by0 + pad_y - 1), badge_text, fill=(255, 255, 255), font=badge_font)

    # Horizontal separator lines for header and footer
    draw.line([(0, HEADER_HEIGHT - 1), (total_width, HEADER_HEIGHT - 1)], fill=(50, 50, 55), width=1)
    draw.line(
        [(0, HEADER_HEIGHT + PANEL_HEIGHT), (total_width, HEADER_HEIGHT + PANEL_HEIGHT)],
        fill=(50, 50, 55),
        width=1,
    )

    return composite


def render_compare_video(
    ref_dir: Path,
    port_dirs: list[Path],
    start_frame: int = 0,
    seconds: float = 60.0,
    output_path: Path = Path("runs/three_way_60s.mp4"),
) -> Path:
    ref_dir = Path(ref_dir)
    raw_port_dirs = [Path(p) for p in port_dirs]
    resolved_port_dirs = [resolve_port_dir(p) for p in raw_port_dirs]

    ref_label = derive_label(ref_dir, is_ref=True)
    port_labels = [derive_label(p, is_ref=False) for p in raw_port_dirs]

    total_frames = max(1, int(round(seconds * FPS)))
    start = max(0, start_frame)
    end = start + total_frames

    output_path = Path(output_path)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    num_panels = 1 + len(resolved_port_dirs)
    total_width = PANEL_WIDTH * num_panels

    try:
        font = ImageFont.load_default(size=18)
        badge_font = ImageFont.load_default(size=16)
    except TypeError:
        font = ImageFont.load_default()
        badge_font = ImageFont.load_default()

    no_frame_panel = render_no_frame_panel(font)

    print(
        f"Rendering {total_frames}-frame comparison ({seconds:.1f}s at {FPS} fps, frames {start:05d}..{end - 1:05d})",
        file=sys.stderr,
    )
    print(f"Panels ({num_panels}): {[ref_label] + port_labels}", file=sys.stderr)
    print(f"Resolution: {total_width}x{TOTAL_HEIGHT} -> {output_path.as_posix()}", file=sys.stderr)

    ffmpeg_cmd = [
        "ffmpeg",
        "-y",
        "-f", "rawvideo",
        "-vcodec", "rawvideo",
        "-s", f"{total_width}x{TOTAL_HEIGHT}",
        "-pix_fmt", "rgb24",
        "-r", str(FPS),
        "-i", "-",
        "-c:v", "libx264",
        "-preset", "fast",
        "-crf", "23",
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
            if (idx + 1) % 350 == 0 or (idx + 1) == total_frames:
                print(f"  Processed {idx + 1}/{total_frames} frames...", file=sys.stderr)

            ref_file = ref_dir / f"{fnum:05d}.raw"
            ref_indices, ref_palette, ref_valid = load_raw_frame(ref_file)

            if ref_valid and ref_indices is not None:
                ref_img = indices_to_rgb(ref_indices, ref_palette)
                ref_panel = ref_img.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST)
            else:
                ref_panel = no_frame_panel

            port_panels: list[Image.Image] = []
            match_infos: list[tuple[bool, float]] = []

            for pdir in resolved_port_dirs:
                pfile = pdir / f"{fnum:05d}.raw"
                p_indices, p_palette, p_valid = load_raw_frame(pfile)

                if not p_valid or p_indices is None:
                    port_panels.append(no_frame_panel)
                    match_infos.append((False, 0.0))
                else:
                    p_img = indices_to_rgb(p_indices, p_palette)
                    port_panels.append(p_img.resize((PANEL_WIDTH, PANEL_HEIGHT), Image.Resampling.NEAREST))

                    if ref_valid and ref_indices is not None:
                        if np.array_equal(ref_indices, p_indices):
                            match_infos.append((True, 100.0))
                        else:
                            matching = int(np.count_nonzero(ref_indices == p_indices))
                            pct = (matching / SCREEN_SIZE) * 100.0
                            # Prevent rounding to 100.00% if not all pixels match
                            pct = min(pct, 99.99)
                            match_infos.append((False, pct))
                    else:
                        match_infos.append((False, 0.0))

            composite = render_comparison_frame(
                ref_panel=ref_panel,
                port_panels=port_panels,
                ref_label=ref_label,
                port_labels=port_labels,
                match_infos=match_infos,
                frame_num=fnum,
                font=font,
                badge_font=badge_font,
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
    parser = argparse.ArgumentParser(
        description="Render a side-by-side comparison video with reference and any number of ports."
    )
    parser.add_argument("--ref", type=Path, required=True, help="Reference frames directory (e.g. frames/c)")
    parser.add_argument(
        "--ports",
        type=Path,
        nargs="+",
        required=True,
        help="One or more port frame directories (e.g. frames/odin frames/rust)",
    )
    parser.add_argument("--start", type=int, default=0, help="Starting frame number (default: 0)")
    parser.add_argument("--seconds", type=float, default=60.0, help="Duration in seconds (default: 60.0)")
    parser.add_argument(
        "--out",
        type=Path,
        default=Path("runs/three_way_60s.mp4"),
        help="Output MP4 file path",
    )

    args = parser.parse_args()
    out = render_compare_video(
        ref_dir=args.ref,
        port_dirs=args.ports,
        start_frame=args.start,
        seconds=args.seconds,
        output_path=args.out,
    )
    print(out.as_posix())


if __name__ == "__main__":
    main()
