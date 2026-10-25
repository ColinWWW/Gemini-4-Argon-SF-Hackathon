// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

T_VerticalDoor :: proc(door_raw: rawptr) {
	door := (^vldoor_t)(door_raw)

	switch door.direction {
	case 0:
		door.topcountdown -= 1
		if door.topcountdown == 0 {
			#partial switch door.type {
			case .blazeRaise:
				door.direction = -1
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_bdcls))
			case .normal:
				door.direction = -1
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_dorcls))
			case .close30ThenOpen:
				door.direction = 1
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_doropn))
			}
		}
	case 2:
		door.topcountdown -= 1
		if door.topcountdown == 0 {
			#partial switch door.type {
			case .raiseIn5Mins:
				door.direction = 1
				door.type = .normal
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_doropn))
			}
		}
	case -1:
		res := T_MovePlane(door.sector, door.speed, door.sector.floorheight, false, 1, door.direction)
		if res == .pastdest {
			#partial switch door.type {
			case .blazeRaise, .blazeClose:
				door.sector.specialdata = nil
				P_RemoveThinker(&door.thinker)
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_bdcls))
			case .normal, .close:
				door.sector.specialdata = nil
				P_RemoveThinker(&door.thinker)
			case .close30ThenOpen:
				door.direction = 0
				door.topcountdown = 35 * 30
			}
		} else if res == .crushed {
			#partial switch door.type {
			case .blazeClose, .close:
			case:
				door.direction = 1
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_doropn))
			}
		}
	case 1:
		res := T_MovePlane(door.sector, door.speed, door.topheight, false, 1, door.direction)
		if res == .pastdest {
			#partial switch door.type {
			case .blazeRaise, .normal:
				door.direction = 0
				door.topcountdown = door.topwait
			case .close30ThenOpen, .blazeOpen, .open:
				door.sector.specialdata = nil
				P_RemoveThinker(&door.thinker)
			}
		}
	}
}

EV_DoLockedDoor :: proc(line: ^line_t, type: vldoor_e, thing: ^mobj_t) -> int {
	p := thing.player
	if p == nil {
		return 0
	}

	switch line.special {
	case 99, 133:
		if !p.cards[card_t.it_bluecard] && !p.cards[card_t.it_blueskull] {
			p.message = PD_BLUEO
			S_StartSound(nil, int(sfxenum_t.sfx_oof))
			return 0
		}
	case 134, 135:
		if !p.cards[card_t.it_redcard] && !p.cards[card_t.it_redskull] {
			p.message = PD_REDO
			S_StartSound(nil, int(sfxenum_t.sfx_oof))
			return 0
		}
	case 136, 137:
		if !p.cards[card_t.it_yellowcard] && !p.cards[card_t.it_yellowskull] {
			p.message = PD_YELLOWO
			S_StartSound(nil, int(sfxenum_t.sfx_oof))
			return 0
		}
	}

	return EV_DoDoor(line, type)
}

EV_DoDoor :: proc(line: ^line_t, type: vldoor_e) -> int {
	secnum := -1
	rtn := 0

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
		door := (^vldoor_t)(Z_Malloc(size_of(vldoor_t), PU_LEVSPEC, nil))
		P_AddThinker(&door.thinker)
		sec.specialdata = door

		door.thinker.function.acp1 = T_VerticalDoor
		door.sector = sec
		door.type = type
		door.topwait = VDOORWAIT
		door.speed = VDOORSPEED

		#partial switch type {
		case .blazeClose:
			door.topheight = P_FindLowestCeilingSurrounding(sec)
			door.topheight -= 4 * FRACUNIT
			door.direction = -1
			door.speed = VDOORSPEED * 4
			S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_bdcls))
		case .close:
			door.topheight = P_FindLowestCeilingSurrounding(sec)
			door.topheight -= 4 * FRACUNIT
			door.direction = -1
			S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_dorcls))
		case .close30ThenOpen:
			door.topheight = sec.ceilingheight
			door.direction = -1
			S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_dorcls))
		case .blazeRaise, .blazeOpen:
			door.direction = 1
			door.topheight = P_FindLowestCeilingSurrounding(sec)
			door.topheight -= 4 * FRACUNIT
			door.speed = VDOORSPEED * 4
			if door.topheight != sec.ceilingheight {
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_bdopn))
			}
		case .normal, .open:
			door.direction = 1
			door.topheight = P_FindLowestCeilingSurrounding(sec)
			door.topheight -= 4 * FRACUNIT
			if door.topheight != sec.ceilingheight {
				S_StartSound(&door.sector.soundorg, int(sfxenum_t.sfx_doropn))
			}
		}
	}
	return rtn
}

EV_VerticalDoor :: proc(line: ^line_t, thing: ^mobj_t) {
	side := 0
	player := thing.player

	switch line.special {
	case 26, 32:
		if player == nil {
			return
		}
		if !player.cards[card_t.it_bluecard] && !player.cards[card_t.it_blueskull] {
			player.message = PD_BLUEK
			S_StartSound(nil, int(sfxenum_t.sfx_oof))
			return
		}
	case 27, 34:
		if player == nil {
			return
		}
		if !player.cards[card_t.it_yellowcard] && !player.cards[card_t.it_yellowskull] {
			player.message = PD_YELLOWK
			S_StartSound(nil, int(sfxenum_t.sfx_oof))
			return
		}
	case 28, 33:
		if player == nil {
			return
		}
		if !player.cards[card_t.it_redcard] && !player.cards[card_t.it_redskull] {
			player.message = PD_REDK
			S_StartSound(nil, int(sfxenum_t.sfx_oof))
			return
		}
	}

	sec := sides[line.sidenum[side ~ 1]].sector

	if sec.specialdata != nil {
		door := (^vldoor_t)(sec.specialdata)
		switch line.special {
		case 1, 26, 27, 28, 117:
			if door.direction == -1 {
				door.direction = 1
			} else {
				if thing.player == nil {
					return
				}
				door.direction = -1
			}
			return
		}
	}

	switch line.special {
	case 117, 118:
		S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_bdopn))
	case 1, 31:
		S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_doropn))
	case:
		S_StartSound(&sec.soundorg, int(sfxenum_t.sfx_doropn))
	}

	door := (^vldoor_t)(Z_Malloc(size_of(vldoor_t), PU_LEVSPEC, nil))
	P_AddThinker(&door.thinker)
	sec.specialdata = door
	door.thinker.function.acp1 = T_VerticalDoor
	door.sector = sec
	door.direction = 1
	door.speed = VDOORSPEED
	door.topwait = VDOORWAIT

	switch line.special {
	case 1, 26, 27, 28:
		door.type = .normal
	case 31, 32, 33, 34:
		door.type = .open
		line.special = 0
	case 117:
		door.type = .blazeRaise
		door.speed = VDOORSPEED * 4
	case 118:
		door.type = .blazeOpen
		line.special = 0
		door.speed = VDOORSPEED * 4
	}

	door.topheight = P_FindLowestCeilingSurrounding(sec)
	door.topheight -= 4 * FRACUNIT
}

P_SpawnDoorCloseIn30 :: proc(sec: ^sector_t) {
	door := (^vldoor_t)(Z_Malloc(size_of(vldoor_t), PU_LEVSPEC, nil))
	P_AddThinker(&door.thinker)

	sec.specialdata = door
	sec.special = 0

	door.thinker.function.acp1 = T_VerticalDoor
	door.sector = sec
	door.direction = 0
	door.type = .normal
	door.speed = VDOORSPEED
	door.topcountdown = 30 * 35
}

P_SpawnDoorRaiseIn5Mins :: proc(sec: ^sector_t, secnum: int) {
	door := (^vldoor_t)(Z_Malloc(size_of(vldoor_t), PU_LEVSPEC, nil))
	P_AddThinker(&door.thinker)

	sec.specialdata = door
	sec.special = 0

	door.thinker.function.acp1 = T_VerticalDoor
	door.sector = sec
	door.direction = 2
	door.type = .raiseIn5Mins
	door.speed = VDOORSPEED
	door.topheight = P_FindLowestCeilingSurrounding(sec)
	door.topheight -= 4 * FRACUNIT
	door.topwait = VDOORWAIT
	door.topcountdown = 5 * 60 * 35
}
