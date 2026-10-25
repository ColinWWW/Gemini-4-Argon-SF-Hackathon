// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

myargc: int
myargv: [^]cstring

M_CheckParm :: proc(check: cstring) -> int {
	for i := 1; i < myargc; i += 1 {
		if c_strcasecmp(check, myargv[i]) == 0 {
			return i
		}
	}
	return 0
}
