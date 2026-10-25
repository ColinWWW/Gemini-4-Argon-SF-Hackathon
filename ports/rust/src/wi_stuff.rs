// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_event::*;
use crate::d_player::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::m_random::*;
use crate::m_swap::*;
use crate::r_defs::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::st_stuff::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub type stateenum_t = i32;
pub const NoState: stateenum_t = -1;
pub const StatCount: stateenum_t = 0;
pub const ShowNextLoc: stateenum_t = 1;

const NUMEPISODES: usize = 4;
const NUMMAPS: usize = 9;

const WI_TITLEY: i32 = 2;
const WI_SPACINGY: i32 = 33;

const SP_STATSX: i32 = 50;
const SP_STATSY: i32 = 50;
const SP_TIMEX: i32 = 16;
const SP_TIMEY: i32 = SCREENHEIGHT - 32;

const NG_STATSY: i32 = 50;
const NG_SPACINGX: i32 = 64;

const DM_MATRIXX: i32 = 42;
const DM_MATRIXY: i32 = 68;
const DM_SPACINGX: i32 = 40;
const DM_TOTALSX: i32 = 269;
const DM_KILLERSX: i32 = 10;
const DM_KILLERSY: i32 = 100;
const DM_VICTIMSX: i32 = 5;
const DM_VICTIMSY: i32 = 50;

pub type animenum_t = i32;
pub const ANIM_ALWAYS: animenum_t = 0;
pub const ANIM_RANDOM: animenum_t = 1;
pub const ANIM_LEVEL: animenum_t = 2;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct point_t {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anim_t {
    pub type_: animenum_t,
    pub period: i32,
    pub nanims: i32,
    pub loc: point_t,
    pub data1: i32,
    pub data2: i32,
    pub p: [*mut patch_t; 3],
    pub nexttic: i32,
    pub lastdrawn: i32,
    pub ctr: i32,
    pub state: i32,
}

const fn mk_anim(type_: animenum_t, period: i32, nanims: i32, x: i32, y: i32, data1: i32) -> anim_t {
    anim_t {
        type_,
        period,
        nanims,
        loc: point_t { x, y },
        data1,
        data2: 0,
        p: [std::ptr::null_mut(); 3],
        nexttic: 0,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    }
}

static mut lnodes: [[point_t; NUMMAPS]; NUMEPISODES] = [
    [
        point_t { x: 185, y: 164 },
        point_t { x: 148, y: 143 },
        point_t { x: 69, y: 122 },
        point_t { x: 209, y: 102 },
        point_t { x: 116, y: 89 },
        point_t { x: 166, y: 55 },
        point_t { x: 71, y: 56 },
        point_t { x: 135, y: 29 },
        point_t { x: 71, y: 24 },
    ],
    [
        point_t { x: 254, y: 25 },
        point_t { x: 97, y: 50 },
        point_t { x: 188, y: 64 },
        point_t { x: 128, y: 78 },
        point_t { x: 214, y: 92 },
        point_t { x: 133, y: 130 },
        point_t { x: 208, y: 136 },
        point_t { x: 148, y: 140 },
        point_t { x: 235, y: 158 },
    ],
    [
        point_t { x: 156, y: 168 },
        point_t { x: 48, y: 154 },
        point_t { x: 174, y: 95 },
        point_t { x: 265, y: 75 },
        point_t { x: 130, y: 48 },
        point_t { x: 279, y: 23 },
        point_t { x: 198, y: 48 },
        point_t { x: 140, y: 25 },
        point_t { x: 281, y: 136 },
    ],
    [point_t { x: 0, y: 0 }; NUMMAPS],
];

static mut epsd0animinfo: [anim_t; 10] = [
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 224, 104, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 184, 160, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 112, 136, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 72, 112, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 88, 96, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 64, 48, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 192, 40, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 136, 16, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 80, 16, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 64, 24, 0),
];

static mut epsd1animinfo: [anim_t; 9] = [
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 1),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 2),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 3),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 4),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 5),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 6),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 7),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 3, 192, 144, 8),
    mk_anim(ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 8),
];

static mut epsd2animinfo: [anim_t; 6] = [
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 104, 168, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 40, 136, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 160, 96, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 104, 80, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 3, 3, 120, 32, 0),
    mk_anim(ANIM_ALWAYS, TICRATE / 4, 3, 40, 0, 0),
];

static mut NUMANIMS: [usize; NUMEPISODES] = [10, 9, 6, 0];

unsafe fn get_anim(epsd: usize, idx: usize) -> *mut anim_t {
    match epsd {
        0 => &mut epsd0animinfo[idx],
        1 => &mut epsd1animinfo[idx],
        2 => &mut epsd2animinfo[idx],
        _ => std::ptr::null_mut(),
    }
}

const FB: i32 = 0;
const SHOWNEXTLOCDELAY: i32 = 4;

static mut acceleratestage: i32 = 0;
static mut me: i32 = 0;
static mut state: stateenum_t = 0;
static mut wbs: *mut wbstartstruct_t = std::ptr::null_mut();
static mut plrs: *mut wbplayerstruct_t = std::ptr::null_mut();
static mut cnt: i32 = 0;
static mut bcnt: i32 = 0;
static mut firstrefresh: i32 = 0;

static mut cnt_kills: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_items: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_secret: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_time: i32 = 0;
static mut cnt_par: i32 = 0;
static mut cnt_pause: i32 = 0;

static mut NUMCMAPS: i32 = 0;

static mut bg: *mut patch_t = std::ptr::null_mut();
static mut yah: [*mut patch_t; 2] = [std::ptr::null_mut(); 2];
static mut splat: *mut patch_t = std::ptr::null_mut();
static mut percent: *mut patch_t = std::ptr::null_mut();
static mut colon: *mut patch_t = std::ptr::null_mut();
static mut num: [*mut patch_t; 10] = [std::ptr::null_mut(); 10];
static mut wiminus: *mut patch_t = std::ptr::null_mut();
static mut finished: *mut patch_t = std::ptr::null_mut();
static mut entering: *mut patch_t = std::ptr::null_mut();
static mut sp_secret: *mut patch_t = std::ptr::null_mut();
static mut kills: *mut patch_t = std::ptr::null_mut();
static mut secret: *mut patch_t = std::ptr::null_mut();
static mut items: *mut patch_t = std::ptr::null_mut();
static mut frags: *mut patch_t = std::ptr::null_mut();
static mut time_patch: *mut patch_t = std::ptr::null_mut();
static mut par: *mut patch_t = std::ptr::null_mut();
static mut sucks: *mut patch_t = std::ptr::null_mut();
static mut killers: *mut patch_t = std::ptr::null_mut();
static mut victims: *mut patch_t = std::ptr::null_mut();
static mut total: *mut patch_t = std::ptr::null_mut();
static mut star: *mut patch_t = std::ptr::null_mut();
static mut bstar: *mut patch_t = std::ptr::null_mut();
static mut p: [*mut patch_t; MAXPLAYERS] = [std::ptr::null_mut(); MAXPLAYERS];
static mut bp: [*mut patch_t; MAXPLAYERS] = [std::ptr::null_mut(); MAXPLAYERS];
static mut lnames: *mut *mut patch_t = std::ptr::null_mut();

pub unsafe fn WI_slamBackground() {
    std::ptr::copy_nonoverlapping(screens[1], screens[0], (SCREENWIDTH * SCREENHEIGHT) as usize);
    V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);
}

pub unsafe fn WI_Responder(_ev: *mut event_t) -> boolean {
    false_
}

pub unsafe fn WI_drawLF() {
    let mut y = WI_TITLEY;
    let lname = *lnames.add((*wbs).last as usize);
    V_DrawPatch((SCREENWIDTH - (SHORT((*lname).width) as i32)) / 2, y, FB, lname);

    y += (5 * (SHORT((*lname).height) as i32)) / 4;
    V_DrawPatch(
        (SCREENWIDTH - (SHORT((*finished).width) as i32)) / 2,
        y,
        FB,
        finished,
    );
}

pub unsafe fn WI_drawEL() {
    let mut y = WI_TITLEY;
    V_DrawPatch(
        (SCREENWIDTH - (SHORT((*entering).width) as i32)) / 2,
        y,
        FB,
        entering,
    );

    let lname = *lnames.add((*wbs).next as usize);
    y += (5 * (SHORT((*lname).height) as i32)) / 4;
    V_DrawPatch((SCREENWIDTH - (SHORT((*lname).width) as i32)) / 2, y, FB, lname);
}

pub unsafe fn WI_drawOnLnode(n: i32, c: *mut *mut patch_t) {
    let mut i = 0usize;
    let mut fits = false;

    loop {
        let patch = *c.add(i);
        let left =
            lnodes[(*wbs).epsd as usize][n as usize].x - (SHORT((*patch).leftoffset) as i32);
        let top = lnodes[(*wbs).epsd as usize][n as usize].y - (SHORT((*patch).topoffset) as i32);
        let right = left + (SHORT((*patch).width) as i32);
        let bottom = top + (SHORT((*patch).height) as i32);

        if left >= 0 && right < SCREENWIDTH && top >= 0 && bottom < SCREENHEIGHT {
            fits = true;
        } else {
            i += 1;
        }
        if fits || i == 2 {
            break;
        }
    }

    if fits && i < 2 {
        V_DrawPatch(
            lnodes[(*wbs).epsd as usize][n as usize].x,
            lnodes[(*wbs).epsd as usize][n as usize].y,
            FB,
            *c.add(i),
        );
    } else {
        println!("Could not place patch on level {}", n + 1);
    }
}

pub unsafe fn WI_initAnimatedBack() {
    if gamemode == commercial {
        return;
    }
    if (*wbs).epsd > 2 {
        return;
    }

    let epsd = (*wbs).epsd as usize;
    for i in 0..NUMANIMS[epsd] {
        let a = get_anim(epsd, i);
        (*a).ctr = -1;
        if (*a).type_ == ANIM_ALWAYS {
            (*a).nexttic = bcnt + 1 + (M_Random() % (*a).period);
        } else if (*a).type_ == ANIM_RANDOM {
            (*a).nexttic = bcnt + 1 + (*a).data2 + (M_Random() % (*a).data1);
        } else if (*a).type_ == ANIM_LEVEL {
            (*a).nexttic = bcnt + 1;
        }
    }
}

pub unsafe fn WI_updateAnimatedBack() {
    if gamemode == commercial {
        return;
    }
    if (*wbs).epsd > 2 {
        return;
    }

    let epsd = (*wbs).epsd as usize;
    for i in 0..NUMANIMS[epsd] {
        let a = get_anim(epsd, i);
        if bcnt == (*a).nexttic {
            match (*a).type_ {
                ANIM_ALWAYS => {
                    (*a).ctr += 1;
                    if (*a).ctr >= (*a).nanims {
                        (*a).ctr = 0;
                    }
                    (*a).nexttic = bcnt + (*a).period;
                }
                ANIM_RANDOM => {
                    (*a).ctr += 1;
                    if (*a).ctr == (*a).nanims {
                        (*a).ctr = -1;
                        (*a).nexttic = bcnt + (*a).data2 + (M_Random() % (*a).data1);
                    } else {
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                ANIM_LEVEL => {
                    if !(state == StatCount && i == 7) && (*wbs).next == (*a).data1 {
                        (*a).ctr += 1;
                        if (*a).ctr == (*a).nanims {
                            (*a).ctr -= 1;
                        }
                        (*a).nexttic = bcnt + (*a).period;
                    }
                }
                _ => {}
            }
        }
    }
}

pub unsafe fn WI_drawAnimatedBack() {
    // Note: in original C: `if (commercial) return;` where `commercial` is enum constant 2!
    if commercial != 0 {
        return;
    }
    if (*wbs).epsd > 2 {
        return;
    }

    let epsd = (*wbs).epsd as usize;
    for i in 0..NUMANIMS[epsd] {
        let a = get_anim(epsd, i);
        if (*a).ctr >= 0 {
            V_DrawPatch((*a).loc.x, (*a).loc.y, FB, (*a).p[(*a).ctr as usize]);
        }
    }
}

pub unsafe fn WI_drawNum(mut x: i32, y: i32, mut n: i32, mut digits: i32) -> i32 {
    let fontwidth = SHORT((*num[0]).width) as i32;

    if digits < 0 {
        if n == 0 {
            digits = 1;
        } else {
            digits = 0;
            let mut temp = n;
            while temp != 0 {
                temp /= 10;
                digits += 1;
            }
        }
    }

    let neg = n < 0;
    if neg {
        n = -n;
    }

    if n == 1994 {
        return 0;
    }

    while digits > 0 {
        digits -= 1;
        x -= fontwidth;
        V_DrawPatch(x, y, FB, num[(n % 10) as usize]);
        n /= 10;
    }

    if neg {
        x -= 8;
        V_DrawPatch(x, y, FB, wiminus);
    }

    x
}

pub unsafe fn WI_drawPercent(x: i32, y: i32, p_val: i32) {
    if p_val < 0 {
        return;
    }
    V_DrawPatch(x, y, FB, percent);
    WI_drawNum(x, y, p_val, -1);
}

pub unsafe fn WI_drawTime(mut x: i32, y: i32, t: i32) {
    if t < 0 {
        return;
    }

    if t <= 61 * 59 {
        let mut div = 1;
        loop {
            let n = (t / div) % 60;
            x = WI_drawNum(x, y, n, 2) - (SHORT((*colon).width) as i32);
            div *= 60;

            if div == 60 || (t / div) != 0 {
                V_DrawPatch(x, y, FB, colon);
            }
            if (t / div) == 0 {
                break;
            }
        }
    } else {
        V_DrawPatch(x - (SHORT((*sucks).width) as i32), y, FB, sucks);
    }
}

pub unsafe fn WI_End() {
    WI_unloadData();
}

pub unsafe fn WI_initNoState() {
    state = NoState;
    acceleratestage = 0;
    cnt = 10;
}

pub unsafe fn WI_updateNoState() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 {
        WI_End();
        G_WorldDone();
    }
}

static mut snl_pointeron: boolean = false_;

pub unsafe fn WI_initShowNextLoc() {
    state = ShowNextLoc;
    acceleratestage = 0;
    cnt = SHOWNEXTLOCDELAY * TICRATE;
    WI_initAnimatedBack();
}

pub unsafe fn WI_updateShowNextLoc() {
    WI_updateAnimatedBack();
    cnt -= 1;
    if cnt == 0 || acceleratestage != 0 {
        WI_initNoState();
    } else {
        snl_pointeron = ((cnt & 31) < 20) as boolean;
    }
}

pub unsafe fn WI_drawShowNextLoc() {
    WI_slamBackground();
    WI_drawAnimatedBack();

    if gamemode != commercial {
        if (*wbs).epsd > 2 {
            WI_drawEL();
            return;
        }

        let last = if (*wbs).last == 8 {
            (*wbs).next - 1
        } else {
            (*wbs).last
        };

        for i in 0..=last {
            WI_drawOnLnode(i, &mut splat);
        }

        if (*wbs).didsecret != 0 {
            WI_drawOnLnode(8, &mut splat);
        }

        if snl_pointeron != 0 {
            WI_drawOnLnode((*wbs).next, yah.as_mut_ptr());
        }
    }

    if gamemode != commercial || (*wbs).next != 30 {
        WI_drawEL();
    }
}

pub unsafe fn WI_drawNoState() {
    snl_pointeron = true_;
    WI_drawShowNextLoc();
}

pub unsafe fn WI_fragSum(playernum: i32) -> i32 {
    let mut frags_cnt = 0;
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 && i != (playernum as usize) {
            frags_cnt += (*plrs.add(playernum as usize)).frags[i];
        }
    }
    frags_cnt -= (*plrs.add(playernum as usize)).frags[playernum as usize];
    frags_cnt
}

static mut dm_state: i32 = 0;
static mut dm_frags: [[i32; MAXPLAYERS]; MAXPLAYERS] = [[0; MAXPLAYERS]; MAXPLAYERS];
static mut dm_totals: [i32; MAXPLAYERS] = [0; MAXPLAYERS];

pub unsafe fn WI_initDeathmatchStats() {
    state = StatCount;
    acceleratestage = 0;
    dm_state = 1;
    cnt_pause = TICRATE;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            for j in 0..MAXPLAYERS {
                if playeringame[j] != 0 {
                    dm_frags[i][j] = 0;
                }
            }
            dm_totals[i] = 0;
        }
    }

    WI_initAnimatedBack();
}

pub unsafe fn WI_updateDeathmatchStats() {
    WI_updateAnimatedBack();

    if acceleratestage != 0 && dm_state != 4 {
        acceleratestage = 0;
        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                for j in 0..MAXPLAYERS {
                    if playeringame[j] != 0 {
                        dm_frags[i][j] = (*plrs.add(i)).frags[j];
                    }
                }
                dm_totals[i] = WI_fragSum(i as i32);
            }
        }
        S_StartSound(std::ptr::null_mut(), sfx_barexp);
        dm_state = 4;
    }

    if dm_state == 2 {
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }

        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                for j in 0..MAXPLAYERS {
                    if playeringame[j] != 0 && dm_frags[i][j] != (*plrs.add(i)).frags[j] {
                        if (*plrs.add(i)).frags[j] < 0 {
                            dm_frags[i][j] -= 1;
                        } else {
                            dm_frags[i][j] += 1;
                        }
                        if dm_frags[i][j] > 99 {
                            dm_frags[i][j] = 99;
                        }
                        if dm_frags[i][j] < -99 {
                            dm_frags[i][j] = -99;
                        }
                        stillticking = true;
                    }
                }
                dm_totals[i] = WI_fragSum(i as i32);
                if dm_totals[i] > 99 {
                    dm_totals[i] = 99;
                }
                if dm_totals[i] < -99 {
                    dm_totals[i] = -99;
                }
            }
        }
        if !stillticking {
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            dm_state += 1;
        }
    } else if dm_state == 4 {
        if acceleratestage != 0 {
            S_StartSound(std::ptr::null_mut(), sfx_slop);
            if gamemode == commercial {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if (dm_state & 1) != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            dm_state += 1;
            cnt_pause = TICRATE;
        }
    }
}

pub unsafe fn WI_drawDeathmatchStats() {
    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();

    V_DrawPatch(
        DM_TOTALSX - (SHORT((*total).width) as i32) / 2,
        DM_MATRIXY - WI_SPACINGY + 10,
        FB,
        total,
    );
    V_DrawPatch(DM_KILLERSX, DM_KILLERSY, FB, killers);
    V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, FB, victims);

    let mut x = DM_MATRIXX + DM_SPACINGX;
    let mut y = DM_MATRIXY;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            V_DrawPatch(
                x - (SHORT((*p[i]).width) as i32) / 2,
                DM_MATRIXY - WI_SPACINGY,
                FB,
                p[i],
            );
            V_DrawPatch(
                DM_MATRIXX - (SHORT((*p[i]).width) as i32) / 2,
                y,
                FB,
                p[i],
            );

            if i == (me as usize) {
                V_DrawPatch(
                    x - (SHORT((*p[i]).width) as i32) / 2,
                    DM_MATRIXY - WI_SPACINGY,
                    FB,
                    bstar,
                );
                V_DrawPatch(
                    DM_MATRIXX - (SHORT((*p[i]).width) as i32) / 2,
                    y,
                    FB,
                    star,
                );
            }
        }
        x += DM_SPACINGX;
        y += WI_SPACINGY;
    }

    y = DM_MATRIXY + 10;
    let w = SHORT((*num[0]).width) as i32;

    for i in 0..MAXPLAYERS {
        x = DM_MATRIXX + DM_SPACINGX;
        if playeringame[i] != 0 {
            for j in 0..MAXPLAYERS {
                if playeringame[j] != 0 {
                    WI_drawNum(x + w, y, dm_frags[i][j], 2);
                }
                x += DM_SPACINGX;
            }
            WI_drawNum(DM_TOTALSX + w, y, dm_totals[i], 2);
        }
        y += WI_SPACINGY;
    }
}

static mut cnt_frags: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut dofrags: i32 = 0;
static mut ng_state: i32 = 0;

pub unsafe fn WI_initNetgameStats() {
    state = StatCount;
    acceleratestage = 0;
    ng_state = 1;
    cnt_pause = TICRATE;

    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        cnt_kills[i] = 0;
        cnt_items[i] = 0;
        cnt_secret[i] = 0;
        cnt_frags[i] = 0;
        dofrags += WI_fragSum(i as i32);
    }

    dofrags = (dofrags != 0) as i32;
    WI_initAnimatedBack();
}

pub unsafe fn WI_updateNetgameStats() {
    WI_updateAnimatedBack();

    if acceleratestage != 0 && ng_state != 10 {
        acceleratestage = 0;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_kills[i] = ((*plrs.add(i)).skills * 100) / (*wbs).maxkills;
            cnt_items[i] = ((*plrs.add(i)).sitems * 100) / (*wbs).maxitems;
            cnt_secret[i] = ((*plrs.add(i)).ssecret * 100) / (*wbs).maxsecret;
            if dofrags != 0 {
                cnt_frags[i] = WI_fragSum(i as i32);
            }
        }
        S_StartSound(std::ptr::null_mut(), sfx_barexp);
        ng_state = 10;
    }

    if ng_state == 2 {
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_kills[i] += 2;
            if cnt_kills[i] >= ((*plrs.add(i)).skills * 100) / (*wbs).maxkills {
                cnt_kills[i] = ((*plrs.add(i)).skills * 100) / (*wbs).maxkills;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            ng_state += 1;
        }
    } else if ng_state == 4 {
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_items[i] += 2;
            if cnt_items[i] >= ((*plrs.add(i)).sitems * 100) / (*wbs).maxitems {
                cnt_items[i] = ((*plrs.add(i)).sitems * 100) / (*wbs).maxitems;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            ng_state += 1;
        }
    } else if ng_state == 6 {
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_secret[i] += 2;
            if cnt_secret[i] >= ((*plrs.add(i)).ssecret * 100) / (*wbs).maxsecret {
                cnt_secret[i] = ((*plrs.add(i)).ssecret * 100) / (*wbs).maxsecret;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            ng_state += 1 + 2 * ((dofrags == 0) as i32);
        }
    } else if ng_state == 8 {
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        let mut stillticking = false;
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            cnt_frags[i] += 1;
            let fsum = WI_fragSum(i as i32);
            if cnt_frags[i] >= fsum {
                cnt_frags[i] = fsum;
            } else {
                stillticking = true;
            }
        }
        if !stillticking {
            S_StartSound(std::ptr::null_mut(), sfx_pldeth);
            ng_state += 1;
        }
    } else if ng_state == 10 {
        if acceleratestage != 0 {
            S_StartSound(std::ptr::null_mut(), sfx_sgcock);
            if gamemode == commercial {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if (ng_state & 1) != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            ng_state += 1;
            cnt_pause = TICRATE;
        }
    }
}

pub unsafe fn WI_drawNetgameStats() {
    let pwidth = SHORT((*percent).width) as i32;
    let ng_statsx = 32 + (SHORT((*star).width) as i32) / 2 + 32 * ((dofrags == 0) as i32);

    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();

    V_DrawPatch(
        ng_statsx + NG_SPACINGX - (SHORT((*kills).width) as i32),
        NG_STATSY,
        FB,
        kills,
    );
    V_DrawPatch(
        ng_statsx + 2 * NG_SPACINGX - (SHORT((*items).width) as i32),
        NG_STATSY,
        FB,
        items,
    );
    V_DrawPatch(
        ng_statsx + 3 * NG_SPACINGX - (SHORT((*secret).width) as i32),
        NG_STATSY,
        FB,
        secret,
    );

    if dofrags != 0 {
        V_DrawPatch(
            ng_statsx + 4 * NG_SPACINGX - (SHORT((*frags).width) as i32),
            NG_STATSY,
            FB,
            frags,
        );
    }

    let mut y = NG_STATSY + (SHORT((*kills).height) as i32);

    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }

        let mut x = ng_statsx;
        V_DrawPatch(x - (SHORT((*p[i]).width) as i32), y, FB, p[i]);

        if i == (me as usize) {
            V_DrawPatch(x - (SHORT((*p[i]).width) as i32), y, FB, star);
        }

        x += NG_SPACINGX;
        WI_drawPercent(x - pwidth, y + 10, cnt_kills[i]);
        x += NG_SPACINGX;
        WI_drawPercent(x - pwidth, y + 10, cnt_items[i]);
        x += NG_SPACINGX;
        WI_drawPercent(x - pwidth, y + 10, cnt_secret[i]);
        x += NG_SPACINGX;

        if dofrags != 0 {
            WI_drawNum(x, y + 10, cnt_frags[i], -1);
        }

        y += WI_SPACINGY;
    }
}

static mut sp_state: i32 = 0;

pub unsafe fn WI_initStats() {
    state = StatCount;
    acceleratestage = 0;
    sp_state = 1;
    cnt_kills[0] = -1;
    cnt_items[0] = -1;
    cnt_secret[0] = -1;
    cnt_time = -1;
    cnt_par = -1;
    cnt_pause = TICRATE;

    WI_initAnimatedBack();
}

pub unsafe fn WI_updateStats() {
    WI_updateAnimatedBack();

    if acceleratestage != 0 && sp_state != 10 {
        acceleratestage = 0;
        cnt_kills[0] = ((*plrs.add(me as usize)).skills * 100) / (*wbs).maxkills;
        cnt_items[0] = ((*plrs.add(me as usize)).sitems * 100) / (*wbs).maxitems;
        cnt_secret[0] = ((*plrs.add(me as usize)).ssecret * 100) / (*wbs).maxsecret;
        cnt_time = (*plrs.add(me as usize)).stime / TICRATE;
        cnt_par = (*wbs).partime / TICRATE;
        S_StartSound(std::ptr::null_mut(), sfx_barexp);
        sp_state = 10;
    }

    if sp_state == 2 {
        cnt_kills[0] += 2;
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        if cnt_kills[0] >= ((*plrs.add(me as usize)).skills * 100) / (*wbs).maxkills {
            cnt_kills[0] = ((*plrs.add(me as usize)).skills * 100) / (*wbs).maxkills;
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            sp_state += 1;
        }
    } else if sp_state == 4 {
        cnt_items[0] += 2;
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        if cnt_items[0] >= ((*plrs.add(me as usize)).sitems * 100) / (*wbs).maxitems {
            cnt_items[0] = ((*plrs.add(me as usize)).sitems * 100) / (*wbs).maxitems;
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            sp_state += 1;
        }
    } else if sp_state == 6 {
        cnt_secret[0] += 2;
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        if cnt_secret[0] >= ((*plrs.add(me as usize)).ssecret * 100) / (*wbs).maxsecret {
            cnt_secret[0] = ((*plrs.add(me as usize)).ssecret * 100) / (*wbs).maxsecret;
            S_StartSound(std::ptr::null_mut(), sfx_barexp);
            sp_state += 1;
        }
    } else if sp_state == 8 {
        if (bcnt & 3) == 0 {
            S_StartSound(std::ptr::null_mut(), sfx_pistol);
        }
        cnt_time += 3;
        if cnt_time >= (*plrs.add(me as usize)).stime / TICRATE {
            cnt_time = (*plrs.add(me as usize)).stime / TICRATE;
        }
        cnt_par += 3;
        if cnt_par >= (*wbs).partime / TICRATE {
            cnt_par = (*wbs).partime / TICRATE;
            if cnt_time >= (*plrs.add(me as usize)).stime / TICRATE {
                S_StartSound(std::ptr::null_mut(), sfx_barexp);
                sp_state += 1;
            }
        }
    } else if sp_state == 10 {
        if acceleratestage != 0 {
            S_StartSound(std::ptr::null_mut(), sfx_sgcock);
            if gamemode == commercial {
                WI_initNoState();
            } else {
                WI_initShowNextLoc();
            }
        }
    } else if (sp_state & 1) != 0 {
        cnt_pause -= 1;
        if cnt_pause == 0 {
            sp_state += 1;
            cnt_pause = TICRATE;
        }
    }
}

pub unsafe fn WI_drawStats() {
    let lh = (3 * (SHORT((*num[0]).height) as i32)) / 2;

    WI_slamBackground();
    WI_drawAnimatedBack();
    WI_drawLF();

    V_DrawPatch(SP_STATSX, SP_STATSY, FB, kills);
    WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY, cnt_kills[0]);

    V_DrawPatch(SP_STATSX, SP_STATSY + lh, FB, items);
    WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + lh, cnt_items[0]);

    V_DrawPatch(SP_STATSX, SP_STATSY + 2 * lh, FB, sp_secret);
    WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + 2 * lh, cnt_secret[0]);

    V_DrawPatch(SP_TIMEX, SP_TIMEY, FB, time_patch);
    WI_drawTime(SCREENWIDTH / 2 - SP_TIMEX, SP_TIMEY, cnt_time);

    if (*wbs).epsd < 3 {
        V_DrawPatch(SCREENWIDTH / 2 + SP_TIMEX, SP_TIMEY, FB, par);
        WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par);
    }
}

pub unsafe fn WI_checkForAccelerate() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            let player = &mut players[i];
            if player.cmd.buttons & BT_ATTACK != 0 {
                if player.attackdown == 0 {
                    acceleratestage = 1;
                }
                player.attackdown = true_;
            } else {
                player.attackdown = false_;
            }
            if player.cmd.buttons & BT_USE != 0 {
                if player.usedown == 0 {
                    acceleratestage = 1;
                }
                player.usedown = true_;
            } else {
                player.usedown = false_;
            }
        }
    }
}

pub unsafe fn WI_Ticker() {
    bcnt += 1;

    if bcnt == 1 {
        if gamemode == commercial {
            S_ChangeMusic(mus_dm2int, true_);
        } else {
            S_ChangeMusic(mus_inter, true_);
        }
    }

    WI_checkForAccelerate();

    match state {
        StatCount => {
            if deathmatch != 0 {
                WI_updateDeathmatchStats();
            } else if netgame != 0 {
                WI_updateNetgameStats();
            } else {
                WI_updateStats();
            }
        }
        ShowNextLoc => WI_updateShowNextLoc(),
        NoState => WI_updateNoState(),
        _ => {}
    }
}

pub unsafe fn WI_loadData() {
    let mut name = if gamemode == commercial {
        "INTERPIC".to_string()
    } else {
        format!("WIMAP{}", (*wbs).epsd)
    };

    if gamemode == retail && (*wbs).epsd == 3 {
        name = "INTERPIC".to_string();
    }

    bg = W_CacheLumpNameStr(&name, PU_CACHE) as *mut patch_t;
    V_DrawPatch(0, 0, 1, bg);

    if gamemode == commercial {
        NUMCMAPS = 32;
        lnames = Z_Malloc(
            (std::mem::size_of::<*mut patch_t>() as i32) * NUMCMAPS,
            PU_STATIC,
            std::ptr::null_mut(),
        ) as *mut *mut patch_t;
        for i in 0..(NUMCMAPS as usize) {
            let lname = format!("CWILV{:02}", i);
            *lnames.add(i) = W_CacheLumpNameStr(&lname, PU_STATIC) as *mut patch_t;
        }
    } else {
        lnames = Z_Malloc(
            (std::mem::size_of::<*mut patch_t>() * NUMMAPS) as i32,
            PU_STATIC,
            std::ptr::null_mut(),
        ) as *mut *mut patch_t;
        for i in 0..NUMMAPS {
            let lname = format!("WILV{}{}", (*wbs).epsd, i);
            *lnames.add(i) = W_CacheLumpNameStr(&lname, PU_STATIC) as *mut patch_t;
        }

        yah[0] = W_CacheLumpNameStr("WIURH0", PU_STATIC) as *mut patch_t;
        yah[1] = W_CacheLumpNameStr("WIURH1", PU_STATIC) as *mut patch_t;
        splat = W_CacheLumpNameStr("WISPLAT", PU_STATIC) as *mut patch_t;

        if (*wbs).epsd < 3 {
            let epsd = (*wbs).epsd as usize;
            for j in 0..NUMANIMS[epsd] {
                let a = get_anim(epsd, j);
                for i in 0..((*a).nanims as usize) {
                    if epsd != 1 || j != 8 {
                        let aname = format!("WIA{}{:02}{:02}", epsd, j, i);
                        (*a).p[i] = W_CacheLumpNameStr(&aname, PU_STATIC) as *mut patch_t;
                    } else {
                        (*a).p[i] = (*get_anim(1, 4)).p[i];
                    }
                }
            }
        }
    }

    wiminus = W_CacheLumpNameStr("WIMINUS", PU_STATIC) as *mut patch_t;

    for i in 0..10 {
        let nname = format!("WINUM{}", i);
        num[i] = W_CacheLumpNameStr(&nname, PU_STATIC) as *mut patch_t;
    }

    percent = W_CacheLumpNameStr("WIPCNT", PU_STATIC) as *mut patch_t;
    finished = W_CacheLumpNameStr("WIF", PU_STATIC) as *mut patch_t;
    entering = W_CacheLumpNameStr("WIENTER", PU_STATIC) as *mut patch_t;
    kills = W_CacheLumpNameStr("WIOSTK", PU_STATIC) as *mut patch_t;
    secret = W_CacheLumpNameStr("WIOSTS", PU_STATIC) as *mut patch_t;
    sp_secret = W_CacheLumpNameStr("WISCRT2", PU_STATIC) as *mut patch_t;

    // In C: `if (french)` where `french` is enum constant 1 (always true!).
    // When netgame && !deathmatch, loads WIOBJ, else WIOSTI.
    if french != 0 {
        if netgame != 0 && deathmatch == 0 {
            items = W_CacheLumpNameStr("WIOBJ", PU_STATIC) as *mut patch_t;
        } else {
            items = W_CacheLumpNameStr("WIOSTI", PU_STATIC) as *mut patch_t;
        }
    } else {
        items = W_CacheLumpNameStr("WIOSTI", PU_STATIC) as *mut patch_t;
    }

    frags = W_CacheLumpNameStr("WIFRGS", PU_STATIC) as *mut patch_t;
    colon = W_CacheLumpNameStr("WICOLON", PU_STATIC) as *mut patch_t;
    time_patch = W_CacheLumpNameStr("WITIME", PU_STATIC) as *mut patch_t;
    sucks = W_CacheLumpNameStr("WISUCKS", PU_STATIC) as *mut patch_t;
    par = W_CacheLumpNameStr("WIPAR", PU_STATIC) as *mut patch_t;
    killers = W_CacheLumpNameStr("WIKILRS", PU_STATIC) as *mut patch_t;
    victims = W_CacheLumpNameStr("WIVCTMS", PU_STATIC) as *mut patch_t;
    total = W_CacheLumpNameStr("WIMSTT", PU_STATIC) as *mut patch_t;
    star = W_CacheLumpNameStr("STFST01", PU_STATIC) as *mut patch_t;
    bstar = W_CacheLumpNameStr("STFDEAD0", PU_STATIC) as *mut patch_t;

    for i in 0..MAXPLAYERS {
        let pname = format!("STPB{}", i);
        p[i] = W_CacheLumpNameStr(&pname, PU_STATIC) as *mut patch_t;

        let bpname = format!("WIBP{}", i + 1);
        bp[i] = W_CacheLumpNameStr(&bpname, PU_STATIC) as *mut patch_t;
    }
}

pub unsafe fn WI_unloadData() {
    Z_ChangeTag(wiminus as *mut std::ffi::c_void, PU_CACHE);

    for i in 0..10 {
        Z_ChangeTag(num[i] as *mut std::ffi::c_void, PU_CACHE);
    }

    if gamemode == commercial {
        for i in 0..(NUMCMAPS as usize) {
            Z_ChangeTag((*lnames.add(i)) as *mut std::ffi::c_void, PU_CACHE);
        }
    } else {
        Z_ChangeTag(yah[0] as *mut std::ffi::c_void, PU_CACHE);
        Z_ChangeTag(yah[1] as *mut std::ffi::c_void, PU_CACHE);
        Z_ChangeTag(splat as *mut std::ffi::c_void, PU_CACHE);

        for i in 0..NUMMAPS {
            Z_ChangeTag((*lnames.add(i)) as *mut std::ffi::c_void, PU_CACHE);
        }

        if (*wbs).epsd < 3 {
            let epsd = (*wbs).epsd as usize;
            for j in 0..NUMANIMS[epsd] {
                if epsd != 1 || j != 8 {
                    let a = get_anim(epsd, j);
                    for i in 0..((*a).nanims as usize) {
                        Z_ChangeTag((*a).p[i] as *mut std::ffi::c_void, PU_CACHE);
                    }
                }
            }
        }
    }

    Z_Free(lnames as *mut std::ffi::c_void);

    Z_ChangeTag(percent as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(colon as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(finished as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(entering as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(kills as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(secret as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(sp_secret as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(items as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(frags as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(time_patch as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(sucks as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(par as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(victims as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(killers as *mut std::ffi::c_void, PU_CACHE);
    Z_ChangeTag(total as *mut std::ffi::c_void, PU_CACHE);

    for i in 0..MAXPLAYERS {
        Z_ChangeTag(p[i] as *mut std::ffi::c_void, PU_CACHE);
        Z_ChangeTag(bp[i] as *mut std::ffi::c_void, PU_CACHE);
    }
}

pub unsafe fn WI_Drawer() {
    match state {
        StatCount => {
            if deathmatch != 0 {
                WI_drawDeathmatchStats();
            } else if netgame != 0 {
                WI_drawNetgameStats();
            } else {
                WI_drawStats();
            }
        }
        ShowNextLoc => WI_drawShowNextLoc(),
        NoState => WI_drawNoState(),
        _ => {}
    }
}

pub unsafe fn WI_initVariables(wbstartstruct: *mut wbstartstruct_t) {
    wbs = wbstartstruct;
    acceleratestage = 0;
    cnt = 0;
    bcnt = 0;
    firstrefresh = 1;
    me = (*wbs).pnum;
    plrs = (*wbs).plyr.as_mut_ptr();

    if (*wbs).maxkills == 0 {
        (*wbs).maxkills = 1;
    }
    if (*wbs).maxitems == 0 {
        (*wbs).maxitems = 1;
    }
    if (*wbs).maxsecret == 0 {
        (*wbs).maxsecret = 1;
    }

    if gamemode != retail && (*wbs).epsd > 2 {
        (*wbs).epsd -= 3;
    }
}

pub unsafe fn WI_Start(wbstartstruct: *mut wbstartstruct_t) {
    WI_initVariables(wbstartstruct);
    WI_loadData();

    if deathmatch != 0 {
        WI_initDeathmatchStats();
    } else if netgame != 0 {
        WI_initNetgameStats();
    } else {
        WI_initStats();
    }
}
