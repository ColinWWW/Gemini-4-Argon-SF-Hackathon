# Referee

**Pixel-verified autonomous code migration, built with Gemini 4 Argon.**

Argon ports the original 1997 Doom source (id Software's `linuxdoom-1.10`, GPL) from C to Odin and Zig (Rust as a stretch) in single long-decode passes. The original C program is the referee: both versions replay the same deterministic demo, every frame is compared byte-for-byte, and Argon watches video of where they diverge and repairs its own port in a loop until they match.

Doom is the test case, not the product. The product is a way to *prove* a migration behaves identically to the original.

- **Problem Statement 1 (Million Token Engineering):** whole-engine translation in one response per language, measured against a baseline.
- **Problem Statement 2 (More Dimensions):** render → watch → critique → fix loop driven by agentic video understanding.

---

## Ground rules

- **Gemini writes all the code.** I write prompts, review plans, and approve merges.
- **Argon only where it's needed:** the ports and the repair calls. Everything else runs on Gemini 3.8 Flash (or Antigravity on a personal account) to save credits.
- **Never commit the API key.** The repo is public.
- **The reference stays close to the original.** Every change to the 1997 source is its own commit and is listed in `reference/CHANGES.md`.

---

## Budget: $25 of API credit

Launch pricing: $2 per 1M input tokens, $10 per 1M output tokens. Cached input prefixes get a 95% discount automatically.

| Item | Estimate |
|---|---|
| Odin port (single pass) | ~$5 |
| Zig port (single pass) | ~$5 |
| Compile-fix loop (3.8 Flash) | ~$1 |
| Repair iterations (Argon, trimmed context) | ~$6 |
| Reserve (Rust port or extra repairs) | ~$8 |

Rules:
1. Run `count_tokens` on `reference/` before any port. It's free and gives the real cost.
2. Don't build the harness with Argon. Use `gemini -m gemini-3.8-flash` or Antigravity.
3. Launch the Zig port only after Odin compiles. Rust only if $8+ remains at 2:00.
4. Repairs get trimmed context: Flash finds the diverging file first, then Argon sees only those files plus the clip.
5. Prompt order for caching: unchanging content first (original C), changing content last (port files, clip, scores).
6. Cap every call with `max_output_tokens`: ~400K for ports, ~32K for repairs.
7. Check usage in AI Studio after the first port lands.
8. Ask the Google booth about extra credits.

---

## Repo layout

```
referee/
  reference/        linuxdoom-1.10 + headless platform layer (the referee)
  ports/odin/       Argon's Odin port
  ports/zig/        Argon's Zig port
  ports/rust/       stretch
  harness/
    port.py         launches background port calls
    compile_fix.py  Flash compile-error loop
    score.py        frame comparison → JSON
    clip.py         ffmpeg side-by-side + diff heatmap
    repair.py       clip → Argon → patch → rebuild → rescore
  runs/             JSON, clips, and patches for every iteration
  web/index.html    demo view: videos + match-% chart
  wad/              DOOM1.WAD (gitignored)
  frames/           raw frame dumps (gitignored)
```

`.gitignore` must contain: `wad/`, `frames/`, `.env`

Each language lives on its own branch (`port/odin`, `port/zig`). Every Argon patch is one commit; I approve the merge.

---

## Setup checklist

- [ ] WSL terminal open; work in `~/referee`, not the Windows drive
- [ ] `git clone https://github.com/ColinWWW/Gemini-4-Argon-SF-Hackathon.git ~/referee`
- [ ] Copy `linuxdoom-1.10` from `https://github.com/id-Software/DOOM` into `reference/`, plus the GPL license file into the repo root
- [ ] `.gitignore` created, then commit "Pristine linuxdoom-1.10 (GPL)" and push
- [ ] Shareware `DOOM1.WAD` (v1.9, ~4.2 MB) in `wad/`
- [ ] `sudo apt install gcc-multilib make ffmpeg python3-numpy`
- [ ] API key exported in `~/.bashrc` as `GEMINI_API_KEY`
- [ ] Node 20+ and `npm install -g @google/gemini-cli@latest`
- [ ] Confirm the key sees Argon: list models and grep for `gemini-4-argon`
- [ ] Odin and Zig toolchains installed (have the CLI do this)

---

## Timeline

Assumes demos around 5:00. Adjust if the schedule says otherwise.

| Time | Phase | Done when |
|---|---|---|
| 11:30–12:15 | 1. Reference build | Two runs produce byte-identical frame sets |
| 12:15 | 2. Launch Odin port | Background call running |
| 12:15–1:30 | 3. Harness | Scorer and clip renderer work on C-vs-C |
| 1:30–2:30 | 4. Compile Odin | Odin binary runs the demo and dumps frames. Then launch Zig |
| 2:30–4:00 | 5. Repair loop | Match % climbing, every iteration logged |
| 4:00–5:00 | 6. Demo prep | Chart, diagram, rehearsed 3-minute run, recorded backup |

**If behind:** one language fully working beats two half-working. Cut Zig before cutting the repair loop.

---

## Phase 1: Reference build

Tool: Gemini CLI on Flash. Paste:

> **Context:** We're building "Referee," a system that proves a code migration is correct by comparing frame-by-frame output against the original program. `reference/` contains the original id Software `linuxdoom-1.10` source, committed pristine. It is our ground truth, so change as little as possible and commit each change separately with a clear message.
>
> **Goal:** make `reference/` build and run headless on 64-bit Linux, playing a demo and dumping every frame to disk.
>
> 1. Build it as a 32-bit binary (`-m32`); the code assumes 32-bit pointers. Fix only the minimum compile errors for a modern gcc (for example the old `extern int errno`). Do not refactor or fix 64-bit issues.
> 2. Replace `i_video.c` with a headless version. No X11. `I_FinishUpdate` writes the 320×200 8-bit `screens[0]` buffer plus the current 768-byte palette to `frames/c/NNNNN.raw` (zero-padded frame number). `I_SetPalette` stores the palette. `I_InitGraphics`, `I_StartTic`, and the other video functions do nothing.
> 3. Replace `i_sound.c` with stubs that do nothing. Remove any dependency on `sndserv`.
> 4. Leave `i_net.c` alone unless it fails to compile.
> 5. If the 1.9 shareware demos are rejected with a version error, patch only that check in `g_game.c`.
> 6. Run with `-iwad wad/DOOM1.WAD -timedemo demo1` and confirm that it exits on its own and that `frames/c/` fills with files of exactly 64,768 bytes each.
> 7. Run it twice and confirm the two frame sets are byte-identical. This proves the reference is deterministic.
> 8. Write `CHANGES.md` listing every change to the original source and why.
>
> Before writing any code, tell me your plan and any risks you see.

**Gate:** step 7 passes. If the reference isn't deterministic, nothing else works.

---

## Phase 2: Single-pass ports

Tool: Gemini CLI on Flash writes `harness/port.py`; the script calls Argon directly.

Have it build `port.py` to:
- Count tokens for `reference/` first and print the estimated cost
- Send the full `reference/` source (including the headless `i_video.c` and sound stubs) to `gemini-4-argon` on the Interactions API with `background=True`, `max_output_tokens` ~400K, `thinking_level: "medium"`
- Poll until done, then write each output file into `ports/<lang>/`
- Save token usage to `runs/port_<lang>.json`

Port prompt (one per language; swap the language name):

> Translate this entire C codebase, the original linuxdoom-1.10 engine, to **Odin** in a single response.
>
> Rules:
> 1. Faithful, not idiomatic. Keep every file, function name, global, and the overall structure. One output file per input file.
> 2. The code assumes 32-bit pointers. The port is native 64-bit: WAD on-disk structures must use fixed-width types, and never store a pointer in an integer. Fix every such assumption.
> 3. Translate the headless `i_video.c`, the `i_sound.c` stubs, and the frame dump exactly. The output frame files must be byte-identical in format: 64,000 palette-index bytes plus the 768-byte palette, written to `frames/odin/NNNNN.raw`.
> 4. Same command line: `-iwad <path> -timedemo demo1`.
> 5. Output every file in full. No placeholders, no "rest unchanged," no TODOs.
> 6. Format each file as a fenced code block preceded by its path on its own line.

For **Rust** (stretch), add: "Use `unsafe` and `static mut` freely to mirror the C; do not restructure to satisfy the borrow checker."

---

## Phase 3: Harness

Tool: Gemini CLI on Flash.

**`score.py`:** compare `frames/c/` with `frames/<lang>/`. Output JSON with per-frame pixel match %, overall match %, frame count difference, and the **first divergent frame**. Test it C-vs-C first: it must report 100%.

**`clip.py`:** given a frame number, render a 2-second clip (70 frames at 35 fps) centered on it, as three panels side by side: C, port, and a diff heatmap. Convert palette indices to RGB using each frame's palette. Output an mp4 to `runs/`.

---

## Phase 4: Compile loop

Tool: `compile_fix.py`, built by the CLI on Flash.

- Run the language's compiler; send errors plus the failing file to `gemini-3.8-flash`; apply the fix; repeat
- After 5 failed attempts on the same error, escalate that file to Argon
- Log attempts per language to `runs/compile_<lang>.json`. "Iterations to first compile" is chart data

---

## Phase 5: Repair loop

Tool: `repair.py`, built by the CLI on Flash; repairs call Argon.

Each iteration:
1. Run the port, score it, find the first divergent frame
2. `clip.py` renders the side-by-side clip
3. Flash names the most likely file(s) and function(s) responsible (cheap localization)
4. Argon (`thinking_level: "high"`, `max_output_tokens` ~32K) gets: the matching original C file(s), the port file(s), the clip, and the scorer JSON. It returns an explanation and a unified diff
5. Apply the diff as a commit on the language branch, rebuild, rescore
6. **If the score drops, revert automatically**
7. Log the iteration to `runs/repair_<lang>_<n>.json` (score before and after, tokens used, explanation)

Stop when the match reaches 100% or the budget limit is hit.

---

## Phase 6: Demo

**Demo view (`web/index.html`):** side-by-side videos and a chart of match % per iteration, one line per language. Supporting material, not the feature.

**Ask Argon for an interactive architecture diagram** of the pipeline (the builder guide recommends this).

**3-minute script:**
1. **(20s) Problem:** "Companies won't migrate C because they can't prove the new code behaves the same. We built a system that proves it."
2. **(40s) Single pass:** Argon translated the entire 1997 engine in one response per language, and fixed its 32-bit assumptions along the way.
3. **(60s) Live repair:** play the divergence clip, show Argon's explanation, apply the patch, rerun, and watch the match % go up.
4. **(40s) Chart:** both languages converging toward 100%. Note which language was harder to migrate into.
5. **(20s) Generality:** works on any program with a deterministic replay. The referee diff is ~60 lines; here's every one.

**Record a backup** of one full repair iteration in case Argon is slow on stage.

---

## Judging checklist (from the builder guide)

- [ ] **Real person:** me. I write Odin and wanted Doom's engine in a language I use. Broader user: any team with a legacy C codebase they're afraid to migrate.
- [ ] **Needs Argon:** 1M-token single-pass output plus video-based divergence diagnosis
- [ ] **People in control:** every patch is a reviewable commit; I approve merges
- [ ] **Live in under 3 minutes** with real inputs

---

## Stretch goals (in order, only after the one above is done)

1. Zig port converging (core plus one)
2. Rust port, if $8+ remains
3. **Unseen content test:** Nano Banana generates new textures and a title screen, packed into a PWAD. Show the match still holds on content no model has seen.
4. Move the repair loop into an Antigravity managed agent sandbox (`antigravity-preview-09-2026`) for the Best Use of Managed Agents prize

---

## Risks and fallbacks

| Risk | Fallback |
|---|---|
| Port won't fit in one response | Split by module group with a shared interface file; say so honestly on stage |
| Port won't compile by 2:30 | Drop that language; keep its compile-attempt data for the chart |
| Demo desyncs | Irrelevant: the C build is the referee, not the demo file |
| Credits run low | Stop launching ports; spend what's left on repairs for the best language |
| Argon slow on stage | Play the recorded backup iteration |

---

## License

Contains id Software's Doom source, released under the GNU GPL. This repository and all ports are distributed under the GPL as well.
