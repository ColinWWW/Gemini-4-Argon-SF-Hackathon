// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::fs::File;
use std::io::Write;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::v_video::*;

static mut current_palette: [byte; 768] = [0; 768];
static mut frame_count: i32 = 0;

unsafe fn ensure_dir(dir: &str) {
    let _ = std::fs::create_dir_all(dir);
}

pub unsafe fn I_InitGraphics() {
    if screens[0].is_null() {
        let size = (SCREENWIDTH * SCREENHEIGHT) as usize;
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        screens[0] = std::alloc::alloc_zeroed(layout);
    }
}

pub unsafe fn I_ShutdownGraphics() {}

pub unsafe fn I_SetPalette(palette: *mut byte) {
    if !palette.is_null() {
        std::ptr::copy_nonoverlapping(palette, current_palette.as_mut_ptr(), 768);
    }
}

pub unsafe fn I_UpdateNoBlit() {}

pub unsafe fn I_FinishUpdate() {
    let frame_dir = std::env::var("DOOM_FRAME_DIR")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "frames/rust".to_string());

    ensure_dir(&frame_dir);

    let filename = format!("{}/{:05}.raw", frame_dir, frame_count);
    frame_count += 1;

    if let Ok(mut f) = File::create(&filename) {
        let screen_size = (SCREENWIDTH * SCREENHEIGHT) as usize;
        if !screens[0].is_null() {
            let slice = std::slice::from_raw_parts(screens[0], screen_size);
            let _ = f.write_all(slice);
        } else {
            let empty = [0u8; (SCREENWIDTH * SCREENHEIGHT) as usize];
            let _ = f.write_all(&empty);
        }
        let _ = f.write_all(&current_palette);
    }
}

pub unsafe fn I_ReadScreen(scr: *mut byte) {
    if !screens[0].is_null() && !scr.is_null() {
        std::ptr::copy_nonoverlapping(screens[0], scr, (SCREENWIDTH * SCREENHEIGHT) as usize);
    }
}

pub unsafe fn I_StartTic() {}

pub unsafe fn I_StartFrame() {}
