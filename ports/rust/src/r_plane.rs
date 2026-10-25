// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_main::*;
use crate::r_sky::*;
use crate::r_things::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub type planefunction_t = Option<unsafe fn(i32, i32)>;

pub static mut floorfunc: planefunction_t = None;
pub static mut ceilingfunc: planefunction_t = None;

pub const MAXVISPLANES: usize = 128;
pub static mut visplanes: [visplane_t; MAXVISPLANES] = [visplane_t {
    height: 0,
    picnum: 0,
    lightlevel: 0,
    minx: 0,
    maxx: 0,
    pad1: 0,
    top: [0; SCREENWIDTH as usize],
    pad2: 0,
    pad3: 0,
    bottom: [0; SCREENWIDTH as usize],
    pad4: 0,
}; MAXVISPLANES];
pub static mut lastvisplane: *mut visplane_t = std::ptr::null_mut();
pub static mut floorplane: *mut visplane_t = std::ptr::null_mut();
pub static mut ceilingplane: *mut visplane_t = std::ptr::null_mut();

pub const MAXOPENINGS: usize = (SCREENWIDTH as usize) * 64;
pub static mut openings: [i16; MAXOPENINGS] = [0; MAXOPENINGS];
pub static mut lastopening: *mut i16 = std::ptr::null_mut();

pub static mut floorclip: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];
pub static mut ceilingclip: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];

pub static mut spanstart: [i32; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];
pub static mut spanstop: [i32; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];

pub static mut planezlight: *mut *mut lighttable_t = std::ptr::null_mut();
pub static mut planeheight: fixed_t = 0;

pub static mut yslope: [fixed_t; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];
pub static mut distscale: [fixed_t; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];
pub static mut basexscale: fixed_t = 0;
pub static mut baseyscale: fixed_t = 0;

pub static mut cachedheight: [fixed_t; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];
pub static mut cacheddistance: [fixed_t; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];
pub static mut cachedxstep: [fixed_t; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];
pub static mut cachedystep: [fixed_t; SCREENHEIGHT as usize] = [0; SCREENHEIGHT as usize];

pub unsafe fn R_InitPlanes() {}

pub unsafe fn R_MapPlane(y: i32, x1: i32, x2: i32) {
    if x2 < x1 || x1 < 0 || x2 >= viewwidth || (y as u32) > (viewheight as u32) {
        I_Error(&format!("R_MapPlane: {}, {} at {}", x1, x2, y));
    }

    let distance;
    let yu = y as usize;
    if planeheight != cachedheight[yu] {
        cachedheight[yu] = planeheight;
        distance = FixedMul(planeheight, yslope[yu]);
        cacheddistance[yu] = distance;
        ds_xstep = FixedMul(distance, basexscale);
        cachedxstep[yu] = ds_xstep;
        ds_ystep = FixedMul(distance, baseyscale);
        cachedystep[yu] = ds_ystep;
    } else {
        distance = cacheddistance[yu];
        ds_xstep = cachedxstep[yu];
        ds_ystep = cachedystep[yu];
    }

    let length = FixedMul(distance, distscale[x1 as usize]);
    let angle = (viewangle.wrapping_add(xtoviewangle[x1 as usize]) >> ANGLETOFINESHIFT) as usize;
    ds_xfrac = viewx.wrapping_add(FixedMul(finecosine(angle), length));
    ds_yfrac = (-viewy).wrapping_sub(FixedMul(finesine[angle], length));

    if !fixedcolormap.is_null() {
        ds_colormap = fixedcolormap;
    } else {
        let mut index = (distance >> LIGHTZSHIFT) as usize;
        if index >= MAXLIGHTZ {
            index = MAXLIGHTZ - 1;
        }
        ds_colormap = *planezlight.add(index);
    }

    ds_y = y;
    ds_x1 = x1;
    ds_x2 = x2;

    if let Some(f) = spanfunc {
        f();
    }
}

pub unsafe fn R_ClearPlanes() {
    for i in 0..(viewwidth as usize) {
        floorclip[i] = viewheight as i16;
        ceilingclip[i] = -1;
    }

    lastvisplane = visplanes.as_mut_ptr();
    lastopening = openings.as_mut_ptr();

    cachedheight = [0; SCREENHEIGHT as usize];

    let angle = (viewangle.wrapping_sub(ANG90) >> ANGLETOFINESHIFT) as usize;
    basexscale = FixedDiv(finecosine(angle), centerxfrac);
    baseyscale = -FixedDiv(finesine[angle], centerxfrac);
}

pub unsafe fn R_FindPlane(mut height: fixed_t, picnum: i32, mut lightlevel: i32) -> *mut visplane_t {
    if picnum == skyflatnum {
        height = 0;
        lightlevel = 0;
    }

    let mut check = visplanes.as_mut_ptr();
    while (check as usize) < (lastvisplane as usize) {
        if height == (*check).height
            && picnum == (*check).picnum
            && lightlevel == (*check).lightlevel
        {
            break;
        }
        check = check.add(1);
    }

    if (check as usize) < (lastvisplane as usize) {
        return check;
    }

    let idx = (lastvisplane as usize - visplanes.as_ptr() as usize)
        / std::mem::size_of::<visplane_t>();
    if idx == MAXVISPLANES {
        I_Error("R_FindPlane: no more visplanes");
    }

    lastvisplane = lastvisplane.add(1);

    (*check).height = height;
    (*check).picnum = picnum;
    (*check).lightlevel = lightlevel;
    (*check).minx = SCREENWIDTH;
    (*check).maxx = -1;
    (*check).top = [0xff; SCREENWIDTH as usize];

    check
}

pub unsafe fn R_CheckPlane(mut pl: *mut visplane_t, start: i32, stop: i32) -> *mut visplane_t {
    let intrl;
    let intrh;
    let unionl;
    let unionh;

    if start < (*pl).minx {
        intrl = (*pl).minx;
        unionl = start;
    } else {
        unionl = (*pl).minx;
        intrl = start;
    }

    if stop > (*pl).maxx {
        intrh = (*pl).maxx;
        unionh = stop;
    } else {
        unionh = (*pl).maxx;
        intrh = stop;
    }

    let mut x = intrl;
    while x <= intrh {
        if (*pl).top[x as usize] != 0xff {
            break;
        }
        x += 1;
    }

    if x > intrh {
        (*pl).minx = unionl;
        (*pl).maxx = unionh;
        return pl;
    }

    (*lastvisplane).height = (*pl).height;
    (*lastvisplane).picnum = (*pl).picnum;
    (*lastvisplane).lightlevel = (*pl).lightlevel;

    pl = lastvisplane;
    lastvisplane = lastvisplane.add(1);
    (*pl).minx = start;
    (*pl).maxx = stop;
    (*pl).top = [0xff; SCREENWIDTH as usize];

    pl
}

pub unsafe fn R_MakeSpans(x: i32, mut t1: i32, mut b1: i32, mut t2: i32, mut b2: i32) {
    while t1 < t2 && t1 <= b1 {
        R_MapPlane(t1, spanstart[t1 as usize], x - 1);
        t1 += 1;
    }
    while b1 > b2 && b1 >= t1 {
        R_MapPlane(b1, spanstart[b1 as usize], x - 1);
        b1 -= 1;
    }

    while t2 < t1 && t2 <= b2 {
        spanstart[t2 as usize] = x;
        t2 += 1;
    }
    while b2 > b1 && b2 >= t2 {
        spanstart[b2 as usize] = x;
        b2 -= 1;
    }
}

pub unsafe fn R_DrawPlanes() {
    let ds_diff =
        (ds_p as usize - drawsegs.as_ptr() as usize) / std::mem::size_of::<drawseg_t>();
    if ds_diff > MAXDRAWSEGS {
        I_Error(&format!("R_DrawPlanes: drawsegs overflow ({})", ds_diff));
    }

    let vp_diff = (lastvisplane as usize - visplanes.as_ptr() as usize)
        / std::mem::size_of::<visplane_t>();
    if vp_diff > MAXVISPLANES {
        I_Error(&format!("R_DrawPlanes: visplane overflow ({})", vp_diff));
    }

    let op_diff = (lastopening as usize - openings.as_ptr() as usize) / std::mem::size_of::<i16>();
    if op_diff > MAXOPENINGS {
        I_Error(&format!("R_DrawPlanes: opening overflow ({})", op_diff));
    }

    let mut pl = visplanes.as_mut_ptr();
    while (pl as usize) < (lastvisplane as usize) {
        if (*pl).minx > (*pl).maxx {
            pl = pl.add(1);
            continue;
        }

        if (*pl).picnum == skyflatnum {
            dc_iscale = pspriteiscale >> detailshift;
            dc_colormap = colormaps;
            dc_texturemid = skytexturemid;

            for x in (*pl).minx..=(*pl).maxx {
                dc_yl = (*pl).top[x as usize] as i32;
                dc_yh = (*pl).bottom[x as usize] as i32;

                if dc_yl <= dc_yh {
                    let angle = (viewangle.wrapping_add(xtoviewangle[x as usize])
                        >> ANGLETOSKYSHIFT) as i32;
                    dc_x = x;
                    dc_source = R_GetColumn(skytexture, angle);
                    if let Some(f) = colfunc {
                        f();
                    }
                }
            }
            pl = pl.add(1);
            continue;
        }

        ds_source = W_CacheLumpNum(
            firstflat + *flattranslation.add((*pl).picnum as usize),
            PU_STATIC,
        ) as *mut byte;

        planeheight = ((*pl).height - viewz).wrapping_abs();
        let mut light = ((*pl).lightlevel >> LIGHTSEGSHIFT) + extralight;

        if light >= (LIGHTLEVELS as i32) {
            light = (LIGHTLEVELS as i32) - 1;
        }
        if light < 0 {
            light = 0;
        }

        planezlight = zlight[light as usize].as_mut_ptr();

        // Pad access: top[maxx+1] and top[minx-1]
        // Note: in visplane_t, pad1 is right before top[0], and pad2 is right after top[319]!
        let top_base = (*pl).top.as_mut_ptr();
        *top_base.offset(((*pl).maxx + 1) as isize) = 0xff;
        *top_base.offset(((*pl).minx - 1) as isize) = 0xff;

        let bottom_base = (*pl).bottom.as_mut_ptr();
        let stop = (*pl).maxx + 1;

        for x in (*pl).minx..=stop {
            let t1 = *top_base.offset((x - 1) as isize) as i32;
            let b1 = *bottom_base.offset((x - 1) as isize) as i32;
            let t2 = *top_base.offset(x as isize) as i32;
            let b2 = *bottom_base.offset(x as isize) as i32;
            R_MakeSpans(x, t1, b1, t2, b2);
        }

        Z_ChangeTag(ds_source as *mut std::ffi::c_void, PU_CACHE);

        pl = pl.add(1);
    }
}
