// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::m_random::*;
use crate::p_spec::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::z_zone::*;

pub unsafe fn T_FireFlicker(flick_ptr: *mut c_void) {
    let flick = flick_ptr as *mut fireflicker_t;
    (*flick).count -= 1;
    if (*flick).count != 0 {
        return;
    }

    let amount = (P_Random() & 3) * 16;

    if ((*(*flick).sector).lightlevel as i32) - amount < (*flick).minlight {
        (*(*flick).sector).lightlevel = (*flick).minlight as i16;
    } else {
        (*(*flick).sector).lightlevel = ((*flick).maxlight - amount) as i16;
    }

    (*flick).count = 4;
}

pub unsafe fn P_SpawnFireFlicker(sector: *mut sector_t) {
    (*sector).special = 0;

    let flick = Z_Malloc(
        std::mem::size_of::<fireflicker_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut fireflicker_t;

    P_AddThinker(&mut (*flick).thinker);

    (*flick).thinker.function = actionf_t::acp1(T_FireFlicker);
    (*flick).sector = sector;
    (*flick).maxlight = (*sector).lightlevel as i32;
    (*flick).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel as i32) + 16;
    (*flick).count = 4;
}

pub unsafe fn T_LightFlash(flash_ptr: *mut c_void) {
    let flash = flash_ptr as *mut lightflash_t;
    (*flash).count -= 1;
    if (*flash).count != 0 {
        return;
    }

    if ((*(*flash).sector).lightlevel as i32) == (*flash).maxlight {
        (*(*flash).sector).lightlevel = (*flash).minlight as i16;
        (*flash).count = (P_Random() & (*flash).mintime) + 1;
    } else {
        (*(*flash).sector).lightlevel = (*flash).maxlight as i16;
        (*flash).count = (P_Random() & (*flash).maxtime) + 1;
    }
}

pub unsafe fn P_SpawnLightFlash(sector: *mut sector_t) {
    (*sector).special = 0;

    let flash = Z_Malloc(
        std::mem::size_of::<lightflash_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut lightflash_t;

    P_AddThinker(&mut (*flash).thinker);

    (*flash).thinker.function = actionf_t::acp1(T_LightFlash);
    (*flash).sector = sector;
    (*flash).maxlight = (*sector).lightlevel as i32;
    (*flash).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel as i32);
    (*flash).maxtime = 64;
    (*flash).mintime = 7;
    (*flash).count = (P_Random() & (*flash).maxtime) + 1;
}

pub unsafe fn T_StrobeFlash(flash_ptr: *mut c_void) {
    let flash = flash_ptr as *mut strobe_t;
    (*flash).count -= 1;
    if (*flash).count != 0 {
        return;
    }

    if ((*(*flash).sector).lightlevel as i32) == (*flash).minlight {
        (*(*flash).sector).lightlevel = (*flash).maxlight as i16;
        (*flash).count = (*flash).brighttime;
    } else {
        (*(*flash).sector).lightlevel = (*flash).minlight as i16;
        (*flash).count = (*flash).darktime;
    }
}

pub unsafe fn P_SpawnStrobeFlash(sector: *mut sector_t, fastOrSlow: i32, inSync: i32) {
    let flash = Z_Malloc(
        std::mem::size_of::<strobe_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut strobe_t;

    P_AddThinker(&mut (*flash).thinker);

    (*flash).sector = sector;
    (*flash).darktime = fastOrSlow;
    (*flash).brighttime = STROBEBRIGHT;
    (*flash).thinker.function = actionf_t::acp1(T_StrobeFlash);
    (*flash).maxlight = (*sector).lightlevel as i32;
    (*flash).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel as i32);

    if (*flash).minlight == (*flash).maxlight {
        (*flash).minlight = 0;
    }

    (*sector).special = 0;

    if inSync == 0 {
        (*flash).count = (P_Random() & 7) + 1;
    } else {
        (*flash).count = 1;
    }
}

pub unsafe fn EV_StartLightStrobing(line: *mut line_t) {
    let mut secnum = -1;
    loop {
        secnum = P_FindSectorFromLineTag(line, secnum);
        if secnum < 0 {
            break;
        }
        let sec = sectors.add(secnum as usize);
        if !(*sec).specialdata.is_null() {
            continue;
        }
        P_SpawnStrobeFlash(sec, SLOWDARK, 0);
    }
}

pub unsafe fn EV_TurnTagLightsOff(line: *mut line_t) {
    for j in 0..(numsectors as usize) {
        let sector = sectors.add(j);
        if (*sector).tag == (*line).tag {
            let mut min = (*sector).lightlevel;
            for i in 0..((*sector).linecount as usize) {
                let templine = *(*sector).lines.add(i);
                let tsec = getNextSector(templine, sector);
                if tsec.is_null() {
                    continue;
                }
                if (*tsec).lightlevel < min {
                    min = (*tsec).lightlevel;
                }
            }
            (*sector).lightlevel = min;
        }
    }
}

pub unsafe fn EV_LightTurnOn(line: *mut line_t, mut bright: i32) {
    for i in 0..(numsectors as usize) {
        let sector = sectors.add(i);
        if (*sector).tag == (*line).tag {
            if bright == 0 {
                for j in 0..((*sector).linecount as usize) {
                    let templine = *(*sector).lines.add(j);
                    let temp = getNextSector(templine, sector);
                    if temp.is_null() {
                        continue;
                    }
                    if ((*temp).lightlevel as i32) > bright {
                        bright = (*temp).lightlevel as i32;
                    }
                }
            }
            (*sector).lightlevel = bright as i16;
        }
    }
}

pub unsafe fn T_Glow(g_ptr: *mut c_void) {
    let g = g_ptr as *mut glow_t;
    match (*g).direction {
        -1 => {
            (*(*g).sector).lightlevel -= GLOWSPEED as i16;
            if ((*(*g).sector).lightlevel as i32) <= (*g).minlight {
                (*(*g).sector).lightlevel += GLOWSPEED as i16;
                (*g).direction = 1;
            }
        }
        1 => {
            (*(*g).sector).lightlevel += GLOWSPEED as i16;
            if ((*(*g).sector).lightlevel as i32) >= (*g).maxlight {
                (*(*g).sector).lightlevel -= GLOWSPEED as i16;
                (*g).direction = -1;
            }
        }
        _ => {}
    }
}

pub unsafe fn P_SpawnGlowingLight(sector: *mut sector_t) {
    let g = Z_Malloc(
        std::mem::size_of::<glow_t>() as i32,
        PU_LEVSPEC,
        std::ptr::null_mut(),
    ) as *mut glow_t;

    P_AddThinker(&mut (*g).thinker);

    (*g).sector = sector;
    (*g).minlight = P_FindMinSurroundingLight(sector, (*sector).lightlevel as i32);
    (*g).maxlight = (*sector).lightlevel as i32;
    (*g).thinker.function = actionf_t::acp1(T_Glow);
    (*g).direction = -1;

    (*sector).special = 0;
}
