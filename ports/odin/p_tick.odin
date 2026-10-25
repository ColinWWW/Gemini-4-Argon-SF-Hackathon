// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

leveltime: int
thinkercap: thinker_t

P_InitThinkers :: proc() {
	thinkercap.prev = &thinkercap
	thinkercap.next = &thinkercap
}

P_AddThinker :: proc(thinker: ^thinker_t) {
	thinkercap.prev.next = thinker
	thinker.next = &thinkercap
	thinker.prev = thinkercap.prev
	thinkercap.prev = thinker
}

P_RemoveThinker :: proc(thinker: ^thinker_t) {
	thinker.function.int_val = -1
}

P_AllocateThinker :: proc(thinker: ^thinker_t) {
}

P_RunThinkers :: proc() {
	currentthinker := thinkercap.next
	for currentthinker != &thinkercap {
		if currentthinker.function.int_val == -1 {
			currentthinker.next.prev = currentthinker.prev
			currentthinker.prev.next = currentthinker.next
			Z_Free(currentthinker)
		} else {
			if currentthinker.function.acp1 != nil {
				currentthinker.function.acp1(currentthinker)
			}
		}
		currentthinker = currentthinker.next
	}
}

P_Ticker :: proc() {
	if paused {
		return
	}

	if !netgame && menuactive && !demoplayback && players[consoleplayer].viewz != 1 {
		return
	}

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			P_PlayerThink(&players[i])
		}
	}

	P_RunThinkers()
	P_UpdateSpecials()
	P_RespawnSpecials()

	leveltime += 1
}
