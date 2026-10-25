// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_player::*;
use crate::doomdata::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_inter::*;
use crate::p_local::*;
use crate::p_maputl::*;
use crate::p_mobj::*;
use crate::p_setup::*;
use crate::p_sight::*;
use crate::p_spec::*;
use crate::p_switch::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_sky::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;

pub static mut tmbbox: [fixed_t; 4] = [0; 4];
pub static mut tmthing: *mut mobj_t = std::ptr::null_mut();
pub static mut tmflags: i32 = 0;
pub static mut tmx: fixed_t = 0;
pub static mut tmy: fixed_t = 0;

pub static mut floatok: boolean = 0;
pub static mut tmfloorz: fixed_t = 0;
pub static mut tmceilingz: fixed_t = 0;
pub static mut tmdropoffz: fixed_t = 0;

pub static mut ceilingline: *mut line_t = std::ptr::null_mut();

pub const MAXSPECIALCROSS: usize = 8;
pub static mut spechit: [*mut line_t; MAXSPECIALCROSS] = [std::ptr::null_mut(); MAXSPECIALCROSS];
pub static mut numspechit: i32 = 0;

pub unsafe fn PIT_StompThing(thing: *mut mobj_t) -> boolean {
    if (*thing).flags & MF_SHOOTABLE == 0 {
        return true_;
    }

    let blockdist = (*thing).radius + (*tmthing).radius;

    if ((*thing).x - tmx).wrapping_abs() >= blockdist
        || ((*thing).y - tmy).wrapping_abs() >= blockdist
    {
        return true_;
    }

    if thing == tmthing {
        return true_;
    }

    if (*tmthing).player.is_null() && gamemap != 30 {
        return false_;
    }

    P_DamageMobj(thing, tmthing, tmthing, 10000);

    true_
}

pub unsafe fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean {
    tmthing = thing;
    tmflags = (*thing).flags;

    tmx = x;
    tmy = y;

    tmbbox[BOXTOP] = y + (*tmthing).radius;
    tmbbox[BOXBOTTOM] = y - (*tmthing).radius;
    tmbbox[BOXRIGHT] = x + (*tmthing).radius;
    tmbbox[BOXLEFT] = x - (*tmthing).radius;

    let newsubsec = R_PointInSubsector(x, y);
    ceilingline = std::ptr::null_mut();

    tmfloorz = (*(*newsubsec).sector).floorheight;
    tmdropoffz = tmfloorz;
    tmceilingz = (*(*newsubsec).sector).ceilingheight;

    validcount += 1;
    numspechit = 0;

    let xl = (tmbbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
    let xh = (tmbbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
    let yl = (tmbbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
    let yh = (tmbbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;

    for bx in xl..=xh {
        for by in yl..=yh {
            if P_BlockThingsIterator(bx, by, PIT_StompThing) == 0 {
                return false_;
            }
        }
    }

    P_UnsetThingPosition(thing);

    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    (*thing).x = x;
    (*thing).y = y;

    P_SetThingPosition(thing);

    true_
}

pub unsafe fn PIT_CheckLine(ld: *mut line_t) -> boolean {
    if tmbbox[BOXRIGHT] <= (*ld).bbox[BOXLEFT]
        || tmbbox[BOXLEFT] >= (*ld).bbox[BOXRIGHT]
        || tmbbox[BOXTOP] <= (*ld).bbox[BOXBOTTOM]
        || tmbbox[BOXBOTTOM] >= (*ld).bbox[BOXTOP]
    {
        return true_;
    }

    if P_BoxOnLineSide(tmbbox.as_mut_ptr(), ld) != -1 {
        return true_;
    }

    if (*ld).backsector.is_null() {
        return false_;
    }

    if (*tmthing).flags & MF_MISSILE == 0 {
        if ((*ld).flags as i32) & ML_BLOCKING != 0 {
            return false_;
        }
        if (*tmthing).player.is_null() && (((*ld).flags as i32) & ML_BLOCKMONSTERS != 0) {
            return false_;
        }
    }

    P_LineOpening(ld);

    if opentop < tmceilingz {
        tmceilingz = opentop;
        ceilingline = ld;
    }

    if openbottom > tmfloorz {
        tmfloorz = openbottom;
    }

    if lowfloor < tmdropoffz {
        tmdropoffz = lowfloor;
    }

    if (*ld).special != 0 {
        spechit[numspechit as usize] = ld;
        numspechit += 1;
    }

    true_
}

pub unsafe fn PIT_CheckThing(thing: *mut mobj_t) -> boolean {
    if (*thing).flags & (MF_SOLID | MF_SPECIAL | MF_SHOOTABLE) == 0 {
        return true_;
    }

    let blockdist = (*thing).radius + (*tmthing).radius;

    if ((*thing).x - tmx).wrapping_abs() >= blockdist
        || ((*thing).y - tmy).wrapping_abs() >= blockdist
    {
        return true_;
    }

    if thing == tmthing {
        return true_;
    }

    if (*tmthing).flags & MF_SKULLFLY != 0 {
        let damage = ((P_Random() % 8) + 1) * (*(*tmthing).info).damage;
        P_DamageMobj(thing, tmthing, tmthing, damage);
        (*tmthing).flags &= !MF_SKULLFLY;
        (*tmthing).momx = 0;
        (*tmthing).momy = 0;
        (*tmthing).momz = 0;
        P_SetMobjState(tmthing, (*(*tmthing).info).spawnstate);
        return false_;
    }

    if (*tmthing).flags & MF_MISSILE != 0 {
        if (*tmthing).z > (*thing).z + (*thing).height {
            return true_;
        }
        if (*tmthing).z + (*tmthing).height < (*thing).z {
            return true_;
        }

        if !(*tmthing).target.is_null()
            && ((*(*tmthing).target).type_ == (*thing).type_
                || ((*(*tmthing).target).type_ == MT_KNIGHT && (*thing).type_ == MT_BRUISER)
                || ((*(*tmthing).target).type_ == MT_BRUISER && (*thing).type_ == MT_KNIGHT))
        {
            if thing == (*tmthing).target {
                return true_;
            }
            if (*thing).type_ != MT_PLAYER {
                return false_;
            }
        }

        if (*thing).flags & MF_SHOOTABLE == 0 {
            return ((*thing).flags & MF_SOLID == 0) as boolean;
        }

        let damage = ((P_Random() % 8) + 1) * (*(*tmthing).info).damage;
        P_DamageMobj(thing, tmthing, (*tmthing).target, damage);
        return false_;
    }

    if (*thing).flags & MF_SPECIAL != 0 {
        let solid = (*thing).flags & MF_SOLID;
        if tmflags & MF_PICKUP != 0 {
            P_TouchSpecialThing(thing, tmthing);
        }
        return (solid == 0) as boolean;
    }

    ((*thing).flags & MF_SOLID == 0) as boolean
}

pub unsafe fn P_CheckPosition(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean {
    tmthing = thing;
    tmflags = (*thing).flags;

    tmx = x;
    tmy = y;

    tmbbox[BOXTOP] = y + (*tmthing).radius;
    tmbbox[BOXBOTTOM] = y - (*tmthing).radius;
    tmbbox[BOXRIGHT] = x + (*tmthing).radius;
    tmbbox[BOXLEFT] = x - (*tmthing).radius;

    let newsubsec = R_PointInSubsector(x, y);
    ceilingline = std::ptr::null_mut();

    tmfloorz = (*(*newsubsec).sector).floorheight;
    tmdropoffz = tmfloorz;
    tmceilingz = (*(*newsubsec).sector).ceilingheight;

    validcount += 1;
    numspechit = 0;

    if tmflags & MF_NOCLIP != 0 {
        return true_;
    }

    let xl = (tmbbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
    let xh = (tmbbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
    let yl = (tmbbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
    let yh = (tmbbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;

    for bx in xl..=xh {
        for by in yl..=yh {
            if P_BlockThingsIterator(bx, by, PIT_CheckThing) == 0 {
                return false_;
            }
        }
    }

    let xl2 = (tmbbox[BOXLEFT] - bmaporgx) >> MAPBLOCKSHIFT;
    let xh2 = (tmbbox[BOXRIGHT] - bmaporgx) >> MAPBLOCKSHIFT;
    let yl2 = (tmbbox[BOXBOTTOM] - bmaporgy) >> MAPBLOCKSHIFT;
    let yh2 = (tmbbox[BOXTOP] - bmaporgy) >> MAPBLOCKSHIFT;

    for bx in xl2..=xh2 {
        for by in yl2..=yh2 {
            if P_BlockLinesIterator(bx, by, PIT_CheckLine) == 0 {
                return false_;
            }
        }
    }

    true_
}

pub unsafe fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean {
    floatok = false_;
    if P_CheckPosition(thing, x, y) == 0 {
        return false_;
    }

    if (*thing).flags & MF_NOCLIP == 0 {
        if tmceilingz - tmfloorz < (*thing).height {
            return false_;
        }

        floatok = true_;

        if ((*thing).flags & MF_TELEPORT == 0) && tmceilingz - (*thing).z < (*thing).height {
            return false_;
        }

        if ((*thing).flags & MF_TELEPORT == 0) && tmfloorz - (*thing).z > 24 * FRACUNIT {
            return false_;
        }

        if ((*thing).flags & (MF_DROPOFF | MF_FLOAT) == 0) && tmfloorz - tmdropoffz > 24 * FRACUNIT
        {
            return false_;
        }
    }

    P_UnsetThingPosition(thing);

    let oldx = (*thing).x;
    let oldy = (*thing).y;
    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;
    (*thing).x = x;
    (*thing).y = y;

    P_SetThingPosition(thing);

    if (*thing).flags & (MF_TELEPORT | MF_NOCLIP) == 0 {
        while numspechit > 0 {
            numspechit -= 1;
            let ld = spechit[numspechit as usize];
            let side = P_PointOnLineSide((*thing).x, (*thing).y, ld);
            let oldside = P_PointOnLineSide(oldx, oldy, ld);
            if side != oldside {
                if (*ld).special != 0 {
                    let linenum =
                        ((ld as usize - lines as usize) / std::mem::size_of::<line_t>()) as i32;
                    P_CrossSpecialLine(linenum, oldside, thing);
                }
            }
        }
    }

    true_
}

pub unsafe fn P_ThingHeightClip(thing: *mut mobj_t) -> boolean {
    let onfloor = (*thing).z == (*thing).floorz;

    P_CheckPosition(thing, (*thing).x, (*thing).y);

    (*thing).floorz = tmfloorz;
    (*thing).ceilingz = tmceilingz;

    if onfloor {
        (*thing).z = (*thing).floorz;
    } else {
        if (*thing).z + (*thing).height > (*thing).ceilingz {
            (*thing).z = (*thing).ceilingz - (*thing).height;
        }
    }

    if (*thing).ceilingz - (*thing).floorz < (*thing).height {
        return false_;
    }

    true_
}

pub static mut bestslidefrac: fixed_t = 0;
pub static mut secondslidefrac: fixed_t = 0;
pub static mut bestslideline: *mut line_t = std::ptr::null_mut();
pub static mut secondslideline: *mut line_t = std::ptr::null_mut();
pub static mut slidemo: *mut mobj_t = std::ptr::null_mut();
pub static mut tmxmove: fixed_t = 0;
pub static mut tmymove: fixed_t = 0;

pub unsafe fn P_HitSlideLine(ld: *mut line_t) {
    if (*ld).slopetype == ST_HORIZONTAL {
        tmymove = 0;
        return;
    }

    if (*ld).slopetype == ST_VERTICAL {
        tmxmove = 0;
        return;
    }

    let side = P_PointOnLineSide((*slidemo).x, (*slidemo).y, ld);
    let mut lineangle = R_PointToAngle2(0, 0, (*ld).dx, (*ld).dy);

    if side == 1 {
        lineangle = lineangle.wrapping_add(ANG180);
    }

    let moveangle = R_PointToAngle2(0, 0, tmxmove, tmymove);
    let mut deltaangle = moveangle.wrapping_sub(lineangle);

    if deltaangle > ANG180 {
        deltaangle = deltaangle.wrapping_add(ANG180);
    }

    let lineangle_idx = (lineangle >> ANGLETOFINESHIFT) as usize;
    let deltaangle_idx = (deltaangle >> ANGLETOFINESHIFT) as usize;

    let movelen = P_AproxDistance(tmxmove, tmymove);
    let newlen = FixedMul(movelen, finecosine(deltaangle_idx));

    tmxmove = FixedMul(newlen, finecosine(lineangle_idx));
    tmymove = FixedMul(newlen, finesine[lineangle_idx]);
}

pub unsafe fn PTR_SlideTraverse(in_: *mut intercept_t) -> boolean {
    if (*in_).isaline == 0 {
        I_Error("PTR_SlideTraverse: not a line?");
    }

    let li = (*in_).d.line;

    if ((*li).flags as i32) & ML_TWOSIDED == 0 {
        if P_PointOnLineSide((*slidemo).x, (*slidemo).y, li) != 0 {
            return true_;
        }
    } else {
        P_LineOpening(li);
        if openrange >= (*slidemo).height
            && opentop - (*slidemo).z >= (*slidemo).height
            && openbottom - (*slidemo).z <= 24 * FRACUNIT
        {
            return true_;
        }
    }

    if (*in_).frac < bestslidefrac {
        secondslidefrac = bestslidefrac;
        secondslideline = bestslideline;
        bestslidefrac = (*in_).frac;
        bestslideline = li;
    }

    false_
}

pub unsafe fn P_SlideMove(mo: *mut mobj_t) {
    slidemo = mo;
    let mut hitcount = 0;

    loop {
        hitcount += 1;
        if hitcount == 3 {
            if P_TryMove(mo, (*mo).x, (*mo).y + (*mo).momy) == 0 {
                P_TryMove(mo, (*mo).x + (*mo).momx, (*mo).y);
            }
            return;
        }

        let (leadx, trailx) = if (*mo).momx > 0 {
            ((*mo).x + (*mo).radius, (*mo).x - (*mo).radius)
        } else {
            ((*mo).x - (*mo).radius, (*mo).x + (*mo).radius)
        };

        let (leady, traily) = if (*mo).momy > 0 {
            ((*mo).y + (*mo).radius, (*mo).y - (*mo).radius)
        } else {
            ((*mo).y - (*mo).radius, (*mo).y + (*mo).radius)
        };

        bestslidefrac = FRACUNIT + 1;

        P_PathTraverse(
            leadx,
            leady,
            leadx + (*mo).momx,
            leady + (*mo).momy,
            PT_ADDLINES,
            PTR_SlideTraverse,
        );
        P_PathTraverse(
            trailx,
            leady,
            trailx + (*mo).momx,
            leady + (*mo).momy,
            PT_ADDLINES,
            PTR_SlideTraverse,
        );
        P_PathTraverse(
            leadx,
            traily,
            leadx + (*mo).momx,
            traily + (*mo).momy,
            PT_ADDLINES,
            PTR_SlideTraverse,
        );

        if bestslidefrac == FRACUNIT + 1 {
            if P_TryMove(mo, (*mo).x, (*mo).y + (*mo).momy) == 0 {
                P_TryMove(mo, (*mo).x + (*mo).momx, (*mo).y);
            }
            return;
        }

        bestslidefrac -= 0x800;
        if bestslidefrac > 0 {
            let newx = FixedMul((*mo).momx, bestslidefrac);
            let newy = FixedMul((*mo).momy, bestslidefrac);

            if P_TryMove(mo, (*mo).x + newx, (*mo).y + newy) == 0 {
                if P_TryMove(mo, (*mo).x, (*mo).y + (*mo).momy) == 0 {
                    P_TryMove(mo, (*mo).x + (*mo).momx, (*mo).y);
                }
                return;
            }
        }

        bestslidefrac = FRACUNIT - (bestslidefrac + 0x800);

        if bestslidefrac > FRACUNIT {
            bestslidefrac = FRACUNIT;
        }

        if bestslidefrac <= 0 {
            return;
        }

        tmxmove = FixedMul((*mo).momx, bestslidefrac);
        tmymove = FixedMul((*mo).momy, bestslidefrac);

        P_HitSlideLine(bestslideline);

        (*mo).momx = tmxmove;
        (*mo).momy = tmymove;

        if P_TryMove(mo, (*mo).x + tmxmove, (*mo).y + tmymove) != 0 {
            break;
        }
    }
}

pub static mut linetarget: *mut mobj_t = std::ptr::null_mut();
pub static mut shootthing: *mut mobj_t = std::ptr::null_mut();
pub static mut shootz: fixed_t = 0;
pub static mut la_damage: i32 = 0;
pub static mut attackrange: fixed_t = 0;
pub static mut aimslope: fixed_t = 0;

pub unsafe fn PTR_AimTraverse(in_: *mut intercept_t) -> boolean {
    if (*in_).isaline != 0 {
        let li = (*in_).d.line;

        if ((*li).flags as i32) & ML_TWOSIDED == 0 {
            return false_;
        }

        P_LineOpening(li);

        if openbottom >= opentop {
            return false_;
        }

        let dist = FixedMul(attackrange, (*in_).frac);

        if (*(*li).frontsector).floorheight != (*(*li).backsector).floorheight {
            let slope = FixedDiv(openbottom - shootz, dist);
            if slope > bottomslope {
                bottomslope = slope;
            }
        }

        if (*(*li).frontsector).ceilingheight != (*(*li).backsector).ceilingheight {
            let slope = FixedDiv(opentop - shootz, dist);
            if slope < topslope {
                topslope = slope;
            }
        }

        if topslope <= bottomslope {
            return false_;
        }

        return true_;
    }

    let th = (*in_).d.thing;
    if th == shootthing {
        return true_;
    }

    if (*th).flags & MF_SHOOTABLE == 0 {
        return true_;
    }

    let dist = FixedMul(attackrange, (*in_).frac);
    let mut thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);

    if thingtopslope < bottomslope {
        return true_;
    }

    let mut thingbottomslope = FixedDiv((*th).z - shootz, dist);

    if thingbottomslope > topslope {
        return true_;
    }

    if thingtopslope > topslope {
        thingtopslope = topslope;
    }

    if thingbottomslope < bottomslope {
        thingbottomslope = bottomslope;
    }

    aimslope = (thingtopslope + thingbottomslope) / 2;
    linetarget = th;

    false_
}

pub unsafe fn PTR_ShootTraverse(in_: *mut intercept_t) -> boolean {
    if (*in_).isaline != 0 {
        let li = (*in_).d.line;

        if (*li).special != 0 {
            P_ShootSpecialLine(shootthing, li);
        }

        let mut hitline = false;
        if ((*li).flags as i32) & ML_TWOSIDED == 0 {
            hitline = true;
        } else {
            P_LineOpening(li);
            let dist = FixedMul(attackrange, (*in_).frac);

            if (*(*li).frontsector).floorheight != (*(*li).backsector).floorheight {
                let slope = FixedDiv(openbottom - shootz, dist);
                if slope > aimslope {
                    hitline = true;
                }
            }

            if !hitline && (*(*li).frontsector).ceilingheight != (*(*li).backsector).ceilingheight {
                let slope = FixedDiv(opentop - shootz, dist);
                if slope < aimslope {
                    hitline = true;
                }
            }
        }

        if !hitline {
            return true_;
        }

        let frac = (*in_).frac - FixedDiv(4 * FRACUNIT, attackrange);
        let x = trace.x + FixedMul(trace.dx, frac);
        let y = trace.y + FixedMul(trace.dy, frac);
        let z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));

        if (*(*li).frontsector).ceilingpic == (skyflatnum as i16) {
            if z > (*(*li).frontsector).ceilingheight {
                return false_;
            }
            if !(*li).backsector.is_null()
                && (*(*li).backsector).ceilingpic == (skyflatnum as i16)
            {
                return false_;
            }
        }

        P_SpawnPuff(x, y, z);
        return false_;
    }

    let th = (*in_).d.thing;
    if th == shootthing {
        return true_;
    }

    if (*th).flags & MF_SHOOTABLE == 0 {
        return true_;
    }

    let dist = FixedMul(attackrange, (*in_).frac);
    let thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);

    if thingtopslope < aimslope {
        return true_;
    }

    let thingbottomslope = FixedDiv((*th).z - shootz, dist);

    if thingbottomslope > aimslope {
        return true_;
    }

    let frac = (*in_).frac - FixedDiv(10 * FRACUNIT, attackrange);
    let x = trace.x + FixedMul(trace.dx, frac);
    let y = trace.y + FixedMul(trace.dy, frac);
    let z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));

    if (*(*in_).d.thing).flags & MF_NOBLOOD != 0 {
        P_SpawnPuff(x, y, z);
    } else {
        P_SpawnBlood(x, y, z, la_damage);
    }

    if la_damage != 0 {
        P_DamageMobj(th, shootthing, shootthing, la_damage);
    }

    false_
}

pub unsafe fn P_AimLineAttack(t1: *mut mobj_t, angle: angle_t, distance: fixed_t) -> fixed_t {
    let ang_idx = (angle >> ANGLETOFINESHIFT) as usize;
    shootthing = t1;

    let x2 = (*t1).x + (distance >> FRACBITS) * finecosine(ang_idx);
    let y2 = (*t1).y + (distance >> FRACBITS) * finesine[ang_idx];
    shootz = (*t1).z + ((*t1).height >> 1) + 8 * FRACUNIT;

    topslope = 100 * FRACUNIT / 160;
    bottomslope = -100 * FRACUNIT / 160;

    attackrange = distance;
    linetarget = std::ptr::null_mut();

    P_PathTraverse(
        (*t1).x,
        (*t1).y,
        x2,
        y2,
        PT_ADDLINES | PT_ADDTHINGS,
        PTR_AimTraverse,
    );

    if !linetarget.is_null() {
        return aimslope;
    }

    0
}

pub unsafe fn P_LineAttack(
    t1: *mut mobj_t,
    angle: angle_t,
    distance: fixed_t,
    slope: fixed_t,
    damage: i32,
) {
    let ang_idx = (angle >> ANGLETOFINESHIFT) as usize;
    shootthing = t1;
    la_damage = damage;
    let x2 = (*t1).x + (distance >> FRACBITS) * finecosine(ang_idx);
    let y2 = (*t1).y + (distance >> FRACBITS) * finesine[ang_idx];
    shootz = (*t1).z + ((*t1).height >> 1) + 8 * FRACUNIT;
    attackrange = distance;
    aimslope = slope;

    P_PathTraverse(
        (*t1).x,
        (*t1).y,
        x2,
        y2,
        PT_ADDLINES | PT_ADDTHINGS,
        PTR_ShootTraverse,
    );
}

pub static mut usething: *mut mobj_t = std::ptr::null_mut();

pub unsafe fn PTR_UseTraverse(in_: *mut intercept_t) -> boolean {
    if (*(*in_).d.line).special == 0 {
        P_LineOpening((*in_).d.line);
        if openrange <= 0 {
            S_StartSound(usething as *mut std::ffi::c_void, sfx_noway);
            return false_;
        }
        return true_;
    }

    let mut side = 0;
    if P_PointOnLineSide((*usething).x, (*usething).y, (*in_).d.line) == 1 {
        side = 1;
    }

    P_UseSpecialLine(usething, (*in_).d.line, side);
    false_
}

pub unsafe fn P_UseLines(player: *mut player_t) {
    usething = (*player).mo;

    let angle = ((*(*player).mo).angle >> ANGLETOFINESHIFT) as usize;
    let x1 = (*(*player).mo).x;
    let y1 = (*(*player).mo).y;
    let x2 = x1 + (USERANGE >> FRACBITS) * finecosine(angle);
    let y2 = y1 + (USERANGE >> FRACBITS) * finesine[angle];

    P_PathTraverse(x1, y1, x2, y2, PT_ADDLINES, PTR_UseTraverse);
}

pub static mut bombsource: *mut mobj_t = std::ptr::null_mut();
pub static mut bombspot: *mut mobj_t = std::ptr::null_mut();
pub static mut bombdamage: i32 = 0;

pub unsafe fn PIT_RadiusAttack(thing: *mut mobj_t) -> boolean {
    if (*thing).flags & MF_SHOOTABLE == 0 {
        return true_;
    }

    if (*thing).type_ == MT_CYBORG || (*thing).type_ == MT_SPIDER {
        return true_;
    }

    let dx = ((*thing).x - (*bombspot).x).wrapping_abs();
    let dy = ((*thing).y - (*bombspot).y).wrapping_abs();

    let mut dist = if dx > dy { dx } else { dy };
    dist = (dist - (*thing).radius) >> FRACBITS;

    if dist < 0 {
        dist = 0;
    }

    if dist >= bombdamage {
        return true_;
    }

    if P_CheckSight(thing, bombspot) != 0 {
        P_DamageMobj(thing, bombspot, bombsource, bombdamage - dist);
    }

    true_
}

pub unsafe fn P_RadiusAttack(spot: *mut mobj_t, source: *mut mobj_t, damage: i32) {
    let dist = (damage + (MAXRADIUS >> FRACBITS)) << FRACBITS;
    let yh = ((*spot).y + dist - bmaporgy) >> MAPBLOCKSHIFT;
    let yl = ((*spot).y - dist - bmaporgy) >> MAPBLOCKSHIFT;
    let xh = ((*spot).x + dist - bmaporgx) >> MAPBLOCKSHIFT;
    let xl = ((*spot).x - dist - bmaporgx) >> MAPBLOCKSHIFT;
    bombspot = spot;
    bombsource = source;
    bombdamage = damage;

    for y in yl..=yh {
        for x in xl..=xh {
            P_BlockThingsIterator(x, y, PIT_RadiusAttack);
        }
    }
}

pub static mut crushchange: boolean = 0;
pub static mut nofit: boolean = 0;

pub unsafe fn PIT_ChangeSector(thing: *mut mobj_t) -> boolean {
    if P_ThingHeightClip(thing) != 0 {
        return true_;
    }

    if (*thing).health <= 0 {
        P_SetMobjState(thing, S_GIBS);
        (*thing).flags &= !MF_SOLID;
        (*thing).height = 0;
        (*thing).radius = 0;
        return true_;
    }

    if (*thing).flags & MF_DROPPED != 0 {
        P_RemoveMobj(thing);
        return true_;
    }

    if (*thing).flags & MF_SHOOTABLE == 0 {
        return true_;
    }

    nofit = true_;

    if crushchange != 0 && (leveltime & 3) == 0 {
        P_DamageMobj(thing, std::ptr::null_mut(), std::ptr::null_mut(), 10);

        let mo = P_SpawnMobj(
            (*thing).x,
            (*thing).y,
            (*thing).z + (*thing).height / 2,
            MT_BLOOD,
        );

        let r1 = P_Random();
        let r2 = P_Random();
        (*mo).momx = (r1 - r2) << 12;
        let r3 = P_Random();
        let r4 = P_Random();
        (*mo).momy = (r3 - r4) << 12;
    }

    true_
}

pub unsafe fn P_ChangeSector(sector: *mut sector_t, crunch: boolean) -> boolean {
    nofit = false_;
    crushchange = crunch;

    for x in (*sector).blockbox[BOXLEFT]..=(*sector).blockbox[BOXRIGHT] {
        for y in (*sector).blockbox[BOXBOTTOM]..=(*sector).blockbox[BOXTOP] {
            P_BlockThingsIterator(x, y, PIT_ChangeSector);
        }
    }

    nofit
}
