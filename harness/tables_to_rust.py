#!/usr/bin/env python3
"""
harness/tables_to_rust.py - Mechanically convert Doom C lookup tables to Rust.

Converts:
  - reference/linuxdoom-1.10/tables.c -> ports/rust/src/tables_data.rs
  - reference/linuxdoom-1.10/info.c   -> ports/rust/src/info_data.rs

Usage:
  python harness/tables_to_rust.py [output_dir]
"""

import os
from pathlib import Path
import re
import sys

ACP2_ACTIONS = {
    # 2-arg weapon actions from p_pspr
    "A_WeaponReady", "A_ReFire", "A_CheckReload", "A_Lower", "A_Raise",
    "A_GunFlash", "A_Punch", "A_Saw", "A_FireMissile", "A_FireBFG",
    "A_FirePlasma", "A_FirePistol", "A_FireShotgun", "A_FireShotgun2",
    "A_FireCGun", "A_Light0", "A_Light1", "A_Light2", "A_BFGsound",
    # 2-arg actions from p_enemy
    "A_OpenShotgun2", "A_LoadShotgun2", "A_CloseShotgun2",
}

PSPR_ACTIONS = {
    "A_WeaponReady", "A_ReFire", "A_CheckReload", "A_Lower", "A_Raise",
    "A_GunFlash", "A_Punch", "A_Saw", "A_FireMissile", "A_FireBFG",
    "A_FirePlasma", "A_FirePistol", "A_FireShotgun", "A_FireShotgun2",
    "A_FireCGun", "A_Light0", "A_Light1", "A_Light2", "A_BFGSpray", "A_BFGsound",
}


def convert_tables_c(tables_c_path: Path, output_path: Path):
    text = tables_c_path.read_text(encoding="latin1")

    # Extract finetangent (4096)
    m_tan = re.search(r'int\s+finetangent\[\s*(\d+)\s*\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_tan:
        raise ValueError("Could not find finetangent in tables.c")
    tan_vals = [x.strip() for x in re.sub(r'//[^\n]*', '', m_tan.group(2)).split(',') if x.strip()]
    if len(tan_vals) != 4096:
        raise ValueError(f"Expected 4096 finetangent values, got {len(tan_vals)}")

    # Extract finesine (10240)
    m_sin = re.search(r'int\s+finesine\[\s*([^\]]+)\s*\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_sin:
        raise ValueError("Could not find finesine in tables.c")
    sin_vals = [x.strip() for x in re.sub(r'//[^\n]*', '', m_sin.group(2)).split(',') if x.strip()]
    if len(sin_vals) != 10240:
        raise ValueError(f"Expected 10240 finesine values, got {len(sin_vals)}")

    # Extract tantoangle (2049)
    m_tta = re.search(r'(?:int|angle_t)\s+tantoangle\[\s*([^\]]+)\s*\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_tta:
        raise ValueError("Could not find tantoangle in tables.c")
    tta_vals = [x.strip() for x in re.sub(r'//[^\n]*', '', m_tta.group(2)).split(',') if x.strip()]
    if len(tta_vals) != 2049:
        raise ValueError(f"Expected 2049 tantoangle values, got {len(tta_vals)}")

    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("// DOOM lookup tables mechanically converted to Rust by harness/tables_to_rust.py\n")
        f.write("// Based on id Software's linuxdoom-1.10 tables.c (GNU GPL)\n\n")
        f.write("use crate::m_fixed::fixed_t;\n")
        f.write("use crate::tables::angle_t;\n\n")

        # finetangent
        f.write(f"pub static finetangent: [fixed_t; {len(tan_vals)}] = [\n")
        for i in range(0, len(tan_vals), 8):
            chunk = tan_vals[i:i+8]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("];\n\n")

        # finesine
        f.write(f"pub static finesine: [fixed_t; {len(sin_vals)}] = [\n")
        for i in range(0, len(sin_vals), 8):
            chunk = sin_vals[i:i+8]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("];\n\n")

        # tantoangle
        f.write(f"pub static tantoangle: [angle_t; {len(tta_vals)}] = [\n")
        for i in range(0, len(tta_vals), 8):
            chunk = tta_vals[i:i+8]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("];\n")

    print(f"Generated {output_path.as_posix()} (finetangent: {len(tan_vals)}, finesine: {len(sin_vals)}, tantoangle: {len(tta_vals)})")


def convert_info_c(info_c_path: Path, output_path: Path):
    text = info_c_path.read_text(encoding="latin1")

    # 1. sprnames
    m_spr = re.search(r'char\s*\*sprnames\[NUMSPRITES\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_spr:
        raise ValueError("Could not find sprnames in info.c")
    spr_body = re.sub(r'//[^\n]*', '', m_spr.group(1))
    spr_names = re.findall(r'\"([^\"]+)\"', spr_body)

    # 2. states
    m_states = re.search(r'state_t\s+states\[NUMSTATES\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_states:
        raise ValueError("Could not find states in info.c")
    states_body = m_states.group(1)

    state_entries = []
    for line in states_body.splitlines():
        line = line.strip()
        if not line or line.startswith("//"):
            continue
        comment = ""
        if "//" in line:
            parts = line.split("//", 1)
            line = parts[0].strip()
            comment = "// " + parts[1].strip()

        # Match: {SPR_TROO,0,-1,{NULL},S_NULL,0,0},
        m_entry = re.search(r'\{([^,]+),([^,]+),([^,]+),\{([^}]+)\},([^,]+),([^,]+),([^}]+)\}', line)
        if m_entry:
            sprite = m_entry.group(1).strip()
            frame = m_entry.group(2).strip()
            tics = m_entry.group(3).strip()
            action_raw = m_entry.group(4).strip()
            nextstate = m_entry.group(5).strip()
            misc1 = m_entry.group(6).strip()
            misc2 = m_entry.group(7).strip()

            if action_raw in ("NULL", "0", ""):
                action_rust = "actionf_t::None"
            elif action_raw in ACP2_ACTIONS:
                mod_name = "crate::p_pspr" if action_raw in PSPR_ACTIONS else "crate::p_enemy"
                action_rust = f"actionf_t::acp2({mod_name}::{action_raw})"
            else:
                mod_name = "crate::p_pspr" if action_raw in PSPR_ACTIONS else "crate::p_enemy"
                action_rust = f"actionf_t::acp1({mod_name}::{action_raw})"

            state_entries.append((sprite, frame, tics, action_rust, nextstate, misc1, misc2, comment))

    # 3. mobjinfo
    m_mobj = re.search(r'mobjinfo_t\s+mobjinfo\[NUMMOBJTYPES\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_mobj:
        raise ValueError("Could not find mobjinfo in info.c")
    mobj_body = m_mobj.group(1)

    mobj_blocks = re.findall(r'\{([^}]+)\}', mobj_body, re.DOTALL)
    mobj_field_names = [
        "doomednum", "spawnstate", "spawnhealth", "seestate", "seesound",
        "reactiontime", "attacksound", "painstate", "painchance", "painsound",
        "meleestate", "missilestate", "deathstate", "xdeathstate", "deathsound",
        "speed", "radius", "height", "mass", "damage",
        "activesound", "flags", "raisestate"
    ]

    mobj_entries = []
    for block in mobj_blocks:
        lines = [l.strip() for l in block.splitlines() if l.strip()]
        fields = []
        field_idx = 0
        for l in lines:
            c = ""
            if "//" in l:
                p = l.split("//", 1)
                l = p[0].strip()
                c = "// " + p[1].strip()
            l = l.rstrip(",")
            if l:
                val = l.replace("|", " | ")
                val = re.sub(r'\s+', ' ', val).strip()
                fname = mobj_field_names[field_idx] if field_idx < len(mobj_field_names) else f"field_{field_idx}"
                fields.append((fname, val, c))
                field_idx += 1

        if fields:
            mobj_entries.append(fields)

    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("// DOOM info LUT tables mechanically converted to Rust by harness/tables_to_rust.py\n")
        f.write("// Based on id Software's linuxdoom-1.10 info.c (GNU GPL)\n\n")
        f.write("use crate::d_think::actionf_t;\n")
        f.write("use crate::info::*;\n")
        f.write("use crate::p_mobj::*;\n")
        f.write("use crate::p_pspr::*;\n")
        f.write("use crate::sounds::*;\n")
        f.write("use crate::m_fixed::*;\n\n")

        # sprnames
        f.write(f"pub static sprnames: [&'static str; {len(spr_names)}] = [\n")
        for i in range(0, len(spr_names), 10):
            chunk = [f'"{name}"' for name in spr_names[i:i+10]]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("];\n\n")

        # states
        f.write(f"pub static mut states: [state_t; {len(state_entries)}] = [\n")
        for s in state_entries:
            sprite, frame, tics, action_rust, nextstate, misc1, misc2, comment = s
            entry_str = f"    state_t {{ sprite: {sprite}, frame: {frame}, tics: {tics}, action: {action_rust}, nextstate: {nextstate}, misc1: {misc1}, misc2: {misc2} }},"
            if comment:
                f.write(f"{entry_str:<95} {comment}\n")
            else:
                f.write(f"{entry_str}\n")
        f.write("];\n\n")

        # mobjinfo
        f.write(f"pub static mut mobjinfo: [mobjinfo_t; {len(mobj_entries)}] = [\n")
        for entry in mobj_entries:
            f.write("    mobjinfo_t {\n")
            for fname, val, comm in entry:
                line_str = f"        {fname}: {val},"
                if comm:
                    f.write(f"{line_str:<45} {comm}\n")
                else:
                    f.write(f"{line_str}\n")
            f.write("    },\n")
        f.write("];\n")

    print(f"Generated {output_path.as_posix()} (sprnames: {len(spr_names)}, states: {len(state_entries)}, mobjinfo: {len(mobj_entries)})")


def main():
    out_dir = Path("ports/rust/src") if len(sys.argv) < 2 else Path(sys.argv[1])
    ref_dir = Path("reference/linuxdoom-1.10")

    tables_c = ref_dir / "tables.c"
    info_c = ref_dir / "info.c"

    tables_rust = out_dir / "tables_data.rs"
    info_rust = out_dir / "info_data.rs"

    convert_tables_c(tables_c, tables_rust)
    convert_info_c(info_c, info_rust)
    print("Done generating mechanical Rust table files.")


if __name__ == "__main__":
    main()
