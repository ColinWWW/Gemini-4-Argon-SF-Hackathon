// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

I_InitNetwork :: proc() {
	doomcom = (^doomcom_t)(libc.malloc(size_of(doomcom_t)))
	libc.memset(doomcom, 0, size_of(doomcom_t))

	netgame = false
	doomcom.id = DOOMCOM_ID
	doomcom.numplayers = 1
	doomcom.numnodes = 1
	doomcom.deathmatch = 0
	doomcom.consoleplayer = 0
	doomcom.ticdup = 1
	doomcom.extratics = 0
}

I_NetCmd :: proc() {
	I_Error("I_NetCmd: Network play not supported in headless reference build")
}
