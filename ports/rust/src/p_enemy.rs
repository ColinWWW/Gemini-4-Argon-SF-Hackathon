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
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_doors::*;
use crate::p_floor::*;
use crate::p_inter::*;
use crate::p_local::*;
use crate::p_map::*;
use crate::p_maputl::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_setup::*;
use crate::p_sight::*;
use crate::p_spec::*;
use crate::p_switch::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;

pub type dirtype_t = i32;
pub const DI_EAST: dirtype_t = 0;
pub const DI_NORTHEAST: dirtype_t = 1;
pub const DI_NORTH: dirtype_t = 2;
pub const DI_NORTHWEST: dirtype_t = 3;
pub const DI_WEST: dirtype_t = 4;
pub const DI_SOUTHWEST: dirtype_t = 5;
pub const DI_SOUTH: dirtype_t = 6;
pub const DI_SOUTHEAST: dirtype_t = 7;
pub const DI_NODIR: dirtype_t = 8;
pub const NUMDIRS: usize = 9;

pub static mut opposite: [dirtype_t; 9] = [
    DI_WEST,
    DI_SOUTHWEST,
    DI_SOUTH,
    DI_SOUTHEAST,
    DI_EAST,
    DI_NORTHEAST,
    DI_NORTH,
    DI_NORTHWEST,
    DI_NODIR,
];

pub static mut diags: [dirtype_t; 4] = [DI_NORTHWEST, DI_NORTHEAST, DI_SOUTHWEST, DI_SOUTHEAST];

pub static mut soundtarget: *mut mobj_t = std::ptr::null_mut();

pub unsafe fn P_RecursiveSound(sec: *mut sector_t, soundblocks: i32) {
    if (*sec).validcount == validcount && (*sec).soundtraversed <= soundblocks + 1 {
        return;
    }

    (*sec).validcount = validcount;
    (*sec).soundtraversed = soundblocks + 1;
    (*sec).soundtarget = soundtarget;

    for i in 0..((*sec).linecount as usize) {
        let check = *(*sec).lines.add(i);
        if ((*check).flags as i32) & ML_TWOSIDED == 0 {
            continue;
        }

        P_LineOpening(check);

        if openrange <= 0 {
            continue;
        }

        let other = if (*sides.add((*check).sidenum[0] as usize)).sector == sec {
            (*sides.add((*check).sidenum[1] as usize)).sector
        } else {
            (*sides.add((*check).sidenum[0] as usize)).sector
        };

        if ((*check).flags as i32) & ML_SOUNDBLOCK != 0 {
            if soundblocks == 0 {
                P_RecursiveSound(other, 1);
            }
        } else {
            P_RecursiveSound(other, soundblocks);
        }
    }
}

pub unsafe fn P_NoiseAlert(target: *mut mobj_t, emmiter: *mut mobj_t) {
    soundtarget = target;
    validcount += 1;
    P_RecursiveSound((*(*emmiter).subsector).sector, 0);
}

pub unsafe fn P_CheckMeleeRange(actor: *mut mobj_t) -> boolean {
    if (*actor).target.is_null() {
        return false_;
    }

    let pl = (*actor).target;
    let dist = P_AproxDistance((*pl).x - (*actor).x, (*pl).y - (*actor).y);

    if dist >= MELEERANGE - 20 * FRACUNIT + (*(*pl).info).radius {
        return false_;
    }

    if P_CheckSight(actor, (*actor).target) == 0 {
        return false_;
    }

    true_
}

pub unsafe fn P_CheckMissileRange(actor: *mut mobj_t) -> boolean {
    if P_CheckSight(actor, (*actor).target) == 0 {
        return false_;
    }

    if (*actor).flags & MF_JUSTHIT != 0 {
        (*actor).flags &= !MF_JUSTHIT;
        return true_;
    }

    if (*actor).reactiontime != 0 {
        return false_;
    }

    let mut dist = P_AproxDistance(
        (*actor).x - (*(*actor).target).x,
        (*actor).y - (*(*actor).target).y,
    ) - 64 * FRACUNIT;

    if (*(*actor).info).meleestate == 0 {
        dist -= 128 * FRACUNIT;
    }

    dist >>= 16;

    if (*actor).type_ == MT_VILE && dist > 14 * 64 {
        return false_;
    }

    if (*actor).type_ == MT_UNDEAD {
        if dist < 196 {
            return false_;
        }
        dist >>= 1;
    }

    if (*actor).type_ == MT_CYBORG || (*actor).type_ == MT_SPIDER || (*actor).type_ == MT_SKULL {
        dist >>= 1;
    }

    if dist > 200 {
        dist = 200;
    }

    if (*actor).type_ == MT_CYBORG && dist > 160 {
        dist = 160;
    }

    if P_Random() < dist {
        return false_;
    }

    true_
}

pub static mut xspeed: [fixed_t; 8] = [
    FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000, 0, 47000,
];
pub static mut yspeed: [fixed_t; 8] = [
    0, 47000, FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000,
];

pub unsafe fn P_Move(actor: *mut mobj_t) -> boolean {
    if (*actor).movedir == DI_NODIR {
        return false_;
    }

    if ((*actor).movedir as u32) >= 8 {
        I_Error("Weird actor->movedir!");
    }

    let tryx = (*actor).x + (*(*actor).info).speed * xspeed[(*actor).movedir as usize];
    let tryy = (*actor).y + (*(*actor).info).speed * yspeed[(*actor).movedir as usize];

    let try_ok = P_TryMove(actor, tryx, tryy);

    if try_ok == 0 {
        if ((*actor).flags & MF_FLOAT != 0) && floatok != 0 {
            if (*actor).z < tmfloorz {
                (*actor).z += FLOATSPEED;
            } else {
                (*actor).z -= FLOATSPEED;
            }
            (*actor).flags |= MF_INFLOAT;
            return true_;
        }

        if numspechit == 0 {
            return false_;
        }

        (*actor).movedir = DI_NODIR;
        let mut good = false_;
        while numspechit > 0 {
            numspechit -= 1;
            let ld = spechit[numspechit as usize];
            if P_UseSpecialLine(actor, ld, 0) != 0 {
                good = true_;
            }
        }
        return good;
    } else {
        (*actor).flags &= !MF_INFLOAT;
    }

    if (*actor).flags & MF_FLOAT == 0 {
        (*actor).z = (*actor).floorz;
    }
    true_
}

pub unsafe fn P_TryWalk(actor: *mut mobj_t) -> boolean {
    if P_Move(actor) == 0 {
        return false_;
    }
    (*actor).movecount = P_Random() & 15;
    true_
}

pub unsafe fn P_NewChaseDir(actor: *mut mobj_t) {
    let mut d = [0; 3];

    if (*actor).target.is_null() {
        I_Error("P_NewChaseDir: called with no target");
    }

    let olddir = (*actor).movedir;
    let turnaround = opposite[olddir as usize];

    let deltax = (*(*actor).target).x - (*actor).x;
    let deltay = (*(*actor).target).y - (*actor).y;

    if deltax > 10 * FRACUNIT {
        d[1] = DI_EAST;
    } else if deltax < -10 * FRACUNIT {
        d[1] = DI_WEST;
    } else {
        d[1] = DI_NODIR;
    }

    if deltay < -10 * FRACUNIT {
        d[2] = DI_SOUTH;
    } else if deltay > 10 * FRACUNIT {
        d[2] = DI_NORTH;
    } else {
        d[2] = DI_NODIR;
    }

    if d[1] != DI_NODIR && d[2] != DI_NODIR {
        (*actor).movedir = diags[(((deltay < 0) as usize) << 1) + ((deltax > 0) as usize)];
        if (*actor).movedir != turnaround && P_TryWalk(actor) != 0 {
            return;
        }
    }

    if P_Random() > 200 || deltay.wrapping_abs() > deltax.wrapping_abs() {
        let tdir = d[1];
        d[1] = d[2];
        d[2] = tdir;
    }

    if d[1] == turnaround {
        d[1] = DI_NODIR;
    }
    if d[2] == turnaround {
        d[2] = DI_NODIR;
    }

    if d[1] != DI_NODIR {
        (*actor).movedir = d[1];
        if P_TryWalk(actor) != 0 {
            return;
        }
    }

    if d[2] != DI_NODIR {
        (*actor).movedir = d[2];
        if P_TryWalk(actor) != 0 {
            return;
        }
    }

    if olddir != DI_NODIR {
        (*actor).movedir = olddir;
        if P_TryWalk(actor) != 0 {
            return;
        }
    }

    if (P_Random() & 1) != 0 {
        for tdir in DI_EAST..=DI_SOUTHEAST {
            if tdir != turnaround {
                (*actor).movedir = tdir;
                if P_TryWalk(actor) != 0 {
                    return;
                }
            }
        }
    } else {
        let mut tdir = DI_SOUTHEAST;
        while tdir != (DI_EAST - 1) {
            if tdir != turnaround {
                (*actor).movedir = tdir;
                if P_TryWalk(actor) != 0 {
                    return;
                }
            }
            tdir -= 1;
        }
    }

    if turnaround != DI_NODIR {
        (*actor).movedir = turnaround;
        if P_TryWalk(actor) != 0 {
            return;
        }
    }

    (*actor).movedir = DI_NODIR;
}

pub unsafe fn P_LookForPlayers(actor: *mut mobj_t, allaround: boolean) -> boolean {
    let mut c = 0;
    let stop = ((*actor).lastlook - 1) & 3;

    loop {
        if playeringame[(*actor).lastlook as usize] == 0 {
            (*actor).lastlook = ((*actor).lastlook + 1) & 3;
            continue;
        }

        let old_c = c;
        c += 1;
        if old_c == 2 || (*actor).lastlook == stop {
            return false_;
        }

        let player = &mut players[(*actor).lastlook as usize];

        if player.health <= 0 {
            (*actor).lastlook = ((*actor).lastlook + 1) & 3;
            continue;
        }

        if P_CheckSight(actor, player.mo) == 0 {
            (*actor).lastlook = ((*actor).lastlook + 1) & 3;
            continue;
        }

        if allaround == 0 {
            let an = R_PointToAngle2((*actor).x, (*actor).y, (*player.mo).x, (*player.mo).y)
                .wrapping_sub((*actor).angle);

            if an > ANG90 && an < ANG270 {
                let dist =
                    P_AproxDistance((*player.mo).x - (*actor).x, (*player.mo).y - (*actor).y);
                if dist > MELEERANGE {
                    (*actor).lastlook = ((*actor).lastlook + 1) & 3;
                    continue;
                }
            }
        }

        (*actor).target = player.mo;
        return true_;
    }
}

pub unsafe fn A_KeenDie(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    let mut junk: line_t = std::mem::zeroed();

    A_Fall(mo_ptr);

    let mut th = thinkercap.next;
    while th != &mut thinkercap {
        if (*th).function != actionf_t::acp1(P_MobjThinker) {
            th = (*th).next;
            continue;
        }

        let mo2 = th as *mut mobj_t;
        if mo2 != mo && (*mo2).type_ == (*mo).type_ && (*mo2).health > 0 {
            return;
        }
        th = (*th).next;
    }

    junk.tag = 666;
    EV_DoDoor(&mut junk, open);
}

pub unsafe fn A_Look(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    (*actor).threshold = 0;
    let targ = (*(*(*actor).subsector).sector).soundtarget;

    let mut seen = false;
    if !targ.is_null() && ((*targ).flags & MF_SHOOTABLE != 0) {
        (*actor).target = targ;
        if (*actor).flags & MF_AMBUSH != 0 {
            if P_CheckSight(actor, (*actor).target) != 0 {
                seen = true;
            }
        } else {
            seen = true;
        }
    }

    if !seen {
        if P_LookForPlayers(actor, false_) == 0 {
            return;
        }
    }

    if (*(*actor).info).seesound != 0 {
        let sound = match (*(*actor).info).seesound {
            sfx_posit1 | sfx_posit2 | sfx_posit3 => sfx_posit1 + (P_Random() % 3),
            sfx_bgsit1 | sfx_bgsit2 => sfx_bgsit1 + (P_Random() % 2),
            s => s,
        };

        if (*actor).type_ == MT_SPIDER || (*actor).type_ == MT_CYBORG {
            S_StartSound(std::ptr::null_mut(), sound);
        } else {
            S_StartSound(actor as *mut c_void, sound);
        }
    }

    P_SetMobjState(actor, (*(*actor).info).seestate);
}

pub unsafe fn A_Chase(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;

    if (*actor).reactiontime != 0 {
        (*actor).reactiontime -= 1;
    }

    if (*actor).threshold != 0 {
        if (*actor).target.is_null() || (*(*actor).target).health <= 0 {
            (*actor).threshold = 0;
        } else {
            (*actor).threshold -= 1;
        }
    }

    if (*actor).movedir < 8 {
        (*actor).angle &= 7 << 29;
        let delta = ((*actor).angle as i32).wrapping_sub((*actor).movedir << 29);

        if delta > 0 {
            (*actor).angle = (*actor).angle.wrapping_sub(ANG90 / 2);
        } else if delta < 0 {
            (*actor).angle = (*actor).angle.wrapping_add(ANG90 / 2);
        }
    }

    if (*actor).target.is_null() || ((*(*actor).target).flags & MF_SHOOTABLE == 0) {
        if P_LookForPlayers(actor, true_) != 0 {
            return;
        }
        P_SetMobjState(actor, (*(*actor).info).spawnstate);
        return;
    }

    if (*actor).flags & MF_JUSTATTACKED != 0 {
        (*actor).flags &= !MF_JUSTATTACKED;
        if gameskill != sk_nightmare && fastparm == 0 {
            P_NewChaseDir(actor);
        }
        return;
    }

    if (*(*actor).info).meleestate != 0 && P_CheckMeleeRange(actor) != 0 {
        if (*(*actor).info).attacksound != 0 {
            S_StartSound(actor as *mut c_void, (*(*actor).info).attacksound);
        }
        P_SetMobjState(actor, (*(*actor).info).meleestate);
        return;
    }

    if (*(*actor).info).missilestate != 0 {
        if !(gameskill < sk_nightmare && fastparm == 0 && (*actor).movecount != 0) {
            if P_CheckMissileRange(actor) != 0 {
                P_SetMobjState(actor, (*(*actor).info).missilestate);
                (*actor).flags |= MF_JUSTATTACKED;
                return;
            }
        }
    }

    if netgame != 0 && (*actor).threshold == 0 && P_CheckSight(actor, (*actor).target) == 0 {
        if P_LookForPlayers(actor, true_) != 0 {
            return;
        }
    }

    (*actor).movecount -= 1;
    if (*actor).movecount < 0 || P_Move(actor) == 0 {
        P_NewChaseDir(actor);
    }

    if (*(*actor).info).activesound != 0 && P_Random() < 3 {
        S_StartSound(actor as *mut c_void, (*(*actor).info).activesound);
    }
}

pub unsafe fn A_FaceTarget(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    (*actor).flags &= !MF_AMBUSH;

    (*actor).angle = R_PointToAngle2(
        (*actor).x,
        (*actor).y,
        (*(*actor).target).x,
        (*(*actor).target).y,
    );

    if (*(*actor).target).flags & MF_SHADOW != 0 {
        let r1 = P_Random();
        let r2 = P_Random();
        (*actor).angle = (*actor).angle.wrapping_add(((r1 - r2) << 21) as u32);
    }
}

pub unsafe fn A_PosAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    let mut angle = (*actor).angle;
    let slope = P_AimLineAttack(actor, angle, MISSILERANGE);

    S_StartSound(actor as *mut c_void, sfx_pistol);
    let r1 = P_Random();
    let r2 = P_Random();
    angle = angle.wrapping_add(((r1 - r2) << 20) as u32);
    let damage = ((P_Random() % 5) + 1) * 3;
    P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
}

pub unsafe fn A_SPosAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    S_StartSound(actor as *mut c_void, sfx_shotgn);
    A_FaceTarget(actor_ptr);
    let bangle = (*actor).angle;
    let slope = P_AimLineAttack(actor, bangle, MISSILERANGE);

    for _ in 0..3 {
        let r1 = P_Random();
        let r2 = P_Random();
        let angle = bangle.wrapping_add(((r1 - r2) << 20) as u32);
        let damage = ((P_Random() % 5) + 1) * 3;
        P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
    }
}

pub unsafe fn A_CPosAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    S_StartSound(actor as *mut c_void, sfx_shotgn);
    A_FaceTarget(actor_ptr);
    let bangle = (*actor).angle;
    let slope = P_AimLineAttack(actor, bangle, MISSILERANGE);

    let r1 = P_Random();
    let r2 = P_Random();
    let angle = bangle.wrapping_add(((r1 - r2) << 20) as u32);
    let damage = ((P_Random() % 5) + 1) * 3;
    P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
}

pub unsafe fn A_CPosRefire(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    A_FaceTarget(actor_ptr);

    if P_Random() < 40 {
        return;
    }

    if (*actor).target.is_null()
        || (*(*actor).target).health <= 0
        || P_CheckSight(actor, (*actor).target) == 0
    {
        P_SetMobjState(actor, (*(*actor).info).seestate);
    }
}

pub unsafe fn A_SpidRefire(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    A_FaceTarget(actor_ptr);

    if P_Random() < 10 {
        return;
    }

    if (*actor).target.is_null()
        || (*(*actor).target).health <= 0
        || P_CheckSight(actor, (*actor).target) == 0
    {
        P_SetMobjState(actor, (*(*actor).info).seestate);
    }
}

pub unsafe fn A_BspiAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    P_SpawnMissile(actor, (*actor).target, MT_ARACHPLAZ);
}

pub unsafe fn A_TroopAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    if P_CheckMeleeRange(actor) != 0 {
        S_StartSound(actor as *mut c_void, sfx_claw);
        let damage = (P_Random() % 8 + 1) * 3;
        P_DamageMobj((*actor).target, actor, actor, damage);
        return;
    }

    P_SpawnMissile(actor, (*actor).target, MT_TROOPSHOT);
}

pub unsafe fn A_SargAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    if P_CheckMeleeRange(actor) != 0 {
        let damage = ((P_Random() % 10) + 1) * 4;
        P_DamageMobj((*actor).target, actor, actor, damage);
    }
}

pub unsafe fn A_HeadAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    if P_CheckMeleeRange(actor) != 0 {
        let damage = (P_Random() % 6 + 1) * 10;
        P_DamageMobj((*actor).target, actor, actor, damage);
        return;
    }

    P_SpawnMissile(actor, (*actor).target, MT_HEADSHOT);
}

pub unsafe fn A_CyberAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    P_SpawnMissile(actor, (*actor).target, MT_ROCKET);
}

pub unsafe fn A_BruisAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    if P_CheckMeleeRange(actor) != 0 {
        S_StartSound(actor as *mut c_void, sfx_claw);
        let damage = (P_Random() % 8 + 1) * 10;
        P_DamageMobj((*actor).target, actor, actor, damage);
        return;
    }

    P_SpawnMissile(actor, (*actor).target, MT_BRUISERSHOT);
}

pub unsafe fn A_SkelMissile(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    (*actor).z += 16 * FRACUNIT;
    let mo = P_SpawnMissile(actor, (*actor).target, MT_TRACER);
    (*actor).z -= 16 * FRACUNIT;

    (*mo).x += (*mo).momx;
    (*mo).y += (*mo).momy;
    (*mo).tracer = (*actor).target;
}

pub const TRACEANGLE: u32 = 0xc000000;

pub unsafe fn A_Tracer(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (gametic & 3) != 0 {
        return;
    }

    P_SpawnPuff((*actor).x, (*actor).y, (*actor).z);

    let th = P_SpawnMobj(
        (*actor).x - (*actor).momx,
        (*actor).y - (*actor).momy,
        (*actor).z,
        MT_SMOKE,
    );

    (*th).momz = FRACUNIT;
    (*th).tics -= P_Random() & 3;
    if (*th).tics < 1 {
        (*th).tics = 1;
    }

    let dest = (*actor).tracer;
    if dest.is_null() || (*dest).health <= 0 {
        return;
    }

    let exact = R_PointToAngle2((*actor).x, (*actor).y, (*dest).x, (*dest).y);

    if exact != (*actor).angle {
        if exact.wrapping_sub((*actor).angle) > 0x80000000 {
            (*actor).angle = (*actor).angle.wrapping_sub(TRACEANGLE);
            if exact.wrapping_sub((*actor).angle) < 0x80000000 {
                (*actor).angle = exact;
            }
        } else {
            (*actor).angle = (*actor).angle.wrapping_add(TRACEANGLE);
            if exact.wrapping_sub((*actor).angle) > 0x80000000 {
                (*actor).angle = exact;
            }
        }
    }

    let exact_idx = ((*actor).angle >> ANGLETOFINESHIFT) as usize;
    (*actor).momx = FixedMul((*(*actor).info).speed, finecosine(exact_idx));
    (*actor).momy = FixedMul((*(*actor).info).speed, finesine[exact_idx]);

    let mut dist = P_AproxDistance((*dest).x - (*actor).x, (*dest).y - (*actor).y);
    dist /= (*(*actor).info).speed;

    if dist < 1 {
        dist = 1;
    }
    let slope = ((*dest).z + 40 * FRACUNIT - (*actor).z) / dist;

    if slope < (*actor).momz {
        (*actor).momz -= FRACUNIT / 8;
    } else {
        (*actor).momz += FRACUNIT / 8;
    }
}

pub unsafe fn A_SkelWhoosh(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }
    A_FaceTarget(actor_ptr);
    S_StartSound(actor as *mut c_void, sfx_skeswg);
}

pub unsafe fn A_SkelFist(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);

    if P_CheckMeleeRange(actor) != 0 {
        let damage = ((P_Random() % 10) + 1) * 6;
        S_StartSound(actor as *mut c_void, sfx_skepch);
        P_DamageMobj((*actor).target, actor, actor, damage);
    }
}

pub static mut corpsehit: *mut mobj_t = std::ptr::null_mut();
pub static mut vileobj: *mut mobj_t = std::ptr::null_mut();
pub static mut viletryx: fixed_t = 0;
pub static mut viletryy: fixed_t = 0;

pub unsafe fn PIT_VileCheck(thing: *mut mobj_t) -> boolean {
    if (*thing).flags & MF_CORPSE == 0 {
        return true_;
    }
    if (*thing).tics != -1 {
        return true_;
    }
    if (*(*thing).info).raisestate == S_NULL {
        return true_;
    }

    let maxdist = (*(*thing).info).radius + mobjinfo[MT_VILE as usize].radius;

    if ((*thing).x - viletryx).wrapping_abs() > maxdist
        || ((*thing).y - viletryy).wrapping_abs() > maxdist
    {
        return true_;
    }

    corpsehit = thing;
    (*corpsehit).momx = 0;
    (*corpsehit).momy = 0;
    (*corpsehit).height <<= 2;
    let check = P_CheckPosition(corpsehit, (*corpsehit).x, (*corpsehit).y);
    (*corpsehit).height >>= 2;

    if check == 0 {
        return true_;
    }

    false_
}

pub unsafe fn A_VileChase(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).movedir != DI_NODIR {
        viletryx = (*actor).x + (*(*actor).info).speed * xspeed[(*actor).movedir as usize];
        viletryy = (*actor).y + (*(*actor).info).speed * yspeed[(*actor).movedir as usize];

        let xl = (viletryx - bmaporgx - MAXRADIUS * 2) >> MAPBLOCKSHIFT;
        let xh = (viletryx - bmaporgx + MAXRADIUS * 2) >> MAPBLOCKSHIFT;
        let yl = (viletryy - bmaporgy - MAXRADIUS * 2) >> MAPBLOCKSHIFT;
        let yh = (viletryy - bmaporgy + MAXRADIUS * 2) >> MAPBLOCKSHIFT;

        vileobj = actor;
        for bx in xl..=xh {
            for by in yl..=yh {
                if P_BlockThingsIterator(bx, by, PIT_VileCheck) == 0 {
                    let temp = (*actor).target;
                    (*actor).target = corpsehit;
                    A_FaceTarget(actor_ptr);
                    (*actor).target = temp;

                    P_SetMobjState(actor, S_VILE_HEAL1);
                    S_StartSound(corpsehit as *mut c_void, sfx_slop);
                    let info = (*corpsehit).info;

                    P_SetMobjState(corpsehit, (*info).raisestate);
                    (*corpsehit).height <<= 2;
                    (*corpsehit).flags = (*info).flags;
                    (*corpsehit).health = (*info).spawnhealth;
                    (*corpsehit).target = std::ptr::null_mut();

                    return;
                }
            }
        }
    }

    A_Chase(actor_ptr);
}

pub unsafe fn A_VileStart(actor_ptr: *mut c_void) {
    S_StartSound(actor_ptr, sfx_vilatk);
}

pub unsafe fn A_StartFire(actor_ptr: *mut c_void) {
    S_StartSound(actor_ptr, sfx_flamst);
    A_Fire(actor_ptr);
}

pub unsafe fn A_FireCrackle(actor_ptr: *mut c_void) {
    S_StartSound(actor_ptr, sfx_flame);
    A_Fire(actor_ptr);
}

pub unsafe fn A_Fire(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    let dest = (*actor).tracer;
    if dest.is_null() {
        return;
    }

    if P_CheckSight((*actor).target, dest) == 0 {
        return;
    }

    let an = ((*dest).angle >> ANGLETOFINESHIFT) as usize;

    P_UnsetThingPosition(actor);
    (*actor).x = (*dest).x + FixedMul(24 * FRACUNIT, finecosine(an));
    (*actor).y = (*dest).y + FixedMul(24 * FRACUNIT, finesine[an]);
    (*actor).z = (*dest).z;
    P_SetThingPosition(actor);
}

pub unsafe fn A_VileTarget(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);

    let fog = P_SpawnMobj(
        (*(*actor).target).x,
        (*(*actor).target).x,
        (*(*actor).target).z,
        MT_FIRE,
    );

    (*actor).tracer = fog;
    (*fog).target = actor;
    (*fog).tracer = (*actor).target;
    A_Fire(fog as *mut c_void);
}

pub unsafe fn A_VileAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);

    if P_CheckSight(actor, (*actor).target) == 0 {
        return;
    }

    S_StartSound(actor_ptr, sfx_barexp);
    P_DamageMobj((*actor).target, actor, actor, 20);
    (*(*actor).target).momz = 1000 * FRACUNIT / (*(*(*actor).target).info).mass;

    let an = ((*actor).angle >> ANGLETOFINESHIFT) as usize;

    let fire = (*actor).tracer;
    if fire.is_null() {
        return;
    }

    (*fire).x = (*(*actor).target).x - FixedMul(24 * FRACUNIT, finecosine(an));
    (*fire).y = (*(*actor).target).y - FixedMul(24 * FRACUNIT, finesine[an]);
    P_RadiusAttack(fire, actor, 70);
}

pub const FATSPREAD: u32 = ANG90 / 8;

pub unsafe fn A_FatRaise(actor_ptr: *mut c_void) {
    A_FaceTarget(actor_ptr);
    S_StartSound(actor_ptr, sfx_manatk);
}

pub unsafe fn A_FatAttack1(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    A_FaceTarget(actor_ptr);
    (*actor).angle = (*actor).angle.wrapping_add(FATSPREAD);
    P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);

    let mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
    (*mo).angle = (*mo).angle.wrapping_add(FATSPREAD);
    let an = ((*mo).angle >> ANGLETOFINESHIFT) as usize;
    (*mo).momx = FixedMul((*(*mo).info).speed, finecosine(an));
    (*mo).momy = FixedMul((*(*mo).info).speed, finesine[an]);
}

pub unsafe fn A_FatAttack2(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    A_FaceTarget(actor_ptr);
    (*actor).angle = (*actor).angle.wrapping_sub(FATSPREAD);
    P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);

    let mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
    (*mo).angle = (*mo).angle.wrapping_sub(FATSPREAD * 2);
    let an = ((*mo).angle >> ANGLETOFINESHIFT) as usize;
    (*mo).momx = FixedMul((*(*mo).info).speed, finecosine(an));
    (*mo).momy = FixedMul((*(*mo).info).speed, finesine[an]);
}

pub unsafe fn A_FatAttack3(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    A_FaceTarget(actor_ptr);

    let mo = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
    (*mo).angle = (*mo).angle.wrapping_sub(FATSPREAD / 2);
    let an = ((*mo).angle >> ANGLETOFINESHIFT) as usize;
    (*mo).momx = FixedMul((*(*mo).info).speed, finecosine(an));
    (*mo).momy = FixedMul((*(*mo).info).speed, finesine[an]);

    let mo2 = P_SpawnMissile(actor, (*actor).target, MT_FATSHOT);
    (*mo2).angle = (*mo2).angle.wrapping_add(FATSPREAD / 2);
    let an2 = ((*mo2).angle >> ANGLETOFINESHIFT) as usize;
    (*mo2).momx = FixedMul((*(*mo2).info).speed, finecosine(an2));
    (*mo2).momy = FixedMul((*(*mo2).info).speed, finesine[an2]);
}

pub const SKULLSPEED: fixed_t = 20 * FRACUNIT;

pub unsafe fn A_SkullAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    let dest = (*actor).target;
    (*actor).flags |= MF_SKULLFLY;

    S_StartSound(actor_ptr, (*(*actor).info).attacksound);
    A_FaceTarget(actor_ptr);
    let an = ((*actor).angle >> ANGLETOFINESHIFT) as usize;
    (*actor).momx = FixedMul(SKULLSPEED, finecosine(an));
    (*actor).momy = FixedMul(SKULLSPEED, finesine[an]);
    let mut dist = P_AproxDistance((*dest).x - (*actor).x, (*dest).y - (*actor).y);
    dist /= SKULLSPEED;

    if dist < 1 {
        dist = 1;
    }
    (*actor).momz = ((*dest).z + ((*dest).height >> 1) - (*actor).z) / dist;
}

pub unsafe fn A_PainShootSkull(actor: *mut mobj_t, angle: angle_t) {
    let mut count = 0;
    let mut currentthinker = thinkercap.next;
    while currentthinker != &mut thinkercap {
        if (*currentthinker).function == actionf_t::acp1(P_MobjThinker)
            && (*(currentthinker as *mut mobj_t)).type_ == MT_SKULL
        {
            count += 1;
        }
        currentthinker = (*currentthinker).next;
    }

    if count > 20 {
        return;
    }

    let an = (angle >> ANGLETOFINESHIFT) as usize;
    let prestep =
        4 * FRACUNIT + 3 * ((*(*actor).info).radius + mobjinfo[MT_SKULL as usize].radius) / 2;

    let x = (*actor).x + FixedMul(prestep, finecosine(an));
    let y = (*actor).y + FixedMul(prestep, finesine[an]);
    let z = (*actor).z + 8 * FRACUNIT;

    let newmobj = P_SpawnMobj(x, y, z, MT_SKULL);

    if P_TryMove(newmobj, (*newmobj).x, (*newmobj).y) == 0 {
        P_DamageMobj(newmobj, actor, actor, 10000);
        return;
    }

    (*newmobj).target = (*actor).target;
    A_SkullAttack(newmobj as *mut c_void);
}

pub unsafe fn A_PainAttack(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*actor).target.is_null() {
        return;
    }

    A_FaceTarget(actor_ptr);
    A_PainShootSkull(actor, (*actor).angle);
}

pub unsafe fn A_PainDie(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    A_Fall(actor_ptr);
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG90));
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG180));
    A_PainShootSkull(actor, (*actor).angle.wrapping_add(ANG270));
}

pub unsafe fn A_Scream(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    let sound = match (*(*actor).info).deathsound {
        0 => return,
        sfx_podth1 | sfx_podth2 | sfx_podth3 => sfx_podth1 + (P_Random() % 3),
        sfx_bgdth1 | sfx_bgdth2 => sfx_bgdth1 + (P_Random() % 2),
        s => s,
    };

    if (*actor).type_ == MT_SPIDER || (*actor).type_ == MT_CYBORG {
        S_StartSound(std::ptr::null_mut(), sound);
    } else {
        S_StartSound(actor_ptr, sound);
    }
}

pub unsafe fn A_XScream(actor_ptr: *mut c_void) {
    S_StartSound(actor_ptr, sfx_slop);
}

pub unsafe fn A_Pain(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    if (*(*actor).info).painsound != 0 {
        S_StartSound(actor_ptr, (*(*actor).info).painsound);
    }
}

pub unsafe fn A_Fall(actor_ptr: *mut c_void) {
    let actor = actor_ptr as *mut mobj_t;
    (*actor).flags &= !MF_SOLID;
}

pub unsafe fn A_Explode(thingy_ptr: *mut c_void) {
    let thingy = thingy_ptr as *mut mobj_t;
    P_RadiusAttack(thingy, (*thingy).target, 128);
}

pub unsafe fn A_BossDeath(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    let mut junk: line_t = std::mem::zeroed();

    if gamemode == commercial {
        if gamemap != 7 {
            return;
        }
        if (*mo).type_ != MT_FATSO && (*mo).type_ != MT_BABY {
            return;
        }
    } else {
        match gameepisode {
            1 => {
                if gamemap != 8 || (*mo).type_ != MT_BRUISER {
                    return;
                }
            }
            2 => {
                if gamemap != 8 || (*mo).type_ != MT_CYBORG {
                    return;
                }
            }
            3 => {
                if gamemap != 8 || (*mo).type_ != MT_SPIDER {
                    return;
                }
            }
            4 => match gamemap {
                6 => {
                    if (*mo).type_ != MT_CYBORG {
                        return;
                    }
                }
                8 => {
                    if (*mo).type_ != MT_SPIDER {
                        return;
                    }
                }
                _ => return,
            },
            _ => {
                if gamemap != 8 {
                    return;
                }
            }
        }
    }

    let mut i = 0;
    while i < MAXPLAYERS {
        if playeringame[i] != 0 && players[i].health > 0 {
            break;
        }
        i += 1;
    }

    if i == MAXPLAYERS {
        return;
    }

    let mut th = thinkercap.next;
    while th != &mut thinkercap {
        if (*th).function != actionf_t::acp1(P_MobjThinker) {
            th = (*th).next;
            continue;
        }
        let mo2 = th as *mut mobj_t;
        if mo2 != mo && (*mo2).type_ == (*mo).type_ && (*mo2).health > 0 {
            return;
        }
        th = (*th).next;
    }

    if gamemode == commercial {
        if gamemap == 7 {
            if (*mo).type_ == MT_FATSO {
                junk.tag = 666;
                EV_DoFloor(&mut junk, lowerFloorToLowest);
                return;
            }
            if (*mo).type_ == MT_BABY {
                junk.tag = 667;
                EV_DoFloor(&mut junk, raiseToTexture);
                return;
            }
        }
    } else {
        match gameepisode {
            1 => {
                junk.tag = 666;
                EV_DoFloor(&mut junk, lowerFloorToLowest);
                return;
            }
            4 => match gamemap {
                6 => {
                    junk.tag = 666;
                    EV_DoDoor(&mut junk, blazeOpen);
                    return;
                }
                8 => {
                    junk.tag = 666;
                    EV_DoFloor(&mut junk, lowerFloorToLowest);
                    return;
                }
                _ => {}
            },
            _ => {}
        }
    }

    G_ExitLevel();
}

pub unsafe fn A_Hoof(mo_ptr: *mut c_void) {
    S_StartSound(mo_ptr, sfx_hoof);
    A_Chase(mo_ptr);
}

pub unsafe fn A_Metal(mo_ptr: *mut c_void) {
    S_StartSound(mo_ptr, sfx_metal);
    A_Chase(mo_ptr);
}

pub unsafe fn A_BabyMetal(mo_ptr: *mut c_void) {
    S_StartSound(mo_ptr, sfx_bspwlk);
    A_Chase(mo_ptr);
}

pub unsafe fn A_OpenShotgun2(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_dbopn);
}

pub unsafe fn A_LoadShotgun2(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_dbload);
}

pub unsafe fn A_CloseShotgun2(player_ptr: *mut c_void, psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_dbcls);
    A_ReFire(player_ptr, psp_ptr);
}

pub static mut braintargets: [*mut mobj_t; 32] = [std::ptr::null_mut(); 32];
pub static mut numbraintargets: i32 = 0;
pub static mut braintargeton: i32 = 0;

pub unsafe fn A_BrainAwake(_mo_ptr: *mut c_void) {
    numbraintargets = 0;
    braintargeton = 0;

    let mut thinker = thinkercap.next;
    while thinker != &mut thinkercap {
        if (*thinker).function != actionf_t::acp1(P_MobjThinker) {
            thinker = (*thinker).next;
            continue;
        }
        let m = thinker as *mut mobj_t;
        if (*m).type_ == MT_BOSSTARGET {
            braintargets[numbraintargets as usize] = m;
            numbraintargets += 1;
        }
        thinker = (*thinker).next;
    }

    S_StartSound(std::ptr::null_mut(), sfx_bossit);
}

pub unsafe fn A_BrainPain(_mo_ptr: *mut c_void) {
    S_StartSound(std::ptr::null_mut(), sfx_bospn);
}

pub unsafe fn A_BrainScream(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    let mut x = (*mo).x - 196 * FRACUNIT;
    while x < (*mo).x + 320 * FRACUNIT {
        let y = (*mo).y - 320 * FRACUNIT;
        let z = 128 + P_Random() * 2 * FRACUNIT;
        let th = P_SpawnMobj(x, y, z, MT_ROCKET);
        (*th).momz = P_Random() * 512;

        P_SetMobjState(th, S_BRAINEXPLODE1);

        (*th).tics -= P_Random() & 7;
        if (*th).tics < 1 {
            (*th).tics = 1;
        }
        x += FRACUNIT * 8;
    }

    S_StartSound(std::ptr::null_mut(), sfx_bosdth);
}

pub unsafe fn A_BrainExplode(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    let r1 = P_Random();
    let r2 = P_Random();
    let x = (*mo).x + (r1 - r2) * 2048;
    let y = (*mo).y;
    let z = 128 + P_Random() * 2 * FRACUNIT;
    let th = P_SpawnMobj(x, y, z, MT_ROCKET);
    (*th).momz = P_Random() * 512;

    P_SetMobjState(th, S_BRAINEXPLODE1);

    (*th).tics -= P_Random() & 7;
    if (*th).tics < 1 {
        (*th).tics = 1;
    }
}

pub unsafe fn A_BrainDie(_mo_ptr: *mut c_void) {
    G_ExitLevel();
}

static mut easy: i32 = 0;

pub unsafe fn A_BrainSpit(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    easy ^= 1;
    if gameskill <= sk_easy && easy == 0 {
        return;
    }

    let targ = braintargets[braintargeton as usize];
    braintargeton = (braintargeton + 1) % numbraintargets;

    let newmobj = P_SpawnMissile(mo, targ, MT_SPAWNSHOT);
    (*newmobj).target = targ;
    (*newmobj).reactiontime =
        (((*targ).y - (*mo).y) / (*newmobj).momy) / (*(*newmobj).state).tics;

    S_StartSound(std::ptr::null_mut(), sfx_bospit);
}

pub unsafe fn A_SpawnSound(mo_ptr: *mut c_void) {
    S_StartSound(mo_ptr, sfx_boscub);
    A_SpawnFly(mo_ptr);
}

pub unsafe fn A_SpawnFly(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    (*mo).reactiontime -= 1;
    if (*mo).reactiontime != 0 {
        return;
    }

    let targ = (*mo).target;

    let fog = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, MT_SPAWNFIRE);
    S_StartSound(fog as *mut c_void, sfx_telept);

    let r = P_Random();
    let type_ = if r < 50 {
        MT_TROOP
    } else if r < 90 {
        MT_SERGEANT
    } else if r < 120 {
        MT_SHADOWS
    } else if r < 130 {
        MT_PAIN
    } else if r < 160 {
        MT_HEAD
    } else if r < 162 {
        MT_VILE
    } else if r < 172 {
        MT_UNDEAD
    } else if r < 192 {
        MT_BABY
    } else if r < 222 {
        MT_FATSO
    } else if r < 246 {
        MT_KNIGHT
    } else {
        MT_BRUISER
    };

    let newmobj = P_SpawnMobj((*targ).x, (*targ).y, (*targ).z, type_);
    if P_LookForPlayers(newmobj, true_) != 0 {
        P_SetMobjState(newmobj, (*(*newmobj).info).seestate);
    }

    P_TeleportMove(newmobj, (*newmobj).x, (*newmobj).y);
    P_RemoveMobj(mo);
}

pub unsafe fn A_PlayerScream(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    let mut sound = sfx_pldeth;
    if gamemode == commercial && (*mo).health < -50 {
        sound = sfx_pdiehi;
    }
    S_StartSound(mo_ptr, sound);
}
