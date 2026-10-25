// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

activeceilings: [MAXCEILINGS]^ceiling_t

T_MoveCeiling :: proc(ceiling_raw: rawptr) {
	ceiling := (^ceiling_t)(ceiling_raw)

	switch ceiling.direction {
	case 0:
	case 1:
		res := T_MovePlane(ceiling.sector, ceiling.speed, ceiling.topheight, false, 1, ceiling.direction)

		if (leveltime & 7) == 0 {
			#partial switch ceiling.type {
			case .silentCrushAndRaise:
			case:
				S_StartSound(&ceiling.sector.soundorg, int(sfxenum_t.sfx_stnmov))
			}
		}

		if res == .pastdest {
			#partial switch ceiling.type {
			case .raiseToHighest:
				P_RemoveActiveCeiling(ceiling)
			case .silentCrushAndRaise:
				S_StartSound(&ceiling.sector.soundorg, int(sfxenum_t.sfx_pstop))
				ceiling.direction = -1
			case .fastCrushAndRaise, .crushAndRaise:
				ceiling.direction = -1
			}
		}
	case -1:
		res := T_MovePlane(ceiling.sector, ceiling.speed, ceiling.bottomheight, ceiling.crush, 1, ceiling.direction)

		if (leveltime & 7) == 0 {
			#partial switch ceiling.type {
			case .silentCrushAndRaise:
			case:
				S_StartSound(&ceiling.sector.soundorg, int(sfxenum_t.sfx_stnmov))
			}
		}

		if res == .pastdest {
			#partial switch ceiling.type {
			case .silentCrushAndRaise:
				S_StartSound(&ceiling.sector.soundorg, int(sfxenum_t.sfx_pstop))
				ceiling.speed = CEILSPEED
				ceiling.direction = 1
			case .crushAndRaise:
				ceiling.speed = CEILSPEED
				ceiling.direction = 1
			case .fastCrushAndRaise:
				ceiling.direction = 1
			case .lowerAndCrush, .lowerToFloor:
				P_RemoveActiveCeiling(ceiling)
			}
		} else {
			if res == .crushed {
				#partial switch ceiling.type {
				case .silentCrushAndRaise, .crushAndRaise, .lowerAndCrush:
					ceiling.speed = CEILSPEED / 8
				}
			}
		}
	}
}

EV_DoCeiling :: proc(line: ^line_t, type: ceiling_e) -> int {
	secnum := -1
	rtn := 0

	#partial switch type {
	case .fastCrushAndRaise, .silentCrushAndRaise, .crushAndRaise:
		P_ActivateInStasisCeiling(line)
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
		ceiling := (^ceiling_t)(Z_Malloc(size_of(ceiling_t), PU_LEVSPEC, nil))
		P_AddThinker(&ceiling.thinker)
		sec.specialdata = ceiling
		ceiling.thinker.function.acp1 = T_MoveCeiling
		ceiling.sector = sec
		ceiling.crush = false

		switch type {
		case .fastCrushAndRaise:
			ceiling.crush = true
			ceiling.topheight = sec.ceilingheight
			ceiling.bottomheight = sec.floorheight + (8 * FRACUNIT)
			ceiling.direction = -1
			ceiling.speed = CEILSPEED * 2
		case .silentCrushAndRaise, .crushAndRaise:
			ceiling.crush = true
			ceiling.topheight = sec.ceilingheight
			ceiling.bottomheight = sec.floorheight + (8 * FRACUNIT)
			ceiling.direction = -1
			ceiling.speed = CEILSPEED
		case .lowerAndCrush, .lowerToFloor:
			ceiling.bottomheight = sec.floorheight
			if type != .lowerToFloor {
				ceiling.bottomheight += 8 * FRACUNIT
			}
			ceiling.direction = -1
			ceiling.speed = CEILSPEED
		case .raiseToHighest:
			ceiling.topheight = P_FindHighestCeilingSurrounding(sec)
			ceiling.direction = 1
			ceiling.speed = CEILSPEED
		}

		ceiling.tag = int(sec.tag)
		ceiling.type = type
		P_AddActiveCeiling(ceiling)
	}
	return rtn
}

P_AddActiveCeiling :: proc(c: ^ceiling_t) {
	for i := 0; i < MAXCEILINGS; i += 1 {
		if activeceilings[i] == nil {
			activeceilings[i] = c
			return
		}
	}
}

P_RemoveActiveCeiling :: proc(c: ^ceiling_t) {
	for i := 0; i < MAXCEILINGS; i += 1 {
		if activeceilings[i] == c {
			activeceilings[i].sector.specialdata = nil
			P_RemoveThinker(&activeceilings[i].thinker)
			activeceilings[i] = nil
			break
		}
	}
}

P_ActivateInStasisCeiling :: proc(line: ^line_t) {
	for i := 0; i < MAXCEILINGS; i += 1 {
		if activeceilings[i] != nil && activeceilings[i].tag == int(line.tag) && activeceilings[i].direction == 0 {
			activeceilings[i].direction = activeceilings[i].olddirection
			activeceilings[i].thinker.function.acp1 = T_MoveCeiling
		}
	}
}

EV_CeilingCrushStop :: proc(line: ^line_t) -> int {
	rtn := 0
	for i := 0; i < MAXCEILINGS; i += 1 {
		if activeceilings[i] != nil && activeceilings[i].tag == int(line.tag) && activeceilings[i].direction != 0 {
			activeceilings[i].olddirection = activeceilings[i].direction
			activeceilings[i].thinker.function.acv = nil
			activeceilings[i].direction = 0
			rtn = 1
		}
	}
	return rtn
}
