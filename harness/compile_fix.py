#!/usr/bin/env python3
"""
harness/compile_fix.py - Compile error loop for Odin port using Gemini 3.8 Flash.

Usage:
    python harness/compile_fix.py [odin]
"""

import argparse
from datetime import datetime
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

from dotenv import load_dotenv
from google import genai

load_dotenv()

MODEL_NAME = "gemini-3.8-flash"
MAX_SAME_ERROR_ATTEMPTS = 5


def run_odin_build(ports_dir: Path) -> tuple[int, str]:
    """Runs odin build and returns (returncode, combined_output)."""
    exe_path = ports_dir / "doom_odin.exe"
    cmd = ["odin", "build", str(ports_dir), f"-out:{exe_path.as_posix()}"]
    proc = subprocess.run(cmd, capture_output=True, text=True)
    output = (proc.stderr + "\n" + proc.stdout).replace("\r\n", "\n")
    return proc.returncode, output


def parse_odin_errors(output: str) -> list[dict]:
    """Parses individual error blocks from Odin compiler output."""
    header_re = re.compile(r'((?:[A-Za-z]:)?[^\n\(]+\.odin)\((\d+):(\d+)\)')
    positions = [m.start() for m in header_re.finditer(output)] + [len(output)]

    errors = []
    for i in range(len(positions) - 1):
        chunk = output[positions[i]:positions[i+1]].strip()
        m = header_re.match(chunk)
        if m:
            raw_path = m.group(1).strip()
            line_num = int(m.group(2))
            col_num = int(m.group(3))
            body = chunk[m.end():].strip()
            first_line = body.splitlines()[0] if body else ""
            errors.append({
                "file_path": raw_path,
                "file_name": Path(raw_path).name,
                "line": line_num,
                "col": col_num,
                "full_error": chunk,
                "first_line": first_line,
            })
    return errors


def extract_code_block(text: str) -> str | None:
    """Extracts code block from model response."""
    # Look for ```odin ... ``` or generic ``` ... ```
    m = re.search(r'```(?:odin)?\s*\n(.*?)\n```', text, re.DOTALL)
    if m:
        return m.group(1)
    # If model returned pure code without fences
    if "package main" in text:
        return text.strip()
    return None


def call_flash_fix(client: genai.Client, file_path: Path, errors: list[dict], all_errors_summary: str) -> tuple[str | None, dict]:
    """Calls gemini-3.8-flash to fix the compiler errors in file_path."""
    content = file_path.read_text(encoding="utf-8")
    err_text = "\n\n".join(e["full_error"] for e in errors[:5])

    prompt = f"""You are an expert systems programmer fixing Odin compilation errors in a Doom source port (`ports/odin`).

File to fix: `{file_path.name}`

Compiler error(s) targeting this file:
```
{err_text}
```

Full compiler context summary:
```
{all_errors_summary[:1000]}
```

Current contents of `{file_path.name}`:
```odin
{content}
```

Rules:
1. Preserve `package main`.
2. Fix the reported compiler error(s) faithfully.
3. Keep all procedures, globals, and types intact. Do NOT delete or stub code.
4. Output the complete updated file content inside a single ```odin ... ``` code block.
"""

    resp = client.models.generate_content(
        model=MODEL_NAME,
        contents=prompt,
        config={
            "temperature": 0.1,
        },
    )

    tokens = {}
    if hasattr(resp, "usage_metadata") and resp.usage_metadata:
        tokens = {
            "prompt_tokens": getattr(resp.usage_metadata, "prompt_token_count", 0),
            "candidates_tokens": getattr(resp.usage_metadata, "candidates_token_count", 0),
            "total_tokens": getattr(resp.usage_metadata, "total_token_count", 0),
        }

    fixed_code = extract_code_block(resp.text)
    return fixed_code, tokens


def main():
    parser = argparse.ArgumentParser(description="Compile and fix Odin port using Gemini 3.8 Flash.")
    parser.add_argument("language", nargs="?", default="odin", choices=["odin"])
    args = parser.parse_args()

    ports_dir = Path("ports") / args.language
    runs_dir = Path("runs")
    runs_dir.mkdir(parents=True, exist_ok=True)
    log_json = runs_dir / f"compile_{args.language}.json"

    # Initialize log
    attempts_log = []
    if log_json.exists():
        try:
            attempts_log = json.loads(log_json.read_text(encoding="utf-8"))
        except Exception:
            attempts_log = []

    client = genai.Client()

    print(f"=== Starting Compile-Fix Loop for {args.language.upper()} using {MODEL_NAME} ===")
    exe_target = ports_dir / "doom_odin.exe"

    last_error_signature = None
    same_error_count = 0
    total_attempts = len(attempts_log)

    while True:
        total_attempts += 1
        print(f"\n--- Attempt {total_attempts} ---")
        print(f"Building {ports_dir.as_posix()}...")

        ret, output = run_odin_build(ports_dir)

        if ret == 0 and exe_target.exists():
            print(f"\nSUCCESS! Build succeeded with code 0.")
            print(f"Executable produced: {exe_target.as_posix()} ({os.path.getsize(exe_target):,} bytes)")
            log_data = {
                "total_attempts": total_attempts,
                "status": "compiled",
                "completed_at": datetime.now().isoformat(),
                "attempts": attempts_log,
            }
            log_json.write_text(json.dumps(log_data, indent=2), encoding="utf-8")
            break

        parsed_errors = parse_odin_errors(output)
        if not parsed_errors:
            print("Build failed but could not parse specific Odin error blocks:")
            print(output[:1000])
            sys.exit(1)

        # Pick the first failing file
        first_err = parsed_errors[0]
        failing_filename = first_err["file_name"]
        failing_file = ports_dir / failing_filename

        if not failing_file.exists():
            print(f"Failing file does not exist at {failing_file.as_posix()}")
            sys.exit(1)

        # Group all errors belonging to this file
        file_errors = [e for e in parsed_errors if e["file_name"] == failing_filename]
        error_sig = f"{failing_filename}:{first_err['line']}:{first_err['first_line']}"

        print(f"Failing file: {failing_filename} ({len(file_errors)} error(s))")
        print(f"Error: {first_err['first_line']}")

        if error_sig == last_error_signature:
            same_error_count += 1
            print(f"Same error encountered {same_error_count} time(s) consecutively.")
            if same_error_count >= MAX_SAME_ERROR_ATTEMPTS:
                print(f"\n[STOP] The same error survived {MAX_SAME_ERROR_ATTEMPTS} attempts:")
                print(f"File: {failing_filename}:{first_err['line']}")
                print(f"Error: {first_err['first_line']}")
                print("Stopping loop as required. Reporting to user.")
                sys.exit(1)
        else:
            same_error_count = 1
            last_error_signature = error_sig

        # Call Gemini 3.8 Flash
        print(f"Sending {failing_filename} and errors to {MODEL_NAME}...")
        summary_of_all = "\n".join(e["full_error"] for e in parsed_errors[:10])
        fixed_code, tokens = call_flash_fix(client, failing_file, file_errors, summary_of_all)

        if not fixed_code:
            print(f"Failed to extract fixed code from model response. Retrying...")
            continue

        # Sanity check: must contain package main
        if "package main" not in fixed_code:
            print("Model response did not contain 'package main'. Skipping application.")
            continue

        # Write fixed file
        failing_file.write_text(fixed_code, encoding="utf-8")
        print(f"Wrote updated {failing_file.as_posix()}")

        # Commit fix on git branch
        commit_msg = f"fix(odin): {first_err['first_line'][:70]}"
        cmd_add = ["git", "add", str(failing_file)]
        cmd_commit = ["git", "commit", "-m", commit_msg]
        subprocess.run(cmd_add, check=True)
        commit_res = subprocess.run(cmd_commit, capture_output=True, text=True)
        if commit_res.returncode == 0:
            print(f"Committed fix: {commit_msg}")
        else:
            print(f"Git commit returned code {commit_res.returncode}: {commit_res.stdout.strip() or commit_res.stderr.strip()}")

        # Record attempt
        attempt_record = {
            "attempt": total_attempts,
            "timestamp": datetime.now().isoformat(),
            "file": failing_filename,
            "line": first_err["line"],
            "error": first_err["first_line"],
            "tokens": tokens,
        }
        attempts_log.append(attempt_record)
        log_json.write_text(json.dumps(attempts_log, indent=2), encoding="utf-8")

        time.sleep(1)


if __name__ == "__main__":
    main()
