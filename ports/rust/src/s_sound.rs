// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_sound::*;
use crate::i_system::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_mobj::*;
use crate::r_main::*;
use crate::sounds::*;
use crate::tables::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub const S_MAX_VOLUME: i32 = 127;
pub const S_CLIPPING_DIST: fixed_t = 1200 * 0x10000;
pub const S_CLOSE_DIST: fixed_t = 160 * 0x10000;
pub const S_ATTENUATOR: i32 = (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS;
pub const NORM_PITCH: i32 = 128;
pub const NORM_PRIORITY: i32 = 64;
pub const NORM_SEP: i32 = 128;
pub const S_PITCH_PERTURB: i32 = 1;
pub const S_STEREO_SWING: fixed_t = 96 * 0x10000;
pub const S_IFRACVOL: i32 = 30;
pub const S_NUMCHANNELS: i32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_t {
    pub sfxinfo: *mut sfxinfo_t,
    pub origin: *mut c_void,
    pub handle: i32,
}

static mut channels: *mut channel_t = std::ptr::null_mut();
pub static mut snd_SfxVolume: i32 = 15;
pub static mut snd_MusicVolume: i32 = 15;
static mut mus_paused: boolean = 0;
static mut mus_playing: *mut musicinfo_t = std::ptr::null_mut();
pub static mut numChannels: i32 = 0;
static mut nextcleanup: i32 = 0;

pub unsafe fn S_Init(sfxVolume: i32, musicVolume: i32) {
    eprintln!("S_Init: default sfx volume {}", sfxVolume);
    I_SetChannels();
    S_SetSfxVolume(sfxVolume);
    S_SetMusicVolume(musicVolume);

    channels = Z_Malloc(
        numChannels * (std::mem::size_of::<channel_t>() as i32),
        PU_STATIC,
        std::ptr::null_mut(),
    ) as *mut channel_t;

    for i in 0..numChannels {
        (*channels.add(i as usize)).sfxinfo = std::ptr::null_mut();
    }

    mus_paused = 0;

    for i in 1..NUMSFX {
        S_sfx[i].lumpnum = -1;
        S_sfx[i].usefulness = -1;
    }
}

pub unsafe fn S_Start() {
    for cnum in 0..numChannels {
        if !(*channels.add(cnum as usize)).sfxinfo.is_null() {
            S_StopChannel(cnum);
        }
    }

    mus_paused = 0;

    let mnum = if gamemode == commercial {
        mus_runnin + gamemap - 1
    } else {
        let spmus = [
            mus_e3m4, mus_e3m2, mus_e3m3, mus_e1m5, mus_e2m7, mus_e2m4, mus_e2m6, mus_e2m5,
            mus_e1m9,
        ];
        if gameepisode < 4 {
            mus_e1m1 + (gameepisode - 1) * 9 + gamemap - 1
        } else {
            spmus[(gamemap - 1) as usize]
        }
    };

    S_ChangeMusic(mnum, true_);
    nextcleanup = 15;
}

pub unsafe fn S_StartSoundAtVolume(origin_p: *mut c_void, sfx_id: i32, mut volume: i32) {
    let mut sep: i32 = 0;
    let mut pitch: i32;
    let priority: i32;
    let origin = origin_p as *mut mobj_t;

    if sfx_id < 1 || (sfx_id as usize) > NUMSFX {
        I_Error(&format!("Bad sfx #: {}", sfx_id));
    }

    let sfx = &mut S_sfx[sfx_id as usize] as *mut sfxinfo_t;

    if (*sfx).link_idx >= 0 {
        pitch = (*sfx).pitch;
        priority = (*sfx).priority;
        volume += (*sfx).volume;

        if volume < 1 {
            return;
        }
        if volume > snd_SfxVolume {
            volume = snd_SfxVolume;
        }
    } else {
        pitch = NORM_PITCH;
        priority = NORM_PRIORITY;
    }

    if !origin.is_null() && origin != players[consoleplayer as usize].mo {
        let rc = S_AdjustSoundParams(
            players[consoleplayer as usize].mo,
            origin,
            &mut volume,
            &mut sep,
            &mut pitch,
        );

        if (*origin).x == (*players[consoleplayer as usize].mo).x
            && (*origin).y == (*players[consoleplayer as usize].mo).y
        {
            sep = NORM_SEP;
        }

        if rc == 0 {
            return;
        }
    } else {
        sep = NORM_SEP;
    }

    if sfx_id >= sfx_sawup && sfx_id <= sfx_sawhit {
        pitch += 8 - (M_Random() & 15);
        if pitch < 0 {
            pitch = 0;
        } else if pitch > 255 {
            pitch = 255;
        }
    } else if sfx_id != sfx_itemup && sfx_id != sfx_tink {
        pitch += 16 - (M_Random() & 31);
        if pitch < 0 {
            pitch = 0;
        } else if pitch > 255 {
            pitch = 255;
        }
    }

    S_StopSound(origin_p);

    let cnum = S_getChannel(origin_p, sfx);
    if cnum < 0 {
        return;
    }

    if (*sfx).lumpnum < 0 {
        (*sfx).lumpnum = I_GetSfxLumpNum(sfx);
    }

    let old_use = (*sfx).usefulness;
    (*sfx).usefulness += 1;
    if old_use < 0 {
        (*sfx).usefulness = 1;
    }

    (*channels.add(cnum as usize)).handle = I_StartSound(sfx_id, volume, sep, pitch, priority);
}

pub unsafe fn S_StartSound(origin: *mut c_void, sfx_id: i32) {
    S_StartSoundAtVolume(origin, sfx_id, snd_SfxVolume);
}

pub unsafe fn S_StopSound(origin: *mut c_void) {
    for cnum in 0..numChannels {
        let c = channels.add(cnum as usize);
        if !(*c).sfxinfo.is_null() && (*c).origin == origin {
            S_StopChannel(cnum);
            break;
        }
    }
}

pub unsafe fn S_PauseSound() {
    if !mus_playing.is_null() && mus_paused == 0 {
        I_PauseSong((*mus_playing).handle);
        mus_paused = true_;
    }
}

pub unsafe fn S_ResumeSound() {
    if !mus_playing.is_null() && mus_paused != 0 {
        I_ResumeSong((*mus_playing).handle);
        mus_paused = false_;
    }
}

pub unsafe fn S_UpdateSounds(listener_p: *mut c_void) {
    let listener = listener_p as *mut mobj_t;

    for cnum in 0..numChannels {
        let c = channels.add(cnum as usize);
        let sfx = (*c).sfxinfo;

        if !sfx.is_null() {
            if I_SoundIsPlaying((*c).handle) != 0 {
                let mut volume = snd_SfxVolume;
                let mut pitch = NORM_PITCH;
                let mut sep = NORM_SEP;

                if (*sfx).link_idx >= 0 {
                    pitch = (*sfx).pitch;
                    volume += (*sfx).volume;
                    if volume < 1 {
                        S_StopChannel(cnum);
                        continue;
                    } else if volume > snd_SfxVolume {
                        volume = snd_SfxVolume;
                    }
                }

                if !(*c).origin.is_null() && listener_p != (*c).origin {
                    let audible = S_AdjustSoundParams(
                        listener,
                        (*c).origin as *mut mobj_t,
                        &mut volume,
                        &mut sep,
                        &mut pitch,
                    );
                    if audible == 0 {
                        S_StopChannel(cnum);
                    } else {
                        I_UpdateSoundParams((*c).handle, volume, sep, pitch);
                    }
                }
            } else {
                S_StopChannel(cnum);
            }
        }
    }
}

pub unsafe fn S_SetMusicVolume(volume: i32) {
    if volume < 0 || volume > 127 {
        I_Error(&format!("Attempt to set music volume at {}", volume));
    }
    I_SetMusicVolume(127);
    I_SetMusicVolume(volume);
    snd_MusicVolume = volume;
}

pub unsafe fn S_SetSfxVolume(volume: i32) {
    if volume < 0 || volume > 127 {
        I_Error(&format!("Attempt to set sfx volume at {}", volume));
    }
    snd_SfxVolume = volume;
}

pub unsafe fn S_StartMusic(m_id: i32) {
    S_ChangeMusic(m_id, false_);
}

pub unsafe fn S_ChangeMusic(musicnum: i32, looping: i32) {
    if musicnum <= mus_None || (musicnum as usize) >= NUMMUSIC {
        I_Error(&format!("Bad music number {}", musicnum));
    }
    let music = &mut S_music[musicnum as usize] as *mut musicinfo_t;

    if mus_playing == music {
        return;
    }

    S_StopMusic();

    if (*music).lumpnum == 0 {
        let namebuf = format!("d_{}", (*music).name);
        (*music).lumpnum = W_GetNumForNameStr(&namebuf);
    }

    (*music).data = W_CacheLumpNum((*music).lumpnum, PU_MUSIC);
    (*music).handle = I_RegisterSong((*music).data);

    I_PlaySong((*music).handle, looping);
    mus_playing = music;
}

pub unsafe fn S_StopMusic() {
    if !mus_playing.is_null() {
        if mus_paused != 0 {
            I_ResumeSong((*mus_playing).handle);
        }
        I_StopSong((*mus_playing).handle);
        I_UnRegisterSong((*mus_playing).handle);
        Z_ChangeTag((*mus_playing).data, PU_CACHE);

        (*mus_playing).data = std::ptr::null_mut();
        mus_playing = std::ptr::null_mut();
    }
}

pub unsafe fn S_StopChannel(cnum: i32) {
    let c = channels.add(cnum as usize);
    if !(*c).sfxinfo.is_null() {
        if I_SoundIsPlaying((*c).handle) != 0 {
            I_StopSound((*c).handle);
        }
        (*(*c).sfxinfo).usefulness -= 1;
        (*c).sfxinfo = std::ptr::null_mut();
    }
}

pub unsafe fn S_AdjustSoundParams(
    listener: *mut mobj_t,
    source: *mut mobj_t,
    vol: *mut i32,
    sep: *mut i32,
    _pitch: *mut i32,
) -> i32 {
    let adx = ((*listener).x - (*source).x).wrapping_abs();
    let ady = ((*listener).y - (*source).y).wrapping_abs();

    let mut approx_dist = adx + ady - ((if adx < ady { adx } else { ady }) >> 1);

    if gamemap != 8 && approx_dist > S_CLIPPING_DIST {
        return 0;
    }

    let mut angle = R_PointToAngle2((*listener).x, (*listener).y, (*source).x, (*source).y);
    if angle > (*listener).angle {
        angle = angle.wrapping_sub((*listener).angle);
    } else {
        angle = angle.wrapping_add(0xffffffffu32.wrapping_sub((*listener).angle));
    }

    angle >>= ANGLETOFINESHIFT;

    *sep = 128 - (FixedMul(S_STEREO_SWING, finesine[angle as usize]) >> FRACBITS);

    if approx_dist < S_CLOSE_DIST {
        *vol = snd_SfxVolume;
    } else if gamemap == 8 {
        if approx_dist > S_CLIPPING_DIST {
            approx_dist = S_CLIPPING_DIST;
        }
        *vol = 15
            + ((snd_SfxVolume - 15) * ((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR;
    } else {
        *vol = (snd_SfxVolume * ((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR;
    }

    (*vol > 0) as i32
}

pub unsafe fn S_getChannel(origin: *mut c_void, sfxinfo: *mut sfxinfo_t) -> i32 {
    let mut cnum = 0;
    while cnum < numChannels {
        let c = channels.add(cnum as usize);
        if (*c).sfxinfo.is_null() {
            break;
        } else if !origin.is_null() && (*c).origin == origin {
            S_StopChannel(cnum);
            break;
        }
        cnum += 1;
    }

    if cnum == numChannels {
        cnum = 0;
        while cnum < numChannels {
            let c = channels.add(cnum as usize);
            if (*(*c).sfxinfo).priority >= (*sfxinfo).priority {
                break;
            }
            cnum += 1;
        }

        if cnum == numChannels {
            return -1;
        } else {
            S_StopChannel(cnum);
        }
    }

    let c = channels.add(cnum as usize);
    (*c).sfxinfo = sfxinfo;
    (*c).origin = origin;

    cnum
}
