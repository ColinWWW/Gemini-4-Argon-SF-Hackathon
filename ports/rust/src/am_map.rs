// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_event::*;
use crate::d_player::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::m_cheat::*;
use crate::m_fixed::*;
use crate::p_local::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::write_c_str;

pub const AM_MSGHEADER: i32 = (('a' as i32) << 24) + (('m' as i32) << 16);
pub const AM_MSGENTERED: i32 = AM_MSGHEADER | (('e' as i32) << 8);
pub const AM_MSGEXITED: i32 = AM_MSGHEADER | (('x' as i32) << 8);

const REDS: i32 = 256 - 5 * 16;
const REDRANGE: i32 = 16;
const BLUES: i32 = 256 - 4 * 16 + 8;
const BLUERANGE: i32 = 8;
const GREENS: i32 = 7 * 16;
const GREENRANGE: i32 = 16;
const GRAYS: i32 = 6 * 16;
const GRAYSRANGE: i32 = 16;
const BROWNS: i32 = 4 * 16;
const BROWNRANGE: i32 = 16;
const YELLOWS: i32 = 256 - 32 + 7;
const YELLOWRANGE: i32 = 1;
const BLACK: i32 = 0;
const WHITE: i32 = 256 - 47;

const BACKGROUND: i32 = BLACK;
const YOURCOLORS: i32 = WHITE;
const YOURRANGE: i32 = 0;
const WALLCOLORS: i32 = REDS;
const WALLRANGE: i32 = REDRANGE;
const TSWALLCOLORS: i32 = GRAYS;
const TSWALLRANGE: i32 = GRAYSRANGE;
const FDWALLCOLORS: i32 = BROWNS;
const FDWALLRANGE: i32 = BROWNRANGE;
const CDWALLCOLORS: i32 = YELLOWS;
const CDWALLRANGE: i32 = YELLOWRANGE;
const THINGCOLORS: i32 = GREENS;
const THINGRANGE: i32 = GREENRANGE;
const SECRETWALLCOLORS: i32 = WALLCOLORS;
const SECRETWALLRANGE: i32 = WALLRANGE;
const GRIDCOLORS: i32 = GRAYS + GRAYSRANGE / 2;
const GRIDRANGE: i32 = 0;
const XHAIRCOLORS: i32 = GRAYS;

const FB: i32 = 0;

const AM_PANDOWNKEY: i32 = KEY_DOWNARROW;
const AM_PANUPKEY: i32 = KEY_UPARROW;
const AM_PANRIGHTKEY: i32 = KEY_RIGHTARROW;
const AM_PANLEFTKEY: i32 = KEY_LEFTARROW;
const AM_ZOOMINKEY: i32 = b'=' as i32;
const AM_ZOOMOUTKEY: i32 = b'-' as i32;
const AM_STARTKEY: i32 = KEY_TAB;
const AM_ENDKEY: i32 = KEY_TAB;
const AM_GOBIGKEY: i32 = b'0' as i32;
const AM_FOLLOWKEY: i32 = b'f' as i32;
const AM_GRIDKEY: i32 = b'g' as i32;
const AM_MARKKEY: i32 = b'm' as i32;
const AM_CLEARMARKKEY: i32 = b'c' as i32;

const AM_NUMMARKPOINTS: usize = 10;

const INITSCALEMTOF: fixed_t = (0.2 * (FRACUNIT as f64)) as fixed_t;
const F_PANINC: i32 = 4;
const M_ZOOMIN: fixed_t = (1.02 * (FRACUNIT as f64)) as fixed_t;
const M_ZOOMOUT: fixed_t = ((FRACUNIT as f64) / 1.02) as fixed_t;

const LINE_NEVERSEE: i16 = ML_DONTDRAW as i16;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct fpoint_t {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct fline_t {
    pub a: fpoint_t,
    pub b: fpoint_t,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct mpoint_t {
    pub x: fixed_t,
    pub y: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct mline_t {
    pub a: mpoint_t,
    pub b: mpoint_t,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct islope_t {
    pub slp: fixed_t,
    pub islp: fixed_t,
}

const R_PLYR: fixed_t = (8 * PLAYERRADIUS) / 7;
pub static mut player_arrow: [mline_t; 7] = [
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t { x: R_PLYR, y: 0 },
    },
    mline_t {
        a: mpoint_t { x: R_PLYR, y: 0 },
        b: mpoint_t {
            x: R_PLYR - R_PLYR / 2,
            y: R_PLYR / 4,
        },
    },
    mline_t {
        a: mpoint_t { x: R_PLYR, y: 0 },
        b: mpoint_t {
            x: R_PLYR - R_PLYR / 2,
            y: -R_PLYR / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR - R_PLYR / 8,
            y: R_PLYR / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR - R_PLYR / 8,
            y: -R_PLYR / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + 3 * R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: R_PLYR / 4,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + 3 * R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: -R_PLYR / 4,
        },
    },
];
const NUMPLYRLINES: i32 = 7;

pub static mut cheat_player_arrow: [mline_t; 16] = [
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t { x: R_PLYR, y: 0 },
    },
    mline_t {
        a: mpoint_t { x: R_PLYR, y: 0 },
        b: mpoint_t {
            x: R_PLYR - R_PLYR / 2,
            y: R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t { x: R_PLYR, y: 0 },
        b: mpoint_t {
            x: R_PLYR - R_PLYR / 2,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR - R_PLYR / 8,
            y: R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR - R_PLYR / 8,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + 3 * R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR + 3 * R_PLYR / 8,
            y: 0,
        },
        b: mpoint_t {
            x: -R_PLYR + R_PLYR / 8,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t { x: -R_PLYR / 2, y: 0 },
        b: mpoint_t {
            x: -R_PLYR / 2,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR / 2,
            y: -R_PLYR / 6,
        },
        b: mpoint_t {
            x: -R_PLYR / 2 + R_PLYR / 6,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR / 2 + R_PLYR / 6,
            y: -R_PLYR / 6,
        },
        b: mpoint_t {
            x: -R_PLYR / 2 + R_PLYR / 6,
            y: R_PLYR / 4,
        },
    },
    mline_t {
        a: mpoint_t { x: -R_PLYR / 6, y: 0 },
        b: mpoint_t {
            x: -R_PLYR / 6,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: -R_PLYR / 6,
            y: -R_PLYR / 6,
        },
        b: mpoint_t {
            x: 0,
            y: -R_PLYR / 6,
        },
    },
    mline_t {
        a: mpoint_t {
            x: 0,
            y: -R_PLYR / 6,
        },
        b: mpoint_t { x: 0, y: R_PLYR / 4 },
    },
    mline_t {
        a: mpoint_t {
            x: R_PLYR / 6,
            y: R_PLYR / 4,
        },
        b: mpoint_t {
            x: R_PLYR / 6,
            y: -R_PLYR / 7,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_PLYR / 6,
            y: -R_PLYR / 7,
        },
        b: mpoint_t {
            x: R_PLYR / 6 + R_PLYR / 32,
            y: -R_PLYR / 7 - R_PLYR / 32,
        },
    },
    mline_t {
        a: mpoint_t {
            x: R_PLYR / 6 + R_PLYR / 32,
            y: -R_PLYR / 7 - R_PLYR / 32,
        },
        b: mpoint_t {
            x: R_PLYR / 6 + R_PLYR / 10,
            y: -R_PLYR / 7,
        },
    },
];
const NUMCHEATPLYRLINES: i32 = 16;

pub static mut thintriangle_guy: [mline_t; 3] = [
    mline_t {
        a: mpoint_t {
            x: (-0.5 * (FRACUNIT as f64)) as fixed_t,
            y: (-0.7 * (FRACUNIT as f64)) as fixed_t,
        },
        b: mpoint_t { x: FRACUNIT, y: 0 },
    },
    mline_t {
        a: mpoint_t { x: FRACUNIT, y: 0 },
        b: mpoint_t {
            x: (-0.5 * (FRACUNIT as f64)) as fixed_t,
            y: (0.7 * (FRACUNIT as f64)) as fixed_t,
        },
    },
    mline_t {
        a: mpoint_t {
            x: (-0.5 * (FRACUNIT as f64)) as fixed_t,
            y: (0.7 * (FRACUNIT as f64)) as fixed_t,
        },
        b: mpoint_t {
            x: (-0.5 * (FRACUNIT as f64)) as fixed_t,
            y: (-0.7 * (FRACUNIT as f64)) as fixed_t,
        },
    },
];
const NUMTHINTRIANGLEGUYLINES: i32 = 3;

static mut cheating: i32 = 0;
static mut grid: i32 = 0;
static mut leveljuststarted: i32 = 1;
pub static mut automapactive: boolean = false_;
static mut finit_width: i32 = SCREENWIDTH;
static mut finit_height: i32 = SCREENHEIGHT - 32;

static mut f_x: i32 = 0;
static mut f_y: i32 = 0;
static mut f_w: i32 = 0;
static mut f_h: i32 = 0;

static mut lightlev: i32 = 0;
static mut fb: *mut byte = std::ptr::null_mut();
static mut amclock: i32 = 0;

static mut m_paninc: mpoint_t = mpoint_t { x: 0, y: 0 };
static mut mtof_zoommul: fixed_t = 0;
static mut ftom_zoommul: fixed_t = 0;

static mut m_x: fixed_t = 0;
static mut m_y: fixed_t = 0;
static mut m_x2: fixed_t = 0;
static mut m_y2: fixed_t = 0;

static mut m_w: fixed_t = 0;
static mut m_h: fixed_t = 0;

static mut min_x: fixed_t = 0;
static mut min_y: fixed_t = 0;
static mut max_x: fixed_t = 0;
static mut max_y: fixed_t = 0;

static mut max_w: fixed_t = 0;
static mut max_h: fixed_t = 0;

static mut min_w: fixed_t = 0;
static mut min_h: fixed_t = 0;

static mut min_scale_mtof: fixed_t = 0;
static mut max_scale_mtof: fixed_t = 0;

static mut old_m_w: fixed_t = 0;
static mut old_m_h: fixed_t = 0;
static mut old_m_x: fixed_t = 0;
static mut old_m_y: fixed_t = 0;

static mut f_oldloc: mpoint_t = mpoint_t { x: 0, y: 0 };

static mut scale_mtof: fixed_t = INITSCALEMTOF;
static mut scale_ftom: fixed_t = 0;

static mut plr: *mut player_t = std::ptr::null_mut();

static mut marknums: [*mut patch_t; 10] = [std::ptr::null_mut(); 10];
static mut markpoints: [mpoint_t; AM_NUMMARKPOINTS] = [mpoint_t { x: 0, y: 0 }; AM_NUMMARKPOINTS];
static mut markpointnum: i32 = 0;

static mut followplayer: i32 = 1;

static mut cheat_amap_seq: [u8; 5] = [0xb2, 0x26, 0x26, 0x2e, 0xff];
static mut cheat_amap: cheatseq_t = cheatseq_t {
    sequence: std::ptr::null_mut(),
    p: std::ptr::null_mut(),
};

static mut stopped: boolean = true_;

#[inline]
unsafe fn FTOM(x: i32) -> fixed_t {
    FixedMul(x << 16, scale_ftom)
}

#[inline]
unsafe fn MTOF(x: fixed_t) -> i32 {
    FixedMul(x, scale_mtof) >> 16
}

#[inline]
unsafe fn CXMTOF(x: fixed_t) -> i32 {
    f_x + MTOF(x.wrapping_sub(m_x))
}

#[inline]
unsafe fn CYMTOF(y: fixed_t) -> i32 {
    f_y + (f_h - MTOF(y.wrapping_sub(m_y)))
}

pub unsafe fn AM_getIslope(ml: *mut mline_t, is: *mut islope_t) {
    let dy = (*ml).a.y - (*ml).b.y;
    let dx = (*ml).b.x - (*ml).a.x;
    if dy == 0 {
        (*is).islp = if dx < 0 { -MAXINT } else { MAXINT };
    } else {
        (*is).islp = FixedDiv(dx, dy);
    }
    if dx == 0 {
        (*is).slp = if dy < 0 { -MAXINT } else { MAXINT };
    } else {
        (*is).slp = FixedDiv(dy, dx);
    }
}

pub unsafe fn AM_activateNewScale() {
    m_x += m_w / 2;
    m_y += m_h / 2;
    m_w = FTOM(f_w);
    m_h = FTOM(f_h);
    m_x -= m_w / 2;
    m_y -= m_h / 2;
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
}

pub unsafe fn AM_saveScaleAndLoc() {
    old_m_x = m_x;
    old_m_y = m_y;
    old_m_w = m_w;
    old_m_h = m_h;
}

pub unsafe fn AM_restoreScaleAndLoc() {
    m_w = old_m_w;
    m_h = old_m_h;
    if followplayer == 0 {
        m_x = old_m_x;
        m_y = old_m_y;
    } else {
        m_x = (*(*plr).mo).x - m_w / 2;
        m_y = (*(*plr).mo).y - m_h / 2;
    }
    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;

    scale_mtof = FixedDiv(f_w << FRACBITS, m_w);
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
}

pub unsafe fn AM_addMark() {
    markpoints[markpointnum as usize].x = m_x + m_w / 2;
    markpoints[markpointnum as usize].y = m_y + m_h / 2;
    markpointnum = (markpointnum + 1) % (AM_NUMMARKPOINTS as i32);
}

pub unsafe fn AM_findMinMaxBoundaries() {
    min_x = MAXINT;
    min_y = MAXINT;
    max_x = -MAXINT;
    max_y = -MAXINT;

    for i in 0..(numvertexes as usize) {
        let vx = (*vertexes.add(i)).x;
        let vy = (*vertexes.add(i)).y;
        if vx < min_x {
            min_x = vx;
        } else if vx > max_x {
            max_x = vx;
        }
        if vy < min_y {
            min_y = vy;
        } else if vy > max_y {
            max_y = vy;
        }
    }

    max_w = max_x - min_x;
    max_h = max_y - min_y;

    min_w = 2 * PLAYERRADIUS;
    min_h = 2 * PLAYERRADIUS;

    let a = FixedDiv(f_w << FRACBITS, max_w);
    let b = FixedDiv(f_h << FRACBITS, max_h);

    min_scale_mtof = if a < b { a } else { b };
    max_scale_mtof = FixedDiv(f_h << FRACBITS, 2 * PLAYERRADIUS);
}

pub unsafe fn AM_changeWindowLoc() {
    if m_paninc.x != 0 || m_paninc.y != 0 {
        followplayer = 0;
        f_oldloc.x = MAXINT;
    }

    m_x += m_paninc.x;
    m_y += m_paninc.y;

    if m_x + m_w / 2 > max_x {
        m_x = max_x - m_w / 2;
    } else if m_x + m_w / 2 < min_x {
        m_x = min_x - m_w / 2;
    }

    if m_y + m_h / 2 > max_y {
        m_y = max_y - m_h / 2;
    } else if m_y + m_h / 2 < min_y {
        m_y = min_y - m_h / 2;
    }

    m_x2 = m_x + m_w;
    m_y2 = m_y + m_h;
}

pub unsafe fn AM_initVariables() {
    let mut st_notify = event_t {
        type_: ev_keyup,
        data1: AM_MSGENTERED,
        data2: 0,
        data3: 0,
    };

    automapactive = true_;
    fb = screens[0];

    f_oldloc.x = MAXINT;
    amclock = 0;
    lightlev = 0;

    m_paninc.x = 0;
    m_paninc.y = 0;
    ftom_zoommul = FRACUNIT;
    mtof_zoommul = FRACUNIT;

    m_w = FTOM(f_w);
    m_h = FTOM(f_h);

    let mut pnum = consoleplayer as usize;
    if playeringame[pnum] == 0 {
        pnum = 0;
        while pnum < MAXPLAYERS {
            if playeringame[pnum] != 0 {
                break;
            }
            pnum += 1;
        }
    }

    plr = &mut players[pnum];
    m_x = (*(*plr).mo).x - m_w / 2;
    m_y = (*(*plr).mo).y - m_h / 2;
    AM_changeWindowLoc();

    old_m_x = m_x;
    old_m_y = m_y;
    old_m_w = m_w;
    old_m_h = m_h;

    ST_Responder(&mut st_notify);
}

pub unsafe fn AM_loadPics() {
    for i in 0..10 {
        let name = format!("AMMNUM{}", i);
        marknums[i] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
    }
}

pub unsafe fn AM_unloadPics() {
    for i in 0..10 {
        Z_ChangeTag(marknums[i] as *mut std::ffi::c_void, PU_CACHE);
    }
}

pub unsafe fn AM_clearMarks() {
    for i in 0..AM_NUMMARKPOINTS {
        markpoints[i].x = -1;
    }
    markpointnum = 0;
}

pub unsafe fn AM_LevelInit() {
    leveljuststarted = 0;

    f_x = 0;
    f_y = 0;
    f_w = finit_width;
    f_h = finit_height;

    AM_clearMarks();

    AM_findMinMaxBoundaries();
    scale_mtof = FixedDiv(min_scale_mtof, (0.7 * (FRACUNIT as f64)) as fixed_t);
    if scale_mtof > max_scale_mtof {
        scale_mtof = min_scale_mtof;
    }
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
}

pub unsafe fn AM_Stop() {
    let mut st_notify = event_t {
        type_: 0,
        data1: ev_keyup,
        data2: AM_MSGEXITED,
        data3: 0,
    };

    AM_unloadPics();
    automapactive = false_;
    ST_Responder(&mut st_notify);
    stopped = true_;
}

static mut lastlevel: i32 = -1;
static mut lastepisode: i32 = -1;

pub unsafe fn AM_Start() {
    if stopped == 0 {
        AM_Stop();
    }
    stopped = false_;
    if lastlevel != gamemap || lastepisode != gameepisode {
        AM_LevelInit();
        lastlevel = gamemap;
        lastepisode = gameepisode;
    }
    AM_initVariables();
    AM_loadPics();
}

pub unsafe fn AM_minOutWindowScale() {
    scale_mtof = min_scale_mtof;
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    AM_activateNewScale();
}

pub unsafe fn AM_maxOutWindowScale() {
    scale_mtof = max_scale_mtof;
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
    AM_activateNewScale();
}

static mut cheatstate: i32 = 0;
static mut bigstate: i32 = 0;
static mut am_buffer: [u8; 20] = [0; 20];

pub unsafe fn AM_Responder(ev: *mut event_t) -> boolean {
    if cheat_amap.sequence.is_null() {
        cheat_amap.sequence = cheat_amap_seq.as_mut_ptr();
    }

    let mut rc = false_;

    if automapactive == 0 {
        if (*ev).type_ == ev_keydown && (*ev).data1 == AM_STARTKEY {
            AM_Start();
            viewactive = false_;
            rc = true_;
        }
    } else if (*ev).type_ == ev_keydown {
        rc = true_;
        match (*ev).data1 {
            AM_PANRIGHTKEY => {
                if followplayer == 0 {
                    m_paninc.x = FTOM(F_PANINC);
                } else {
                    rc = false_;
                }
            }
            AM_PANLEFTKEY => {
                if followplayer == 0 {
                    m_paninc.x = -FTOM(F_PANINC);
                } else {
                    rc = false_;
                }
            }
            AM_PANUPKEY => {
                if followplayer == 0 {
                    m_paninc.y = FTOM(F_PANINC);
                } else {
                    rc = false_;
                }
            }
            AM_PANDOWNKEY => {
                if followplayer == 0 {
                    m_paninc.y = -FTOM(F_PANINC);
                } else {
                    rc = false_;
                }
            }
            AM_ZOOMOUTKEY => {
                mtof_zoommul = M_ZOOMOUT;
                ftom_zoommul = M_ZOOMIN;
            }
            AM_ZOOMINKEY => {
                mtof_zoommul = M_ZOOMIN;
                ftom_zoommul = M_ZOOMOUT;
            }
            AM_ENDKEY => {
                bigstate = 0;
                viewactive = true_;
                AM_Stop();
            }
            AM_GOBIGKEY => {
                bigstate = (bigstate == 0) as i32;
                if bigstate != 0 {
                    AM_saveScaleAndLoc();
                    AM_minOutWindowScale();
                } else {
                    AM_restoreScaleAndLoc();
                }
            }
            AM_FOLLOWKEY => {
                followplayer = (followplayer == 0) as i32;
                f_oldloc.x = MAXINT;
                (*plr).message = if followplayer != 0 {
                    b"Follow Mode ON\0".as_ptr()
                } else {
                    b"Follow Mode OFF\0".as_ptr()
                };
            }
            AM_GRIDKEY => {
                grid = (grid == 0) as i32;
                (*plr).message = if grid != 0 {
                    b"Grid ON\0".as_ptr()
                } else {
                    b"Grid OFF\0".as_ptr()
                };
            }
            AM_MARKKEY => {
                let s = format!("Marked Spot {}", markpointnum);
                write_c_str(am_buffer.as_mut_ptr(), &s);
                (*plr).message = am_buffer.as_ptr();
                AM_addMark();
            }
            AM_CLEARMARKKEY => {
                AM_clearMarks();
                (*plr).message = b"All Marks Cleared\0".as_ptr();
            }
            _ => {
                cheatstate = 0;
                rc = false_;
            }
        }
        if deathmatch == 0 && cht_CheckCheat(&mut cheat_amap, (*ev).data1 as u8) != 0 {
            rc = false_;
            cheating = (cheating + 1) % 3;
        }
    } else if (*ev).type_ == ev_keyup {
        rc = false_;
        match (*ev).data1 {
            AM_PANRIGHTKEY => {
                if followplayer == 0 {
                    m_paninc.x = 0;
                }
            }
            AM_PANLEFTKEY => {
                if followplayer == 0 {
                    m_paninc.x = 0;
                }
            }
            AM_PANUPKEY => {
                if followplayer == 0 {
                    m_paninc.y = 0;
                }
            }
            AM_PANDOWNKEY => {
                if followplayer == 0 {
                    m_paninc.y = 0;
                }
            }
            AM_ZOOMOUTKEY | AM_ZOOMINKEY => {
                mtof_zoommul = FRACUNIT;
                ftom_zoommul = FRACUNIT;
            }
            _ => {}
        }
    }

    rc
}

pub unsafe fn AM_changeWindowScale() {
    scale_mtof = FixedMul(scale_mtof, mtof_zoommul);
    scale_ftom = FixedDiv(FRACUNIT, scale_mtof);

    if scale_mtof < min_scale_mtof {
        AM_minOutWindowScale();
    } else if scale_mtof > max_scale_mtof {
        AM_maxOutWindowScale();
    } else {
        AM_activateNewScale();
    }
}

pub unsafe fn AM_doFollowPlayer() {
    if f_oldloc.x != (*(*plr).mo).x || f_oldloc.y != (*(*plr).mo).y {
        m_x = FTOM(MTOF((*(*plr).mo).x)) - m_w / 2;
        m_y = FTOM(MTOF((*(*plr).mo).y)) - m_h / 2;
        m_x2 = m_x + m_w;
        m_y2 = m_y + m_h;
        f_oldloc.x = (*(*plr).mo).x;
        f_oldloc.y = (*(*plr).mo).y;
    }
}

pub unsafe fn AM_Ticker() {
    if automapactive == 0 {
        return;
    }

    amclock += 1;

    if followplayer != 0 {
        AM_doFollowPlayer();
    }

    if ftom_zoommul != FRACUNIT {
        AM_changeWindowScale();
    }

    if m_paninc.x != 0 || m_paninc.y != 0 {
        AM_changeWindowLoc();
    }
}

pub unsafe fn AM_clearFB(color: i32) {
    std::ptr::write_bytes(fb, color as u8, (f_w * f_h) as usize);
}

pub unsafe fn AM_clipMline(ml: *mut mline_t, fl: *mut fline_t) -> boolean {
    const LEFT: i32 = 1;
    const RIGHT: i32 = 2;
    const BOTTOM: i32 = 4;
    const TOP: i32 = 8;

    let mut outcode1 = 0;
    let mut outcode2 = 0;

    if (*ml).a.y > m_y2 {
        outcode1 = TOP;
    } else if (*ml).a.y < m_y {
        outcode1 = BOTTOM;
    }

    if (*ml).b.y > m_y2 {
        outcode2 = TOP;
    } else if (*ml).b.y < m_y {
        outcode2 = BOTTOM;
    }

    if outcode1 & outcode2 != 0 {
        return false_;
    }

    if (*ml).a.x < m_x {
        outcode1 |= LEFT;
    } else if (*ml).a.x > m_x2 {
        outcode1 |= RIGHT;
    }

    if (*ml).b.x < m_x {
        outcode2 |= LEFT;
    } else if (*ml).b.x > m_x2 {
        outcode2 |= RIGHT;
    }

    if outcode1 & outcode2 != 0 {
        return false_;
    }

    (*fl).a.x = CXMTOF((*ml).a.x);
    (*fl).a.y = CYMTOF((*ml).a.y);
    (*fl).b.x = CXMTOF((*ml).b.x);
    (*fl).b.y = CYMTOF((*ml).b.y);

    let dooutcode = |mx: i32, my: i32| -> i32 {
        let mut oc = 0;
        if my < 0 {
            oc |= TOP;
        } else if my >= f_h {
            oc |= BOTTOM;
        }
        if mx < 0 {
            oc |= LEFT;
        } else if mx >= f_w {
            oc |= RIGHT;
        }
        oc
    };

    outcode1 = dooutcode((*fl).a.x, (*fl).a.y);
    outcode2 = dooutcode((*fl).b.x, (*fl).b.y);

    if outcode1 & outcode2 != 0 {
        return false_;
    }

    while (outcode1 | outcode2) != 0 {
        let outside = if outcode1 != 0 { outcode1 } else { outcode2 };
        let mut tmp = fpoint_t { x: 0, y: 0 };

        if outside & TOP != 0 {
            let dy = (*fl).a.y - (*fl).b.y;
            let dx = (*fl).b.x - (*fl).a.x;
            tmp.x = (*fl).a.x + (dx * (*fl).a.y) / dy;
            tmp.y = 0;
        } else if outside & BOTTOM != 0 {
            let dy = (*fl).a.y - (*fl).b.y;
            let dx = (*fl).b.x - (*fl).a.x;
            tmp.x = (*fl).a.x + (dx * ((*fl).a.y - f_h)) / dy;
            tmp.y = f_h - 1;
        } else if outside & RIGHT != 0 {
            let dy = (*fl).b.y - (*fl).a.y;
            let dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + (dy * (f_w - 1 - (*fl).a.x)) / dx;
            tmp.x = f_w - 1;
        } else if outside & LEFT != 0 {
            let dy = (*fl).b.y - (*fl).a.y;
            let dx = (*fl).b.x - (*fl).a.x;
            tmp.y = (*fl).a.y + (dy * (-(*fl).a.x)) / dx;
            tmp.x = 0;
        }

        if outside == outcode1 {
            (*fl).a = tmp;
            outcode1 = dooutcode((*fl).a.x, (*fl).a.y);
        } else {
            (*fl).b = tmp;
            outcode2 = dooutcode((*fl).b.x, (*fl).b.y);
        }

        if outcode1 & outcode2 != 0 {
            return false_;
        }
    }

    true_
}

static mut fuck: i32 = 0;

pub unsafe fn AM_drawFline(fl: *mut fline_t, color: i32) {
    if (*fl).a.x < 0
        || (*fl).a.x >= f_w
        || (*fl).a.y < 0
        || (*fl).a.y >= f_h
        || (*fl).b.x < 0
        || (*fl).b.x >= f_w
        || (*fl).b.y < 0
        || (*fl).b.y >= f_h
    {
        fuck += 1;
        return;
    }

    let dx = (*fl).b.x - (*fl).a.x;
    let ax = 2 * (if dx < 0 { -dx } else { dx });
    let sx = if dx < 0 { -1 } else { 1 };

    let dy = (*fl).b.y - (*fl).a.y;
    let ay = 2 * (if dy < 0 { -dy } else { dy });
    let sy = if dy < 0 { -1 } else { 1 };

    let mut x = (*fl).a.x;
    let mut y = (*fl).a.y;

    if ax > ay {
        let mut d = ay - ax / 2;
        loop {
            *fb.add((y * f_w + x) as usize) = color as byte;
            if x == (*fl).b.x {
                return;
            }
            if d >= 0 {
                y += sy;
                d -= ax;
            }
            x += sx;
            d += ay;
        }
    } else {
        let mut d = ax - ay / 2;
        loop {
            *fb.add((y * f_w + x) as usize) = color as byte;
            if y == (*fl).b.y {
                return;
            }
            if d >= 0 {
                x += sx;
                d -= ay;
            }
            y += sy;
            d += ax;
        }
    }
}

pub unsafe fn AM_drawMline(ml: *mut mline_t, color: i32) {
    let mut fl = fline_t::default();
    if AM_clipMline(ml, &mut fl) != 0 {
        AM_drawFline(&mut fl, color);
    }
}

pub unsafe fn AM_drawGrid(color: i32) {
    let mut ml = mline_t::default();
    let step = MAPBLOCKUNITS << FRACBITS;

    let mut start = m_x;
    if (start - bmaporgx) % step != 0 {
        start += step - ((start - bmaporgx) % step);
    }
    let end = m_x + m_w;

    ml.a.y = m_y;
    ml.b.y = m_y + m_h;
    let mut x = start;
    while x < end {
        ml.a.x = x;
        ml.b.x = x;
        AM_drawMline(&mut ml, color);
        x += step;
    }

    start = m_y;
    if (start - bmaporgy) % step != 0 {
        start += step - ((start - bmaporgy) % step);
    }
    let end_y = m_y + m_h;

    ml.a.x = m_x;
    ml.b.x = m_x + m_w;
    let mut y = start;
    while y < end_y {
        ml.a.y = y;
        ml.b.y = y;
        AM_drawMline(&mut ml, color);
        y += step;
    }
}

pub unsafe fn AM_drawWalls() {
    let mut l = mline_t::default();
    for i in 0..(numlines as usize) {
        let line = lines.add(i);
        l.a.x = (*(*line).v1).x;
        l.a.y = (*(*line).v1).y;
        l.b.x = (*(*line).v2).x;
        l.b.y = (*(*line).v2).y;

        if cheating != 0 || ((*line).flags & (ML_MAPPED as i16) != 0) {
            if ((*line).flags & LINE_NEVERSEE != 0) && cheating == 0 {
                continue;
            }
            if (*line).backsector.is_null() {
                AM_drawMline(&mut l, WALLCOLORS + lightlev);
            } else if (*line).special == 39 {
                AM_drawMline(&mut l, WALLCOLORS + WALLRANGE / 2);
            } else if (*line).flags & (ML_SECRET as i16) != 0 {
                if cheating != 0 {
                    AM_drawMline(&mut l, SECRETWALLCOLORS + lightlev);
                } else {
                    AM_drawMline(&mut l, WALLCOLORS + lightlev);
                }
            } else if (*(*line).backsector).floorheight != (*(*line).frontsector).floorheight {
                AM_drawMline(&mut l, FDWALLCOLORS + lightlev);
            } else if (*(*line).backsector).ceilingheight != (*(*line).frontsector).ceilingheight {
                AM_drawMline(&mut l, CDWALLCOLORS + lightlev);
            } else if cheating != 0 {
                AM_drawMline(&mut l, TSWALLCOLORS + lightlev);
            }
        } else if (*plr).powers[pw_allmap as usize] != 0 {
            if (*line).flags & LINE_NEVERSEE == 0 {
                AM_drawMline(&mut l, GRAYS + 3);
            }
        }
    }
}

pub unsafe fn AM_rotate(x: *mut fixed_t, y: *mut fixed_t, a: angle_t) {
    let idx = (a >> ANGLETOFINESHIFT) as usize;
    let tmpx = FixedMul(*x, finecosine(idx)) - FixedMul(*y, finesine[idx]);
    *y = FixedMul(*x, finesine[idx]) + FixedMul(*y, finecosine(idx));
    *x = tmpx;
}

pub unsafe fn AM_drawLineCharacter(
    lineguy: *mut mline_t,
    lineguylines: i32,
    scale: fixed_t,
    angle: angle_t,
    color: i32,
    x: fixed_t,
    y: fixed_t,
) {
    let mut l = mline_t::default();
    for i in 0..(lineguylines as usize) {
        let lg = lineguy.add(i);
        l.a.x = (*lg).a.x;
        l.a.y = (*lg).a.y;

        if scale != 0 {
            l.a.x = FixedMul(scale, l.a.x);
            l.a.y = FixedMul(scale, l.a.y);
        }
        if angle != 0 {
            AM_rotate(&mut l.a.x, &mut l.a.y, angle);
        }
        l.a.x += x;
        l.a.y += y;

        l.b.x = (*lg).b.x;
        l.b.y = (*lg).b.y;

        if scale != 0 {
            l.b.x = FixedMul(scale, l.b.x);
            l.b.y = FixedMul(scale, l.b.y);
        }
        if angle != 0 {
            AM_rotate(&mut l.b.x, &mut l.b.y, angle);
        }
        l.b.x += x;
        l.b.y += y;

        AM_drawMline(&mut l, color);
    }
}

pub unsafe fn AM_drawPlayers() {
    let their_colors = [GREENS, GRAYS, BROWNS, REDS];
    let mut their_color = -1i32;

    if netgame == 0 {
        if cheating != 0 {
            AM_drawLineCharacter(
                cheat_player_arrow.as_mut_ptr(),
                NUMCHEATPLYRLINES,
                0,
                (*(*plr).mo).angle,
                WHITE,
                (*(*plr).mo).x,
                (*(*plr).mo).y,
            );
        } else {
            AM_drawLineCharacter(
                player_arrow.as_mut_ptr(),
                NUMPLYRLINES,
                0,
                (*(*plr).mo).angle,
                WHITE,
                (*(*plr).mo).x,
                (*(*plr).mo).y,
            );
        }
        return;
    }

    for i in 0..MAXPLAYERS {
        their_color += 1;
        let p = &mut players[i] as *mut player_t;

        if (deathmatch != 0 && singledemo == 0) && p != plr {
            continue;
        }
        if playeringame[i] == 0 {
            continue;
        }

        let color = if (*p).powers[pw_invisibility as usize] != 0 {
            246
        } else {
            their_colors[their_color as usize]
        };

        AM_drawLineCharacter(
            player_arrow.as_mut_ptr(),
            NUMPLYRLINES,
            0,
            (*(*p).mo).angle,
            color,
            (*(*p).mo).x,
            (*(*p).mo).y,
        );
    }
}

pub unsafe fn AM_drawThings(colors: i32, _colorrange: i32) {
    for i in 0..(numsectors as usize) {
        let mut t = (*sectors.add(i)).thinglist;
        while !t.is_null() {
            AM_drawLineCharacter(
                thintriangle_guy.as_mut_ptr(),
                NUMTHINTRIANGLEGUYLINES,
                16 << FRACBITS,
                (*t).angle,
                colors + lightlev,
                (*t).x,
                (*t).y,
            );
            t = (*t).snext;
        }
    }
}

pub unsafe fn AM_drawMarks() {
    for i in 0..AM_NUMMARKPOINTS {
        if markpoints[i].x != -1 {
            let w = 5;
            let h = 6;
            let fx = CXMTOF(markpoints[i].x);
            let fy = CYMTOF(markpoints[i].y);
            if fx >= f_x && fx <= f_w - w && fy >= f_y && fy <= f_h - h {
                V_DrawPatch(fx, fy, FB, marknums[i]);
            }
        }
    }
}

pub unsafe fn AM_drawCrosshair(color: i32) {
    *fb.add(((f_w * (f_h + 1)) / 2) as usize) = color as byte;
}

pub unsafe fn AM_Drawer() {
    if automapactive == 0 {
        return;
    }

    AM_clearFB(BACKGROUND);
    if grid != 0 {
        AM_drawGrid(GRIDCOLORS);
    }
    AM_drawWalls();
    AM_drawPlayers();
    if cheating == 2 {
        AM_drawThings(THINGCOLORS, THINGRANGE);
    }
    AM_drawCrosshair(XHAIRCOLORS);
    AM_drawMarks();

    V_MarkRect(f_x, f_y, f_w, f_h);
}
