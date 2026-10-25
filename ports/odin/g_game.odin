// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

SAVEGAMESIZE   :: 0x2c000
SAVESTRINGSIZE :: 24

gameaction: gameaction_t
gamestate: gamestate_t
gameskill: skill_t
respawnmonsters: boolean
gameepisode: int
gamemap: int

paused: boolean
sendpause: boolean
sendsave: boolean
usergame: boolean

timingdemo: boolean
nodrawers: boolean
noblit: boolean
starttime: int

viewactive: boolean

deathmatch: int
netgame: boolean
playeringame: [MAXPLAYERS]boolean
players: [MAXPLAYERS]player_t

consoleplayer: int
displayplayer: int
gametic: int
levelstarttic: int
totalkills, totalitems, totalsecret: int

demoname: [32]u8
demorecording: boolean
demoplayback: boolean
netdemo: boolean
demobuffer: [^]byte
demo_p: [^]byte
demoend: [^]byte
singledemo: boolean

precache: boolean = true

wminfo: wbstartstruct_t

consistancy: [MAXPLAYERS][BACKUPTICS]i16

savebuffer: [^]byte

key_right: int
key_left: int
key_up: int
key_down: int
key_strafeleft: int
key_straferight: int
key_fire: int
key_use: int
key_strafe: int
key_speed: int

mousebfire: int
mousebstrafe: int
mousebforward: int

joybfire: int
joybstrafe: int
joybuse: int
joybspeed: int

TURBOTHRESHOLD :: 0x32

forwardmove := [2]fixed_t{0x19, 0x32}
sidemove    := [2]fixed_t{0x18, 0x28}
angleturn   := [3]fixed_t{640, 1280, 320}

SLOWTURNTICS :: 6
NUMKEYS      :: 256

gamekeydown: [NUMKEYS]boolean
turnheld: int

mousearray: [4]boolean
mousex: int
mousey: int

dclicktime: int
dclickstate: boolean
dclicks: int
dclicktime2: int
dclickstate2: boolean
dclicks2: int

joyxmove: int
joyymove: int
joyarray: [5]boolean

savegameslot: int
savedescription: [32]u8

BODYQUESIZE :: 32
bodyque: [BODYQUESIZE]^mobj_t
bodyqueslot: int

statcopy: rawptr

turbomessage: [80]u8

pars := [4][10]int{
	{0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
	{0, 30, 75, 120, 90, 165, 180, 180, 30, 165},
	{0, 90, 90, 90, 120, 90, 360, 240, 30, 170},
	{0, 90, 45, 90, 150, 90, 90, 165, 30, 135},
}

cpars := [32]int{
	30, 90, 120, 120, 90, 150, 120, 120, 270, 90,
	210, 150, 150, 150, 210, 150, 420, 150, 210, 150,
	240, 150, 180, 150, 150, 300, 330, 420, 300, 180,
	120, 30,
}

secretexit: boolean
savename: [256]u8
VERSIONSIZE :: 16

d_skill: skill_t
d_episode: int
d_map: int

defdemoname: cstring
DEMOMARKER :: 0x80

G_CmdChecksum :: proc(cmd: ^ticcmd_t) -> int {
	sum := 0
	p := ([^]i32)(cmd)
	for i := 0; i < size_of(ticcmd_t) / 4 - 1; i += 1 {
		sum += int(p[i])
	}
	return sum
}

G_BuildTiccmd :: proc(cmd: ^ticcmd_t) {
	base := I_BaseTiccmd()
	cmd^ = base^

	cmd.consistancy = consistancy[consoleplayer][maketic % BACKUPTICS]

	strafe := gamekeydown[key_strafe] || mousearray[mousebstrafe + 1] || joyarray[joybstrafe + 1]
	speed := (gamekeydown[key_speed] || joyarray[joybspeed + 1]) ? 1 : 0

	forward := 0
	side := 0

	if joyxmove < 0 || joyxmove > 0 || gamekeydown[key_right] || gamekeydown[key_left] {
		turnheld += ticdup
	} else {
		turnheld = 0
	}

	tspeed: int
	if turnheld < SLOWTURNTICS {
		tspeed = 2
	} else {
		tspeed = speed
	}

	if strafe {
		if gamekeydown[key_right] {
			side += int(sidemove[speed])
		}
		if gamekeydown[key_left] {
			side -= int(sidemove[speed])
		}
		if joyxmove > 0 {
			side += int(sidemove[speed])
		}
		if joyxmove < 0 {
			side -= int(sidemove[speed])
		}
	} else {
		if gamekeydown[key_right] {
			cmd.angleturn -= i16(angleturn[tspeed])
		}
		if gamekeydown[key_left] {
			cmd.angleturn += i16(angleturn[tspeed])
		}
		if joyxmove > 0 {
			cmd.angleturn -= i16(angleturn[tspeed])
		}
		if joyxmove < 0 {
			cmd.angleturn += i16(angleturn[tspeed])
		}
	}

	if gamekeydown[key_up] {
		forward += int(forwardmove[speed])
	}
	if gamekeydown[key_down] {
		forward -= int(forwardmove[speed])
	}
	if joyymove < 0 {
		forward += int(forwardmove[speed])
	}
	if joyymove > 0 {
		forward -= int(forwardmove[speed])
	}
	if gamekeydown[key_straferight] {
		side += int(sidemove[speed])
	}
	if gamekeydown[key_strafeleft] {
		side -= int(sidemove[speed])
	}

	cmd.chatchar = HU_dequeueChatChar()

	if gamekeydown[key_fire] || mousearray[mousebfire + 1] || joyarray[joybfire + 1] {
		cmd.buttons |= BT_ATTACK
	}

	if gamekeydown[key_use] || joyarray[joybuse + 1] {
		cmd.buttons |= BT_USE
		dclicks = 0
	}

	for i := 0; i < NUMWEAPONS - 1; i += 1 {
		if gamekeydown[int('1') + i] {
			cmd.buttons |= BT_CHANGE
			cmd.buttons |= byte(i << BT_WEAPONSHIFT)
			break
		}
	}

	if mousearray[mousebforward + 1] {
		forward += int(forwardmove[speed])
	}

	if mousearray[mousebforward + 1] != dclickstate && dclicktime > 1 {
		dclickstate = mousearray[mousebforward + 1]
		if dclickstate {
			dclicks += 1
		}
		if dclicks == 2 {
			cmd.buttons |= BT_USE
			dclicks = 0
		} else {
			dclicktime = 0
		}
	} else {
		dclicktime += ticdup
		if dclicktime > 20 {
			dclicks = 0
			dclickstate = false
		}
	}

	bstrafe := mousearray[mousebstrafe + 1] || joyarray[joybstrafe + 1]
	if bstrafe != dclickstate2 && dclicktime2 > 1 {
		dclickstate2 = bstrafe
		if dclickstate2 {
			dclicks2 += 1
		}
		if dclicks2 == 2 {
			cmd.buttons |= BT_USE
			dclicks2 = 0
		} else {
			dclicktime2 = 0
		}
	} else {
		dclicktime2 += ticdup
		if dclicktime2 > 20 {
			dclicks2 = 0
			dclickstate2 = false
		}
	}

	forward += mousey
	if strafe {
		side += mousex * 2
	} else {
		cmd.angleturn -= i16(mousex * 0x8)
	}

	mousex = 0
	mousey = 0

	maxplmove := int(forwardmove[1])
	if forward > maxplmove {
		forward = maxplmove
	} else if forward < -maxplmove {
		forward = -maxplmove
	}
	if side > maxplmove {
		side = maxplmove
	} else if side < -maxplmove {
		side = -maxplmove
	}

	cmd.forwardmove += i8(forward)
	cmd.sidemove += i8(side)

	if sendpause {
		sendpause = false
		cmd.buttons = BT_SPECIAL | BTS_PAUSE
	}

	if sendsave {
		sendsave = false
		cmd.buttons = byte(BT_SPECIAL | BTS_SAVEGAME | (savegameslot << BTS_SAVESHIFT))
	}
}

G_DoLoadLevel :: proc() {
	skyflatnum = R_FlatNumForName(SKYFLATNAME)

	if gamemode == .commercial || int(gamemode) == int(GameMission_t.pack_tnt) || int(gamemode) == int(GameMission_t.pack_plut) {
		skytexture = R_TextureNumForName("SKY3")
		if gamemap < 12 {
			skytexture = R_TextureNumForName("SKY1")
		} else if gamemap < 21 {
			skytexture = R_TextureNumForName("SKY2")
		}
	}

	levelstarttic = gametic

	if wipegamestate == .GS_LEVEL {
		wipegamestate = gamestate_t(-1)
	}

	gamestate = .GS_LEVEL

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] && players[i].playerstate == .PST_DEAD {
			players[i].playerstate = .PST_REBORN
		}
		libc.memset(&players[i].frags[0], 0, size_of(players[i].frags))
	}

	P_SetupLevel(gameepisode, gamemap, 0, gameskill)
	displayplayer = consoleplayer
	starttime = I_GetTime()
	gameaction = .ga_nothing
	Z_CheckHeap()

	libc.memset(&gamekeydown[0], 0, size_of(gamekeydown))
	joyxmove = 0
	joyymove = 0
	mousex = 0
	mousey = 0
	sendpause = false
	sendsave = false
	paused = false
	libc.memset(&mousearray[1], 0, 3 * size_of(boolean))
	libc.memset(&joyarray[1], 0, 4 * size_of(boolean))
}

G_Responder :: proc(ev: ^event_t) -> boolean {
	if gamestate == .GS_LEVEL && ev.type == .ev_keydown && ev.data1 == KEY_F12 && (singledemo || deathmatch == 0) {
		for {
			displayplayer += 1
			if displayplayer == MAXPLAYERS {
				displayplayer = 0
			}
			if playeringame[displayplayer] || displayplayer == consoleplayer {
				break
			}
		}
		return true
	}

	if gameaction == .ga_nothing && !singledemo && (demoplayback || gamestate == .GS_DEMOSCREEN) {
		if ev.type == .ev_keydown || (ev.type == .ev_mouse && ev.data1 != 0) || (ev.type == .ev_joystick && ev.data1 != 0) {
			M_StartControlPanel()
			return true
		}
		return false
	}

	if gamestate == .GS_LEVEL {
		if HU_Responder(ev) {
			return true
		}
		if ST_Responder(ev) {
			return true
		}
		if AM_Responder(ev) {
			return true
		}
	}

	if gamestate == .GS_FINALE {
		if F_Responder(ev) {
			return true
		}
	}

	#partial switch ev.type {
	case .ev_keydown:
		if ev.data1 == KEY_PAUSE {
			sendpause = true
			return true
		}
		if ev.data1 < NUMKEYS && ev.data1 >= 0 {
			gamekeydown[ev.data1] = true
		}
		return true
	case .ev_keyup:
		if ev.data1 < NUMKEYS && ev.data1 >= 0 {
			gamekeydown[ev.data1] = false
		}
		return false
	case .ev_mouse:
		mousearray[1] = (ev.data1 & 1) != 0
		mousearray[2] = (ev.data1 & 2) != 0
		mousearray[3] = (ev.data1 & 4) != 0
		mousex = ev.data2 * (mouseSensitivity + 5) / 10
		mousey = ev.data3 * (mouseSensitivity + 5) / 10
		return true
	case .ev_joystick:
		joyarray[1] = (ev.data1 & 1) != 0
		joyarray[2] = (ev.data1 & 2) != 0
		joyarray[3] = (ev.data1 & 4) != 0
		joyarray[4] = (ev.data1 & 8) != 0
		joyxmove = ev.data2
		joyymove = ev.data3
		return true
	}

	return false
}

G_Ticker :: proc() {
	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] && players[i].playerstate == .PST_REBORN {
			G_DoReborn(i)
		}
	}

	for gameaction != .ga_nothing {
		switch gameaction {
		case .ga_loadlevel:
			G_DoLoadLevel()
		case .ga_newgame:
			G_DoNewGame()
		case .ga_loadgame:
			G_DoLoadGame()
		case .ga_savegame:
			G_DoSaveGame()
		case .ga_playdemo:
			G_DoPlayDemo()
		case .ga_completed:
			G_DoCompleted()
		case .ga_victory:
			F_StartFinale()
		case .ga_worlddone:
			G_DoWorldDone()
		case .ga_screenshot:
			M_ScreenShot()
			gameaction = .ga_nothing
		case .ga_nothing:
		}
	}

	buf := (gametic / ticdup) % BACKUPTICS

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			cmd := &players[i].cmd
			cmd^ = netcmds[i][buf]

			if demoplayback {
				G_ReadDemoTiccmd(cmd)
			}
			if demorecording {
				G_WriteDemoTiccmd(cmd)
			}

			if int(cmd.forwardmove) > TURBOTHRESHOLD && (gametic & 31) == 0 && ((gametic >> 5) & 3) == i {
				libc.snprintf(&turbomessage[0], size_of(turbomessage), "%s is turbo!", player_names[i])
				players[consoleplayer].message = cstring(&turbomessage[0])
			}

			if netgame && !netdemo && (gametic % ticdup) == 0 {
				if gametic > BACKUPTICS && consistancy[i][buf] != cmd.consistancy {
					I_Error("consistency failure (%i should be %i)", i32(cmd.consistancy), i32(consistancy[i][buf]))
				}
				if players[i].mo != nil {
					consistancy[i][buf] = i16(players[i].mo.x)
				} else {
					consistancy[i][buf] = i16(rndindex)
				}
			}
		}
	}

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			if (players[i].cmd.buttons & BT_SPECIAL) != 0 {
				switch players[i].cmd.buttons & BT_SPECIALMASK {
				case BTS_PAUSE:
					paused = !paused
					if paused {
						S_PauseSound()
					} else {
						S_ResumeSound()
					}
				case BTS_SAVEGAME:
					if savedescription[0] == 0 {
						libc.strcpy(&savedescription[0], "NET GAME")
					}
					savegameslot = int((players[i].cmd.buttons & BTS_SAVEMASK) >> BTS_SAVESHIFT)
					gameaction = .ga_savegame
				}
			}
		}
	}

	switch gamestate {
	case .GS_LEVEL:
		P_Ticker()
		ST_Ticker()
		AM_Ticker()
		HU_Ticker()
	case .GS_INTERMISSION:
		WI_Ticker()
	case .GS_FINALE:
		F_Ticker()
	case .GS_DEMOSCREEN:
		D_PageTicker()
	}
}

G_InitPlayer :: proc(player: int) {
	G_PlayerReborn(player)
}

G_PlayerFinishLevel :: proc(player: int) {
	p := &players[player]
	libc.memset(&p.powers[0], 0, size_of(p.powers))
	libc.memset(&p.cards[0], 0, size_of(p.cards))
	p.mo.flags &= ~int(MF_SHADOW)
	p.extralight = 0
	p.fixedcolormap = 0
	p.damagecount = 0
	p.bonuscount = 0
}

G_PlayerReborn :: proc(player: int) {
	frags: [MAXPLAYERS]int
	p := &players[player]

	frags = p.frags
	killcount := p.killcount
	itemcount := p.itemcount
	secretcount := p.secretcount

	p^ = {}

	p.frags = frags
	p.killcount = killcount
	p.itemcount = itemcount
	p.secretcount = secretcount

	p.usedown = 1
	p.attackdown = 1
	p.playerstate = .PST_LIVE
	p.health = MAXHEALTH
	p.readyweapon = .wp_pistol
	p.pendingweapon = .wp_pistol
	p.weaponowned[weapontype_t.wp_fist] = true
	p.weaponowned[weapontype_t.wp_pistol] = true
	p.ammo[ammotype_t.am_clip] = 50

	for i := 0; i < NUMAMMO; i += 1 {
		p.maxammo[i] = maxammo[i]
	}
}

G_CheckSpot :: proc(playernum: int, mthing: ^mapthing_t) -> boolean {
	if players[playernum].mo == nil {
		for i := 0; i < playernum; i += 1 {
			if players[i].mo.x == fixed_t(i32(mthing.x) << FRACBITS) && players[i].mo.y == fixed_t(i32(mthing.y) << FRACBITS) {
				return false
			}
		}
		return true
	}

	x := fixed_t(i32(mthing.x) << FRACBITS)
	y := fixed_t(i32(mthing.y) << FRACBITS)

	if !P_CheckPosition(players[playernum].mo, x, y) {
		return false
	}

	if bodyqueslot >= BODYQUESIZE {
		P_RemoveMobj(bodyque[bodyqueslot % BODYQUESIZE])
	}
	bodyque[bodyqueslot % BODYQUESIZE] = players[playernum].mo
	bodyqueslot += 1

	ss := R_PointInSubsector(x, y)
	an := (ANG45 * u32(int(mthing.angle) / 45)) >> ANGLETOFINESHIFT

	mo := P_SpawnMobj(x + 20 * finecosine[an], y + 20 * finesine[an], ss.sector.floorheight, .MT_TFOG)

	if players[consoleplayer].viewz != 1 {
		S_StartSound(mo, int(sfxenum_t.sfx_telept))
	}

	return true
}

G_DeathMatchSpawnPlayer :: proc(playernum: int) {
	selections := int(uintptr(deathmatch_p) - uintptr(&deathmatchstarts[0])) / size_of(mapthing_t)
	if selections < 4 {
		I_Error("Only %i deathmatch spots, 4 required", i32(selections))
	}

	for j := 0; j < 20; j += 1 {
		i := P_Random() % selections
		if G_CheckSpot(playernum, &deathmatchstarts[i]) {
			deathmatchstarts[i].type = i16(playernum + 1)
			P_SpawnPlayer(&deathmatchstarts[i])
			return
		}
	}

	P_SpawnPlayer(&playerstarts[playernum])
}

G_DoReborn :: proc(playernum: int) {
	if !netgame {
		gameaction = .ga_loadlevel
	} else {
		players[playernum].mo.player = nil

		if deathmatch != 0 {
			G_DeathMatchSpawnPlayer(playernum)
			return
		}

		if G_CheckSpot(playernum, &playerstarts[playernum]) {
			P_SpawnPlayer(&playerstarts[playernum])
			return
		}

		for i := 0; i < MAXPLAYERS; i += 1 {
			if G_CheckSpot(playernum, &playerstarts[i]) {
				playerstarts[i].type = i16(playernum + 1)
				P_SpawnPlayer(&playerstarts[i])
				playerstarts[i].type = i16(i + 1)
				return
			}
		}
		P_SpawnPlayer(&playerstarts[playernum])
	}
}

G_ScreenShot :: proc() {
	gameaction = .ga_screenshot
}

G_ExitLevel :: proc() {
	secretexit = false
	gameaction = .ga_completed
}

G_SecretExitLevel :: proc() {
	if gamemode == .commercial && W_CheckNumForName("map31") < 0 {
		secretexit = false
	} else {
		secretexit = true
	}
	gameaction = .ga_completed
}

G_DoCompleted :: proc() {
	gameaction = .ga_nothing

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			G_PlayerFinishLevel(i)
		}
	}

	if automapactive {
		AM_Stop()
	}

	if gamemode != .commercial {
		switch gamemap {
		case 8:
			gameaction = .ga_victory
			return
		case 9:
			for i := 0; i < MAXPLAYERS; i += 1 {
				players[i].didsecret = true
			}
		}
	}

	if gamemap == 8 && gamemode != .commercial {
		gameaction = .ga_victory
		return
	}

	if gamemap == 9 && gamemode != .commercial {
		for i := 0; i < MAXPLAYERS; i += 1 {
			players[i].didsecret = true
		}
	}

	wminfo.didsecret = players[consoleplayer].didsecret
	wminfo.epsd = gameepisode - 1
	wminfo.last = gamemap - 1

	if gamemode == .commercial {
		if secretexit {
			switch gamemap {
			case 15: wminfo.next = 30
			case 31: wminfo.next = 31
			}
		} else {
			switch gamemap {
			case 31, 32: wminfo.next = 15
			case: wminfo.next = gamemap
			}
		}
	} else {
		if secretexit {
			wminfo.next = 8
		} else if gamemap == 9 {
			switch gameepisode {
			case 1: wminfo.next = 3
			case 2: wminfo.next = 5
			case 3: wminfo.next = 6
			case 4: wminfo.next = 2
			}
		} else {
			wminfo.next = gamemap
		}
	}

	wminfo.maxkills = totalkills
	wminfo.maxitems = totalitems
	wminfo.maxsecret = totalsecret
	wminfo.maxfrags = 0
	if gamemode == .commercial {
		wminfo.partime = 35 * cpars[gamemap - 1]
	} else {
		wminfo.partime = 35 * pars[gameepisode][gamemap]
	}
	wminfo.pnum = consoleplayer

	for i := 0; i < MAXPLAYERS; i += 1 {
		wminfo.plyr[i].in_game = playeringame[i]
		wminfo.plyr[i].skills = players[i].killcount
		wminfo.plyr[i].sitems = players[i].itemcount
		wminfo.plyr[i].ssecret = players[i].secretcount
		wminfo.plyr[i].stime = leveltime
		wminfo.plyr[i].frags = players[i].frags
	}

	gamestate = .GS_INTERMISSION
	viewactive = false
	automapactive = false

	if statcopy != nil {
		libc.memcpy(statcopy, &wminfo, size_of(wminfo))
	}

	WI_Start(&wminfo)
}

G_WorldDone :: proc() {
	gameaction = .ga_worlddone

	if secretexit {
		players[consoleplayer].didsecret = true
	}

	if gamemode == .commercial {
		switch gamemap {
		case 15, 31:
			if !secretexit {
				break
			}
			F_StartFinale()
		case 6, 11, 20, 30:
			F_StartFinale()
		}
	}
}

G_DoWorldDone :: proc() {
	gamestate = .GS_LEVEL
	gamemap = wminfo.next + 1
	G_DoLoadLevel()
	gameaction = .ga_nothing
	viewactive = true
}

G_LoadGame :: proc(name: cstring) {
	libc.strcpy(&savename[0], name)
	gameaction = .ga_loadgame
}

G_DoLoadGame :: proc() {
	vcheck: [VERSIONSIZE]u8

	gameaction = .ga_nothing

	M_ReadFile(cstring(&savename[0]), &savebuffer)
	save_p = &savebuffer[SAVESTRINGSIZE]

	libc.memset(&vcheck[0], 0, VERSIONSIZE)
	libc.snprintf(&vcheck[0], size_of(vcheck), "version %i", i32(VERSION))
	if libc.strcmp(cstring(save_p), cstring(&vcheck[0])) != 0 {
		return
	}
	save_p = &save_p[VERSIONSIZE]

	gameskill = skill_t(save_p[0])
	save_p = &save_p[1]
	gameepisode = int(save_p[0])
	save_p = &save_p[1]
	gamemap = int(save_p[0])
	save_p = &save_p[1]
	for i := 0; i < MAXPLAYERS; i += 1 {
		playeringame[i] = save_p[0] != 0
		save_p = &save_p[1]
	}

	G_InitNew(gameskill, gameepisode, gamemap)

	a := int(save_p[0]); save_p = &save_p[1]
	b := int(save_p[0]); save_p = &save_p[1]
	c := int(save_p[0]); save_p = &save_p[1]
	leveltime = (a << 16) + (b << 8) + c

	P_UnArchivePlayers()
	P_UnArchiveWorld()
	P_UnArchiveThinkers()
	P_UnArchiveSpecials()

	if save_p[0] != 0x1d {
		I_Error("Bad savegame")
	}

	Z_Free(savebuffer)

	if setsizeneeded {
		R_ExecuteSetViewSize()
	}

	R_FillBackScreen()
}

G_SaveGame :: proc(slot: int, description: cstring) {
	savegameslot = slot
	libc.strcpy(&savedescription[0], description)
	sendsave = true
}

G_DoSaveGame :: proc() {
	name: [100]u8
	name2: [VERSIONSIZE]u8

	if M_CheckParm("-cdrom") != 0 {
		libc.snprintf(&name[0], size_of(name), "c:\\doomdata\\" + SAVEGAMENAME + "%d.dsg", i32(savegameslot))
	} else {
		libc.snprintf(&name[0], size_of(name), SAVEGAMENAME + "%d.dsg", i32(savegameslot))
	}

	savebuffer = &screens[1][0x4000]
	save_p = savebuffer

	libc.memcpy(save_p, &savedescription[0], SAVESTRINGSIZE)
	save_p = &save_p[SAVESTRINGSIZE]
	libc.memset(&name2[0], 0, VERSIONSIZE)
	libc.snprintf(&name2[0], size_of(name2), "version %i", i32(VERSION))
	libc.memcpy(save_p, &name2[0], VERSIONSIZE)
	save_p = &save_p[VERSIONSIZE]

	save_p[0] = byte(gameskill); save_p = &save_p[1]
	save_p[0] = byte(gameepisode); save_p = &save_p[1]
	save_p[0] = byte(gamemap); save_p = &save_p[1]
	for i := 0; i < MAXPLAYERS; i += 1 {
		save_p[0] = playeringame[i] ? 1 : 0
		save_p = &save_p[1]
	}
	save_p[0] = byte(leveltime >> 16); save_p = &save_p[1]
	save_p[0] = byte(leveltime >> 8); save_p = &save_p[1]
	save_p[0] = byte(leveltime); save_p = &save_p[1]

	P_ArchivePlayers()
	P_ArchiveWorld()
	P_ArchiveThinkers()
	P_ArchiveSpecials()

	save_p[0] = 0x1d
	save_p = &save_p[1]

	length := int(uintptr(save_p) - uintptr(savebuffer))
	if length > SAVEGAMESIZE {
		I_Error("Savegame buffer overrun")
	}
	M_WriteFile(cstring(&name[0]), savebuffer, length)
	gameaction = .ga_nothing
	savedescription[0] = 0

	players[consoleplayer].message = GGSAVED

	R_FillBackScreen()
}

G_DeferedInitNew :: proc(skill: skill_t, episode: int, map_num: int) {
	d_skill = skill
	d_episode = episode
	d_map = map_num
	gameaction = .ga_newgame
}

G_DoNewGame :: proc() {
	demoplayback = false
	netdemo = false
	netgame = false
	deathmatch = 0
	playeringame[1] = false
	playeringame[2] = false
	playeringame[3] = false
	respawnparm = false
	fastparm = false
	nomonsters = false
	consoleplayer = 0
	G_InitNew(d_skill, d_episode, d_map)
	gameaction = .ga_nothing
}

G_InitNew :: proc(skill_in: skill_t, episode_in: int, map_in: int) {
	skill := skill_in
	episode := episode_in
	map_num := map_in

	if paused {
		paused = false
		S_ResumeSound()
	}

	if skill > .sk_nightmare {
		skill = .sk_nightmare
	}

	if episode < 1 {
		episode = 1
	}

	if gamemode == .retail {
		if episode > 4 {
			episode = 4
		}
	} else if gamemode == .shareware {
		if episode > 1 {
			episode = 1
		}
	} else {
		if episode > 3 {
			episode = 3
		}
	}

	if map_num < 1 {
		map_num = 1
	}

	if map_num > 9 && gamemode != .commercial {
		map_num = 9
	}

	M_ClearRandom()

	if skill == .sk_nightmare || respawnparm {
		respawnmonsters = true
	} else {
		respawnmonsters = false
	}

	if fastparm || (skill == .sk_nightmare && gameskill != .sk_nightmare) {
		for i := int(statenum_t.S_SARG_RUN1); i <= int(statenum_t.S_SARG_PAIN2); i += 1 {
			states[i].tics >>= 1
		}
		mobjinfo[mobjtype_t.MT_BRUISERSHOT].speed = 20 * FRACUNIT
		mobjinfo[mobjtype_t.MT_HEADSHOT].speed = 20 * FRACUNIT
		mobjinfo[mobjtype_t.MT_TROOPSHOT].speed = 20 * FRACUNIT
	} else if skill != .sk_nightmare && gameskill == .sk_nightmare {
		for i := int(statenum_t.S_SARG_RUN1); i <= int(statenum_t.S_SARG_PAIN2); i += 1 {
			states[i].tics <<= 1
		}
		mobjinfo[mobjtype_t.MT_BRUISERSHOT].speed = 15 * FRACUNIT
		mobjinfo[mobjtype_t.MT_HEADSHOT].speed = 10 * FRACUNIT
		mobjinfo[mobjtype_t.MT_TROOPSHOT].speed = 10 * FRACUNIT
	}

	for i := 0; i < MAXPLAYERS; i += 1 {
		players[i].playerstate = .PST_REBORN
	}

	usergame = true
	paused = false
	demoplayback = false
	automapactive = false
	viewactive = true
	gameepisode = episode
	gamemap = map_num
	gameskill = skill

	viewactive = true

	if gamemode == .commercial {
		skytexture = R_TextureNumForName("SKY3")
		if gamemap < 12 {
			skytexture = R_TextureNumForName("SKY1")
		} else if gamemap < 21 {
			skytexture = R_TextureNumForName("SKY2")
		}
	} else {
		switch episode {
		case 1: skytexture = R_TextureNumForName("SKY1")
		case 2: skytexture = R_TextureNumForName("SKY2")
		case 3: skytexture = R_TextureNumForName("SKY3")
		case 4: skytexture = R_TextureNumForName("SKY4")
		}
	}

	G_DoLoadLevel()
}

G_ReadDemoTiccmd :: proc(cmd: ^ticcmd_t) {
	if demo_p[0] == DEMOMARKER {
		G_CheckDemoStatus()
		return
	}
	cmd.forwardmove = i8(demo_p[0]); demo_p = &demo_p[1]
	cmd.sidemove = i8(demo_p[0]); demo_p = &demo_p[1]
	cmd.angleturn = i16(u16(demo_p[0]) << 8); demo_p = &demo_p[1]
	cmd.buttons = demo_p[0]; demo_p = &demo_p[1]
}

G_WriteDemoTiccmd :: proc(cmd: ^ticcmd_t) {
	if gamekeydown[int('q')] {
		G_CheckDemoStatus()
	}
	demo_p[0] = byte(cmd.forwardmove)
	demo_p[1] = byte(cmd.sidemove)
	demo_p[2] = byte((cmd.angleturn + 128) >> 8)
	demo_p[3] = cmd.buttons
	if uintptr(demo_p) > uintptr(demoend) - 16 {
		G_CheckDemoStatus()
		return
	}

	G_ReadDemoTiccmd(cmd)
}

G_RecordDemo :: proc(name: cstring) {
	usergame = false
	libc.strcpy(&demoname[0], name)
	libc.strcat(&demoname[0], ".lmp")
	maxsize := 0x20000
	i := M_CheckParm("-maxdemo")
	if i != 0 && i < myargc - 1 {
		maxsize = int(libc.atoi(myargv[i + 1])) * 1024
	}
	demobuffer = ([^]byte)(Z_Malloc(maxsize, PU_STATIC, nil))
	demoend = &demobuffer[maxsize]
	demorecording = true
}

G_BeginRecording :: proc() {
	demo_p = demobuffer

	demo_p[0] = byte(VERSION); demo_p = &demo_p[1]
	demo_p[0] = byte(gameskill); demo_p = &demo_p[1]
	demo_p[0] = byte(gameepisode); demo_p = &demo_p[1]
	demo_p[0] = byte(gamemap); demo_p = &demo_p[1]
	demo_p[0] = byte(deathmatch); demo_p = &demo_p[1]
	demo_p[0] = respawnparm ? 1 : 0; demo_p = &demo_p[1]
	demo_p[0] = fastparm ? 1 : 0; demo_p = &demo_p[1]
	demo_p[0] = nomonsters ? 1 : 0; demo_p = &demo_p[1]
	demo_p[0] = byte(consoleplayer); demo_p = &demo_p[1]

	for i := 0; i < MAXPLAYERS; i += 1 {
		demo_p[0] = playeringame[i] ? 1 : 0
		demo_p = &demo_p[1]
	}
}

G_DeferedPlayDemo :: proc(name: cstring) {
	defdemoname = name
	gameaction = .ga_playdemo
}

G_DoPlayDemo :: proc() {
	gameaction = .ga_nothing
	demobuffer = ([^]byte)(W_CacheLumpName(defdemoname, PU_STATIC))
	demo_p = demobuffer

	demov := demo_p[0]
	demo_p = &demo_p[1]
	if int(demov) != VERSION && int(demov) != 109 {
		libc.fprintf(libc.stderr, "Demo is from a different game version!\n")
		gameaction = .ga_nothing
		return
	}

	skill := skill_t(demo_p[0]); demo_p = &demo_p[1]
	episode := int(demo_p[0]); demo_p = &demo_p[1]
	map_num := int(demo_p[0]); demo_p = &demo_p[1]
	deathmatch = int(demo_p[0]); demo_p = &demo_p[1]
	respawnparm = demo_p[0] != 0; demo_p = &demo_p[1]
	fastparm = demo_p[0] != 0; demo_p = &demo_p[1]
	nomonsters = demo_p[0] != 0; demo_p = &demo_p[1]
	consoleplayer = int(demo_p[0]); demo_p = &demo_p[1]

	for i := 0; i < MAXPLAYERS; i += 1 {
		playeringame[i] = demo_p[0] != 0
		demo_p = &demo_p[1]
	}
	if playeringame[1] {
		netgame = true
		netdemo = true
	}

	precache = false
	G_InitNew(skill, episode, map_num)
	precache = true

	usergame = false
	demoplayback = true
}

G_TimeDemo :: proc(name: cstring) {
	nodrawers = M_CheckParm("-nodraw") != 0
	noblit = M_CheckParm("-noblit") != 0
	timingdemo = true
	singletics = true

	defdemoname = name
	gameaction = .ga_playdemo
}

G_CheckDemoStatus :: proc() -> boolean {
	if timingdemo {
		endtime := I_GetTime()
		I_Error("timed %i gametics in %i realtics", i32(gametic), i32(endtime - starttime))
	}

	if demoplayback {
		if singledemo {
			I_Quit()
		}

		Z_ChangeTag(demobuffer, PU_CACHE)
		demoplayback = false
		netdemo = false
		netgame = false
		deathmatch = 0
		playeringame[1] = false
		playeringame[2] = false
		playeringame[3] = false
		respawnparm = false
		fastparm = false
		nomonsters = false
		consoleplayer = 0
		D_AdvanceDemo()
		return true
	}

	if demorecording {
		demo_p[0] = DEMOMARKER
		demo_p = &demo_p[1]
		M_WriteFile(cstring(&demoname[0]), demobuffer, int(uintptr(demo_p) - uintptr(demobuffer)))
		Z_Free(demobuffer)
		demorecording = false
		I_Error("Demo %s recorded", &demoname[0])
	}

	return false
}