// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

MF_SPECIAL      :: 1
MF_SOLID        :: 2
MF_SHOOTABLE    :: 4
MF_NOSECTOR     :: 8
MF_NOBLOCKMAP   :: 16
MF_AMBUSH       :: 32
MF_JUSTHIT      :: 64
MF_JUSTATTACKED :: 128
MF_SPAWNCEILING :: 256
MF_NOGRAVITY    :: 512
MF_DROPOFF      :: 0x400
MF_PICKUP       :: 0x800
MF_NOCLIP       :: 0x1000
MF_SLIDE        :: 0x2000
MF_FLOAT        :: 0x4000
MF_TELEPORT     :: 0x8000
MF_MISSILE      :: 0x10000
MF_DROPPED      :: 0x20000
MF_SHADOW       :: 0x40000
MF_NOBLOOD      :: 0x80000
MF_CORPSE       :: 0x100000
MF_INFLOAT      :: 0x200000
MF_COUNTKILL    :: 0x400000
MF_COUNTITEM    :: 0x800000
MF_SKULLFLY     :: 0x1000000
MF_NOTDMATCH    :: 0x2000000
MF_TRANSLATION  :: 0xc000000
MF_TRANSSHIFT   :: 26

mobj_t :: struct {
	thinker:      thinker_t,
	x:            fixed_t,
	y:            fixed_t,
	z:            fixed_t,
	snext:        ^mobj_t,
	sprev:        ^mobj_t,
	angle:        angle_t,
	sprite:       spritenum_t,
	frame:        int,
	bnext:        ^mobj_t,
	bprev:        ^mobj_t,
	subsector:    ^subsector_t,
	floorz:       fixed_t,
	ceilingz:     fixed_t,
	radius:       fixed_t,
	height:       fixed_t,
	momx:         fixed_t,
	momy:         fixed_t,
	momz:         fixed_t,
	validcount:   int,
	type:         mobjtype_t,
	info:         ^mobjinfo_t,
	tics:         int,
	state:        ^state_t,
	flags:        int,
	health:       int,
	movedir:      int,
	movecount:    int,
	target:       ^mobj_t,
	reactiontime: int,
	threshold:    int,
	player:       ^player_t,
	lastlook:     int,
	spawnpoint:   mapthing_t,
	tracer:       ^mobj_t,
}
