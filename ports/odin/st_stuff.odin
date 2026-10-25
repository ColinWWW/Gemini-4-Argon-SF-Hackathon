// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

STARTREDPALS   :: 1
STARTBONUSPALS :: 9
NUMREDPALS     :: 8
NUMBONUSPALS   :: 4
RADIATIONPAL   :: 13

ST_FACEPROBABILITY :: 96
ST_TOGGLECHAT      :: KEY_ENTER

ST_X  :: 0
ST_X2 :: 104
ST_FX :: 143
ST_FY :: 169

ST_NUMPAINFACES     :: 5
ST_NUMSTRAIGHTFACES :: 3
ST_NUMTURNFACES     :: 2
ST_NUMSPECIALFACES  :: 3
ST_FACESTRIDE       :: ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES + ST_NUMSPECIALFACES
ST_NUMEXTRAFACES    :: 2
ST_NUMFACES         :: ST_FACESTRIDE * ST_NUMPAINFACES + ST_NUMEXTRAFACES

ST_TURNOFFSET     :: ST_NUMSTRAIGHTFACES
ST_OUCHOFFSET     :: ST_TURNOFFSET + ST_NUMTURNFACES
ST_EVILGRINOFFSET :: ST_OUCHOFFSET + 1
ST_RAMPAGEOFFSET  :: ST_EVILGRINOFFSET + 1
ST_GODFACE        :: ST_NUMPAINFACES * ST_FACESTRIDE
ST_DEADFACE       :: ST_GODFACE + 1

ST_FACESX :: 143
ST_FACESY :: 168

ST_EVILGRINCOUNT     :: 2 * TICRATE
ST_STRAIGHTFACECOUNT :: TICRATE / 2
ST_TURNCOUNT         :: 1 * TICRATE
ST_OUCHCOUNT         :: 1 * TICRATE
ST_RAMPAGEDELAY      :: 2 * TICRATE

ST_MUCHPAIN :: 20

ST_AMMOWIDTH :: 3
ST_AMMOX     :: 44
ST_AMMOY     :: 171

ST_HEALTHWIDTH :: 3
ST_HEALTHX     :: 90
ST_HEALTHY     :: 171

ST_ARMSX      :: 111
ST_ARMSY      :: 172
ST_ARMSBGX    :: 104
ST_ARMSBGY    :: 168
ST_ARMSXSPACE :: 12
ST_ARMSYSPACE :: 10

ST_FRAGSX     :: 138
ST_FRAGSY     :: 171
ST_FRAGSWIDTH :: 2

ST_ARMORWIDTH :: 3
ST_ARMORX     :: 221
ST_ARMORY     :: 171

ST_KEY0WIDTH  :: 8
ST_KEY0HEIGHT :: 5
ST_KEY0X      :: 239
ST_KEY0Y      :: 171
ST_KEY1WIDTH  :: ST_KEY0WIDTH
ST_KEY1X      :: 239
ST_KEY1Y      :: 181
ST_KEY2WIDTH  :: ST_KEY0WIDTH
ST_KEY2X      :: 239
ST_KEY2Y      :: 191

ST_AMMO0WIDTH  :: 3
ST_AMMO0HEIGHT :: 6
ST_AMMO0X      :: 288
ST_AMMO0Y      :: 173
ST_AMMO1WIDTH  :: ST_AMMO0WIDTH
ST_AMMO1X      :: 288
ST_AMMO1Y      :: 179
ST_AMMO2WIDTH  :: ST_AMMO0WIDTH
ST_AMMO2X      :: 288
ST_AMMO2Y      :: 191
ST_AMMO3WIDTH  :: ST_AMMO0WIDTH
ST_AMMO3X      :: 288
ST_AMMO3Y      :: 185

ST_MAXAMMO0WIDTH  :: 3
ST_MAXAMMO0HEIGHT :: 5
ST_MAXAMMO0X      :: 314
ST_MAXAMMO0Y      :: 173
ST_MAXAMMO1WIDTH  :: ST_MAXAMMO0WIDTH
ST_MAXAMMO1X      :: 314
ST_MAXAMMO1Y      :: 179
ST_MAXAMMO2WIDTH  :: ST_MAXAMMO0WIDTH
ST_MAXAMMO2X      :: 314
ST_MAXAMMO2Y      :: 191
ST_MAXAMMO3WIDTH  :: ST_MAXAMMO0WIDTH
ST_MAXAMMO3X      :: 314
ST_MAXAMMO3Y      :: 185

ST_MSGWIDTH :: 52

st_plr: ^player_t
st_firsttime: boolean
veryfirsttime: int = 1
lu_palette: int
st_clock: u32
st_msgcounter: int = 0
st_chatstate: st_chatstateenum_t
st_gamestate: st_stateenum_t
st_statusbaron: boolean
st_chat: boolean
st_oldchat: boolean
st_cursoron: boolean
st_notdeathmatch: boolean
st_armson: boolean
st_fragson: boolean

sbar: ^patch_t
tallnum: [10]^patch_t
tallpercent: ^patch_t
shortnum: [10]^patch_t
keys: [NUMCARDS]^patch_t
faces: [ST_NUMFACES]^patch_t
faceback: ^patch_t
armsbg: ^patch_t
arms: [6][2]^patch_t
st_arms_owned: [6]int

w_ready: st_number_t
w_frags: st_number_t
w_health: st_percent_t
w_armsbg: st_binicon_t
w_arms: [6]st_multicon_t
w_faces: st_multicon_t
w_keyboxes: [3]st_multicon_t
w_armor: st_percent_t
w_ammo: [4]st_number_t
w_maxammo: [4]st_number_t

st_fragscount: int
st_oldhealth: int = -1
oldweaponsowned: [NUMWEAPONS]boolean
st_facecount: int = 0
st_faceindex: int = 0
keyboxes: [3]int
st_randomnumber: int

st_lastcalc: int
st_calc_oldhealth: int = -1
st_lastattackdown: int = -1
st_priority: int = 0
st_largeammo: int = 1994
st_palette: int = 0
st_stopped: boolean = true
st_mypos_buf: [ST_MSGWIDTH]u8

cheat_mus_seq := [9]u8{ 0xb2, 0x26, 0xb6, 0xae, 0xea, 1, 0, 0, 0xff }
cheat_choppers_seq := [11]u8{ 0xb2, 0x26, 0xe2, 0x32, 0xf6, 0x2a, 0x2a, 0xa6, 0x6a, 0xea, 0xff }
cheat_god_seq := [6]u8{ 0xb2, 0x26, 0x26, 0xaa, 0x26, 0xff }
cheat_ammo_seq := [6]u8{ 0xb2, 0x26, 0xf2, 0x66, 0xa2, 0xff }
cheat_ammonokey_seq := [5]u8{ 0xb2, 0x26, 0x66, 0xa2, 0xff }
cheat_noclip_seq := [11]u8{ 0xb2, 0x26, 0xea, 0x2a, 0xb2, 0xea, 0x2a, 0xf6, 0x2a, 0x26, 0xff }
cheat_commercial_noclip_seq := [7]u8{ 0xb2, 0x26, 0xe2, 0x36, 0xb2, 0x2a, 0xff }

cheat_powerup_seq := [7][10]u8{
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6e, 0xff },
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xea, 0xff },
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xb2, 0xff },
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6a, 0xff },
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xa2, 0xff },
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x36, 0xff },
	{ 0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xff, 0 },
}

cheat_clev_seq := [10]u8{ 0xb2, 0x26, 0xe2, 0x36, 0xa6, 0x6e, 1, 0, 0, 0xff }
cheat_mypos_seq := [8]u8{ 0xb2, 0x26, 0xb6, 0xba, 0x2a, 0xf6, 0xea, 0xff }

cheat_mus := cheatseq_t{ &cheat_mus_seq[0], nil }
cheat_god := cheatseq_t{ &cheat_god_seq[0], nil }
cheat_ammo := cheatseq_t{ &cheat_ammo_seq[0], nil }
cheat_ammonokey := cheatseq_t{ &cheat_ammonokey_seq[0], nil }
cheat_noclip := cheatseq_t{ &cheat_noclip_seq[0], nil }
cheat_commercial_noclip := cheatseq_t{ &cheat_commercial_noclip_seq[0], nil }

cheat_powerup := [7]cheatseq_t{
	{ &cheat_powerup_seq[0][0], nil },
	{ &cheat_powerup_seq[1][0], nil },
	{ &cheat_powerup_seq[2][0], nil },
	{ &cheat_powerup_seq[3][0], nil },
	{ &cheat_powerup_seq[4][0], nil },
	{ &cheat_powerup_seq[5][0], nil },
	{ &cheat_powerup_seq[6][0], nil },
}

cheat_choppers := cheatseq_t{ &cheat_choppers_seq[0], nil }
cheat_clev := cheatseq_t{ &cheat_clev_seq[0], nil }
cheat_mypos := cheatseq_t{ &cheat_mypos_seq[0], nil }

ST_refreshBackground :: proc() {
	if st_statusbaron {
		V_DrawPatch(ST_X, 0, ST_BG, sbar)
		if netgame {
			V_DrawPatch(ST_FX, 0, ST_BG, faceback)
		}
		V_CopyRect(ST_X, 0, ST_BG, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y, ST_FG)
	}
}

ST_Responder :: proc(ev: ^event_t) -> boolean {
	if ev.type == .ev_keyup && ((ev.data1 & 0xffff0000) == AM_MSGHEADER) {
		switch ev.data1 {
		case AM_MSGENTERED:
			st_gamestate = .AutomapState
			st_firsttime = true
		case AM_MSGEXITED:
			st_gamestate = .FirstPersonState
		}
	} else if ev.type == .ev_keydown {
		if !netgame {
			if cht_CheckCheat(&cheat_god, u8(ev.data1)) != 0 {
				st_plr.cheats ~= CF_GODMODE
				if (st_plr.cheats & CF_GODMODE) != 0 {
					if st_plr.mo != nil {
						st_plr.mo.health = 100
					}
					st_plr.health = 100
					st_plr.message = STSTR_DQDON
				} else {
					st_plr.message = STSTR_DQDOFF
				}
			} else if cht_CheckCheat(&cheat_ammonokey, u8(ev.data1)) != 0 {
				st_plr.armorpoints = 200
				st_plr.armortype = 2
				for i := 0; i < NUMWEAPONS; i += 1 {
					st_plr.weaponowned[i] = true
				}
				for i := 0; i < NUMAMMO; i += 1 {
					st_plr.ammo[i] = st_plr.maxammo[i]
				}
				st_plr.message = STSTR_FAADDED
			} else if cht_CheckCheat(&cheat_ammo, u8(ev.data1)) != 0 {
				st_plr.armorpoints = 200
				st_plr.armortype = 2
				for i := 0; i < NUMWEAPONS; i += 1 {
					st_plr.weaponowned[i] = true
				}
				for i := 0; i < NUMAMMO; i += 1 {
					st_plr.ammo[i] = st_plr.maxammo[i]
				}
				for i := 0; i < NUMCARDS; i += 1 {
					st_plr.cards[i] = true
				}
				st_plr.message = STSTR_KFAADDED
			} else if cht_CheckCheat(&cheat_mus, u8(ev.data1)) != 0 {
				buf: [3]u8
				st_plr.message = STSTR_MUS
				cht_GetParam(&cheat_mus, &buf[0])
				if gamemode == .commercial {
					musnum := int(musicenum_t.mus_runnin) + int(buf[0] - '0') * 10 + int(buf[1] - '0') - 1
					if int(buf[0] - '0') * 10 + int(buf[1] - '0') > 35 {
						st_plr.message = STSTR_NOMUS
					} else {
						S_ChangeMusic(musnum, 1)
					}
				} else {
					musnum := int(musicenum_t.mus_e1m1) + int(buf[0] - '1') * 9 + int(buf[1] - '1')
					if int(buf[0] - '1') * 9 + int(buf[1] - '1') > 31 {
						st_plr.message = STSTR_NOMUS
					} else {
						S_ChangeMusic(musnum, 1)
					}
				}
			} else if cht_CheckCheat(&cheat_noclip, u8(ev.data1)) != 0 || cht_CheckCheat(&cheat_commercial_noclip, u8(ev.data1)) != 0 {
				st_plr.cheats ~= CF_NOCLIP
				if (st_plr.cheats & CF_NOCLIP) != 0 {
					st_plr.message = STSTR_NCON
				} else {
					st_plr.message = STSTR_NCOFF
				}
			}

			for i := 0; i < 6; i += 1 {
				if cht_CheckCheat(&cheat_powerup[i], u8(ev.data1)) != 0 {
					if st_plr.powers[i] == 0 {
						P_GivePower(st_plr, i)
					} else if i != pw_strength {
						st_plr.powers[i] = 1
					} else {
						st_plr.powers[i] = 0
					}
					st_plr.message = STSTR_BEHOLDX
				}
			}

			if cht_CheckCheat(&cheat_powerup[6], u8(ev.data1)) != 0 {
				st_plr.message = STSTR_BEHOLD
			} else if cht_CheckCheat(&cheat_choppers, u8(ev.data1)) != 0 {
				st_plr.weaponowned[weapontype_t.wp_chainsaw] = true
				st_plr.powers[pw_invulnerability] = 1
				st_plr.message = STSTR_CHOPPERS
			} else if cht_CheckCheat(&cheat_mypos, u8(ev.data1)) != 0 {
				libc.sprintf(&st_mypos_buf[0], "ang=0x%x;x,y=(0x%x,0x%x)", u32(players[consoleplayer].mo.angle), u32(players[consoleplayer].mo.x), u32(players[consoleplayer].mo.y))
				st_plr.message = cstring(&st_mypos_buf[0])
			}
		}

		if cht_CheckCheat(&cheat_clev, u8(ev.data1)) != 0 {
			buf: [3]u8
			cht_GetParam(&cheat_clev, &buf[0])

			epsd, map_num: int
			if gamemode == .commercial {
				epsd = 0
				map_num = int(buf[0] - '0') * 10 + int(buf[1] - '0')
			} else {
				epsd = int(buf[0] - '0')
				map_num = int(buf[1] - '0')
			}

			if epsd < 1 { return false }
			if map_num < 1 { return false }
			if gamemode == .retail && (epsd > 4 || map_num > 9) { return false }
			if gamemode == .registered && (epsd > 3 || map_num > 9) { return false }
			if gamemode == .shareware && (epsd > 1 || map_num > 9) { return false }
			if gamemode == .commercial && (epsd > 1 || map_num > 34) { return false }

			st_plr.message = STSTR_CLEV
			G_DeferedInitNew(gameskill, epsd, map_num)
		}
	}
	return false
}

ST_calcPainOffset :: proc() -> int {
	health := st_plr.health > 100 ? 100 : st_plr.health
	if health != st_calc_oldhealth {
		st_lastcalc = ST_FACESTRIDE * (((100 - health) * ST_NUMPAINFACES) / 101)
		st_calc_oldhealth = health
	}
	return st_lastcalc
}

ST_updateFaceWidget :: proc() {
	if st_priority < 10 {
		if st_plr.health == 0 {
			st_priority = 9
			st_faceindex = ST_DEADFACE
			st_facecount = 1
		}
	}

	if st_priority < 9 {
		if st_plr.bonuscount != 0 {
			doevilgrin := false
			for i := 0; i < NUMWEAPONS; i += 1 {
				if oldweaponsowned[i] != st_plr.weaponowned[i] {
					doevilgrin = true
					oldweaponsowned[i] = st_plr.weaponowned[i]
				}
			}
			if doevilgrin {
				st_priority = 8
				st_facecount = ST_EVILGRINCOUNT
				st_faceindex = ST_calcPainOffset() + ST_EVILGRINOFFSET
			}
		}
	}

	if st_priority < 8 {
		if st_plr.damagecount != 0 && st_plr.attacker != nil && st_plr.attacker != st_plr.mo {
			st_priority = 7

			if st_plr.health - st_oldhealth > ST_MUCHPAIN {
				st_facecount = ST_TURNCOUNT
				st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET
			} else {
				badguyangle := R_PointToAngle2(st_plr.mo.x, st_plr.mo.y, st_plr.attacker.x, st_plr.attacker.y)
				diffang: angle_t
				i: boolean
				if badguyangle > st_plr.mo.angle {
					diffang = badguyangle - st_plr.mo.angle
					i = diffang > ANG180
				} else {
					diffang = st_plr.mo.angle - badguyangle
					i = diffang <= ANG180
				}

				st_facecount = ST_TURNCOUNT
				st_faceindex = ST_calcPainOffset()

				if diffang < ANG45 {
					st_faceindex += ST_RAMPAGEOFFSET
				} else if i {
					st_faceindex += ST_TURNOFFSET
				} else {
					st_faceindex += ST_TURNOFFSET + 1
				}
			}
		}
	}

	if st_priority < 7 {
		if st_plr.damagecount != 0 {
			if st_plr.health - st_oldhealth > ST_MUCHPAIN {
				st_priority = 7
				st_facecount = ST_TURNCOUNT
				st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET
			} else {
				st_priority = 6
				st_facecount = ST_TURNCOUNT
				st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET
			}
		}
	}

	if st_priority < 6 {
		if st_plr.attackdown != 0 {
			if st_lastattackdown == -1 {
				st_lastattackdown = ST_RAMPAGEDELAY
			} else {
				st_lastattackdown -= 1
				if st_lastattackdown == 0 {
					st_priority = 5
					st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET
					st_facecount = 1
					st_lastattackdown = 1
				}
			}
		} else {
			st_lastattackdown = -1
		}
	}

	if st_priority < 5 {
		if (st_plr.cheats & CF_GODMODE) != 0 || st_plr.powers[pw_invulnerability] != 0 {
			st_priority = 4
			st_faceindex = ST_GODFACE
			st_facecount = 1
		}
	}

	if st_facecount == 0 {
		st_faceindex = ST_calcPainOffset() + (st_randomnumber % 3)
		st_facecount = ST_STRAIGHTFACECOUNT
		st_priority = 0
	}

	st_facecount -= 1
}

ST_updateWidgets :: proc() {
	if weaponinfo[st_plr.readyweapon].ammo == .am_noammo {
		w_ready.num = &st_largeammo
	} else {
		w_ready.num = &st_plr.ammo[weaponinfo[st_plr.readyweapon].ammo]
	}
	w_ready.data = int(st_plr.readyweapon)

	for i := 0; i < 6; i += 1 {
		st_arms_owned[i] = st_plr.weaponowned[i + 1] ? 1 : 0
	}

	for i := 0; i < 3; i += 1 {
		keyboxes[i] = st_plr.cards[i] ? i : -1
		if st_plr.cards[i + 3] {
			keyboxes[i] = i + 3
		}
	}

	ST_updateFaceWidget()

	st_notdeathmatch = deathmatch == 0
	st_armson = st_statusbaron && deathmatch == 0

	st_fragson = deathmatch != 0 && st_statusbaron
	st_fragscount = 0

	for i := 0; i < MAXPLAYERS; i += 1 {
		if i != consoleplayer {
			st_fragscount += st_plr.frags[i]
		} else {
			st_fragscount -= st_plr.frags[i]
		}
	}

	st_msgcounter -= 1
	if st_msgcounter == 0 {
		st_chat = st_oldchat
	}
}

ST_Ticker :: proc() {
	st_clock += 1
	st_randomnumber = M_Random()
	ST_updateWidgets()
	st_oldhealth = st_plr.health
}

ST_doPaletteStuff :: proc() {
	cnt := st_plr.damagecount

	if st_plr.powers[pw_strength] != 0 {
		bzc := 12 - (st_plr.powers[pw_strength] >> 6)
		if bzc > cnt {
			cnt = bzc
		}
	}

	palette: int
	if cnt != 0 {
		palette = (cnt + 7) >> 3
		if palette >= NUMREDPALS {
			palette = NUMREDPALS - 1
		}
		palette += STARTREDPALS
	} else if st_plr.bonuscount != 0 {
		palette = (st_plr.bonuscount + 7) >> 3
		if palette >= NUMBONUSPALS {
			palette = NUMBONUSPALS - 1
		}
		palette += STARTBONUSPALS
	} else if st_plr.powers[pw_ironfeet] > 4 * 32 || (st_plr.powers[pw_ironfeet] & 8) != 0 {
		palette = RADIATIONPAL
	} else {
		palette = 0
	}

	if palette != st_palette {
		st_palette = palette
		pal_base := ([^]byte)(W_CacheLumpNum(lu_palette, PU_CACHE))
		I_SetPalette(&pal_base[palette * 768])
	}
}

ST_drawWidgets :: proc(refresh: boolean) {
	st_armson = st_statusbaron && deathmatch == 0
	st_fragson = deathmatch != 0 && st_statusbaron

	for i := 0; i < 6; i += 1 {
		st_arms_owned[i] = st_plr.weaponowned[i + 1] ? 1 : 0
	}

	STlib_updateNum(&w_ready, refresh)

	for i := 0; i < 4; i += 1 {
		STlib_updateNum(&w_ammo[i], refresh)
		STlib_updateNum(&w_maxammo[i], refresh)
	}

	STlib_updatePercent(&w_health, refresh ? 1 : 0)
	STlib_updatePercent(&w_armor, refresh ? 1 : 0)

	STlib_updateBinIcon(&w_armsbg, refresh)

	for i := 0; i < 6; i += 1 {
		STlib_updateMultIcon(&w_arms[i], refresh)
	}

	STlib_updateMultIcon(&w_faces, refresh)

	for i := 0; i < 3; i += 1 {
		STlib_updateMultIcon(&w_keyboxes[i], refresh)
	}

	STlib_updateNum(&w_frags, refresh)
}

ST_doRefresh :: proc() {
	st_firsttime = false
	ST_refreshBackground()
	ST_drawWidgets(true)
}

ST_diffDraw :: proc() {
	ST_drawWidgets(false)
}

ST_Drawer :: proc(fullscreen: boolean, refresh: boolean) {
	st_statusbaron = (!fullscreen) || automapactive
	st_firsttime = st_firsttime || refresh

	ST_doPaletteStuff()

	if st_firsttime {
		ST_doRefresh()
	} else {
		ST_diffDraw()
	}
}

ST_loadGraphics :: proc() {
	namebuf: [9]u8

	for i := 0; i < 10; i += 1 {
		libc.sprintf(&namebuf[0], "STTNUM%d", i32(i))
		tallnum[i] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))

		libc.sprintf(&namebuf[0], "STYSNUM%d", i32(i))
		shortnum[i] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
	}

	tallpercent = (^patch_t)(W_CacheLumpName("STTPRCNT", PU_STATIC))

	for i := 0; i < NUMCARDS; i += 1 {
		libc.sprintf(&namebuf[0], "STKEYS%d", i32(i))
		keys[i] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
	}

	armsbg = (^patch_t)(W_CacheLumpName("STARMS", PU_STATIC))

	for i := 0; i < 6; i += 1 {
		libc.sprintf(&namebuf[0], "STGNUM%d", i32(i + 2))
		arms[i][0] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
		arms[i][1] = shortnum[i + 2]
	}

	libc.sprintf(&namebuf[0], "STFB%d", i32(consoleplayer))
	faceback = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))

	sbar = (^patch_t)(W_CacheLumpName("STBAR", PU_STATIC))

	facenum := 0
	for i := 0; i < ST_NUMPAINFACES; i += 1 {
		for j := 0; j < ST_NUMSTRAIGHTFACES; j += 1 {
			libc.sprintf(&namebuf[0], "STFST%d%d", i32(i), i32(j))
			faces[facenum] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
			facenum += 1
		}
		libc.sprintf(&namebuf[0], "STFTR%d0", i32(i))
		faces[facenum] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
		facenum += 1
		libc.sprintf(&namebuf[0], "STFTL%d0", i32(i))
		faces[facenum] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
		facenum += 1
		libc.sprintf(&namebuf[0], "STFOUCH%d", i32(i))
		faces[facenum] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
		facenum += 1
		libc.sprintf(&namebuf[0], "STFEVL%d", i32(i))
		faces[facenum] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
		facenum += 1
		libc.sprintf(&namebuf[0], "STFKILL%d", i32(i))
		faces[facenum] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
		facenum += 1
	}
	faces[facenum] = (^patch_t)(W_CacheLumpName("STFGOD0", PU_STATIC))
	facenum += 1
	faces[facenum] = (^patch_t)(W_CacheLumpName("STFDEAD0", PU_STATIC))
}

ST_loadData :: proc() {
	lu_palette = W_GetNumForName("PLAYPAL")
	ST_loadGraphics()
}

ST_unloadGraphics :: proc() {
	for i := 0; i < 10; i += 1 {
		Z_ChangeTag(tallnum[i], PU_CACHE)
		Z_ChangeTag(shortnum[i], PU_CACHE)
	}
	Z_ChangeTag(tallpercent, PU_CACHE)
	Z_ChangeTag(armsbg, PU_CACHE)

	for i := 0; i < 6; i += 1 {
		Z_ChangeTag(arms[i][0], PU_CACHE)
	}

	for i := 0; i < NUMCARDS; i += 1 {
		Z_ChangeTag(keys[i], PU_CACHE)
	}

	Z_ChangeTag(sbar, PU_CACHE)
	Z_ChangeTag(faceback, PU_CACHE)

	for i := 0; i < ST_NUMFACES; i += 1 {
		Z_ChangeTag(faces[i], PU_CACHE)
	}
}

ST_unloadData :: proc() {
	ST_unloadGraphics()
}

ST_initData :: proc() {
	st_firsttime = true
	st_plr = &players[consoleplayer]

	st_clock = 0
	st_chatstate = .StartChatState
	st_gamestate = .FirstPersonState

	st_statusbaron = true
	st_chat = false
	st_oldchat = false
	st_cursoron = false

	st_faceindex = 0
	st_palette = -1

	st_oldhealth = -1

	for i := 0; i < NUMWEAPONS; i += 1 {
		oldweaponsowned[i] = st_plr.weaponowned[i]
	}

	for i := 0; i < 3; i += 1 {
		keyboxes[i] = -1
	}

	STlib_init()
}

ST_createWidgets :: proc() {
	STlib_initNum(&w_ready, ST_AMMOX, ST_AMMOY, &tallnum[0], &st_plr.ammo[weaponinfo[st_plr.readyweapon].ammo], &st_statusbaron, ST_AMMOWIDTH)
	w_ready.data = int(st_plr.readyweapon)

	STlib_initPercent(&w_health, ST_HEALTHX, ST_HEALTHY, &tallnum[0], &st_plr.health, &st_statusbaron, tallpercent)

	STlib_initBinIcon(&w_armsbg, ST_ARMSBGX, ST_ARMSBGY, armsbg, &st_notdeathmatch, &st_statusbaron)

	for i := 0; i < 6; i += 1 {
		st_arms_owned[i] = st_plr.weaponowned[i + 1] ? 1 : 0
		STlib_initMultIcon(&w_arms[i], ST_ARMSX + (i % 3) * ST_ARMSXSPACE, ST_ARMSY + (i / 3) * ST_ARMSYSPACE, &arms[i][0], &st_arms_owned[i], &st_armson)
	}

	STlib_initNum(&w_frags, ST_FRAGSX, ST_FRAGSY, &tallnum[0], &st_fragscount, &st_fragson, ST_FRAGSWIDTH)

	STlib_initMultIcon(&w_faces, ST_FACESX, ST_FACESY, &faces[0], &st_faceindex, &st_statusbaron)

	STlib_initPercent(&w_armor, ST_ARMORX, ST_ARMORY, &tallnum[0], &st_plr.armorpoints, &st_statusbaron, tallpercent)

	STlib_initMultIcon(&w_keyboxes[0], ST_KEY0X, ST_KEY0Y, &keys[0], &keyboxes[0], &st_statusbaron)
	STlib_initMultIcon(&w_keyboxes[1], ST_KEY1X, ST_KEY1Y, &keys[0], &keyboxes[1], &st_statusbaron)
	STlib_initMultIcon(&w_keyboxes[2], ST_KEY2X, ST_KEY2Y, &keys[0], &keyboxes[2], &st_statusbaron)

	STlib_initNum(&w_ammo[0], ST_AMMO0X, ST_AMMO0Y, &shortnum[0], &st_plr.ammo[0], &st_statusbaron, ST_AMMO0WIDTH)
	STlib_initNum(&w_ammo[1], ST_AMMO1X, ST_AMMO1Y, &shortnum[0], &st_plr.ammo[1], &st_statusbaron, ST_AMMO1WIDTH)
	STlib_initNum(&w_ammo[2], ST_AMMO2X, ST_AMMO2Y, &shortnum[0], &st_plr.ammo[2], &st_statusbaron, ST_AMMO2WIDTH)
	STlib_initNum(&w_ammo[3], ST_AMMO3X, ST_AMMO3Y, &shortnum[0], &st_plr.ammo[3], &st_statusbaron, ST_AMMO3WIDTH)

	STlib_initNum(&w_maxammo[0], ST_MAXAMMO0X, ST_MAXAMMO0Y, &shortnum[0], &st_plr.maxammo[0], &st_statusbaron, ST_MAXAMMO0WIDTH)
	STlib_initNum(&w_maxammo[1], ST_MAXAMMO1X, ST_MAXAMMO1Y, &shortnum[0], &st_plr.maxammo[1], &st_statusbaron, ST_MAXAMMO1WIDTH)
	STlib_initNum(&w_maxammo[2], ST_MAXAMMO2X, ST_MAXAMMO2Y, &shortnum[0], &st_plr.maxammo[2], &st_statusbaron, ST_MAXAMMO2WIDTH)
	STlib_initNum(&w_maxammo[3], ST_MAXAMMO3X, ST_MAXAMMO3Y, &shortnum[0], &st_plr.maxammo[3], &st_statusbaron, ST_MAXAMMO3WIDTH)
}

ST_Start :: proc() {
	if !st_stopped {
		ST_Stop()
	}
	ST_initData()
	ST_createWidgets()
	st_stopped = false
}

ST_Stop :: proc() {
	if st_stopped {
		return
	}
	I_SetPalette(([^]byte)(W_CacheLumpNum(lu_palette, PU_CACHE)))
	st_stopped = true
}

ST_Init :: proc() {
	veryfirsttime = 0
	ST_loadData()
	screens[4] = ([^]byte)(Z_Malloc(ST_WIDTH * ST_HEIGHT, PU_STATIC, nil))
}
