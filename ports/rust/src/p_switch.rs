// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_system::*;
use crate::p_ceilng::*;
use crate::p_doors::*;
use crate::p_floor::*;
use crate::p_lights::*;
use crate::p_mobj::*;
use crate::p_plats::*;
use crate::p_spec::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;

pub static mut alphSwitchList: [switchlist_t; 41] = [
    switchlist_t {
        name1: "SW1BRCOM",
        name2: "SW2BRCOM",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BRN1",
        name2: "SW2BRN1",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BRN2",
        name2: "SW2BRN2",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BRNGN",
        name2: "SW2BRNGN",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BROWN",
        name2: "SW2BROWN",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1COMM",
        name2: "SW2COMM",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1COMP",
        name2: "SW2COMP",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1DIRT",
        name2: "SW2DIRT",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1EXIT",
        name2: "SW2EXIT",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1GRAY",
        name2: "SW2GRAY",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1GRAY1",
        name2: "SW2GRAY1",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1METAL",
        name2: "SW2METAL",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1PIPE",
        name2: "SW2PIPE",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1SLAD",
        name2: "SW2SLAD",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STARG",
        name2: "SW2STARG",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STON1",
        name2: "SW2STON1",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STON2",
        name2: "SW2STON2",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STONE",
        name2: "SW2STONE",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STRTN",
        name2: "SW2STRTN",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BLUE",
        name2: "SW2BLUE",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1CMT",
        name2: "SW2CMT",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1GARG",
        name2: "SW2GARG",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1GSTON",
        name2: "SW2GSTON",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1HOT",
        name2: "SW2HOT",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1LION",
        name2: "SW2LION",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1SATYR",
        name2: "SW2SATYR",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1SKIN",
        name2: "SW2SKIN",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1VINE",
        name2: "SW2VINE",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1WOOD",
        name2: "SW2WOOD",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1PANEL",
        name2: "SW2PANEL",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1ROCK",
        name2: "SW2ROCK",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1MET2",
        name2: "SW2MET2",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1WDMET",
        name2: "SW2WDMET",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1BRIK",
        name2: "SW2BRIK",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1MOD1",
        name2: "SW2MOD1",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1ZIM",
        name2: "SW2ZIM",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1STON6",
        name2: "SW2STON6",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1TEK",
        name2: "SW2TEK",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1MARB",
        name2: "SW2MARB",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1SKULL",
        name2: "SW2SKULL",
        episode: 3,
    },
    switchlist_t {
        name1: "",
        name2: "",
        episode: 0,
    },
];

pub static mut switchlist: [i32; MAXSWITCHES * 2] = [0; MAXSWITCHES * 2];
pub static mut numswitches: i32 = 0;
pub static mut buttonlist: [button_t; MAXBUTTONS] = [button_t {
    line: std::ptr::null_mut(),
    where_: 0,
    btexture: 0,
    btimer: 0,
    soundorg: std::ptr::null_mut(),
}; MAXBUTTONS];

pub unsafe fn P_InitSwitchList() {
    let mut episode = 1;
    if gamemode == registered {
        episode = 2;
    } else if gamemode == commercial {
        episode = 3;
    }

    let mut index = 0usize;
    for i in 0..MAXSWITCHES {
        if alphSwitchList[i].episode == 0 {
            numswitches = (index / 2) as i32;
            switchlist[index] = -1;
            break;
        }

        if alphSwitchList[i].episode <= episode {
            switchlist[index] = R_TextureNumForNameStr(alphSwitchList[i].name1);
            index += 1;
            switchlist[index] = R_TextureNumForNameStr(alphSwitchList[i].name2);
            index += 1;
        }
    }
}

pub unsafe fn P_StartButton(line: *mut line_t, w: bwhere_e, texture: i32, time: i32) {
    for i in 0..MAXBUTTONS {
        if buttonlist[i].btimer != 0 && buttonlist[i].line == line {
            return;
        }
    }

    for i in 0..MAXBUTTONS {
        if buttonlist[i].btimer == 0 {
            buttonlist[i].line = line;
            buttonlist[i].where_ = w;
            buttonlist[i].btexture = texture;
            buttonlist[i].btimer = time;
            buttonlist[i].soundorg =
                &mut (*(*line).frontsector).soundorg as *mut degenmobj_t as *mut mobj_t;
            return;
        }
    }

    I_Error("P_StartButton: no button slots left!");
}

pub unsafe fn P_ChangeSwitchTexture(line: *mut line_t, useAgain: i32) {
    if useAgain == 0 {
        (*line).special = 0;
    }

    let side0 = sides.add((*line).sidenum[0] as usize);
    let texTop = (*side0).toptexture as i32;
    let texMid = (*side0).midtexture as i32;
    let texBot = (*side0).bottomtexture as i32;

    let mut sound = sfx_swtchn;
    if (*line).special == 11 {
        sound = sfx_swtchx;
    }

    for i in 0..((numswitches * 2) as usize) {
        if switchlist[i] == texTop {
            S_StartSound(buttonlist[0].soundorg as *mut c_void, sound);
            (*side0).toptexture = switchlist[i ^ 1] as i16;
            if useAgain != 0 {
                P_StartButton(line, top, switchlist[i], BUTTONTIME);
            }
            return;
        } else if switchlist[i] == texMid {
            S_StartSound(buttonlist[0].soundorg as *mut c_void, sound);
            (*side0).midtexture = switchlist[i ^ 1] as i16;
            if useAgain != 0 {
                P_StartButton(line, middle, switchlist[i], BUTTONTIME);
            }
            return;
        } else if switchlist[i] == texBot {
            S_StartSound(buttonlist[0].soundorg as *mut c_void, sound);
            (*side0).bottomtexture = switchlist[i ^ 1] as i16;
            if useAgain != 0 {
                P_StartButton(line, bottom, switchlist[i], BUTTONTIME);
            }
            return;
        }
    }
}

pub unsafe fn P_UseSpecialLine(thing: *mut mobj_t, line: *mut line_t, side: i32) -> boolean {
    if side != 0 {
        match (*line).special {
            124 => {}
            _ => return false_,
        }
    }

    if (*thing).player.is_null() {
        if ((*line).flags as i32) & ML_SECRET != 0 {
            return false_;
        }
        match (*line).special {
            1 | 32 | 33 | 34 => {}
            _ => return false_,
        }
    }

    match (*line).special {
        1 | 26 | 27 | 28 | 31 | 32 | 33 | 34 | 117 | 118 => {
            EV_VerticalDoor(line, thing);
        }
        7 => {
            if EV_BuildStairs(line, build8) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        9 => {
            if EV_DoDonut(line) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        11 => {
            P_ChangeSwitchTexture(line, 0);
            G_ExitLevel();
        }
        14 => {
            if EV_DoPlat(line, raiseAndChange, 32) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        15 => {
            if EV_DoPlat(line, raiseAndChange, 24) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        18 => {
            if EV_DoFloor(line, raiseFloorToNearest) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        20 => {
            if EV_DoPlat(line, raiseToNearestAndChange, 0) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        21 => {
            if EV_DoPlat(line, downWaitUpStay, 0) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        23 => {
            if EV_DoFloor(line, lowerFloorToLowest) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        29 => {
            if EV_DoDoor(line, normal) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        41 => {
            if EV_DoCeiling(line, lowerToFloor) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        71 => {
            if EV_DoFloor(line, turboLower) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        49 => {
            if EV_DoCeiling(line, crushAndRaise) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        50 => {
            if EV_DoDoor(line, close) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        51 => {
            P_ChangeSwitchTexture(line, 0);
            G_SecretExitLevel();
        }
        55 => {
            if EV_DoFloor(line, raiseFloorCrush) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        101 => {
            if EV_DoFloor(line, raiseFloor) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        102 => {
            if EV_DoFloor(line, lowerFloor) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        103 => {
            if EV_DoDoor(line, open) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        111 => {
            if EV_DoDoor(line, blazeRaise) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        112 => {
            if EV_DoDoor(line, blazeOpen) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        113 => {
            if EV_DoDoor(line, blazeClose) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        122 => {
            if EV_DoPlat(line, blazeDWUS, 0) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        127 => {
            if EV_BuildStairs(line, turbo16) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        131 => {
            if EV_DoFloor(line, raiseFloorTurbo) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        133 | 135 | 137 => {
            if EV_DoLockedDoor(line, blazeOpen, thing) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        140 => {
            if EV_DoFloor(line, raiseFloor512) != 0 {
                P_ChangeSwitchTexture(line, 0);
            }
        }
        42 => {
            if EV_DoDoor(line, close) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        43 => {
            if EV_DoCeiling(line, lowerToFloor) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        45 => {
            if EV_DoFloor(line, lowerFloor) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        60 => {
            if EV_DoFloor(line, lowerFloorToLowest) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        61 => {
            if EV_DoDoor(line, open) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        62 => {
            if EV_DoPlat(line, downWaitUpStay, 1) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        63 => {
            if EV_DoDoor(line, normal) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        64 => {
            if EV_DoFloor(line, raiseFloor) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        66 => {
            if EV_DoPlat(line, raiseAndChange, 24) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        67 => {
            if EV_DoPlat(line, raiseAndChange, 32) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        65 => {
            if EV_DoFloor(line, raiseFloorCrush) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        68 => {
            if EV_DoPlat(line, raiseToNearestAndChange, 0) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        69 => {
            if EV_DoFloor(line, raiseFloorToNearest) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        70 => {
            if EV_DoFloor(line, turboLower) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        114 => {
            if EV_DoDoor(line, blazeRaise) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        115 => {
            if EV_DoDoor(line, blazeOpen) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        116 => {
            if EV_DoDoor(line, blazeClose) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        123 => {
            if EV_DoPlat(line, blazeDWUS, 0) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        132 => {
            if EV_DoFloor(line, raiseFloorTurbo) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        99 | 134 | 136 => {
            if EV_DoLockedDoor(line, blazeOpen, thing) != 0 {
                P_ChangeSwitchTexture(line, 1);
            }
        }
        138 => {
            EV_LightTurnOn(line, 255);
            P_ChangeSwitchTexture(line, 1);
        }
        139 => {
            EV_LightTurnOn(line, 35);
            P_ChangeSwitchTexture(line, 1);
        }
        _ => {}
    }

    true_
}
