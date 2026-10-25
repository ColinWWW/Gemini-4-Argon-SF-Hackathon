// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_mobj::*;
use crate::p_setup::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_sky::*;
use crate::r_things::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::{c_str, c_strncasecmp, c_strncpy};

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mappatch_t {
    pub originx: i16,
    pub originy: i16,
    pub patch: i16,
    pub stepdir: i16,
    pub colormap: i16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct maptexture_t {
    pub name: [u8; 8],
    pub masked: i32,
    pub width: i16,
    pub height: i16,
    pub columndirectory: u32, // 4-byte obsolete field in WAD!
    pub patchcount: i16,
    pub patches: [mappatch_t; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct texpatch_t {
    pub originx: i32,
    pub originy: i32,
    pub patch: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct texture_t {
    pub name: [u8; 8],
    pub width: i16,
    pub height: i16,
    pub patchcount: i16,
    pub patches: [texpatch_t; 1],
}

pub static mut firstflat: i32 = 0;
pub static mut lastflat: i32 = 0;
pub static mut numflats: i32 = 0;

pub static mut firstpatch: i32 = 0;
pub static mut lastpatch: i32 = 0;
pub static mut numpatches: i32 = 0;

pub static mut firstspritelump: i32 = 0;
pub static mut lastspritelump: i32 = 0;
pub static mut numspritelumps: i32 = 0;

pub static mut numtextures: i32 = 0;
pub static mut textures: *mut *mut texture_t = std::ptr::null_mut();

pub static mut texturewidthmask: *mut i32 = std::ptr::null_mut();
pub static mut textureheight: *mut fixed_t = std::ptr::null_mut();
pub static mut texturecompositesize: *mut i32 = std::ptr::null_mut();
pub static mut texturecolumnlump: *mut *mut i16 = std::ptr::null_mut();
pub static mut texturecolumnofs: *mut *mut u16 = std::ptr::null_mut();
pub static mut texturecomposite: *mut *mut byte = std::ptr::null_mut();

pub static mut flattranslation: *mut i32 = std::ptr::null_mut();
pub static mut texturetranslation: *mut i32 = std::ptr::null_mut();

pub static mut spritewidth: *mut fixed_t = std::ptr::null_mut();
pub static mut spriteoffset: *mut fixed_t = std::ptr::null_mut();
pub static mut spritetopoffset: *mut fixed_t = std::ptr::null_mut();

pub static mut colormaps: *mut lighttable_t = std::ptr::null_mut();

pub unsafe fn R_DrawColumnInCache(
    mut patch: *const column_t,
    cache: *mut byte,
    originy: i32,
    cacheheight: i32,
) {
    while (*patch).topdelta != 0xff {
        let source = (patch as *const byte).add(3);
        let mut count = (*patch).length as i32;
        let mut position = originy + ((*patch).topdelta as i32);

        if position < 0 {
            count += position;
            position = 0;
        }

        if position + count > cacheheight {
            count = cacheheight - position;
        }

        if count > 0 {
            std::ptr::copy_nonoverlapping(source, cache.add(position as usize), count as usize);
        }

        patch = (patch as *const byte).add(((*patch).length as usize) + 4) as *const column_t;
    }
}

pub unsafe fn R_GenerateComposite(texnum: i32) {
    let texture = *textures.add(texnum as usize);

    let block = Z_Malloc(
        *texturecompositesize.add(texnum as usize),
        PU_STATIC,
        texturecomposite.add(texnum as usize) as *mut c_void,
    ) as *mut byte;

    let collump = *texturecolumnlump.add(texnum as usize);
    let colofs = *texturecolumnofs.add(texnum as usize);

    let patches_ptr = (*texture).patches.as_ptr();
    for i in 0..((*texture).patchcount as usize) {
        let patch = patches_ptr.add(i);
        let realpatch = W_CacheLumpNum((*patch).patch, PU_CACHE) as *mut patch_t;
        let x1 = (*patch).originx;
        let mut x2 = x1 + (SHORT((*realpatch).width) as i32);

        let mut x = if x1 < 0 { 0 } else { x1 };
        if x2 > ((*texture).width as i32) {
            x2 = (*texture).width as i32;
        }

        let colofs_rp = (*realpatch).columnofs.as_ptr();
        while x < x2 {
            if *collump.add(x as usize) >= 0 {
                x += 1;
                continue;
            }

            let patchcol = (realpatch as *const byte)
                .add(LONG(*colofs_rp.add((x - x1) as usize)) as usize)
                as *const column_t;
            R_DrawColumnInCache(
                patchcol,
                block.add(*colofs.add(x as usize) as usize),
                (*patch).originy,
                (*texture).height as i32,
            );
            x += 1;
        }
    }

    Z_ChangeTag(block as *mut c_void, PU_CACHE);
}

pub unsafe fn R_GenerateLookup(texnum: i32) {
    let texture = *textures.add(texnum as usize);

    *texturecomposite.add(texnum as usize) = std::ptr::null_mut();
    *texturecompositesize.add(texnum as usize) = 0;

    let collump = *texturecolumnlump.add(texnum as usize);
    let colofs = *texturecolumnofs.add(texnum as usize);

    let width = (*texture).width as usize;
    let mut patchcount = vec![0u8; width];
    let patches_ptr = (*texture).patches.as_ptr();

    for i in 0..((*texture).patchcount as usize) {
        let patch = patches_ptr.add(i);
        let realpatch = W_CacheLumpNum((*patch).patch, PU_CACHE) as *mut patch_t;
        let x1 = (*patch).originx;
        let mut x2 = x1 + (SHORT((*realpatch).width) as i32);

        let mut x = if x1 < 0 { 0 } else { x1 };
        if x2 > (width as i32) {
            x2 = width as i32;
        }

        let colofs_rp = (*realpatch).columnofs.as_ptr();
        while x < x2 {
            patchcount[x as usize] += 1;
            *collump.add(x as usize) = (*patch).patch as i16;
            *colofs.add(x as usize) = (LONG(*colofs_rp.add((x - x1) as usize)) + 3) as u16;
            x += 1;
        }
    }

    for x in 0..width {
        if patchcount[x] == 0 {
            let mut nbuf = [0u8; 9];
            nbuf[..8].copy_from_slice(&(*texture).name);
            println!(
                "R_GenerateLookup: column without a patch ({})",
                c_str(nbuf.as_ptr())
            );
            return;
        }

        if patchcount[x] > 1 {
            *collump.add(x) = -1;
            *colofs.add(x) = (*texturecompositesize.add(texnum as usize)) as u16;

            if *texturecompositesize.add(texnum as usize) > 0x10000 - ((*texture).height as i32) {
                I_Error(&format!("R_GenerateLookup: texture {} is >64k", texnum));
            }

            *texturecompositesize.add(texnum as usize) += (*texture).height as i32;
        }
    }
}

pub unsafe fn R_GetColumn(tex: i32, mut col: i32) -> *mut byte {
    col &= *texturewidthmask.add(tex as usize);
    let lump = *(*texturecolumnlump.add(tex as usize)).add(col as usize) as i32;
    let ofs = *(*texturecolumnofs.add(tex as usize)).add(col as usize) as usize;

    if lump > 0 {
        return (W_CacheLumpNum(lump, PU_CACHE) as *mut byte).add(ofs);
    }

    if (*texturecomposite.add(tex as usize)).is_null() {
        R_GenerateComposite(tex);
    }

    (*texturecomposite.add(tex as usize)).add(ofs)
}

pub unsafe fn R_InitTextures() {
    let names = W_CacheLumpNameStr("PNAMES", PU_STATIC) as *const u8;
    let nummappatches = LONG(*(names as *const i32)) as usize;
    let name_p = names.add(4);
    let mut patchlookup = vec![0i32; nummappatches];

    for i in 0..nummappatches {
        let mut name = [0u8; 9];
        c_strncpy(name.as_mut_ptr(), name_p.add(i * 8), 8);
        patchlookup[i] = W_CheckNumForName(name.as_ptr());
    }
    Z_Free(names as *mut c_void);

    let maptex1 = W_CacheLumpNameStr("TEXTURE1", PU_STATIC) as *const i32;
    let mut maptex = maptex1;
    let numtextures1 = LONG(*maptex);
    let mut maxoff = W_LumpLength(W_GetNumForNameStr("TEXTURE1"));
    let mut directory = maptex.add(1);

    let maptex2: *const i32;
    let numtextures2: i32;
    let maxoff2: i32;

    if W_CheckNumForNameStr("TEXTURE2") != -1 {
        maptex2 = W_CacheLumpNameStr("TEXTURE2", PU_STATIC) as *const i32;
        numtextures2 = LONG(*maptex2);
        maxoff2 = W_LumpLength(W_GetNumForNameStr("TEXTURE2"));
    } else {
        maptex2 = std::ptr::null();
        numtextures2 = 0;
        maxoff2 = 0;
    }

    numtextures = numtextures1 + numtextures2;
    let ptr_sz = std::mem::size_of::<usize>() as i32;

    textures = Z_Malloc(numtextures * ptr_sz, PU_STATIC, std::ptr::null_mut())
        as *mut *mut texture_t;
    texturecolumnlump = Z_Malloc(numtextures * ptr_sz, PU_STATIC, std::ptr::null_mut())
        as *mut *mut i16;
    texturecolumnofs = Z_Malloc(numtextures * ptr_sz, PU_STATIC, std::ptr::null_mut())
        as *mut *mut u16;
    texturecomposite = Z_Malloc(numtextures * ptr_sz, PU_STATIC, std::ptr::null_mut())
        as *mut *mut byte;
    texturecompositesize =
        Z_Malloc(numtextures * 4, PU_STATIC, std::ptr::null_mut()) as *mut i32;
    texturewidthmask = Z_Malloc(numtextures * 4, PU_STATIC, std::ptr::null_mut()) as *mut i32;
    textureheight = Z_Malloc(numtextures * 4, PU_STATIC, std::ptr::null_mut()) as *mut fixed_t;

    let temp1 = W_GetNumForNameStr("S_START");
    let temp2 = W_GetNumForNameStr("S_END") - 1;
    let temp3 = ((temp2 - temp1 + 63) / 64) + ((numtextures + 63) / 64);
    print!("[");
    for _ in 0..temp3 {
        print!(" ");
    }
    print!("         ]");
    for _ in 0..temp3 {
        print!("\x08");
    }
    print!("\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08");

    for i in 0..(numtextures as usize) {
        if (i & 63) == 0 {
            print!(".");
        }

        if (i as i32) == numtextures1 {
            maptex = maptex2;
            maxoff = maxoff2;
            directory = maptex.add(1);
        }

        let offset = LONG(*directory);
        directory = directory.add(1);

        if offset > maxoff {
            I_Error("R_InitTextures: bad texture directory");
        }

        let mtexture = (maptex as *const byte).add(offset as usize) as *const maptexture_t;
        let patchcount = SHORT((*mtexture).patchcount) as usize;

        let tex_alloc_size = std::mem::size_of::<texture_t>()
            + std::mem::size_of::<texpatch_t>() * patchcount.saturating_sub(1);
        let texture = Z_Malloc(tex_alloc_size as i32, PU_STATIC, std::ptr::null_mut())
            as *mut texture_t;
        *textures.add(i) = texture;

        (*texture).width = SHORT((*mtexture).width);
        (*texture).height = SHORT((*mtexture).height);
        (*texture).patchcount = patchcount as i16;

        (*texture).name = (*mtexture).name;

        let mpatch_base = std::ptr::addr_of!((*mtexture).patches) as *const mappatch_t;
        let patch_base = (*texture).patches.as_mut_ptr();

        for j in 0..patchcount {
            let mp = *mpatch_base.add(j);
            let p = patch_base.add(j);
            (*p).originx = SHORT(mp.originx) as i32;
            (*p).originy = SHORT(mp.originy) as i32;
            (*p).patch = patchlookup[SHORT(mp.patch) as usize];
            if (*p).patch == -1 {
                let mut nbuf = [0u8; 9];
                nbuf[..8].copy_from_slice(&(*texture).name);
                I_Error(&format!(
                    "R_InitTextures: Missing patch in texture {}",
                    c_str(nbuf.as_ptr())
                ));
            }
        }

        *texturecolumnlump.add(i) = Z_Malloc(
            ((*texture).width as i32) * 2,
            PU_STATIC,
            std::ptr::null_mut(),
        ) as *mut i16;
        *texturecolumnofs.add(i) = Z_Malloc(
            ((*texture).width as i32) * 2,
            PU_STATIC,
            std::ptr::null_mut(),
        ) as *mut u16;

        let mut j = 1i32;
        while j * 2 <= ((*texture).width as i32) {
            j <<= 1;
        }

        *texturewidthmask.add(i) = j - 1;
        *textureheight.add(i) = ((*texture).height as fixed_t) << FRACBITS;
    }

    Z_Free(maptex1 as *mut c_void);
    if !maptex2.is_null() {
        Z_Free(maptex2 as *mut c_void);
    }

    for i in 0..numtextures {
        R_GenerateLookup(i);
    }

    texturetranslation =
        Z_Malloc((numtextures + 1) * 4, PU_STATIC, std::ptr::null_mut()) as *mut i32;
    for i in 0..(numtextures as usize) {
        *texturetranslation.add(i) = i as i32;
    }
}

pub unsafe fn R_InitFlats() {
    firstflat = W_GetNumForNameStr("F_START") + 1;
    lastflat = W_GetNumForNameStr("F_END") - 1;
    numflats = lastflat - firstflat + 1;

    flattranslation = Z_Malloc((numflats + 1) * 4, PU_STATIC, std::ptr::null_mut()) as *mut i32;
    for i in 0..(numflats as usize) {
        *flattranslation.add(i) = i as i32;
    }
}

pub unsafe fn R_InitSpriteLumps() {
    firstspritelump = W_GetNumForNameStr("S_START") + 1;
    lastspritelump = W_GetNumForNameStr("S_END") - 1;

    numspritelumps = lastspritelump - firstspritelump + 1;
    spritewidth = Z_Malloc(numspritelumps * 4, PU_STATIC, std::ptr::null_mut()) as *mut fixed_t;
    spriteoffset = Z_Malloc(numspritelumps * 4, PU_STATIC, std::ptr::null_mut()) as *mut fixed_t;
    spritetopoffset =
        Z_Malloc(numspritelumps * 4, PU_STATIC, std::ptr::null_mut()) as *mut fixed_t;

    for i in 0..(numspritelumps as usize) {
        if (i & 63) == 0 {
            print!(".");
        }

        let patch = W_CacheLumpNum(firstspritelump + (i as i32), PU_CACHE) as *mut patch_t;
        *spritewidth.add(i) = (SHORT((*patch).width) as fixed_t) << FRACBITS;
        *spriteoffset.add(i) = (SHORT((*patch).leftoffset) as fixed_t) << FRACBITS;
        *spritetopoffset.add(i) = (SHORT((*patch).topoffset) as fixed_t) << FRACBITS;
    }
}

pub unsafe fn R_InitColormaps() {
    let lump = W_GetNumForNameStr("COLORMAP");
    let length = W_LumpLength(lump) + 255;
    let raw = Z_Malloc(length, PU_STATIC, std::ptr::null_mut()) as usize;
    colormaps = ((raw + 255) & !0xff) as *mut lighttable_t;
    W_ReadLump(lump, colormaps as *mut c_void);
}

pub unsafe fn R_InitData() {
    R_InitTextures();
    print!("\nInitTextures");
    R_InitFlats();
    print!("\nInitFlats");
    R_InitSpriteLumps();
    print!("\nInitSprites");
    R_InitColormaps();
    print!("\nInitColormaps");
}

pub unsafe fn R_FlatNumForName(name: *const u8) -> i32 {
    let i = W_CheckNumForName(name);
    if i == -1 {
        let mut namet = [0u8; 9];
        std::ptr::copy_nonoverlapping(name, namet.as_mut_ptr(), 8);
        I_Error(&format!(
            "R_FlatNumForName: {} not found",
            c_str(namet.as_ptr())
        ));
    }
    i - firstflat
}

pub unsafe fn R_FlatNumForNameStr(name: &str) -> i32 {
    let mut buf = [0u8; 9];
    let b = name.as_bytes();
    let len = b.len().min(8);
    buf[..len].copy_from_slice(&b[..len]);
    R_FlatNumForName(buf.as_ptr())
}

pub unsafe fn R_CheckTextureNumForName(name: *const u8) -> i32 {
    if *name == b'-' {
        return 0;
    }

    for i in 0..(numtextures as usize) {
        let tex = *textures.add(i);
        if c_strncasecmp((*tex).name.as_ptr(), name, 8) == 0 {
            return i as i32;
        }
    }

    -1
}

pub unsafe fn R_CheckTextureNumForNameStr(name: &str) -> i32 {
    let mut buf = [0u8; 9];
    let b = name.as_bytes();
    let len = b.len().min(8);
    buf[..len].copy_from_slice(&b[..len]);
    R_CheckTextureNumForName(buf.as_ptr())
}

pub unsafe fn R_TextureNumForName(name: *const u8) -> i32 {
    let i = R_CheckTextureNumForName(name);
    if i == -1 {
        let mut namet = [0u8; 9];
        std::ptr::copy_nonoverlapping(name, namet.as_mut_ptr(), 8);
        I_Error(&format!(
            "R_TextureNumForName: {} not found",
            c_str(namet.as_ptr())
        ));
    }
    i
}

pub unsafe fn R_TextureNumForNameStr(name: &str) -> i32 {
    let mut buf = [0u8; 9];
    let b = name.as_bytes();
    let len = b.len().min(8);
    buf[..len].copy_from_slice(&b[..len]);
    R_TextureNumForName(buf.as_ptr())
}

pub static mut flatmemory: i32 = 0;
pub static mut texturememory: i32 = 0;
pub static mut spritememory: i32 = 0;

pub unsafe fn R_PrecacheLevel() {
    if demoplayback != 0 {
        return;
    }

    let mut flatpresent = vec![0u8; numflats as usize];
    for i in 0..(numsectors as usize) {
        flatpresent[(*sectors.add(i)).floorpic as usize] = 1;
        flatpresent[(*sectors.add(i)).ceilingpic as usize] = 1;
    }

    flatmemory = 0;
    for i in 0..(numflats as usize) {
        if flatpresent[i] != 0 {
            let lump = firstflat + (i as i32);
            flatmemory += (*lumpinfo.add(lump as usize)).size;
            W_CacheLumpNum(lump, PU_CACHE);
        }
    }

    let mut texturepresent = vec![0u8; numtextures as usize];
    for i in 0..(numsides as usize) {
        texturepresent[(*sides.add(i)).toptexture as usize] = 1;
        texturepresent[(*sides.add(i)).midtexture as usize] = 1;
        texturepresent[(*sides.add(i)).bottomtexture as usize] = 1;
    }

    texturepresent[skytexture as usize] = 1;

    texturememory = 0;
    for i in 0..(numtextures as usize) {
        if texturepresent[i] == 0 {
            continue;
        }
        let texture = *textures.add(i);
        let patches_ptr = (*texture).patches.as_ptr();
        for j in 0..((*texture).patchcount as usize) {
            let lump = (*patches_ptr.add(j)).patch;
            texturememory += (*lumpinfo.add(lump as usize)).size;
            W_CacheLumpNum(lump, PU_CACHE);
        }
    }

    let mut spritepresent = vec![0u8; numsprites as usize];
    let mut th = thinkercap.next;
    while th != &mut thinkercap {
        if (*th).function == actionf_t::acp1(P_MobjThinker) {
            spritepresent[(*(th as *mut mobj_t)).sprite as usize] = 1;
        }
        th = (*th).next;
    }

    spritememory = 0;
    for i in 0..(numsprites as usize) {
        if spritepresent[i] == 0 {
            continue;
        }
        let spr = sprites.add(i);
        for j in 0..((*spr).numframes as usize) {
            let sf = (*spr).spriteframes.add(j);
            for k in 0..8 {
                let lump = firstspritelump + ((*sf).lump[k] as i32);
                spritememory += (*lumpinfo.add(lump as usize)).size;
                W_CacheLumpNum(lump, PU_CACHE);
            }
        }
    }
}
