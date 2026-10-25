// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::sounds::*;

pub static mut sndserver_filename: *const u8 = b"./sndserver \0".as_ptr();

pub unsafe fn I_InitSound() {}
pub unsafe fn I_UpdateSound() {}
pub unsafe fn I_SubmitSound() {}
pub unsafe fn I_ShutdownSound() {}
pub unsafe fn I_SetChannels() {}

pub unsafe fn I_GetSfxLumpNum(_sfxinfo: *mut sfxinfo_t) -> i32 {
    0
}

pub unsafe fn I_StartSound(_id: i32, _vol: i32, _sep: i32, _pitch: i32, _priority: i32) -> i32 {
    1
}

pub unsafe fn I_StopSound(_handle: i32) {}

pub unsafe fn I_SoundIsPlaying(_handle: i32) -> i32 {
    0
}

pub unsafe fn I_UpdateSoundParams(_handle: i32, _vol: i32, _sep: i32, _pitch: i32) {}

pub unsafe fn I_InitMusic() {}
pub unsafe fn I_ShutdownMusic() {}
pub unsafe fn I_SetMusicVolume(_volume: i32) {}
pub unsafe fn I_PauseSong(_handle: i32) {}
pub unsafe fn I_ResumeSong(_handle: i32) {}

pub unsafe fn I_RegisterSong(_data: *mut c_void) -> i32 {
    1
}

pub unsafe fn I_PlaySong(_handle: i32, _looping: i32) {}
pub unsafe fn I_StopSong(_handle: i32) {}
pub unsafe fn I_UnRegisterSong(_handle: i32) {}
