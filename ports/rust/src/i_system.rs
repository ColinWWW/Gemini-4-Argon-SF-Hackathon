// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_net::*;
use crate::d_ticcmd::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_sound::*;
use crate::i_video::*;
use crate::m_misc::*;

pub static mut mb_used: i32 = 6;

pub unsafe fn I_Tactile(_on: i32, _off: i32, _total: i32) {}

pub static mut emptycmd: ticcmd_t = ticcmd_t {
    forwardmove: 0,
    sidemove: 0,
    angleturn: 0,
    consistancy: 0,
    chatchar: 0,
    buttons: 0,
};

pub unsafe fn I_BaseTiccmd() -> *mut ticcmd_t {
    &mut emptycmd
}

pub unsafe fn I_GetHeapSize() -> i32 {
    mb_used * 1024 * 1024
}

pub unsafe fn I_ZoneBase(size: *mut i32) -> *mut byte {
    *size = mb_used * 1024 * 1024;
    let layout = std::alloc::Layout::from_size_align(*size as usize, 8).unwrap();
    std::alloc::alloc_zeroed(layout)
}

static mut fake_tics: i32 = 0;

pub unsafe fn I_GetTime() -> i32 {
    let t = fake_tics;
    fake_tics += 1;
    t
}

pub unsafe fn I_Init() {
    I_InitSound();
}

pub unsafe fn I_Quit() {
    D_QuitNetGame();
    I_ShutdownSound();
    I_ShutdownMusic();
    M_SaveDefaults();
    I_ShutdownGraphics();
    std::process::exit(0);
}

pub unsafe fn I_WaitVBL(_count: i32) {}

pub unsafe fn I_BeginRead() {}

pub unsafe fn I_EndRead() {}

pub unsafe fn I_AllocLow(length: i32) -> *mut byte {
    let layout = std::alloc::Layout::from_size_align((length as usize).max(1), 8).unwrap();
    std::alloc::alloc_zeroed(layout)
}

pub unsafe fn I_Error(error: &str) -> ! {
    eprintln!("Error: {}", error);
    if demorecording != 0 {
        G_CheckDemoStatus();
    }
    D_QuitNetGame();
    I_ShutdownGraphics();
    std::process::exit(-1);
}
