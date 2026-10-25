// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::doomdata::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_map::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::z_zone::*;

pub unsafe fn T_MovePlane(
    sector: *mut sector_t,
    speed: fixed_t,
    dest: fixed_t,
    crush: boolean,
    floorOrCeiling: i32,
    direction: i32,
) -> result_e {
    match floorOrCeiling {
        0 => match direction {
            -1 => {
                if (*sector).floorheight - speed < dest {
                    let lastpos = (*sector).floorheight;
                    (*sector).floorheight = dest;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        (*sector).floorheight = lastpos;
                        P_ChangeSector(sector, crush);
                    }
                    return pastdest;
                } else {
                    let lastpos = (*sector).floorheight;
                    (*sector).floorheight -= speed;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        (*sector).floorheight = lastpos;
                        P_ChangeSector(sector, crush);
                        return crushed;
                    }
                }
            }
            1 => {
                if (*sector).floorheight + speed > dest {
                    let lastpos = (*sector).floorheight;
                    (*sector).floorheight = dest;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        (*sector).floorheight = lastpos;
                        P_ChangeSector(sector, crush);
                    }
                    return pastdest;
                } else {
                    let lastpos = (*sector).floorheight;
                    (*sector).floorheight += speed;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        if crush == true_ {
                            return crushed;
                        }
                        (*sector).floorheight = lastpos;
                        P_ChangeSector(sector, crush);
                        return crushed;
                    }
                }
            }
            _ => {}
        },
        1 => match direction {
            -1 => {
                if (*sector).ceilingheight - speed < dest {
                    let lastpos = (*sector).ceilingheight;
                    (*sector).ceilingheight = dest;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        (*sector).ceilingheight = lastpos;
                        P_ChangeSector(sector, crush);
                    }
                    return pastdest;
                } else {
                    let lastpos = (*sector).ceilingheight;
                    (*sector).ceilingheight -= speed;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        if crush == true_ {
                            return crushed;
                        }
                        (*sector).ceilingheight = lastpos;
                        P_ChangeSector(sector, crush);
                        return crushed;
                    }
                }
            }
            1 => {
                if (*sector).ceilingheight + speed > dest {
                    let lastpos = (*sector).ceilingheight;
                    (*sector).ceilingheight = dest;
                    let flag = P_ChangeSector(sector, crush);
                    if flag == true_ {
                        (*sector).ceilingheight = lastpos;
                        P_ChangeSector(sector, crush);
                    }
                    return pastdest;
                } else {
                    (*sector).ceilingheight += speed;
                    P_ChangeSector(sector, crush);
                }
            }
            _ => {}
        },
        _ => {}
    }
    ok
}

pub unsafe fn T_MoveFloor(floor_ptr: *mut c_void) {
    let floor = floor_ptr as *mut floormove_t;
    let res = T_MovePlane(
        (*floor).sector,
        (*floor).speed,
        (*floor).floordestheight,
        (*floor).crush,
        0,
        (*floor).direction,
    );

    if (leveltime & 7) == 0 {
        S_StartSound(
            &mut (*(*floor).sector).soundorg as *mut degenmobj_t as *mut c_void,
            sfx_stnmov,
        );
    }

    if res == pastdest {
        (*(*floor).sector).specialdata = std::ptr::null_mut();

        if (*floor).direction == 1 {
            if (*floor).type_ == donutRaise {
                (*(*floor).sector).special = (*floor).newspecial as i16;
                (*(*floor).sector).floorpic = (*floor).texture;
            }
        } else if (*floor).direction == -1 {
            if (*floor).type_ == lowerAndChange {
                (*(*floor).sector).special = (*floor).newspecial as i16;
                (*(*floor).sector).floorpic = (*floor).texture;
            }
        }
        P_RemoveThinker(&mut (*floor).thinker);

        S_StartSound(
            &mut (*(*floor).sector).soundorg as *mut degenmobj_t as *mut c_void,
            sfx_pstop,
        );
    }
}

pub unsafe fn EV_DoFloor(line: *mut line_t, floortype: floor_e) -> i32 {
    let mut secnum = -1;
    let mut rtn = 0;

    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if secnum < 0 {
            break;
        }
        let mut sec = sectors.add(secnum as usize);
        if !(*sec).specialdata.is_null() {
            continue;
        }

        rtn = 1;
        let floor = Z_Malloc(
            std::mem::size_of::<floormove_t>() as i32,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut floormove_t;
        P_AddThinker(&mut (*floor).thinker);
        (*sec).specialdata = floor as *mut c_void;
        (*floor).thinker.function = actionf_t::acp1(T_MoveFloor);
        (*floor).type_ = floortype;
        (*floor).crush = false_;

        match floortype {
            lowerFloor => {
                (*floor).direction = -1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = P_FindHighestFloorSurrounding(sec);
            }
            lowerFloorToLowest => {
                (*floor).direction = -1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = P_FindLowestFloorSurrounding(sec);
            }
            turboLower => {
                (*floor).direction = -1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED * 4;
                (*floor).floordestheight = P_FindHighestFloorSurrounding(sec);
                if (*floor).floordestheight != (*sec).floorheight {
                    (*floor).floordestheight += 8 * FRACUNIT;
                }
            }
            raiseFloorCrush | raiseFloor => {
                if floortype == raiseFloorCrush {
                    (*floor).crush = true_;
                }
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = P_FindLowestCeilingSurrounding(sec);
                if (*floor).floordestheight > (*sec).ceilingheight {
                    (*floor).floordestheight = (*sec).ceilingheight;
                }
                (*floor).floordestheight -=
                    (8 * FRACUNIT) * ((floortype == raiseFloorCrush) as i32);
            }
            raiseFloorTurbo => {
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED * 4;
                (*floor).floordestheight = P_FindNextHighestFloor(sec, (*sec).floorheight);
            }
            raiseFloorToNearest => {
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = P_FindNextHighestFloor(sec, (*sec).floorheight);
            }
            raiseFloor24 => {
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = (*(*floor).sector).floorheight + 24 * FRACUNIT;
            }
            raiseFloor512 => {
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = (*(*floor).sector).floorheight + 512 * FRACUNIT;
            }
            raiseFloor24AndChange => {
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = (*(*floor).sector).floorheight + 24 * FRACUNIT;
                (*sec).floorpic = (*(*line).frontsector).floorpic;
                (*sec).special = (*(*line).frontsector).special;
            }
            raiseToTexture => {
                let mut minsize = MAXINT;
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                for i in 0..(*sec).linecount {
                    if twoSided(secnum, i) != 0 {
                        let mut side = getSide(secnum, i, 0);
                        if (*side).bottomtexture >= 0 {
                            if *textureheight.add((*side).bottomtexture as usize) < minsize {
                                minsize = *textureheight.add((*side).bottomtexture as usize);
                            }
                        }
                        side = getSide(secnum, i, 1);
                        if (*side).bottomtexture >= 0 {
                            if *textureheight.add((*side).bottomtexture as usize) < minsize {
                                minsize = *textureheight.add((*side).bottomtexture as usize);
                            }
                        }
                    }
                }
                (*floor).floordestheight = (*(*floor).sector).floorheight + minsize;
            }
            lowerAndChange => {
                (*floor).direction = -1;
                (*floor).sector = sec;
                (*floor).speed = FLOORSPEED;
                (*floor).floordestheight = P_FindLowestFloorSurrounding(sec);
                (*floor).texture = (*sec).floorpic;

                for i in 0..(*sec).linecount {
                    if twoSided(secnum, i) != 0 {
                        let s0_sec = (*getSide(secnum, i, 0)).sector;
                        let s0_num =
                            ((s0_sec as usize - sectors as usize) / std::mem::size_of::<sector_t>())
                                as i32;
                        if s0_num == secnum {
                            sec = getSector(secnum, i, 1);
                            if (*sec).floorheight == (*floor).floordestheight {
                                (*floor).texture = (*sec).floorpic;
                                (*floor).newspecial = (*sec).special as i32;
                                break;
                            }
                        } else {
                            sec = getSector(secnum, i, 0);
                            if (*sec).floorheight == (*floor).floordestheight {
                                (*floor).texture = (*sec).floorpic;
                                (*floor).newspecial = (*sec).special as i32;
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    rtn
}

pub unsafe fn EV_BuildStairs(line: *mut line_t, type_: stair_e) -> i32 {
    let mut secnum = -1;
    let mut rtn = 0;

    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if secnum < 0 {
            break;
        }
        let mut sec = sectors.add(secnum as usize);
        if !(*sec).specialdata.is_null() {
            continue;
        }

        rtn = 1;
        let mut floor = Z_Malloc(
            std::mem::size_of::<floormove_t>() as i32,
            PU_LEVSPEC,
            std::ptr::null_mut(),
        ) as *mut floormove_t;
        P_AddThinker(&mut (*floor).thinker);
        (*sec).specialdata = floor as *mut c_void;
        (*floor).thinker.function = actionf_t::acp1(T_MoveFloor);
        (*floor).direction = 1;
        (*floor).sector = sec;

        let (speed, stairsize) = match type_ {
            build8 => (FLOORSPEED / 4, 8 * FRACUNIT),
            turbo16 => (FLOORSPEED * 4, 16 * FRACUNIT),
            _ => (FLOORSPEED / 4, 8 * FRACUNIT),
        };

        (*floor).speed = speed;
        let mut height = (*sec).floorheight + stairsize;
        (*floor).floordestheight = height;

        let texture = (*sec).floorpic;
        let mut cur_secnum = secnum;

        loop {
            let mut ok_flag = false;
            for i in 0..((*sec).linecount as usize) {
                let l = *(*sec).lines.add(i);
                if ((*l).flags as i32) & ML_TWOSIDED == 0 {
                    continue;
                }

                let mut tsec = (*l).frontsector;
                let mut newsecnum =
                    ((tsec as usize - sectors as usize) / std::mem::size_of::<sector_t>()) as i32;

                if cur_secnum != newsecnum {
                    continue;
                }

                tsec = (*l).backsector;
                newsecnum =
                    ((tsec as usize - sectors as usize) / std::mem::size_of::<sector_t>()) as i32;

                if (*tsec).floorpic != texture {
                    continue;
                }

                height += stairsize;

                if !(*tsec).specialdata.is_null() {
                    continue;
                }

                sec = tsec;
                cur_secnum = newsecnum;
                floor = Z_Malloc(
                    std::mem::size_of::<floormove_t>() as i32,
                    PU_LEVSPEC,
                    std::ptr::null_mut(),
                ) as *mut floormove_t;

                P_AddThinker(&mut (*floor).thinker);

                (*sec).specialdata = floor as *mut c_void;
                (*floor).thinker.function = actionf_t::acp1(T_MoveFloor);
                (*floor).direction = 1;
                (*floor).sector = sec;
                (*floor).speed = speed;
                (*floor).floordestheight = height;
                ok_flag = true;
                break;
            }
            if !ok_flag {
                break;
            }
        }
    }
    rtn
}
