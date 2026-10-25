// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

HU_BG :: 1
HU_FG :: 0

HU_CHARERASE     :: KEY_BACKSPACE
HU_MAXLINES      :: 4
HU_MAXLINELENGTH :: 80

hu_textline_t :: struct {
	x:           int,
	y:           int,
	f:           [^]^patch_t,
	sc:          int,
	l:           [HU_MAXLINELENGTH + 1]u8,
	len:         int,
	needsupdate: int,
}

hu_stext_t :: struct {
	l:      [HU_MAXLINES]hu_textline_t,
	h:      int,
	cl:     int,
	on:     ^boolean,
	laston: boolean,
}

hu_itext_t :: struct {
	l:      hu_textline_t,
	lm:     int,
	on:     ^boolean,
	laston: boolean,
}
