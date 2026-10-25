// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;
use crate::i_video::*;
use crate::m_random::*;
use crate::v_video::*;
use crate::z_zone::*;

pub const wipe_ColorXForm: i32 = 0;
pub const wipe_Melt: i32 = 1;
pub const wipe_NUMWIPES: i32 = 2;

static mut go: boolean = 0;
static mut wipe_scr_start: *mut byte = std::ptr::null_mut();
static mut wipe_scr_end: *mut byte = std::ptr::null_mut();
static mut wipe_scr: *mut byte = std::ptr::null_mut();
static mut y: *mut i32 = std::ptr::null_mut();

pub unsafe fn wipe_shittyColMajorXform(array: *mut i16, width: i32, height: i32) {
    let dest = Z_Malloc(width * height * 2, PU_STATIC, std::ptr::null_mut()) as *mut i16;

    for row in 0..height {
        for col in 0..width {
            *dest.add((col * height + row) as usize) = *array.add((row * width + col) as usize);
        }
    }

    std::ptr::copy_nonoverlapping(dest, array, (width * height) as usize);
    Z_Free(dest as *mut std::ffi::c_void);
}

pub unsafe fn wipe_initColorXForm(width: i32, height: i32, _ticks: i32) -> i32 {
    std::ptr::copy_nonoverlapping(wipe_scr_start, wipe_scr, (width * height) as usize);
    0
}

pub unsafe fn wipe_doColorXForm(width: i32, height: i32, ticks: i32) -> i32 {
    let mut changed = false_;
    let mut w = wipe_scr;
    let mut e = wipe_scr_end;
    let end = wipe_scr.add((width * height) as usize);

    while w != end {
        if *w != *e {
            if *w > *e {
                let newval = (*w as i32) - ticks;
                if newval < (*e as i32) {
                    *w = *e;
                } else {
                    *w = newval as byte;
                }
                changed = true_;
            } else if *w < *e {
                let newval = (*w as i32) + ticks;
                if newval > (*e as i32) {
                    *w = *e;
                } else {
                    *w = newval as byte;
                }
                changed = true_;
            }
        }
        w = w.add(1);
        e = e.add(1);
    }

    (changed == 0) as i32
}

pub unsafe fn wipe_exitColorXForm(_width: i32, _height: i32, _ticks: i32) -> i32 {
    0
}

pub unsafe fn wipe_initMelt(width: i32, height: i32, _ticks: i32) -> i32 {
    std::ptr::copy_nonoverlapping(wipe_scr_start, wipe_scr, (width * height) as usize);

    wipe_shittyColMajorXform(wipe_scr_start as *mut i16, width / 2, height);
    wipe_shittyColMajorXform(wipe_scr_end as *mut i16, width / 2, height);

    y = Z_Malloc(
        width * (std::mem::size_of::<i32>() as i32),
        PU_STATIC,
        std::ptr::null_mut(),
    ) as *mut i32;
    *y = -(M_Random() % 16);
    for i in 1..width {
        let r = (M_Random() % 3) - 1;
        *y.add(i as usize) = *y.add((i - 1) as usize) + r;
        if *y.add(i as usize) > 0 {
            *y.add(i as usize) = 0;
        } else if *y.add(i as usize) == -16 {
            *y.add(i as usize) = -15;
        }
    }

    0
}

pub unsafe fn wipe_doMelt(mut width: i32, height: i32, mut ticks: i32) -> i32 {
    let mut done = true_;
    width /= 2;

    while ticks > 0 {
        ticks -= 1;
        for i in 0..width {
            let yi = *y.add(i as usize);
            if yi < 0 {
                *y.add(i as usize) += 1;
                done = false_;
            } else if yi < height {
                let mut dy = if yi < 16 { yi + 1 } else { 8 };
                if yi + dy >= height {
                    dy = height - yi;
                }
                let mut s = (wipe_scr_end as *mut i16).add((i * height + yi) as usize);
                let mut d = (wipe_scr as *mut i16).add((yi * width + i) as usize);
                let mut idx = 0isize;
                for _ in 0..dy {
                    *d.offset(idx) = *s;
                    s = s.add(1);
                    idx += width as isize;
                }
                *y.add(i as usize) += dy;
                let yi2 = *y.add(i as usize);
                s = (wipe_scr_start as *mut i16).add((i * height) as usize);
                d = (wipe_scr as *mut i16).add((yi2 * width + i) as usize);
                idx = 0;
                for _ in 0..(height - yi2) {
                    *d.offset(idx) = *s;
                    s = s.add(1);
                    idx += width as isize;
                }
                done = false_;
            }
        }
    }

    done
}

pub unsafe fn wipe_exitMelt(_width: i32, _height: i32, _ticks: i32) -> i32 {
    Z_Free(y as *mut std::ffi::c_void);
    0
}

pub unsafe fn wipe_StartScreen(_x: i32, _y: i32, _width: i32, _height: i32) -> i32 {
    wipe_scr_start = screens[2];
    I_ReadScreen(wipe_scr_start);
    0
}

pub unsafe fn wipe_EndScreen(x: i32, y_pos: i32, width: i32, height: i32) -> i32 {
    wipe_scr_end = screens[3];
    I_ReadScreen(wipe_scr_end);
    V_DrawBlock(x, y_pos, 0, width, height, wipe_scr_start);
    0
}

pub unsafe fn wipe_ScreenWipe(
    wipeno: i32,
    _x: i32,
    _y: i32,
    width: i32,
    height: i32,
    ticks: i32,
) -> i32 {
    let wipes: [unsafe fn(i32, i32, i32) -> i32; 6] = [
        wipe_initColorXForm,
        wipe_doColorXForm,
        wipe_exitColorXForm,
        wipe_initMelt,
        wipe_doMelt,
        wipe_exitMelt,
    ];

    if go == 0 {
        go = 1;
        wipe_scr = screens[0];
        wipes[(wipeno * 3) as usize](width, height, ticks);
    }

    V_MarkRect(0, 0, width, height);
    let rc = wipes[(wipeno * 3 + 1) as usize](width, height, ticks);

    if rc != 0 {
        go = 0;
        wipes[(wipeno * 3 + 2) as usize](width, height, ticks);
    }

    (go == 0) as i32
}
