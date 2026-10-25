// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_setup::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_state::*;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct divline_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union intercept_d {
    pub thing: *mut mobj_t,
    pub line: *mut line_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intercept_t {
    pub frac: fixed_t,
    pub isaline: boolean,
    pub d: intercept_d,
}

pub type traverser_t = unsafe fn(*mut intercept_t) -> boolean;

pub unsafe fn P_AproxDistance(mut dx: fixed_t, mut dy: fixed_t) -> fixed_t {
    dx = dx.wrapping_abs();
    dy = dy.wrapping_abs();
    if dx < dy {
        dx + dy - (dx >> 1)
    } else {
        dx + dy - (dy >> 1)
    }
}

pub unsafe fn P_PointOnLineSide(x: fixed_t, y: fixed_t, line: *mut line_t) -> i32 {
    if (*line).dx == 0 {
        if x <= (*(*line).v1).x {
            return ((*line).dy > 0) as i32;
        }
        return ((*line).dy < 0) as i32;
    }
    if (*line).dy == 0 {
        if y <= (*(*line).v1).y {
            return ((*line).dx < 0) as i32;
        }
        return ((*line).dx > 0) as i32;
    }

    let dx = x - (*(*line).v1).x;
    let dy = y - (*(*line).v1).y;

    let left = FixedMul((*line).dy >> FRACBITS, dx);
    let right = FixedMul(dy, (*line).dx >> FRACBITS);

    if right < left {
        0
    } else {
        1
    }
}

pub unsafe fn P_BoxOnLineSide(tmbox: *mut fixed_t, ld: *mut line_t) -> i32 {
    let mut p1 = 0;
    let mut p2 = 0;

    match (*ld).slopetype {
        ST_HORIZONTAL => {
            p1 = (*tmbox.add(BOXTOP) > (*(*ld).v1).y) as i32;
            p2 = (*tmbox.add(BOXBOTTOM) > (*(*ld).v1).y) as i32;
            if (*ld).dx < 0 {
                p1 ^= 1;
                p2 ^= 1;
            }
        }
        ST_VERTICAL => {
            p1 = (*tmbox.add(BOXRIGHT) < (*(*ld).v1).x) as i32;
            p2 = (*tmbox.add(BOXLEFT) < (*(*ld).v1).x) as i32;
            if (*ld).dy < 0 {
                p1 ^= 1;
                p2 ^= 1;
            }
        }
        ST_POSITIVE => {
            p1 = P_PointOnLineSide(*tmbox.add(BOXLEFT), *tmbox.add(BOXTOP), ld);
            p2 = P_PointOnLineSide(*tmbox.add(BOXRIGHT), *tmbox.add(BOXBOTTOM), ld);
        }
        ST_NEGATIVE => {
            p1 = P_PointOnLineSide(*tmbox.add(BOXRIGHT), *tmbox.add(BOXTOP), ld);
            p2 = P_PointOnLineSide(*tmbox.add(BOXLEFT), *tmbox.add(BOXBOTTOM), ld);
        }
        _ => {}
    }

    if p1 == p2 {
        p1
    } else {
        -1
    }
}

pub unsafe fn P_PointOnDivlineSide(x: fixed_t, y: fixed_t, line: *mut divline_t) -> i32 {
    if (*line).dx == 0 {
        if x <= (*line).x {
            return ((*line).dy > 0) as i32;
        }
        return ((*line).dy < 0) as i32;
    }
    if (*line).dy == 0 {
        if y <= (*line).y {
            return ((*line).dx < 0) as i32;
        }
        return ((*line).dx > 0) as i32;
    }

    let dx = x - (*line).x;
    let dy = y - (*line).y;

    if (((*line).dy ^ (*line).dx ^ dx ^ dy) as u32 & 0x80000000) != 0 {
        if (((*line).dy ^ dx) as u32 & 0x80000000) != 0 {
            return 1;
        }
        return 0;
    }

    let left = FixedMul((*line).dy >> 8, dx >> 8);
    let right = FixedMul(dy >> 8, (*line).dx >> 8);

    if right < left {
        0
    } else {
        1
    }
}

pub unsafe fn P_MakeDivline(li: *mut line_t, dl: *mut divline_t) {
    (*dl).x = (*(*li).v1).x;
    (*dl).y = (*(*li).v1).y;
    (*dl).dx = (*li).dx;
    (*dl).dy = (*li).dy;
}

pub unsafe fn P_InterceptVector(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    let den = FixedMul((*v1).dy >> 8, (*v2).dx) - FixedMul((*v1).dx >> 8, (*v2).dy);
    if den == 0 {
        return 0;
    }
    let num = FixedMul(((*v1).x - (*v2).x) >> 8, (*v1).dy)
        + FixedMul(((*v2).y - (*v1).y) >> 8, (*v1).dx);
    FixedDiv(num, den)
}

pub static mut opentop: fixed_t = 0;
pub static mut openbottom: fixed_t = 0;
pub static mut openrange: fixed_t = 0;
pub static mut lowfloor: fixed_t = 0;

pub unsafe fn P_LineOpening(ld: *mut line_t) {
    if (*ld).sidenum[1] == -1 {
        openrange = 0;
        return;
    }

    let front = (*ld).frontsector;
    let back = (*ld).backsector;

    if (*front).ceilingheight < (*back).ceilingheight {
        opentop = (*front).ceilingheight;
    } else {
        opentop = (*back).ceilingheight;
    }

    if (*front).floorheight > (*back).floorheight {
        openbottom = (*front).floorheight;
        lowfloor = (*back).floorheight;
    } else {
        openbottom = (*back).floorheight;
        lowfloor = (*front).floorheight;
    }

    openrange = opentop - openbottom;
}

pub unsafe fn P_UnsetThingPosition(thing: *mut mobj_t) {
    if (*thing).flags & MF_NOSECTOR == 0 {
        if !(*thing).snext.is_null() {
            (*(*thing).snext).sprev = (*thing).sprev;
        }
        if !(*thing).sprev.is_null() {
            (*(*thing).sprev).snext = (*thing).snext;
        } else {
            (*(*(*thing).subsector).sector).thinglist = (*thing).snext;
        }
    }

    if (*thing).flags & MF_NOBLOCKMAP == 0 {
        if !(*thing).bnext.is_null() {
            (*(*thing).bnext).bprev = (*thing).bprev;
        }
        if !(*thing).bprev.is_null() {
            (*(*thing).bprev).bnext = (*thing).bnext;
        } else {
            let blockx = ((*thing).x - bmaporgx) >> MAPBLOCKSHIFT;
            let blocky = ((*thing).y - bmaporgy) >> MAPBLOCKSHIFT;

            if blockx >= 0 && blockx < bmapwidth && blocky >= 0 && blocky < bmapheight {
                *blocklinks.add((blocky * bmapwidth + blockx) as usize) = (*thing).bnext;
            }
        }
    }
}

pub unsafe fn P_SetThingPosition(thing: *mut mobj_t) {
    let ss = R_PointInSubsector((*thing).x, (*thing).y);
    (*thing).subsector = ss;

    if (*thing).flags & MF_NOSECTOR == 0 {
        let sec = (*ss).sector;
        (*thing).sprev = std::ptr::null_mut();
        (*thing).snext = (*sec).thinglist;

        if !(*sec).thinglist.is_null() {
            (*(*sec).thinglist).sprev = thing;
        }
        (*sec).thinglist = thing;
    }

    if (*thing).flags & MF_NOBLOCKMAP == 0 {
        let blockx = ((*thing).x - bmaporgx) >> MAPBLOCKSHIFT;
        let blocky = ((*thing).y - bmaporgy) >> MAPBLOCKSHIFT;

        if blockx >= 0 && blockx < bmapwidth && blocky >= 0 && blocky < bmapheight {
            let link = blocklinks.add((blocky * bmapwidth + blockx) as usize);
            (*thing).bprev = std::ptr::null_mut();
            (*thing).bnext = *link;
            if !(*link).is_null() {
                (*(*link)).bprev = thing;
            }
            *link = thing;
        } else {
            (*thing).bnext = std::ptr::null_mut();
            (*thing).bprev = std::ptr::null_mut();
        }
    }
}

pub unsafe fn P_BlockLinesIterator(
    x: i32,
    y: i32,
    func: unsafe fn(*mut line_t) -> boolean,
) -> boolean {
    if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
        return true_;
    }

    let mut offset = y * bmapwidth + x;
    offset = (*blockmap.add(offset as usize)) as i32;

    let mut list = blockmaplump.add(offset as usize);
    while *list != -1 {
        let ld = lines.add((*list) as usize);
        if (*ld).validcount == validcount {
            list = list.add(1);
            continue;
        }
        (*ld).validcount = validcount;
        if func(ld) == 0 {
            return false_;
        }
        list = list.add(1);
    }
    true_
}

pub unsafe fn P_BlockThingsIterator(
    x: i32,
    y: i32,
    func: unsafe fn(*mut mobj_t) -> boolean,
) -> boolean {
    if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
        return true_;
    }

    let mut mobj = *blocklinks.add((y * bmapwidth + x) as usize);
    while !mobj.is_null() {
        let next = (*mobj).bnext;
        if func(mobj) == 0 {
            return false_;
        }
        mobj = next;
    }
    true_
}

pub static mut intercepts: [intercept_t; MAXINTERCEPTS] = [intercept_t {
    frac: 0,
    isaline: 0,
    d: intercept_d {
        thing: std::ptr::null_mut(),
    },
}; MAXINTERCEPTS];
pub static mut intercept_p: *mut intercept_t = std::ptr::null_mut();
pub static mut trace: divline_t = divline_t {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};
pub static mut earlyout: boolean = 0;
pub static mut ptflags: i32 = 0;

pub unsafe fn PIT_AddLineIntercepts(ld: *mut line_t) -> boolean {
    let s1;
    let s2;

    if trace.dx > FRACUNIT * 16
        || trace.dy > FRACUNIT * 16
        || trace.dx < -FRACUNIT * 16
        || trace.dy < -FRACUNIT * 16
    {
        s1 = P_PointOnDivlineSide((*(*ld).v1).x, (*(*ld).v1).y, &mut trace);
        s2 = P_PointOnDivlineSide((*(*ld).v2).x, (*(*ld).v2).y, &mut trace);
    } else {
        s1 = P_PointOnLineSide(trace.x, trace.y, ld);
        s2 = P_PointOnLineSide(trace.x + trace.dx, trace.y + trace.dy, ld);
    }

    if s1 == s2 {
        return true_;
    }

    let mut dl = divline_t::default();
    P_MakeDivline(ld, &mut dl);
    let frac = P_InterceptVector(&mut trace, &mut dl);

    if frac < 0 {
        return true_;
    }

    if earlyout != 0 && frac < FRACUNIT && (*ld).backsector.is_null() {
        return false_;
    }

    (*intercept_p).frac = frac;
    (*intercept_p).isaline = true_;
    (*intercept_p).d.line = ld;
    intercept_p = intercept_p.add(1);

    true_
}

pub unsafe fn PIT_AddThingIntercepts(thing: *mut mobj_t) -> boolean {
    let tracepositive = (trace.dx ^ trace.dy) > 0;

    let (x1, y1, x2, y2) = if tracepositive {
        (
            (*thing).x - (*thing).radius,
            (*thing).y + (*thing).radius,
            (*thing).x + (*thing).radius,
            (*thing).y - (*thing).radius,
        )
    } else {
        (
            (*thing).x - (*thing).radius,
            (*thing).y - (*thing).radius,
            (*thing).x + (*thing).radius,
            (*thing).y + (*thing).radius,
        )
    };

    let s1 = P_PointOnDivlineSide(x1, y1, &mut trace);
    let s2 = P_PointOnDivlineSide(x2, y2, &mut trace);

    if s1 == s2 {
        return true_;
    }

    let mut dl = divline_t {
        x: x1,
        y: y1,
        dx: x2 - x1,
        dy: y2 - y1,
    };

    let frac = P_InterceptVector(&mut trace, &mut dl);

    if frac < 0 {
        return true_;
    }

    (*intercept_p).frac = frac;
    (*intercept_p).isaline = false_;
    (*intercept_p).d.thing = thing;
    intercept_p = intercept_p.add(1);

    true_
}

pub unsafe fn P_TraverseIntercepts(func: traverser_t, maxfrac: fixed_t) -> boolean {
    let mut count =
        (intercept_p as usize - intercepts.as_ptr() as usize) / std::mem::size_of::<intercept_t>();
    let mut in_: *mut intercept_t = std::ptr::null_mut();

    while count > 0 {
        count -= 1;
        let mut dist = MAXINT;
        let mut scan = intercepts.as_mut_ptr();
        while (scan as usize) < (intercept_p as usize) {
            if (*scan).frac < dist {
                dist = (*scan).frac;
                in_ = scan;
            }
            scan = scan.add(1);
        }

        if dist > maxfrac {
            return true_;
        }

        if func(in_) == 0 {
            return false_;
        }

        (*in_).frac = MAXINT;
    }

    true_
}

pub unsafe fn P_PathTraverse(
    mut x1: fixed_t,
    mut y1: fixed_t,
    mut x2: fixed_t,
    mut y2: fixed_t,
    flags: i32,
    trav: traverser_t,
) -> boolean {
    earlyout = flags & PT_EARLYOUT;

    validcount += 1;
    intercept_p = intercepts.as_mut_ptr();

    if ((x1 - bmaporgx) & (MAPBLOCKSIZE - 1)) == 0 {
        x1 += FRACUNIT;
    }
    if ((y1 - bmaporgy) & (MAPBLOCKSIZE - 1)) == 0 {
        y1 += FRACUNIT;
    }

    trace.x = x1;
    trace.y = y1;
    trace.dx = x2 - x1;
    trace.dy = y2 - y1;

    x1 -= bmaporgx;
    y1 -= bmaporgy;
    let xt1 = x1 >> MAPBLOCKSHIFT;
    let yt1 = y1 >> MAPBLOCKSHIFT;

    x2 -= bmaporgx;
    y2 -= bmaporgy;
    let xt2 = x2 >> MAPBLOCKSHIFT;
    let yt2 = y2 >> MAPBLOCKSHIFT;

    let mapxstep;
    let mut partial;
    let ystep;

    if xt2 > xt1 {
        mapxstep = 1;
        partial = FRACUNIT - ((x1 >> MAPBTOFRAC) & (FRACUNIT - 1));
        ystep = FixedDiv(y2 - y1, (x2 - x1).wrapping_abs());
    } else if xt2 < xt1 {
        mapxstep = -1;
        partial = (x1 >> MAPBTOFRAC) & (FRACUNIT - 1);
        ystep = FixedDiv(y2 - y1, (x2 - x1).wrapping_abs());
    } else {
        mapxstep = 0;
        partial = FRACUNIT;
        ystep = 256 * FRACUNIT;
    }

    let mut yintercept = (y1 >> MAPBTOFRAC) + FixedMul(partial, ystep);

    let mapystep;
    let xstep;

    if yt2 > yt1 {
        mapystep = 1;
        partial = FRACUNIT - ((y1 >> MAPBTOFRAC) & (FRACUNIT - 1));
        xstep = FixedDiv(x2 - x1, (y2 - y1).wrapping_abs());
    } else if yt2 < yt1 {
        mapystep = -1;
        partial = (y1 >> MAPBTOFRAC) & (FRACUNIT - 1);
        xstep = FixedDiv(x2 - x1, (y2 - y1).wrapping_abs());
    } else {
        mapystep = 0;
        partial = FRACUNIT;
        xstep = 256 * FRACUNIT;
    }

    let mut xintercept = (x1 >> MAPBTOFRAC) + FixedMul(partial, xstep);

    let mut mapx = xt1;
    let mut mapy = yt1;

    for _ in 0..64 {
        if flags & PT_ADDLINES != 0 {
            if P_BlockLinesIterator(mapx, mapy, PIT_AddLineIntercepts) == 0 {
                return false_;
            }
        }
        if flags & PT_ADDTHINGS != 0 {
            if P_BlockThingsIterator(mapx, mapy, PIT_AddThingIntercepts) == 0 {
                return false_;
            }
        }
        if mapx == xt2 && mapy == yt2 {
            break;
        }
        if (yintercept >> FRACBITS) == mapy {
            yintercept += ystep;
            mapx += mapxstep;
        } else if (xintercept >> FRACBITS) == mapx {
            xintercept += xstep;
            mapy += mapystep;
        }
    }

    P_TraverseIntercepts(trav, FRACUNIT)
}