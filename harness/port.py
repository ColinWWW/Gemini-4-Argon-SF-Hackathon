#!/usr/bin/env python3
"""
harness/port.py - Launch and manage single-pass Doom ports to Odin, Zig, or Rust using Gemini 4 Argon.

Usage:
    python harness/port.py odin               # Dry run (counts tokens and estimates cost)
    python harness/port.py odin --launch      # Launch background translation call
    python harness/port.py odin --resume <id> # Resume polling an existing interaction
"""

import argparse
from datetime import datetime
import json
import os
from pathlib import Path
import re
import sys
import time
from typing import Any

import dotenv
from google import genai

# Load environment variables (e.g. GEMINI_API_KEY from .env)
dotenv.load_dotenv()

MODEL_NAME = "gemini-4-argon"
CONTEXT_LIMIT = 1_048_576     # 1M token context window limit
INPUT_PRICE_PER_M = 2.00      # $2.00 per 1M input tokens
OUTPUT_PRICE_PER_M = 10.00    # $10.00 per 1M output tokens
MAX_OUTPUT_TOKENS = 500_000   # Raised comfortably above expected output (~340K tokens)

# Pure data tables excluded from Argon input (generated mechanically by tables_to_odin.py)
EXCLUDED_DATA_FILES = {"tables.c", "info.c"}


def strip_license_header(content: str) -> str:
    """
    Strips the GPL / DOOM Source Code License comment block from the top of the file in memory.
    Does not touch files on disk.
    """
    # Pattern 1: C++ single-line comments // Copyright (C) ... for more details.
    content = re.sub(
        r'//\s*Copyright\s*\(C\)[^\n]*\n(//[^\n]*\n)*?//[^\n]*for more details\.[^\n]*\n',
        '',
        content,
        flags=re.IGNORECASE,
    )
    # Pattern 2: C block comments /* ... Copyright (C) ... for more details ... */
    content = re.sub(
        r'/\*.*?Copyright\s*\(C\).*?for more details.*?\*/\s*',
        '',
        content,
        flags=re.DOTALL | re.IGNORECASE,
    )
    # Clean up any leftover empty comment lines or log/id tags at the top
    content = re.sub(r'//\s*\$Id:.*?\n', '', content)
    content = re.sub(r'//\s*\$Log:.*?\n', '', content)
    return content.lstrip()


def collect_source_files(ref_dir: Path) -> list[tuple[str, str]]:
    """
    Collects only C source and header files (.c, .h, and compat headers alloca.h, values.h).
    Excludes Makefile, ChangeLog, README*, TODO, DOOMLIC.TXT, FILES*, CHANGES.md,
    build outputs (linux/), and pure lookup data tables (tables.c, info.c).
    Strips license header comments in memory only.
    """
    if not ref_dir.exists():
        raise FileNotFoundError(f"Reference directory not found: {ref_dir}")

    source_files: list[tuple[str, str]] = []

    for root, dirs, files in os.walk(ref_dir):
        # Exclude build output directory
        if "linux" in dirs:
            dirs.remove("linux")

        for file_name in sorted(files):
            # Include only .c and .h source files
            if not (file_name.endswith(".c") or file_name.endswith(".h")):
                continue

            # Exclude pure lookup table implementations (headers tables.h and info.h are retained)
            if file_name in EXCLUDED_DATA_FILES:
                continue

            full_path = Path(root) / file_name
            rel_path = full_path.relative_to(ref_dir.parent.parent).as_posix()
            try:
                raw_content = full_path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                raw_content = full_path.read_text(encoding="latin1")

            cleaned_content = strip_license_header(raw_content)
            source_files.append((rel_path, cleaned_content))

    source_files.sort(key=lambda x: x[0])
    return source_files


def build_port_prompt(source_files: list[tuple[str, str]], language: str) -> str:
    """
    Constructs the prompt with source files first and instructions last.
    """
    lang_lower = language.lower()
    lang_title = {
        "odin": "Odin",
        "zig": "Zig",
        "rust": "Rust",
    }.get(lang_lower, language.capitalize())

    prompt_parts: list[str] = [
        "// ==============================================================================",
        "// SOURCE CODEBASE: reference/linuxdoom-1.10 (id Software linuxdoom-1.10 referee)",
        "// Note: License headers have been stripped from these inputs to optimize context.",
        "// ==============================================================================\n",
    ]

    for rel_path, content in source_files:
        prompt_parts.append(f"{rel_path}\n```c\n{content}\n```\n")

    # Instructions last
    instructions = f"""Translate this entire C codebase, the original linuxdoom-1.10 engine, to **{lang_title}** in a single response.

License and Table Notice:
- The GPL license header comment blocks have been removed from the input files above to save context. Every output file you produce must start with a one-line GPL notice instead (e.g. `// Based on id Software's linuxdoom-1.10 (GNU GPL)`).
- Pure lookup data tables (`tables.c` and `info.c`) are omitted from this prompt; only their declarations in `tables.h` and `info.h` are provided. The matching data files will be generated mechanically with identical names and array values. Do NOT write `tables.{lang_lower}` or `info.{lang_lower}`.

Rules:
1. Faithful, not idiomatic. Keep every file, function name, global, and the overall structure. One output file per input file.
2. The code assumes 32-bit pointers. The port is native 64-bit: WAD on-disk structures must use fixed-width types, and never store a pointer in an integer. Fix every such assumption.
3. Translate the headless `i_video.c`, the `i_sound.c` stubs, and the frame dump exactly. The output frame files must be byte-identical in format: 64,000 palette-index bytes plus the 768-byte palette, written to `frames/{lang_lower}/NNNNN.raw`.
4. Same command line: `-iwad <path> -timedemo demo1`.
5. Output every file in full. No placeholders, no "rest unchanged," no TODOs.
6. Format each file as a fenced code block preceded by its path on its own line.
"""

    if lang_lower == "rust":
        instructions += 'Use `unsafe` and `static mut` freely to mirror the C; do not restructure to satisfy the borrow checker.\n'

    instructions += f"""7. Honor the DOOM_FRAME_DIR environment variable for the frame output directory, defaulting to frames/{lang_lower}/, exactly as the reference does.
8. Target 64-bit Windows. On Windows, long is 32-bit but pointers are 64-bit, so any code that stores a pointer in a long or int must be fixed.
9. Preserve the -iwad flag and the fake clock in I_GetTime exactly.
"""

    prompt_parts.append(instructions)
    return "\n".join(prompt_parts)


def parse_output_files(output_text: str, target_dir: Path, lang: str) -> list[Path]:
    """
    Parses generated files from model output where each file is formatted as:
    <path>
    ```<lang>
    <code>
    ```
    """
    target_dir.mkdir(parents=True, exist_ok=True)
    lines = output_text.splitlines()
    written_files: list[Path] = []
    i = 0
    n = len(lines)

    while i < n:
        line = lines[i]
        if line.startswith("```"):
            path_candidate: str | None = None
            fence_rest = line[3:].strip()

            # Check if filename is specified directly on the fence line
            if " " in fence_rest or "=" in fence_rest or ":" in fence_rest:
                m = re.search(
                    r'(?:path|file|filename)?=?[\"\']?([a-zA-Z0-9_\-\./\\]+\.[a-zA-Z0-9_]+)[\"\']?',
                    fence_rest,
                )
                if m:
                    path_candidate = m.group(1)

            # Otherwise look backward for the closest preceding non-empty line
            if not path_candidate:
                j = i - 1
                while j >= 0 and not lines[j].strip():
                    j -= 1
                if j >= 0:
                    cand = lines[j].strip()
                    cand = re.sub(r"^#+\s*", "", cand)
                    cand = re.sub(r"^(?:file|path|filename|source):\s*", "", cand, flags=re.IGNORECASE)
                    cand = cand.strip("`*'\": ")
                    if cand:
                        path_candidate = cand

            # Gather code block content
            i += 1
            code_block: list[str] = []
            while i < n and not lines[i].startswith("```"):
                code_block.append(lines[i])
                i += 1

            if path_candidate:
                cleaned_path = path_candidate.replace("\\", "/").strip()
                # Strip common redundant prefixes
                prefixes_to_strip = [
                    f"ports/{lang}/",
                    f"{lang}/",
                    "reference/linuxdoom-1.10/",
                    "linuxdoom-1.10/",
                    "./",
                    "/",
                ]
                for prefix in prefixes_to_strip:
                    if cleaned_path.startswith(prefix):
                        cleaned_path = cleaned_path[len(prefix):]

                out_file = target_dir / cleaned_path
                out_file.parent.mkdir(parents=True, exist_ok=True)
                out_file.write_text("\n".join(code_block) + "\n", encoding="utf-8")
                written_files.append(out_file)

        i += 1

    return written_files


def serialize_interaction(interaction: Any) -> dict[str, Any]:
    """Helper to convert interaction response to a JSON-serializable dict."""
    if hasattr(interaction, "model_dump"):
        try:
            return interaction.model_dump(mode="json")
        except Exception:
            pass
    if hasattr(interaction, "to_dict"):
        try:
            return interaction.to_dict()
        except Exception:
            pass
    return {"raw": str(interaction)}


def extract_usage(interaction: Any) -> dict[str, Any]:
    """Extracts usage token counts from interaction object."""
    usage_obj = getattr(interaction, "usage", None)
    if not usage_obj:
        return {}

    usage_dict: dict[str, Any] = {}
    for attr in [
        "total_input_tokens",
        "total_output_tokens",
        "total_thought_tokens",
        "total_tokens",
        "total_cached_tokens",
    ]:
        val = getattr(usage_obj, attr, None)
        if val is not None:
            usage_dict[attr] = val
    return usage_dict


def poll_interaction(
    client: genai.Client,
    interaction_id: str,
    language: str,
) -> None:
    """
    Polls an interaction every 15 seconds until completion, then parses files and saves results.
    """
    lang_lower = language.lower()
    ports_dir = Path("ports") / lang_lower
    runs_dir = Path("runs")
    runs_dir.mkdir(parents=True, exist_ok=True)
    json_path = runs_dir / f"port_{lang_lower}.json"

    print(f"Polling interaction {interaction_id} for {lang_lower} every 15 seconds...")

    while True:
        try:
            interaction = client.interactions.get(interaction_id)
        except Exception as e:
            print(f"[{datetime.now().strftime('%H:%M:%S')}] Polling error: {e}. Retrying in 15s...")
            time.sleep(15)
            continue

        status = getattr(interaction, "status", "unknown")
        timestamp = datetime.now().strftime("%H:%M:%S")
        print(f"[{timestamp}] Status: {status}")

        if status == "completed":
            print(f"\nInteraction completed successfully!")

            output_text = getattr(interaction, "output_text", None)
            if not output_text and hasattr(interaction, "steps"):
                text_parts = []
                for step in getattr(interaction, "steps", []):
                    if getattr(step, "type", None) == "model_output":
                        for c in getattr(step, "content", []):
                            if getattr(c, "type", None) == "text" and hasattr(c, "text"):
                                text_parts.append(c.text)
                if text_parts:
                    output_text = "\n".join(text_parts)

            if not output_text:
                print("Warning: Completed interaction returned empty output_text.")
                output_text = ""

            print(f"Parsing generated files into {ports_dir.as_posix()}...")
            written = parse_output_files(output_text, ports_dir, lang_lower)
            print(f"Successfully extracted {len(written)} files into {ports_dir.as_posix()}.")

            usage_info = extract_usage(interaction)
            print("\nToken Usage:")
            for k, v in usage_info.items():
                print(f"  {k}: {v:,}")

            run_data = {
                "language": lang_lower,
                "interaction_id": interaction_id,
                "completed_at": datetime.now().isoformat(),
                "status": status,
                "files_written": [p.as_posix() for p in written],
                "usage": usage_info,
                "output_text": output_text,
                "response": serialize_interaction(interaction),
            }

            json_path.write_text(json.dumps(run_data, indent=2), encoding="utf-8")
            print(f"Saved run report to {json_path.as_posix()}")
            break

        elif status in ("failed", "cancelled"):
            errors = getattr(interaction, "errors", None)
            print(f"\nInteraction finished with status: {status}. Errors: {errors}")
            run_data = {
                "language": lang_lower,
                "interaction_id": interaction_id,
                "failed_at": datetime.now().isoformat(),
                "status": status,
                "errors": str(errors),
                "response": serialize_interaction(interaction),
            }
            json_path.write_text(json.dumps(run_data, indent=2), encoding="utf-8")
            sys.exit(1)

        time.sleep(15)


def main():
    parser = argparse.ArgumentParser(
        description="Launch single-pass long-decode Doom port or dry-run count tokens."
    )
    parser.add_argument(
        "language",
        nargs="?",
        default="odin",
        choices=["odin", "zig", "rust"],
        help="Target programming language (default: odin)",
    )
    parser.add_argument(
        "--launch",
        action="store_true",
        help="Launch the background translation call with gemini-4-argon (default: dry run)",
    )
    parser.add_argument(
        "--resume",
        nargs="?",
        const="AUTO",
        metavar="INTERACTION_ID",
        help="Resume polling an existing interaction ID (or read from runs/port_<lang>_id.txt if omitted)",
    )

    args = parser.parse_args()
    lang = args.language.lower()
    ref_dir = Path("reference/linuxdoom-1.10")

    client = genai.Client()

    # Handle --resume
    if args.resume:
        interaction_id = args.resume
        if interaction_id == "AUTO":
            id_file = Path("runs") / f"port_{lang}_id.txt"
            if not id_file.exists():
                print(f"Error: No existing interaction ID file found at {id_file.as_posix()}.")
                sys.exit(1)
            interaction_id = id_file.read_text(encoding="utf-8").strip()
            print(f"Resuming interaction {interaction_id} from {id_file.as_posix()}")

        poll_interaction(client, interaction_id, lang)
        return

    # Collect source files
    source_files = collect_source_files(ref_dir)
    prompt = build_port_prompt(source_files, lang)

    # Dry run by default
    if not args.launch:
        print(f"=== DRY RUN: {lang.upper()} PORT TOKEN COUNT & COST ESTIMATION ===")
        print(f"Target language:       {lang.capitalize()}")
        print(f"Model:                 {MODEL_NAME}")
        print(f"Source files included: {len(source_files)} (pure data tables.c & info.c excluded)")
        print(f"Total prompt length:   {len(prompt):,} characters")
        print("\nCalling API count_tokens...")

        res = client.models.count_tokens(model=MODEL_NAME, contents=prompt)
        input_tokens = res.total_tokens
        est_output_tokens = input_tokens
        total_tokens_est = input_tokens + est_output_tokens
        headroom = CONTEXT_LIMIT - total_tokens_est

        input_cost = (input_tokens / 1_000_000) * INPUT_PRICE_PER_M
        est_output_cost = (est_output_tokens / 1_000_000) * OUTPUT_PRICE_PER_M
        est_total_cost = input_cost + est_output_cost

        print(f"\n--- Token Verification vs 1,048,576 Limit ---")
        print(f"Input tokens:            {input_tokens:,}  (target < 350,000: {'PASS' if input_tokens < 350000 else 'FAIL'})")
        print(f"Est. output tokens:      ~{est_output_tokens:,}")
        print(f"Total tokens (in + out): ~{total_tokens_est:,} / {CONTEXT_LIMIT:,} limit ({total_tokens_est / CONTEXT_LIMIT * 100:.1f}%)")
        print(f"Headroom for thinking:   ~{headroom:,} tokens ({headroom / CONTEXT_LIMIT * 100:.1f}%)")
        print(f"Max output tokens cap:   {MAX_OUTPUT_TOKENS:,}")

        print(f"\n--- Cost Estimation ---")
        print(f"Input cost ($2/M):       ${input_cost:.4f}")
        print(f"Est. output cost ($10/M): ${est_output_cost:.4f}")
        print(f"Est. total cost:         ${est_total_cost:.4f}")
        print(f"--------------------------------------------------")
        print(f"DRY RUN COMPLETE. Ready for review. Do NOT pass --launch until approved.\n")
        return

    # With --launch:
    print(f"=== LAUNCHING {lang.upper()} PORT ON {MODEL_NAME} ===")
    print(f"Sending {len(source_files)} source files and instructions to Interactions API...")
    runs_dir = Path("runs")
    runs_dir.mkdir(parents=True, exist_ok=True)
    id_file = runs_dir / f"port_{lang}_id.txt"

    interaction = client.interactions.create(
        model=MODEL_NAME,
        input=prompt,
        background=True,
        generation_config={
            "max_output_tokens": MAX_OUTPUT_TOKENS,
            "thinking_level": "medium",
        },
    )

    interaction_id = interaction.id
    print(f"\nInteraction successfully created!")
    print(f"Interaction ID: {interaction_id}")

    id_file.write_text(f"{interaction_id.strip()}\n", encoding="utf-8")
    print(f"Saved interaction ID to {id_file.as_posix()}\n")

    # Start polling
    poll_interaction(client, interaction_id, lang)


if __name__ == "__main__":
    main()
