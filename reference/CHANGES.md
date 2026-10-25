# Reference Implementation Changes (linuxdoom-1.10)

This document tracks all changes made to the pristine 1997 id Software `linuxdoom-1.10` source code (committed at `ac5db89`).
The reference build acts as the ground truth "referee" for autonomous code migration verification. All changes are committed individually with minimal invasiveness.

---

## Summary of Changes by File

| File | Changes | Description / Rationale |
|---|---|---|
| `alloca.h` | +6 | Added compatibility header mapping `<alloca.h>` to `<malloc.h>` for MinGW/Windows builds. |
| `values.h` | +16 | Added compatibility header defining `MAXINT`, `MININT`, `MAXSHORT`, etc. from `<limits.h>` expected by `doomdef.h`. |
| `Makefile` | +4, -4 | Configured for 32-bit compilation (`CC?= i686-w64-mingw32-gcc`, `CFLAGS=-m32 -g -Wall -std=gnu89 -DNORMALUNIX -I.`, `LDFLAGS=-m32`), removed X11/network libraries, linking only `-lm`. |
| `d_main.c` | +21, -1 | Added `-iwad <path>` CLI parameter parsing in `IdentifyVersion` to allow loading custom WAD paths; added fallback from `$HOME` to `%USERPROFILE%` and `.` on Windows; fixed Windows `mkdir` to 1 argument. Replaced obsolete `extern int errno;` with `<errno.h>`. |
| `g_game.c` | +7, -4 | Allowed demo version header byte `109` in `G_DoPlayDemo` in addition to `110` (`VERSION`) to support shareware `DOOM1.WAD` v1.9 demos. |
| `i_net.c` | +18, -310 | Replaced POSIX BSD socket networking with single-player dummy stubs (`I_InitNetwork` initializes 1 node / 1 player, `I_NetCmd` errors if invoked). |
| `i_sound.c` | +29, -916 | Replaced Linux OSS `/dev/dsp` audio driver and external `sndserv` IPC process code with silent no-op stubs. Replaced obsolete `extern int errno;` with `<errno.h>`. |
| `i_system.c` | +3, -11 | Replaced wall-clock `gettimeofday()` in `I_GetTime` with a monotonic fake clock advancing by one tic per call (`static int fake_tics = 0; return fake_tics++;`) to ensure 100% byte-for-byte deterministic replay during screen wipes. |
| `i_video.c` | +67, -995 | Stripped X11 dependencies. Replaced X11 rendering with headless frame dumper in `I_FinishUpdate` writing 320x200 8-bit `screens[0]` (64,000 bytes) plus 256-color palette (768 bytes) to `$DOOM_FRAME_DIR/%05d.raw` (default `frames/c/%05d.raw`, exactly 64,768 bytes per frame). |
| `m_misc.c` | +1, -1 | Replaced obsolete `errnos.h` / `extern int errno;` with `<errno.h>`. |
| `w_wad.c` | +4, -0 | Protected `O_BINARY` macro against redefinition; protected `strupr` and `filelength` against conflicting with Windows CRT built-ins via `#ifndef _WIN32`. |

---

## Detailed Commit Log

1. **`93cdcb5` - errno fix: replace obsolete extern int errno and errnos.h with errno.h**
   - Replaced obsolete Unix `extern int errno;` declarations and obsolete `errnos.h` header with modern standard `<errno.h>` in `d_main.c`, `i_sound.c`, and `m_misc.c`.

2. **`891be72` & `b08e11a` - headless video: strip X11 and dump 320x200 frames plus palette**
   - Removed all X11 header includes and X11 display connection/window creation logic.
   - Configured `I_FinishUpdate` to dump 64,768-byte raw frames (64,000 bytes pixel data + 768 bytes RGB palette) to `$DOOM_FRAME_DIR/%05d.raw` (default: `frames/c/%05d.raw`).
   - Removed duplicate `I_BeginRead` and `I_EndRead` stubs that conflicted with definitions in `i_system.c`.

3. **`d0113c2` - sound stubs: replace i_sound.c with no-op stubs and remove sndserv dependency**
   - Removed Linux soundcard driver code, IPC pipes, and `sndserv` invocation.
   - Implemented no-op stubs for all sound system calls (`I_InitSound`, `I_StartSound`, `I_StopSound`, etc.).

4. **`33c1809` - network stubs: single-player dummy stubs for i_net.c**
   - Removed POSIX socket headers (`sys/socket.h`, `netinet/in.h`, `arpa/inet.h`, `netdb.h`).
   - `I_InitNetwork` defaults cleanly to single-player mode (`netgame = false`, `consoleplayer = 0`, `numnodes = 1`).

5. **`0ff36ec` - compat headers: add alloca.h and values.h for MinGW build**
   - Added `reference/linuxdoom-1.10/alloca.h` (redirects to `<malloc.h>`).
   - Added `reference/linuxdoom-1.10/values.h` (maps `MAXINT`, `MININT`, `MAXSHORT`, etc. from `<limits.h>`).

6. **`95b0dcd` - build: update Makefile flags and compiler for 32-bit MinGW**
   - Set `CC?= i686-w64-mingw32-gcc`.
   - Set `CFLAGS=-m32 -g -Wall -std=gnu89 -DNORMALUNIX -I.`.
   - Set `LDFLAGS=-m32`.
   - Removed X11 libraries, linking `-lm`.

7. **`426662b` - w_wad: avoid redefinition of O_BINARY, strupr, and filelength under MinGW**
   - Added `#ifndef O_BINARY` guard in `w_wad.c`.
   - Added `#ifndef _WIN32` guard around `strupr` and `filelength` to prevent symbol collision with the C runtime library.

8. **`d0717aa` - d_main: support -iwad flag, HOME fallback to USERPROFILE, and win32 mkdir**
   - Added `-iwad` CLI parameter parsing to `IdentifyVersion` to allow pointing directly to shareware WAD (`wad/DOOM1.WAD`).
   - Added fallback from `$HOME` to `%USERPROFILE%` and `.` on Windows when creating `.doomrc`.
   - Wrapped `mkdir("c:\\doomdata")` in `#if defined(_WIN32)` to accept 1 argument on Windows.

9. **`85117e8` - g_game: accept version 109 demos from shareware DOOM1.WAD**
   - In `G_DoPlayDemo`, allowed demo version header byte `109` alongside `110` (`VERSION`) so the v1.9 shareware `demo1` lump plays without error.

10. **`40a3f40` - i_system: implement fake clock in I_GetTime advancing one tic per call**
    - Replaced real-time `gettimeofday()` clock with a deterministic monotonic tick generator (`static int fake_tics = 0; return fake_tics++;`).
    - Guarantees that `wipe_ScreenWipe` during the level start melt transition runs identical numbers of tics and renders 100% byte-for-byte identical frames on every run.

---

## Gate results

Verification results for Phase 1 reference build (`linuxxdoom.exe`):

1. **Binary architecture check (`file`):**
   ```
   reference/linuxdoom-1.10/linux/linuxxdoom.exe: PE32 executable for MS Windows 6.00 (console), Intel i386, 12 sections
   ```
   Confirmed 32-bit PE32 binary (Intel i386, not PE32+).

2. **Timedemo completion output:**
   ```
   Error: timed 5026 gametics in 25173 realtics
   ```
   Gametics: 5026 (exact match with required N = 5026).

3. **Frame count and size check:**
   - Total frames dumped in `frames/c/`: 5,066 (`00000.raw` through `05065.raw`).
   - File size verification: 100% of files (5,066 / 5,066) are exactly 64,768 bytes (64,000 bytes 320x200 8-bit indexed buffer + 768 bytes RGB palette).

4. **Determinism verification (two independent runs compared via SHA-256):**
   - Run 1 directory: `frames/c/`
   - Run 2 directory: `frames/c_run2/` (with `$DOOM_FRAME_DIR="frames/c_run2"`)
   - Frames compared: 5,066
   - Frames mismatched: 0
   - Result: 100.0% byte-identical determinism across all frames.

5. **Frame to PNG conversion (`harness/frame2png.py`):**
   - Frame 0: `frames/c/00000.png`
   - Frame 100: `frames/c/00100.png`
   - Frame 500: `frames/c/00500.png`
   - Last frame (5065): `frames/c/05065.png`

