Read README.md, reference/CHANGES.md, and runs/compile_odin.json. Run git log --oneline. This folder is a git worktree on branch port/rust. Stay on it, and don't touch ports/odin/.

ports/rust/ holds Argon's single-pass Rust port of linuxdoom-1.10 (snapshot already committed, src/ layout). 79 .rs files from 124 C files is expected because Rust merges .c/.h pairs. The Odin port of the same code already works (5,060 of 5,066 frames match the original), so we follow the same steps and apply its lessons up front.

1. Audit (report, don't fix):
   - Map each C file in reference/linuxdoom-1.10/ to the .rs file holding its code. List any C file whose code is missing.
   - Grep for placeholders: TODO, todo!(), unimplemented!(), "...", "unchanged", "omitted", empty function bodies (except the sound stubs).
   - Check for Cargo.toml and a main.rs that declares every module.

2. Make it buildable:
   - Add or fix Cargo.toml and main.rs so every module is declared. No extra crates unless truly needed.
   - Write harness/tables_to_rust.py, modeled on harness/tables_to_odin.py, generating the tables.c and info.c data with names and types matching how this port declares them. Verify sizes: finesine 10240, finetangent 4096, tantoangle 2049. If R_InitSpriteDefs walks sprnames until null, add a null terminator.
   - Commit.

3. Apply the Odin lessons before compiling, one commit each:
   - Zone allocator: memblock fields that are int in C must be i32. Round allocation sizes to 8 bytes, not 4. Find block headers with size_of, never a hardcoded 24. Keep "user == 2 means in use, no owner" and the user > 0x100 check before writing through it. Add caller info (file and line) to the Z_Free error.
   - Every sprintf / format: output must be byte-identical to C. Many build WAD lump names. C's "%.3d" means zero-padded to 3 digits ({:03}).

4. Extend harness/compile_fix.py to support rust: cargo build --release, parse rustc errors, send ALL errors for one file in a single gemini-3.8-flash call. Log to runs/compile_rust.json. One commit per round. Never call gemini-4-argon. Stop and report if the error count stops falling for 5 rounds. Report progress each time the error count halves.

5. Once it builds, from this folder:
   $env:DOOM_FRAME_DIR="frames/rust"
   .\target\release\<binary>.exe -iwad wad/DOOM1.WAD -timedemo demo1
   Success = "timed 5026 gametics" (ignore the exit code). If it crashes, report where.

6. Score against the original frames:
   python harness/score.py ..\Gemini-4-Argon-SF-Hackathon\frames\c frames\rust
   Report match %, identical frames, missing frames, and the first divergent frame.

Stop after step 6.