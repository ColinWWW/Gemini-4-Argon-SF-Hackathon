// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_bsp::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_main::*;
use crate::r_segs::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const MAXVISSPRITES: usize = 128;
pub const MINZ: fixed_t = FRACUNIT * 4;
pub const BASEYCENTER: i32 = 100;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct maskdraw_t {
    pub x1: i32,
    pub x2: i32,
    pub column: i32,
    pub topclip: i32,
    pub bottomclip: i32,
}

pub static mut pspritescale: fixed_t = 0;
pub static mut pspriteiscale: fixed_t = 0;

pub static mut spritelights: *mut *mut lighttable_t = std::ptr::null_mut();

pub static mut negonearray: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];
pub static mut screenheightarray: [i16; SCREENWIDTH as usize] = [0; SCREENWIDTH as usize];

pub static mut sprites: *mut spritedef_t = std::ptr::null_mut();
pub static mut numsprites: i32 = 0;

pub static mut sprtemp: [spriteframe_t; 29] = [spriteframe_t {
    rotate: 0,
    lump: [0; 8],
    flip: [0; 8],
}; 29];
pub static mut maxframe: i32 = 0;
pub static mut spritename: &'static str = "";

pub unsafe fn R_InstallSpriteLump(lump: i32, frame: u32, mut rotation: u32, flipped: boolean) {
    if frame >= 29 || rotation > 8 {
        I_Error(&format!(
            "R_InstallSpriteLump: Bad frame characters in lump {}",
            lump
        ));
    }

    if (frame as i32) > maxframe {
        maxframe = frame as i32;
    }

    let f = frame as usize;
    if rotation == 0 {
        if sprtemp[f].rotate == false_ {
            I_Error(&format!(
                "R_InitSprites: Sprite {} frame {} has multip rot=0 lump",
                spritename,
                (b'A' + (frame as u8)) as char
            ));
        }

        if sprtemp[f].rotate == true_ {
            I_Error(&format!(
                "R_InitSprites: Sprite {} frame {} has rotations and a rot=0 lump",
                spritename,
                (b'A' + (frame as u8)) as char
            ));
        }

        sprtemp[f].rotate = false_;
        for r in 0..8 {
            sprtemp[f].lump[r] = (lump - firstspritelump) as i16;
            sprtemp[f].flip[r] = flipped as byte;
        }
        return;
    }

    if sprtemp[f].rotate == false_ {
        I_Error(&format!(
            "R_InitSprites: Sprite {} frame {} has rotations and a rot=0 lump",
            spritename,
            (b'A' + (frame as u8)) as char
        ));
    }

    sprtemp[f].rotate = true_;

    rotation -= 1;
    let rot = rotation as usize;
    if sprtemp[f].lump[rot] != -1 {
        I_Error(&format!(
            "R_InitSprites: Sprite {} : {} : {} has two lumps mapped to it",
            spritename,
            (b'A' + (frame as u8)) as char,
            (b'1' + (rotation as u8)) as char
        ));
    }

    sprtemp[f].lump[rot] = (lump - firstspritelump) as i16;
    sprtemp[f].flip[rot] = flipped as byte;
}

pub unsafe fn R_InitSpriteDefs(namelist: &[&'static str]) {
    numsprites = namelist.len() as i32;
    if numsprites == 0 {
        return;
    }

    sprites = Z_Malloc(
        numsprites * (std::mem::size_of::<spritedef_t>() as i32),
        PU_STATIC,
        std::ptr::null_mut(),
    ) as *mut spritedef_t;

    let start = firstspritelump - 1;
    let end = lastspritelump + 1;

    for i in 0..(numsprites as usize) {
        spritename = namelist[i];
        std::ptr::write_bytes(
            sprtemp.as_mut_ptr() as *mut u8,
            0xff,
            std::mem::size_of::<[spriteframe_t; 29]>(),
        );

        maxframe = -1;
        let nb = namelist[i].as_bytes();

        for l in (start + 1)..end {
            let lname = &(*lumpinfo.add(l as usize)).name;
            if lname[0] == nb[0] && lname[1] == nb[1] && lname[2] == nb[2] && lname[3] == nb[3] {
                let frame = (lname[4].wrapping_sub(b'A')) as u32;
                let rotation = (lname[5].wrapping_sub(b'0')) as u32;

                let patched = if modifiedgame != 0 {
                    W_GetNumForName(lname.as_ptr())
                } else {
                    l
                };

                R_InstallSpriteLump(patched, frame, rotation, false_);

                if lname[6] != 0 {
                    let frame2 = (lname[6].wrapping_sub(b'A')) as u32;
                    let rotation2 = (lname[7].wrapping_sub(b'0')) as u32;
                    R_InstallSpriteLump(l, frame2, rotation2, true_);
                }
            }
        }

        if maxframe == -1 {
            (*sprites.add(i)).numframes = 0;
            continue;
        }

        maxframe += 1;

        for frame in 0..(maxframe as usize) {
            match sprtemp[frame].rotate {
                -1 => {
                    I_Error(&format!(
                        "R_InitSprites: No patches found for {} frame {}",
                        namelist[i],
                        (b'A' + (frame as u8)) as char
                    ));
                }
                0 => {}
                1 => {
                    for rotation in 0..8 {
                        if sprtemp[frame].lump[rotation] == -1 {
                            I_Error(&format!(
                                "R_InitSprites: Sprite {} frame {} is missing rotations",
                                namelist[i],
                                (b'A' + (frame as u8)) as char
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        (*sprites.add(i)).numframes = maxframe;
        (*sprites.add(i)).spriteframes = Z_Malloc(
            maxframe * (std::mem::size_of::<spriteframe_t>() as i32),
            PU_STATIC,
            std::ptr::null_mut(),
        ) as *mut spriteframe_t;
        std::ptr::copy_nonoverlapping(
            sprtemp.as_ptr(),
            (*sprites.add(i)).spriteframes,
            maxframe as usize,
        );
    }
}

pub static mut vissprites: [vissprite_t; MAXVISSPRITES] = [vissprite_t {
    prev: std::ptr::null_mut(),
    next: std::ptr::null_mut(),
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: std::ptr::null_mut(),
    mobjflags: 0,
}; MAXVISSPRITES];
pub static mut vissprite_p: *mut vissprite_t = std::ptr::null_mut();
pub static mut newvissprite: i32 = 0;

pub unsafe fn R_InitSprites(namelist: &[&'static str]) {
    for i in 0..(SCREENWIDTH as usize) {
        negonearray[i] = -1;
    }
    R_InitSpriteDefs(namelist);
}

pub unsafe fn R_ClearSprites() {
    vissprite_p = vissprites.as_mut_ptr();
}

pub static mut overflowsprite: vissprite_t = vissprite_t {
    prev: std::ptr::null_mut(),
    next: std::ptr::null_mut(),
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: std::ptr::null_mut(),
    mobjflags: 0,
};

pub unsafe fn R_NewVisSprite() -> *mut vissprite_t {
    if vissprite_p == vissprites.as_mut_ptr().add(MAXVISSPRITES) {
        return &mut overflowsprite;
    }
    vissprite_p = vissprite_p.add(1);
    vissprite_p.sub(1)
}

pub static mut mfloorclip: *mut i16 = std::ptr::null_mut();
pub static mut mceilingclip: *mut i16 = std::ptr::null_mut();

pub static mut spryscale: fixed_t = 0;
pub static mut sprtopscreen: fixed_t = 0;

pub unsafe fn R_DrawMaskedColumn(mut column: *mut column_t) {
    let basetexturemid = dc_texturemid;

    while (*column).topdelta != 0xff {
        let topscreen = sprtopscreen + spryscale * ((*column).topdelta as i32);
        let bottomscreen = topscreen + spryscale * ((*column).length as i32);

        dc_yl = (topscreen + FRACUNIT - 1) >> FRACBITS;
        dc_yh = (bottomscreen - 1) >> FRACBITS;

        if dc_yh >= (*mfloorclip.offset(dc_x as isize) as i32) {
            dc_yh = (*mfloorclip.offset(dc_x as isize) as i32) - 1;
        }
        if dc_yl <= (*mceilingclip.offset(dc_x as isize) as i32) {
            dc_yl = (*mceilingclip.offset(dc_x as isize) as i32) + 1;
        }

        if dc_yl <= dc_yh {
            dc_source = (column as *mut byte).add(3);
            dc_texturemid = basetexturemid - (((*column).topdelta as i32) << FRACBITS);
            if let Some(f) = colfunc {
                f();
            }
        }
        column = (column as *mut byte).add(((*column).length as usize) + 4) as *mut column_t;
    }

    dc_texturemid = basetexturemid;
}

pub unsafe fn R_DrawVisSprite(vis: *mut vissprite_t, _x1: i32, _x2: i32) {
    let patch = W_CacheLumpNum((*vis).patch + firstspritelump, PU_CACHE) as *mut patch_t;

    dc_colormap = (*vis).colormap;

    if dc_colormap.is_null() {
        colfunc = fuzzcolfunc;
    } else if (*vis).mobjflags & MF_TRANSLATION != 0 {
        colfunc = Some(R_DrawTranslatedColumn);
        dc_translation = translationtables
            .offset(-256 + (((*vis).mobjflags & MF_TRANSLATION) >> (MF_TRANSSHIFT - 8)) as isize);
    }

    dc_iscale = (*vis).xiscale.wrapping_abs() >> detailshift;
    dc_texturemid = (*vis).texturemid;
    let mut frac = (*vis).startfrac;
    spryscale = (*vis).scale;
    sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);

    let colofs_ptr = (*patch).columnofs.as_ptr();
    dc_x = (*vis).x1;
    while dc_x <= (*vis).x2 {
        let texturecolumn = frac >> FRACBITS;
        if texturecolumn < 0 || texturecolumn >= (SHORT((*patch).width) as i32) {
            I_Error("R_DrawSpriteRange: bad texturecolumn");
        }
        let column = (patch as *mut byte).add(LONG(*colofs_ptr.add(texturecolumn as usize)) as usize)
            as *mut column_t;
        R_DrawMaskedColumn(column);

        dc_x += 1;
        frac += (*vis).xiscale;
    }

    colfunc = basecolfunc;
}

pub unsafe fn R_ProjectSprite(thing: *mut mobj_t) {
    let tr_x = (*thing).x - viewx;
    let tr_y = (*thing).y - viewy;

    let mut gxt = FixedMul(tr_x, viewcos);
    let mut gyt = -FixedMul(tr_y, viewsin);

    let tz = gxt - gyt;

    if tz < MINZ {
        return;
    }

    let xscale = FixedDiv(projection, tz);

    gxt = -FixedMul(tr_x, viewsin);
    gyt = FixedMul(tr_y, viewcos);
    let mut tx = -(gyt + gxt);

    if tx.wrapping_abs() > (tz << 2) {
        return;
    }

    if ((*thing).sprite as u32) >= (numsprites as u32) {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite number {} ",
            (*thing).sprite
        ));
    }

    let sprdef = sprites.add((*thing).sprite as usize);
    if ((*thing).frame & FF_FRAMEMASK) >= (*sprdef).numframes {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite frame {} : {} ",
            (*thing).sprite,
            (*thing).frame
        ));
    }

    let sprframe = (*sprdef)
        .spriteframes
        .add(((*thing).frame & FF_FRAMEMASK) as usize);

    let lump;
    let flip;

    if (*sprframe).rotate != 0 {
        let ang = R_PointToAngle((*thing).x, (*thing).y);
        let rot = (ang
            .wrapping_sub((*thing).angle)
            .wrapping_add((ANG45 / 2) * 9))
            >> 29;
        lump = (*sprframe).lump[rot as usize] as i32;
        flip = (*sprframe).flip[rot as usize] as boolean;
    } else {
        lump = (*sprframe).lump[0] as i32;
        flip = (*sprframe).flip[0] as boolean;
    }

    tx -= *spriteoffset.add(lump as usize);
    let x1 = (centerxfrac + FixedMul(tx, xscale)) >> FRACBITS;

    if x1 > viewwidth {
        return;
    }

    tx += *spritewidth.add(lump as usize);
    let x2 = ((centerxfrac + FixedMul(tx, xscale)) >> FRACBITS) - 1;

    if x2 < 0 {
        return;
    }

    let vis = R_NewVisSprite();
    (*vis).mobjflags = (*thing).flags;
    (*vis).scale = xscale << detailshift;
    (*vis).gx = (*thing).x;
    (*vis).gy = (*thing).y;
    (*vis).gz = (*thing).z;
    (*vis).gzt = (*thing).z + *spritetopoffset.add(lump as usize);
    (*vis).texturemid = (*vis).gzt - viewz;
    (*vis).x1 = if x1 < 0 { 0 } else { x1 };
    (*vis).x2 = if x2 >= viewwidth { viewwidth - 1 } else { x2 };
    let iscale = FixedDiv(FRACUNIT, xscale);

    if flip != 0 {
        (*vis).startfrac = *spritewidth.add(lump as usize) - 1;
        (*vis).xiscale = -iscale;
    } else {
        (*vis).startfrac = 0;
        (*vis).xiscale = iscale;
    }

    if (*vis).x1 > x1 {
        (*vis).startfrac += (*vis).xiscale * ((*vis).x1 - x1);
    }
    (*vis).patch = lump;

    if (*thing).flags & MF_SHADOW != 0 {
        (*vis).colormap = std::ptr::null_mut();
    } else if !fixedcolormap.is_null() {
        (*vis).colormap = fixedcolormap;
    } else if (*thing).frame & FF_FULLBRIGHT != 0 {
        (*vis).colormap = colormaps;
    } else {
        let mut index = (xscale >> (LIGHTSCALESHIFT - detailshift)) as usize;
        if index >= MAXLIGHTSCALE {
            index = MAXLIGHTSCALE - 1;
        }
        (*vis).colormap = *spritelights.add(index);
    }
}

pub unsafe fn R_AddSprites(sec: *mut sector_t) {
    if (*sec).validcount == validcount {
        return;
    }

    (*sec).validcount = validcount;

    let mut lightnum = (((*sec).lightlevel as i32) >> LIGHTSEGSHIFT) + extralight;

    if lightnum < 0 {
        spritelights = scalelight[0].as_mut_ptr();
    } else if lightnum >= (LIGHTLEVELS as i32) {
        spritelights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
    } else {
        spritelights = scalelight[lightnum as usize].as_mut_ptr();
    }

    let mut thing = (*sec).thinglist;
    while !thing.is_null() {
        R_ProjectSprite(thing);
        thing = (*thing).snext;
    }
}

pub unsafe fn R_DrawPSprite(psp: *mut pspdef_t) {
    if ((*(*psp).state).sprite as u32) >= (numsprites as u32) {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite number {} ",
            (*(*psp).state).sprite
        ));
    }

    let sprdef = sprites.add((*(*psp).state).sprite as usize);
    if ((*(*psp).state).frame & FF_FRAMEMASK) >= (*sprdef).numframes {
        I_Error(&format!(
            "R_ProjectSprite: invalid sprite frame {} : {} ",
            (*(*psp).state).sprite,
            (*(*psp).state).frame
        ));
    }

    let sprframe = (*sprdef)
        .spriteframes
        .add(((*(*psp).state).frame & FF_FRAMEMASK) as usize);

    let lump = (*sprframe).lump[0] as i32;
    let flip = (*sprframe).flip[0] as boolean;

    let mut tx = (*psp).sx - 160 * FRACUNIT;
    tx -= *spriteoffset.add(lump as usize);
    let x1 = (centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS;

    if x1 > viewwidth {
        return;
    }

    tx += *spritewidth.add(lump as usize);
    let x2 = ((centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS) - 1;

    if x2 < 0 {
        return;
    }

    let mut avis = vissprite_t::default();
    let vis = &mut avis as *mut vissprite_t;
    (*vis).mobjflags = 0;
    (*vis).texturemid =
        (BASEYCENTER << FRACBITS) + FRACUNIT / 2 - ((*psp).sy - *spritetopoffset.add(lump as usize));
    (*vis).x1 = if x1 < 0 { 0 } else { x1 };
    (*vis).x2 = if x2 >= viewwidth { viewwidth - 1 } else { x2 };
    (*vis).scale = pspritescale << detailshift;

    if flip != 0 {
        (*vis).xiscale = -pspriteiscale;
        (*vis).startfrac = *spritewidth.add(lump as usize) - 1;
    } else {
        (*vis).xiscale = pspriteiscale;
        (*vis).startfrac = 0;
    }

    if (*vis).x1 > x1 {
        (*vis).startfrac += (*vis).xiscale * ((*vis).x1 - x1);
    }

    (*vis).patch = lump;

    if (*viewplayer).powers[pw_invisibility as usize] > 4 * 32
        || ((*viewplayer).powers[pw_invisibility as usize] & 8 != 0)
    {
        (*vis).colormap = std::ptr::null_mut();
    } else if !fixedcolormap.is_null() {
        (*vis).colormap = fixedcolormap;
    } else if (*(*psp).state).frame & FF_FULLBRIGHT != 0 {
        (*vis).colormap = colormaps;
    } else {
        (*vis).colormap = *spritelights.add(MAXLIGHTSCALE - 1);
    }

    R_DrawVisSprite(vis, (*vis).x1, (*vis).x2);
}

pub unsafe fn R_DrawPlayerSprites() {
    let mut lightnum =
        ((*(*(*(*viewplayer).mo).subsector).sector).lightlevel as i32 >> LIGHTSEGSHIFT)
            + extralight;

    if lightnum < 0 {
        spritelights = scalelight[0].as_mut_ptr();
    } else if lightnum >= (LIGHTLEVELS as i32) {
        spritelights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
    } else {
        spritelights = scalelight[lightnum as usize].as_mut_ptr();
    }

    mfloorclip = screenheightarray.as_mut_ptr();
    mceilingclip = negonearray.as_mut_ptr();

    for i in 0..NUMPSPRITES {
        let psp = &mut (*viewplayer).psprites[i] as *mut pspdef_t;
        if !(*psp).state.is_null() {
            R_DrawPSprite(psp);
        }
    }
}

pub static mut vsprsortedhead: vissprite_t = vissprite_t {
    prev: std::ptr::null_mut(),
    next: std::ptr::null_mut(),
    x1: 0,
    x2: 0,
    gx: 0,
    gy: 0,
    gz: 0,
    gzt: 0,
    startfrac: 0,
    scale: 0,
    xiscale: 0,
    texturemid: 0,
    patch: 0,
    colormap: std::ptr::null_mut(),
    mobjflags: 0,
};

pub unsafe fn R_SortVisSprites() {
    let count =
        (vissprite_p as usize - vissprites.as_ptr() as usize) / std::mem::size_of::<vissprite_t>();

    let mut unsorted = vissprite_t::default();
    unsorted.next = &mut unsorted;
    unsorted.prev = &mut unsorted;

    if count == 0 {
        return;
    }

    let mut ds = vissprites.as_mut_ptr();
    while (ds as usize) < (vissprite_p as usize) {
        (*ds).next = ds.add(1);
        (*ds).prev = ds.sub(1);
        ds = ds.add(1);
    }

    vissprites[0].prev = &mut unsorted;
    unsorted.next = &mut vissprites[0];
    (*vissprite_p.sub(1)).next = &mut unsorted;
    unsorted.prev = vissprite_p.sub(1);

    let mut best: *mut vissprite_t = std::ptr::null_mut();
    vsprsortedhead.next = &mut vsprsortedhead;
    vsprsortedhead.prev = &mut vsprsortedhead;

    for _ in 0..count {
        let mut bestscale = MAXINT;
        ds = unsorted.next;
        while ds != (&mut unsorted as *mut vissprite_t) {
            if (*ds).scale < bestscale {
                bestscale = (*ds).scale;
                best = ds;
            }
            ds = (*ds).next;
        }
        (*(*best).next).prev = (*best).prev;
        (*(*best).prev).next = (*best).next;
        (*best).next = &mut vsprsortedhead;
        (*best).prev = vsprsortedhead.prev;
        (*vsprsortedhead.prev).next = best;
        vsprsortedhead.prev = best;
    }
}

pub unsafe fn R_DrawSprite(spr: *mut vissprite_t) {
    let mut clipbot = [0i16; SCREENWIDTH as usize];
    let mut cliptop = [0i16; SCREENWIDTH as usize];

    for x in (*spr).x1..=(*spr).x2 {
        clipbot[x as usize] = -2;
        cliptop[x as usize] = -2;
    }

    let mut ds = ds_p.sub(1);
    while (ds as usize) >= (drawsegs.as_ptr() as usize) {
        if (*ds).x1 > (*spr).x2
            || (*ds).x2 < (*spr).x1
            || ((*ds).silhouette == 0 && (*ds).maskedtexturecol.is_null())
        {
            ds = ds.sub(1);
            continue;
        }

        let r1 = if (*ds).x1 < (*spr).x1 {
            (*spr).x1
        } else {
            (*ds).x1
        };
        let r2 = if (*ds).x2 > (*spr).x2 {
            (*spr).x2
        } else {
            (*ds).x2
        };

        let (lowscale, scale) = if (*ds).scale1 > (*ds).scale2 {
            ((*ds).scale2, (*ds).scale1)
        } else {
            ((*ds).scale1, (*ds).scale2)
        };

        if scale < (*spr).scale
            || (lowscale < (*spr).scale && R_PointOnSegSide((*spr).gx, (*spr).gy, (*ds).curline) == 0)
        {
            if !(*ds).maskedtexturecol.is_null() {
                R_RenderMaskedSegRange(ds, r1, r2);
            }
            ds = ds.sub(1);
            continue;
        }

        let mut silhouette = (*ds).silhouette;

        if (*spr).gz >= (*ds).bsilheight {
            silhouette &= !SIL_BOTTOM;
        }

        if (*spr).gzt <= (*ds).tsilheight {
            silhouette &= !SIL_TOP;
        }

        if silhouette == 1 {
            for x in r1..=r2 {
                if clipbot[x as usize] == -2 {
                    clipbot[x as usize] = *(*ds).sprbottomclip.offset(x as isize);
                }
            }
        } else if silhouette == 2 {
            for x in r1..=r2 {
                if cliptop[x as usize] == -2 {
                    cliptop[x as usize] = *(*ds).sprtopclip.offset(x as isize);
                }
            }
        } else if silhouette == 3 {
            for x in r1..=r2 {
                if clipbot[x as usize] == -2 {
                    clipbot[x as usize] = *(*ds).sprbottomclip.offset(x as isize);
                }
                if cliptop[x as usize] == -2 {
                    cliptop[x as usize] = *(*ds).sprtopclip.offset(x as isize);
                }
            }
        }

        ds = ds.sub(1);
    }

    for x in (*spr).x1..=(*spr).x2 {
        if clipbot[x as usize] == -2 {
            clipbot[x as usize] = viewheight as i16;
        }
        if cliptop[x as usize] == -2 {
            cliptop[x as usize] = -1;
        }
    }

    mfloorclip = clipbot.as_mut_ptr();
    mceilingclip = cliptop.as_mut_ptr();
    R_DrawVisSprite(spr, (*spr).x1, (*spr).x2);
}

pub unsafe fn R_DrawMasked() {
    R_SortVisSprites();

    if (vissprite_p as usize) > (vissprites.as_ptr() as usize) {
        let mut spr = vsprsortedhead.next;
        while spr != (&mut vsprsortedhead as *mut vissprite_t) {
            R_DrawSprite(spr);
            spr = (*spr).next;
        }
    }

    let mut ds = ds_p.sub(1);
    while (ds as usize) >= (drawsegs.as_ptr() as usize) {
        if !(*ds).maskedtexturecol.is_null() {
            R_RenderMaskedSegRange(ds, (*ds).x1, (*ds).x2);
        }
        ds = ds.sub(1);
    }

    if viewangleoffset == 0 {
        R_DrawPlayerSprites();
    }
}