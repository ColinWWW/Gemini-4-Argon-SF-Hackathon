// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_player::*;
use crate::d_think::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_ceilng::*;
use crate::p_doors::*;
use crate::p_floor::*;
use crate::p_lights::*;
use crate::p_maputl::*;
use crate::p_mobj::*;
use crate::p_plats::*;
use crate::p_pspr::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::z_zone::*;

pub static mut save_p: *mut byte = std::ptr::null_mut();

#[inline]
unsafe fn PADSAVEP() {
    save_p = save_p.add((4 - ((save_p as usize) & 3)) & 3);
}

pub unsafe fn P_ArchivePlayers() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        PADSAVEP();
        let dest = save_p as *mut player_t;
        std::ptr::copy_nonoverlapping(&players[i], dest, 1);
        save_p = save_p.add(std::mem::size_of::<player_t>());
        for j in 0..NUMPSPRITES {
            if !(*dest).psprites[j].state.is_null() {
                let idx = ((*dest).psprites[j].state as usize - states.as_ptr() as usize)
                    / std::mem::size_of::<state_t>();
                (*dest).psprites[j].state = idx as *mut state_t;
            }
        }
    }
}

pub unsafe fn P_UnArchivePlayers() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] == 0 {
            continue;
        }
        PADSAVEP();
        std::ptr::copy_nonoverlapping(save_p as *const player_t, &mut players[i], 1);
        save_p = save_p.add(std::mem::size_of::<player_t>());

        players[i].mo = std::ptr::null_mut();
        players[i].message = std::ptr::null();
        players[i].attacker = std::ptr::null_mut();

        for j in 0..NUMPSPRITES {
            if !players[i].psprites[j].state.is_null() {
                let idx = players[i].psprites[j].state as usize;
                players[i].psprites[j].state = &mut states[idx];
            }
        }
    }
}

pub unsafe fn P_ArchiveWorld() {
    let mut put = save_p as *mut i16;

    for i in 0..(numsectors as usize) {
        let sec = sectors.add(i);
        *put = ((*sec).floorheight >> FRACBITS) as i16;
        put = put.add(1);
        *put = ((*sec).ceilingheight >> FRACBITS) as i16;
        put = put.add(1);
        *put = (*sec).floorpic;
        put = put.add(1);
        *put = (*sec).ceilingpic;
        put = put.add(1);
        *put = (*sec).lightlevel;
        put = put.add(1);
        *put = (*sec).special;
        put = put.add(1);
        *put = (*sec).tag;
        put = put.add(1);
    }

    for i in 0..(numlines as usize) {
        let li = lines.add(i);
        *put = (*li).flags;
        put = put.add(1);
        *put = (*li).special;
        put = put.add(1);
        *put = (*li).tag;
        put = put.add(1);

        for j in 0..2 {
            if (*li).sidenum[j] == -1 {
                continue;
            }
            let si = sides.add((*li).sidenum[j] as usize);
            *put = ((*si).textureoffset >> FRACBITS) as i16;
            put = put.add(1);
            *put = ((*si).rowoffset >> FRACBITS) as i16;
            put = put.add(1);
            *put = (*si).toptexture;
            put = put.add(1);
            *put = (*si).bottomtexture;
            put = put.add(1);
            *put = (*si).midtexture;
            put = put.add(1);
        }
    }

    save_p = put as *mut byte;
}

pub unsafe fn P_UnArchiveWorld() {
    let mut get = save_p as *mut i16;

    for i in 0..(numsectors as usize) {
        let sec = sectors.add(i);
        (*sec).floorheight = (*get as i32) << FRACBITS;
        get = get.add(1);
        (*sec).ceilingheight = (*get as i32) << FRACBITS;
        get = get.add(1);
        (*sec).floorpic = *get;
        get = get.add(1);
        (*sec).ceilingpic = *get;
        get = get.add(1);
        (*sec).lightlevel = *get;
        get = get.add(1);
        (*sec).special = *get;
        get = get.add(1);
        (*sec).tag = *get;
        get = get.add(1);
        (*sec).specialdata = std::ptr::null_mut();
        (*sec).soundtarget = std::ptr::null_mut();
    }

    for i in 0..(numlines as usize) {
        let li = lines.add(i);
        (*li).flags = *get;
        get = get.add(1);
        (*li).special = *get;
        get = get.add(1);
        (*li).tag = *get;
        get = get.add(1);

        for j in 0..2 {
            if (*li).sidenum[j] == -1 {
                continue;
            }
            let si = sides.add((*li).sidenum[j] as usize);
            (*si).textureoffset = (*get as i32) << FRACBITS;
            get = get.add(1);
            (*si).rowoffset = (*get as i32) << FRACBITS;
            get = get.add(1);
            (*si).toptexture = *get;
            get = get.add(1);
            (*si).bottomtexture = *get;
            get = get.add(1);
            (*si).midtexture = *get;
            get = get.add(1);
        }
    }

    save_p = get as *mut byte;
}

const tc_end: byte = 0;
const tc_mobj: byte = 1;

pub unsafe fn P_ArchiveThinkers() {
    let mut th = thinkercap.next;
    while th != &mut thinkercap {
        if (*th).function == actionf_t::acp1(P_MobjThinker) {
            *save_p = tc_mobj;
            save_p = save_p.add(1);
            PADSAVEP();
            let mobj = save_p as *mut mobj_t;
            std::ptr::copy_nonoverlapping(th as *const mobj_t, mobj, 1);
            save_p = save_p.add(std::mem::size_of::<mobj_t>());

            let state_idx = ((*mobj).state as usize - states.as_ptr() as usize)
                / std::mem::size_of::<state_t>();
            (*mobj).state = state_idx as *mut state_t;

            if !(*mobj).player.is_null() {
                let p_idx = ((*mobj).player as usize - players.as_ptr() as usize)
                    / std::mem::size_of::<player_t>();
                (*mobj).player = (p_idx + 1) as *mut player_t;
            }
        }
        th = (*th).next;
    }

    *save_p = tc_end;
    save_p = save_p.add(1);
}

pub unsafe fn P_UnArchiveThinkers() {
    let mut currentthinker = thinkercap.next;
    while currentthinker != &mut thinkercap {
        let next = (*currentthinker).next;
        if (*currentthinker).function == actionf_t::acp1(P_MobjThinker) {
            P_RemoveMobj(currentthinker as *mut mobj_t);
        } else {
            Z_Free(currentthinker as *mut std::ffi::c_void);
        }
        currentthinker = next;
    }
    P_InitThinkers();

    loop {
        let tclass = *save_p;
        save_p = save_p.add(1);
        match tclass {
            tc_end => return,
            tc_mobj => {
                PADSAVEP();
                let mobj = Z_Malloc(
                    std::mem::size_of::<mobj_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut mobj_t;
                std::ptr::copy_nonoverlapping(save_p as *const mobj_t, mobj, 1);
                save_p = save_p.add(std::mem::size_of::<mobj_t>());

                let state_idx = (*mobj).state as usize;
                (*mobj).state = &mut states[state_idx];
                (*mobj).target = std::ptr::null_mut();

                if !(*mobj).player.is_null() {
                    let p_idx = ((*mobj).player as usize) - 1;
                    (*mobj).player = &mut players[p_idx];
                    (*(*mobj).player).mo = mobj;
                }
                P_SetThingPosition(mobj);
                (*mobj).info = &mut mobjinfo[(*mobj).type_ as usize];
                (*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
                (*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
                (*mobj).thinker.function = actionf_t::acp1(P_MobjThinker);
                P_AddThinker(&mut (*mobj).thinker);
            }
            _ => {
                I_Error(&format!("Unknown tclass {} in savegame", tclass));
            }
        }
    }
}

const tc_ceiling: byte = 0;
const tc_door: byte = 1;
const tc_floor: byte = 2;
const tc_plat: byte = 3;
const tc_flash: byte = 4;
const tc_strobe: byte = 5;
const tc_glow: byte = 6;
const tc_endspecials: byte = 7;

pub unsafe fn P_ArchiveSpecials() {
    let mut th = thinkercap.next;
    while th != &mut thinkercap {
        if (*th).function == actionf_t::None {
            let mut i = 0;
            while i < MAXCEILINGS {
                if activeceilings[i] == (th as *mut ceiling_t) {
                    break;
                }
                i += 1;
            }
            if i < MAXCEILINGS {
                *save_p = tc_ceiling;
                save_p = save_p.add(1);
                PADSAVEP();
                let ceiling = save_p as *mut ceiling_t;
                std::ptr::copy_nonoverlapping(th as *const ceiling_t, ceiling, 1);
                save_p = save_p.add(std::mem::size_of::<ceiling_t>());
                let sec_idx = ((*ceiling).sector as usize - sectors as usize)
                    / std::mem::size_of::<sector_t>();
                (*ceiling).sector = sec_idx as *mut sector_t;
            }
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_MoveCeiling) {
            *save_p = tc_ceiling;
            save_p = save_p.add(1);
            PADSAVEP();
            let ceiling = save_p as *mut ceiling_t;
            std::ptr::copy_nonoverlapping(th as *const ceiling_t, ceiling, 1);
            save_p = save_p.add(std::mem::size_of::<ceiling_t>());
            let sec_idx =
                ((*ceiling).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*ceiling).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_VerticalDoor) {
            *save_p = tc_door;
            save_p = save_p.add(1);
            PADSAVEP();
            let door = save_p as *mut vldoor_t;
            std::ptr::copy_nonoverlapping(th as *const vldoor_t, door, 1);
            save_p = save_p.add(std::mem::size_of::<vldoor_t>());
            let sec_idx =
                ((*door).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*door).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_MoveFloor) {
            *save_p = tc_floor;
            save_p = save_p.add(1);
            PADSAVEP();
            let floor = save_p as *mut floormove_t;
            std::ptr::copy_nonoverlapping(th as *const floormove_t, floor, 1);
            save_p = save_p.add(std::mem::size_of::<floormove_t>());
            let sec_idx =
                ((*floor).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*floor).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_PlatRaise) {
            *save_p = tc_plat;
            save_p = save_p.add(1);
            PADSAVEP();
            let plat = save_p as *mut plat_t;
            std::ptr::copy_nonoverlapping(th as *const plat_t, plat, 1);
            save_p = save_p.add(std::mem::size_of::<plat_t>());
            let sec_idx =
                ((*plat).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*plat).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_LightFlash) {
            *save_p = tc_flash;
            save_p = save_p.add(1);
            PADSAVEP();
            let flash = save_p as *mut lightflash_t;
            std::ptr::copy_nonoverlapping(th as *const lightflash_t, flash, 1);
            save_p = save_p.add(std::mem::size_of::<lightflash_t>());
            let sec_idx =
                ((*flash).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*flash).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_StrobeFlash) {
            *save_p = tc_strobe;
            save_p = save_p.add(1);
            PADSAVEP();
            let strobe = save_p as *mut strobe_t;
            std::ptr::copy_nonoverlapping(th as *const strobe_t, strobe, 1);
            save_p = save_p.add(std::mem::size_of::<strobe_t>());
            let sec_idx =
                ((*strobe).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*strobe).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        if (*th).function == actionf_t::acp1(T_Glow) {
            *save_p = tc_glow;
            save_p = save_p.add(1);
            PADSAVEP();
            let glow = save_p as *mut glow_t;
            std::ptr::copy_nonoverlapping(th as *const glow_t, glow, 1);
            save_p = save_p.add(std::mem::size_of::<glow_t>());
            let sec_idx =
                ((*glow).sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
            (*glow).sector = sec_idx as *mut sector_t;
            th = (*th).next;
            continue;
        }

        th = (*th).next;
    }

    *save_p = tc_endspecials;
    save_p = save_p.add(1);
}

pub unsafe fn P_UnArchiveSpecials() {
    loop {
        let tclass = *save_p;
        save_p = save_p.add(1);
        match tclass {
            tc_endspecials => return,
            tc_ceiling => {
                PADSAVEP();
                let ceiling = Z_Malloc(
                    std::mem::size_of::<ceiling_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut ceiling_t;
                std::ptr::copy_nonoverlapping(save_p as *const ceiling_t, ceiling, 1);
                save_p = save_p.add(std::mem::size_of::<ceiling_t>());
                (*ceiling).sector = sectors.add((*ceiling).sector as usize);
                (*(*ceiling).sector).specialdata = ceiling as *mut std::ffi::c_void;

                if (*ceiling).thinker.function != actionf_t::None {
                    (*ceiling).thinker.function = actionf_t::acp1(T_MoveCeiling);
                }

                P_AddThinker(&mut (*ceiling).thinker);
                P_AddActiveCeiling(ceiling);
            }
            tc_door => {
                PADSAVEP();
                let door = Z_Malloc(
                    std::mem::size_of::<vldoor_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut vldoor_t;
                std::ptr::copy_nonoverlapping(save_p as *const vldoor_t, door, 1);
                save_p = save_p.add(std::mem::size_of::<vldoor_t>());
                (*door).sector = sectors.add((*door).sector as usize);
                (*(*door).sector).specialdata = door as *mut std::ffi::c_void;
                (*door).thinker.function = actionf_t::acp1(T_VerticalDoor);
                P_AddThinker(&mut (*door).thinker);
            }
            tc_floor => {
                PADSAVEP();
                let floor = Z_Malloc(
                    std::mem::size_of::<floormove_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut floormove_t;
                std::ptr::copy_nonoverlapping(save_p as *const floormove_t, floor, 1);
                save_p = save_p.add(std::mem::size_of::<floormove_t>());
                (*floor).sector = sectors.add((*floor).sector as usize);
                (*(*floor).sector).specialdata = floor as *mut std::ffi::c_void;
                (*floor).thinker.function = actionf_t::acp1(T_MoveFloor);
                P_AddThinker(&mut (*floor).thinker);
            }
            tc_plat => {
                PADSAVEP();
                let plat = Z_Malloc(
                    std::mem::size_of::<plat_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut plat_t;
                std::ptr::copy_nonoverlapping(save_p as *const plat_t, plat, 1);
                save_p = save_p.add(std::mem::size_of::<plat_t>());
                (*plat).sector = sectors.add((*plat).sector as usize);
                (*(*plat).sector).specialdata = plat as *mut std::ffi::c_void;

                if (*plat).thinker.function != actionf_t::None {
                    (*plat).thinker.function = actionf_t::acp1(T_PlatRaise);
                }

                P_AddThinker(&mut (*plat).thinker);
                P_AddActivePlat(plat);
            }
            tc_flash => {
                PADSAVEP();
                let flash = Z_Malloc(
                    std::mem::size_of::<lightflash_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut lightflash_t;
                std::ptr::copy_nonoverlapping(save_p as *const lightflash_t, flash, 1);
                save_p = save_p.add(std::mem::size_of::<lightflash_t>());
                (*flash).sector = sectors.add((*flash).sector as usize);
                (*flash).thinker.function = actionf_t::acp1(T_LightFlash);
                P_AddThinker(&mut (*flash).thinker);
            }
            tc_strobe => {
                PADSAVEP();
                let strobe = Z_Malloc(
                    std::mem::size_of::<strobe_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut strobe_t;
                std::ptr::copy_nonoverlapping(save_p as *const strobe_t, strobe, 1);
                save_p = save_p.add(std::mem::size_of::<strobe_t>());
                (*strobe).sector = sectors.add((*strobe).sector as usize);
                (*strobe).thinker.function = actionf_t::acp1(T_StrobeFlash);
                P_AddThinker(&mut (*strobe).thinker);
            }
            tc_glow => {
                PADSAVEP();
                let glow = Z_Malloc(
                    std::mem::size_of::<glow_t>() as i32,
                    PU_LEVEL,
                    std::ptr::null_mut(),
                ) as *mut glow_t;
                std::ptr::copy_nonoverlapping(save_p as *const glow_t, glow, 1);
                save_p = save_p.add(std::mem::size_of::<glow_t>());
                (*glow).sector = sectors.add((*glow).sector as usize);
                (*glow).thinker.function = actionf_t::acp1(T_Glow);
                P_AddThinker(&mut (*glow).thinker);
            }
            _ => {
                I_Error(&format!(
                    "P_UnarchiveSpecials:Unknown tclass {} in savegame",
                    tclass
                ));
            }
        }
    }
}
