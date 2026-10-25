// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

PU_STATIC     :: 1
PU_SOUND      :: 2
PU_MUSIC      :: 3
PU_DAVE       :: 4
PU_LEVEL      :: 50
PU_LEVSPEC    :: 51
PU_PURGELEVEL :: 100
PU_CACHE      :: 101

memblock_t :: struct {
	size: i32,
	user: ^rawptr,
	tag:  i32,
	id:   i32,
	next: ^memblock_t,
	prev: ^memblock_t,
}
