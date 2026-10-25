// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdata::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::p_maputl::*;
use crate::p_mobj::*;
use crate::p_setup::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_state::*;

pub static mut sightzstart: fixed_t = 0;
pub static mut topslope: fixed_t = 0;
pub static mut bottomslope: fixed_t = 0;

pub static mut strace: divline_t = divline_t {
    x: 0,
    y: 0,
    dx: 0,
    dy: 0,
};
pub static mut t2x: fixed_t = 0;
pub static mut t2y: fixed_t = 0;

pub static mut sightcounts: [i32; 2] = [0; 2];

pub unsafe fn P_DivlineSide(x: fixed_t, y: fixed_t, node: *mut divline_t) -> i32 {
    if (*node).dx == 0 {
        if x == (*node).x {
            return 2;
        }
        if x <= (*node).x {
            return ((*node).dy > 0) as i32;
        }
        return ((*node).dy < 0) as i32;
    }

    if (*node).dy == 0 {
        // Faithful bug from original DOOM: `if (x == node->y)`
        if x == (*node).y {
            return 2;
        }
        if y <= (*node).y {
            return ((*node).dx < 0) as i32;
        }
        return ((*node).dx > 0) as i32;
    }

    let dx = x - (*node).x;
    let dy = y - (*node).y;

    let left = ((*node).dy >> FRACBITS) * (dx >> FRACBITS);
    let right = (dy >> FRACBITS) * ((*node).dx >> FRACBITS);

    if right < left {
        return 0;
    }
    if left == right {
        return 2;
    }
    1
}

pub unsafe fn P_InterceptVector2(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    let den = FixedMul((*v1).dy >> 8, (*v2).dx) - FixedMul((*v1).dx >> 8, (*v2).dy);
    if den == 0 {
        return 0;
    }
    let num = FixedMul(((*v1).x - (*v2).x) >> 8, (*v1).dy)
        + FixedMul(((*v2).y - (*v1).y) >> 8, (*v1).dx);
    FixedDiv(num, den)
}

pub unsafe fn P_CrossSubsector(num: i32) -> boolean {
    if num >= numsubsectors {
        I_Error(&format!(
            "P_CrossSubsector: ss {} with numss = {}",
            num, numsubsectors
        ));
    }

    let sub = subsectors.add(num as usize);
    let mut count = (*sub).numlines as i32;
    let mut seg = segs.add((*sub).firstline as usize);

    while count > 0 {
        let line = (*seg).linedef;

        if (*line).validcount == validcount {
            seg = seg.add(1);
            count -= 1;
            continue;
        }

        (*line).validcount = validcount;

        let v1 = (*line).v1;
        let v2 = (*line).v2;
        let mut s1 = P_DivlineSide((*v1).x, (*v1).y, &mut strace);
        let mut s2 = P_DivlineSide((*v2).x, (*v2).y, &mut strace);

        if s1 == s2 {
            seg = seg.add(1);
            count -= 1;
            continue;
        }

        let mut divl = divline_t {
            x: (*v1).x,
            y: (*v1).y,
            dx: (*v2).x - (*v1).x,
            dy: (*v2).y - (*v1).y,
        };
        s1 = P_DivlineSide(strace.x, strace.y, &mut divl);
        s2 = P_DivlineSide(t2x, t2y, &mut divl);

        if s1 == s2 {
            seg = seg.add(1);
            count -= 1;
            continue;
        }

        if ((*line).flags as i32) & ML_TWOSIDED == 0 {
            return false_;
        }

        let front = (*seg).frontsector;
        let back = (*seg).backsector;

        if (*front).floorheight == (*back).floorheight
            && (*front).ceilingheight == (*back).ceilingheight
        {
            seg = seg.add(1);
            count -= 1;
            continue;
        }

        let opentop_l = if (*front).ceilingheight < (*back).ceilingheight {
            (*front).ceilingheight
        } else {
            (*back).ceilingheight
        };

        let openbottom_l = if (*front).floorheight > (*back).floorheight {
            (*front).floorheight
        } else {
            (*back).floorheight
        };

        if openbottom_l >= opentop_l {
            return false_;
        }

        let frac = P_InterceptVector2(&mut strace, &mut divl);

        if (*front).floorheight != (*back).floorheight {
            let slope = FixedDiv(openbottom_l - sightzstart, frac);
            if slope > bottomslope {
                bottomslope = slope;
            }
        }

        if (*front).ceilingheight != (*back).ceilingheight {
            let slope = FixedDiv(opentop_l - sightzstart, frac);
            if slope < topslope {
                topslope = slope;
            }
        }

        if topslope <= bottomslope {
            return false_;
        }

        seg = seg.add(1);
        count -= 1;
    }

    true_
}

pub unsafe fn P_CrossBSPNode(bspnum: i32) -> boolean {
    if bspnum & NF_SUBSECTOR != 0 {
        if bspnum == -1 {
            return P_CrossSubsector(0);
        } else {
            return P_CrossSubsector(bspnum & !NF_SUBSECTOR);
        }
    }

    let bsp = nodes.add(bspnum as usize);

    let mut side = P_DivlineSide(strace.x, strace.y, bsp as *mut divline_t);
    if side == 2 {
        side = 0;
    }

    if P_CrossBSPNode((*bsp).children[side as usize] as i32) == 0 {
        return false_;
    }

    if side == P_DivlineSide(t2x, t2y, bsp as *mut divline_t) {
        return true_;
    }

    P_CrossBSPNode((*bsp).children[(side ^ 1) as usize] as i32)
}

pub unsafe fn P_CheckSight(t1: *mut mobj_t, t2: *mut mobj_t) -> boolean {
    let s1 = (((*(*t1).subsector).sector as usize) - (sectors as usize))
        / std::mem::size_of::<sector_t>();
    let s2 = (((*(*t2).subsector).sector as usize) - (sectors as usize))
        / std::mem::size_of::<sector_t>();
    let pnum = (s1 as i32) * numsectors + (s2 as i32);
    let bytenum = pnum >> 3;
    let bitnum = 1u8 << (pnum & 7);

    if (*rejectmatrix.add(bytenum as usize)) & bitnum != 0 {
        sightcounts[0] += 1;
        return false_;
    }

    sightcounts[1] += 1;
    validcount += 1;

    sightzstart = (*t1).z + (*t1).height - ((*t1).height >> 2);
    topslope = ((*t2).z + (*t2).height) - sightzstart;
    bottomslope = (*t2).z - sightzstart;

    strace.x = (*t1).x;
    strace.y = (*t1).y;
    t2x = (*t2).x;
    t2y = (*t2).y;
    strace.dx = (*t2).x - (*t1).x;
    strace.dy = (*t2).y - (*t1).y;

    P_CrossBSPNode(numnodes - 1)
}
