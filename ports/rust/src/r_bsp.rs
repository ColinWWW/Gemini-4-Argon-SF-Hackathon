// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdata::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::p_setup::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_segs::*;
use crate::r_sky::*;
use crate::r_things::*;
use crate::tables::*;

pub static mut curline: *mut seg_t = std::ptr::null_mut();
pub static mut sidedef: *mut side_t = std::ptr::null_mut();
pub static mut linedef: *mut line_t = std::ptr::null_mut();
pub static mut frontsector: *mut sector_t = std::ptr::null_mut();
pub static mut backsector: *mut sector_t = std::ptr::null_mut();

pub static mut drawsegs: [drawseg_t; MAXDRAWSEGS] = [drawseg_t {
    curline: std::ptr::null_mut(),
    x1: 0,
    x2: 0,
    scale1: 0,
    scale2: 0,
    scalestep: 0,
    silhouette: 0,
    bsilheight: 0,
    tsilheight: 0,
    sprtopclip: std::ptr::null_mut(),
    sprbottomclip: std::ptr::null_mut(),
    maskedtexturecol: std::ptr::null_mut(),
}; MAXDRAWSEGS];
pub static mut ds_p: *mut drawseg_t = std::ptr::null_mut();

pub unsafe fn R_ClearDrawSegs() {
    ds_p = drawsegs.as_mut_ptr();
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct cliprange_t {
    pub first: i32,
    pub last: i32,
}

pub const MAXSEGS: usize = 32;
pub static mut newend: *mut cliprange_t = std::ptr::null_mut();
pub static mut solidsegs: [cliprange_t; MAXSEGS] = [cliprange_t { first: 0, last: 0 }; MAXSEGS];

pub unsafe fn R_ClipSolidWallSegment(first: i32, last: i32) {
    let mut start = solidsegs.as_mut_ptr();
    while (*start).last < first - 1 {
        start = start.add(1);
    }

    if first < (*start).first {
        if last < (*start).first - 1 {
            R_StoreWallRange(first, last);
            let mut next = newend;
            newend = newend.add(1);

            while next != start {
                *next = *next.sub(1);
                next = next.sub(1);
            }
            (*next).first = first;
            (*next).last = last;
            return;
        }

        R_StoreWallRange(first, (*start).first - 1);
        (*start).first = first;
    }

    if last <= (*start).last {
        return;
    }

    let mut next = start;
    while last >= (*next.add(1)).first - 1 {
        R_StoreWallRange((*next).last + 1, (*next.add(1)).first - 1);
        next = next.add(1);

        if last <= (*next).last {
            (*start).last = (*next).last;
            if next == start {
                return;
            }
            loop {
                next = next.add(1);
                if next == newend.add(1) {
                    break;
                }
                start = start.add(1);
                *start = *next;
            }
            newend = start.add(1);
            return;
        }
    }

    R_StoreWallRange((*next).last + 1, last);
    (*start).last = last;

    if next == start {
        return;
    }

    loop {
        next = next.add(1);
        if next == newend.add(1) {
            break;
        }
        start = start.add(1);
        *start = *next;
    }
    newend = start.add(1);
}

pub unsafe fn R_ClipPassWallSegment(first: i32, last: i32) {
    let mut start = solidsegs.as_mut_ptr();
    while (*start).last < first - 1 {
        start = start.add(1);
    }

    if first < (*start).first {
        if last < (*start).first - 1 {
            R_StoreWallRange(first, last);
            return;
        }
        R_StoreWallRange(first, (*start).first - 1);
    }

    if last <= (*start).last {
        return;
    }

    while last >= (*start.add(1)).first - 1 {
        R_StoreWallRange((*start).last + 1, (*start.add(1)).first - 1);
        start = start.add(1);

        if last <= (*start).last {
            return;
        }
    }

    R_StoreWallRange((*start).last + 1, last);
}

pub unsafe fn R_ClearClipSegs() {
    solidsegs[0].first = -0x7fffffff;
    solidsegs[0].last = -1;
    solidsegs[1].first = viewwidth;
    solidsegs[1].last = 0x7fffffff;
    newend = solidsegs.as_mut_ptr().add(2);
}

pub unsafe fn R_AddLine(line: *mut seg_t) {
    curline = line;

    let mut angle1 = R_PointToAngle((*(*line).v1).x, (*(*line).v1).y);
    let mut angle2 = R_PointToAngle((*(*line).v2).x, (*(*line).v2).y);

    let span = angle1.wrapping_sub(angle2);

    if span >= ANG180 {
        return;
    }

    rw_angle1 = angle1 as i32;
    angle1 = angle1.wrapping_sub(viewangle);
    angle2 = angle2.wrapping_sub(viewangle);

    let mut tspan = angle1.wrapping_add(clipangle);
    if tspan > 2u32.wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub(2u32.wrapping_mul(clipangle));
        if tspan >= span {
            return;
        }
        angle1 = clipangle;
    }
    tspan = clipangle.wrapping_sub(angle2);
    if tspan > 2u32.wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub(2u32.wrapping_mul(clipangle));
        if tspan >= span {
            return;
        }
        angle2 = 0u32.wrapping_sub(clipangle);
    }

    angle1 = angle1.wrapping_add(ANG90) >> ANGLETOFINESHIFT;
    angle2 = angle2.wrapping_add(ANG90) >> ANGLETOFINESHIFT;
    let x1 = viewangletox[angle1 as usize];
    let x2 = viewangletox[angle2 as usize];

    if x1 == x2 {
        return;
    }

    backsector = (*line).backsector;

    if backsector.is_null() {
        R_ClipSolidWallSegment(x1, x2 - 1);
        return;
    }

    if (*backsector).ceilingheight <= (*frontsector).floorheight
        || (*backsector).floorheight >= (*frontsector).ceilingheight
    {
        R_ClipSolidWallSegment(x1, x2 - 1);
        return;
    }

    if (*backsector).ceilingheight != (*frontsector).ceilingheight
        || (*backsector).floorheight != (*frontsector).floorheight
    {
        R_ClipPassWallSegment(x1, x2 - 1);
        return;
    }

    if (*backsector).ceilingpic == (*frontsector).ceilingpic
        && (*backsector).floorpic == (*frontsector).floorpic
        && (*backsector).lightlevel == (*frontsector).lightlevel
        && (*(*curline).sidedef).midtexture == 0
    {
        return;
    }

    R_ClipPassWallSegment(x1, x2 - 1);
}

pub static mut checkcoord: [[usize; 4]; 12] = [
    [3, 0, 2, 1],
    [3, 0, 2, 0],
    [3, 1, 2, 0],
    [0, 0, 0, 0],
    [2, 0, 2, 1],
    [0, 0, 0, 0],
    [3, 1, 3, 0],
    [0, 0, 0, 0],
    [2, 0, 3, 1],
    [2, 1, 3, 1],
    [2, 1, 3, 0],
    [0, 0, 0, 0],
];

pub unsafe fn R_CheckBBox(bspcoord: *mut fixed_t) -> boolean {
    let boxx = if viewx <= *bspcoord.add(BOXLEFT) {
        0
    } else if viewx < *bspcoord.add(BOXRIGHT) {
        1
    } else {
        2
    };

    let boxy = if viewy >= *bspcoord.add(BOXTOP) {
        0
    } else if viewy > *bspcoord.add(BOXBOTTOM) {
        1
    } else {
        2
    };

    let boxpos = (boxy << 2) + boxx;
    if boxpos == 5 {
        return true_;
    }

    let x1 = *bspcoord.add(checkcoord[boxpos][0]);
    let y1 = *bspcoord.add(checkcoord[boxpos][1]);
    let x2 = *bspcoord.add(checkcoord[boxpos][2]);
    let y2 = *bspcoord.add(checkcoord[boxpos][3]);

    let mut angle1 = R_PointToAngle(x1, y1).wrapping_sub(viewangle);
    let mut angle2 = R_PointToAngle(x2, y2).wrapping_sub(viewangle);

    let span = angle1.wrapping_sub(angle2);

    if span >= ANG180 {
        return true_;
    }

    let mut tspan = angle1.wrapping_add(clipangle);
    if tspan > 2u32.wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub(2u32.wrapping_mul(clipangle));
        if tspan >= span {
            return false_;
        }
        angle1 = clipangle;
    }
    tspan = clipangle.wrapping_sub(angle2);
    if tspan > 2u32.wrapping_mul(clipangle) {
        tspan = tspan.wrapping_sub(2u32.wrapping_mul(clipangle));
        if tspan >= span {
            return false_;
        }
        angle2 = 0u32.wrapping_sub(clipangle);
    }

    angle1 = angle1.wrapping_add(ANG90) >> ANGLETOFINESHIFT;
    angle2 = angle2.wrapping_add(ANG90) >> ANGLETOFINESHIFT;
    let sx1 = viewangletox[angle1 as usize];
    let mut sx2 = viewangletox[angle2 as usize];

    if sx1 == sx2 {
        return false_;
    }
    sx2 -= 1;

    let mut start = solidsegs.as_mut_ptr();
    while (*start).last < sx2 {
        start = start.add(1);
    }

    if sx1 >= (*start).first && sx2 <= (*start).last {
        return false_;
    }

    true_
}

pub unsafe fn R_Subsector(num: i32) {
    if num >= numsubsectors {
        I_Error(&format!(
            "R_Subsector: ss {} with numss = {}",
            num, numsubsectors
        ));
    }

    sscount += 1;
    let sub = subsectors.add(num as usize);
    frontsector = (*sub).sector;
    let mut count = (*sub).numlines as i32;
    let mut line = segs.add((*sub).firstline as usize);

    if (*frontsector).floorheight < viewz {
        floorplane = R_FindPlane(
            (*frontsector).floorheight,
            (*frontsector).floorpic as i32,
            (*frontsector).lightlevel as i32,
        );
    } else {
        floorplane = std::ptr::null_mut();
    }

    if (*frontsector).ceilingheight > viewz
        || ((*frontsector).ceilingpic as i32) == skyflatnum
    {
        ceilingplane = R_FindPlane(
            (*frontsector).ceilingheight,
            (*frontsector).ceilingpic as i32,
            (*frontsector).lightlevel as i32,
        );
    } else {
        ceilingplane = std::ptr::null_mut();
    }

    R_AddSprites(frontsector);

    while count > 0 {
        count -= 1;
        R_AddLine(line);
        line = line.add(1);
    }
}

pub unsafe fn R_RenderBSPNode(bspnum: i32) {
    if bspnum & NF_SUBSECTOR != 0 {
        if bspnum == -1 {
            R_Subsector(0);
        } else {
            R_Subsector(bspnum & !NF_SUBSECTOR);
        }
        return;
    }

    let bsp = nodes.add(bspnum as usize);
    let side = R_PointOnSide(viewx, viewy, bsp);

    R_RenderBSPNode((*bsp).children[side as usize] as i32);

    if R_CheckBBox((*bsp).bbox[(side ^ 1) as usize].as_mut_ptr()) != 0 {
        R_RenderBSPNode((*bsp).children[(side ^ 1) as usize] as i32);
    }
}
