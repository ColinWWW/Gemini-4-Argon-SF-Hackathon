// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const MAXWIDTH: usize = 1120;
pub const MAXHEIGHT: usize = 832;
pub const SBARHEIGHT: i32 = 32;

pub static mut viewimage: *mut byte = std::ptr::null_mut();
pub static mut viewwidth: i32 = 0;
pub static mut scaledviewwidth: i32 = 0;
pub static mut viewheight: i32 = 0;
pub static mut viewwindowx: i32 = 0;
pub static mut viewwindowy: i32 = 0;
pub static mut ylookup: [*mut byte; MAXHEIGHT] = [std::ptr::null_mut(); MAXHEIGHT];
pub static mut columnofs: [i32; MAXWIDTH] = [0; MAXWIDTH];

pub static mut translations: [[byte; 256]; 3] = [[0; 256]; 3];

pub static mut dc_colormap: *mut lighttable_t = std::ptr::null_mut();
pub static mut dc_x: i32 = 0;
pub static mut dc_yl: i32 = 0;
pub static mut dc_yh: i32 = 0;
pub static mut dc_iscale: fixed_t = 0;
pub static mut dc_texturemid: fixed_t = 0;
pub static mut dc_source: *mut byte = std::ptr::null_mut();
pub static mut dccount: i32 = 0;

pub unsafe fn R_DrawColumn() {
    let mut count = dc_yh - dc_yl;
    if count < 0 {
        return;
    }

    if (dc_x as u32) >= (SCREENWIDTH as u32) || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }

    let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);
    let fracstep = dc_iscale;
    let mut frac = dc_texturemid.wrapping_add((dc_yl - centery).wrapping_mul(fracstep));

    loop {
        *dest = *dc_colormap.add(*dc_source.add(((frac >> FRACBITS) & 127) as usize) as usize);
        dest = dest.add(SCREENWIDTH as usize);
        frac = frac.wrapping_add(fracstep);
        count -= 1;
        if count < 0 {
            break;
        }
    }
}

pub unsafe fn R_DrawColumnLow() {
    let mut count = dc_yh - dc_yl;
    if count < 0 {
        return;
    }

    if (dc_x as u32) >= (SCREENWIDTH as u32) || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }

    dc_x <<= 1;

    let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);
    let mut dest2 = ylookup[dc_yl as usize].add(columnofs[(dc_x + 1) as usize] as usize);

    let fracstep = dc_iscale;
    let mut frac = dc_texturemid.wrapping_add((dc_yl - centery).wrapping_mul(fracstep));

    loop {
        let val = *dc_colormap.add(*dc_source.add(((frac >> FRACBITS) & 127) as usize) as usize);
        *dest = val;
        *dest2 = val;
        dest = dest.add(SCREENWIDTH as usize);
        dest2 = dest2.add(SCREENWIDTH as usize);
        frac = frac.wrapping_add(fracstep);
        count -= 1;
        if count < 0 {
            break;
        }
    }
}

pub const FUZZTABLE: usize = 50;
pub const FUZZOFF: isize = SCREENWIDTH as isize;

pub static mut fuzzoffset: [isize; FUZZTABLE] = [
    FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
];

pub static mut fuzzpos: usize = 0;

pub unsafe fn R_DrawFuzzColumn() {
    if dc_yl == 0 {
        dc_yl = 1;
    }
    if dc_yh == viewheight - 1 {
        dc_yh = viewheight - 2;
    }

    let mut count = dc_yh - dc_yl;
    if count < 0 {
        return;
    }

    if (dc_x as u32) >= (SCREENWIDTH as u32) || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
        I_Error(&format!(
            "R_DrawFuzzColumn: {} to {} at {}",
            dc_yl, dc_yh, dc_x
        ));
    }

    let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);
    let fracstep = dc_iscale;
    let mut frac = dc_texturemid.wrapping_add((dc_yl - centery).wrapping_mul(fracstep));

    loop {
        let src_pixel = *dest.offset(fuzzoffset[fuzzpos]);
        *dest = *colormaps.add(6 * 256 + (src_pixel as usize));

        fuzzpos += 1;
        if fuzzpos == FUZZTABLE {
            fuzzpos = 0;
        }

        dest = dest.add(SCREENWIDTH as usize);
        frac = frac.wrapping_add(fracstep);
        count -= 1;
        if count < 0 {
            break;
        }
    }
}

pub static mut dc_translation: *mut byte = std::ptr::null_mut();
pub static mut translationtables: *mut byte = std::ptr::null_mut();

pub unsafe fn R_DrawTranslatedColumn() {
    let mut count = dc_yh - dc_yl;
    if count < 0 {
        return;
    }

    if (dc_x as u32) >= (SCREENWIDTH as u32) || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
        I_Error(&format!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
    }

    let mut dest = ylookup[dc_yl as usize].add(columnofs[dc_x as usize] as usize);
    let fracstep = dc_iscale;
    let mut frac = dc_texturemid.wrapping_add((dc_yl - centery).wrapping_mul(fracstep));

    loop {
        let src = *dc_source.add((frac >> FRACBITS) as usize);
        let trans = *dc_translation.add(src as usize);
        *dest = *dc_colormap.add(trans as usize);
        dest = dest.add(SCREENWIDTH as usize);
        frac = frac.wrapping_add(fracstep);
        count -= 1;
        if count < 0 {
            break;
        }
    }
}

pub unsafe fn R_InitTranslationTables() {
    let raw = Z_Malloc(256 * 3 + 255, PU_STATIC, std::ptr::null_mut()) as usize;
    translationtables = ((raw + 255) & !255) as *mut byte;

    for i in 0..256 {
        if i >= 0x70 && i <= 0x7f {
            *translationtables.add(i) = (0x60 + (i & 0xf)) as byte;
            *translationtables.add(i + 256) = (0x40 + (i & 0xf)) as byte;
            *translationtables.add(i + 512) = (0x20 + (i & 0xf)) as byte;
        } else {
            *translationtables.add(i) = i as byte;
            *translationtables.add(i + 256) = i as byte;
            *translationtables.add(i + 512) = i as byte;
        }
    }
}

pub static mut ds_y: i32 = 0;
pub static mut ds_x1: i32 = 0;
pub static mut ds_x2: i32 = 0;
pub static mut ds_colormap: *mut lighttable_t = std::ptr::null_mut();
pub static mut ds_xfrac: fixed_t = 0;
pub static mut ds_yfrac: fixed_t = 0;
pub static mut ds_xstep: fixed_t = 0;
pub static mut ds_ystep: fixed_t = 0;
pub static mut ds_source: *mut byte = std::ptr::null_mut();
pub static mut dscount: i32 = 0;

pub unsafe fn R_DrawSpan() {
    if ds_x2 < ds_x1
        || ds_x1 < 0
        || ds_x2 >= SCREENWIDTH
        || (ds_y as u32) > (SCREENHEIGHT as u32)
    {
        I_Error(&format!("R_DrawSpan: {} to {} at {}", ds_x1, ds_x2, ds_y));
    }

    let mut xfrac = ds_xfrac;
    let mut yfrac = ds_yfrac;
    let mut dest = ylookup[ds_y as usize].add(columnofs[ds_x1 as usize] as usize);
    let mut count = ds_x2 - ds_x1;

    loop {
        let spot = (((yfrac >> (16 - 6)) & (63 * 64)) + ((xfrac >> 16) & 63)) as usize;
        *dest = *ds_colormap.add(*ds_source.add(spot) as usize);
        dest = dest.add(1);
        xfrac = xfrac.wrapping_add(ds_xstep);
        yfrac = yfrac.wrapping_add(ds_ystep);
        count -= 1;
        if count < 0 {
            break;
        }
    }
}

pub unsafe fn R_DrawSpanLow() {
    if ds_x2 < ds_x1
        || ds_x1 < 0
        || ds_x2 >= SCREENWIDTH
        || (ds_y as u32) > (SCREENHEIGHT as u32)
    {
        I_Error(&format!("R_DrawSpan: {} to {} at {}", ds_x1, ds_x2, ds_y));
    }

    let mut xfrac = ds_xfrac;
    let mut yfrac = ds_yfrac;

    let count_orig = ds_x2 - ds_x1;
    ds_x1 <<= 1;
    ds_x2 <<= 1;

    let mut dest = ylookup[ds_y as usize].add(columnofs[ds_x1 as usize] as usize);
    let mut count = count_orig;

    loop {
        let spot = (((yfrac >> (16 - 6)) & (63 * 64)) + ((xfrac >> 16) & 63)) as usize;
        let val = *ds_colormap.add(*ds_source.add(spot) as usize);
        *dest = val;
        dest = dest.add(1);
        *dest = val;
        dest = dest.add(1);

        xfrac = xfrac.wrapping_add(ds_xstep);
        yfrac = yfrac.wrapping_add(ds_ystep);
        count -= 1;
        if count < 0 {
            break;
        }
    }
}

pub unsafe fn R_InitBuffer(width: i32, height: i32) {
    viewwindowx = (SCREENWIDTH - width) >> 1;

    for i in 0..(width as usize) {
        columnofs[i] = viewwindowx + (i as i32);
    }

    if width == SCREENWIDTH {
        viewwindowy = 0;
    } else {
        viewwindowy = (SCREENHEIGHT - SBARHEIGHT - height) >> 1;
    }

    for i in 0..(height as usize) {
        ylookup[i] = screens[0].add((((i as i32) + viewwindowy) * SCREENWIDTH) as usize);
    }
}

pub unsafe fn R_FillBackScreen() {
    if scaledviewwidth == 320 {
        return;
    }

    let name = if gamemode == commercial {
        "GRNROCK"
    } else {
        "FLOOR7_2"
    };

    let src = W_CacheLumpNameStr(name, PU_CACHE) as *mut byte;
    let mut dest = screens[1];

    for y in 0..(SCREENHEIGHT - SBARHEIGHT) {
        for _ in 0..(SCREENWIDTH / 64) {
            std::ptr::copy_nonoverlapping(src.add(((y & 63) << 6) as usize), dest, 64);
            dest = dest.add(64);
        }
        if (SCREENWIDTH & 63) != 0 {
            std::ptr::copy_nonoverlapping(
                src.add(((y & 63) << 6) as usize),
                dest,
                (SCREENWIDTH & 63) as usize,
            );
            dest = dest.add((SCREENWIDTH & 63) as usize);
        }
    }

    let mut patch = W_CacheLumpNameStr("brdr_t", PU_CACHE) as *mut patch_t;
    let mut x = 0;
    while x < scaledviewwidth {
        V_DrawPatch(viewwindowx + x, viewwindowy - 8, 1, patch);
        x += 8;
    }

    patch = W_CacheLumpNameStr("brdr_b", PU_CACHE) as *mut patch_t;
    x = 0;
    while x < scaledviewwidth {
        V_DrawPatch(viewwindowx + x, viewwindowy + viewheight, 1, patch);
        x += 8;
    }

    patch = W_CacheLumpNameStr("brdr_l", PU_CACHE) as *mut patch_t;
    let mut y = 0;
    while y < viewheight {
        V_DrawPatch(viewwindowx - 8, viewwindowy + y, 1, patch);
        y += 8;
    }

    patch = W_CacheLumpNameStr("brdr_r", PU_CACHE) as *mut patch_t;
    y = 0;
    while y < viewheight {
        V_DrawPatch(viewwindowx + scaledviewwidth, viewwindowy + y, 1, patch);
        y += 8;
    }

    V_DrawPatch(
        viewwindowx - 8,
        viewwindowy - 8,
        1,
        W_CacheLumpNameStr("brdr_tl", PU_CACHE) as *mut patch_t,
    );
    V_DrawPatch(
        viewwindowx + scaledviewwidth,
        viewwindowy - 8,
        1,
        W_CacheLumpNameStr("brdr_tr", PU_CACHE) as *mut patch_t,
    );
    V_DrawPatch(
        viewwindowx - 8,
        viewwindowy + viewheight,
        1,
        W_CacheLumpNameStr("brdr_bl", PU_CACHE) as *mut patch_t,
    );
    V_DrawPatch(
        viewwindowx + scaledviewwidth,
        viewwindowy + viewheight,
        1,
        W_CacheLumpNameStr("brdr_br", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn R_VideoErase(ofs: u32, count: i32) {
    std::ptr::copy_nonoverlapping(
        screens[1].add(ofs as usize),
        screens[0].add(ofs as usize),
        count as usize,
    );
}

pub unsafe fn R_DrawViewBorder() {
    if scaledviewwidth == SCREENWIDTH {
        return;
    }

    let top = ((SCREENHEIGHT - SBARHEIGHT) - viewheight) / 2;
    let mut side = (SCREENWIDTH - scaledviewwidth) / 2;

    R_VideoErase(0, top * SCREENWIDTH + side);

    let mut ofs = (viewheight + top) * SCREENWIDTH - side;
    R_VideoErase(ofs as u32, top * SCREENWIDTH + side);

    ofs = top * SCREENWIDTH + SCREENWIDTH - side;
    side <<= 1;

    for _ in 1..viewheight {
        R_VideoErase(ofs as u32, side);
        ofs += SCREENWIDTH;
    }

    V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT - SBARHEIGHT);
}
