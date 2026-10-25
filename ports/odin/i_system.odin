// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"
import "core:fmt"

when ODIN_OS == .Windows {
	foreign import crt "system:libucrt.lib"
	@(default_calling_convention="c")
	foreign crt {
		@(link_name="_mkdir") _win_mkdir :: proc(path: cstring) -> i32 ---
		@(link_name="_access") _win_access :: proc(path: cstring, mode: i32) -> i32 ---
		@(link_name="_stricmp") _win_stricmp :: proc(s1: cstring, s2: cstring) -> i32 ---
		@(link_name="_strnicmp") _win_strnicmp :: proc(s1: cstring, s2: cstring, n: uint) -> i32 ---
	}
	c_mkdir :: proc(path: cstring) { _win_mkdir(path) }
	c_access :: proc(path: cstring, mode: i32) -> i32 { return _win_access(path, mode) }
	c_strcasecmp :: proc(s1: cstring, s2: cstring) -> i32 { return _win_stricmp(s1, s2) }
	c_strncasecmp :: proc(s1: cstring, s2: cstring, n: uint) -> i32 { return _win_strnicmp(s1, s2, n) }
} else {
	foreign import posix "system:c"
	@(default_calling_convention="c")
	foreign posix {
		mkdir :: proc(path: cstring, mode: u32) -> i32 ---
		access :: proc(path: cstring, mode: i32) -> i32 ---
		strcasecmp :: proc(s1: cstring, s2: cstring) -> i32 ---
		strncasecmp :: proc(s1: cstring, s2: cstring, n: uint) -> i32 ---
	}
	c_mkdir :: proc(path: cstring) { mkdir(path, 0o777) }
	c_access :: proc(path: cstring, mode: i32) -> i32 { return access(path, mode) }
	c_strcasecmp :: proc(s1: cstring, s2: cstring) -> i32 { return strcasecmp(s1, s2) }
	c_strncasecmp :: proc(s1: cstring, s2: cstring, n: uint) -> i32 { return strncasecmp(s1, s2, n) }
}

R_OK :: 4

mb_used: int = 6
emptycmd: ticcmd_t
fake_tics: int = 0

I_Tactile :: proc(on: int, off: int, total: int) {
}

I_BaseTiccmd :: proc() -> ^ticcmd_t {
	return &emptycmd
}

I_GetHeapSize :: proc() -> int {
	return mb_used * 1024 * 1024
}

I_ZoneBase :: proc(size: ^int) -> [^]byte {
	size^ = mb_used * 1024 * 1024
	return ([^]byte)(libc.malloc(uint(size^)))
}

I_GetTime :: proc() -> int {
	res := fake_tics
	fake_tics += 1
	return res
}

I_Init :: proc() {
	I_InitSound()
}

I_Quit :: proc() {
	D_QuitNetGame()
	I_ShutdownSound()
	I_ShutdownMusic()
	M_SaveDefaults()
	I_ShutdownGraphics()
	libc.exit(0)
}

I_WaitVBL :: proc(count: int) {
}

I_BeginRead :: proc() {
}

I_EndRead :: proc() {
}

I_AllocLow :: proc(length: int) -> [^]byte {
	mem := ([^]byte)(libc.malloc(uint(length)))
	libc.memset(mem, 0, uint(length))
	return mem
}

I_Error :: proc(error: cstring, args: ..any) {
	fmt.eprint("Error: ")
	if error != nil {
		if len(args) > 0 {
			fmt.eprintf(string(error), ..args)
		} else {
			fmt.eprint(string(error))
		}
	}
	fmt.eprintln()
	libc.fflush(libc.stderr)

	if demorecording {
		G_CheckDemoStatus()
	}

	D_QuitNetGame()
	I_ShutdownGraphics()

	libc.exit(-1)
}