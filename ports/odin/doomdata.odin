// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

ML_LABEL    :: 0
ML_THINGS   :: 1
ML_LINEDEFS :: 2
ML_SIDEDEFS :: 3
ML_VERTEXES :: 4
ML_SEGS     :: 5
ML_SSECTORS :: 6
ML_NODES    :: 7
ML_SECTORS  :: 8
ML_REJECT   :: 9
ML_BLOCKMAP :: 10

mapvertex_t :: struct #packed {
	x: i16,
	y: i16,
}

mapsidedef_t :: struct #packed {
	textureoffset: i16,
	rowoffset:     i16,
	toptexture:    [8]u8,
	bottomtexture: [8]u8,
	midtexture:    [8]u8,
	sector:        i16,
}

maplinedef_t :: struct #packed {
	v1:      i16,
	v2:      i16,
	flags:   i16,
	special: i16,
	tag:     i16,
	sidenum: [2]i16,
}

ML_BLOCKING      :: 1
ML_BLOCKMONSTERS :: 2
ML_TWOSIDED      :: 4
ML_DONTPEGTOP    :: 8
ML_DONTPEGBOTTOM :: 16
ML_SECRET        :: 32
ML_SOUNDBLOCK    :: 64
ML_DONTDRAW      :: 128
ML_MAPPED        :: 256

mapsector_t :: struct #packed {
	floorheight:   i16,
	ceilingheight: i16,
	floorpic:      [8]u8,
	ceilingpic:    [8]u8,
	lightlevel:    i16,
	special:       i16,
	tag:           i16,
}

mapsubsector_t :: struct #packed {
	numsegs:  i16,
	firstseg: i16,
}

mapseg_t :: struct #packed {
	v1:      i16,
	v2:      i16,
	angle:   i16,
	linedef: i16,
	side:    i16,
	offset:  i16,
}

NF_SUBSECTOR :: 0x8000

mapnode_t :: struct #packed {
	x:        i16,
	y:        i16,
	dx:       i16,
	dy:       i16,
	bbox:     [2][4]i16,
	children: [2]u16,
}

mapthing_t :: struct #packed {
	x:       i16,
	y:       i16,
	angle:   i16,
	type:    i16,
	options: i16,
}
