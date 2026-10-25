#!/usr/bin/env python3
"""
harness/score.py - Compare raw frame dumps between reference and port.

Usage:
    python harness/score.py <ref_dir> <port_dir> [--out <output_json>]

Outputs JSON to stdout and saves to runs/score_<port_dir_name>_<timestamp>.json with:
  - overall pixel match %
  - frames fully identical
  - frame count in each directory
  - frames missing from either side
  - first divergent frame number (or null)
  - per-frame match % as a list
"""

import argparse
from datetime import datetime
import json
import os
from pathlib import Path
import sys
from typing import Any
import numpy as np

FRAME_PIXELS = 64_000
PALETTE_BYTES = 768
EXPECTED_FRAME_SIZE = FRAME_PIXELS + PALETTE_BYTES  # 64,768


def count_pixel_matches(buf1: bytes, buf2: bytes) -> int:
    """Count matching bytes between two 64,000-byte frame buffers using numpy."""
    if buf1 == buf2:
        return len(buf1)
    a1 = np.frombuffer(buf1, dtype=np.uint8)
    a2 = np.frombuffer(buf2, dtype=np.uint8)
    return int(np.count_nonzero(a1 == a2))


def score_runs(ref_dir: Path, port_dir: Path, output_file: Path | None = None) -> tuple[dict[str, Any], Path]:
    if not ref_dir.exists():
        raise FileNotFoundError(f"Reference directory not found: {ref_dir}")
    if not port_dir.exists():
        raise FileNotFoundError(f"Port directory not found: {port_dir}")

    # Gather *.raw files only, ignoring any .png or other files
    ref_files = {p.name: p for p in sorted(ref_dir.glob("*.raw"))}
    port_files = {p.name: p for p in sorted(port_dir.glob("*.raw"))}

    all_frame_names = sorted(set(ref_files.keys()) | set(port_files.keys()))
    common_frames = sorted(set(ref_files.keys()) & set(port_files.keys()))

    missing_in_port = sorted(list(set(ref_files.keys()) - set(port_files.keys())))
    missing_in_ref = sorted(list(set(port_files.keys()) - set(ref_files.keys())))

    per_frame: dict[str, dict[str, Any]] = {}
    per_frame_match_pct: list[float] = []

    total_matching_pixels = 0
    # Expected total pixels is based on the reference or full set of frames
    expected_frames_count = max(len(ref_files), len(all_frame_names))
    total_possible_pixels = expected_frames_count * FRAME_PIXELS

    palette_mismatches = 0
    frames_fully_identical = 0
    first_divergent_frame: int | None = None

    for fname in all_frame_names:
        stem = Path(fname).stem
        frame_num = int(stem) if stem.isdigit() else None

        if fname not in ref_files or fname not in port_files:
            if first_divergent_frame is None and frame_num is not None:
                first_divergent_frame = frame_num
            per_frame_match_pct.append(0.0)
            per_frame[stem] = {
                "pixel_match_pct": 0.0,
                "palette_match": False,
                "matching_pixels": 0,
                "missing": "port" if fname not in port_files else "ref",
            }
            continue

        ref_data = ref_files[fname].read_bytes()
        port_data = port_files[fname].read_bytes()

        # Extract 64,000 index bytes and 768 palette bytes
        ref_screen = ref_data[:FRAME_PIXELS]
        port_screen = port_data[:FRAME_PIXELS]
        ref_palette = ref_data[FRAME_PIXELS:EXPECTED_FRAME_SIZE]
        port_palette = port_data[FRAME_PIXELS:EXPECTED_FRAME_SIZE]

        # Fast path: full frame byte-equality
        if ref_data == port_data and len(ref_data) >= EXPECTED_FRAME_SIZE:
            matching_pixels = FRAME_PIXELS
            palette_matches = True
        else:
            matching_pixels = count_pixel_matches(ref_screen, port_screen)
            palette_matches = (ref_palette == port_palette)

        if not palette_matches:
            palette_mismatches += 1

        total_matching_pixels += matching_pixels
        match_pct = round((matching_pixels / FRAME_PIXELS) * 100.0, 4) if FRAME_PIXELS > 0 else 0.0
        per_frame_match_pct.append(match_pct)

        is_identical = (matching_pixels == FRAME_PIXELS and palette_matches)
        if is_identical:
            frames_fully_identical += 1
        elif first_divergent_frame is None and frame_num is not None:
            first_divergent_frame = frame_num

        per_frame[stem] = {
            "pixel_match_pct": match_pct,
            "palette_match": palette_matches,
            "matching_pixels": matching_pixels,
        }

    overall_match_pct = (
        round((total_matching_pixels / total_possible_pixels * 100.0), 4)
        if total_possible_pixels > 0
        else 0.0
    )

    results: dict[str, Any] = {
        "ref_dir": ref_dir.as_posix(),
        "port_dir": port_dir.as_posix(),
        "ref_frame_count": len(ref_files),
        "port_frame_count": len(port_files),
        "frame_count_diff": len(port_files) - len(ref_files),
        "missing_in_port": len(missing_in_port),
        "missing_in_ref": len(missing_in_ref),
        "missing_in_port_frames": missing_in_port[:50],
        "missing_in_ref_frames": missing_in_ref[:50],
        "frames_compared": len(common_frames),
        "frames_fully_identical": frames_fully_identical,
        "overall_pixel_match_pct": overall_match_pct,
        "palette_mismatches": palette_mismatches,
        "first_divergent_frame": first_divergent_frame,
        "per_frame_match_pct": per_frame_match_pct,
        "per_frame": per_frame,
    }

    # Determine output file location in runs/
    if output_file is None:
        runs_dir = Path("runs")
        runs_dir.mkdir(parents=True, exist_ok=True)
        port_name = Path(str(port_dir).rstrip("/\\")).name or "port"
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_file = runs_dir / f"score_{port_name}_{timestamp}.json"
    else:
        output_file.parent.mkdir(parents=True, exist_ok=True)

    output_file.write_text(json.dumps(results, indent=2), encoding="utf-8")
    return results, output_file


def main():
    parser = argparse.ArgumentParser(description="Score raw frame dumps between reference and port.")
    parser.add_argument("ref_dir", type=Path, help="Directory containing reference raw frames (e.g. frames/c)")
    parser.add_argument("port_dir", type=Path, help="Directory containing port raw frames (e.g. frames/c_run2)")
    parser.add_argument("--out", type=Path, default=None, help="Output JSON path (defaults to runs/score_<port_name>_<timestamp>.json)")

    args = parser.parse_args()

    results, saved_path = score_runs(args.ref_dir, args.port_dir, args.out)

    # Print summary to stderr for human convenience
    print(f"Comparison report saved to: {saved_path.as_posix()}", file=sys.stderr)
    print(
        f"Overall Match: {results['overall_pixel_match_pct']}% | "
        f"Identical: {results['frames_fully_identical']}/{results['ref_frame_count']} | "
        f"Divergent: {results['first_divergent_frame']}",
        file=sys.stderr,
    )

    # Output JSON to stdout
    print(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()
