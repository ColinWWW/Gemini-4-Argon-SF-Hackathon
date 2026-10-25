// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

FF_FULLBRIGHT :: 0x8000
FF_FRAMEMASK  :: 0x7fff

psprnum_t :: enum i32 {
	ps_weapon,
	ps_flash,
	NUMPSPRITES,
}
ps_weapon   :: int(psprnum_t.ps_weapon)
ps_flash    :: int(psprnum_t.ps_flash)
NUMPSPRITES :: int(psprnum_t.NUMPSPRITES)

pspdef_t :: struct {
	state: ^state_t,
	tics:  int,
	sx:    fixed_t,
	sy:    fixed_t,
}
