// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::fs::File;
use std::io::{Read, Write};
use crate::d_main::basedefault;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::m_argv::*;
use crate::m_menu::*;
use crate::m_swap::*;
use crate::r_defs::*;
use crate::s_sound::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::{c_str, write_c_str};

pub static mut usemouse: i32 = 1;
pub static mut usejoystick: i32 = 0;

pub unsafe fn M_DrawText(mut x: i32, y: i32, direct: boolean, mut string: *const u8) -> i32 {
    while *string != 0 {
        let c = ((*string).to_ascii_uppercase() as i32) - (HU_FONTSTART as i32);
        string = string.add(1);
        if c < 0 || (c as usize) > HU_FONTSIZE {
            x += 4;
            continue;
        }
        let w = SHORT((*hu_font[c as usize]).width) as i32;
        if x + w > SCREENWIDTH {
            break;
        }
        if direct != 0 {
            V_DrawPatchDirect(x, y, 0, hu_font[c as usize]);
        } else {
            V_DrawPatch(x, y, 0, hu_font[c as usize]);
        }
        x += w;
    }
    x
}

pub unsafe fn M_WriteFile(name: *const u8, source: *const std::ffi::c_void, length: i32) -> boolean {
    let fname = c_str(name);
    match File::create(fname) {
        Ok(mut f) => {
            let slice = std::slice::from_raw_parts(source as *const u8, length as usize);
            match f.write_all(slice) {
                Ok(_) => true_,
                Err(_) => false_,
            }
        }
        Err(_) => false_,
    }
}

pub unsafe fn M_ReadFile(name: *const u8, buffer: *mut *mut byte) -> i32 {
    let fname = c_str(name);
    match File::open(fname) {
        Ok(mut f) => {
            let len = f.metadata().map(|m| m.len() as i32).unwrap_or(0);
            let buf = Z_Malloc(len, PU_STATIC, std::ptr::null_mut()) as *mut byte;
            let slice = std::slice::from_raw_parts_mut(buf, len as usize);
            if f.read_exact(slice).is_err() {
                I_Error(&format!("Couldn't read file {}", fname));
            }
            *buffer = buf;
            len
        }
        Err(_) => {
            I_Error(&format!("Couldn't read file {}", fname));
        }
    }
}

pub static mut defaultfile: [u8; 1024] = [0; 1024];

pub unsafe fn M_SaveDefaults() {}

pub unsafe fn M_LoadDefaults() {
    mouseSensitivity = 5;
    snd_SfxVolume = 8;
    snd_MusicVolume = 8;
    showMessages = 1;

    key_right = KEY_RIGHTARROW;
    key_left = KEY_LEFTARROW;
    key_up = KEY_UPARROW;
    key_down = KEY_DOWNARROW;
    key_strafeleft = b',' as i32;
    key_straferight = b'.' as i32;
    key_fire = KEY_RCTRL;
    key_use = b' ' as i32;
    key_strafe = KEY_RALT;
    key_speed = KEY_RSHIFT;

    usemouse = 1;
    mousebfire = 0;
    mousebstrafe = 1;
    mousebforward = 2;

    usejoystick = 0;
    joybfire = 0;
    joybstrafe = 1;
    joybuse = 3;
    joybspeed = 2;

    screenblocks = 9;
    detailLevel = 0;
    numChannels = 3;
    usegamma = 0;

    let i = M_CheckParmStr("-config");
    if i != 0 && i < myargc - 1 {
        let cfg = c_str(*myargv.add((i + 1) as usize));
        write_c_str(defaultfile.as_mut_ptr(), cfg);
        println!("\tdefault file: {}", cfg);
    } else {
        std::ptr::copy_nonoverlapping(basedefault.as_ptr(), defaultfile.as_mut_ptr(), 1024);
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pcx_t {
    pub manufacturer: u8,
    pub version: u8,
    pub encoding: u8,
    pub bits_per_pixel: u8,
    pub xmin: u16,
    pub ymin: u16,
    pub xmax: u16,
    pub ymax: u16,
    pub hres: u16,
    pub vres: u16,
    pub palette: [u8; 48],
    pub reserved: u8,
    pub color_planes: u8,
    pub bytes_per_line: u16,
    pub palette_type: u16,
    pub filler: [u8; 58],
    pub data: u8,
}

pub unsafe fn WritePCXfile(
    filename: *const u8,
    mut data: *mut byte,
    width: i32,
    height: i32,
    mut palette: *mut byte,
) {
    let pcx = Z_Malloc(width * height * 2 + 1000, PU_STATIC, std::ptr::null_mut()) as *mut pcx_t;
    (*pcx).manufacturer = 0x0a;
    (*pcx).version = 5;
    (*pcx).encoding = 1;
    (*pcx).bits_per_pixel = 8;
    (*pcx).xmin = 0;
    (*pcx).ymin = 0;
    (*pcx).xmax = SHORT((width - 1) as i16) as u16;
    (*pcx).ymax = SHORT((height - 1) as i16) as u16;
    (*pcx).hres = SHORT(width as i16) as u16;
    (*pcx).vres = SHORT(height as i16) as u16;
    (*pcx).palette = [0; 48];
    (*pcx).color_planes = 1;
    (*pcx).bytes_per_line = SHORT(width as i16) as u16;
    (*pcx).palette_type = SHORT(2) as u16;
    (*pcx).filler = [0; 58];

    let mut pack = std::ptr::addr_of_mut!((*pcx).data);

    for _ in 0..(width * height) {
        if (*data & 0xc0) != 0xc0 {
            *pack = *data;
            pack = pack.add(1);
            data = data.add(1);
        } else {
            *pack = 0xc1;
            pack = pack.add(1);
            *pack = *data;
            pack = pack.add(1);
            data = data.add(1);
        }
    }

    *pack = 0x0c;
    pack = pack.add(1);
    for _ in 0..768 {
        *pack = *palette;
        pack = pack.add(1);
        palette = palette.add(1);
    }

    let length = (pack as usize - pcx as usize) as i32;
    M_WriteFile(filename, pcx as *const std::ffi::c_void, length);
    Z_Free(pcx as *mut std::ffi::c_void);
}

pub unsafe fn M_ScreenShot() {
    let linear = screens[2];
    I_ReadScreen(linear);
    players[consoleplayer as usize].message = b"screen shot\0".as_ptr();
}
