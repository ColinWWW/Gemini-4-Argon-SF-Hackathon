// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

weaponinfo_t :: struct {
	ammo:       ammotype_t,
	upstate:    int,
	downstate:  int,
	readystate: int,
	atkstate:   int,
	flashstate: int,
}
