// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

sndserver_filename: cstring = "./sndserver "

I_InitSound :: proc() {}
I_UpdateSound :: proc() {}
I_SubmitSound :: proc() {}
I_ShutdownSound :: proc() {}
I_SetChannels :: proc() {}

I_GetSfxLumpNum :: proc(sfxinfo: ^sfxinfo_t) -> int {
	return 0
}

I_StartSound :: proc(id: int, vol: int, sep: int, pitch: int, priority: int) -> int {
	return 1
}

I_StopSound :: proc(handle: int) {}

I_SoundIsPlaying :: proc(handle: int) -> int {
	return 0
}

I_UpdateSoundParams :: proc(handle: int, vol: int, sep: int, pitch: int) {}

I_InitMusic :: proc() {}
I_ShutdownMusic :: proc() {}
I_SetMusicVolume :: proc(volume: int) {}
I_PauseSong :: proc(handle: int) {}
I_ResumeSong :: proc(handle: int) {}

I_RegisterSong :: proc(data: rawptr) -> int {
	return 1
}

I_PlaySong :: proc(handle: int, looping: int) {}
I_StopSong :: proc(handle: int) {}
I_UnRegisterSong :: proc(handle: int) {}
