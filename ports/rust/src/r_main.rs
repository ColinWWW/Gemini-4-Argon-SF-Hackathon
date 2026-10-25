// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_net::*;
use crate::d_player::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_menu::{detailLevel, screenblocks};
use crate::p_setup::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_things::*;
use crate::tables::*;

pub const FIELDOFVIEW: usize = 2048;

pub const LIGHTLEVELS: usize = 16;
pub const LIGHTSEGSHIFT: i32 = 4;
pub const MAXLIGHTSCALE: usize = 48;
pub const LIGHTSCALESHIFT: i32 = 12;
pub const MAXLIGHTZ: usize = 128;
pub const LIGHTZSHIFT: i32 = 20;
pub const NUMCOLORMAPS: i32 = 32;

pub static mut viewangleoffset: i32 = 0;
pub static mut validcount: i32 = 1;

pub static mut fixedcolormap: *mut lighttable_t = std::ptr::null_mut();

pub static mut centerx: i32 = 0;
pub static mut centery: i32 = 0;

pub static mut centerxfrac: fixed_t = 0;
pub static mut centeryfrac: fixed_t = 0;
pub static mut projection: fixed_t = 0;

pub static mut framecount: i32 = 0;
pub static mut sscount: i32 = 0;
pub static mut linecount: i32 = 0;
pub static mut loopcount: i32 = 0;

pub static mut viewx: fixed_t = 0;
pub static mut viewy: fixed_t = 0;
pub static mut viewz: fixed_t = 0;

pub static mut viewangle: angle_t = 0;
pub static mut viewcos: fixed_t = 0;
pub static mut viewsin: fixed_t = 0;

pub static mut viewplayer: *mut player_t = std::ptr::null_mut();

pub static mut detailshift: i32 = 0;

pub static mut clipangle: angle_t = 0;
pub static mut viewangletox: [i32; FINEANGLES / 2] = [0; FINEANGLES / 2];
pub static mut xtoviewangle: [angle_t; (SCREENWIDTH as usize) + 1] =
    [0; (SCREENWIDTH as usize) + 1];

pub static mut scalelight: [[*mut lighttable_t; MAXLIGHTSCALE]; LIGHTLEVELS] =
    [[std::ptr::null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS];
pub static mut scalelightfixed: [*mut lighttable_t; MAXLIGHTSCALE] =
    [std::ptr::null_mut(); MAXLIGHTSCALE];
pub static mut zlight: [[*mut lighttable_t; MAXLIGHTZ]; LIGHTLEVELS] =
    [[std::ptr::null_mut(); MAXLIGHTZ]; LIGHTLEVELS];

pub static mut extralight: i32 = 0;

pub static mut colfunc: Option<unsafe fn()> = None;
pub static mut basecolfunc: Option<unsafe fn()> = None;
pub static mut fuzzcolfunc: Option<unsafe fn()> = None;
pub static mut transcolfunc: Option<unsafe fn()> = None;
pub static mut spanfunc: Option<unsafe fn()> = None;

pub fn SlopeDiv(num: u32, den: u32) -> u32 {
    let mut ans: u32;
    if den < 512 {
        return SLOPERANGE as u32;
    }
    ans = ((num << 3) / (den >> 8));
    if ans > SLOPERANGE as u32 {
        ans = SLOPERANGE as u32;
    }
    ans
}

pub unsafe fn R_AddPointToBox(x: i32, y: i32, box_: *mut fixed_t) {
    if x < *box_.add(BOXLEFT) {
        *box_.add(BOXLEFT) = x;
    }
    if x > *box_.add(BOXRIGHT) {
        *box_.add(BOXRIGHT) = x;
    }
    if y < *box_.add(BOXBOTTOM) {
        *box_.add(BOXBOTTOM) = y;
    }
    if y > *box_.add(BOXTOP) {
        *box_.add(BOXTOP) = y;
    }
}

pub unsafe fn R_PointOnSide(x: fixed_t, y: fixed_t, node: *mut node_t) -> i32 {
    if (*node).dx == 0 {
        if x <= (*node).x {
            return ((*node).dy > 0) as i32;
        }
        return ((*node).dy < 0) as i32;
    }
    if (*node).dy == 0 {
        if y <= (*node).y {
            return ((*node).dx < 0) as i32;
        }
        return ((*node).dx > 0) as i32;
    }

    let dx = x - (*node).x;
    let dy = y - (*node).y;

    if (((*node).dy ^ (*node).dx ^ dx ^ dy) as u32 & 0x80000000) != 0 {
        if (((*node).dy ^ dx) as u32 & 0x80000000) != 0 {
            return 1;
        }
        return 0;
    }

    let left = FixedMul((*node).dy >> FRACBITS, dx);
    let right = FixedMul(dy, (*node).dx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

pub unsafe fn R_PointOnSegSide(x: fixed_t, y: fixed_t, line: *mut seg_t) -> i32 {
    let lx = (*(*line).v1).x;
    let ly = (*(*line).v1).y;

    let ldx = (*(*line).v2).x - lx;
    let ldy = (*(*line).v2).y - ly;

    if ldx == 0 {
        if x <= lx {
            return (ldy > 0) as i32;
        }
        return (ldy < 0) as i32;
    }
    if ldy == 0 {
        if y <= ly {
            return (ldx < 0) as i32;
        }
        return (ldx > 0) as i32;
    }

    let dx = x - lx;
    let dy = y - ly;

    if ((ldy ^ ldx ^ dx ^ dy) as u32 & 0x80000000) != 0 {
        if ((ldy ^ dx) as u32 & 0x80000000) != 0 {
            return 1;
        }
        return 0;
    }

    let left = FixedMul(ldy >> FRACBITS, dx);
    let right = FixedMul(dy, ldx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

pub unsafe fn R_PointToAngle(mut x: fixed_t, mut y: fixed_t) -> angle_t {
    x -= viewx;
    y -= viewy;

    if x == 0 && y == 0 {
        return 0;
    }

    if x >= 0 {
        if y >= 0 {
            if x > y {
                return tantoangle[SlopeDiv(y as u32, x as u32) as usize];
            } else {
                return (ANG90 - 1).wrapping_sub(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        } else {
            y = -y;
            if x > y {
                return 0u32.wrapping_sub(tantoangle[SlopeDiv(y as u32, x as u32) as usize]);
            } else {
                return ANG270.wrapping_add(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        }
    } else {
        x = -x;
        if y >= 0 {
            if x > y {
                return (ANG180 - 1).wrapping_sub(tantoangle[SlopeDiv(y as u32, x as u32) as usize]);
            } else {
                return ANG90.wrapping_add(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        } else {
            y = -y;
            if x > y {
                return ANG180.wrapping_add(tantoangle[SlopeDiv(y as u32, x as u32) as usize]);
            } else {
                return (ANG270 - 1).wrapping_sub(tantoangle[SlopeDiv(x as u32, y as u32) as usize]);
            }
        }
    }
}

pub unsafe fn R_PointToAngle2(x1: fixed_t, y1: fixed_t, x2: fixed_t, y2: fixed_t) -> angle_t {
    viewx = x1;
    viewy = y1;
    R_PointToAngle(x2, y2)
}

pub unsafe fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
    let mut dx = (x - viewx).wrapping_abs();
    let mut dy = (y - viewy).wrapping_abs();

    if dy > dx {
        let temp = dx;
        dx = dy;
        dy = temp;
    }

    let angle =
        (tantoangle[(FixedDiv(dy, dx) >> DBITS) as usize].wrapping_add(ANG90)) >> ANGLETOFINESHIFT;

    FixedDiv(dx, finesine[angle as usize])
}

pub unsafe fn R_InitPointToAngle() {}

pub unsafe fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
    let anglea = ANG90.wrapping_add(visangle.wrapping_sub(viewangle));
    let angleb = ANG90.wrapping_add(visangle.wrapping_sub(rw_normalangle));

    let sinea = finesine[(anglea >> ANGLETOFINESHIFT) as usize];
    let sineb = finesine[(angleb >> ANGLETOFINESHIFT) as usize];
    let num = FixedMul(projection, sineb) << detailshift;
    let den = FixedMul(rw_distance, sinea);

    let mut scale = if den > (num >> 16) {
        let mut s = FixedDiv(num, den);
        if s > 64 * FRACUNIT {
            s = 64 * FRACUNIT;
        } else if s < 256 {
            s = 256;
        }
        s
    } else {
        64 * FRACUNIT
    };

    scale
}

pub unsafe fn R_InitTables() {}

pub unsafe fn R_InitTextureMapping() {
    let focallength = FixedDiv(
        centerxfrac,
        finetangent[FINEANGLES / 4 + FIELDOFVIEW / 2],
    );

    for i in 0..(FINEANGLES / 2) {
        let mut t = if finetangent[i] > FRACUNIT * 2 {
            -1
        } else if finetangent[i] < -FRACUNIT * 2 {
            viewwidth + 1
        } else {
            let mut tmp = FixedMul(finetangent[i], focallength);
            tmp = (centerxfrac - tmp + FRACUNIT - 1) >> FRACBITS;
            if tmp < -1 {
                tmp = -1;
            } else if tmp > viewwidth + 1 {
                tmp = viewwidth + 1;
            }
            tmp
        };
        viewangletox[i] = t;
    }

    for x in 0..=(viewwidth as usize) {
        let mut i = 0;
        while viewangletox[i] > (x as i32) {
            i += 1;
        }
        xtoviewangle[x] = ((i as u32) << ANGLETOFINESHIFT).wrapping_sub(ANG90);
    }

    for i in 0..(FINEANGLES / 2) {
        if viewangletox[i] == -1 {
            viewangletox[i] = 0;
        } else if viewangletox[i] == viewwidth + 1 {
            viewangletox[i] = viewwidth;
        }
    }

    clipangle = xtoviewangle[0];
}

pub const DISTMAP: i32 = 2;

pub unsafe fn R_InitLightTables() {
    for i in 0..LIGHTLEVELS {
        let startmap = (((LIGHTLEVELS - 1 - i) * 2) as i32) * NUMCOLORMAPS / (LIGHTLEVELS as i32);
        for j in 0..MAXLIGHTZ {
            let mut scale =
                FixedDiv(SCREENWIDTH / 2 * FRACUNIT, ((j + 1) as i32) << LIGHTZSHIFT);
            scale >>= LIGHTSCALESHIFT;
            let mut level = startmap - scale / DISTMAP;

            if level < 0 {
                level = 0;
            }
            if level >= NUMCOLORMAPS {
                level = NUMCOLORMAPS - 1;
            }

            zlight[i][j] = colormaps.add((level * 256) as usize);
        }
    }
}

pub static mut setsizeneeded: boolean = 0;
pub static mut setblocks: i32 = 0;
pub static mut setdetail: i32 = 0;

pub unsafe fn R_SetViewSize(blocks: i32, detail: i32) {
    setsizeneeded = true_;
    setblocks = blocks;
    setdetail = detail;
}

pub unsafe fn R_ExecuteSetViewSize() {
    setsizeneeded = false_;

    if setblocks == 11 {
        scaledviewwidth = SCREENWIDTH;
        viewheight = SCREENHEIGHT;
    } else {
        scaledviewwidth = setblocks * 32;
        viewheight = (setblocks * 168 / 10) & !7;
    }

    detailshift = setdetail;
    viewwidth = scaledviewwidth >> detailshift;

    centery = viewheight / 2;
    centerx = viewwidth / 2;
    centerxfrac = centerx << FRACBITS;
    centeryfrac = centery << FRACBITS;
    projection = centerxfrac;

    if detailshift == 0 {
        colfunc = Some(R_DrawColumn);
        basecolfunc = Some(R_DrawColumn);
        fuzzcolfunc = Some(R_DrawFuzzColumn);
        transcolfunc = Some(R_DrawTranslatedColumn);
        spanfunc = Some(R_DrawSpan);
    } else {
        colfunc = Some(R_DrawColumnLow);
        basecolfunc = Some(R_DrawColumnLow);
        fuzzcolfunc = Some(R_DrawFuzzColumn);
        transcolfunc = Some(R_DrawTranslatedColumn);
        spanfunc = Some(R_DrawSpanLow);
    }

    R_InitBuffer(scaledviewwidth, viewheight);
    R_InitTextureMapping();

    pspritescale = FRACUNIT * viewwidth / SCREENWIDTH;
    pspriteiscale = FRACUNIT * SCREENWIDTH / viewwidth;

    for i in 0..(viewwidth as usize) {
        screenheightarray[i] = viewheight as i16;
    }

    for i in 0..(viewheight as usize) {
        let mut dy = (((i as i32) - viewheight / 2) << FRACBITS) + FRACUNIT / 2;
        dy = dy.wrapping_abs();
        yslope[i] = FixedDiv(((viewwidth << detailshift) / 2) * FRACUNIT, dy);
    }

    for i in 0..(viewwidth as usize) {
        let cosadj = finecosine((xtoviewangle[i] >> ANGLETOFINESHIFT) as usize).wrapping_abs();
        distscale[i] = FixedDiv(FRACUNIT, cosadj);
    }

    for i in 0..LIGHTLEVELS {
        let startmap = (((LIGHTLEVELS - 1 - i) * 2) as i32) * NUMCOLORMAPS / (LIGHTLEVELS as i32);
        for j in 0..MAXLIGHTSCALE {
            let mut level =
                startmap - (j as i32) * SCREENWIDTH / (viewwidth << detailshift) / DISTMAP;
            if level < 0 {
                level = 0;
            }
            if level >= NUMCOLORMAPS {
                level = NUMCOLORMAPS - 1;
            }
            scalelight[i][j] = colormaps.add((level * 256) as usize);
        }
    }
}

pub unsafe fn R_Init() {
    R_InitData();
    print!("\nR_InitData");
    R_InitPointToAngle();
    print!("\nR_InitPointToAngle");
    R_InitTables();
    print!("\nR_InitTables");

    R_SetViewSize(screenblocks, detailLevel);
    R_InitPlanes();
    print!("\nR_InitPlanes");
    R_InitLightTables();
    print!("\nR_InitLightTables");
    R_InitSkyMap();
    print!("\nR_InitSkyMap");
    R_InitTranslationTables();
    print!("\nR_InitTranslationsTables");

    framecount = 0;
}

pub unsafe fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t {
    if numnodes == 0 {
        return subsectors;
    }

    let mut nodenum = numnodes - 1;

    while (nodenum & NF_SUBSECTOR) == 0 {
        let node = nodes.add(nodenum as usize);
        let side = R_PointOnSide(x, y, node);
        nodenum = (*node).children[side as usize] as i32;
    }

    subsectors.add((nodenum & !NF_SUBSECTOR) as usize)
}

pub unsafe fn R_SetupFrame(player: *mut player_t) {
    viewplayer = player;
    viewx = (*(*player).mo).x;
    viewy = (*(*player).mo).y;
    viewangle = (*(*player).mo).angle.wrapping_add(viewangleoffset as u32);
    extralight = (*player).extralight;

    viewz = (*player).viewz;

    viewsin = finesine[(viewangle >> ANGLETOFINESHIFT) as usize];
    viewcos = finecosine((viewangle >> ANGLETOFINESHIFT) as usize);

    sscount = 0;

    if (*player).fixedcolormap != 0 {
        fixedcolormap = colormaps.add(((*player).fixedcolormap * 256) as usize);
        walllights = scalelightfixed.as_mut_ptr();

        for i in 0..MAXLIGHTSCALE {
            scalelightfixed[i] = fixedcolormap;
        }
    } else {
        fixedcolormap = std::ptr::null_mut();
    }

    framecount += 1;
    validcount += 1;
}

pub unsafe fn R_RenderPlayerView(player: *mut player_t) {
    R_SetupFrame(player);

    R_ClearClipSegs();
    R_ClearDrawSegs();
    R_ClearPlanes();
    R_ClearSprites();

    NetUpdate();

    R_RenderBSPNode(numnodes - 1);

    NetUpdate();

    R_DrawPlanes();

    NetUpdate();

    R_DrawMasked();

    NetUpdate();
}