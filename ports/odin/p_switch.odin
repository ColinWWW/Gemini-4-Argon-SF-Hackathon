// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

alphSwitchList := [41]switchlist_t{
	{"SW1BRCOM", "SW2BRCOM", 1},
	{"SW1BRN1", "SW2BRN1", 1},
	{"SW1BRN2", "SW2BRN2", 1},
	{"SW1BRNGN", "SW2BRNGN", 1},
	{"SW1BROWN", "SW2BROWN", 1},
	{"SW1COMM", "SW2COMM", 1},
	{"SW1COMP", "SW2COMP", 1},
	{"SW1DIRT", "SW2DIRT", 1},
	{"SW1EXIT", "SW2EXIT", 1},
	{"SW1GRAY", "SW2GRAY", 1},
	{"SW1GRAY1", "SW2GRAY1", 1},
	{"SW1METAL", "SW2METAL", 1},
	{"SW1PIPE", "SW2PIPE", 1},
	{"SW1SLAD", "SW2SLAD", 1},
	{"SW1STARG", "SW2STARG", 1},
	{"SW1STON1", "SW2STON1", 1},
	{"SW1STON2", "SW2STON2", 1},
	{"SW1STONE", "SW2STONE", 1},
	{"SW1STRTN", "SW2STRTN", 1},
	{"SW1BLUE", "SW2BLUE", 2},
	{"SW1CMT", "SW2CMT", 2},
	{"SW1GARG", "SW2GARG", 2},
	{"SW1GSTON", "SW2GSTON", 2},
	{"SW1HOT", "SW2HOT", 2},
	{"SW1LION", "SW2LION", 2},
	{"SW1SATYR", "SW2SATYR", 2},
	{"SW1SKIN", "SW2SKIN", 2},
	{"SW1VINE", "SW2VINE", 2},
	{"SW1WOOD", "SW2WOOD", 2},
	{"SW1PANEL", "SW2PANEL", 3},
	{"SW1ROCK", "SW2ROCK", 3},
	{"SW1MET2", "SW2MET2", 3},
	{"SW1WDMET", "SW2WDMET", 3},
	{"SW1BRIK", "SW2BRIK", 3},
	{"SW1MOD1", "SW2MOD1", 3},
	{"SW1ZIM", "SW2ZIM", 3},
	{"SW1STON6", "SW2STON6", 3},
	{"SW1TEK", "SW2TEK", 3},
	{"SW1MARB", "SW2MARB", 3},
	{"SW1SKULL", "SW2SKULL", 3},
	{"", "", 0},
}

switchlist: [MAXSWITCHES * 2]int
numswitches: int
buttonlist: [MAXBUTTONS]button_t

P_InitSwitchList :: proc() {
	episode := 1
	if gamemode == .registered {
		episode = 2
	} else if gamemode == .commercial {
		episode = 3
	}

	index := 0
	for i := 0; i < MAXSWITCHES; i += 1 {
		if alphSwitchList[i].episode == 0 {
			numswitches = index / 2
			switchlist[index] = -1
			break
		}

		if int(alphSwitchList[i].episode) <= episode {
			switchlist[index] = R_TextureNumForName(alphSwitchList[i].name1)
			index += 1
			switchlist[index] = R_TextureNumForName(alphSwitchList[i].name2)
			index += 1
		}
	}
}

P_StartButton :: proc(line: ^line_t, w: bwhere_e, texture: int, time: int) {
	for i := 0; i < MAXBUTTONS; i += 1 {
		if buttonlist[i].btimer != 0 && buttonlist[i].line == line {
			return
		}
	}

	for i := 0; i < MAXBUTTONS; i += 1 {
		if buttonlist[i].btimer == 0 {
			buttonlist[i].line = line
			buttonlist[i].where_ = w
			buttonlist[i].btexture = texture
			buttonlist[i].btimer = time
			buttonlist[i].soundorg = (^mobj_t)(&line.frontsector.soundorg)
			return
		}
	}

	I_Error("P_StartButton: no button slots left!")
}

P_ChangeSwitchTexture :: proc(line: ^line_t, useAgain: int) {
	if useAgain == 0 {
		line.special = 0
	}

	texTop := int(sides[line.sidenum[0]].toptexture)
	texMid := int(sides[line.sidenum[0]].midtexture)
	texBot := int(sides[line.sidenum[0]].bottomtexture)

	sound := int(sfxenum_t.sfx_swtchn)

	if line.special == 11 {
		sound = int(sfxenum_t.sfx_swtchx)
	}

	for i := 0; i < numswitches * 2; i += 1 {
		if switchlist[i] == texTop {
			S_StartSound(buttonlist[0].soundorg, sound)
			sides[line.sidenum[0]].toptexture = i16(switchlist[i ~ 1])
			if useAgain != 0 {
				P_StartButton(line, .top, switchlist[i], BUTTONTIME)
			}
			return
		} else if switchlist[i] == texMid {
			S_StartSound(buttonlist[0].soundorg, sound)
			sides[line.sidenum[0]].midtexture = i16(switchlist[i ~ 1])
			if useAgain != 0 {
				P_StartButton(line, .middle, switchlist[i], BUTTONTIME)
			}
			return
		} else if switchlist[i] == texBot {
			S_StartSound(buttonlist[0].soundorg, sound)
			sides[line.sidenum[0]].bottomtexture = i16(switchlist[i ~ 1])
			if useAgain != 0 {
				P_StartButton(line, .bottom, switchlist[i], BUTTONTIME)
			}
			return
		}
	}
}

P_UseSpecialLine :: proc(thing: ^mobj_t, line: ^line_t, side: int) -> boolean {
	if side != 0 {
		switch line.special {
		case 124:
		case:
			return false
		}
	}

	if thing.player == nil {
		if (int(line.flags) & ML_SECRET) != 0 {
			return false
		}
		switch line.special {
		case 1, 32, 33, 34:
		case:
			return false
		}
	}

	switch line.special {
	case 1, 26, 27, 28, 31, 32, 33, 34, 117, 118:
		EV_VerticalDoor(line, thing)
	case 7:
		if EV_BuildStairs(line, .build8) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 9:
		if EV_DoDonut(line) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 11:
		P_ChangeSwitchTexture(line, 0)
		G_ExitLevel()
	case 14:
		if EV_DoPlat(line, .raiseAndChange, 32) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 15:
		if EV_DoPlat(line, .raiseAndChange, 24) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 18:
		if EV_DoFloor(line, .raiseFloorToNearest) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 20:
		if EV_DoPlat(line, .raiseToNearestAndChange, 0) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 21:
		if EV_DoPlat(line, .downWaitUpStay, 0) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 23:
		if EV_DoFloor(line, .lowerFloorToLowest) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 29:
		if EV_DoDoor(line, .normal) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 41:
		if EV_DoCeiling(line, .lowerToFloor) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 71:
		if EV_DoFloor(line, .turboLower) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 49:
		if EV_DoCeiling(line, .crushAndRaise) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 50:
		if EV_DoDoor(line, .close) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 51:
		P_ChangeSwitchTexture(line, 0)
		G_SecretExitLevel()
	case 55:
		if EV_DoFloor(line, .raiseFloorCrush) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 101:
		if EV_DoFloor(line, .raiseFloor) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 102:
		if EV_DoFloor(line, .lowerFloor) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 103:
		if EV_DoDoor(line, .open) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 111:
		if EV_DoDoor(line, .blazeRaise) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 112:
		if EV_DoDoor(line, .blazeOpen) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 113:
		if EV_DoDoor(line, .blazeClose) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 122:
		if EV_DoPlat(line, .blazeDWUS, 0) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 127:
		if EV_BuildStairs(line, .turbo16) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 131:
		if EV_DoFloor(line, .raiseFloorTurbo) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 133, 135, 137:
		if EV_DoLockedDoor(line, .blazeOpen, thing) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 140:
		if EV_DoFloor(line, .raiseFloor512) != 0 {
			P_ChangeSwitchTexture(line, 0)
		}
	case 42:
		if EV_DoDoor(line, .close) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 43:
		if EV_DoCeiling(line, .lowerToFloor) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 45:
		if EV_DoFloor(line, .lowerFloor) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 60:
		if EV_DoFloor(line, .lowerFloorToLowest) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 61:
		if EV_DoDoor(line, .open) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 62:
		if EV_DoPlat(line, .downWaitUpStay, 1) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 63:
		if EV_DoDoor(line, .normal) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 64:
		if EV_DoFloor(line, .raiseFloor) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 66:
		if EV_DoPlat(line, .raiseAndChange, 24) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 67:
		if EV_DoPlat(line, .raiseAndChange, 32) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 65:
		if EV_DoFloor(line, .raiseFloorCrush) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 68:
		if EV_DoPlat(line, .raiseToNearestAndChange, 0) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 69:
		if EV_DoFloor(line, .raiseFloorToNearest) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 70:
		if EV_DoFloor(line, .turboLower) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 114:
		if EV_DoDoor(line, .blazeRaise) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 115:
		if EV_DoDoor(line, .blazeOpen) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 116:
		if EV_DoDoor(line, .blazeClose) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 123:
		if EV_DoPlat(line, .blazeDWUS, 0) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 132:
		if EV_DoFloor(line, .raiseFloorTurbo) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 99, 134, 136:
		if EV_DoLockedDoor(line, .blazeOpen, thing) != 0 {
			P_ChangeSwitchTexture(line, 1)
		}
	case 138:
		EV_LightTurnOn(line, 255)
		P_ChangeSwitchTexture(line, 1)
	case 139:
		EV_LightTurnOn(line, 35)
		P_ChangeSwitchTexture(line, 1)
	}

	return true
}