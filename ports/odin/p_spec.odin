// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

p_anim_t :: struct {
	istexture: int,
	picnum:    int,
	basepic:   int,
	numpics:   int,
	speed:     int,
}

animdef_t :: struct {
	istexture: int,
	endname:   cstring,
	startname: cstring,
	speed:     int,
}

MAXANIMS :: 32

animdefs := [23]animdef_t{
	{0, "NUKAGE3", "NUKAGE1", 8},
	{0, "FWATER4", "FWATER1", 8},
	{0, "SWATER4", "SWATER1", 8},
	{0, "LAVA4", "LAVA1", 8},
	{0, "BLOOD3", "BLOOD1", 8},
	{0, "RROCK08", "RROCK05", 8},
	{0, "SLIME04", "SLIME01", 8},
	{0, "SLIME08", "SLIME05", 8},
	{0, "SLIME12", "SLIME09", 8},
	{1, "BLODGR4", "BLODGR1", 8},
	{1, "SLADRIP3", "SLADRIP1", 8},
	{1, "BLODRIP4", "BLODRIP1", 8},
	{1, "FIREWALL", "FIREWALA", 8},
	{1, "GSTFONT3", "GSTFONT1", 8},
	{1, "FIRELAVA", "FIRELAV3", 8},
	{1, "FIREMAG3", "FIREMAG1", 8},
	{1, "FIREBLU2", "FIREBLU1", 8},
	{1, "ROCKRED3", "ROCKRED1", 8},
	{1, "BFALL4", "BFALL1", 8},
	{1, "SFALL4", "SFALL1", 8},
	{1, "WFALL4", "WFALL1", 8},
	{1, "DBRAIN4", "DBRAIN1", 8},
	{-1, nil, nil, 0},
}

p_anims: [MAXANIMS]p_anim_t
lastanim: [^]p_anim_t

MAXLINEANIMS :: 64
numlinespecials: i16
linespeciallist: [MAXLINEANIMS]^line_t

levelTimer: boolean
levelTimeCount: int

P_InitPicAnims :: proc() {
	lastanim = &p_anims[0]
	for i := 0; animdefs[i].istexture != -1; i += 1 {
		if animdefs[i].istexture != 0 {
			if R_CheckTextureNumForName(animdefs[i].startname) == -1 {
				continue
			}
			lastanim[0].picnum = R_TextureNumForName(animdefs[i].endname)
			lastanim[0].basepic = R_TextureNumForName(animdefs[i].startname)
		} else {
			if W_CheckNumForName(animdefs[i].startname) == -1 {
				continue
			}
			lastanim[0].picnum = R_FlatNumForName(animdefs[i].endname)
			lastanim[0].basepic = R_FlatNumForName(animdefs[i].startname)
		}

		lastanim[0].istexture = animdefs[i].istexture
		lastanim[0].numpics = lastanim[0].picnum - lastanim[0].basepic + 1

		if lastanim[0].numpics < 2 {
			I_Error("P_InitPicAnims: bad cycle from %s to %s", animdefs[i].startname, animdefs[i].endname)
		}

		lastanim[0].speed = animdefs[i].speed
		lastanim = &lastanim[1]
	}
}

getSide :: proc(currentSector: int, line: int, side: int) -> ^side_t {
	return &sides[sectors[currentSector].lines[line].sidenum[side]]
}

getSector :: proc(currentSector: int, line: int, side: int) -> ^sector_t {
	return sides[sectors[currentSector].lines[line].sidenum[side]].sector
}

twoSided :: proc(sector: int, line: int) -> int {
	return int(sectors[sector].lines[line].flags) & ML_TWOSIDED
}

getNextSector :: proc(line: ^line_t, sec: ^sector_t) -> ^sector_t {
	if (int(line.flags) & ML_TWOSIDED) == 0 {
		return nil
	}
	if line.frontsector == sec {
		return line.backsector
	}
	return line.frontsector
}

P_FindLowestFloorSurrounding :: proc(sec: ^sector_t) -> fixed_t {
	floor := sec.floorheight
	for i := 0; i < sec.linecount; i += 1 {
		check := sec.lines[i]
		other := getNextSector(check, sec)
		if other == nil {
			continue
		}
		if other.floorheight < floor {
			floor = other.floorheight
		}
	}
	return floor
}

P_FindHighestFloorSurrounding :: proc(sec: ^sector_t) -> fixed_t {
	floor := fixed_t(-500 * FRACUNIT)
	for i := 0; i < sec.linecount; i += 1 {
		check := sec.lines[i]
		other := getNextSector(check, sec)
		if other == nil {
			continue
		}
		if other.floorheight > floor {
			floor = other.floorheight
		}
	}
	return floor
}

MAX_ADJOINING_SECTORS :: 20

P_FindNextHighestFloor :: proc(sec: ^sector_t, currentheight: int) -> fixed_t {
	height := fixed_t(currentheight)
	heightlist: [MAX_ADJOINING_SECTORS]fixed_t

	h := 0
	for i := 0; i < sec.linecount; i += 1 {
		check := sec.lines[i]
		other := getNextSector(check, sec)
		if other == nil {
			continue
		}
		if other.floorheight > height {
			heightlist[h] = other.floorheight
			h += 1
		}
		if h >= MAX_ADJOINING_SECTORS {
			libc.fprintf(libc.stderr, "Sector with more than 20 adjoining sectors\n")
			break
		}
	}

	if h == 0 {
		return fixed_t(currentheight)
	}

	min_h := heightlist[0]
	for i := 1; i < h; i += 1 {
		if heightlist[i] < min_h {
			min_h = heightlist[i]
		}
	}
	return min_h
}

P_FindLowestCeilingSurrounding :: proc(sec: ^sector_t) -> fixed_t {
	height := fixed_t(MAXINT)
	for i := 0; i < sec.linecount; i += 1 {
		check := sec.lines[i]
		other := getNextSector(check, sec)
		if other == nil {
			continue
		}
		if other.ceilingheight < height {
			height = other.ceilingheight
		}
	}
	return height
}

P_FindHighestCeilingSurrounding :: proc(sec: ^sector_t) -> fixed_t {
	height := fixed_t(0)
	for i := 0; i < sec.linecount; i += 1 {
		check := sec.lines[i]
		other := getNextSector(check, sec)
		if other == nil {
			continue
		}
		if other.ceilingheight > height {
			height = other.ceilingheight
		}
	}
	return height
}

P_FindSectorFromLineTag :: proc(line: ^line_t, start: int) -> int {
	for i := start + 1; i < numsectors; i += 1 {
		if sectors[i].tag == line.tag {
			return i
		}
	}
	return -1
}

P_FindMinSurroundingLight :: proc(sector: ^sector_t, max_val: int) -> int {
	min_val := max_val
	for i := 0; i < sector.linecount; i += 1 {
		line := sector.lines[i]
		check := getNextSector(line, sector)
		if check == nil {
			continue
		}
		if int(check.lightlevel) < min_val {
			min_val = int(check.lightlevel)
		}
	}
	return min_val
}

P_CrossSpecialLine :: proc(linenum: int, side: int, thing: ^mobj_t) {
	line := &lines[linenum]

	if thing.player == nil {
		#partial switch thing.type {
		case .MT_ROCKET, .MT_PLASMA, .MT_BFG, .MT_TROOPSHOT, .MT_HEADSHOT, .MT_BRUISERSHOT:
			return
		}

		ok := 0
		switch line.special {
		case 39, 97, 125, 126, 4, 10, 88:
			ok = 1
		}
		if ok == 0 {
			return
		}
	}

	switch line.special {
	case 2:
		EV_DoDoor(line, .open)
		line.special = 0
	case 3:
		EV_DoDoor(line, .close)
		line.special = 0
	case 4:
		EV_DoDoor(line, .normal)
		line.special = 0
	case 5:
		EV_DoFloor(line, .raiseFloor)
		line.special = 0
	case 6:
		EV_DoCeiling(line, .fastCrushAndRaise)
		line.special = 0
	case 8:
		EV_BuildStairs(line, .build8)
		line.special = 0
	case 10:
		EV_DoPlat(line, .downWaitUpStay, 0)
		line.special = 0
	case 12:
		EV_LightTurnOn(line, 0)
		line.special = 0
	case 13:
		EV_LightTurnOn(line, 255)
		line.special = 0
	case 16:
		EV_DoDoor(line, .close30ThenOpen)
		line.special = 0
	case 17:
		EV_StartLightStrobing(line)
		line.special = 0
	case 19:
		EV_DoFloor(line, .lowerFloor)
		line.special = 0
	case 22:
		EV_DoPlat(line, .raiseToNearestAndChange, 0)
		line.special = 0
	case 25:
		EV_DoCeiling(line, .crushAndRaise)
		line.special = 0
	case 30:
		EV_DoFloor(line, .raiseToTexture)
		line.special = 0
	case 35:
		EV_LightTurnOn(line, 35)
		line.special = 0
	case 36:
		EV_DoFloor(line, .turboLower)
		line.special = 0
	case 37:
		EV_DoFloor(line, .lowerAndChange)
		line.special = 0
	case 38:
		EV_DoFloor(line, .lowerFloorToLowest)
		line.special = 0
	case 39:
		EV_Teleport(line, side, thing)
		line.special = 0
	case 40:
		EV_DoCeiling(line, .raiseToHighest)
		EV_DoFloor(line, .lowerFloorToLowest)
		line.special = 0
	case 44:
		EV_DoCeiling(line, .lowerAndCrush)
		line.special = 0
	case 52:
		G_ExitLevel()
	case 53:
		EV_DoPlat(line, .perpetualRaise, 0)
		line.special = 0
	case 54:
		EV_StopPlat(line)
		line.special = 0
	case 56:
		EV_DoFloor(line, .raiseFloorCrush)
		line.special = 0
	case 57:
		EV_CeilingCrushStop(line)
		line.special = 0
	case 58:
		EV_DoFloor(line, .raiseFloor24)
		line.special = 0
	case 59:
		EV_DoFloor(line, .raiseFloor24AndChange)
		line.special = 0
	case 104:
		EV_TurnTagLightsOff(line)
		line.special = 0
	case 108:
		EV_DoDoor(line, .blazeRaise)
		line.special = 0
	case 109:
		EV_DoDoor(line, .blazeOpen)
		line.special = 0
	case 100:
		EV_BuildStairs(line, .turbo16)
		line.special = 0
	case 110:
		EV_DoDoor(line, .blazeClose)
		line.special = 0
	case 119:
		EV_DoFloor(line, .raiseFloorToNearest)
		line.special = 0
	case 121:
		EV_DoPlat(line, .blazeDWUS, 0)
		line.special = 0
	case 124:
		G_SecretExitLevel()
	case 125:
		if thing.player == nil {
			EV_Teleport(line, side, thing)
			line.special = 0
		}
	case 130:
		EV_DoFloor(line, .raiseFloorTurbo)
		line.special = 0
	case 141:
		EV_DoCeiling(line, .silentCrushAndRaise)
		line.special = 0
	case 72:
		EV_DoCeiling(line, .lowerAndCrush)
	case 73:
		EV_DoCeiling(line, .crushAndRaise)
	case 74:
		EV_CeilingCrushStop(line)
	case 75:
		EV_DoDoor(line, .close)
	case 76:
		EV_DoDoor(line, .close30ThenOpen)
	case 77:
		EV_DoCeiling(line, .fastCrushAndRaise)
	case 79:
		EV_LightTurnOn(line, 35)
	case 80:
		EV_LightTurnOn(line, 0)
	case 81:
		EV_LightTurnOn(line, 255)
	case 82:
		EV_DoFloor(line, .lowerFloorToLowest)
	case 83:
		EV_DoFloor(line, .lowerFloor)
	case 84:
		EV_DoFloor(line, .lowerAndChange)
	case 86:
		EV_DoDoor(line, .open)
	case 87:
		EV_DoPlat(line, .perpetualRaise, 0)
	case 88:
		EV_DoPlat(line, .downWaitUpStay, 0)
	case 89:
		EV_StopPlat(line)
	case 90:
		EV_DoDoor(line, .normal)
	case 91:
		EV_DoFloor(line, .raiseFloor)
	case 92:
		EV_DoFloor(line, .raiseFloor24)
	case 93:
		EV_DoFloor(line, .raiseFloor24AndChange)
	case 94:
		EV_DoFloor(line, .raiseFloorCrush)
	case 95:
		EV_DoPlat(line, .raiseToNearestAndChange, 0)
	case 96:
		EV_DoFloor(line, .raiseToTexture)
	case 97:
		EV_Teleport(line, side, thing)
	case 98:
		EV_DoFloor(line, .turboLower)
	case 105:
		EV_DoDoor(line, .blazeRaise)
	case 106:
		EV_DoDoor(line, .blazeOpen)
	case 107:
		EV_DoDoor(line, .blazeClose)
	case 120:
		EV_DoPlat(line, .blazeDWUS, 0)
	case 126:
		if thing.player == nil {
			EV_Teleport(line, side, thing)
		}
	case 128:
		EV_DoFloor(line, .raiseFloorToNearest)
	case 129:
		EV_DoFloor(line, .raiseFloorTurbo)
	}
}

P_ShootSpecialLine :: proc(thing: ^mobj_t, line: ^line_t) {
	if thing.player == nil {
		ok := 0
		switch line.special {
		case 46:
			ok = 1
		}
		if ok == 0 {
			return
		}
	}

	switch line.special {
	case 24:
		EV_DoFloor(line, .raiseFloor)
		P_ChangeSwitchTexture(line, 0)
	case 46:
		EV_DoDoor(line, .open)
		P_ChangeSwitchTexture(line, 1)
	case 47:
		EV_DoPlat(line, .raiseToNearestAndChange, 0)
		P_ChangeSwitchTexture(line, 0)
	}
}

P_PlayerInSpecialSector :: proc(player: ^player_t) {
	sector := player.mo.subsector.sector

	if player.mo.z != sector.floorheight {
		return
	}

	switch sector.special {
	case 5:
		if player.powers[pw_ironfeet] == 0 {
			if (leveltime & 0x1f) == 0 {
				P_DamageMobj(player.mo, nil, nil, 10)
			}
		}
	case 7:
		if player.powers[pw_ironfeet] == 0 {
			if (leveltime & 0x1f) == 0 {
				P_DamageMobj(player.mo, nil, nil, 5)
			}
		}
	case 16, 4:
		if player.powers[pw_ironfeet] == 0 || P_Random() < 5 {
			if (leveltime & 0x1f) == 0 {
				P_DamageMobj(player.mo, nil, nil, 20)
			}
		}
	case 9:
		player.secretcount += 1
		sector.special = 0
	case 11:
		player.cheats &= ~int(CF_GODMODE)
		if (leveltime & 0x1f) == 0 {
			P_DamageMobj(player.mo, nil, nil, 20)
		}
		if player.health <= 10 {
			G_ExitLevel()
		}
	case:
		I_Error("P_PlayerInSpecialSector: unknown special %i", i32(sector.special))
	}
}

P_UpdateSpecials :: proc() {
	if levelTimer {
		levelTimeCount -= 1
		if levelTimeCount == 0 {
			G_ExitLevel()
		}
	}

	for anim := ([^]p_anim_t)(&p_anims[0]); anim != lastanim; anim = &anim[1] {
		for i := anim[0].basepic; i < anim[0].basepic + anim[0].numpics; i += 1 {
			pic := anim[0].basepic + ((leveltime / anim[0].speed + i) % anim[0].numpics)
			if anim[0].istexture != 0 {
				texturetranslation[i] = pic
			} else {
				flattranslation[i] = pic
			}
		}
	}

	for i := 0; i < int(numlinespecials); i += 1 {
		line := linespeciallist[i]
		switch line.special {
		case 48:
			sides[line.sidenum[0]].textureoffset += FRACUNIT
		}
	}

	for i := 0; i < MAXBUTTONS; i += 1 {
		if buttonlist[i].btimer != 0 {
			buttonlist[i].btimer -= 1
			if buttonlist[i].btimer == 0 {
				switch buttonlist[i].where {
				case .top:
					sides[buttonlist[i].line.sidenum[0]].toptexture = i16(buttonlist[i].btexture)
				case .middle:
					sides[buttonlist[i].line.sidenum[0]].midtexture = i16(buttonlist[i].btexture)
				case .bottom:
					sides[buttonlist[i].line.sidenum[0]].bottomtexture = i16(buttonlist[i].btexture)
				}
				S_StartSound(buttonlist[i].soundorg, int(sfxenum_t.sfx_swtchn))
				libc.memset(&buttonlist[i], 0, size_of(button_t))
			}
		}
	}
}

EV_DoDonut :: proc(line: ^line_t) -> int {
	secnum := -1
	rtn := 0

	for {
		secnum = P_FindSectorFromLineTag(line, secnum)
		if secnum < 0 {
			break
		}
		s1 := &sectors[secnum]

		if s1.specialdata != nil {
			continue
		}

		rtn = 1
		s2 := getNextSector(s1.lines[0], s1)
		for i := 0; i < s2.linecount; i += 1 {
			// In original C: if ((!s2->lines[i]->flags & ML_TWOSIDED) || (s2->lines[i]->backsector == s1))
			// Note C operator precedence: (!s2->lines[i]->flags) & ML_TWOSIDED
			if ((s2.lines[i].flags == 0 ? 1 : 0) & ML_TWOSIDED) != 0 || s2.lines[i].backsector == s1 {
				continue
			}
			s3 := s2.lines[i].backsector

			floor := (^floormove_t)(Z_Malloc(size_of(floormove_t), PU_LEVSPEC, nil))
			P_AddThinker(&floor.thinker)
			s2.specialdata = floor
			floor.thinker.function.acp1 = T_MoveFloor
			floor.type = .donutRaise
			floor.crush = false
			floor.direction = 1
			floor.sector = s2
			floor.speed = FLOORSPEED / 2
			floor.texture = s3.floorpic
			floor.newspecial = 0
			floor.floordestheight = s3.floorheight

			floor = (^floormove_t)(Z_Malloc(size_of(floormove_t), PU_LEVSPEC, nil))
			P_AddThinker(&floor.thinker)
			s1.specialdata = floor
			floor.thinker.function.acp1 = T_MoveFloor
			floor.type = .lowerFloor
			floor.crush = false
			floor.direction = -1
			floor.sector = s1
			floor.speed = FLOORSPEED / 2
			floor.floordestheight = s3.floorheight
			break
		}
	}
	return rtn
}

P_SpawnSpecials :: proc() {
	levelTimer = false

	i := M_CheckParm("-avg")
	if i != 0 && deathmatch != 0 {
		levelTimer = true
		levelTimeCount = 20 * 60 * 35
	}

	i = M_CheckParm("-timer")
	if i != 0 && deathmatch != 0 {
		time := int(libc.atoi(myargv[i + 1])) * 60 * 35
		levelTimer = true
		levelTimeCount = time
	}

	for j := 0; j < numsectors; j += 1 {
		sector := &sectors[j]
		if sector.special == 0 {
			continue
		}

		switch sector.special {
		case 1:
			P_SpawnLightFlash(sector)
		case 2:
			P_SpawnStrobeFlash(sector, FASTDARK, 0)
		case 3:
			P_SpawnStrobeFlash(sector, SLOWDARK, 0)
		case 4:
			P_SpawnStrobeFlash(sector, FASTDARK, 0)
			sector.special = 4
		case 8:
			P_SpawnGlowingLight(sector)
		case 9:
			totalsecret += 1
		case 10:
			P_SpawnDoorCloseIn30(sector)
		case 12:
			P_SpawnStrobeFlash(sector, SLOWDARK, 1)
		case 13:
			P_SpawnStrobeFlash(sector, FASTDARK, 1)
		case 14:
			P_SpawnDoorRaiseIn5Mins(sector, j)
		case 17:
			P_SpawnFireFlicker(sector)
		}
	}

	numlinespecials = 0
	for j := 0; j < numlines; j += 1 {
		switch lines[j].special {
		case 48:
			linespeciallist[numlinespecials] = &lines[j]
			numlinespecials += 1
		}
	}

	for j := 0; j < MAXCEILINGS; j += 1 {
		activeceilings[j] = nil
	}

	for j := 0; j < MAXPLATS; j += 1 {
		activeplats[j] = nil
	}

	for j := 0; j < MAXBUTTONS; j += 1 {
		libc.memset(&buttonlist[j], 0, size_of(button_t))
	}
}
