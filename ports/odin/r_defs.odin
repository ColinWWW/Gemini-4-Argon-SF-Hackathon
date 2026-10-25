// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

SIL_NONE   :: 0
SIL_BOTTOM :: 1
SIL_TOP    :: 2
SIL_BOTH   :: 3

MAXDRAWSEGS :: 256

vertex_t :: struct {
	x: fixed_t,
	y: fixed_t,
}

degenmobj_t :: struct {
	thinker: thinker_t,
	x:       fixed_t,
	y:       fixed_t,
	z:       fixed_t,
}

sector_t :: struct {
	floorheight:    fixed_t,
	ceilingheight:  fixed_t,
	floorpic:       i16,
	ceilingpic:     i16,
	lightlevel:     i16,
	special:        i16,
	tag:            i16,
	soundtraversed: int,
	soundtarget:    ^mobj_t,
	blockbox:       [4]int,
	soundorg:       degenmobj_t,
	validcount:     int,
	thinglist:      ^mobj_t,
	specialdata:    rawptr,
	linecount:      int,
	lines:          [^]^line_t,
}

side_t :: struct {
	textureoffset: fixed_t,
	rowoffset:     fixed_t,
	toptexture:    i16,
	bottomtexture: i16,
	midtexture:    i16,
	sector:        ^sector_t,
}

slopetype_t :: enum i32 {
	ST_HORIZONTAL,
	ST_VERTICAL,
	ST_POSITIVE,
	ST_NEGATIVE,
}

line_t :: struct {
	v1:          ^vertex_t,
	v2:          ^vertex_t,
	dx:          fixed_t,
	dy:          fixed_t,
	flags:       i16,
	special:     i16,
	tag:         i16,
	sidenum:     [2]i16,
	bbox:        [4]fixed_t,
	slopetype:   slopetype_t,
	frontsector: ^sector_t,
	backsector:  ^sector_t,
	validcount:  int,
	specialdata: rawptr,
}

subsector_t :: struct {
	sector:    ^sector_t,
	numlines:  i16,
	firstline: i16,
}

seg_t :: struct {
	v1:          ^vertex_t,
	v2:          ^vertex_t,
	offset:      fixed_t,
	angle:       angle_t,
	sidedef:     ^side_t,
	linedef:     ^line_t,
	frontsector: ^sector_t,
	backsector:  ^sector_t,
}

node_t :: struct {
	x:        fixed_t,
	y:        fixed_t,
	dx:       fixed_t,
	dy:       fixed_t,
	bbox:     [2][4]fixed_t,
	children: [2]u16,
}

post_t :: struct #packed {
	topdelta: byte,
	length:   byte,
}

column_t :: post_t

lighttable_t :: byte

drawseg_t :: struct {
	curline:          ^seg_t,
	x1:               int,
	x2:               int,
	scale1:           fixed_t,
	scale2:           fixed_t,
	scalestep:        fixed_t,
	silhouette:       int,
	bsilheight:       fixed_t,
	tsilheight:       fixed_t,
	sprtopclip:       [^]i16,
	sprbottomclip:    [^]i16,
	maskedtexturecol: [^]i16,
}

patch_t :: struct #packed {
	width:      i16,
	height:     i16,
	leftoffset: i16,
	topoffset:  i16,
	columnofs:  [8]i32,
}

vissprite_t :: struct {
	prev:       ^vissprite_t,
	next:       ^vissprite_t,
	x1:         int,
	x2:         int,
	gx:         fixed_t,
	gy:         fixed_t,
	gz:         fixed_t,
	gzt:        fixed_t,
	startfrac:  fixed_t,
	scale:      fixed_t,
	xiscale:    fixed_t,
	texturemid: fixed_t,
	patch:      int,
	colormap:   [^]lighttable_t,
	mobjflags:  int,
}

spriteframe_t :: struct {
	rotate: int,
	lump:   [8]i16,
	flip:   [8]byte,
}

spritedef_t :: struct {
	numframes:    int,
	spriteframes: [^]spriteframe_t,
}

visplane_t :: struct {
	height:     fixed_t,
	picnum:     int,
	lightlevel: int,
	minx:       int,
	maxx:       int,
	pad1:       byte,
	top:        [SCREENWIDTH]byte,
	pad2:       byte,
	pad3:       byte,
	bottom:     [SCREENWIDTH]byte,
	pad4:       byte,
}
