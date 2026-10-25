#!/usr/bin/env python3
"""
harness/score.py - Compare raw frame dumps between reference and port.

Usage:
    python harness/score.py <ref_dir> <port_dir> [--out <output_json>]

Outputs JSON to stdout and writes to a file in runs/ containing:
  - overall_match_pct
  - ref_frame_count, port_frame_count, frame_count_diff
  - missing_in_port, missing_in_ref
  - first_divergent_frame
  - palette_mismatches
  - per_frame: { "<frame>": { "pixel_match_pct": ..., "palette_match": ... } }
"""

import argparse
import json
import os
from pathlib import Path
import sys
from typing import Any

FRAME_PIXELS = 64_000
PALETTE_BYTES = 768
EXPECTED_FRAME_SIZE = FRAME_PIXELS + PALETTE_BYTES  # 64,768


def count_pixel_matches(buf1: bytes, buf2: bytes) -> int:
    """Count matching bytes between two 64,000-byte frame buffers."""
    if buf1 == buf2:
        return len(buf1)
    try:
        import numpy as np
        a1 = np.frombuffer(buf1, dtype=np.uint8)
        a2 = np.frombuffer(buf2, dtype=np.uint8)
        return int(np.count_nonzero(a1 == a2))
    except ImportError:
        return sum(1 for b1, b2 in zip(buf1, buf2) if b1 == b2)


def score_runs(ref_dir: Path, port_dir: Path, output_file: Path | None = None) -> dict[str, Any]:
    if not ref_dir.exists():
        raise FileNotFoundError(f"Reference directory not found: {ref_dir}")
    if not port_dir.exists():
        raise FileNotFoundError(f"Port directory not found: {port_dir}")

    # Gather *.raw files only
    ref_files = {p.name: p for p in sorted(ref_dir.glob("*.raw"))}
    port_files = {p.name: p for p in sorted(port_dir.glob("*.raw"))}

    all_frame_names = sorted(set(ref_files.keys()) | set(port_files.keys()))
    common_frames = sorted(set(ref_files.keys()) & set(port_files.keys()))

    missing_in_port = sorted(list(set(ref_files.keys()) - set(port_files.keys())))
    missing_in_ref = sorted(list(set(port_files.keys()) - set(ref_files.keys())))

    per_frame: dict[str, dict[str, Any]] = {}
    total_matching_pixels = 0
    total_possible_pixels = len(common_frames) * FRAME_PIXELS
    palette_mismatches = 0
    first_divergent_frame: str | int | None = None

    for fname in all_frame_names:
        stem = Path(fname).stem
        frame_num = int(stem) if stem.isdigit() else stem

        if fname not in ref_files or fname not in port_files:
            if first_divergent_frame is None:
                first_divergent_frame = frame_num
            continue

        ref_data = ref_files[fname].read_bytes()
        port_data = port_files[fname].read_bytes()

        # Handle size anomalies
        ref_screen = ref_data[:FRAME_PIXELS]
        port_screen = port_data[:FRAME_PIXELS]
        ref_palette = ref_data[FRAME_PIXELS:EXPECTED_FRAME_SIZE]
        port_palette = port_data[FRAME_PIXELS:EXPECTED_FRAME_SIZE]

        matching_pixels = count_pixel_matches(ref_screen, port_screen)
        total_matching_pixels += matching_pixels

        match_pct = (matching_pixels / FRAME_PIXELS) * 100.0 if FRAME_PIXELS > 0 else 0.0
        palette_matches = (ref_palette == port_palette)
        if not palette_matches:
            palette_mismatches += 1

        if (matching_pixels < FRAME_PIXELS or not palette_matches) and first_divergent_frame is None:
            first_divergent_frame = frame_num

        per_frame[stem] = {
            "pixel_match_pct": round(match_pct, 4),
            "palette_match": palette_matches,
            "matching_pixels": matching_pixels,
        }

    overall_match_pct = (
        (total_matching_pixels / total_possible_pixels * 100.0)
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
        "overall_pixel_match_pct": round(overall_match_pct, 4),
        "palette_mismatches": palette_mismatches,
        "first_divergent_frame": first_divergent_frame,
        "per_frame": per_frame,
    }

    # Determine output file location in runs/
    if output_file is None:
        runs_dir = Path("runs")
        runs_dir.mkdir(parents=True, exist_ok=True)
        # Create safe name from directories
        ref_name = ref_dir.name or "ref"
        port_name = port_dir.name or "port"
        output_file = runs_dir / f"score_{ref_name}_vs_{port_name}.json"
    else:
        output_file.parent.mkdir(parents=True, exist_ok=True)

    output_file.write_text(json.dumps(results, indent=2), encoding="utf-8")
    return results, output_file


def main():
    parser = argparse.ArgumentParser(description="Score raw frame dumps between reference and port.")
    parser.add_argument("ref_dir", type=Path, help="Directory containing reference raw frames (e.g. frames/c)")
    parser.add_argument("port_dir", type=Path, help="Directory containing port raw frames (e.g. frames/c_run2)")
    parser.add_argument("--out", type=Path, default=None, help="Output JSON path (defaults to runs/score_<ref>_vs_<port>.json)")

    args = parser.parse_args()

    results, saved_path = score_runs(args.ref_dir, args.port_dir, args.out)

    # Print summary to stderr for human convenience, JSON to stdout
    print(f"Comparison report saved to: {saved_path.as_posix()}", file=sys.stderr)
    print(f"Overall Match: {results['overall_pixel_match_pct']}% | Compared: {results['frames_compared']} | Divergent: {results['first_divergent_frame']}", file=sys.stderr)

    # Output JSON to stdout
    print(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()
