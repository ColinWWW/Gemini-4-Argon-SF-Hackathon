// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_player::player_t;
use crate::d_think::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_map::*;
use crate::p_maputl::*;
use crate::p_pspr::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::r_sky::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::st_stuff::*;
use crate::tables::*;
use crate::z_zone::*;

pub type mobjflag_t = i32;
pub const MF_SPECIAL: i32 = 1;
pub const MF_SOLID: i32 = 2;
pub const MF_SHOOTABLE: i32 = 4;
pub const MF_NOSECTOR: i32 = 8;
pub const MF_NOBLOCKMAP: i32 = 16;
pub const MF_AMBUSH: i32 = 32;
pub const MF_JUSTHIT: i32 = 64;
pub const MF_JUSTATTACKED: i32 = 128;
pub const MF_SPAWNCEILING: i32 = 256;
pub const MF_NOGRAVITY: i32 = 512;
pub const MF_DROPOFF: i32 = 0x400;
pub const MF_PICKUP: i32 = 0x800;
pub const MF_NOCLIP: i32 = 0x1000;
pub const MF_SLIDE: i32 = 0x2000;
pub const MF_FLOAT: i32 = 0x4000;
pub const MF_TELEPORT: i32 = 0x8000;
pub const MF_MISSILE: i32 = 0x10000;
pub const MF_DROPPED: i32 = 0x20000;
pub const MF_SHADOW: i32 = 0x40000;
pub const MF_NOBLOOD: i32 = 0x80000;
pub const MF_CORPSE: i32 = 0x100000;
pub const MF_INFLOAT: i32 = 0x200000;
pub const MF_COUNTKILL: i32 = 0x400000;
pub const MF_COUNTITEM: i32 = 0x800000;
pub const MF_SKULLFLY: i32 = 0x1000000;
pub const MF_NOTDMATCH: i32 = 0x2000000;
pub const MF_TRANSLATION: i32 = 0xc000000;
pub const MF_TRANSSHIFT: i32 = 26;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub snext: *mut mobj_t,
    pub sprev: *mut mobj_t,
    pub angle: angle_t,
    pub sprite: spritenum_t,
    pub frame: i32,
    pub bnext: *mut mobj_t,
    pub bprev: *mut mobj_t,
    pub subsector: *mut subsector_t,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
    pub validcount: i32,
    pub type_: mobjtype_t,
    pub info: *mut mobjinfo_t,
    pub tics: i32,
    pub state: *mut state_t,
    pub flags: i32,
    pub health: i32,
    pub movedir: i32,
    pub movecount: i32,
    pub target: *mut mobj_t,
    pub reactiontime: i32,
    pub threshold: i32,
    pub player: *mut player_t,
    pub lastlook: i32,
    pub spawnpoint: mapthing_t,
    pub tracer: *mut mobj_t,
}

pub static mut test: i32 = 0;

pub unsafe fn P_SetMobjState(mobj: *mut mobj_t, mut state: statenum_t) -> boolean {
    let mut st: *mut state_t;
    loop {
        if state == S_NULL {
            (*mobj).state = std::ptr::null_mut();
            P_RemoveMobj(mobj);
            return false_;
        }

        st = &mut states[state as usize] as *mut state_t;
        (*mobj).state = st;
        (*mobj).tics = (*st).tics;
        (*mobj).sprite = (*st).sprite;
        (*mobj).frame = (*st).frame;

        if let actionf_t::acp1(f) = (*st).action {
            f(mobj as *mut c_void);
        }

        state = (*st).nextstate;
        if (*mobj).tics != 0 {
            break;
        }
    }
    true_
}

pub unsafe fn P_ExplodeMissile(mo: *mut mobj_t) {
    (*mo).momx = 0;
    (*mo).momy = 0;
    (*mo).momz = 0;

    P_SetMobjState(mo, mobjinfo[(*mo).type_ as usize].deathstate);

    (*mo).tics -= P_Random() & 3;
    if (*mo).tics < 1 {
        (*mo).tics = 1;
    }

    (*mo).flags &= !MF_MISSILE;

    if (*(*mo).info).deathsound != 0 {
        S_StartSound(mo as *mut c_void, (*(*mo).info).deathsound);
    }
}

pub const STOPSPEED: fixed_t = 0x1000;
pub const FRICTION: fixed_t = 0xe800;

pub unsafe fn P_XYMovement(mo: *mut mobj_t) {
    let mut ptryx: fixed_t;
    let mut ptryy: fixed_t;
    let player: *mut player_t;
    let mut xmove: fixed_t;
    let mut ymove: fixed_t;

    if (*mo).momx == 0 && (*mo).momy == 0 {
        if (*mo).flags & MF_SKULLFLY != 0 {
            (*mo).flags &= !MF_SKULLFLY;
            (*mo).momx = 0;
            (*mo).momy = 0;
            (*mo).momz = 0;
            P_SetMobjState(mo, (*(*mo).info).spawnstate);
        }
        return;
    }

    player = (*mo).player;

    if (*mo).momx > MAXMOVE {
        (*mo).momx = MAXMOVE;
    } else if (*mo).momx < -MAXMOVE {
        (*mo).momx = -MAXMOVE;
    }

    if (*mo).momy > MAXMOVE {
        (*mo).momy = MAXMOVE;
    } else if (*mo).momy < -MAXMOVE {
        (*mo).momy = -MAXMOVE;
    }

    xmove = (*mo).momx;
    ymove = (*mo).momy;

    loop {
        if xmove > MAXMOVE / 2 || ymove > MAXMOVE / 2 {
            ptryx = (*mo).x + xmove / 2;
            ptryy = (*mo).y + ymove / 2;
            xmove >>= 1;
            ymove >>= 1;
        } else {
            ptryx = (*mo).x + xmove;
            ptryy = (*mo).y + ymove;
            xmove = 0;
            ymove = 0;
        }

        if P_TryMove(mo, ptryx, ptryy) == 0 {
            if !(*mo).player.is_null() {
                P_SlideMove(mo);
            } else if (*mo).flags & MF_MISSILE != 0 {
                if !ceilingline.is_null()
                    && !(*ceilingline).backsector.is_null()
                    && (*(*ceilingline).backsector).ceilingpic == skyflatnum as i16
                {
                    P_RemoveMobj(mo);
                    return;
                }
                P_ExplodeMissile(mo);
            } else {
                (*mo).momx = 0;
                (*mo).momy = 0;
            }
        }
        if xmove == 0 && ymove == 0 {
            break;
        }
    }

    if !player.is_null() && ((*player).cheats & CF_NOMOMENTUM != 0) {
        (*mo).momx = 0;
        (*mo).momy = 0;
        return;
    }

    if (*mo).flags & (MF_MISSILE | MF_SKULLFLY) != 0 {
        return;
    }

    if (*mo).z > (*mo).floorz {
        return;
    }

    if (*mo).flags & MF_CORPSE != 0 {
        if (*mo).momx > FRACUNIT / 4
            || (*mo).momx < -FRACUNIT / 4
            || (*mo).momy > FRACUNIT / 4
            || (*mo).momy < -FRACUNIT / 4
        {
            if (*mo).floorz != (*(*(*mo).subsector).sector).floorheight {
                return;
            }
        }
    }

    if (*mo).momx > -STOPSPEED
        && (*mo).momx < STOPSPEED
        && (*mo).momy > -STOPSPEED
        && (*mo).momy < STOPSPEED
        && (player.is_null()
            || ((*player).cmd.forwardmove == 0 && (*player).cmd.sidemove == 0))
    {
        if !player.is_null() {
            let state_idx = ((*(*player).mo).state as usize - states.as_ptr() as usize)
                / std::mem::size_of::<state_t>();
            if (state_idx as i32 - S_PLAY_RUN1) >= 0 && (state_idx as i32 - S_PLAY_RUN1) < 4 {
                P_SetMobjState((*player).mo, S_PLAY);
            }
        }
        (*mo).momx = 0;
        (*mo).momy = 0;
    } else {
        (*mo).momx = FixedMul((*mo).momx, FRICTION);
        (*mo).momy = FixedMul((*mo).momy, FRICTION);
    }
}

pub unsafe fn P_ZMovement(mo: *mut mobj_t) {
    let dist: fixed_t;
    let delta: fixed_t;

    if !(*mo).player.is_null() && (*mo).z < (*mo).floorz {
        (*(*mo).player).viewheight -= (*mo).floorz - (*mo).z;
        (*(*mo).player).deltaviewheight = (VIEWHEIGHT - (*(*mo).player).viewheight) >> 3;
    }

    (*mo).z += (*mo).momz;

    if ((*mo).flags & MF_FLOAT != 0) && !(*mo).target.is_null() {
        if ((*mo).flags & MF_SKULLFLY == 0) && ((*mo).flags & MF_INFLOAT == 0) {
            dist = P_AproxDistance((*mo).x - (*(*mo).target).x, (*mo).y - (*(*mo).target).y);
            delta = ((*(*mo).target).z + ((*mo).height >> 1)) - (*mo).z;

            if delta < 0 && dist < -(delta * 3) {
                (*mo).z -= FLOATSPEED;
            } else if delta > 0 && dist < (delta * 3) {
                (*mo).z += FLOATSPEED;
            }
        }
    }

    if (*mo).z <= (*mo).floorz {
        if (*mo).flags & MF_SKULLFLY != 0 {
            (*mo).momz = -(*mo).momz;
        }

        if (*mo).momz < 0 {
            if !(*mo).player.is_null() && (*mo).momz < -GRAVITY * 8 {
                (*(*mo).player).deltaviewheight = (*mo).momz >> 3;
                S_StartSound(mo as *mut c_void, sfx_oof);
            }
            (*mo).momz = 0;
        }
        (*mo).z = (*mo).floorz;

        if ((*mo).flags & MF_MISSILE != 0) && ((*mo).flags & MF_NOCLIP == 0) {
            P_ExplodeMissile(mo);
            return;
        }
    } else if (*mo).flags & MF_NOGRAVITY == 0 {
        if (*mo).momz == 0 {
            (*mo).momz = -GRAVITY * 2;
        } else {
            (*mo).momz -= GRAVITY;
        }
    }

    if (*mo).z + (*mo).height > (*mo).ceilingz {
        if (*mo).momz > 0 {
            (*mo).momz = 0;
        }
        (*mo).z = (*mo).ceilingz - (*mo).height;

        if (*mo).flags & MF_SKULLFLY != 0 {
            (*mo).momz = -(*mo).momz;
        }

        if ((*mo).flags & MF_MISSILE != 0) && ((*mo).flags & MF_NOCLIP == 0) {
            P_ExplodeMissile(mo);
            return;
        }
    }
}

pub unsafe fn P_NightmareRespawn(mobj: *mut mobj_t) {
    let x: fixed_t = ((*mobj).spawnpoint.x as fixed_t) << FRACBITS;
    let y: fixed_t = ((*mobj).spawnpoint.y as fixed_t) << FRACBITS;
    let z: fixed_t;
    let ss: *mut subsector_t;
    let mut mo: *mut mobj_t;
    let mthing: *mut mapthing_t;

    if P_CheckPosition(mobj, x, y) == 0 {
        return;
    }

    mo = P_SpawnMobj(
        (*mobj).x,
        (*mobj).y,
        (*(*(*mobj).subsector).sector).floorheight,
        MT_TFOG,
    );
    S_StartSound(mo as *mut c_void, sfx_telept);

    ss = R_PointInSubsector(x, y);
    mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_TFOG);
    S_StartSound(mo as *mut c_void, sfx_telept);

    mthing = &mut (*mobj).spawnpoint;

    if (*(*mobj).info).flags & MF_SPAWNCEILING != 0 {
        z = ONCEILINGZ;
    } else {
        z = ONFLOORZ;
    }

    mo = P_SpawnMobj(x, y, z, (*mobj).type_);
    (*mo).spawnpoint = (*mobj).spawnpoint;
    (*mo).angle = ANG45.wrapping_mul(((*mthing).angle / 45) as u32);

    if ((*mthing).options as i32) & MTF_AMBUSH != 0 {
        (*mo).flags |= MF_AMBUSH;
    }

    (*mo).reactiontime = 18;

    P_RemoveMobj(mobj);
}

pub unsafe fn P_MobjThinker(mobj_ptr: *mut c_void) {
    let mobj = mobj_ptr as *mut mobj_t;
    if (*mobj).momx != 0 || (*mobj).momy != 0 || ((*mobj).flags & MF_SKULLFLY != 0) {
        P_XYMovement(mobj);
        if (*mobj).thinker.function == actionf_t::Removed {
            return;
        }
    }
    if ((*mobj).z != (*mobj).floorz) || (*mobj).momz != 0 {
        P_ZMovement(mobj);
        if (*mobj).thinker.function == actionf_t::Removed {
            return;
        }
    }

    if (*mobj).tics != -1 {
        (*mobj).tics -= 1;
        if (*mobj).tics == 0 {
            if P_SetMobjState(mobj, (*(*mobj).state).nextstate) == 0 {
                return;
            }
        }
    } else {
        if (*mobj).flags & MF_COUNTKILL == 0 {
            return;
        }
        if respawnmonsters == 0 {
            return;
        }
        (*mobj).movecount += 1;
        if (*mobj).movecount < 12 * 35 {
            return;
        }
        if leveltime & 31 != 0 {
            return;
        }
        if P_Random() > 4 {
            return;
        }
        P_NightmareRespawn(mobj);
    }
}

pub unsafe fn P_SpawnMobj(x: fixed_t, y: fixed_t, z: fixed_t, type_: mobjtype_t) -> *mut mobj_t {
    let mobj = Z_Malloc(
        std::mem::size_of::<mobj_t>() as i32,
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut mobj_t;
    std::ptr::write_bytes(mobj, 0, 1);
    let info = &mut mobjinfo[type_ as usize] as *mut mobjinfo_t;

    (*mobj).type_ = type_;
    (*mobj).info = info;
    (*mobj).x = x;
    (*mobj).y = y;
    (*mobj).radius = (*info).radius;
    (*mobj).height = (*info).height;
    (*mobj).flags = (*info).flags;
    (*mobj).health = (*info).spawnhealth;

    if gameskill != sk_nightmare {
        (*mobj).reactiontime = (*info).reactiontime;
    }

    (*mobj).lastlook = P_Random() % (MAXPLAYERS as i32);

    let st = &mut states[(*info).spawnstate as usize] as *mut state_t;
    (*mobj).state = st;
    (*mobj).tics = (*st).tics;
    (*mobj).sprite = (*st).sprite;
    (*mobj).frame = (*st).frame;

    P_SetThingPosition(mobj);

    (*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
    (*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;

    if z == ONFLOORZ {
        (*mobj).z = (*mobj).floorz;
    } else if z == ONCEILINGZ {
        (*mobj).z = (*mobj).ceilingz - (*(*mobj).info).height;
    } else {
        (*mobj).z = z;
    }

    (*mobj).thinker.function = actionf_t::acp1(P_MobjThinker);

    P_AddThinker(&mut (*mobj).thinker);

    mobj
}

pub static mut itemrespawnque: [mapthing_t; ITEMQUESIZE] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    type_: 0,
    options: 0,
}; ITEMQUESIZE];
pub static mut itemrespawntime: [i32; ITEMQUESIZE] = [0; ITEMQUESIZE];
pub static mut iquehead: i32 = 0;
pub static mut iquetail: i32 = 0;

pub unsafe fn P_RemoveMobj(mobj: *mut mobj_t) {
    if ((*mobj).flags & MF_SPECIAL != 0)
        && ((*mobj).flags & MF_DROPPED == 0)
        && ((*mobj).type_ != MT_INV)
        && ((*mobj).type_ != MT_INS)
    {
        itemrespawnque[iquehead as usize] = (*mobj).spawnpoint;
        itemrespawntime[iquehead as usize] = leveltime;
        iquehead = (iquehead + 1) & ((ITEMQUESIZE as i32) - 1);

        if iquehead == iquetail {
            iquetail = (iquetail + 1) & ((ITEMQUESIZE as i32) - 1);
        }
    }

    P_UnsetThingPosition(mobj);
    S_StopSound(mobj as *mut c_void);
    P_RemoveThinker(mobj as *mut thinker_t);
}

pub unsafe fn P_RespawnSpecials() {
    if deathmatch != 2 {
        return;
    }
    if iquehead == iquetail {
        return;
    }
    if leveltime - itemrespawntime[iquetail as usize] < 30 * 35 {
        return;
    }

    let mthing = &mut itemrespawnque[iquetail as usize] as *mut mapthing_t;
    let x = ((*mthing).x as fixed_t) << FRACBITS;
    let y = ((*mthing).y as fixed_t) << FRACBITS;

    let ss = R_PointInSubsector(x, y);
    let mut mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, MT_IFOG);
    S_StartSound(mo as *mut c_void, sfx_itmbk);

    let mut i = 0;
    while i < NUMMOBJTYPES {
        if ((*mthing).type_ as i32) == mobjinfo[i].doomednum {
            break;
        }
        i += 1;
    }

    let z = if mobjinfo[i].flags & MF_SPAWNCEILING != 0 {
        ONCEILINGZ
    } else {
        ONFLOORZ
    };

    mo = P_SpawnMobj(x, y, z, i as mobjtype_t);
    (*mo).spawnpoint = *mthing;
    (*mo).angle = ANG45.wrapping_mul(((*mthing).angle / 45) as u32);

    iquetail = (iquetail + 1) & ((ITEMQUESIZE as i32) - 1);
}

pub unsafe fn P_SpawnPlayer(mthing: *mut mapthing_t) {
    if playeringame[((*mthing).type_ - 1) as usize] == 0 {
        return;
    }

    let p = &mut players[((*mthing).type_ - 1) as usize] as *mut player_t;

    if (*p).playerstate == PST_REBORN {
        G_PlayerReborn(((*mthing).type_ - 1) as i32);
    }

    let x = ((*mthing).x as fixed_t) << FRACBITS;
    let y = ((*mthing).y as fixed_t) << FRACBITS;
    let z = ONFLOORZ;
    let mobj = P_SpawnMobj(x, y, z, MT_PLAYER);

    if (*mthing).type_ > 1 {
        (*mobj).flags |= (((*mthing).type_ - 1) as i32) << MF_TRANSSHIFT;
    }

    (*mobj).angle = ANG45.wrapping_mul(((*mthing).angle / 45) as u32);
    (*mobj).player = p;
    (*mobj).health = (*p).health;

    (*p).mo = mobj;
    (*p).playerstate = PST_LIVE;
    (*p).refire = 0;
    (*p).message = std::ptr::null();
    (*p).damagecount = 0;
    (*p).bonuscount = 0;
    (*p).extralight = 0;
    (*p).fixedcolormap = 0;
    (*p).viewheight = VIEWHEIGHT;

    P_SetupPsprites(p);

    if deathmatch != 0 {
        for i in 0..NUMCARDS {
            (*p).cards[i] = true_;
        }
    }

    if ((*mthing).type_ - 1) as i32 == consoleplayer {
        ST_Start();
        HU_Start();
    }
}

pub unsafe fn P_SpawnMapThing(mthing: *mut mapthing_t) {
    if (*mthing).type_ == 11 {
        if (deathmatch_p as usize) < (deathmatchstarts.as_mut_ptr().add(10) as usize) {
            *deathmatch_p = *mthing;
            deathmatch_p = deathmatch_p.add(1);
        }
        return;
    }

    if (*mthing).type_ <= 4 {
        playerstarts[((*mthing).type_ - 1) as usize] = *mthing;
        if deathmatch == 0 {
            P_SpawnPlayer(mthing);
        }
        return;
    }

    if netgame == 0 && ((*mthing).options & 16 != 0) {
        return;
    }

    let bit = if gameskill == sk_baby {
        1
    } else if gameskill == sk_nightmare {
        4
    } else {
        1 << (gameskill - 1)
    };

    if ((*mthing).options as i32) & bit == 0 {
        return;
    }

    let mut i = 0;
    while i < NUMMOBJTYPES {
        if ((*mthing).type_ as i32) == mobjinfo[i].doomednum {
            break;
        }
        i += 1;
    }

    if i == NUMMOBJTYPES {
        I_Error(&format!(
            "P_SpawnMapThing: Unknown type {} at ({}, {})",
            (*mthing).type_,
            (*mthing).x,
            (*mthing).y
        ));
    }

    if deathmatch != 0 && (mobjinfo[i].flags & MF_NOTDMATCH != 0) {
        return;
    }

    if nomonsters != 0 && (i as i32 == MT_SKULL || (mobjinfo[i].flags & MF_COUNTKILL != 0)) {
        return;
    }

    let x = ((*mthing).x as fixed_t) << FRACBITS;
    let y = ((*mthing).y as fixed_t) << FRACBITS;
    let z = if mobjinfo[i].flags & MF_SPAWNCEILING != 0 {
        ONCEILINGZ
    } else {
        ONFLOORZ
    };

    let mobj = P_SpawnMobj(x, y, z, i as mobjtype_t);
    (*mobj).spawnpoint = *mthing;

    if (*mobj).tics > 0 {
        (*mobj).tics = 1 + (P_Random() % (*mobj).tics);
    }
    if (*mobj).flags & MF_COUNTKILL != 0 {
        totalkills += 1;
    }
    if (*mobj).flags & MF_COUNTITEM != 0 {
        totalitems += 1;
    }

    (*mobj).angle = ANG45.wrapping_mul(((*mthing).angle / 45) as u32);
    if ((*mthing).options as i32) & MTF_AMBUSH != 0 {
        (*mobj).flags |= MF_AMBUSH;
    }
}

pub unsafe fn P_SpawnPuff(x: fixed_t, y: fixed_t, mut z: fixed_t) {
    let r1 = P_Random();
    let r2 = P_Random();
    z += (r1 - r2) << 10;

    let th = P_SpawnMobj(x, y, z, MT_PUFF);
    (*th).momz = FRACUNIT;
    (*th).tics -= P_Random() & 3;

    if (*th).tics < 1 {
        (*th).tics = 1;
    }

    if attackrange == MELEERANGE {
        P_SetMobjState(th, S_PUFF3);
    }
}

pub unsafe fn P_SpawnBlood(x: fixed_t, y: fixed_t, mut z: fixed_t, damage: i32) {
    let r1 = P_Random();
    let r2 = P_Random();
    z += (r1 - r2) << 10;
    let th = P_SpawnMobj(x, y, z, MT_BLOOD);
    (*th).momz = FRACUNIT * 2;
    (*th).tics -= P_Random() & 3;

    if (*th).tics < 1 {
        (*th).tics = 1;
    }

    if damage <= 12 && damage >= 9 {
        P_SetMobjState(th, S_BLOOD2);
    } else if damage < 9 {
        P_SetMobjState(th, S_BLOOD3);
    }
}

pub unsafe fn P_CheckMissileSpawn(th: *mut mobj_t) {
    (*th).tics -= P_Random() & 3;
    if (*th).tics < 1 {
        (*th).tics = 1;
    }

    (*th).x += (*th).momx >> 1;
    (*th).y += (*th).momy >> 1;
    (*th).z += (*th).momz >> 1;

    if P_TryMove(th, (*th).x, (*th).y) == 0 {
        P_ExplodeMissile(th);
    }
}

pub unsafe fn P_SpawnMissile(
    source: *mut mobj_t,
    dest: *mut mobj_t,
    type_: mobjtype_t,
) -> *mut mobj_t {
    let th = P_SpawnMobj((*source).x, (*source).y, (*source).z + 4 * 8 * FRACUNIT, type_);

    if (*(*th).info).seesound != 0 {
        S_StartSound(th as *mut c_void, (*(*th).info).seesound);
    }

    (*th).target = source;
    let mut an = R_PointToAngle2((*source).x, (*source).y, (*dest).x, (*dest).y);

    if (*dest).flags & MF_SHADOW != 0 {
        let r1 = P_Random();
        let r2 = P_Random();
        an = an.wrapping_add(((r1 - r2) << 20) as u32);
    }

    (*th).angle = an;
    an >>= ANGLETOFINESHIFT;
    (*th).momx = FixedMul((*(*th).info).speed, finecosine(an as usize));
    (*th).momy = FixedMul((*(*th).info).speed, finesine[an as usize]);

    let mut dist = P_AproxDistance((*dest).x - (*source).x, (*dest).y - (*source).y);
    dist /= (*(*th).info).speed;

    if dist < 1 {
        dist = 1;
    }

    (*th).momz = ((*dest).z - (*source).z) / dist;
    P_CheckMissileSpawn(th);

    th
}

pub unsafe fn P_SpawnPlayerMissile(source: *mut mobj_t, type_: mobjtype_t) {
    let mut an = (*source).angle;
    let mut slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT);

    if linetarget.is_null() {
        an = an.wrapping_add(1 << 26);
        slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT);

        if linetarget.is_null() {
            an = an.wrapping_sub(2 << 26);
            slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT);
        }

        if linetarget.is_null() {
            an = (*source).angle;
            slope = 0;
        }
    }

    let x = (*source).x;
    let y = (*source).y;
    let z = (*source).z + 4 * 8 * FRACUNIT;

    let th = P_SpawnMobj(x, y, z, type_);

    if (*(*th).info).seesound != 0 {
        S_StartSound(th as *mut c_void, (*(*th).info).seesound);
    }

    (*th).target = source;
    (*th).angle = an;
    (*th).momx = FixedMul(
        (*(*th).info).speed,
        finecosine((an >> ANGLETOFINESHIFT) as usize),
    );
    (*th).momy = FixedMul(
        (*(*th).info).speed,
        finesine[(an >> ANGLETOFINESHIFT) as usize],
    );
    (*th).momz = FixedMul((*(*th).info).speed, slope);

    P_CheckMissileSpawn(th);
}
