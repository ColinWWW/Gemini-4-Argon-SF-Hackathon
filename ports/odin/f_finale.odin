// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

finalestage: int
finalecount: int

TEXTSPEED :: 3
TEXTWAIT  :: 250

e1text: cstring = E1TEXT
e2text: cstring = E2TEXT
e3text: cstring = E3TEXT
e4text: cstring = E4TEXT

c1text: cstring = C1TEXT
c2text: cstring = C2TEXT
c3text: cstring = C3TEXT
c4text: cstring = C4TEXT
c5text: cstring = C5TEXT
c6text: cstring = C6TEXT

p1text: cstring = P1TEXT
p2text: cstring = P2TEXT
p3text: cstring = P3TEXT
p4text: cstring = P4TEXT
p5text: cstring = P5TEXT
p6text: cstring = P6TEXT

t1text: cstring = T1TEXT
t2text: cstring = T2TEXT
t3text: cstring = T3TEXT
t4text: cstring = T4TEXT
t5text: cstring = T5TEXT
t6text: cstring = T6TEXT

finaletext: cstring
finaleflat: cstring

castinfo_t :: struct {
	name: cstring,
	type: mobjtype_t,
}

castorder := [18]castinfo_t{
	{CC_ZOMBIE, .MT_POSSESSED},
	{CC_SHOTGUN, .MT_SHOTGUY},
	{CC_HEAVY, .MT_CHAINGUY},
	{CC_IMP, .MT_TROOP},
	{CC_DEMON, .MT_SERGEANT},
	{CC_LOST, .MT_SKULL},
	{CC_CACO, .MT_HEAD},
	{CC_HELL, .MT_KNIGHT},
	{CC_BARON, .MT_BRUISER},
	{CC_ARACH, .MT_BABY},
	{CC_PAIN, .MT_PAIN},
	{CC_REVEN, .MT_UNDEAD},
	{CC_MANCU, .MT_FATSO},
	{CC_ARCH, .MT_VILE},
	{CC_SPIDER, .MT_SPIDER},
	{CC_CYBER, .MT_CYBORG},
	{CC_HERO, .MT_PLAYER},
	{nil, .MT_PLAYER},
}

castnum: int
casttics: int
caststate: ^state_t
castdeath: boolean
castframes: int
castonmelee: int
castattacking: boolean
f_laststage: int

F_StartFinale :: proc() {
	gameaction = .ga_nothing
	gamestate = .GS_FINALE
	viewactive = false
	automapactive = false

	switch gamemode {
	case .shareware, .registered, .retail:
		S_ChangeMusic(int(musicenum_t.mus_victor), 1)
		switch gameepisode {
		case 1:
			finaleflat = "FLOOR4_8"
			finaletext = e1text
		case 2:
			finaleflat = "SFLR6_1"
			finaletext = e2text
		case 3:
			finaleflat = "MFLR8_4"
			finaletext = e3text
		case 4:
			finaleflat = "MFLR8_3"
			finaletext = e4text
		}
	case .commercial:
		S_ChangeMusic(int(musicenum_t.mus_read_m), 1)
		switch gamemap {
		case 6:
			finaleflat = "SLIME16"
			finaletext = c1text
		case 11:
			finaleflat = "RROCK14"
			finaletext = c2text
		case 20:
			finaleflat = "RROCK07"
			finaletext = c3text
		case 30:
			finaleflat = "RROCK17"
			finaletext = c4text
		case 15:
			finaleflat = "RROCK13"
			finaletext = c5text
		case 31:
			finaleflat = "RROCK19"
			finaletext = c6text
		}
	case .indetermined:
		S_ChangeMusic(int(musicenum_t.mus_read_m), 1)
		finaleflat = "F_SKY1"
		finaletext = c1text
	}

	finalestage = 0
	finalecount = 0
}

F_Responder :: proc(event: ^event_t) -> boolean {
	if finalestage == 2 {
		return F_CastResponder(event)
	}
	return false
}

F_Ticker :: proc() {
	if gamemode == .commercial && finalecount > 50 {
		i := 0
		for ; i < MAXPLAYERS; i += 1 {
			if players[i].cmd.buttons != 0 {
				break
			}
		}
		if i < MAXPLAYERS {
			if gamemap == 30 {
				F_StartCast()
			} else {
				gameaction = .ga_worlddone
			}
		}
	}

	finalecount += 1

	if finalestage == 2 {
		F_CastTicker()
		return
	}

	if gamemode == .commercial {
		return
	}

	if finalestage == 0 && finalecount > int(libc.strlen(finaletext)) * TEXTSPEED + TEXTWAIT {
		finalecount = 0
		finalestage = 1
		wipegamestate = gamestate_t(-1)
		if gameepisode == 3 {
			S_StartMusic(int(musicenum_t.mus_bunny))
		}
	}
}

F_TextWrite :: proc() {
	src := ([^]byte)(W_CacheLumpName(finaleflat, PU_CACHE))
	dest := screens[0]

	for y := 0; y < SCREENHEIGHT; y += 1 {
		for x := 0; x < SCREENWIDTH / 64; x += 1 {
			libc.memcpy(dest, &src[(y & 63) << 6], 64)
			dest = &dest[64]
		}
		if (SCREENWIDTH & 63) != 0 {
			libc.memcpy(dest, &src[(y & 63) << 6], SCREENWIDTH & 63)
			dest = &dest[SCREENWIDTH & 63]
		}
	}

	V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT)

	cx := 10
	cy := 10
	ch := ([^]u8)(finaletext)

	count := (finalecount - 10) / TEXTSPEED
	if count < 0 {
		count = 0
	}
	for ; count != 0; count -= 1 {
		c := int(ch[0])
		ch = &ch[1]
		if c == 0 {
			break
		}
		if c == '\n' {
			cx = 10
			cy += 11
			continue
		}

		c = int(libc.toupper(i32(c))) - int(HU_FONTSTART)
		if c < 0 || c > HU_FONTSIZE {
			cx += 4
			continue
		}

		w := int(SHORT(hu_font[c].width))
		if cx + w > SCREENWIDTH {
			break
		}
		V_DrawPatch(cx, cy, 0, hu_font[c])
		cx += w
	}
}

F_StartCast :: proc() {
	wipegamestate = gamestate_t(-1)
	castnum = 0
	caststate = &states[mobjinfo[castorder[castnum].type].seestate]
	casttics = int(caststate.tics)
	castdeath = false
	finalestage = 2
	castframes = 0
	castonmelee = 0
	castattacking = false
	S_ChangeMusic(int(musicenum_t.mus_evil), 1)
}

F_CastTicker :: proc() {
	casttics -= 1
	if casttics > 0 {
		return
	}

	if caststate.tics == -1 || caststate.nextstate == .S_NULL {
		castnum += 1
		castdeath = false
		if castorder[castnum].name == nil {
			castnum = 0
		}
		if mobjinfo[castorder[castnum].type].seesound != 0 {
			S_StartSound(nil, int(mobjinfo[castorder[castnum].type].seesound))
		}
		caststate = &states[mobjinfo[castorder[castnum].type].seestate]
		castframes = 0
	} else {
		if caststate == &states[statenum_t.S_PLAY_ATK1] {
			castattacking = false
			castframes = 0
			caststate = &states[mobjinfo[castorder[castnum].type].seestate]
		} else {
			st := caststate.nextstate
			caststate = &states[st]
			castframes += 1

			sfx: sfxenum_t
			#partial switch st {
			case .S_PLAY_ATK1: sfx = .sfx_dshtgn
			case .S_POSS_ATK2: sfx = .sfx_pistol
			case .S_SPOS_ATK2: sfx = .sfx_shotgn
			case .S_VILE_ATK2: sfx = .sfx_vilatk
			case .S_SKEL_FIST2: sfx = .sfx_skeswg
			case .S_SKEL_FIST4: sfx = .sfx_skepch
			case .S_SKEL_MISS2: sfx = .sfx_skeatk
			case .S_FATT_ATK8, .S_FATT_ATK5, .S_FATT_ATK2: sfx = .sfx_firsht
			case .S_CPOS_ATK2, .S_CPOS_ATK3, .S_CPOS_ATK4: sfx = .sfx_shotgn
			case .S_TROO_ATK3: sfx = .sfx_claw
			case .S_SARG_ATK2: sfx = .sfx_sgtatk
			case .S_BOSS_ATK2, .S_BOS2_ATK2, .S_HEAD_ATK2: sfx = .sfx_firsht
			case .S_SKULL_ATK2: sfx = .sfx_sklatk
			case .S_SPID_ATK2, .S_SPID_ATK3: sfx = .sfx_shotgn
			case .S_BSPI_ATK2: sfx = .sfx_plasma
			case .S_CYBER_ATK2, .S_CYBER_ATK4, .S_CYBER_ATK6: sfx = .sfx_rlaunc
			case .S_PAIN_ATK3: sfx = .sfx_sklatk
			case: sfx = .sfx_None
			}

			if sfx != .sfx_None {
				S_StartSound(nil, int(sfx))
			}
		}
	}

	if castframes == 12 {
		castattacking = true
		if castonmelee != 0 {
			caststate = &states[mobjinfo[castorder[castnum].type].meleestate]
		} else {
			caststate = &states[mobjinfo[castorder[castnum].type].missilestate]
		}
		castonmelee ~= 1
		if caststate == &states[statenum_t.S_NULL] {
			if castonmelee != 0 {
				caststate = &states[mobjinfo[castorder[castnum].type].meleestate]
			} else {
				caststate = &states[mobjinfo[castorder[castnum].type].missilestate]
			}
		}
	}

	if castattacking {
		if castframes == 24 || caststate == &states[mobjinfo[castorder[castnum].type].seestate] {
			castattacking = false
			castframes = 0
			caststate = &states[mobjinfo[castorder[castnum].type].seestate]
		}
	}

	casttics = int(caststate.tics)
	if casttics == -1 {
		casttics = 15
	}
}

F_CastResponder :: proc(ev: ^event_t) -> boolean {
	if ev.type != .ev_keydown {
		return false
	}
	if castdeath {
		return true
	}

	castdeath = true
	caststate = &states[mobjinfo[castorder[castnum].type].deathstate]
	casttics = int(caststate.tics)
	castframes = 0
	castattacking = false
	if mobjinfo[castorder[castnum].type].deathsound != 0 {
		S_StartSound(nil, int(mobjinfo[castorder[castnum].type].deathsound))
	}

	return true
}

F_CastPrint :: proc(text: cstring) {
	ch := ([^]u8)(text)
	width := 0

	for ch != nil {
		c := int(ch[0])
		ch = &ch[1]
		if c == 0 {
			break
		}
		c = int(libc.toupper(i32(c))) - int(HU_FONTSTART)
		if c < 0 || c > HU_FONTSIZE {
			width += 4
			continue
		}
		w := int(SHORT(hu_font[c].width))
		width += w
	}

	cx := 160 - width / 2
	ch = ([^]u8)(text)
	for ch != nil {
		c := int(ch[0])
		ch = &ch[1]
		if c == 0 {
			break
		}
		c = int(libc.toupper(i32(c))) - int(HU_FONTSTART)
		if c < 0 || c > HU_FONTSIZE {
			cx += 4
			continue
		}
		w := int(SHORT(hu_font[c].width))
		V_DrawPatch(cx, 180, 0, hu_font[c])
		cx += w
	}
}

F_CastDrawer :: proc() {
	V_DrawPatch(0, 0, 0, (^patch_t)(W_CacheLumpName("BOSSBACK", PU_CACHE)))

	F_CastPrint(castorder[castnum].name)

	sprdef := &sprites[caststate.sprite]
	sprframe := &sprdef.spriteframes[caststate.frame & FF_FRAMEMASK]
	lump := int(sprframe.lump[0])
	flip := sprframe.flip[0] != 0

	patch := (^patch_t)(W_CacheLumpNum(lump + firstspritelump, PU_CACHE))
	if flip {
		V_DrawPatchFlipped(160, 170, 0, patch)
	} else {
		V_DrawPatch(160, 170, 0, patch)
	}
}

F_DrawPatchCol :: proc(x: int, patch: ^patch_t, col: int) {
	colofs := ([^]i32)(&patch.columnofs[0])
	column := (^column_t)(uintptr(patch) + uintptr(LONG(colofs[col])))
	desttop := &screens[0][x]

	for column.topdelta != 0xff {
		source := ([^]byte)(uintptr(column) + 3)
		dest := ([^]byte)(uintptr(desttop) + uintptr(column.topdelta) * SCREENWIDTH)
		count := int(column.length)

		for count > 0 {
			count -= 1
			dest[0] = source[0]
			source = &source[1]
			dest = &dest[SCREENWIDTH]
		}
		column = (^column_t)(uintptr(column) + uintptr(column.length) + 4)
	}
}

F_BunnyScroll :: proc() {
	name: [10]u8
	p1 := (^patch_t)(W_CacheLumpName("PFUB2", PU_LEVEL))
	p2 := (^patch_t)(W_CacheLumpName("PFUB1", PU_LEVEL))

	V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT)

	scrolled := 320 - (finalecount - 230) / 2
	if scrolled > 320 {
		scrolled = 320
	}
	if scrolled < 0 {
		scrolled = 0
	}

	for x := 0; x < SCREENWIDTH; x += 1 {
		if x + scrolled < 320 {
			F_DrawPatchCol(x, p1, x + scrolled)
		} else {
			F_DrawPatchCol(x, p2, x + scrolled - 320)
		}
	}

	if finalecount < 1130 {
		return
	}
	if finalecount < 1180 {
		V_DrawPatch((SCREENWIDTH - 13 * 8) / 2, (SCREENHEIGHT - 8 * 8) / 2, 0, (^patch_t)(W_CacheLumpName("END0", PU_CACHE)))
		f_laststage = 0
		return
	}

	stage := (finalecount - 1180) / 5
	if stage > 6 {
		stage = 6
	}
	if stage > f_laststage {
		S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		f_laststage = stage
	}

	libc.sprintf(&name[0], "END%i", i32(stage))
	V_DrawPatch((SCREENWIDTH - 13 * 8) / 2, (SCREENHEIGHT - 8 * 8) / 2, 0, (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_CACHE)))
}

F_Drawer :: proc() {
	if finalestage == 2 {
		F_CastDrawer()
		return
	}

	if finalestage == 0 {
		F_TextWrite()
	} else {
		switch gameepisode {
		case 1:
			if gamemode == .retail {
				V_DrawPatch(0, 0, 0, (^patch_t)(W_CacheLumpName("CREDIT", PU_CACHE)))
			} else {
				V_DrawPatch(0, 0, 0, (^patch_t)(W_CacheLumpName("HELP2", PU_CACHE)))
			}
		case 2:
			V_DrawPatch(0, 0, 0, (^patch_t)(W_CacheLumpName("VICTORY2", PU_CACHE)))
		case 3:
			F_BunnyScroll()
		case 4:
			V_DrawPatch(0, 0, 0, (^patch_t)(W_CacheLumpName("ENDPIC", PU_CACHE)))
		}
	}
}
