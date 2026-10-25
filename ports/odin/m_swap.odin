// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

SwapSHORT :: proc(x: u16) -> u16 {
	return (x >> 8) | (x << 8)
}

SwapLONG :: proc(x: u32) -> u32 {
	return (x >> 24) | ((x >> 8) & 0xff00) | ((x << 8) & 0xff0000) | (x << 24)
}
