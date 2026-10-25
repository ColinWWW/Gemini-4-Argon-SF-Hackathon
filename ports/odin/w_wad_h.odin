// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

wadinfo_t :: struct #packed {
	identification: [4]u8,
	numlumps:       i32,
	infotableofs:   i32,
}

filelump_t :: struct #packed {
	filepos: i32,
	size:    i32,
	name:    [8]u8,
}

lumpinfo_t :: struct {
	name:     [8]u8,
	handle:   ^libc.FILE,
	position: int,
	size:     int,
}
