// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_player::*;
use crate::d_think::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_ceilng::*;
use crate::p_doors::*;
use crate::p_floor::*;
use crate::p_inter::*;
use crate::p_lights::*;
use crate::p_mobj::*;
use crate::p_plats::*;
use crate::p_switch::*;
use crate::p_telept::*;
use crate::p_tick::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::c_atoi;

pub const MO_TELEPORTMAN: i32 = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fireflicker_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightflash_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
    pub maxtime: i32,
    pub mintime: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strobe_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub minlight: i32,
    pub maxlight: i32,
    pub darktime: i32,
    pub brighttime: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glow_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub minlight: i32,
    pub maxlight: i32,
    pub direction: i32,
}

pub const GLOWSPEED: i32 = 8;
pub const STROBEBRIGHT: i32 = 5;
pub const FASTDARK: i32 = 15;
pub const SLOWDARK: i32 = 35;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchlist_t {
    pub name1: &'static str,
    pub name2: &'static str,
    pub episode: i16,
}

pub type bwhere_e = i32;
pub const top: bwhere_e = 0;
pub const middle: bwhere_e = 1;
pub const bottom: bwhere_e = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct button_t {
    pub line: *mut line_t,
    pub where_: bwhere_e,
    pub btexture: i32,
    pub btimer: i32,
    pub soundorg: *mut mobj_t,
}

impl Default for button_t {
    fn default() -> Self {
        Self {
            line: std::ptr::null_mut(),
            where_: 0,
            btexture: 0,
            btimer: 0,
            soundorg: std::ptr::null_mut(),
        }
    }
}

pub const MAXSWITCHES: usize = 50;
pub const MAXBUTTONS: usize = 16;
pub const BUTTONTIME: i32 = 35;

pub type plat_e = i32;
pub const up: plat_e = 0;
pub const down: plat_e = 1;
pub const waiting: plat_e = 2;
pub const in_stasis: plat_e = 3;

pub type plattype_e = i32;
pub const perpetualRaise: plattype_e = 0;
pub const downWaitUpStay: plattype_e = 1;
pub const raiseAndChange: plattype_e = 2;
pub const raiseToNearestAndChange: plattype_e = 3;
pub const blazeDWUS: plattype_e = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plat_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub speed: fixed_t,
    pub low: fixed_t,
    pub high: fixed_t,
    pub wait: i32,
    pub count: i32,
    pub status: plat_e,
    pub oldstatus: plat_e,
    pub crush: boolean,
    pub tag: i32,
    pub type_: plattype_e,
}

pub const PLATWAIT: i32 = 3;
pub const PLATSPEED: fixed_t = FRACUNIT;
pub const MAXPLATS: usize = 30;

pub type vldoor_e = i32;
pub const normal: vldoor_e = 0;
pub const close30ThenOpen: vldoor_e = 1;
pub const close: vldoor_e = 2;
pub const open: vldoor_e = 3;
pub const raiseIn5Mins: vldoor_e = 4;
pub const blazeRaise: vldoor_e = 5;
pub const blazeOpen: vldoor_e = 6;
pub const blazeClose: vldoor_e = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vldoor_t {
    pub thinker: thinker_t,
    pub type_: vldoor_e,
    pub sector: *mut sector_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub direction: i32,
    pub topwait: i32,
    pub topcountdown: i32,
}

pub const VDOORSPEED: fixed_t = FRACUNIT * 2;
pub const VDOORWAIT: i32 = 150;

pub type ceiling_e = i32;
pub const lowerToFloor: ceiling_e = 0;
pub const raiseToHighest: ceiling_e = 1;
pub const lowerAndCrush: ceiling_e = 2;
pub const crushAndRaise: ceiling_e = 3;
pub const fastCrushAndRaise: ceiling_e = 4;
pub const silentCrushAndRaise: ceiling_e = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceiling_t {
    pub thinker: thinker_t,
    pub type_: ceiling_e,
    pub sector: *mut sector_t,
    pub bottomheight: fixed_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub crush: boolean,
    pub direction: i32,
    pub tag: i32,
    pub olddirection: i32,
}

pub const CEILSPEED: fixed_t = FRACUNIT;
pub const CEILWAIT: i32 = 150;
pub const MAXCEILINGS: usize = 30;

pub type floor_e = i32;
pub const lowerFloor: floor_e = 0;
pub const lowerFloorToLowest: floor_e = 1;
pub const turboLower: floor_e = 2;
pub const raiseFloor: floor_e = 3;
pub const raiseFloorToNearest: floor_e = 4;
pub const raiseToTexture: floor_e = 5;
pub const lowerAndChange: floor_e = 6;
pub const raiseFloor24: floor_e = 7;
pub const raiseFloor24AndChange: floor_e = 8;
pub const raiseFloorCrush: floor_e = 9;
pub const raiseFloorTurbo: floor_e = 10;
pub const donutRaise: floor_e = 11;
pub const raiseFloor512: floor_e = 12;

pub type stair_e = i32;
pub const build8: stair_e = 0;
pub const turbo16: stair_e = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct floormove_t {
    pub thinker: thinker_t,
    pub type_: floor_e,
    pub crush: boolean,
    pub sector: *mut sector_t,
    pub direction: i32,
    pub newspecial: i32,
    pub texture: i16,
    pub floordestheight: fixed_t,
    pub speed: fixed_t,
}

pub const FLOORSPEED: fixed_t = FRACUNIT;

pub type result_e = i32;
pub const ok: result_e = 0;
pub const crushed: result_e = 1;
pub const pastdest: result_e = 2;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct spec_anim_t {
    pub istexture: boolean,
    pub picnum: i32,
    pub basepic: i32,
    pub numpics: i32,
    pub speed: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct animdef_t {
    pub istexture: i32,
    pub endname: &'static str,
    pub startname: &'static str,
    pub speed: i32,
}

pub const MAXANIMS: usize = 32;
pub const MAXLINEANIMS: usize = 64;

pub static mut animdefs: [animdef_t; 23] = [
    animdef_t {
        istexture: false_,
        endname: "NUKAGE3",
        startname: "NUKAGE1",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "FWATER4",
        startname: "FWATER1",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "SWATER4",
        startname: "SWATER1",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "LAVA4",
        startname: "LAVA1",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "BLOOD3",
        startname: "BLOOD1",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "RROCK08",
        startname: "RROCK05",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "SLIME04",
        startname: "SLIME01",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "SLIME08",
        startname: "SLIME05",
        speed: 8,
    },
    animdef_t {
        istexture: false_,
        endname: "SLIME12",
        startname: "SLIME09",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "BLODGR4",
        startname: "BLODGR1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "SLADRIP3",
        startname: "SLADRIP1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "BLODRIP4",
        startname: "BLODRIP1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "FIREWALL",
        startname: "FIREWALA",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "GSTFONT3",
        startname: "GSTFONT1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "FIRELAVA",
        startname: "FIRELAV3",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "FIREMAG3",
        startname: "FIREMAG1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "FIREBLU2",
        startname: "FIREBLU1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "ROCKRED3",
        startname: "ROCKRED1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "BFALL4",
        startname: "BFALL1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "SFALL4",
        startname: "SFALL1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "WFALL4",
        startname: "WFALL1",
        speed: 8,
    },
    animdef_t {
        istexture: true_,
        endname: "DBRAIN4",
        startname: "DBRAIN1",
        speed: 8,
    },
    animdef_t {
        istexture: -1,
        endname: "",
        startname: "",
        speed: 0,
    },
];

pub static mut anims: [spec_anim_t; MAXANIMS] = [spec_anim_t {
    istexture: 0,
    picnum: 0,
    basepic: 0,
    numpics: 0,
    speed: 0,
}; MAXANIMS];
pub static mut lastanim: *mut spec_anim_t = std::ptr::null_mut();

pub static mut numlinespecials: i16 = 0;
pub static mut linespeciallist: [*mut line_t; MAXLINEANIMS] = [std::ptr::null_mut(); MAXLINEANIMS];

pub static mut levelTimer: boolean = 0;
pub static mut levelTimeCount: i32 = 0;

pub unsafe fn P_InitPicAnims() {
    lastanim = anims.as_mut_ptr();
    let mut i = 0;
    while animdefs[i].istexture != -1 {
        if animdefs[i].istexture != 0 {
            if R_CheckTextureNumForNameStr(animdefs[i].startname) == -1 {
                i += 1;
                continue;
            }
            (*lastanim).picnum = R_TextureNumForNameStr(animdefs[i].endname);
            (*lastanim).basepic = R_TextureNumForNameStr(animdefs[i].startname);
        } else {
            if W_CheckNumForNameStr(animdefs[i].startname) == -1 {
                i += 1;
                continue;
            }
            (*lastanim).picnum = R_FlatNumForNameStr(animdefs[i].endname);
            (*lastanim).basepic = R_FlatNumForNameStr(animdefs[i].startname);
        }

        (*lastanim).istexture = animdefs[i].istexture;
        (*lastanim).numpics = (*lastanim).picnum - (*lastanim).basepic + 1;

        if (*lastanim).numpics < 2 {
            I_Error(&format!(
                "P_InitPicAnims: bad cycle from {} to {}",
                animdefs[i].startname, animdefs[i].endname
            ));
        }

        (*lastanim).speed = animdefs[i].speed;
        lastanim = lastanim.add(1);
        i += 1;
    }
}

pub unsafe fn getSide(currentSector: i32, line: i32, side: i32) -> *mut side_t {
    let sec = sectors.add(currentSector as usize);
    let l = *(*sec).lines.add(line as usize);
    sides.add((*l).sidenum[side as usize] as usize)
}

pub unsafe fn getSector(currentSector: i32, line: i32, side: i32) -> *mut sector_t {
    (*getSide(currentSector, line, side)).sector
}

pub unsafe fn twoSided(sector: i32, line: i32) -> i32 {
    let sec = sectors.add(sector as usize);
    let l = *(*sec).lines.add(line as usize);
    ((*l).flags as i32) & ML_TWOSIDED
}

pub unsafe fn getNextSector(line: *mut line_t, sec: *mut sector_t) -> *mut sector_t {
    if ((*line).flags as i32) & ML_TWOSIDED == 0 {
        return std::ptr::null_mut();
    }
    if (*line).frontsector == sec {
        return (*line).backsector;
    }
    (*line).frontsector
}

pub unsafe fn P_FindLowestFloorSurrounding(sec: *mut sector_t) -> fixed_t {
    let mut floor = (*sec).floorheight;
    for i in 0..((*sec).linecount as usize) {
        let check = *(*sec).lines.add(i);
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).floorheight < floor {
            floor = (*other).floorheight;
        }
    }
    floor
}

pub unsafe fn P_FindHighestFloorSurrounding(sec: *mut sector_t) -> fixed_t {
    let mut floor = -500 * FRACUNIT;
    for i in 0..((*sec).linecount as usize) {
        let check = *(*sec).lines.add(i);
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).floorheight > floor {
            floor = (*other).floorheight;
        }
    }
    floor
}

pub const MAX_ADJOINING_SECTORS: usize = 20;

pub unsafe fn P_FindNextHighestFloor(sec: *mut sector_t, currentheight: i32) -> fixed_t {
    let height = currentheight;
    let mut heightlist = [0; MAX_ADJOINING_SECTORS];
    let mut h = 0usize;

    for i in 0..((*sec).linecount as usize) {
        let check = *(*sec).lines.add(i);
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).floorheight > height {
            heightlist[h] = (*other).floorheight;
            h += 1;
        }
        if h >= MAX_ADJOINING_SECTORS {
            eprintln!("Sector with more than 20 adjoining sectors");
            break;
        }
    }

    if h == 0 {
        return currentheight;
    }

    let mut min = heightlist[0];
    for i in 1..h {
        if heightlist[i] < min {
            min = heightlist[i];
        }
    }

    min
}

pub unsafe fn P_FindLowestCeilingSurrounding(sec: *mut sector_t) -> fixed_t {
    let mut height = MAXINT;
    for i in 0..((*sec).linecount as usize) {
        let check = *(*sec).lines.add(i);
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).ceilingheight < height {
            height = (*other).ceilingheight;
        }
    }
    height
}

pub unsafe fn P_FindHighestCeilingSurrounding(sec: *mut sector_t) -> fixed_t {
    let mut height = 0;
    for i in 0..((*sec).linecount as usize) {
        let check = *(*sec).lines.add(i);
        let other = getNextSector(check, sec);
        if other.is_null() {
            continue;
        }
        if (*other).ceilingheight > height {
            height = (*other).ceilingheight;
        }
    }
    height
}

pub unsafe fn P_FindSectorFromLineTag(line: *mut line_t, start: i32) -> i32 {
    for i in ((start + 1) as usize)..(numsectors as usize) {
        if (*sectors.add(i)).tag == (*line).tag {
            return i as i32;
        }
    }
    -1
}

pub unsafe fn P_FindMinSurroundingLight(sector: *mut sector_t, max: i32) -> i32 {
    let mut min = max;
    for i in 0..((*sector).linecount as usize) {
        let line = *(*sector).lines.add(i);
        let check = getNextSector(line, sector);
        if check.is_null() {
            continue;
        }
        if ((*check).lightlevel as i32) < min {
            min = (*check).lightlevel as i32;
        }
    }
    min
}

pub unsafe fn P_CrossSpecialLine(linenum: i32, side: i32, thing: *mut mobj_t) {
    let line = lines.add(linenum as usize);

    if (*thing).player.is_null() {
        match (*thing).type_ {
            MT_ROCKET | MT_PLASMA | MT_BFG | MT_TROOPSHOT | MT_HEADSHOT | MT_BRUISERSHOT => {
                return;
            }
            _ => {}
        }

        let ok_trig = matches!((*line).special, 39 | 97 | 125 | 126 | 4 | 10 | 88);
        if !ok_trig {
            return;
        }
    }

    match (*line).special {
        2 => {
            EV_DoDoor(line, open);
            (*line).special = 0;
        }
        3 => {
            EV_DoDoor(line, close);
            (*line).special = 0;
        }
        4 => {
            EV_DoDoor(line, normal);
            (*line).special = 0;
        }
        5 => {
            EV_DoFloor(line, raiseFloor);
            (*line).special = 0;
        }
        6 => {
            EV_DoCeiling(line, fastCrushAndRaise);
            (*line).special = 0;
        }
        8 => {
            EV_BuildStairs(line, build8);
            (*line).special = 0;
        }
        10 => {
            EV_DoPlat(line, downWaitUpStay, 0);
            (*line).special = 0;
        }
        12 => {
            EV_LightTurnOn(line, 0);
            (*line).special = 0;
        }
        13 => {
            EV_LightTurnOn(line, 255);
            (*line).special = 0;
        }
        16 => {
            EV_DoDoor(line, close30ThenOpen);
            (*line).special = 0;
        }
        17 => {
            EV_StartLightStrobing(line);
            (*line).special = 0;
        }
        19 => {
            EV_DoFloor(line, lowerFloor);
            (*line).special = 0;
        }
        22 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0);
            (*line).special = 0;
        }
        25 => {
            EV_DoCeiling(line, crushAndRaise);
            (*line).special = 0;
        }
        30 => {
            EV_DoFloor(line, raiseToTexture);
            (*line).special = 0;
        }
        35 => {
            EV_LightTurnOn(line, 35);
            (*line).special = 0;
        }
        36 => {
            EV_DoFloor(line, turboLower);
            (*line).special = 0;
        }
        37 => {
            EV_DoFloor(line, lowerAndChange);
            (*line).special = 0;
        }
        38 => {
            EV_DoFloor(line, lowerFloorToLowest);
            (*line).special = 0;
        }
        39 => {
            EV_Teleport(line, side, thing);
            (*line).special = 0;
        }
        40 => {
            EV_DoCeiling(line, raiseToHighest);
            EV_DoFloor(line, lowerFloorToLowest);
            (*line).special = 0;
        }
        44 => {
            EV_DoCeiling(line, lowerAndCrush);
            (*line).special = 0;
        }
        52 => {
            G_ExitLevel();
        }
        53 => {
            EV_DoPlat(line, perpetualRaise, 0);
            (*line).special = 0;
        }
        54 => {
            EV_StopPlat(line);
            (*line).special = 0;
        }
        56 => {
            EV_DoFloor(line, raiseFloorCrush);
            (*line).special = 0;
        }
        57 => {
            EV_CeilingCrushStop(line);
            (*line).special = 0;
        }
        58 => {
            EV_DoFloor(line, raiseFloor24);
            (*line).special = 0;
        }
        59 => {
            EV_DoFloor(line, raiseFloor24AndChange);
            (*line).special = 0;
        }
        104 => {
            EV_TurnTagLightsOff(line);
            (*line).special = 0;
        }
        108 => {
            EV_DoDoor(line, blazeRaise);
            (*line).special = 0;
        }
        109 => {
            EV_DoDoor(line, blazeOpen);
            (*line).special = 0;
        }
        100 => {
            EV_BuildStairs(line, turbo16);
            (*line).special = 0;
        }
        110 => {
            EV_DoDoor(line, blazeClose);
            (*line).special = 0;
        }
        119 => {
            EV_DoFloor(line, raiseFloorToNearest);
            (*line).special = 0;
        }
        121 => {
            EV_DoPlat(line, blazeDWUS, 0);
            (*line).special = 0;
        }
        124 => {
            G_SecretExitLevel();
        }
        125 => {
            if (*thing).player.is_null() {
                EV_Teleport(line, side, thing);
                (*line).special = 0;
            }
        }
        130 => {
            EV_DoFloor(line, raiseFloorTurbo);
            (*line).special = 0;
        }
        141 => {
            EV_DoCeiling(line, silentCrushAndRaise);
            (*line).special = 0;
        }
        72 => {
            EV_DoCeiling(line, lowerAndCrush);
        }
        73 => {
            EV_DoCeiling(line, crushAndRaise);
        }
        74 => {
            EV_CeilingCrushStop(line);
        }
        75 => {
            EV_DoDoor(line, close);
        }
        76 => {
            EV_DoDoor(line, close30ThenOpen);
        }
        77 => {
            EV_DoCeiling(line, fastCrushAndRaise);
        }
        79 => {
            EV_LightTurnOn(line, 35);
        }
        80 => {
            EV_LightTurnOn(line, 0);
        }
        81 => {
            EV_LightTurnOn(line, 255);
        }
        82 => {
            EV_DoFloor(line, lowerFloorToLowest);
        }
        83 => {
            EV_DoFloor(line, lowerFloor);
        }
        84 => {
            EV_DoFloor(line, lowerAndChange);
        }
        86 => {
            EV_DoDoor(line, open);
        }
        87 => {
            EV_DoPlat(line, perpetualRaise, 0);
        }
        88 => {
            EV_DoPlat(line, downWaitUpStay, 0);
        }
        89 => {
            EV_StopPlat(line);
        }
        90 => {
            EV_DoDoor(line, normal);
        }
        91 => {
            EV_DoFloor(line, raiseFloor);
        }
        92 => {
            EV_DoFloor(line, raiseFloor24);
        }
        93 => {
            EV_DoFloor(line, raiseFloor24AndChange);
        }
        94 => {
            EV_DoFloor(line, raiseFloorCrush);
        }
        95 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0);
        }
        96 => {
            EV_DoFloor(line, raiseToTexture);
        }
        97 => {
            EV_Teleport(line, side, thing);
        }
        98 => {
            EV_DoFloor(line, turboLower);
        }
        105 => {
            EV_DoDoor(line, blazeRaise);
        }
        106 => {
            EV_DoDoor(line, blazeOpen);
        }
        107 => {
            EV_DoDoor(line, blazeClose);
        }
        120 => {
            EV_DoPlat(line, blazeDWUS, 0);
        }
        126 => {
            if (*thing).player.is_null() {
                EV_Teleport(line, side, thing);
            }
        }
        128 => {
            EV_DoFloor(line, raiseFloorToNearest);
        }
        129 => {
            EV_DoFloor(line, raiseFloorTurbo);
        }
        _ => {}
    }
}

pub unsafe fn P_ShootSpecialLine(thing: *mut mobj_t, line: *mut line_t) {
    if (*thing).player.is_null() {
        if (*line).special != 46 {
            return;
        }
    }

    match (*line).special {
        24 => {
            EV_DoFloor(line, raiseFloor);
            P_ChangeSwitchTexture(line, 0);
        }
        46 => {
            EV_DoDoor(line, open);
            P_ChangeSwitchTexture(line, 1);
        }
        47 => {
            EV_DoPlat(line, raiseToNearestAndChange, 0);
            P_ChangeSwitchTexture(line, 0);
        }
        _ => {}
    }
}

pub unsafe fn P_PlayerInSpecialSector(player: *mut player_t) {
    let sector = (*(*(*player).mo).subsector).sector;

    if (*(*player).mo).z != (*sector).floorheight {
        return;
    }

    match (*sector).special {
        5 => {
            if (*player).powers[pw_ironfeet as usize] == 0 {
                if (leveltime & 0x1f) == 0 {
                    P_DamageMobj((*player).mo, std::ptr::null_mut(), std::ptr::null_mut(), 10);
                }
            }
        }
        7 => {
            if (*player).powers[pw_ironfeet as usize] == 0 {
                if (leveltime & 0x1f) == 0 {
                    P_DamageMobj((*player).mo, std::ptr::null_mut(), std::ptr::null_mut(), 5);
                }
            }
        }
        16 | 4 => {
            if (*player).powers[pw_ironfeet as usize] == 0 || P_Random() < 5 {
                if (leveltime & 0x1f) == 0 {
                    P_DamageMobj((*player).mo, std::ptr::null_mut(), std::ptr::null_mut(), 20);
                }
            }
        }
        9 => {
            (*player).secretcount += 1;
            (*sector).special = 0;
        }
        11 => {
            (*player).cheats &= !CF_GODMODE;
            if (leveltime & 0x1f) == 0 {
                P_DamageMobj((*player).mo, std::ptr::null_mut(), std::ptr::null_mut(), 20);
            }
            if (*player).health <= 10 {
                G_ExitLevel();
            }
        }
        _ => {
            I_Error(&format!(
                "P_PlayerInSpecialSector: unknown special {}",
                (*sector).special
            ));
        }
    }
}

pub unsafe fn P_UpdateSpecials() {
    if levelTimer == true_ {
        levelTimeCount -= 1;
        if levelTimeCount == 0 {
            G_ExitLevel();
        }
    }

    let mut anim = anims.as_mut_ptr();
    while (anim as usize) < (lastanim as usize) {
        for i in (*anim).basepic..((*anim).basepic + (*anim).numpics) {
            let pic = (*anim).basepic + ((leveltime / (*anim).speed + i) % (*anim).numpics);
            if (*anim).istexture != 0 {
                *texturetranslation.add(i as usize) = pic;
            } else {
                *flattranslation.add(i as usize) = pic;
            }
        }
        anim = anim.add(1);
    }

    for i in 0..(numlinespecials as usize) {
        let line = linespeciallist[i];
        if (*line).special == 48 {
            (*sides.add((*line).sidenum[0] as usize)).textureoffset += FRACUNIT;
        }
    }

    for i in 0..MAXBUTTONS {
        if buttonlist[i].btimer != 0 {
            buttonlist[i].btimer -= 1;
            if buttonlist[i].btimer == 0 {
                match buttonlist[i].where_ {
                    top => {
                        (*sides.add((*buttonlist[i].line).sidenum[0] as usize)).toptexture =
                            buttonlist[i].btexture as i16;
                    }
                    middle => {
                        (*sides.add((*buttonlist[i].line).sidenum[0] as usize)).midtexture =
                            buttonlist[i].btexture as i16;
                    }
                    bottom => {
                        (*sides.add((*buttonlist[i].line).sidenum[0] as usize)).bottomtexture =
                            buttonlist[i].btexture as i16;
                    }
                    _ => {}
                }
                S_StartSound(buttonlist[i].soundorg as *mut c_void, sfx_swtchn);
                buttonlist[i] = button_t::default();
            }
        }
    }
}

pub unsafe fn EV_DoDonut(line: *mut line_t) -> i32 {
    let mut secnum = -1;
    let mut rtn = 0;

    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if secnum < 0 {
            break;
        }
        let s1 = sectors.add(secnum as usize);
        if !(*s1).specialdata.is_null() {
            continue;
        }

        rtn = 1;
        let s2 = getNextSector(*(*s1).lines.add(0), s1);
        for i in 0..((*s2).linecount as usize) {
            let l = *(*s2).lines.add(i);
            // In C: `if ((!s2->lines[i]->flags & ML_TWOSIDED) || (s2->lines[i]->backsector == s1))`
            // Note: `!s2->lines[i]->flags & ML_TWOSIDED` in C evaluates `(!flags) & 4` which is 0!
            if ((((*l).flags == 0) as i32) & ML_TWOSIDED != 0) || (*l).backsector == s1 {
                continue;
            }
            let s3 = (*l).backsector;

            let floor = Z_Malloc(
                std::mem::size_of::<floormove_t>() as i32,
                PU_LEVSPEC,
                std::ptr::null_mut(),
            ) as *mut floormove_t;
            P_AddThinker(&mut (*floor).thinker);
            (*s2).specialdata = floor as *mut c_void;
            (*floor).thinker.function = actionf_t::acp1(T_MoveFloor);
            (*floor).type_ = donutRaise;
            (*floor).crush = false_;
            (*floor).direction = 1;
            (*floor).sector = s2;
            (*floor).speed = FLOORSPEED / 2;
            (*floor).texture = (*s3).floorpic;
            (*floor).newspecial = 0;
            (*floor).floordestheight = (*s3).floorheight;

            let floor2 = Z_Malloc(
                std::mem::size_of::<floormove_t>() as i32,
                PU_LEVSPEC,
                std::ptr::null_mut(),
            ) as *mut floormove_t;
            P_AddThinker(&mut (*floor2).thinker);
            (*s1).specialdata = floor2 as *mut c_void;
            (*floor2).thinker.function = actionf_t::acp1(T_MoveFloor);
            (*floor2).type_ = lowerFloor;
            (*floor2).crush = false_;
            (*floor2).direction = -1;
            (*floor2).sector = s1;
            (*floor2).speed = FLOORSPEED / 2;
            (*floor2).floordestheight = (*s3).floorheight;
            break;
        }
    }

    rtn
}

pub unsafe fn P_SpawnSpecials() {
    levelTimer = false_;

    let i_avg = M_CheckParmStr("-avg");
    if i_avg != 0 && deathmatch != 0 {
        levelTimer = true_;
        levelTimeCount = 20 * 60 * 35;
    }

    let i_timer = M_CheckParmStr("-timer");
    if i_timer != 0 && deathmatch != 0 {
        let time = c_atoi(*myargv.add((i_timer + 1) as usize)) * 60 * 35;
        levelTimer = true_;
        levelTimeCount = time;
    }

    for i in 0..(numsectors as usize) {
        let sector = sectors.add(i);
        if (*sector).special == 0 {
            continue;
        }

        match (*sector).special {
            1 => P_SpawnLightFlash(sector),
            2 => P_SpawnStrobeFlash(sector, FASTDARK, 0),
            3 => P_SpawnStrobeFlash(sector, SLOWDARK, 0),
            4 => {
                P_SpawnStrobeFlash(sector, FASTDARK, 0);
                (*sector).special = 4;
            }
            8 => P_SpawnGlowingLight(sector),
            9 => totalsecret += 1,
            10 => P_SpawnDoorCloseIn30(sector),
            12 => P_SpawnStrobeFlash(sector, SLOWDARK, 1),
            13 => P_SpawnStrobeFlash(sector, FASTDARK, 1),
            14 => P_SpawnDoorRaiseIn5Mins(sector, i as i32),
            17 => P_SpawnFireFlicker(sector),
            _ => {}
        }
    }

    numlinespecials = 0;
    for i in 0..(numlines as usize) {
        if (*lines.add(i)).special == 48 {
            linespeciallist[numlinespecials as usize] = lines.add(i);
            numlinespecials += 1;
        }
    }

    for i in 0..MAXCEILINGS {
        activeceilings[i] = std::ptr::null_mut();
    }

    for i in 0..MAXPLATS {
        activeplats[i] = std::ptr::null_mut();
    }

    for i in 0..MAXBUTTONS {
        buttonlist[i] = button_t::default();
    }
}
