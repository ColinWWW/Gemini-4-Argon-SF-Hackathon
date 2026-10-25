// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_floor::*;
use crate::p_mobj::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::z_zone::*;

pub unsafe fn T_VerticalDoor(door_ptr: *mut c_void) {
    let door = door_ptr as *mut vldoor_t;
    match (*door).direction {
        0 => {
            (*door).topcountdown -= 1;
            if (*door).topcountdown == 0 {
                match (*door).type_ {
                    blazeRaise => {
                        (*door).direction = -1;
                        S_StartSound(
                            &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_bdcls,
                        );
                    }
                    normal => {
                        (*door).direction = -1;
                        S_StartSound(
                            &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_dorcls,
                        );
                    }
                    close30ThenOpen => {
                        (*door).direction = 1;
                        S_StartSound(
                            &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_doropn,
                        );
                    }
                    _ => {}
                }
            }
        }
        2 => {
            (*door).topcountdown -= 1;
            if (*door).topcountdown == 0 {
                match (*door).type_ {
                    raiseIn5Mins => {
                        (*door).direction = 1;
                        (*door).type_ = normal;
                        S_StartSound(
                            &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_doropn,
                        );
                    }
                    _ => {}
                }
            }
        }
        -1 => {
            let res = T_MovePlane(
                (*door).sector,
                (*door).speed,
                (*(*door).sector).floorheight,
                false_,
                1,
                (*door).direction,
            );
            if res == pastdest {
                match (*door).type_ {
                    blazeRaise | blazeClose => {
                        (*(*door).sector).specialdata = std::ptr::null_mut();
                        P_RemoveThinker(&mut (*door).thinker);
                        S_StartSound(
                            &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_bdcls,
                        );
                    }
                    normal | close => {
                        (*(*door).sector).specialdata = std::ptr::null_mut();
                        P_RemoveThinker(&mut (*door).thinker);
                    }
                    close30ThenOpen => {
                        (*door).direction = 0;
                        (*door).topcountdown = 35 * 30;
                    }
                    _ => {}
                }
            } else if res == crushed {
                match (*door).type_ {
                    blazeClose | close => {}
                    _ => {
                        (*door).direction = 1;
                        S_StartSound(
                            &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                            sfx_doropn,
                        );
                    }
                }
            }
        }
        1 => {
            let res = T_MovePlane(
                (*door).sector,
                (*door).speed,
                (*door).topheight,
                false_,
                1,
                (*door).direction,
            );
            if res == pastdest {
                match (*door).type_ {
                    blazeRaise | normal => {
                        (*door).direction = 0;
                        (*door).topcountdown = (*door).topwait;
                    }
                    close30ThenOpen | blazeOpen | open => {
                        (*(*door).sector).specialdata = std::ptr::null_mut();
                        P_RemoveThinker(&mut (*door).thinker);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

pub unsafe fn EV_DoLockedDoor(line: *mut line_t, type_: vldoor_e, thing: *mut mobj_t) -> i32 {
    let p = (*thing).player;
    if p.is_null() {
        return 0;
    }

    match (*line).special {
        99 | 133 => {
            if (*p).cards[it_bluecard as usize] == 0 && (*p).cards[it_blueskull as usize] == 0 {
                (*p).message = b"You need a blue key to activate this object\0".as_ptr();
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return 0;
            }
        }
        134 | 135 => {
            if (*p).cards[it_redcard as usize] == 0 && (*p).cards[it_redskull as usize] == 0 {
                (*p).message = b"You need a red key to activate this object\0".as_ptr();
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return 0;
            }
        }
        136 | 137 => {
            if (*p).cards[it_yellowcard as usize] == 0 && (*p).cards[it_yellowskull as usize] == 0 {
                (*p).message = b"You need a yellow key to activate this object\0".as_ptr();
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return 0;
            }
        }
        _ => {}
    }

    EV_DoDoor(line, type_)
}

pub unsafe fn EV_DoDoor(line: *mut line_t, type_: vldoor_e) -> i32 {
    let mut secnum = -1;
    let mut rtn = 0;

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
        let door = Z_Malloc(
            std::mem::size_of::<vldoor_t>() as i32,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut vldoor_t;
        P_AddThinker(&mut (*door).thinker);
        (*sec).specialdata = door as *mut c_void;

        (*door).thinker.function = actionf_t::acp1(T_VerticalDoor);
        (*door).sector = sec;
        (*door).type_ = type_;
        (*door).topwait = VDOORWAIT;
        (*door).speed = VDOORSPEED;

        match type_ {
            blazeClose => {
                (*door).topheight = P_FindLowestCeilingSurrounding(sec) - 4 * FRACUNIT;
                (*door).direction = -1;
                (*door).speed = VDOORSPEED * 4;
                S_StartSound(
                    &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_bdcls,
                );
            }
            close => {
                (*door).topheight = P_FindLowestCeilingSurrounding(sec) - 4 * FRACUNIT;
                (*door).direction = -1;
                S_StartSound(
                    &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_dorcls,
                );
            }
            close30ThenOpen => {
                (*door).topheight = (*sec).ceilingheight;
                (*door).direction = -1;
                S_StartSound(
                    &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                    sfx_dorcls,
                );
            }
            blazeRaise | blazeOpen => {
                (*door).direction = 1;
                (*door).topheight = P_FindLowestCeilingSurrounding(sec) - 4 * FRACUNIT;
                (*door).speed = VDOORSPEED * 4;
                if (*door).topheight != (*sec).ceilingheight {
                    S_StartSound(
                        &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                        sfx_bdopn,
                    );
                }
            }
            normal | open => {
                (*door).direction = 1;
                (*door).topheight = P_FindLowestCeilingSurrounding(sec) - 4 * FRACUNIT;
                if (*door).topheight != (*sec).ceilingheight {
                    S_StartSound(
                        &mut (*(*door).sector).soundorg as *mut degenmobj_t as *mut c_void,
                        sfx_doropn,
                    );
                }
            }
            _ => {}
        }
    }

    rtn
}

pub unsafe fn EV_VerticalDoor(line: *mut line_t, thing: *mut mobj_t) {
    let side = 0;
    let player = (*thing).player;

    match (*line).special {
        26 | 32 => {
            if player.is_null() {
                return;
            }
            if (*player).cards[it_bluecard as usize] == 0
                && (*player).cards[it_blueskull as usize] == 0
            {
                (*player).message = b"You need a blue key to open this door\0".as_ptr();
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return;
            }
        }
        27 | 34 => {
            if player.is_null() {
                return;
            }
            if (*player).cards[it_yellowcard as usize] == 0
                && (*player).cards[it_yellowskull as usize] == 0
            {
                (*player).message = b"You need a yellow key to open this door\0".as_ptr();
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return;
            }
        }
        28 | 33 => {
            if player.is_null() {
                return;
            }
            if (*player).cards[it_redcard as usize] == 0
                && (*player).cards[it_redskull as usize] == 0
            {
                (*player).message = b"You need a red key to open this door\0".as_ptr();
                S_StartSound(std::ptr::null_mut(), sfx_oof);
                return;
            }
        }
        _ => {}
    }

    let sec = (*sides.add((*line).sidenum[(side ^ 1) as usize] as usize)).sector;

    if !(*sec).specialdata.is_null() {
        let door = (*sec).specialdata as *mut vldoor_t;
        match (*line).special {
            1 | 26 | 27 | 28 | 117 => {
                if (*door).direction == -1 {
                    (*door).direction = 1;
                } else {
                    if (*thing).player.is_null() {
                        return;
                    }
                    (*door).direction = -1;
                }
                return;
            }
            _ => {}
        }
    }

    match (*line).special {
        117 | 118 => {
            S_StartSound(
                &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                sfx_bdopn,
            );
        }
        1 | 31 => {
            S_StartSound(
                &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                sfx_doropn,
            );
        }
        _ => {
            S_StartSound(
                &mut (*sec).soundorg as *mut degenmobj_t as *mut c_void,
                sfx_doropn,
            );
        }
    }

    let door = Z_Malloc(
        std::mem::size_of::<vldoor_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut vldoor_t;
    P_AddThinker(&mut (*door).thinker);
    (*sec).specialdata = door as *mut c_void;
    (*door).thinker.function = actionf_t::acp1(T_VerticalDoor);
    (*door).sector = sec;
    (*door).direction = 1;
    (*door).speed = VDOORSPEED;
    (*door).topwait = VDOORWAIT;

    match (*line).special {
        1 | 26 | 27 | 28 => {
            (*door).type_ = normal;
        }
        31 | 32 | 33 | 34 => {
            (*door).type_ = open;
            (*line).special = 0;
        }
        117 => {
            (*door).type_ = blazeRaise;
            (*door).speed = VDOORSPEED * 4;
        }
        118 => {
            (*door).type_ = blazeOpen;
            (*line).special = 0;
            (*door).speed = VDOORSPEED * 4;
        }
        _ => {}
    }

    (*door).topheight = P_FindLowestCeilingSurrounding(sec) - 4 * FRACUNIT;
}

pub unsafe fn P_SpawnDoorCloseIn30(sec: *mut sector_t) {
    let door = Z_Malloc(
        std::mem::size_of::<vldoor_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut vldoor_t;

    P_AddThinker(&mut (*door).thinker);

    (*sec).specialdata = door as *mut c_void;
    (*sec).special = 0;

    (*door).thinker.function = actionf_t::acp1(T_VerticalDoor);
    (*door).sector = sec;
    (*door).direction = 0;
    (*door).type_ = normal;
    (*door).speed = VDOORSPEED;
    (*door).topcountdown = 30 * 35;
}

pub unsafe fn P_SpawnDoorRaiseIn5Mins(sec: *mut sector_t, _secnum: i32) {
    let door = Z_Malloc(
        std::mem::size_of::<vldoor_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut vldoor_t;

    P_AddThinker(&mut (*door).thinker);

    (*sec).specialdata = door as *mut c_void;
    (*sec).special = 0;

    (*door).thinker.function = actionf_t::acp1(T_VerticalDoor);
    (*door).sector = sec;
    (*door).direction = 2;
    (*door).type_ = raiseIn5Mins;
    (*door).speed = VDOORSPEED;
    (*door).topheight = P_FindLowestCeilingSurrounding(sec) - 4 * FRACUNIT;
    (*door).topwait = VDOORWAIT;
    (*door).topcountdown = 5 * 60 * 35;
}
