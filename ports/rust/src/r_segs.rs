// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdata::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_main::*;
use crate::r_plane::*;
use crate::r_sky::*;
use crate::r_things::*;
use crate::tables::*;

pub static mut segtextured: boolean = 0;
pub static mut markfloor: boolean = 0;
pub static mut markceiling: boolean = 0;
pub static mut maskedtexture: boolean = 0;
pub static mut toptexture: i32 = 0;
pub static mut bottomtexture: i32 = 0;
pub static mut midtexture: i32 = 0;

pub static mut rw_normalangle: angle_t = 0;
pub static mut rw_angle1: i32 = 0;

pub static mut rw_x: i32 = 0;
pub static mut rw_stopx: i32 = 0;
pub static mut rw_centerangle: angle_t = 0;
pub static mut rw_offset: fixed_t = 0;
pub static mut rw_distance: fixed_t = 0;
pub static mut rw_scale: fixed_t = 0;
pub static mut rw_scalestep: fixed_t = 0;
pub static mut rw_midtexturemid: fixed_t = 0;
pub static mut rw_toptexturemid: fixed_t = 0;
pub static mut rw_bottomtexturemid: fixed_t = 0;

pub static mut worldtop: i32 = 0;
pub static mut worldbottom: i32 = 0;
pub static mut worldhigh: i32 = 0;
pub static mut worldlow: i32 = 0;

pub static mut pixhigh: fixed_t = 0;
pub static mut pixlow: fixed_t = 0;
pub static mut pixhighstep: fixed_t = 0;
pub static mut pixlowstep: fixed_t = 0;

pub static mut topfrac: fixed_t = 0;
pub static mut topstep: fixed_t = 0;

pub static mut bottomfrac: fixed_t = 0;
pub static mut bottomstep: fixed_t = 0;

pub static mut walllights: *mut *mut lighttable_t = std::ptr::null_mut();
pub static mut maskedtexturecol: *mut i16 = std::ptr::null_mut();

pub unsafe fn R_RenderMaskedSegRange(ds: *mut drawseg_t, x1: i32, x2: i32) {
    curline = (*ds).curline;
    frontsector = (*curline).frontsector;
    backsector = (*curline).backsector;
    let texnum = *texturetranslation.add((*(*curline).sidedef).midtexture as usize);

    let mut lightnum = (((*frontsector).lightlevel as i32) >> LIGHTSEGSHIFT) + extralight;

    if (*(*curline).v1).y == (*(*curline).v2).y {
        lightnum -= 1;
    } else if (*(*curline).v1).x == (*(*curline).v2).x {
        lightnum += 1;
    }

    if lightnum < 0 {
        walllights = scalelight[0].as_mut_ptr();
    } else if lightnum >= (LIGHTLEVELS as i32) {
        walllights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
    } else {
        walllights = scalelight[lightnum as usize].as_mut_ptr();
    }

    maskedtexturecol = (*ds).maskedtexturecol;

    rw_scalestep = (*ds).scalestep;
    spryscale = (*ds).scale1 + (x1 - (*ds).x1) * rw_scalestep;
    mfloorclip = (*ds).sprbottomclip;
    mceilingclip = (*ds).sprtopclip;

    if ((*(*curline).linedef).flags as i32) & ML_DONTPEGBOTTOM != 0 {
        dc_texturemid = if (*frontsector).floorheight > (*backsector).floorheight {
            (*frontsector).floorheight
        } else {
            (*backsector).floorheight
        };
        dc_texturemid = dc_texturemid + *textureheight.add(texnum as usize) - viewz;
    } else {
        dc_texturemid = if (*frontsector).ceilingheight < (*backsector).ceilingheight {
            (*frontsector).ceilingheight
        } else {
            (*backsector).ceilingheight
        };
        dc_texturemid -= viewz;
    }
    dc_texturemid += (*(*curline).sidedef).rowoffset;

    if !fixedcolormap.is_null() {
        dc_colormap = fixedcolormap;
    }

    dc_x = x1;
    while dc_x <= x2 {
        let mtc = *maskedtexturecol.offset(dc_x as isize);
        if mtc != MAXSHORT {
            if fixedcolormap.is_null() {
                let mut index = (spryscale >> LIGHTSCALESHIFT) as usize;
                if index >= MAXLIGHTSCALE {
                    index = MAXLIGHTSCALE - 1;
                }
                dc_colormap = *walllights.add(index);
            }

            sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);
            dc_iscale = (0xffffffffu32 / (spryscale as u32)) as fixed_t;

            let col = R_GetColumn(texnum, mtc as i32).sub(3) as *mut column_t;
            R_DrawMaskedColumn(col);
            *maskedtexturecol.offset(dc_x as isize) = MAXSHORT;
        }
        spryscale += rw_scalestep;
        dc_x += 1;
    }
}

pub const HEIGHTBITS: i32 = 12;
pub const HEIGHTUNIT: i32 = 1 << HEIGHTBITS;

pub unsafe fn R_RenderSegLoop() {
    let mut texturecolumn = 0;

    while rw_x < rw_stopx {
        let mut yl = (topfrac + HEIGHTUNIT - 1) >> HEIGHTBITS;

        if yl < (ceilingclip[rw_x as usize] as i32) + 1 {
            yl = (ceilingclip[rw_x as usize] as i32) + 1;
        }

        if markceiling != 0 {
            let top = (ceilingclip[rw_x as usize] as i32) + 1;
            let mut bottom = yl - 1;

            if bottom >= (floorclip[rw_x as usize] as i32) {
                bottom = (floorclip[rw_x as usize] as i32) - 1;
            }

            if top <= bottom {
                (*ceilingplane).top[rw_x as usize] = top as byte;
                (*ceilingplane).bottom[rw_x as usize] = bottom as byte;
            }
        }

        let mut yh = bottomfrac >> HEIGHTBITS;

        if yh >= (floorclip[rw_x as usize] as i32) {
            yh = (floorclip[rw_x as usize] as i32) - 1;
        }

        if markfloor != 0 {
            let mut top = yh + 1;
            let bottom = (floorclip[rw_x as usize] as i32) - 1;
            if top <= (ceilingclip[rw_x as usize] as i32) {
                top = (ceilingclip[rw_x as usize] as i32) + 1;
            }
            if top <= bottom {
                (*floorplane).top[rw_x as usize] = top as byte;
                (*floorplane).bottom[rw_x as usize] = bottom as byte;
            }
        }

        if segtextured != 0 {
            let angle =
                (rw_centerangle.wrapping_add(xtoviewangle[rw_x as usize]) >> ANGLETOFINESHIFT)
                    as usize;
            texturecolumn = rw_offset - FixedMul(finetangent[angle], rw_distance);
            texturecolumn >>= FRACBITS;

            let mut index = (rw_scale >> LIGHTSCALESHIFT) as usize;
            if index >= MAXLIGHTSCALE {
                index = MAXLIGHTSCALE - 1;
            }

            dc_colormap = *walllights.add(index);
            dc_x = rw_x;
            dc_iscale = (0xffffffffu32 / (rw_scale as u32)) as fixed_t;
        }

        if midtexture != 0 {
            dc_yl = yl;
            dc_yh = yh;
            dc_texturemid = rw_midtexturemid;
            dc_source = R_GetColumn(midtexture, texturecolumn);
            if let Some(f) = colfunc {
                f();
            }
            ceilingclip[rw_x as usize] = viewheight as i16;
            floorclip[rw_x as usize] = -1;
        } else {
            if toptexture != 0 {
                let mut mid = pixhigh >> HEIGHTBITS;
                pixhigh += pixhighstep;

                if mid >= (floorclip[rw_x as usize] as i32) {
                    mid = (floorclip[rw_x as usize] as i32) - 1;
                }

                if mid >= yl {
                    dc_yl = yl;
                    dc_yh = mid;
                    dc_texturemid = rw_toptexturemid;
                    dc_source = R_GetColumn(toptexture, texturecolumn);
                    if let Some(f) = colfunc {
                        f();
                    }
                    ceilingclip[rw_x as usize] = mid as i16;
                } else {
                    ceilingclip[rw_x as usize] = (yl - 1) as i16;
                }
            } else {
                if markceiling != 0 {
                    ceilingclip[rw_x as usize] = (yl - 1) as i16;
                }
            }

            if bottomtexture != 0 {
                let mut mid = (pixlow + HEIGHTUNIT - 1) >> HEIGHTBITS;
                pixlow += pixlowstep;

                if mid <= (ceilingclip[rw_x as usize] as i32) {
                    mid = (ceilingclip[rw_x as usize] as i32) + 1;
                }

                if mid <= yh {
                    dc_yl = mid;
                    dc_yh = yh;
                    dc_texturemid = rw_bottomtexturemid;
                    dc_source = R_GetColumn(bottomtexture, texturecolumn);
                    if let Some(f) = colfunc {
                        f();
                    }
                    floorclip[rw_x as usize] = mid as i16;
                } else {
                    floorclip[rw_x as usize] = (yh + 1) as i16;
                }
            } else {
                if markfloor != 0 {
                    floorclip[rw_x as usize] = (yh + 1) as i16;
                }
            }

            if maskedtexture != 0 {
                *maskedtexturecol.offset(rw_x as isize) = texturecolumn as i16;
            }
        }

        rw_scale += rw_scalestep;
        topfrac += topstep;
        bottomfrac += bottomstep;
        rw_x += 1;
    }
}

pub unsafe fn R_StoreWallRange(start: i32, stop: i32) {
    if ds_p == drawsegs.as_mut_ptr().add(MAXDRAWSEGS) {
        return;
    }

    if start >= viewwidth || start > stop {
        I_Error(&format!("Bad R_RenderWallRange: {} to {}", start, stop));
    }

    sidedef = (*curline).sidedef;
    linedef = (*curline).linedef;

    (*linedef).flags |= ML_MAPPED as i16;

    rw_normalangle = (*curline).angle.wrapping_add(ANG90);
    let mut offsetangle =
        ((rw_normalangle as i32).wrapping_sub(rw_angle1)).wrapping_abs() as u32;

    if offsetangle > ANG90 {
        offsetangle = ANG90;
    }

    let distangle = ANG90.wrapping_sub(offsetangle);
    let hyp = R_PointToDist((*(*curline).v1).x, (*(*curline).v1).y);
    let sineval = finesine[(distangle >> ANGLETOFINESHIFT) as usize];
    rw_distance = FixedMul(hyp, sineval);

    (*ds_p).x1 = start;
    rw_x = start;
    (*ds_p).x2 = stop;
    (*ds_p).curline = curline;
    rw_stopx = stop + 1;

    rw_scale = R_ScaleFromGlobalAngle(viewangle.wrapping_add(xtoviewangle[start as usize]));
    (*ds_p).scale1 = rw_scale;

    if stop > start {
        (*ds_p).scale2 =
            R_ScaleFromGlobalAngle(viewangle.wrapping_add(xtoviewangle[stop as usize]));
        rw_scalestep = ((*ds_p).scale2 - rw_scale) / (stop - start);
        (*ds_p).scalestep = rw_scalestep;
    } else {
        (*ds_p).scale2 = (*ds_p).scale1;
    }

    worldtop = (*frontsector).ceilingheight - viewz;
    worldbottom = (*frontsector).floorheight - viewz;

    midtexture = 0;
    toptexture = 0;
    bottomtexture = 0;
    maskedtexture = 0;
    (*ds_p).maskedtexturecol = std::ptr::null_mut();

    if backsector.is_null() {
        midtexture = *texturetranslation.add((*sidedef).midtexture as usize);
        markfloor = true_;
        markceiling = true_;
        if ((*linedef).flags as i32) & ML_DONTPEGBOTTOM != 0 {
            let vtop =
                (*frontsector).floorheight + *textureheight.add((*sidedef).midtexture as usize);
            rw_midtexturemid = vtop - viewz;
        } else {
            rw_midtexturemid = worldtop;
        }
        rw_midtexturemid += (*sidedef).rowoffset;

        (*ds_p).silhouette = SIL_BOTH;
        (*ds_p).sprtopclip = screenheightarray.as_mut_ptr();
        (*ds_p).sprbottomclip = negonearray.as_mut_ptr();
        (*ds_p).bsilheight = MAXINT;
        (*ds_p).tsilheight = MININT;
    } else {
        (*ds_p).sprtopclip = std::ptr::null_mut();
        (*ds_p).sprbottomclip = std::ptr::null_mut();
        (*ds_p).silhouette = 0;

        if (*frontsector).floorheight > (*backsector).floorheight {
            (*ds_p).silhouette = SIL_BOTTOM;
            (*ds_p).bsilheight = (*frontsector).floorheight;
        } else if (*backsector).floorheight > viewz {
            (*ds_p).silhouette = SIL_BOTTOM;
            (*ds_p).bsilheight = MAXINT;
        }

        if (*frontsector).ceilingheight < (*backsector).ceilingheight {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = (*frontsector).ceilingheight;
        } else if (*backsector).ceilingheight < viewz {
            (*ds_p).silhouette |= SIL_TOP;
            (*ds_p).tsilheight = MININT;
        }

        if (*backsector).ceilingheight <= (*frontsector).floorheight {
            (*ds_p).sprbottomclip = negonearray.as_mut_ptr();
            (*ds_p).bsilheight = MAXINT;
            (*ds_p).silhouette |= SIL_BOTTOM;
        }

        if (*backsector).floorheight >= (*frontsector).ceilingheight {
            (*ds_p).sprtopclip = screenheightarray.as_mut_ptr();
            (*ds_p).tsilheight = MININT;
            (*ds_p).silhouette |= SIL_TOP;
        }

        worldhigh = (*backsector).ceilingheight - viewz;
        worldlow = (*backsector).floorheight - viewz;

        if ((*frontsector).ceilingpic as i32) == skyflatnum
            && ((*backsector).ceilingpic as i32) == skyflatnum
        {
            worldtop = worldhigh;
        }

        if worldlow != worldbottom
            || (*backsector).floorpic != (*frontsector).floorpic
            || (*backsector).lightlevel != (*frontsector).lightlevel
        {
            markfloor = true_;
        } else {
            markfloor = false_;
        }

        if worldhigh != worldtop
            || (*backsector).ceilingpic != (*frontsector).ceilingpic
            || (*backsector).lightlevel != (*frontsector).lightlevel
        {
            markceiling = true_;
        } else {
            markceiling = false_;
        }

        if (*backsector).ceilingheight <= (*frontsector).floorheight
            || (*backsector).floorheight >= (*frontsector).ceilingheight
        {
            markceiling = true_;
            markfloor = true_;
        }

        if worldhigh < worldtop {
            toptexture = *texturetranslation.add((*sidedef).toptexture as usize);
            if ((*linedef).flags as i32) & ML_DONTPEGTOP != 0 {
                rw_toptexturemid = worldtop;
            } else {
                let vtop = (*backsector).ceilingheight
                    + *textureheight.add((*sidedef).toptexture as usize);
                rw_toptexturemid = vtop - viewz;
            }
        }
        if worldlow > worldbottom {
            bottomtexture = *texturetranslation.add((*sidedef).bottomtexture as usize);
            if ((*linedef).flags as i32) & ML_DONTPEGBOTTOM != 0 {
                rw_bottomtexturemid = worldtop;
            } else {
                rw_bottomtexturemid = worldlow;
            }
        }
        rw_toptexturemid += (*sidedef).rowoffset;
        rw_bottomtexturemid += (*sidedef).rowoffset;

        if (*sidedef).midtexture != 0 {
            maskedtexture = true_;
            maskedtexturecol = lastopening.offset(-(rw_x as isize));
            (*ds_p).maskedtexturecol = maskedtexturecol;
            lastopening = lastopening.add((rw_stopx - rw_x) as usize);
        }
    }

    segtextured = midtexture | toptexture | bottomtexture | maskedtexture;

    if segtextured != 0 {
        let mut offsetangle2 = rw_normalangle.wrapping_sub(rw_angle1 as u32);
        if offsetangle2 > ANG180 {
            offsetangle2 = 0u32.wrapping_sub(offsetangle2);
        }
        if offsetangle2 > ANG90 {
            offsetangle2 = ANG90;
        }

        let sineval2 = finesine[(offsetangle2 >> ANGLETOFINESHIFT) as usize];
        rw_offset = FixedMul(hyp, sineval2);

        if rw_normalangle.wrapping_sub(rw_angle1 as u32) < ANG180 {
            rw_offset = -rw_offset;
        }

        rw_offset += (*sidedef).textureoffset + (*curline).offset;
        rw_centerangle = ANG90.wrapping_add(viewangle).wrapping_sub(rw_normalangle);

        if fixedcolormap.is_null() {
            let mut lightnum = (((*frontsector).lightlevel as i32) >> LIGHTSEGSHIFT) + extralight;

            if (*(*curline).v1).y == (*(*curline).v2).y {
                lightnum -= 1;
            } else if (*(*curline).v1).x == (*(*curline).v2).x {
                lightnum += 1;
            }

            if lightnum < 0 {
                walllights = scalelight[0].as_mut_ptr();
            } else if lightnum >= (LIGHTLEVELS as i32) {
                walllights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
            } else {
                walllights = scalelight[lightnum as usize].as_mut_ptr();
            }
        }
    }

    if (*frontsector).floorheight >= viewz {
        markfloor = false_;
    }

    if (*frontsector).ceilingheight <= viewz && ((*frontsector).ceilingpic as i32) != skyflatnum {
        markceiling = false_;
    }

    worldtop >>= 4;
    worldbottom >>= 4;

    topstep = -FixedMul(rw_scalestep, worldtop);
    topfrac = (centeryfrac >> 4) - FixedMul(worldtop, rw_scale);

    bottomstep = -FixedMul(rw_scalestep, worldbottom);
    bottomfrac = (centeryfrac >> 4) - FixedMul(worldbottom, rw_scale);

    if !backsector.is_null() {
        worldhigh >>= 4;
        worldlow >>= 4;

        if worldhigh < worldtop {
            pixhigh = (centeryfrac >> 4) - FixedMul(worldhigh, rw_scale);
            pixhighstep = -FixedMul(rw_scalestep, worldhigh);
        }

        if worldlow > worldbottom {
            pixlow = (centeryfrac >> 4) - FixedMul(worldlow, rw_scale);
            pixlowstep = -FixedMul(rw_scalestep, worldlow);
        }
    }

    if markceiling != 0 {
        ceilingplane = R_CheckPlane(ceilingplane, rw_x, rw_stopx - 1);
    }

    if markfloor != 0 {
        floorplane = R_CheckPlane(floorplane, rw_x, rw_stopx - 1);
    }

    R_RenderSegLoop();

    if (((*ds_p).silhouette & SIL_TOP != 0) || maskedtexture != 0)
        && (*ds_p).sprtopclip.is_null()
    {
        std::ptr::copy_nonoverlapping(
            ceilingclip.as_ptr().add(start as usize),
            lastopening,
            (rw_stopx - start) as usize,
        );
        (*ds_p).sprtopclip = lastopening.offset(-(start as isize));
        lastopening = lastopening.add((rw_stopx - start) as usize);
    }

    if (((*ds_p).silhouette & SIL_BOTTOM != 0) || maskedtexture != 0)
        && (*ds_p).sprbottomclip.is_null()
    {
        std::ptr::copy_nonoverlapping(
            floorclip.as_ptr().add(start as usize),
            lastopening,
            (rw_stopx - start) as usize,
        );
        (*ds_p).sprbottomclip = lastopening.offset(-(start as isize));
        lastopening = lastopening.add((rw_stopx - start) as usize);
    }

    if maskedtexture != 0 && ((*ds_p).silhouette & SIL_TOP == 0) {
        (*ds_p).silhouette |= SIL_TOP;
        (*ds_p).tsilheight = MININT;
    }
    if maskedtexture != 0 && ((*ds_p).silhouette & SIL_BOTTOM == 0) {
        (*ds_p).silhouette |= SIL_BOTTOM;
        (*ds_p).bsilheight = MAXINT;
    }
    ds_p = ds_p.add(1);
}
