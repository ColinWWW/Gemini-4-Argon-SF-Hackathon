// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_floor::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::z_zone::*;

pub static mut activeplats: [*mut plat_t; MAXPLATS] = [std::ptr::null_mut(); MAXPLATS];

pub unsafe fn T_PlatRaise(plat_ptr: *mut c_void) {
    let plat = plat_ptr as *mut plat_t;
    match (*plat).status {
        up => {
            let res = T_MovePlane(
                (*plat).sector,
                (*plat).speed,
                (*plat).high,
                (*plat).crush,
                0,
                1,
            );

            if (*plat).type_ == raiseAndChange || (*plat).type_ == raiseToNearestAndChange {
                if (leveltime & 7) == 0 {
                    S_StartSound(
                        &mut (*(*plat).sector).soundorg as *mut degenmobj_t as *mut c_void,
                        sfx_stnmov,
                    );
                }
            }

            if res == crushed && (*plat).crush == 0 {
                (*plat).count = (*plat).wait;
                (*plat).status = down;
                S_StartSound(
                    &mut (*(*plat).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstart,
                );
            } else if res == pastdest {
                (*plat).count = (*plat).wait;
                (*plat).status = waiting;
                S_StartSound(
                    &mut (*(*plat).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstop,
                );

                match (*plat).type_ {
                    blazeDWUS | downWaitUpStay | raiseAndChange | raiseToNearestAndChange => {
                        P_RemoveActivePlat(plat);
                    }
                    _ => {}
                }
            }
        }
        down => {
            let res = T_MovePlane((*plat).sector, (*plat).speed, (*plat).low, false_, 0, -1);
            if res == pastdest {
                (*plat).count = (*plat).wait;
                (*plat).status = waiting;
                S_StartSound(
                    &mut (*(*plat).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstop,
                );
            }
        }
        waiting => {
            (*plat).count -= 1;
            if (*plat).count == 0 {
                if (*(*plat).sector).floorheight == (*plat).low {
                    (*plat).status = up;
                } else {
                    (*plat).status = down;
                }
                S_StartSound(
                    &mut (*(*plat).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstart,
                );
            }
        }
        in_stasis => {}
        _ => {}
    }
}

pub unsafe fn EV_DoPlat(line: *mut line_t, type_: plattype_e, amount: i32) -> i32 {
    let mut secnum = -1;
    let mut rtn = 0;

    if type_ == perpetualRaise {
        P_ActivateInStasis((*line).tag as i32);
    }

    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if secnum < 0 {
            break;
        }
        let sec = sectors.add(secnum as usize);
        if !(*sec).specialdata.is_null() {
            continue;
        }

        rtn = 1;
        let plat = Z_Malloc(
            std::mem::size_of::<plat_t>() as i32,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut plat_t;
        P_AddThinker(&mut (*plat).thinker);

        (*plat).type_ = type_;
        (*plat).sector = sec;
        (*(*plat).sector).specialdata = plat as *mut c_void;
        (*plat).thinker.function = actionf_t::acp1(T_PlatRaise);
        (*plat).crush = false_;
        (*plat).tag = (*line).tag as i32;

        match type_ {
            raiseToNearestAndChange => {
                (*plat).speed = PLATSPEED / 2;
                (*sec).floorpic = (*(*sides.add((*line).sidenum[0] as usize)).sector).floorpic;
                (*plat).high = P_FindNextHighestFloor(sec, (*sec).floorheight);
                (*plat).wait = 0;
                (*plat).status = up;
                (*sec).special = 0;
                S_StartSound(
                    &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_stnmov,
                );
            }
            raiseAndChange => {
                (*plat).speed = PLATSPEED / 2;
                (*sec).floorpic = (*(*sides.add((*line).sidenum[0] as usize)).sector).floorpic;
                (*plat).high = (*sec).floorheight + amount * FRACUNIT;
                (*plat).wait = 0;
                (*plat).status = up;
                S_StartSound(
                    &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_stnmov,
                );
            }
            downWaitUpStay => {
                (*plat).speed = PLATSPEED * 4;
                (*plat).low = P_FindLowestFloorSurrounding(sec);
                if (*plat).low > (*sec).floorheight {
                    (*plat).low = (*sec).floorheight;
                }
                (*plat).high = (*sec).floorheight;
                (*plat).wait = 35 * PLATWAIT;
                (*plat).status = down;
                S_StartSound(
                    &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstart,
                );
            }
            blazeDWUS => {
                (*plat).speed = PLATSPEED * 8;
                (*plat).low = P_FindLowestFloorSurrounding(sec);
                if (*plat).low > (*sec).floorheight {
                    (*plat).low = (*sec).floorheight;
                }
                (*plat).high = (*sec).floorheight;
                (*plat).wait = 35 * PLATWAIT;
                (*plat).status = down;
                S_StartSound(
                    &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstart,
                );
            }
            perpetualRaise => {
                (*plat).speed = PLATSPEED;
                (*plat).low = P_FindLowestFloorSurrounding(sec);
                if (*plat).low > (*sec).floorheight {
                    (*plat).low = (*sec).floorheight;
                }
                (*plat).high = P_FindHighestFloorSurrounding(sec);
                if (*plat).high < (*sec).floorheight {
                    (*plat).high = (*sec).floorheight;
                }
                (*plat).wait = 35 * PLATWAIT;
                (*plat).status = P_Random() & 1;
                S_StartSound(
                    &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_pstart,
                );
            }
            _ => {}
        }
        P_AddActivePlat(plat);
    }

    rtn
}

pub unsafe fn P_ActivateInStasis(tag: i32) {
    for i in 0..MAXPLATS {
        if !activeplats[i].is_null()
            && (*activeplats[i]).tag == tag
            && (*activeplats[i]).status == in_stasis
        {
            (*activeplats[i]).status = (*activeplats[i]).oldstatus;
            (*activeplats[i]).thinker.function = actionf_t::acp1(T_PlatRaise);
        }
    }
}

pub unsafe fn EV_StopPlat(line: *mut line_t) {
    for j in 0..MAXPLATS {
        if !activeplats[j].is_null()
            && (*activeplats[j]).status != in_stasis
            && (*activeplats[j]).tag == ((*line).tag as i32)
        {
            (*activeplats[j]).oldstatus = (*activeplats[j]).status;
            (*activeplats[j]).status = in_stasis;
            (*activeplats[j]).thinker.function = actionf_t::None;
        }
    }
}

pub unsafe fn P_AddActivePlat(plat: *mut plat_t) {
    for i in 0..MAXPLATS {
        if activeplats[i].is_null() {
            activeplats[i] = plat;
            return;
        }
    }
    I_Error("P_AddActivePlat: no more plats!");
}

pub unsafe fn P_RemoveActivePlat(plat: *mut plat_t) {
    for i in 0..MAXPLATS {
        if plat == activeplats[i] {
            (*(*activeplats[i]).sector).specialdata = std::ptr::null_mut();
            P_RemoveThinker(&mut (*activeplats[i]).thinker);
            activeplats[i] = std::ptr::null_mut();
            return;
        }
    }
    I_Error("P_RemoveActivePlat: can't find plat!");
}
