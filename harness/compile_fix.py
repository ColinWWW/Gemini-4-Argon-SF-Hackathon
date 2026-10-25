#!/usr/bin/env python3
"""
harness/compile_fix.py - Compile error loop for Odin and Rust ports using Gemini 3.8 Flash.

Usage:
    python -u harness/compile_fix.py [odin|rust]
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
from google.genai import types

load_dotenv()

MODEL_NAME = "gemini-3.8-flash"
MAX_SAME_ERROR_ATTEMPTS = 5
MAX_STALLED_ROUNDS = 5


def log(msg: str):
    print(msg, flush=True)


def get_cargo_cmd_and_env() -> tuple[list[str], dict]:
    cargo_exe = "cargo"
    cargo_bin = Path.home() / ".cargo" / "bin"
    env = os.environ.copy()
    if (cargo_bin / "cargo.exe").exists():
        cargo_exe = str(cargo_bin / "cargo.exe")
        env["PATH"] = str(cargo_bin) + os.pathsep + env.get("PATH", "")
    elif cargo_bin.exists():
        env["PATH"] = str(cargo_bin) + os.pathsep + env.get("PATH", "")
    return [cargo_exe, "build", "--release", "--message-format=json"], env


# ==================== ODIN ====================

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


def call_flash_fix_odin(client: genai.Client, file_path: Path, errors: list[dict], all_errors_summary: str) -> tuple[str | None, dict]:
    """Calls gemini-3.8-flash to fix the compiler errors in file_path (Odin)."""
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
5. In Odin, `libc.sprintf` is not in `core:c/libc`. Replace with `libc.snprintf(dest, size, ...)` or `fmt.bprintf(buf[:], ...)`. Formatted strings must match C byte-for-byte! Note that C's "%.3d" is zero-padded 3-digit integer, which in Odin fmt is "%03d".
6. In Odin, raw pointers `^T` cannot be indexed with `p[i]`. Use multi-pointers `[^]T` (or cast `([^]T)(p)[i]`).
7. In Odin, `libc.fseek` takes `(stream: ^libc.FILE, offset: libc.long, whence: libc.Whence)`. Pass `.SET`, `.CUR`, or `.END` for whence, and cast offset to `libc.long(...)`.
8. `MAXINT` is `max(i32)`.
9. Comparing `sfxenum_t` to 0: compare with `.sfx_None` or `sfxenum_t(0)`.
"""

    resp = client.models.generate_content(
        model=MODEL_NAME,
        contents=prompt,
        config=types.GenerateContentConfig(
            temperature=0.1,
            thinking_config=types.ThinkingConfig(thinking_level="low"),
        ),
    )

    tokens = {}
    if hasattr(resp, "usage_metadata") and resp.usage_metadata:
        tokens = {
            "prompt_tokens": getattr(resp.usage_metadata, "prompt_token_count", 0),
            "candidates_tokens": getattr(resp.usage_metadata, "candidates_token_count", 0),
            "total_tokens": getattr(resp.usage_metadata, "total_token_count", 0),
        }

    m = re.search(r'```(?:odin)?\s*\n(.*?)\n```', resp.text, re.DOTALL)
    fixed_code = m.group(1) if m else (resp.text.strip() if "package main" in resp.text else None)
    return fixed_code, tokens


# ==================== RUST ====================

def run_rust_build(ports_dir: Path) -> tuple[int, list[dict], Path]:
    """Runs cargo build --release and returns (returncode, parsed_errors, exe_path)."""
    cmd, env = get_cargo_cmd_and_env()
    proc = subprocess.run(cmd, cwd=str(ports_dir), capture_output=True, text=True, env=env)

    errors = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            data = json.loads(line)
        except Exception:
            continue
        if data.get("reason") != "compiler-message":
            continue
        msg = data.get("message", {})
        if msg.get("level") != "error":
            continue

        spans = msg.get("spans", [])
        primary_span = next((s for s in spans if s.get("is_primary")), spans[0] if spans else None)

        file_name = None
        line_num = 0
        col_num = 0
        if primary_span:
            p = Path(primary_span.get("file_name", ""))
            file_name = p.name
            line_num = primary_span.get("line_start", 0)
            col_num = primary_span.get("column_start", 0)

        rendered = msg.get("rendered", "")
        first_line = msg.get("message", "")

        errors.append({
            "file_name": file_name,
            "line": line_num,
            "col": col_num,
            "full_error": rendered.strip(),
            "first_line": first_line.strip(),
        })

    exe_target = ports_dir / "target" / "release" / ("linuxdoom.exe" if os.name == "nt" else "linuxdoom")
    return proc.returncode, errors, exe_target


def call_flash_fix_rust(client: genai.Client, file_path: Path, errors: list[dict], all_errors_summary: str) -> tuple[str | None, dict]:
    """Calls gemini-3.8-flash to fix ALL compiler errors in file_path (Rust)."""
    content = file_path.read_text(encoding="utf-8")
    err_text = "\n\n".join(e["full_error"] for e in errors)

    prompt = f"""You are an expert systems programmer fixing Rust compilation errors in a Doom source port (`ports/rust/src`).

File to fix: `{file_path.name}`

Compiler error(s) targeting this file ({len(errors)} error(s)):
```
{err_text}
```

Context of all crate errors:
```
{all_errors_summary[:2000]}
```

Current contents of `{file_path.name}`:
```rust
{content}
```

Rules:
1. Fix ALL reported compiler error(s) in `{file_path.name}` faithfully.
2. Output the COMPLETE updated file content inside a single ```rust ... ``` code block. No placeholders, no omitted parts!
3. Maintain faithful matching of the original C Doom code semantics. Keep all procedures, globals, and types intact.
4. In Rust, you can use `unsafe`, `static mut`, pointer casts, and imports (`use crate::<module>::*;`) freely as needed to mirror C.
5. If a function parameter shadows a static variable (error E0530), rename the function parameter slightly (e.g. `_linedef` or `line`) to resolve the collision.
6. If an identifier or helper is missing:
   - Check if it belongs to another module and add the appropriate `use crate::<module>::*;` or define/expose it.
   - For example: `gameaction` is in `crate::g_game::gameaction`
   - `PST_LIVE`, `PST_DEAD`, `PST_REBORN` are in `crate::d_player::*`
   - `CF_NOMOMENTUM` is in `crate::d_player::*`
   - `SlopeDiv` is in `crate::r_main::SlopeDiv`
   - `plyr` vs `plr`: check how the static is named in this module or related modules and use the defined name.
   - For sector pointer field access like `(*sector).lightlevel`: ensure sector type definitions or pointer types match.
"""

    resp = client.models.generate_content(
        model=MODEL_NAME,
        contents=prompt,
        config=types.GenerateContentConfig(
            temperature=0.1,
            thinking_config=types.ThinkingConfig(thinking_level="low"),
        ),
    )

    tokens = {}
    if hasattr(resp, "usage_metadata") and resp.usage_metadata:
        tokens = {
            "prompt_tokens": getattr(resp.usage_metadata, "prompt_token_count", 0),
            "candidates_tokens": getattr(resp.usage_metadata, "candidates_token_count", 0),
            "total_tokens": getattr(resp.usage_metadata, "total_token_count", 0),
        }

    m = re.search(r'```(?:rust)?\s*\n(.*?)\n```', resp.text, re.DOTALL)
    if m:
        fixed_code = m.group(1)
    else:
        # Fallback if entire response is the code
        fixed_code = resp.text.strip() if ("pub " in resp.text or "use " in resp.text) else None

    return fixed_code, tokens


# ==================== MAIN LOOPS ====================

def run_odin_loop(ports_dir: Path, runs_dir: Path, client: genai.Client):
    log_json = runs_dir / "compile_odin.json"
    exe_target = ports_dir / "doom_odin.exe"

    attempts_log = []
    if log_json.exists():
        try:
            attempts_log = json.loads(log_json.read_text(encoding="utf-8"))
            if not isinstance(attempts_log, list):
                attempts_log = attempts_log.get("attempts", [])
        except Exception:
            attempts_log = []

    last_error_signature = None
    same_error_count = 0
    total_attempts = len(attempts_log)

    while True:
        total_attempts += 1
        log(f"\n--- Odin Attempt {total_attempts} ---")
        log(f"Building {ports_dir.as_posix()}...")

        ret, output = run_odin_build(ports_dir)

        if ret == 0 and exe_target.exists():
            log(f"\nSUCCESS! Odin build succeeded with code 0.")
            log(f"Executable produced: {exe_target.as_posix()} ({os.path.getsize(exe_target):,} bytes)")
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
            log("Build failed but could not parse specific Odin error blocks:")
            log(output[:1000])
            sys.exit(1)

        first_err = parsed_errors[0]
        failing_filename = first_err["file_name"]
        failing_file = ports_dir / failing_filename

        if not failing_file.exists():
            log(f"Failing file does not exist at {failing_file.as_posix()}")
            sys.exit(1)

        file_errors = [e for e in parsed_errors if e["file_name"] == failing_filename]
        error_sig = f"{failing_filename}:{first_err['line']}:{first_err['first_line']}"

        log(f"Failing file: {failing_filename} ({len(file_errors)} error(s))")
        log(f"Error: {first_err['first_line']}")

        if error_sig == last_error_signature:
            same_error_count += 1
            log(f"Same error encountered {same_error_count} time(s) consecutively.")
            if same_error_count >= MAX_SAME_ERROR_ATTEMPTS:
                log(f"\n[STOP] The same error survived {MAX_SAME_ERROR_ATTEMPTS} attempts:")
                log(f"File: {failing_filename}:{first_err['line']}")
                log(f"Error: {first_err['first_line']}")
                sys.exit(1)
        else:
            same_error_count = 1
            last_error_signature = error_sig

        log(f"Sending {failing_filename} and errors to {MODEL_NAME}...")
        summary_of_all = "\n".join(e["full_error"] for e in parsed_errors[:10])
        fixed_code, tokens = call_flash_fix_odin(client, failing_file, file_errors, summary_of_all)

        if not fixed_code or "package main" not in fixed_code:
            log("Invalid model response. Skipping application.")
            continue

        failing_file.write_text(fixed_code, encoding="utf-8")
        log(f"Wrote updated {failing_file.as_posix()}")

        commit_msg = f"fix(odin): {first_err['first_line'][:70]}"
        subprocess.run(["git", "add", str(failing_file)], check=True)
        commit_res = subprocess.run(["git", "commit", "-m", commit_msg], capture_output=True, text=True)
        if commit_res.returncode == 0:
            log(f"Committed fix: {commit_msg}")

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


def run_rust_loop(ports_dir: Path, runs_dir: Path, client: genai.Client):
    log_json = runs_dir / "compile_rust.json"

    attempts_log = []
    if log_json.exists():
        try:
            attempts_log = json.loads(log_json.read_text(encoding="utf-8"))
            if not isinstance(attempts_log, list):
                attempts_log = attempts_log.get("attempts", [])
        except Exception:
            attempts_log = []

    total_attempts = len(attempts_log)
    min_errors_seen = float("inf")
    stalled_rounds = 0
    initial_errors = None
    halving_milestone = None

    while True:
        total_attempts += 1
        log(f"\n--- Rust Attempt {total_attempts} ---")
        log(f"Building {ports_dir.as_posix()} (cargo build --release)...")

        ret, parsed_errors, exe_target = run_rust_build(ports_dir)

        if ret == 0 and exe_target.exists():
            log(f"\n==================================================")
            log(f"SUCCESS! Rust build succeeded with code 0.")
            log(f"Executable produced: {exe_target.as_posix()} ({os.path.getsize(exe_target):,} bytes)")
            log(f"==================================================\n")
            log_data = {
                "total_attempts": total_attempts,
                "status": "compiled",
                "completed_at": datetime.now().isoformat(),
                "attempts": attempts_log,
            }
            log_json.write_text(json.dumps(log_data, indent=2), encoding="utf-8")
            break

        total_errors = len(parsed_errors)
        log(f"Build failed with {total_errors} error(s).")

        if initial_errors is None:
            initial_errors = total_errors
            halving_milestone = initial_errors // 2
            log(f"Initial error count baseline: {initial_errors} (first halving milestone: {halving_milestone})")

        # Check halving progress milestone
        if halving_milestone and total_errors <= halving_milestone:
            log(f"\n>>> [PROGRESS REPORT] Error count halved: now {total_errors} errors (started at {initial_errors}, passed milestone {halving_milestone}) <<<\n")
            while halving_milestone and total_errors <= halving_milestone:
                halving_milestone //= 2

        # Check if error count stopped falling
        if total_errors < min_errors_seen:
            min_errors_seen = total_errors
            stalled_rounds = 0
        else:
            stalled_rounds += 1
            log(f"Error count did not fall this round ({total_errors} >= min {min_errors_seen}). Stalled rounds: {stalled_rounds}/{MAX_STALLED_ROUNDS}")
            if stalled_rounds >= MAX_STALLED_ROUNDS:
                log(f"\n[STOP] Error count stopped falling for {MAX_STALLED_ROUNDS} consecutive rounds (stuck at {total_errors} errors).")
                log(f"Reporting to user and stopping as required.")
                sys.exit(1)

        if total_errors == 0:
            log("Cargo returned non-zero but 0 diagnostic errors parsed.")
            sys.exit(1)

        # Count errors per file and pick file with most errors
        file_counts = {}
        for e in parsed_errors:
            fn = e["file_name"]
            if fn:
                file_counts[fn] = file_counts.get(fn, 0) + 1

        if not file_counts:
            log("No file names could be extracted from errors.")
            sys.exit(1)

        failing_filename = max(file_counts, key=file_counts.get)
        failing_file = ports_dir / "src" / failing_filename
        if not failing_file.exists():
            log(f"Failing file does not exist at {failing_file.as_posix()}")
            sys.exit(1)

        # Send ALL errors for this file
        file_errors = [e for e in parsed_errors if e["file_name"] == failing_filename]
        log(f"Targeting file: {failing_filename} ({len(file_errors)} error(s) out of {total_errors} total)")

        summary_of_all = "\n".join(e["full_error"] for e in parsed_errors[:15])
        log(f"Sending ALL {len(file_errors)} error(s) for {failing_filename} to {MODEL_NAME}...")
        fixed_code, tokens = call_flash_fix_rust(client, failing_file, file_errors, summary_of_all)

        if not fixed_code or len(fixed_code) < 100:
            log("Invalid model response (empty or too short). Retrying...")
            continue

        # Write updated file
        failing_file.write_text(fixed_code, encoding="utf-8")
        log(f"Wrote updated {failing_file.as_posix()}")

        # One commit per round
        commit_msg = f"fix(rust): fix {len(file_errors)} compiler error(s) in {failing_filename}"
        subprocess.run(["git", "add", str(failing_file)], check=True)
        commit_res = subprocess.run(["git", "commit", "-m", commit_msg], capture_output=True, text=True)
        if commit_res.returncode == 0:
            log(f"Committed round: {commit_msg}")
        else:
            log(f"Git commit returned code {commit_res.returncode}: {commit_res.stdout.strip() or commit_res.stderr.strip()}")

        # Record attempt
        attempt_record = {
            "attempt": total_attempts,
            "timestamp": datetime.now().isoformat(),
            "file": failing_filename,
            "file_errors": len(file_errors),
            "total_errors": total_errors,
            "tokens": tokens,
        }
        attempts_log.append(attempt_record)
        log_json.write_text(json.dumps(attempts_log, indent=2), encoding="utf-8")

        time.sleep(1)


def main():
    parser = argparse.ArgumentParser(description="Compile and fix ports using Gemini 3.8 Flash.")
    parser.add_argument("language", nargs="?", default="rust", choices=["odin", "rust"])
    args = parser.parse_args()

    ports_dir = Path("ports") / args.language
    runs_dir = Path("runs")
    runs_dir.mkdir(parents=True, exist_ok=True)

    client = genai.Client()

    log(f"=== Starting Compile-Fix Loop for {args.language.upper()} using {MODEL_NAME} ===")

    if args.language == "odin":
        run_odin_loop(ports_dir, runs_dir, client)
    elif args.language == "rust":
        run_rust_loop(ports_dir, runs_dir, client)


if __name__ == "__main__":
    main()
