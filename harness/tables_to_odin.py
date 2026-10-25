#!/usr/bin/env python3
"""
harness/tables_to_odin.py - Mechanically convert Doom C lookup tables to Odin.

Converts:
  - reference/linuxdoom-1.10/tables.c -> ports/odin/tables.odin
  - reference/linuxdoom-1.10/info.c   -> ports/odin/info.odin

Usage:
  python harness/tables_to_odin.py [output_dir]
"""

import os
import re
import sys
from pathlib import Path


def convert_tables_c(tables_c_path: Path, output_path: Path):
    text = tables_c_path.read_text(encoding="latin1")

    # Extract finetangent (4096)
    m_tan = re.search(r'int\s+finetangent\[\s*(\d+)\s*\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_tan:
        raise ValueError("Could not find finetangent in tables.c")
    tan_size = m_tan.group(1)
    tan_vals = [x.strip() for x in re.sub(r'//[^\n]*', '', m_tan.group(2)).split(',') if x.strip()]

    # Extract finesine (10240)
    m_sin = re.search(r'int\s+finesine\[\s*([^\]]+)\s*\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_sin:
        raise ValueError("Could not find finesine in tables.c")
    sin_size = "10240"
    sin_vals = [x.strip() for x in re.sub(r'//[^\n]*', '', m_sin.group(2)).split(',') if x.strip()]

    # Extract tantoangle (2049)
    m_tta = re.search(r'(?:int|angle_t)\s+tantoangle\[\s*([^\]]+)\s*\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_tta:
        raise ValueError("Could not find tantoangle in tables.c")
    tta_size = "2049"
    tta_vals = [x.strip() for x in re.sub(r'//[^\n]*', '', m_tta.group(2)).split(',') if x.strip()]

    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("// DOOM lookup tables mechanically converted to Odin by harness/tables_to_odin.py\n")
        f.write("// Based on id Software's linuxdoom-1.10 tables.c (GNU GPL)\n\n")
        f.write("package main\n\n")

        # finetangent
        f.write(f"finetangent: [{tan_size}]fixed_t = {{\n")
        for i in range(0, len(tan_vals), 8):
            chunk = tan_vals[i:i+8]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("}\n\n")

        # finesine
        f.write(f"finesine: [{sin_size}]fixed_t = {{\n")
        for i in range(0, len(sin_vals), 8):
            chunk = sin_vals[i:i+8]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("}\n\n")

        # tantoangle
        f.write(f"tantoangle: [{tta_size}]angle_t = {{\n")
        for i in range(0, len(tta_vals), 8):
            chunk = tta_vals[i:i+8]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("}\n")

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
            sprite_raw = m_entry.group(1).strip()
            frame = m_entry.group(2).strip()
            tics = m_entry.group(3).strip()
            action_raw = m_entry.group(4).strip()
            nextstate_raw = m_entry.group(5).strip()
            misc1 = m_entry.group(6).strip()
            misc2 = m_entry.group(7).strip()

            # Enums need leading dot in Odin
            sprite = f".{sprite_raw}" if not sprite_raw.startswith(".") else sprite_raw

            if nextstate_raw in ("0", "S_NULL"):
                nextstate = ".S_NULL"
            elif not nextstate_raw.startswith("."):
                nextstate = f".{nextstate_raw}"
            else:
                nextstate = nextstate_raw

            if action_raw in ("NULL", "0", ""):
                action_odin = "{}"
            else:
                action_odin = f"{{ acp1 = auto_cast {action_raw} }}"

            state_entries.append((sprite, frame, tics, action_odin, nextstate, misc1, misc2, comment))

    # 3. mobjinfo
    m_mobj = re.search(r'mobjinfo_t\s+mobjinfo\[NUMMOBJTYPES\]\s*=\s*\{([^;]+)\};', text, re.DOTALL)
    if not m_mobj:
        raise ValueError("Could not find mobjinfo in info.c")
    mobj_body = m_mobj.group(1)

    mobj_blocks = re.findall(r'\{([^}]+)\}', mobj_body, re.DOTALL)
    mobj_entries = []

    state_indices = {1, 3, 7, 10, 11, 12, 13, 22}
    sfx_indices = {4, 6, 9, 14, 20}

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
                val = l
                # Format enum values with leading dot
                if field_idx in state_indices:
                    if val in ("0", "S_NULL"):
                        val = ".S_NULL"
                    elif not val.startswith("."):
                        val = f".{val}"
                elif field_idx in sfx_indices:
                    if val in ("0", "sfx_None"):
                        val = ".sfx_None"
                    elif not val.startswith("."):
                        val = f".{val}"
                else:
                    val = val.replace("|", " | ")
                    val = re.sub(r'\s+', ' ', val).strip()

                fields.append((val, c))
                field_idx += 1

        if fields:
            mobj_entries.append(fields)

    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        f.write("// DOOM info LUT tables mechanically converted to Odin by harness/tables_to_odin.py\n")
        f.write("// Based on id Software's linuxdoom-1.10 info.c (GNU GPL)\n\n")
        f.write("package main\n\n")

        # sprnames
        f.write(f"sprnames: [{len(spr_names)}]cstring = {{\n")
        for i in range(0, len(spr_names), 10):
            chunk = [f'"{name}"' for name in spr_names[i:i+10]]
            f.write("    " + ", ".join(chunk) + ",\n")
        f.write("}\n\n")

        # states
        f.write(f"states: [{len(state_entries)}]state_t = {{\n")
        for s in state_entries:
            sprite, frame, tics, action_odin, nextstate, misc1, misc2, comment = s
            entry_str = f"    {{ {sprite}, {frame}, {tics}, {action_odin}, {nextstate}, {misc1}, {misc2} }},"
            if comment:
                f.write(f"{entry_str:<70} {comment}\n")
            else:
                f.write(f"{entry_str}\n")
        f.write("}\n\n")

        # mobjinfo
        f.write(f"mobjinfo: [{len(mobj_entries)}]mobjinfo_t = {{\n")
        for entry in mobj_entries:
            f.write("    {\n")
            for val, comm in entry:
                if comm:
                    f.write(f"        {val + ',':<40} {comm}\n")
                else:
                    f.write(f"        {val},\n")
            f.write("    },\n")
        f.write("}\n")

    print(f"Generated {output_path.as_posix()} (sprnames: {len(spr_names)}, states: {len(state_entries)}, mobjinfo: {len(mobj_entries)})")


def main():
    out_dir = Path("ports/odin") if len(sys.argv) < 2 else Path(sys.argv[1])
    ref_dir = Path("reference/linuxdoom-1.10")

    tables_c = ref_dir / "tables.c"
    info_c = ref_dir / "info.c"

    tables_odin = out_dir / "tables.odin"
    info_odin = out_dir / "info.odin"

    convert_tables_c(tables_c, tables_odin)
    convert_info_c(info_c, info_odin)
    print("Done generating mechanical Odin table files.")


if __name__ == "__main__":
    main()
