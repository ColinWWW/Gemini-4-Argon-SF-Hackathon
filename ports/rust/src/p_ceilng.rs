// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_floor::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::z_zone::*;

pub static mut activeceilings: [*mut ceiling_t; MAXCEILINGS] = [std::ptr::null_mut(); MAXCEILINGS];

pub unsafe fn T_MoveCeiling(ceiling_ptr: *mut c_void) {
    let ceiling = ceiling_ptr as *mut ceiling_t;
    match (*ceiling).direction {
        0 => {}
        1 => {
            let res = T_MovePlane(
                (*ceiling).sector,
                (*ceiling).speed,
                (*ceiling).topheight,
                false_,
                1,
                (*ceiling).direction,
            );

            if (leveltime & 7) == 0 {
                match (*ceiling).type_ {
                    silentCrushAndRaise => {}
                    _ => {
                        S_StartSound(
                            &mut (*(*ceiling).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_stnmov,
                        );
                    }
                }
            }

            if res == pastdest {
                match (*ceiling).type_ {
                    raiseToHighest => {
                        P_RemoveActiveCeiling(ceiling);
                    }
                    silentCrushAndRaise => {
                        S_StartSound(
                            &mut (*(*ceiling).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_pstop,
                        );
                        (*ceiling).direction = -1;
                    }
                    fastCrushAndRaise | crushAndRaise => {
                        (*ceiling).direction = -1;
                    }
                    _ => {}
                }
            }
        }
        -1 => {
            let res = T_MovePlane(
                (*ceiling).sector,
                (*ceiling).speed,
                (*ceiling).bottomheight,
                (*ceiling).crush,
                1,
                (*ceiling).direction,
            );

            if (leveltime & 7) == 0 {
                match (*ceiling).type_ {
                    silentCrushAndRaise => {}
                    _ => {
                        S_StartSound(
                            &mut (*(*ceiling).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_stnmov,
                        );
                    }
                }
            }

            if res == pastdest {
                match (*ceiling).type_ {
                    silentCrushAndRaise => {
                        S_StartSound(
                            &mut (*(*ceiling).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_pstop,
                        );
                        (*ceiling).speed = CEILSPEED;
                        (*ceiling).direction = 1;
                    }
                    crushAndRaise => {
                        (*ceiling).speed = CEILSPEED;
                        (*ceiling).direction = 1;
                    }
                    fastCrushAndRaise => {
                        (*ceiling).direction = 1;
                    }
                    lowerAndCrush | lowerToFloor => {
                        P_RemoveActiveCeiling(ceiling);
                    }
                    _ => {}
                }
            } else if res == crushed {
                match (*ceiling).type_ {
                    silentCrushAndRaise | crushAndRaise | lowerAndCrush => {
                        (*ceiling).speed = CEILSPEED / 8;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub unsafe fn EV_DoCeiling(line: *mut line_t, type_: ceiling_e) -> i32 {
    let mut secnum = -1;
    let mut rtn = 0;

    match type_ {
        fastCrushAndRaise | silentCrushAndRaise | crushAndRaise => {
            P_ActivateInStasisCeiling(line);
        }
        _ => {}
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
        let ceiling = Z_Malloc(
            std::mem::size_of::<ceiling_t>() as i32,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut ceiling_t;
        P_AddThinker(&mut (*ceiling).thinker);
        (*sec).specialdata = ceiling as *mut c_void;
        (*ceiling).thinker.function = actionf_t::acp1(T_MoveCeiling);
        (*ceiling).sector = sec;
        (*ceiling).crush = false_;

        match type_ {
            fastCrushAndRaise => {
                (*ceiling).crush = true_;
                (*ceiling).topheight = (*sec).ceilingheight;
                (*ceiling).bottomheight = (*sec).floorheight + 8 * FRACUNIT;
                (*ceiling).direction = -1;
                (*ceiling).speed = CEILSPEED * 2;
            }
            silentCrushAndRaise | crushAndRaise => {
                (*ceiling).crush = true_;
                (*ceiling).topheight = (*sec).ceilingheight;
                (*ceiling).bottomheight = (*sec).floorheight + 8 * FRACUNIT;
                (*ceiling).direction = -1;
                (*ceiling).speed = CEILSPEED;
            }
            lowerAndCrush | lowerToFloor => {
                (*ceiling).bottomheight = (*sec).floorheight;
                if type_ != lowerToFloor {
                    (*ceiling).bottomheight += 8 * FRACUNIT;
                }
                (*ceiling).direction = -1;
                (*ceiling).speed = CEILSPEED;
            }
            raiseToHighest => {
                (*ceiling).topheight = P_FindHighestCeilingSurrounding(sec);
                (*ceiling).direction = 1;
                (*ceiling).speed = CEILSPEED;
            }
            _ => {}
        }

        (*ceiling).tag = (*sec).tag as i32;
        (*ceiling).type_ = type_;
        P_AddActiveCeiling(ceiling);
    }

    rtn
}

pub unsafe fn P_AddActiveCeiling(c: *mut ceiling_t) {
    for i in 0..MAXCEILINGS {
        if activeceilings[i].is_null() {
            activeceilings[i] = c;
            return;
        }
    }
}

pub unsafe fn P_RemoveActiveCeiling(c: *mut ceiling_t) {
    for i in 0..MAXCEILINGS {
        if activeceilings[i] == c {
            (*(*activeceilings[i]).sector).specialdata = std::ptr::null_mut();
            P_RemoveThinker(&mut (*activeceilings[i]).thinker);
            activeceilings[i] = std::ptr::null_mut();
            break;
        }
    }
}

pub unsafe fn P_ActivateInStasisCeiling(line: *mut line_t) {
    for i in 0..MAXCEILINGS {
        if !activeceilings[i].is_null()
            && (*activeceilings[i]).tag == ((*line).tag as i32)
            && (*activeceilings[i]).direction == 0
        {
            (*activeceilings[i]).direction = (*activeceilings[i]).olddirection;
            (*activeceilings[i]).thinker.function = actionf_t::acp1(T_MoveCeiling);
        }
    }
}

pub unsafe fn EV_CeilingCrushStop(line: *mut line_t) -> i32 {
    let mut rtn = 0;
    for i in 0..MAXCEILINGS {
        if !activeceilings[i].is_null()
            && (*activeceilings[i]).tag == ((*line).tag as i32)
            && (*activeceilings[i]).direction != 0
        {
            (*activeceilings[i]).olddirection = (*activeceilings[i]).direction;
            (*activeceilings[i]).thinker.function = actionf_t::None;
            (*activeceilings[i]).direction = 0;
            rtn = 1;
        }
    }
    rtn
}
