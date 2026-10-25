// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

ST_BG :: 4
ST_FG :: 0

st_number_t :: struct {
	x:      int,
	y:      int,
	width:  int,
	oldnum: int,
	num:    ^int,
	on:     ^boolean,
	p:      [^]^patch_t,
	data:   int,
}

st_percent_t :: struct {
	n: st_number_t,
	p: ^patch_t,
}

st_multicon_t :: struct {
	x:       int,
	y:       int,
	oldinum: int,
	inum:    ^int,
	on:      ^boolean,
	p:       [^]^patch_t,
	data:    int,
}

st_binicon_t :: struct {
	x:      int,
	y:      int,
	oldval: int,
	val:    ^boolean,
	on:     ^boolean,
	p:      ^patch_t,
	data:   int,
}
