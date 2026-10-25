// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::p_mobj::*;
use crate::p_spec::*;
use crate::p_user::*;
use crate::z_zone::*;

pub static mut leveltime: i32 = 0;
pub static mut thinkercap: thinker_t = thinker_t {
    prev: std::ptr::null_mut(),
    next: std::ptr::null_mut(),
    function: actionf_t::None,
};

pub unsafe fn P_InitThinkers() {
    thinkercap.prev = &mut thinkercap;
    thinkercap.next = &mut thinkercap;
}

pub unsafe fn P_AddThinker(thinker: *mut thinker_t) {
    (*thinkercap.prev).next = thinker;
    (*thinker).next = &mut thinkercap;
    (*thinker).prev = thinkercap.prev;
    thinkercap.prev = thinker;
}

pub unsafe fn P_RemoveThinker(thinker: *mut thinker_t) {
    (*thinker).function = actionf_t::Removed;
}

pub unsafe fn P_AllocateThinker(_thinker: *mut thinker_t) {}

pub unsafe fn P_RunThinkers() {
    let mut currentthinker = thinkercap.next;
    while currentthinker != &mut thinkercap {
        let next = (*currentthinker).next;
        if (*currentthinker).function == actionf_t::Removed {
            (*(*currentthinker).next).prev = (*currentthinker).prev;
            (*(*currentthinker).prev).next = (*currentthinker).next;
            Z_Free(currentthinker as *mut c_void);
        } else {
            if let actionf_t::acp1(f) = (*currentthinker).function {
                f(currentthinker as *mut c_void);
            }
        }
        currentthinker = next;
    }
}

pub unsafe fn P_Ticker() {
    if paused != 0 {
        return;
    }

    if netgame == 0
        && menuactive != 0
        && demoplayback == 0
        && players[consoleplayer as usize].viewz != 1
    {
        return;
    }

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            P_PlayerThink(&mut players[i]);
        }
    }

    P_RunThinkers();
    P_UpdateSpecials();
    P_RespawnSpecials();

    leveltime += 1;
}
