// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

current_palette: [768]byte
frame_count: int = 0
empty_screen: [SCREENWIDTH * SCREENHEIGHT]byte

ensure_dir :: proc(dir: cstring) {
	tmp: [512]u8
	libc.snprintf(&tmp[0], 512, "%s", dir)
	length := int(libc.strlen(cstring(&tmp[0])))
	if length > 0 && (tmp[length - 1] == '/' || tmp[length - 1] == '\\') {
		tmp[length - 1] = 0
	}

	for i := 1; tmp[i] != 0; i += 1 {
		if tmp[i] == '/' || tmp[i] == '\\' {
			tmp[i] = 0
			c_mkdir(cstring(&tmp[0]))
			tmp[i] = '/'
		}
	}
	c_mkdir(cstring(&tmp[0]))
}

I_InitGraphics :: proc() {
	if screens[0] == nil {
		screens[0] = ([^]byte)(libc.malloc(SCREENWIDTH * SCREENHEIGHT))
		libc.memset(screens[0], 0, SCREENWIDTH * SCREENHEIGHT)
	}
}

I_ShutdownGraphics :: proc() {
}

I_SetPalette :: proc(palette: [^]byte) {
	if palette != nil {
		libc.memcpy(&current_palette[0], palette, 768)
	}
}

I_UpdateNoBlit :: proc() {
}

I_FinishUpdate :: proc() {
	filename: [512]u8
	frame_dir := libc.getenv("DOOM_FRAME_DIR")
	if frame_dir == nil || ((^u8)(frame_dir))^ == 0 {
		frame_dir = "frames/odin"
	}

	ensure_dir(frame_dir)

	libc.snprintf(&filename[0], 512, "%s/%05d.raw", frame_dir, i32(frame_count))
	frame_count += 1
	f := libc.fopen(cstring(&filename[0]), "wb")
	if f != nil {
		if screens[0] != nil {
			libc.fwrite(screens[0], 1, SCREENWIDTH * SCREENHEIGHT, f)
		} else {
			libc.fwrite(&empty_screen[0], 1, SCREENWIDTH * SCREENHEIGHT, f)
		}
		libc.fwrite(&current_palette[0], 1, 768, f)
		libc.fclose(f)
	}
}

I_ReadScreen :: proc(scr: [^]byte) {
	if screens[0] != nil && scr != nil {
		libc.memcpy(scr, screens[0], SCREENWIDTH * SCREENHEIGHT)
	}
}

I_StartTic :: proc() {
}

I_StartFrame :: proc() {
}
