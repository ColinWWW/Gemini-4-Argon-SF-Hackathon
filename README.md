# Referee

**Pixel-verified autonomous code migration, built with Gemini 4 Argon (`gemini-4-argon`) and Gemini 3.8 Flash (`gemini-3.8-flash`).**

Referee ports id Software's original 1997 Doom source code (`linuxdoom-1.10`, 126 C/header files, ~42,000 lines of C) to **Odin** and **Rust** in single long-decode passes, resolving 32-bit pointer assumptions for native 64-bit execution. The original 1997 C engine acts as the deterministic **referee**: all builds replay the same shareware `demo1` (`5,026 gametics` / `5,066 frames`), dump every raw 320×200 frame + 768-byte RGB palette (`64,768 bytes/frame`), and compare output pixel-for-pixel.

- **Problem Statement 1 (Million Token Engineering):** Whole-engine C-to-Odin and C-to-Rust translations in single model responses, verified against a deterministic ground-truth baseline.
- **Problem Statement 2 (More Dimensions):** Render → watch → critique → fix loop driven by side-by-side frame diff heatmaps and multi-port visual comparison videos.

---

## Results Summary

| Implementation | Target Arch | Source Files | Compile Loop (`gemini-3.8-flash`) | Gametics Timed | Total Frames | Identical Frames | Pixel Match % | First Divergent Frame |
|---|---|---|---|---|---|---|---|---|
| **Original C (`reference/`)** | 32-bit (`-m32`) | 62 `.c` + 64 `.h` | Ground Truth Referee | `5,026` | `5,066` | `5,066 / 5,066` | **100.0%** | — |
| **Odin Port (`ports/odin/`)** | 64-bit native | 124 `.odin` | 41 attempts (31 LLM fixes) | `5,026` | `5,066` | **`5,060 / 5,066`** | **100.0%** *(99.999%)* | `1967` |
| **Rust Port (`ports/rust/`)** | 64-bit native | 79 `.rs` (`src/`) | **9 attempts (7 LLM fixes)** | `5,026` | `5,066` | **`5,060 / 5,066`** | **100.0%** *(99.999%)* | `1967` |

> **Cross-Port Insight:** Applying the 64-bit lessons discovered during the Odin port (`z_zone` 8-byte alignment, `size_of` block header offsets, and byte-identical WAD lump string formatting) reduced the Rust port's compile-fix loop from **41 attempts down to 9 attempts**, converging from 108 initial compiler errors (`108 → 15 → 7 → 4 → 2 → 1 → 0`) in 7 LLM calls and matching the exact `5,060 / 5,066` frame equivalence on its very first run.

---

## Total Token Usage & API Cost Breakdown

All token usage is logged automatically in [`runs/port_odin.json`](runs/port_odin.json), [`runs/port_rust.json`](runs/port_rust.json), [`runs/compile_odin.json`](runs/compile_odin.json), and [`runs/compile_rust.json`](runs/compile_rust.json).

### 1. Single-Pass Whole-Engine Ports (`gemini-4-argon`)

| Port Run | Model | Input Tokens | Cached Input Tokens | Output Tokens | Total Tokens | Artifact |
|---|---|---:|---:|---:|---:|---|
| **C → Odin** (122 files in 1 pass) | `gemini-4-argon` | `1,749,208` | `0` | `247,826` | **`1,997,034`** | [`runs/port_odin.json`](runs/port_odin.json) |
| **C → Rust** (78 files in 1 pass) | `gemini-4-argon` | `2,350,570` | `1,344,809` *(57.2%)* | `311,499` | **`2,662,069`** | [`runs/port_rust.json`](runs/port_rust.json) |
| **Argon Subtotal** | | **`4,099,778`** | **`1,344,809`** | **`559,325`** | **`4,659,103`** | |

### 2. Automated Compile-Fix Loops (`gemini-3.8-flash`)

| Compile Loop | Model | Build Rounds | LLM Fix Calls | Prompt Tokens | Candidate Tokens | Total Tokens (incl. thinking) | Artifact |
|---|---|---:|---:|---:|---:|---:|---|
| **Odin Compile Loop** | `gemini-3.8-flash` | 41 | 31 | `185,374` | `158,996` | **`421,563`** | [`runs/compile_odin.json`](runs/compile_odin.json) |
| **Rust Compile Loop** | `gemini-3.8-flash` | 9 | 7 | `95,223` | `59,378` | **`154,601`** | [`runs/compile_rust.json`](runs/compile_rust.json) |
| **Flash Subtotal** | | **50** | **38** | **`280,597`** | **`218,374`** | **`576,164`** | |

### 3. Grand Total Across Project

| Metric | Tokens |
|---|---:|
| **Total Input / Prompt Tokens** | `4,380,375` *(1,344,809 cached)* |
| **Total Output / Candidate Tokens** | `777,699` |
| **Grand Total Tokens** | **`5,235,267`** |

---

## Visual Examples & Recorded Runs (`runs/`)

- **[`runs/three_way_60s.mp4`](runs/three_way_60s.mp4)**: 60-second (2,100 frames @ 35 fps, 1920×496) side-by-side comparison of **Original C (1997)** vs **Odin (Argon)** vs **Rust (Argon)** from frame `01500` to `03599`, with live per-frame pixel match % badges.
- **[`runs/clip_c_vs_odin_01967.mp4`](runs/clip_c_vs_odin_01967.mp4)**: 2-second 3-panel diff heatmap (`Reference C` | `Odin Port` | `Diff Heatmap`) centered on the first divergent frame (`01967`).
- **[`runs/clip_c_vs_rust_01967.mp4`](runs/clip_c_vs_rust_01967.mp4)**: 2-second 3-panel diff heatmap (`Reference C` | `Rust Port` | `Diff Heatmap`) centered on the first divergent frame (`01967`).
- **[`runs/clip_c_vs_odin_frame00000_60s.mp4`](runs/clip_c_vs_odin_frame00000_60s.mp4)** & **[`runs/clip_c_vs_odin_frame01500_60s.mp4`](runs/clip_c_vs_odin_frame01500_60s.mp4)**: 60-second side-by-side diff heatmap renders for `C vs Odin`.
- **[`runs/clip_c_vs_c_run2_00500.mp4`](runs/clip_c_vs_c_run2_00500.mp4)**: Determinism verification clip comparing two independent C referee runs (`100.0%` identical).

---

## Repository Structure

```text
.
├── reference/
│   ├── linuxdoom-1.10/          # 1997 id Software C source + headless frame dumper referee
│   └── CHANGES.md               # Audit log of every change to pristine linuxdoom-1.10
├── ports/
│   ├── odin/                    # Argon's single-pass Odin port (124 .odin files)
│   └── rust/                    # Argon's single-pass Rust port (Cargo crate, 79 .rs files)
├── harness/
│   ├── port.py                  # Launches background single-pass port calls to gemini-4-argon
│   ├── tables_to_odin.py        # Mechanical converter for tables.c & info.c -> Odin
│   ├── tables_to_rust.py        # Mechanical converter for tables.c & info.c -> Rust
│   ├── compile_fix.py           # Automated compiler error repair loop via gemini-3.8-flash
│   ├── score.py                 # Byte-for-byte frame & palette comparator -> JSON report
│   ├── clip.py                  # 3-panel (Ref, Port, Diff Heatmap) H.264 MP4 renderer
│   ├── compare.py               # Multi-port side-by-side comparison video renderer with match badges
│   └── frame2png.py             # Converts raw 64,768-byte indexed frame dumps to RGB PNGs
└── runs/                        # Token logs, score JSONs, and rendered MP4 comparison clips
```

---

## Usage & Reproducibility Examples

### 1. Run and Score the C Reference Build
```powershell
$env:DOOM_FRAME_DIR="frames/c"
.\reference\linuxdoom-1.10\linux\linuxxdoom.exe -iwad wad/DOOM1.WAD -timedemo demo1
```

### 2. Build, Run, and Score the Odin Port
```powershell
# Generate lookup tables & compile
python harness/tables_to_odin.py ports/odin
odin build ports/odin -out:ports/odin/doom_odin.exe

# Run timedemo and dump 5,066 frames
$env:DOOM_FRAME_DIR="frames/odin"
.\ports\odin\doom_odin.exe -iwad wad/DOOM1.WAD -timedemo demo1

# Score against C referee
python harness/score.py frames/c frames/odin
```

### 3. Build, Run, and Score the Rust Port
```powershell
# Generate lookup tables & compile release binary
python harness/tables_to_rust.py ports/rust/src
cargo build --release --manifest-path ports/rust/Cargo.toml

# Run timedemo and dump 5,066 frames
$env:DOOM_FRAME_DIR="frames/rust"
.\ports\rust\target\release\linuxdoom.exe -iwad wad/DOOM1.WAD -timedemo demo1

# Score against C referee
python harness/score.py frames/c frames/rust
```

### 4. Render Diff Clips & Multi-Port Comparisons
```powershell
# Render a 2-second 3-panel diff heatmap centered on frame 1967
python harness/clip.py frames/c frames/rust 1967 --out runs/clip_c_vs_rust_01967.mp4

# Render a 60-second 3-way side-by-side comparison (Original C vs Odin vs Rust)
python harness/compare.py --ref frames/c --ports frames/odin frames/rust --start 1500 --seconds 60 --out runs/three_way_60s.mp4
```

---

## License

Contains id Software's `linuxdoom-1.10` source code, released under the GNU General Public License (GPL). This repository and all generated ports are distributed under the GNU GPL. See [`LICENSE`](LICENSE) for details.
