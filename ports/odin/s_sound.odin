// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

S_MAX_VOLUME    :: 127
S_CLIPPING_DIST :: 1200 * 0x10000
S_CLOSE_DIST    :: 160 * 0x10000
S_ATTENUATOR    :: (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS
NORM_PITCH      :: 128
NORM_PRIORITY   :: 64
NORM_SEP        :: 128
S_PITCH_PERTURB :: 1
S_STEREO_SWING  :: 96 * 0x10000
S_IFRACVOL      :: 30

channel_t :: struct {
	sfxinfo: ^sfxinfo_t,
	origin:  rawptr,
	handle:  int,
}

channels: [^]channel_t

snd_SfxVolume: int = 15
snd_MusicVolume: int = 15

mus_paused: boolean
mus_playing: ^musicinfo_t = nil

numChannels: int
nextcleanup: int

S_Init :: proc(sfxVolume: int, musicVolume: int) {
	libc.fprintf(libc.stderr, "S_Init: default sfx volume %d\n", i32(sfxVolume))

	I_SetChannels()
	S_SetSfxVolume(sfxVolume)
	S_SetMusicVolume(musicVolume)

	channels = ([^]channel_t)(Z_Malloc(numChannels * size_of(channel_t), PU_STATIC, nil))

	for i := 0; i < numChannels; i += 1 {
		channels[i].sfxinfo = nil
	}

	mus_paused = false

	for i := 1; i < int(sfxenum_t.NUMSFX); i += 1 {
		S_sfx[i].lumpnum = -1
		S_sfx[i].usefulness = -1
	}
}

S_Start :: proc() {
	for cnum := 0; cnum < numChannels; cnum += 1 {
		if channels[cnum].sfxinfo != nil {
			S_StopChannel(cnum)
		}
	}

	mus_paused = false

	mnum: int
	if gamemode == .commercial {
		mnum = int(musicenum_t.mus_runnin) + gamemap - 1
	} else {
		spmus := [9]musicenum_t{
			.mus_e3m4,
			.mus_e3m2,
			.mus_e3m3,
			.mus_e1m5,
			.mus_e2m7,
			.mus_e2m4,
			.mus_e2m6,
			.mus_e2m5,
			.mus_e1m9,
		}

		if gameepisode < 4 {
			mnum = int(musicenum_t.mus_e1m1) + (gameepisode - 1) * 9 + gamemap - 1
		} else {
			mnum = int(spmus[gamemap - 1])
		}
	}

	S_ChangeMusic(mnum, 1)
	nextcleanup = 15
}

S_StartSoundAtVolume :: proc(origin_p: rawptr, sfx_id: int, volume_in: int) {
	volume := volume_in
	origin := (^mobj_t)(origin_p)

	if sfx_id < 1 || sfx_id > int(sfxenum_t.NUMSFX) {
		I_Error("Bad sfx #: %d", i32(sfx_id))
	}

	sfx := &S_sfx[sfx_id]

	pitch, priority: int
	if sfx.link != nil {
		pitch = sfx.pitch
		priority = sfx.priority
		volume += sfx.volume

		if volume < 1 {
			return
		}
		if volume > snd_SfxVolume {
			volume = snd_SfxVolume
		}
	} else {
		pitch = NORM_PITCH
		priority = NORM_PRIORITY
	}

	sep: int
	if origin != nil && origin != players[consoleplayer].mo {
		rc := S_AdjustSoundParams(players[consoleplayer].mo, origin, &volume, &sep, &pitch)

		if origin.x == players[consoleplayer].mo.x && origin.y == players[consoleplayer].mo.y {
			sep = NORM_SEP
		}

		if rc == 0 {
			return
		}
	} else {
		sep = NORM_SEP
	}

	if sfx_id >= int(sfxenum_t.sfx_sawup) && sfx_id <= int(sfxenum_t.sfx_sawhit) {
		pitch += 8 - (M_Random() & 15)
		if pitch < 0 {
			pitch = 0
		} else if pitch > 255 {
			pitch = 255
		}
	} else if sfx_id != int(sfxenum_t.sfx_itemup) && sfx_id != int(sfxenum_t.sfx_tink) {
		pitch += 16 - (M_Random() & 31)
		if pitch < 0 {
			pitch = 0
		} else if pitch > 255 {
			pitch = 255
		}
	}

	S_StopSound(origin)

	cnum := S_getChannel(origin, sfx)
	if cnum < 0 {
		return
	}

	if sfx.lumpnum < 0 {
		sfx.lumpnum = I_GetSfxLumpNum(sfx)
	}

	old_use := sfx.usefulness
	sfx.usefulness += 1
	if old_use < 0 {
		sfx.usefulness = 1
	}

	channels[cnum].handle = I_StartSound(sfx_id, volume, sep, pitch, priority)
}

S_StartSound :: proc(origin: rawptr, sfx_id: int) {
	S_StartSoundAtVolume(origin, sfx_id, snd_SfxVolume)
}

S_StopSound :: proc(origin: rawptr) {
	for cnum := 0; cnum < numChannels; cnum += 1 {
		if channels[cnum].sfxinfo != nil && channels[cnum].origin == origin {
			S_StopChannel(cnum)
			break
		}
	}
}

S_PauseSound :: proc() {
	if mus_playing != nil && !mus_paused {
		I_PauseSong(mus_playing.handle)
		mus_paused = true
	}
}

S_ResumeSound :: proc() {
	if mus_playing != nil && mus_paused {
		I_ResumeSong(mus_playing.handle)
		mus_paused = false
	}
}

S_UpdateSounds :: proc(listener_p: rawptr) {
	listener := (^mobj_t)(listener_p)

	for cnum := 0; cnum < numChannels; cnum += 1 {
		c := &channels[cnum]
		sfx := c.sfxinfo

		if c.sfxinfo != nil {
			if I_SoundIsPlaying(c.handle) != 0 {
				volume := snd_SfxVolume
				pitch := NORM_PITCH
				sep := NORM_SEP

				if sfx.link != nil {
					pitch = sfx.pitch
					volume += sfx.volume
					if volume < 1 {
						S_StopChannel(cnum)
						continue
					} else if volume > snd_SfxVolume {
						volume = snd_SfxVolume
					}
				}

				if c.origin != nil && listener_p != c.origin {
					audible := S_AdjustSoundParams(listener, (^mobj_t)(c.origin), &volume, &sep, &pitch)
					if audible == 0 {
						S_StopChannel(cnum)
					} else {
						I_UpdateSoundParams(c.handle, volume, sep, pitch)
					}
				}
			} else {
				S_StopChannel(cnum)
			}
		}
	}
}

S_SetMusicVolume :: proc(volume: int) {
	if volume < 0 || volume > 127 {
		I_Error("Attempt to set music volume at %d", i32(volume))
	}
	I_SetMusicVolume(127)
	I_SetMusicVolume(volume)
	snd_MusicVolume = volume
}

S_SetSfxVolume :: proc(volume: int) {
	if volume < 0 || volume > 127 {
		I_Error("Attempt to set sfx volume at %d", i32(volume))
	}
	snd_SfxVolume = volume
}

S_StartMusic :: proc(m_id: int) {
	S_ChangeMusic(m_id, 0)
}

S_ChangeMusic :: proc(musicnum: int, looping: int) {
	namebuf: [9]u8

	if musicnum <= int(musicenum_t.mus_None) || musicnum >= int(musicenum_t.NUMMUSIC) {
		I_Error("Bad music number %d", i32(musicnum))
	}
	music := &S_music[musicnum]

	if mus_playing == music {
		return
	}

	S_StopMusic()

	if music.lumpnum == 0 {
		libc.snprintf(&namebuf[0], size_of(namebuf), "d_%s", music.name)
		music.lumpnum = W_GetNumForName(cstring(&namebuf[0]))
	}

	music.data = W_CacheLumpNum(music.lumpnum, PU_MUSIC)
	music.handle = I_RegisterSong(music.data)

	I_PlaySong(music.handle, looping)

	mus_playing = music
}

S_StopMusic :: proc() {
	if mus_playing != nil {
		if mus_paused {
			I_ResumeSong(mus_playing.handle)
		}

		I_StopSong(mus_playing.handle)
		I_UnRegisterSong(mus_playing.handle)
		Z_ChangeTag(mus_playing.data, PU_CACHE)

		mus_playing.data = nil
		mus_playing = nil
	}
}

S_StopChannel :: proc(cnum: int) {
	c := &channels[cnum]

	if c.sfxinfo != nil {
		if I_SoundIsPlaying(c.handle) != 0 {
			I_StopSound(c.handle)
		}

		for i := 0; i < numChannels; i += 1 {
			if cnum != i && c.sfxinfo == channels[i].sfxinfo {
				break
			}
		}

		c.sfxinfo.usefulness -= 1
		c.sfxinfo = nil
	}
}

S_AdjustSoundParams :: proc(listener: ^mobj_t, source: ^mobj_t, vol: ^int, sep: ^int, pitch: ^int) -> int {
	adx := abs(listener.x - source.x)
	ady := abs(listener.y - source.y)

	approx_dist := adx + ady - ((adx < ady ? adx : ady) >> 1)

	if gamemap != 8 && approx_dist > S_CLIPPING_DIST {
		return 0
	}

	angle := R_PointToAngle2(listener.x, listener.y, source.x, source.y)

	if angle > listener.angle {
		angle = angle - listener.angle
	} else {
		angle = angle + (0xffffffff - listener.angle)
	}

	angle >>= ANGLETOFINESHIFT

	sep^ = 128 - int(FixedMul(S_STEREO_SWING, finesine[angle]) >> FRACBITS)

	if approx_dist < S_CLOSE_DIST {
		vol^ = snd_SfxVolume
	} else if gamemap == 8 {
		if approx_dist > S_CLIPPING_DIST {
			approx_dist = S_CLIPPING_DIST
		}
		vol^ = 15 + ((snd_SfxVolume - 15) * int((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR
	} else {
		vol^ = (snd_SfxVolume * int((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR
	}

	return vol^ > 0 ? 1 : 0
}

S_getChannel :: proc(origin: rawptr, sfxinfo: ^sfxinfo_t) -> int {
	cnum := 0
	for ; cnum < numChannels; cnum += 1 {
		if channels[cnum].sfxinfo == nil {
			break
		} else if origin != nil && channels[cnum].origin == origin {
			S_StopChannel(cnum)
			break
		}
	}

	if cnum == numChannels {
		for cnum = 0; cnum < numChannels; cnum += 1 {
			if channels[cnum].sfxinfo.priority >= sfxinfo.priority {
				break
			}
		}

		if cnum == numChannels {
			return -1
		} else {
			S_StopChannel(cnum)
		}
	}

	c := &channels[cnum]
	c.sfxinfo = sfxinfo
	c.origin = origin

	return cnum
}