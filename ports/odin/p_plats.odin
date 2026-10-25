// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

activeplats: [MAXPLATS]^plat_t

T_PlatRaise :: proc(plat_raw: rawptr) {
	plat := (^plat_t)(plat_raw)

	switch plat.status {
	case .up:
		res := T_MovePlane(plat.sector, plat.speed, plat.high, plat.crush, 0, 1)

		if plat.type == .raiseAndChange || plat.type == .raiseToNearestAndChange {
			if (leveltime & 7) == 0 {
				S_StartSound(&plat.sector.soundorg, int(sfxenum_t.sfx_stnmov))
			}
		}

		if res == .crushed && !plat.crush {
			plat.count = plat.wait
			plat.status = .down
			S_StartSound(&plat.sector.soundorg, int(sfxenum_t.sfx_pstart))
		} else {
			if res == .pastdest {
				plat.count = plat.wait
				plat.status = .waiting
				S_StartSound(&plat.sector.soundorg, int(sfxenum_t.sfx_pstop))

				#partial switch plat.type {
				case .blazeDWUS, .downWaitUpStay, .raiseAndChange, .raiseToNearestAndChange:
					P_RemoveActivePlat(plat)
				}
			}
		}
	case .down:
		res := T_MovePlane(plat.sector, plat.speed, plat.low, false, 0, -1)
		if res == .pastdest {
			plat.count = plat.wait
			plat.status = .waiting
			S_StartSound(&plat.sector.soundorg, int(sfxenum_t.sfx_pstop))
		}
	case .waiting:
		plat.count -= 1
		if plat.count == 0 {
			if plat.sector.floorheight == plat.low {
				plat.status = .up
			} else {
				plat.status = .down
			}
			S_StartSound(&plat.sector.soundorg, int(sfxenum_t.sfx_pstart))
		}
	case .in_stasis:
	}
}

EV_DoPlat :: proc(line: ^line_t, type: plattype_e, amount: int) -> int {
	secnum := -1
	rtn := 0

	#partial switch type {
	case .perpetualRaise:
		P_ActivateInStasis(int(line.tag))
	}

	for {
		secnum = P_FindSectorFromLineTag(line, secnum)
		if secnum < 0 {
			break
		}
		sec := &sectors[secnum]

		if sec.specialdata != nil {
			continue
		}

		rtn = 1
		plat := (^plat_t)(Z_Malloc(size_of(plat_t), PU_LEVSPEC, nil))
		P_AddThinker(&plat.thinker)

		plat.type = type
		plat.sector = sec
		plat.sector.specialdata = plat
		plat.thinker.function.acp1 = T_PlatRaise
		plat.crush = false
		plat.tag = int(line.tag)

		switch type {
		case .raiseToNearestAndChange:
			plat.speed = PLATSPEED / 2
			sec.floorpic = sides[line.sidenum[0]].sector.floorpic
			plat.high = P_FindNextHighestFloor(sec, int(sec.floorheight))
			plat.wait = 0
			plat.status = .up
			sec.special = 0
			S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_stnmov))
		case .raiseAndChange:
			plat.speed = PLATSPEED / 2
			sec.floorpic = sides[line.sidenum[0]].sector.floorpic
			plat.high = sec.floorheight + fixed_t(amount * FRACUNIT)
			plat.wait = 0
			plat.status = .up
			S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_stnmov))
		case .downWaitUpStay:
			plat.speed = PLATSPEED * 4
			plat.low = P_FindLowestFloorSurrounding(sec)
			if plat.low > sec.floorheight {
				plat.low = sec.floorheight
			}
			plat.high = sec.floorheight
			plat.wait = 35 * PLATWAIT
			plat.status = .down
			S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_pstart))
		case .blazeDWUS:
			plat.speed = PLATSPEED * 8
			plat.low = P_FindLowestFloorSurrounding(sec)
			if plat.low > sec.floorheight {
				plat.low = sec.floorheight
			}
			plat.high = sec.floorheight
			plat.wait = 35 * PLATWAIT
			plat.status = .down
			S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_pstart))
		case .perpetualRaise:
			plat.speed = PLATSPEED
			plat.low = P_FindLowestFloorSurrounding(sec)
			if plat.low > sec.floorheight {
				plat.low = sec.floorheight
			}
			plat.high = P_FindHighestFloorSurrounding(sec)
			if plat.high < sec.floorheight {
				plat.high = sec.floorheight
			}
			plat.wait = 35 * PLATWAIT
			plat.status = plat_e(P_Random() & 1)
			S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_pstart))
		}
		P_AddActivePlat(plat)
	}
	return rtn
}

P_ActivateInStasis :: proc(tag: int) {
	for i := 0; i < MAXPLATS; i += 1 {
		if activeplats[i] != nil && activeplats[i].tag == tag && activeplats[i].status == .in_stasis {
			activeplats[i].status = activeplats[i].oldstatus
			activeplats[i].thinker.function.acp1 = T_PlatRaise
		}
	}
}

EV_StopPlat :: proc(line: ^line_t) {
	for j := 0; j < MAXPLATS; j += 1 {
		if activeplats[j] != nil && activeplats[j].status != .in_stasis && activeplats[j].tag == int(line.tag) {
			activeplats[j].oldstatus = activeplats[j].status
			activeplats[j].status = .in_stasis
			activeplats[j].thinker.function.acv = nil
		}
	}
}

P_AddActivePlat :: proc(plat: ^plat_t) {
	for i := 0; i < MAXPLATS; i += 1 {
		if activeplats[i] == nil {
			activeplats[i] = plat
			return
		}
	}
	I_Error("P_AddActivePlat: no more plats!")
}

P_RemoveActivePlat :: proc(plat: ^plat_t) {
	for i := 0; i < MAXPLATS; i += 1 {
		if plat == activeplats[i] {
			activeplats[i].sector.specialdata = nil
			P_RemoveThinker(&activeplats[i].thinker)
			activeplats[i] = nil
			return
		}
	}
	I_Error("P_RemoveActivePlat: can't find plat!")
}
