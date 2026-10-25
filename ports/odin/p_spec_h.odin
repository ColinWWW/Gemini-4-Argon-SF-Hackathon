// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

MO_TELEPORTMAN :: 14

fireflicker_t :: struct {
	thinker:  thinker_t,
	sector:   ^sector_t,
	count:    int,
	maxlight: int,
	minlight: int,
}

lightflash_t :: struct {
	thinker:  thinker_t,
	sector:   ^sector_t,
	count:    int,
	maxlight: int,
	minlight: int,
	maxtime:  int,
	mintime:  int,
}

strobe_t :: struct {
	thinker:    thinker_t,
	sector:     ^sector_t,
	count:      int,
	minlight:   int,
	maxlight:   int,
	darktime:   int,
	brighttime: int,
}

glow_t :: struct {
	thinker:   thinker_t,
	sector:    ^sector_t,
	minlight:  int,
	maxlight:  int,
	direction: int,
}

GLOWSPEED    :: 8
STROBEBRIGHT :: 5
FASTDARK     :: 15
SLOWDARK     :: 35

switchlist_t :: struct {
	name1:   cstring,
	name2:   cstring,
	episode: i16,
}

bwhere_e :: enum i32 {
	top,
	middle,
	bottom,
}

button_t :: struct {
	line:     ^line_t,
	where_:   bwhere_e,
	btexture: int,
	btimer:   int,
	soundorg: ^mobj_t,
}

MAXSWITCHES :: 50
MAXBUTTONS  :: 16
BUTTONTIME  :: 35

plat_e :: enum i32 {
	up,
	down,
	waiting,
	in_stasis,
}

plattype_e :: enum i32 {
	perpetualRaise,
	downWaitUpStay,
	raiseAndChange,
	raiseToNearestAndChange,
	blazeDWUS,
}

plat_t :: struct {
	thinker:   thinker_t,
	sector:    ^sector_t,
	speed:     fixed_t,
	low:       fixed_t,
	high:      fixed_t,
	wait:      int,
	count:     int,
	status:    plat_e,
	oldstatus: plat_e,
	crush:     boolean,
	tag:       int,
	type:      plattype_e,
}

PLATWAIT  :: 3
PLATSPEED :: FRACUNIT
MAXPLATS  :: 30

vldoor_e :: enum i32 {
	normal,
	close30ThenOpen,
	close,
	open,
	raiseIn5Mins,
	blazeRaise,
	blazeOpen,
	blazeClose,
}

vldoor_t :: struct {
	thinker:      thinker_t,
	type:         vldoor_e,
	sector:       ^sector_t,
	topheight:    fixed_t,
	speed:        fixed_t,
	direction:    int,
	topwait:      int,
	topcountdown: int,
}

VDOORSPEED :: FRACUNIT * 2
VDOORWAIT  :: 150

ceiling_e :: enum i32 {
	lowerToFloor,
	raiseToHighest,
	lowerAndCrush,
	crushAndRaise,
	fastCrushAndRaise,
	silentCrushAndRaise,
}

ceiling_t :: struct {
	thinker:      thinker_t,
	type:         ceiling_e,
	sector:       ^sector_t,
	bottomheight: fixed_t,
	topheight:    fixed_t,
	speed:        fixed_t,
	crush:        boolean,
	direction:    int,
	tag:          int,
	olddirection: int,
}

CEILSPEED   :: FRACUNIT
CEILWAIT    :: 150
MAXCEILINGS :: 30

floor_e :: enum i32 {
	lowerFloor,
	lowerFloorToLowest,
	turboLower,
	raiseFloor,
	raiseFloorToNearest,
	raiseToTexture,
	lowerAndChange,
	raiseFloor24,
	raiseFloor24AndChange,
	raiseFloorCrush,
	raiseFloorTurbo,
	donutRaise,
	raiseFloor512,
}

stair_e :: enum i32 {
	build8,
	turbo16,
}

floormove_t :: struct {
	thinker:         thinker_t,
	type:            floor_e,
	crush:           boolean,
	sector:          ^sector_t,
	direction:       int,
	newspecial:      int,
	texture:         i16,
	floordestheight: fixed_t,
	speed:           fixed_t,
}

FLOORSPEED :: FRACUNIT

result_e :: enum i32 {
	ok,
	crushed,
	pastdest,
}