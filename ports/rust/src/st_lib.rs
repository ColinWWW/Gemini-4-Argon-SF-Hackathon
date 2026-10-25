// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;
use crate::i_system::*;
use crate::m_swap::*;
use crate::r_defs::*;
use crate::st_stuff::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const BG: i32 = 4;
pub const FG: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_number_t {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub oldnum: i32,
    pub num: *mut i32,
    pub on: *mut boolean,
    pub p: *mut *mut patch_t,
    pub data: i32,
}

impl Default for st_number_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_percent_t {
    pub n: st_number_t,
    pub p: *mut patch_t,
}

impl Default for st_percent_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_multicon_t {
    pub x: i32,
    pub y: i32,
    pub oldinum: i32,
    pub inum: *mut i32,
    pub on: *mut boolean,
    pub p: *mut *mut patch_t,
    pub data: i32,
}

impl Default for st_multicon_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_binicon_t {
    pub x: i32,
    pub y: i32,
    pub oldval: i32,
    pub val: *mut boolean,
    pub on: *mut boolean,
    pub p: *mut patch_t,
    pub data: i32,
}

impl Default for st_binicon_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub static mut sttminus: *mut patch_t = std::ptr::null_mut();

pub unsafe fn STlib_init() {
    sttminus = W_CacheLumpNameStr("STTMINUS", PU_STATIC) as *mut patch_t;
}

pub unsafe fn STlib_initNum(
    n: *mut st_number_t,
    x: i32,
    y: i32,
    pl: *mut *mut patch_t,
    num: *mut i32,
    on: *mut boolean,
    width: i32,
) {
    (*n).x = x;
    (*n).y = y;
    (*n).oldnum = 0;
    (*n).width = width;
    (*n).num = num;
    (*n).on = on;
    (*n).p = pl;
}

pub unsafe fn STlib_drawNum(n: *mut st_number_t, _refresh: boolean) {
    let mut numdigits = (*n).width;
    let mut num = *(*n).num;
    let w = SHORT((*(*(*n).p)).width) as i32;
    let h = SHORT((*(*(*n).p)).height) as i32;

    (*n).oldnum = *(*n).num;

    let neg = num < 0;
    if neg {
        if numdigits == 2 && num < -9 {
            num = -9;
        } else if numdigits == 3 && num < -99 {
            num = -99;
        }
        num = -num;
    }

    let mut x = (*n).x - numdigits * w;
    if (*n).y - ST_Y < 0 {
        I_Error("drawNum: n->y - ST_Y < 0");
    }

    V_CopyRect(x, (*n).y - ST_Y, BG, w * numdigits, h, x, (*n).y, FG);

    if num == 1994 {
        return;
    }

    x = (*n).x;

    if num == 0 {
        V_DrawPatch(x - w, (*n).y, FG, *(*n).p.add(0));
    }

    while num != 0 && numdigits > 0 {
        numdigits -= 1;
        x -= w;
        V_DrawPatch(x, (*n).y, FG, *(*n).p.add((num % 10) as usize));
        num /= 10;
    }

    if neg {
        V_DrawPatch(x - 8, (*n).y, FG, sttminus);
    }
}

pub unsafe fn STlib_updateNum(n: *mut st_number_t, refresh: boolean) {
    if *(*n).on != 0 {
        STlib_drawNum(n, refresh);
    }
}

pub unsafe fn STlib_initPercent(
    p: *mut st_percent_t,
    x: i32,
    y: i32,
    pl: *mut *mut patch_t,
    num: *mut i32,
    on: *mut boolean,
    percent: *mut patch_t,
) {
    STlib_initNum(&mut (*p).n, x, y, pl, num, on, 3);
    (*p).p = percent;
}

pub unsafe fn STlib_updatePercent(per: *mut st_percent_t, refresh: i32) {
    if refresh != 0 && *(*per).n.on != 0 {
        V_DrawPatch((*per).n.x, (*per).n.y, FG, (*per).p);
    }
    STlib_updateNum(&mut (*per).n, refresh);
}

pub unsafe fn STlib_initMultIcon(
    i: *mut st_multicon_t,
    x: i32,
    y: i32,
    il: *mut *mut patch_t,
    inum: *mut i32,
    on: *mut boolean,
) {
    (*i).x = x;
    (*i).y = y;
    (*i).oldinum = -1;
    (*i).inum = inum;
    (*i).on = on;
    (*i).p = il;
}

pub unsafe fn STlib_updateMultIcon(mi: *mut st_multicon_t, refresh: boolean) {
    if *(*mi).on != 0 && ((*mi).oldinum != *(*mi).inum || refresh != 0) && (*(*mi).inum != -1) {
        if (*mi).oldinum != -1 {
            let oldp = *(*mi).p.add((*mi).oldinum as usize);
            let x = (*mi).x - (SHORT((*oldp).leftoffset) as i32);
            let y = (*mi).y - (SHORT((*oldp).topoffset) as i32);
            let w = SHORT((*oldp).width) as i32;
            let h = SHORT((*oldp).height) as i32;

            if y - ST_Y < 0 {
                I_Error("updateMultIcon: y - ST_Y < 0");
            }
            V_CopyRect(x, y - ST_Y, BG, w, h, x, y, FG);
        }
        V_DrawPatch((*mi).x, (*mi).y, FG, *(*mi).p.add((*(*mi).inum) as usize));
        (*mi).oldinum = *(*mi).inum;
    }
}

pub unsafe fn STlib_initBinIcon(
    b: *mut st_binicon_t,
    x: i32,
    y: i32,
    i: *mut patch_t,
    val: *mut boolean,
    on: *mut boolean,
) {
    (*b).x = x;
    (*b).y = y;
    (*b).oldval = 0;
    (*b).val = val;
    (*b).on = on;
    (*b).p = i;
}

pub unsafe fn STlib_updateBinIcon(bi: *mut st_binicon_t, refresh: boolean) {
    if *(*bi).on != 0 && ((*bi).oldval != *(*bi).val || refresh != 0) {
        let x = (*bi).x - (SHORT((*(*bi).p).leftoffset) as i32);
        let y = (*bi).y - (SHORT((*(*bi).p).topoffset) as i32);
        let w = SHORT((*(*bi).p).width) as i32;
        let h = SHORT((*(*bi).p).height) as i32;

        if y - ST_Y < 0 {
            I_Error("updateBinIcon: y - ST_Y < 0");
        }

        if *(*bi).val != 0 {
            V_DrawPatch((*bi).x, (*bi).y, FG, (*bi).p);
        } else {
            V_CopyRect(x, y - ST_Y, BG, w, h, x, y, FG);
        }

        (*bi).oldval = *(*bi).val;
    }
}
