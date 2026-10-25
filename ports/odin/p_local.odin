// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

FLOATSPEED :: FRACUNIT * 4
MAXHEALTH  :: 100
VIEWHEIGHT :: 41 * FRACUNIT

MAPBLOCKUNITS :: 128
MAPBLOCKSIZE  :: MAPBLOCKUNITS * FRACUNIT
MAPBLOCKSHIFT :: FRACBITS + 7
MAPBMASK      :: MAPBLOCKSIZE - 1
MAPBTOFRAC    :: MAPBLOCKSHIFT - FRACBITS

PLAYERRADIUS :: 16 * FRACUNIT
MAXRADIUS    :: 32 * FRACUNIT

GRAVITY      :: FRACUNIT
MAXMOVE      :: 30 * FRACUNIT

USERANGE     :: 64 * FRACUNIT
MELEERANGE   :: 64 * FRACUNIT
MISSILERANGE :: 32 * 64 * FRACUNIT

BASETHRESHOLD :: 100

ONFLOORZ   :: fixed_t(MININT)
ONCEILINGZ :: fixed_t(MAXINT)

ITEMQUESIZE :: 128

divline_t :: struct {
	x:  fixed_t,
	y:  fixed_t,
	dx: fixed_t,
	dy: fixed_t,
}

intercept_d_t :: struct #raw_union {
	thing: ^mobj_t,
	line:  ^line_t,
}

intercept_t :: struct {
	frac:    fixed_t,
	isaline: boolean,
	d:       intercept_d_t,
}

MAXINTERCEPTS :: 128

traverser_t :: #type proc(in_ptr: ^intercept_t) -> boolean

PT_ADDLINES  :: 1
PT_ADDTHINGS :: 2
PT_EARLYOUT  :: 4
