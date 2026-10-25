// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::am_map::*;
use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_video::*;
use crate::m_cheat::*;
use crate::m_random::*;
use crate::p_inter::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::st_lib::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::write_c_str;

pub const ST_HEIGHT: i32 = 32 * SCREEN_MUL;
pub const ST_WIDTH: i32 = SCREENWIDTH;
pub const ST_Y: i32 = SCREENHEIGHT - ST_HEIGHT;

pub type st_stateenum_t = i32;
pub const AutomapState: st_stateenum_t = 0;
pub const FirstPersonState: st_stateenum_t = 1;

pub type st_chatstateenum_t = i32;
pub const StartChatState: st_chatstateenum_t = 0;
pub const WaitDestState: st_chatstateenum_t = 1;
pub const GetChatState: st_chatstateenum_t = 2;

const STARTREDPALS: i32 = 1;
const STARTBONUSPALS: i32 = 9;
const NUMREDPALS: i32 = 8;
const NUMBONUSPALS: i32 = 4;
const RADIATIONPAL: i32 = 13;

const ST_FACEPROBABILITY: i32 = 96;
const ST_TOGGLECHAT: i32 = KEY_ENTER;
const ST_X: i32 = 0;
const ST_X2: i32 = 104;
const ST_FX: i32 = 143;
const ST_FY: i32 = 169;

const ST_NUMPAINFACES: usize = 5;
const ST_NUMSTRAIGHTFACES: usize = 3;
const ST_NUMTURNFACES: usize = 2;
const ST_NUMSPECIALFACES: usize = 3;
const ST_FACESTRIDE: usize = ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES + ST_NUMSPECIALFACES;
const ST_NUMEXTRAFACES: usize = 2;
const ST_NUMFACES: usize = ST_FACESTRIDE * ST_NUMPAINFACES + ST_NUMEXTRAFACES;

const ST_TURNOFFSET: usize = ST_NUMSTRAIGHTFACES;
const ST_OUCHOFFSET: usize = ST_TURNOFFSET + ST_NUMTURNFACES;
const ST_EVILGRINOFFSET: usize = ST_OUCHOFFSET + 1;
const ST_RAMPAGEOFFSET: usize = ST_EVILGRINOFFSET + 1;
const ST_GODFACE: usize = ST_NUMPAINFACES * ST_FACESTRIDE;
const ST_DEADFACE: usize = ST_GODFACE + 1;

const ST_FACESX: i32 = 143;
const ST_FACESY: i32 = 168;

const ST_EVILGRINCOUNT: i32 = 2 * TICRATE;
const ST_STRAIGHTFACECOUNT: i32 = TICRATE / 2;
const ST_TURNCOUNT: i32 = 1 * TICRATE;
const ST_OUCHCOUNT: i32 = 1 * TICRATE;
const ST_RAMPAGEDELAY: i32 = 2 * TICRATE;
const ST_MUCHPAIN: i32 = 20;

const ST_AMMOWIDTH: i32 = 3;
const ST_AMMOX: i32 = 44;
const ST_AMMOY: i32 = 171;

const ST_HEALTHWIDTH: i32 = 3;
const ST_HEALTHX: i32 = 90;
const ST_HEALTHY: i32 = 171;

const ST_ARMSX: i32 = 111;
const ST_ARMSY: i32 = 172;
const ST_ARMSBGX: i32 = 104;
const ST_ARMSBGY: i32 = 168;
const ST_ARMSXSPACE: i32 = 12;
const ST_ARMSYSPACE: i32 = 10;

const ST_FRAGSX: i32 = 138;
const ST_FRAGSY: i32 = 171;
const ST_FRAGSWIDTH: i32 = 2;

const ST_ARMORWIDTH: i32 = 3;
const ST_ARMORX: i32 = 221;
const ST_ARMORY: i32 = 171;

const ST_KEY0WIDTH: i32 = 8;
const ST_KEY0HEIGHT: i32 = 5;
const ST_KEY0X: i32 = 239;
const ST_KEY0Y: i32 = 171;
const ST_KEY1WIDTH: i32 = ST_KEY0WIDTH;
const ST_KEY1X: i32 = 239;
const ST_KEY1Y: i32 = 181;
const ST_KEY2WIDTH: i32 = ST_KEY0WIDTH;
const ST_KEY2X: i32 = 239;
const ST_KEY2Y: i32 = 191;

const ST_AMMO0WIDTH: i32 = 3;
const ST_AMMO0HEIGHT: i32 = 6;
const ST_AMMO0X: i32 = 288;
const ST_AMMO0Y: i32 = 173;
const ST_AMMO1WIDTH: i32 = ST_AMMO0WIDTH;
const ST_AMMO1X: i32 = 288;
const ST_AMMO1Y: i32 = 179;
const ST_AMMO2WIDTH: i32 = ST_AMMO0WIDTH;
const ST_AMMO2X: i32 = 288;
const ST_AMMO2Y: i32 = 191;
const ST_AMMO3WIDTH: i32 = ST_AMMO0WIDTH;
const ST_AMMO3X: i32 = 288;
const ST_AMMO3Y: i32 = 185;

const ST_MAXAMMO0WIDTH: i32 = 3;
const ST_MAXAMMO0HEIGHT: i32 = 5;
const ST_MAXAMMO0X: i32 = 314;
const ST_MAXAMMO0Y: i32 = 173;
const ST_MAXAMMO1WIDTH: i32 = ST_MAXAMMO0WIDTH;
const ST_MAXAMMO1X: i32 = 314;
const ST_MAXAMMO1Y: i32 = 179;
const ST_MAXAMMO2WIDTH: i32 = ST_MAXAMMO0WIDTH;
const ST_MAXAMMO2X: i32 = 314;
const ST_MAXAMMO2Y: i32 = 191;
const ST_MAXAMMO3WIDTH: i32 = ST_MAXAMMO0WIDTH;
const ST_MAXAMMO3X: i32 = 314;
const ST_MAXAMMO3Y: i32 = 185;

const ST_MSGWIDTH: usize = 52;

static mut plyr: *mut player_t = std::ptr::null_mut();
static mut st_firsttime: boolean = 0;
static mut veryfirsttime: i32 = 1;
static mut lu_palette: i32 = 0;
static mut st_clock: u32 = 0;
static mut st_msgcounter: i32 = 0;
static mut st_chatstate: st_chatstateenum_t = 0;
static mut st_gamestate: st_stateenum_t = 0;
static mut st_statusbaron: boolean = 0;
static mut st_chat: boolean = 0;
static mut st_oldchat: boolean = 0;
static mut st_cursoron: boolean = 0;
static mut st_notdeathmatch: boolean = 0;
static mut st_armson: boolean = 0;
static mut st_fragson: boolean = 0;

static mut sbar: *mut patch_t = std::ptr::null_mut();
static mut tallnum: [*mut patch_t; 10] = [std::ptr::null_mut(); 10];
static mut tallpercent: *mut patch_t = std::ptr::null_mut();
static mut shortnum: [*mut patch_t; 10] = [std::ptr::null_mut(); 10];
static mut keys: [*mut patch_t; NUMCARDS] = [std::ptr::null_mut(); NUMCARDS];
static mut faces: [*mut patch_t; ST_NUMFACES] = [std::ptr::null_mut(); ST_NUMFACES];
static mut faceback: *mut patch_t = std::ptr::null_mut();
static mut armsbg: *mut patch_t = std::ptr::null_mut();
static mut arms: [[*mut patch_t; 2]; 6] = [[std::ptr::null_mut(); 2]; 6];

static mut w_ready: st_number_t = st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
};
static mut w_frags: st_number_t = st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
};
static mut w_health: st_percent_t = st_percent_t {
    n: st_number_t {
        x: 0,
        y: 0,
        width: 0,
        oldnum: 0,
        num: std::ptr::null_mut(),
        on: std::ptr::null_mut(),
        p: std::ptr::null_mut(),
        data: 0,
    },
    p: std::ptr::null_mut(),
};
static mut w_armsbg: st_binicon_t = st_binicon_t {
    x: 0,
    y: 0,
    oldval: 0,
    val: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
};
static mut w_arms: [st_multicon_t; 6] = [st_multicon_t {
    x: 0,
    y: 0,
    oldinum: 0,
    inum: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
}; 6];
static mut w_faces: st_multicon_t = st_multicon_t {
    x: 0,
    y: 0,
    oldinum: 0,
    inum: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
};
static mut w_keyboxes: [st_multicon_t; 3] = [st_multicon_t {
    x: 0,
    y: 0,
    oldinum: 0,
    inum: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
}; 3];
static mut w_armor: st_percent_t = st_percent_t {
    n: st_number_t {
        x: 0,
        y: 0,
        width: 0,
        oldnum: 0,
        num: std::ptr::null_mut(),
        on: std::ptr::null_mut(),
        p: std::ptr::null_mut(),
        data: 0,
    },
    p: std::ptr::null_mut(),
};
static mut w_ammo: [st_number_t; 4] = [st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
}; 4];
static mut w_maxammo: [st_number_t; 4] = [st_number_t {
    x: 0,
    y: 0,
    width: 0,
    oldnum: 0,
    num: std::ptr::null_mut(),
    on: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
    data: 0,
}; 4];

static mut st_fragscount: i32 = 0;
static mut st_oldhealth: i32 = -1;
static mut oldweaponsowned: [boolean; NUMWEAPONS] = [0; NUMWEAPONS];
static mut st_facecount: i32 = 0;
static mut st_faceindex: i32 = 0;
static mut keyboxes: [i32; 3] = [0; 3];
static mut st_randomnumber: i32 = 0;

pub static mut cheat_mus_seq: [u8; 9] = [0xb2, 0x26, 0xb6, 0xae, 0xea, 1, 0, 0, 0xff];
pub static mut cheat_choppers_seq: [u8; 11] =
    [0xb2, 0x26, 0xe2, 0x32, 0xf6, 0x2a, 0x2a, 0xa6, 0x6a, 0xea, 0xff];
pub static mut cheat_god_seq: [u8; 6] = [0xb2, 0x26, 0x26, 0xaa, 0x26, 0xff];
pub static mut cheat_ammo_seq: [u8; 6] = [0xb2, 0x26, 0xf2, 0x66, 0xa2, 0xff];
pub static mut cheat_ammonokey_seq: [u8; 5] = [0xb2, 0x26, 0x66, 0xa2, 0xff];
pub static mut cheat_noclip_seq: [u8; 11] =
    [0xb2, 0x26, 0xea, 0x2a, 0xb2, 0xea, 0x2a, 0xf6, 0x2a, 0x26, 0xff];
pub static mut cheat_commercial_noclip_seq: [u8; 7] = [0xb2, 0x26, 0xe2, 0x36, 0xb2, 0x2a, 0xff];

pub static mut cheat_powerup_seq: [[u8; 10]; 7] = [
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6e, 0xff],
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xea, 0xff],
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xb2, 0xff],
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6a, 0xff],
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xa2, 0xff],
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x36, 0xff],
    [0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xff, 0],
];

pub static mut cheat_clev_seq: [u8; 10] = [0xb2, 0x26, 0xe2, 0x36, 0xa6, 0x6e, 1, 0, 0, 0xff];
pub static mut cheat_mypos_seq: [u8; 8] = [0xb2, 0x26, 0xb6, 0xba, 0x2a, 0xf6, 0xea, 0xff];

pub static mut cheat_mus: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_god: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_ammo: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_ammonokey: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_noclip: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_commercial_noclip: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_powerup: [cheatseq_t; 7] = [cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
}; 7];
pub static mut cheat_choppers: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_clev: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};
pub static mut cheat_mypos: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};

unsafe fn init_cheats_if_needed() {
    if cheat_mus.sequence.is_null() {
        cheat_mus.sequence = cheat_mus_seq.as_mut_ptr();
        cheat_god.sequence = cheat_god_seq.as_mut_ptr();
        cheat_ammo.sequence = cheat_ammo_seq.as_mut_ptr();
        cheat_ammonokey.sequence = cheat_ammonokey_seq.as_mut_ptr();
        cheat_noclip.sequence = cheat_noclip_seq.as_mut_ptr();
        cheat_commercial_noclip.sequence = cheat_commercial_noclip_seq.as_mut_ptr();
        for i in 0..7 {
            cheat_powerup[i].sequence = cheat_powerup_seq[i].as_mut_ptr();
        }
        cheat_choppers.sequence = cheat_choppers_seq.as_mut_ptr();
        cheat_clev.sequence = cheat_clev_seq.as_mut_ptr();
        cheat_mypos.sequence = cheat_mypos_seq.as_mut_ptr();
    }
}

pub unsafe fn ST_refreshBackground() {
    if st_statusbaron != 0 {
        V_DrawPatch(ST_X, 0, BG, sbar);
        if netgame != 0 {
            V_DrawPatch(ST_FX, 0, BG, faceback);
        }
        V_CopyRect(ST_X, 0, BG, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y, FG);
    }
}

static mut mypos_buf: [u8; ST_MSGWIDTH] = [0; ST_MSGWIDTH];

pub unsafe fn ST_Responder(ev: *mut event_t) -> boolean {
    init_cheats_if_needed();

    if (*ev).type_ == ev_keyup && (((*ev).data1 as u32 & 0xffff0000) == AM_MSGHEADER as u32) {
        match (*ev).data1 {
            AM_MSGENTERED => {
                st_gamestate = AutomapState;
                st_firsttime = true_;
            }
            AM_MSGEXITED => {
                st_gamestate = FirstPersonState;
            }
            _ => {}
        }
    } else if (*ev).type_ == ev_keydown {
        let key = (*ev).data1 as u8;
        if netgame == 0 {
            if cht_CheckCheat(&mut cheat_god, key) != 0 {
                (*plr).cheats ^= CF_GODMODE;
                if (*plr).cheats & CF_GODMODE != 0 {
                    if !(*plr).mo.is_null() {
                        (*(*plr).mo).health = 100;
                    }
                    (*plr).health = 100;
                    (*plr).message = b"Degreelessness Mode On\0".as_ptr();
                } else {
                    (*plr).message = b"Degreelessness Mode Off\0".as_ptr();
                }
            } else if cht_CheckCheat(&mut cheat_ammonokey, key) != 0 {
                (*plr).armorpoints = 200;
                (*plr).armortype = 2;
                for i in 0..NUMWEAPONS {
                    (*plr).weaponowned[i] = true_;
                }
                for i in 0..NUMAMMO {
                    (*plr).ammo[i] = (*plr).maxammo[i];
                }
                (*plr).message = b"Ammo (no keys) Added\0".as_ptr();
            } else if cht_CheckCheat(&mut cheat_ammo, key) != 0 {
                (*plr).armorpoints = 200;
                (*plr).armortype = 2;
                for i in 0..NUMWEAPONS {
                    (*plr).weaponowned[i] = true_;
                }
                for i in 0..NUMAMMO {
                    (*plr).ammo[i] = (*plr).maxammo[i];
                }
                for i in 0..NUMCARDS {
                    (*plr).cards[i] = true_;
                }
                (*plr).message = b"Very Happy Ammo Added\0".as_ptr();
            } else if cht_CheckCheat(&mut cheat_mus, key) != 0 {
                let mut buf = [0u8; 3];
                (*plr).message = b"Music Change\0".as_ptr();
                cht_GetParam(&mut cheat_mus, buf.as_mut_ptr());

                if gamemode == commercial {
                    let musnum =
                        mus_runnin + ((buf[0] - b'0') as i32) * 10 + ((buf[1] - b'0') as i32) - 1;
                    if (((buf[0] - b'0') as i32) * 10 + ((buf[1] - b'0') as i32)) > 35 {
                        (*plr).message = b"IMPOSSIBLE SELECTION\0".as_ptr();
                    } else {
                        S_ChangeMusic(musnum, 1);
                    }
                } else {
                    let musnum =
                        mus_e1m1 + ((buf[0] - b'1') as i32) * 9 + ((buf[1] - b'1') as i32);
                    if (((buf[0] - b'1') as i32) * 9 + ((buf[1] - b'1') as i32)) > 31 {
                        (*plr).message = b"IMPOSSIBLE SELECTION\0".as_ptr();
                    } else {
                        S_ChangeMusic(musnum, 1);
                    }
                }
            } else if cht_CheckCheat(&mut cheat_noclip, key) != 0
                || cht_CheckCheat(&mut cheat_commercial_noclip, key) != 0
            {
                (*plr).cheats ^= CF_NOCLIP;
                if (*plr).cheats & CF_NOCLIP != 0 {
                    (*plr).message = b"No Clipping Mode ON\0".as_ptr();
                } else {
                    (*plr).message = b"No Clipping Mode OFF\0".as_ptr();
                }
            }

            for i in 0..6 {
                if cht_CheckCheat(&mut cheat_powerup[i], key) != 0 {
                    if (*plr).powers[i] == 0 {
                        P_GivePower(plr, i as i32);
                    } else if i != (pw_strength as usize) {
                        (*plr).powers[i] = 1;
                    } else {
                        (*plr).powers[i] = 0;
                    }
                    (*plr).message = b"Power-up Toggled\0".as_ptr();
                }
            }

            if cht_CheckCheat(&mut cheat_powerup[6], key) != 0 {
                (*plr).message = b"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp\0".as_ptr();
            } else if cht_CheckCheat(&mut cheat_choppers, key) != 0 {
                (*plr).weaponowned[wp_chainsaw as usize] = true_;
                (*plr).powers[pw_invulnerability as usize] = true_;
                (*plr).message = b"... doesn't suck - GM\0".as_ptr();
            } else if cht_CheckCheat(&mut cheat_mypos, key) != 0 {
                let s = format!(
                    "ang=0x{:x};x,y=(0x{:x},0x{:x})",
                    (*players[consoleplayer as usize].mo).angle,
                    (*players[consoleplayer as usize].mo).x,
                    (*players[consoleplayer as usize].mo).y
                );
                write_c_str(mypos_buf.as_mut_ptr(), &s);
                (*plr).message = mypos_buf.as_ptr();
            }
        }

        if cht_CheckCheat(&mut cheat_clev, key) != 0 {
            let mut buf = [0u8; 3];
            cht_GetParam(&mut cheat_clev, buf.as_mut_ptr());

            let (epsd, map) = if gamemode == commercial {
                (
                    0,
                    ((buf[0] - b'0') as i32) * 10 + ((buf[1] - b'0') as i32),
                )
            } else {
                ((buf[0] - b'0') as i32, (buf[1] - b'0') as i32)
            };

            if epsd < 1 || map < 1 {
                return false_;
            }
            if gamemode == retail && (epsd > 4 || map > 9) {
                return false_;
            }
            if gamemode == registered && (epsd > 3 || map > 9) {
                return false_;
            }
            if gamemode == shareware && (epsd > 1 || map > 9) {
                return false_;
            }
            if gamemode == commercial && (epsd > 1 || map > 34) {
                return false_;
            }

            (*plr).message = b"Changing Level...\0".as_ptr();
            G_DeferedInitNew(gameskill, epsd, map);
        }
    }
    false_
}

static mut lastcalc: i32 = 0;
static mut oldhealth_calc: i32 = -1;

pub unsafe fn ST_calcPainOffset() -> i32 {
    let health = if (*plr).health > 100 {
        100
    } else {
        (*plr).health
    };

    if health != oldhealth_calc {
        lastcalc = (ST_FACESTRIDE as i32) * (((100 - health) * (ST_NUMPAINFACES as i32)) / 101);
        oldhealth_calc = health;
    }
    lastcalc
}

static mut lastattackdown: i32 = -1;
static mut priority: i32 = 0;

pub unsafe fn ST_updateFaceWidget() {
    if priority < 10 {
        if (*plr).health == 0 {
            priority = 9;
            st_faceindex = ST_DEADFACE as i32;
            st_facecount = 1;
        }
    }

    if priority < 9 {
        if (*plr).bonuscount != 0 {
            let mut doevilgrin = false;
            for i in 0..NUMWEAPONS {
                if oldweaponsowned[i] != (*plr).weaponowned[i] {
                    doevilgrin = true;
                    oldweaponsowned[i] = (*plr).weaponowned[i];
                }
            }
            if doevilgrin {
                priority = 8;
                st_facecount = ST_EVILGRINCOUNT;
                st_faceindex = ST_calcPainOffset() + (ST_EVILGRINOFFSET as i32);
            }
        }
    }

    if priority < 8 {
        if (*plr).damagecount != 0 && !(*plr).attacker.is_null() && (*plr).attacker != (*plr).mo {
            priority = 7;
            if (*plr).health - st_oldhealth > ST_MUCHPAIN {
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset() + (ST_OUCHOFFSET as i32);
            } else {
                let badguyangle = R_PointToAngle2(
                    (*(*plr).mo).x,
                    (*(*plr).mo).y,
                    (*(*plr).attacker).x,
                    (*(*plr).attacker).y,
                );
                let (diffang, i) = if badguyangle > (*(*plr).mo).angle {
                    let d = badguyangle.wrapping_sub((*(*plr).mo).angle);
                    (d, d > ANG180)
                } else {
                    let d = (*(*plr).mo).angle.wrapping_sub(badguyangle);
                    (d, d <= ANG180)
                };

                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset();

                if diffang < ANG45 {
                    st_faceindex += ST_RAMPAGEOFFSET as i32;
                } else if i {
                    st_faceindex += ST_TURNOFFSET as i32;
                } else {
                    st_faceindex += (ST_TURNOFFSET + 1) as i32;
                }
            }
        }
    }

    if priority < 7 {
        if (*plr).damagecount != 0 {
            if (*plr).health - st_oldhealth > ST_MUCHPAIN {
                priority = 7;
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset() + (ST_OUCHOFFSET as i32);
            } else {
                priority = 6;
                st_facecount = ST_TURNCOUNT;
                st_faceindex = ST_calcPainOffset() + (ST_RAMPAGEOFFSET as i32);
            }
        }
    }

    if priority < 6 {
        if (*plr).attackdown != 0 {
            if lastattackdown == -1 {
                lastattackdown = ST_RAMPAGEDELAY;
            } else {
                lastattackdown -= 1;
                if lastattackdown == 0 {
                    priority = 5;
                    st_faceindex = ST_calcPainOffset() + (ST_RAMPAGEOFFSET as i32);
                    st_facecount = 1;
                    lastattackdown = 1;
                }
            }
        } else {
            lastattackdown = -1;
        }
    }

    if priority < 5 {
        if ((*plr).cheats & CF_GODMODE != 0) || (*plr).powers[pw_invulnerability as usize] != 0 {
            priority = 4;
            st_faceindex = ST_GODFACE as i32;
            st_facecount = 1;
        }
    }

    if st_facecount == 0 {
        st_faceindex = ST_calcPainOffset() + (st_randomnumber % 3);
        st_facecount = ST_STRAIGHTFACECOUNT;
        priority = 0;
    }

    st_facecount -= 1;
}

static mut largeammo: i32 = 1994;

pub unsafe fn ST_updateWidgets() {
    if weaponinfo[(*plr).readyweapon as usize].ammo == am_noammo {
        w_ready.num = &mut largeammo;
    } else {
        w_ready.num = &mut (*plr).ammo[weaponinfo[(*plr).readyweapon as usize].ammo as usize];
    }
    w_ready.data = (*plr).readyweapon;

    for i in 0..3 {
        keyboxes[i] = if (*plr).cards[i] != 0 { i as i32 } else { -1 };
        if (*plr).cards[i + 3] != 0 {
            keyboxes[i] = (i + 3) as i32;
        }
    }

    ST_updateFaceWidget();

    st_notdeathmatch = (deathmatch == 0) as boolean;
    st_armson = (st_statusbaron != 0 && deathmatch == 0) as boolean;
    st_fragson = (deathmatch != 0 && st_statusbaron != 0) as boolean;
    st_fragscount = 0;

    for i in 0..MAXPLAYERS {
        if i != (consoleplayer as usize) {
            st_fragscount += (*plr).frags[i];
        } else {
            st_fragscount -= (*plr).frags[i];
        }
    }

    st_msgcounter -= 1;
    if st_msgcounter == 0 {
        st_chat = st_oldchat;
    }
}

pub unsafe fn ST_Ticker() {
    st_clock = st_clock.wrapping_add(1);
    st_randomnumber = M_Random();
    ST_updateWidgets();
    st_oldhealth = (*plr).health;
}

static mut st_palette: i32 = 0;

pub unsafe fn ST_doPaletteStuff() {
    let mut cnt = (*plr).damagecount;

    if (*plr).powers[pw_strength as usize] != 0 {
        let bzc = 12 - ((*plr).powers[pw_strength as usize] >> 6);
        if bzc > cnt {
            cnt = bzc;
        }
    }

    let palette = if cnt != 0 {
        let mut p = (cnt + 7) >> 3;
        if p >= NUMREDPALS {
            p = NUMREDPALS - 1;
        }
        p + STARTREDPALS
    } else if (*plr).bonuscount != 0 {
        let mut p = ((*plr).bonuscount + 7) >> 3;
        if p >= NUMBONUSPALS {
            p = NUMBONUSPALS - 1;
        }
        p + STARTBONUSPALS
    } else if (*plr).powers[pw_ironfeet as usize] > 4 * 32
        || ((*plr).powers[pw_ironfeet as usize] & 8 != 0)
    {
        RADIATIONPAL
    } else {
        0
    };

    if palette != st_palette {
        st_palette = palette;
        let pal = (W_CacheLumpNum(lu_palette, PU_CACHE) as *mut byte).add((palette * 768) as usize);
        I_SetPalette(pal);
    }
}

pub unsafe fn ST_drawWidgets(refresh: boolean) {
    st_armson = (st_statusbaron != 0 && deathmatch == 0) as boolean;
    st_fragson = (deathmatch != 0 && st_statusbaron != 0) as boolean;

    STlib_updateNum(&mut w_ready, refresh);

    for i in 0..4 {
        STlib_updateNum(&mut w_ammo[i], refresh);
        STlib_updateNum(&mut w_maxammo[i], refresh);
    }

    STlib_updatePercent(&mut w_health, refresh);
    STlib_updatePercent(&mut w_armor, refresh);

    STlib_updateBinIcon(&mut w_armsbg, refresh);

    for i in 0..6 {
        STlib_updateMultIcon(&mut w_arms[i], refresh);
    }

    STlib_updateMultIcon(&mut w_faces, refresh);

    for i in 0..3 {
        STlib_updateMultIcon(&mut w_keyboxes[i], refresh);
    }

    STlib_updateNum(&mut w_frags, refresh);
}

pub unsafe fn ST_doRefresh() {
    st_firsttime = false_;
    ST_refreshBackground();
    ST_drawWidgets(true_);
}

pub unsafe fn ST_diffDraw() {
    ST_drawWidgets(false_);
}

pub unsafe fn ST_Drawer(fullscreen: boolean, refresh: boolean) {
    st_statusbaron = (fullscreen == 0 || automapactive != 0) as boolean;
    st_firsttime = (st_firsttime != 0 || refresh != 0) as boolean;

    ST_doPaletteStuff();

    if st_firsttime != 0 {
        ST_doRefresh();
    } else {
        ST_diffDraw();
    }
}

pub unsafe fn ST_loadGraphics() {
    for i in 0..10 {
        let name = format!("STTNUM{}", i);
        tallnum[i] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;

        let name2 = format!("STYSNUM{}", i);
        shortnum[i] = W_CacheLumpNameStr(&name2, PU_STATIC) as *mut patch_t;
    }

    tallpercent = W_CacheLumpNameStr("STTPRCNT", PU_STATIC) as *mut patch_t;

    for i in 0..NUMCARDS {
        let name = format!("STKEYS{}", i);
        keys[i] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
    }

    armsbg = W_CacheLumpNameStr("STARMS", PU_STATIC) as *mut patch_t;

    for i in 0..6 {
        let name = format!("STGNUM{}", i + 2);
        arms[i][0] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
        arms[i][1] = shortnum[i + 2];
    }

    let fb_name = format!("STFB{}", consoleplayer);
    faceback = W_CacheLumpNameStr(&fb_name, PU_STATIC) as *mut patch_t;

    sbar = W_CacheLumpNameStr("STBAR", PU_STATIC) as *mut patch_t;

    let mut facenum = 0;
    for i in 0..ST_NUMPAINFACES {
        for j in 0..ST_NUMSTRAIGHTFACES {
            let name = format!("STFST{}{}", i, j);
            faces[facenum] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
            facenum += 1;
        }
        let name = format!("STFTR{}0", i);
        faces[facenum] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
        facenum += 1;

        let name = format!("STFTL{}0", i);
        faces[facenum] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
        facenum += 1;

        let name = format!("STFOUCH{}", i);
        faces[facenum] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
        facenum += 1;

        let name = format!("STFEVL{}", i);
        faces[facenum] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
        facenum += 1;

        let name = format!("STFKILL{}", i);
        faces[facenum] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
        facenum += 1;
    }
    faces[facenum] = W_CacheLumpNameStr("STFGOD0", PU_STATIC) as *mut patch_t;
    facenum += 1;
    faces[facenum] = W_CacheLumpNameStr("STFDEAD0", PU_STATIC) as *mut patch_t;
}

pub unsafe fn ST_loadData() {
    lu_palette = W_GetNumForNameStr("PLAYPAL");
    ST_loadGraphics();
}

pub unsafe fn ST_unloadGraphics() {
    for i in 0..10 {
        Z_ChangeTag(tallnum[i] as *mut std::ffi::c_void, PU_CACHE);
        Z_ChangeTag(shortnum[i] as *mut std::ffi::c_void, PU_CACHE);
    }
    Z_ChangeTag(tallpercent as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(armsbg as *mut std::ffi::c_void, PU_CACHE);

    for i in 0..6 {
        Z_ChangeTag(arms[i][0] as *mut std::ffi::c_void, PU_CACHE);
    }
    for i in 0..NUMCARDS {
        Z_ChangeTag(keys[i] as *mut std::ffi::c_void, PU_CACHE);
    }

    Z_ChangeTag(sbar as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(faceback as *mut std::ffi::c_void, PU_CACHE);

    for i in 0..ST_NUMFACES {
        Z_ChangeTag(faces[i] as *mut std::ffi::c_void, PU_CACHE);
    }
}

pub unsafe fn ST_unloadData() {
    ST_unloadGraphics();
}

pub unsafe fn ST_initData() {
    st_firsttime = true_;
    plr = &mut players[consoleplayer as usize];

    st_clock = 0;
    st_chatstate = StartChatState;
    st_gamestate = FirstPersonState;

    st_statusbaron = true_;
    st_oldchat = false_;
    st_chat = false_;
    st_cursoron = false_;

    st_faceindex = 0;
    st_palette = -1;

    st_oldhealth = -1;

    for i in 0..NUMWEAPONS {
        oldweaponsowned[i] = (*plr).weaponowned[i];
    }

    for i in 0..3 {
        keyboxes[i] = -1;
    }

    STlib_init();
}

pub unsafe fn ST_createWidgets() {
    let ammo_idx = weaponinfo[(*plr).readyweapon as usize].ammo as usize;
    let ammo_ptr = if ammo_idx < NUMAMMO {
        &mut (*plr).ammo[ammo_idx] as *mut i32
    } else {
        &mut largeammo as *mut i32
    };

    STlib_initNum(
        &mut w_ready,
        ST_AMMOX,
        ST_AMMOY,
        tallnum.as_mut_ptr(),
        ammo_ptr,
        &mut st_statusbaron,
        ST_AMMOWIDTH,
    );
    w_ready.data = (*plr).readyweapon;

    STlib_initPercent(
        &mut w_health,
        ST_HEALTHX,
        ST_HEALTHY,
        tallnum.as_mut_ptr(),
        &mut (*plr).health,
        &mut st_statusbaron,
        tallpercent,
    );

    STlib_initBinIcon(
        &mut w_armsbg,
        ST_ARMSBGX,
        ST_ARMSBGY,
        armsbg,
        &mut st_notdeathmatch,
        &mut st_statusbaron,
    );

    for i in 0..6 {
        STlib_initMultIcon(
            &mut w_arms[i],
            ST_ARMSX + ((i as i32) % 3) * ST_ARMSXSPACE,
            ST_ARMSY + ((i as i32) / 3) * ST_ARMSYSPACE,
            arms[i].as_mut_ptr(),
            &mut (*plr).weaponowned[i + 1] as *mut i32,
            &mut st_armson,
        );
    }

    STlib_initNum(
        &mut w_frags,
        ST_FRAGSX,
        ST_FRAGSY,
        tallnum.as_mut_ptr(),
        &mut st_fragscount,
        &mut st_fragson,
        ST_FRAGSWIDTH,
    );

    STlib_initMultIcon(
        &mut w_faces,
        ST_FACESX,
        ST_FACESY,
        faces.as_mut_ptr(),
        &mut st_faceindex,
        &mut st_statusbaron,
    );

    STlib_initPercent(
        &mut w_armor,
        ST_ARMORX,
        ST_ARMORY,
        tallnum.as_mut_ptr(),
        &mut (*plr).armorpoints,
        &mut st_statusbaron,
        tallpercent,
    );

    STlib_initMultIcon(
        &mut w_keyboxes[0],
        ST_KEY0X,
        ST_KEY0Y,
        keys.as_mut_ptr(),
        &mut keyboxes[0],
        &mut st_statusbaron,
    );
    STlib_initMultIcon(
        &mut w_keyboxes[1],
        ST_KEY1X,
        ST_KEY1Y,
        keys.as_mut_ptr(),
        &mut keyboxes[1],
        &mut st_statusbaron,
    );
    STlib_initMultIcon(
        &mut w_keyboxes[2],
        ST_KEY2X,
        ST_KEY2Y,
        keys.as_mut_ptr(),
        &mut keyboxes[2],
        &mut st_statusbaron,
    );

    STlib_initNum(
        &mut w_ammo[0],
        ST_AMMO0X,
        ST_AMMO0Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).ammo[0],
        &mut st_statusbaron,
        ST_AMMO0WIDTH,
    );
    STlib_initNum(
        &mut w_ammo[1],
        ST_AMMO1X,
        ST_AMMO1Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).ammo[1],
        &mut st_statusbaron,
        ST_AMMO1WIDTH,
    );
    STlib_initNum(
        &mut w_ammo[2],
        ST_AMMO2X,
        ST_AMMO2Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).ammo[2],
        &mut st_statusbaron,
        ST_AMMO2WIDTH,
    );
    STlib_initNum(
        &mut w_ammo[3],
        ST_AMMO3X,
        ST_AMMO3Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).ammo[3],
        &mut st_statusbaron,
        ST_AMMO3WIDTH,
    );

    STlib_initNum(
        &mut w_maxammo[0],
        ST_MAXAMMO0X,
        ST_MAXAMMO0Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).maxammo[0],
        &mut st_statusbaron,
        ST_MAXAMMO0WIDTH,
    );
    STlib_initNum(
        &mut w_maxammo[1],
        ST_MAXAMMO1X,
        ST_MAXAMMO1Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).maxammo[1],
        &mut st_statusbaron,
        ST_MAXAMMO1WIDTH,
    );
    STlib_initNum(
        &mut w_maxammo[2],
        ST_MAXAMMO2X,
        ST_MAXAMMO2Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).maxammo[2],
        &mut st_statusbaron,
        ST_MAXAMMO2WIDTH,
    );
    STlib_initNum(
        &mut w_maxammo[3],
        ST_MAXAMMO3X,
        ST_MAXAMMO3Y,
        shortnum.as_mut_ptr(),
        &mut (*plr).maxammo[3],
        &mut st_statusbaron,
        ST_MAXAMMO3WIDTH,
    );
}

static mut st_stopped: boolean = true_;

pub unsafe fn ST_Start() {
    if st_stopped == 0 {
        ST_Stop();
    }
    ST_initData();
    ST_createWidgets();
    st_stopped = false_;
}

pub unsafe fn ST_Stop() {
    if st_stopped != 0 {
        return;
    }
    I_SetPalette(W_CacheLumpNum(lu_palette, PU_CACHE) as *mut byte);
    st_stopped = true_;
}

pub unsafe fn ST_Init() {
    veryfirsttime = 0;
    ST_loadData();
    screens[4] = Z_Malloc(ST_WIDTH * ST_HEIGHT, PU_STATIC, std::ptr::null_mut()) as *mut byte;
}
