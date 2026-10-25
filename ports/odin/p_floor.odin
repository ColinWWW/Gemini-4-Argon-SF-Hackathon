// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

T_MovePlane :: proc(sector: ^sector_t, speed: fixed_t, dest: fixed_t, crush: boolean, floorOrCeiling: int, direction: int) -> result_e {
	switch floorOrCeiling {
	case 0:
		switch direction {
		case -1:
			if sector.floorheight - speed < dest {
				lastpos := sector.floorheight
				sector.floorheight = dest
				flag := P_ChangeSector(sector, crush)
				if flag {
					sector.floorheight = lastpos
					P_ChangeSector(sector, crush)
				}
				return .pastdest
			} else {
				lastpos := sector.floorheight
				sector.floorheight -= speed
				flag := P_ChangeSector(sector, crush)
				if flag {
					sector.floorheight = lastpos
					P_ChangeSector(sector, crush)
					return .crushed
				}
			}
		case 1:
			if sector.floorheight + speed > dest {
				lastpos := sector.floorheight
				sector.floorheight = dest
				flag := P_ChangeSector(sector, crush)
				if flag {
					sector.floorheight = lastpos
					P_ChangeSector(sector, crush)
				}
				return .pastdest
			} else {
				lastpos := sector.floorheight
				sector.floorheight += speed
				flag := P_ChangeSector(sector, crush)
				if flag {
					if crush {
						return .crushed
					}
					sector.floorheight = lastpos
					P_ChangeSector(sector, crush)
					return .crushed
				}
			}
		}
	case 1:
		switch direction {
		case -1:
			if sector.ceilingheight - speed < dest {
				lastpos := sector.ceilingheight
				sector.ceilingheight = dest
				flag := P_ChangeSector(sector, crush)
				if flag {
					sector.ceilingheight = lastpos
					P_ChangeSector(sector, crush)
				}
				return .pastdest
			} else {
				lastpos := sector.ceilingheight
				sector.ceilingheight -= speed
				flag := P_ChangeSector(sector, crush)
				if flag {
					if crush {
						return .crushed
					}
					sector.ceilingheight = lastpos
					P_ChangeSector(sector, crush)
					return .crushed
				}
			}
		case 1:
			if sector.ceilingheight + speed > dest {
				lastpos := sector.ceilingheight
				sector.ceilingheight = dest
				flag := P_ChangeSector(sector, crush)
				if flag {
					sector.ceilingheight = lastpos
					P_ChangeSector(sector, crush)
				}
				return .pastdest
			} else {
				sector.ceilingheight += speed
				P_ChangeSector(sector, crush)
			}
		}
	}
	return .ok
}

T_MoveFloor :: proc(floor_raw: rawptr) {
	floor := (^floormove_t)(floor_raw)
	res := T_MovePlane(floor.sector, floor.speed, floor.floordestheight, floor.crush, 0, floor.direction)

	if (leveltime & 7) == 0 {
		S_StartSound(&floor.sector.soundorg, int(sfxenum_t.sfx_stnmov))
	}

	if res == .pastdest {
		floor.sector.specialdata = nil

		if floor.direction == 1 {
			#partial switch floor.type {
			case .donutRaise:
				floor.sector.special = i16(floor.newspecial)
				floor.sector.floorpic = floor.texture
			}
		} else if floor.direction == -1 {
			#partial switch floor.type {
			case .lowerAndChange:
				floor.sector.special = i16(floor.newspecial)
				floor.sector.floorpic = floor.texture
			}
		}
		P_RemoveThinker(&floor.thinker)
		S_StartSound(&floor.sector.soundorg, int(sfxenum_t.sfx_pstop))
	}
}

EV_DoFloor :: proc(line: ^line_t, floortype: floor_e) -> int {
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
		floor := (^floormove_t)(Z_Malloc(size_of(floormove_t), PU_LEVSPEC, nil))
		P_AddThinker(&floor.thinker)
		sec.specialdata = floor
		floor.thinker.function.acp1 = T_MoveFloor
		floor.type = floortype
		floor.crush = false

		#partial switch floortype {
		case .lowerFloor:
			floor.direction = -1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = P_FindHighestFloorSurrounding(sec)
		case .lowerFloorToLowest:
			floor.direction = -1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = P_FindLowestFloorSurrounding(sec)
		case .turboLower:
			floor.direction = -1
			floor.sector = sec
			floor.speed = FLOORSPEED * 4
			floor.floordestheight = P_FindHighestFloorSurrounding(sec)
			if floor.floordestheight != sec.floorheight {
				floor.floordestheight += 8 * FRACUNIT
			}
		case .raiseFloorCrush:
			floor.crush = true
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = P_FindLowestCeilingSurrounding(sec)
			if floor.floordestheight > sec.ceilingheight {
				floor.floordestheight = sec.ceilingheight
			}
			floor.floordestheight -= 8 * FRACUNIT
		case .raiseFloor:
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = P_FindLowestCeilingSurrounding(sec)
			if floor.floordestheight > sec.ceilingheight {
				floor.floordestheight = sec.ceilingheight
			}
		case .raiseFloorTurbo:
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED * 4
			floor.floordestheight = P_FindNextHighestFloor(sec, int(sec.floorheight))
		case .raiseFloorToNearest:
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = P_FindNextHighestFloor(sec, int(sec.floorheight))
		case .raiseFloor24:
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = floor.sector.floorheight + 24 * FRACUNIT
		case .raiseFloor512:
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = floor.sector.floorheight + 512 * FRACUNIT
		case .raiseFloor24AndChange:
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = floor.sector.floorheight + 24 * FRACUNIT
			sec.floorpic = line.frontsector.floorpic
			sec.special = line.frontsector.special
		case .raiseToTexture:
			minsize := fixed_t(MAXINT)
			floor.direction = 1
			floor.sector = sec
			floor.speed = FLOORSPEED
			for i := 0; i < sec.linecount; i += 1 {
				if twoSided(secnum, i) != 0 {
					side := getSide(secnum, i, 0)
					if side.bottomtexture >= 0 {
						if textureheight[side.bottomtexture] < minsize {
							minsize = textureheight[side.bottomtexture]
						}
					}
					side = getSide(secnum, i, 1)
					if side.bottomtexture >= 0 {
						if textureheight[side.bottomtexture] < minsize {
							minsize = textureheight[side.bottomtexture]
						}
					}
				}
			}
			floor.floordestheight = floor.sector.floorheight + minsize
		case .lowerAndChange:
			floor.direction = -1
			floor.sector = sec
			floor.speed = FLOORSPEED
			floor.floordestheight = P_FindLowestFloorSurrounding(sec)
			floor.texture = sec.floorpic

			for i := 0; i < sec.linecount; i += 1 {
				if twoSided(secnum, i) != 0 {
					s0_sec := getSide(secnum, i, 0).sector
					if int(uintptr(s0_sec) - uintptr(&sectors[0])) / size_of(sector_t) == secnum {
						sec = getSector(secnum, i, 1)
						if sec.floorheight == floor.floordestheight {
							floor.texture = sec.floorpic
							floor.newspecial = int(sec.special)
							break
						}
					} else {
						sec = getSector(secnum, i, 0)
						if sec.floorheight == floor.floordestheight {
							floor.texture = sec.floorpic
							floor.newspecial = int(sec.special)
							break
						}
					}
				}
			}
		}
	}
	return rtn
}

EV_BuildStairs :: proc(line: ^line_t, type: stair_e) -> int {
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
		floor := (^floormove_t)(Z_Malloc(size_of(floormove_t), PU_LEVSPEC, nil))
		P_AddThinker(&floor.thinker)
		sec.specialdata = floor
		floor.thinker.function.acp1 = T_MoveFloor
		floor.direction = 1
		floor.sector = sec

		speed: fixed_t
		stairsize: fixed_t
		switch type {
		case .build8:
			speed = FLOORSPEED / 4
			stairsize = 8 * FRACUNIT
		case .turbo16:
			speed = FLOORSPEED * 4
			stairsize = 16 * FRACUNIT
		}
		floor.speed = speed
		height := sec.floorheight + stairsize
		floor.floordestheight = height

		texture := sec.floorpic

		for {
			ok := 0
			for i := 0; i < sec.linecount; i += 1 {
				if (int(sec.lines[i].flags) & ML_TWOSIDED) == 0 {
					continue
				}

				tsec := sec.lines[i].frontsector
				newsecnum := int(uintptr(tsec) - uintptr(&sectors[0])) / size_of(sector_t)

				if secnum != newsecnum {
					continue
				}

				tsec = sec.lines[i].backsector
				newsecnum = int(uintptr(tsec) - uintptr(&sectors[0])) / size_of(sector_t)

				if tsec.floorpic != texture {
					continue
				}

				height += stairsize

				if tsec.specialdata != nil {
					continue
				}

				sec = tsec
				secnum = newsecnum
				floor = (^floormove_t)(Z_Malloc(size_of(floormove_t), PU_LEVSPEC, nil))

				P_AddThinker(&floor.thinker)

				sec.specialdata = floor
				floor.thinker.function.acp1 = T_MoveFloor
				floor.direction = 1
				floor.sector = sec
				floor.speed = speed
				floor.floordestheight = height
				ok = 1
				break
			}
			if ok == 0 {
				break
			}
		}
	}
	return rtn
}
