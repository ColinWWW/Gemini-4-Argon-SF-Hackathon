// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

BGCOLOR :: 7
FGCOLOR :: 8

wadfiles: [MAXWADFILES]cstring

devparm: boolean
nomonsters: boolean
respawnparm: boolean
fastparm: boolean
drone: boolean
singletics: boolean = false

startskill: skill_t
startepisode: int
startmap: int
autostart: boolean

debugfile: ^libc.FILE
advancedemo: boolean

wadfile: [1024]u8
mapdir: [1024]u8
basedefault: [1024]u8

events: [MAXEVENTS]event_t
eventhead: int
eventtail: int

wipegamestate: gamestate_t = .GS_DEMOSCREEN

d_viewactivestate: boolean = false
d_menuactivestate: boolean = false
d_inhelpscreensstate: boolean = false
d_fullscreen: boolean = false
d_oldgamestate: gamestate_t = gamestate_t(-1)
d_borderdrawcount: int = 0

demosequence: int
pagetic: int
pagename: cstring
title: [128]u8

D_PostEvent :: proc(ev: ^event_t) {
	events[eventhead] = ev^
	eventhead += 1
	eventhead &= MAXEVENTS - 1
}

D_ProcessEvents :: proc() {
	if gamemode == .commercial && W_CheckNumForName("map01") < 0 {
		return
	}

	for ; eventtail != eventhead; eventtail = (eventtail + 1) & (MAXEVENTS - 1) {
		ev := &events[eventtail]
		if M_Responder(ev) {
			continue
		}
		G_Responder(ev)
	}
}

D_Display :: proc() {
	if nodrawers {
		return
	}

	redrawsbar := false

	if setsizeneeded {
		R_ExecuteSetViewSize()
		d_oldgamestate = gamestate_t(-1)
		d_borderdrawcount = 3
	}

	wipe: boolean
	if gamestate != wipegamestate {
		wipe = true
		wipe_StartScreen(0, 0, SCREENWIDTH, SCREENHEIGHT)
	} else {
		wipe = false
	}

	if gamestate == .GS_LEVEL && gametic != 0 {
		HU_Erase()
	}

	switch gamestate {
	case .GS_LEVEL:
		if gametic == 0 {
			break
		}
		if automapactive {
			AM_Drawer()
		}
		if wipe || (viewheight != 200 && d_fullscreen) {
			redrawsbar = true
		}
		if d_inhelpscreensstate && !inhelpscreens {
			redrawsbar = true
		}
		ST_Drawer(viewheight == 200, redrawsbar)
		d_fullscreen = viewheight == 200
	case .GS_INTERMISSION:
		WI_Drawer()
	case .GS_FINALE:
		F_Drawer()
	case .GS_DEMOSCREEN:
		D_PageDrawer()
	}

	I_UpdateNoBlit()

	if gamestate == .GS_LEVEL && !automapactive && gametic != 0 {
		R_RenderPlayerView(&players[displayplayer])
	}

	if gamestate == .GS_LEVEL && gametic != 0 {
		HU_Drawer()
	}

	if gamestate != d_oldgamestate && gamestate != .GS_LEVEL {
		I_SetPalette(([^]byte)(W_CacheLumpName("PLAYPAL", PU_CACHE)))
	}

	if gamestate == .GS_LEVEL && d_oldgamestate != .GS_LEVEL {
		d_viewactivestate = false
		R_FillBackScreen()
	}

	if gamestate == .GS_LEVEL && !automapactive && scaledviewwidth != 320 {
		if menuactive || d_menuactivestate || !d_viewactivestate {
			d_borderdrawcount = 3
		}
		if d_borderdrawcount != 0 {
			R_DrawViewBorder()
			d_borderdrawcount -= 1
		}
	}

	d_menuactivestate = menuactive
	d_viewactivestate = viewactive
	d_inhelpscreensstate = inhelpscreens
	d_oldgamestate = gamestate
	wipegamestate = gamestate

	if paused {
		y: int
		if automapactive {
			y = 4
		} else {
			y = viewwindowy + 4
		}
		V_DrawPatchDirect(viewwindowx + (scaledviewwidth - 68) / 2, y, 0, (^patch_t)(W_CacheLumpName("M_PAUSE", PU_CACHE)))
	}

	M_Drawer()
	NetUpdate()

	if !wipe {
		I_FinishUpdate()
		return
	}

	wipe_EndScreen(0, 0, SCREENWIDTH, SCREENHEIGHT)

	wipestart := I_GetTime() - 1

	for {
		nowtime: int
		tics: int
		for {
			nowtime = I_GetTime()
			tics = nowtime - wipestart
			if tics != 0 {
				break
			}
		}
		wipestart = nowtime
		done := wipe_ScreenWipe(wipe_Melt, 0, 0, SCREENWIDTH, SCREENHEIGHT, tics)
		I_UpdateNoBlit()
		M_Drawer()
		I_FinishUpdate()
		if done != 0 {
			break
		}
	}
}

D_DoomLoop :: proc() {
	if demorecording {
		G_BeginRecording()
	}

	if M_CheckParm("-debugfile") != 0 {
		filename: [20]u8
		libc.snprintf(&filename[0], size_of(filename), "debug%i.txt", i32(consoleplayer))
		libc.printf("debug output to: %s\n", &filename[0])
		debugfile = libc.fopen(cstring(&filename[0]), "w")
	}

	I_InitGraphics()

	for {
		I_StartFrame()

		if singletics {
			I_StartTic()
			D_ProcessEvents()
			G_BuildTiccmd(&netcmds[consoleplayer][maketic % BACKUPTICS])
			if advancedemo {
				D_DoAdvanceDemo()
			}
			M_Ticker()
			G_Ticker()
			gametic += 1
			maketic += 1
		} else {
			TryRunTics()
		}

		S_UpdateSounds(players[consoleplayer].mo)
		D_Display()
		I_UpdateSound()
		I_SubmitSound()
	}
}

D_PageTicker :: proc() {
	pagetic -= 1
	if pagetic < 0 {
		D_AdvanceDemo()
	}
}

D_PageDrawer :: proc() {
	V_DrawPatch(0, 0, 0, (^patch_t)(W_CacheLumpName(pagename, PU_CACHE)))
}

D_AdvanceDemo :: proc() {
	advancedemo = true
}

D_DoAdvanceDemo :: proc() {
	players[consoleplayer].playerstate = .PST_LIVE
	advancedemo = false
	usergame = false
	paused = false
	gameaction = .ga_nothing

	if gamemode == .retail {
		demosequence = (demosequence + 1) % 7
	} else {
		demosequence = (demosequence + 1) % 6
	}

	switch demosequence {
	case 0:
		if gamemode == .commercial {
			pagetic = 35 * 11
		} else {
			pagetic = 170
		}
		gamestate = .GS_DEMOSCREEN
		pagename = "TITLEPIC"
		if gamemode == .commercial {
			S_StartMusic(int(musicenum_t.mus_dm2ttl))
		} else {
			S_StartMusic(int(musicenum_t.mus_intro))
		}
	case 1:
		G_DeferedPlayDemo("demo1")
	case 2:
		pagetic = 200
		gamestate = .GS_DEMOSCREEN
		pagename = "CREDIT"
	case 3:
		G_DeferedPlayDemo("demo2")
	case 4:
		gamestate = .GS_DEMOSCREEN
		if gamemode == .commercial {
			pagetic = 35 * 11
			pagename = "TITLEPIC"
			S_StartMusic(int(musicenum_t.mus_dm2ttl))
		} else {
			pagetic = 200
			if gamemode == .retail {
				pagename = "CREDIT"
			} else {
				pagename = "HELP2"
			}
		}
	case 5:
		G_DeferedPlayDemo("demo3")
	case 6:
		G_DeferedPlayDemo("demo4")
	}
}

D_StartTitle :: proc() {
	gameaction = .ga_nothing
	demosequence = -1
	D_AdvanceDemo()
}

D_AddFile :: proc(file: cstring) {
	numwadfiles := 0
	for wadfiles[numwadfiles] != nil {
		numwadfiles += 1
	}

	newfile := ([^]u8)(libc.malloc(libc.strlen(file) + 1))
	libc.strcpy(newfile, file)
	wadfiles[numwadfiles] = cstring(newfile)
}

IdentifyVersion :: proc() {
	doomwaddir := libc.getenv("DOOMWADDIR")
	if doomwaddir == nil {
		doomwaddir = "."
	}

	doom2wad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 9 + 1))
	libc.snprintf(doom2wad, libc.strlen(doomwaddir) + 1 + 9 + 1, "%s/doom2.wad", doomwaddir)

	doomuwad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 8 + 1))
	libc.snprintf(doomuwad, libc.strlen(doomwaddir) + 1 + 8 + 1, "%s/doomu.wad", doomwaddir)

	doomwad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 8 + 1))
	libc.snprintf(doomwad, libc.strlen(doomwaddir) + 1 + 8 + 1, "%s/doom.wad", doomwaddir)

	doom1wad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 9 + 1))
	libc.snprintf(doom1wad, libc.strlen(doomwaddir) + 1 + 9 + 1, "%s/doom1.wad", doomwaddir)

	plutoniawad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 12 + 1))
	libc.snprintf(plutoniawad, libc.strlen(doomwaddir) + 1 + 12 + 1, "%s/plutonia.wad", doomwaddir)

	tntwad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 9 + 1))
	libc.snprintf(tntwad, libc.strlen(doomwaddir) + 1 + 9 + 1, "%s/tnt.wad", doomwaddir)

	doom2fwad := ([^]u8)(libc.malloc(libc.strlen(doomwaddir) + 1 + 10 + 1))
	libc.snprintf(doom2fwad, libc.strlen(doomwaddir) + 1 + 10 + 1, "%s/doom2f.wad", doomwaddir)

	iwad_p := M_CheckParm("-iwad")
	if iwad_p != 0 && iwad_p < myargc - 1 {
		gamemode = .shareware
		D_AddFile(myargv[iwad_p + 1])
		home := libc.getenv("HOME")
		if home == nil {
			home = libc.getenv("USERPROFILE")
		}
		if home == nil {
			home = "."
		}
		libc.snprintf(&basedefault[0], size_of(basedefault), "%s/.doomrc", home)
		return
	}

	home := libc.getenv("HOME")
	if home == nil {
		home = libc.getenv("USERPROFILE")
	}
	if home == nil {
		home = "."
	}
	libc.snprintf(&basedefault[0], size_of(basedefault), "%s/.doomrc", home)

	if M_CheckParm("-shdev") != 0 {
		gamemode = .shareware
		devparm = true
		D_AddFile(DEVDATA + "doom1.wad")
		D_AddFile(DEVMAPS + "data_se/texture1.lmp")
		D_AddFile(DEVMAPS + "data_se/pnames.lmp")
		libc.strcpy(&basedefault[0], DEVDATA + "default.cfg")
		return
	}

	if M_CheckParm("-regdev") != 0 {
		gamemode = .registered
		devparm = true
		D_AddFile(DEVDATA + "doom.wad")
		D_AddFile(DEVMAPS + "data_se/texture1.lmp")
		D_AddFile(DEVMAPS + "data_se/texture2.lmp")
		D_AddFile(DEVMAPS + "data_se/pnames.lmp")
		libc.strcpy(&basedefault[0], DEVDATA + "default.cfg")
		return
	}

	if M_CheckParm("-comdev") != 0 {
		gamemode = .commercial
		devparm = true
		D_AddFile(DEVDATA + "doom2.wad")
		D_AddFile(DEVMAPS + "cdata/texture1.lmp")
		D_AddFile(DEVMAPS + "cdata/pnames.lmp")
		libc.strcpy(&basedefault[0], DEVDATA + "default.cfg")
		return
	}

	if c_access(cstring(doom2fwad), R_OK) == 0 {
		gamemode = .commercial
		language = .french
		libc.printf("French version\n")
		D_AddFile(cstring(doom2fwad))
		return
	}

	if c_access(cstring(doom2wad), R_OK) == 0 {
		gamemode = .commercial
		D_AddFile(cstring(doom2wad))
		return
	}

	if c_access(cstring(plutoniawad), R_OK) == 0 {
		gamemode = .commercial
		D_AddFile(cstring(plutoniawad))
		return
	}

	if c_access(cstring(tntwad), R_OK) == 0 {
		gamemode = .commercial
		D_AddFile(cstring(tntwad))
		return
	}

	if c_access(cstring(doomuwad), R_OK) == 0 {
		gamemode = .retail
		D_AddFile(cstring(doomuwad))
		return
	}

	if c_access(cstring(doomwad), R_OK) == 0 {
		gamemode = .registered
		D_AddFile(cstring(doomwad))
		return
	}

	if c_access(cstring(doom1wad), R_OK) == 0 {
		gamemode = .shareware
		D_AddFile(cstring(doom1wad))
		return
	}

	libc.printf("Game mode indeterminate.\n")
	gamemode = .indetermined
}

FindResponseFile :: proc() {
	MAXARGVS :: 100
	for i := 1; i < myargc; i += 1 {
		arg_bytes := ([^]u8)(myargv[i])
		if arg_bytes[0] == '@' {
			handle := libc.fopen(cstring(&arg_bytes[1]), "rb")
			if handle == nil {
				libc.printf("\nNo such response file!")
				libc.exit(1)
			}
			libc.printf("Found response file %s!\n", &arg_bytes[1])
			libc.fseek(handle, 0, libc.SEEK_END)
			size := int(libc.ftell(handle))
			libc.fseek(handle, 0, libc.SEEK_SET)
			file := ([^]u8)(libc.malloc(uint(size)))
			libc.fread(file, uint(size), 1, handle)
			libc.fclose(handle)

			moreargs: [20]cstring
			index := 0
			for k := i + 1; k < myargc; k += 1 {
				moreargs[index] = myargv[k]
				index += 1
			}

			firstargv := myargv[0]
			myargv = ([^]cstring)(libc.malloc(size_of(cstring) * MAXARGVS))
			libc.memset(myargv, 0, size_of(cstring) * MAXARGVS)
			myargv[0] = firstargv

			infile := file
			k := 0
			indexinfile := 1
			for {
				myargv[indexinfile] = cstring(&infile[k])
				indexinfile += 1
				for k < size && infile[k] >= ' ' + 1 && infile[k] <= 'z' {
					k += 1
				}
				infile[k] = 0
				for k < size && (infile[k] <= ' ' || infile[k] > 'z') {
					k += 1
				}
				if k >= size {
					break
				}
			}

			for k = 0; k < index; k += 1 {
				myargv[indexinfile] = moreargs[k]
				indexinfile += 1
			}
			myargc = indexinfile

			libc.printf("%d command-line args:\n", i32(myargc))
			for k = 1; k < myargc; k += 1 {
				libc.printf("%s\n", myargv[k])
			}
			break
		}
	}
}

D_DoomMain :: proc() {
	file: [256]u8

	FindResponseFile()
	IdentifyVersion()

	libc.setbuf(libc.stdout, nil)
	modifiedgame = false

	nomonsters = M_CheckParm("-nomonsters") != 0
	respawnparm = M_CheckParm("-respawn") != 0
	fastparm = M_CheckParm("-fast") != 0
	devparm = M_CheckParm("-devparm") != 0
	if M_CheckParm("-altdeath") != 0 {
		deathmatch = 2
	} else if M_CheckParm("-deathmatch") != 0 {
		deathmatch = 1
	}

	switch gamemode {
	case .retail:
		libc.snprintf(&title[0], size_of(title), "                         The Ultimate DOOM Startup v%i.%i                           ", i32(VERSION / 100), i32(VERSION % 100))
	case .shareware:
		libc.snprintf(&title[0], size_of(title), "                            DOOM Shareware Startup v%i.%i                           ", i32(VERSION / 100), i32(VERSION % 100))
	case .registered:
		libc.snprintf(&title[0], size_of(title), "                            DOOM Registered Startup v%i.%i                           ", i32(VERSION / 100), i32(VERSION % 100))
	case .commercial:
		libc.snprintf(&title[0], size_of(title), "                         DOOM 2: Hell on Earth v%i.%i                           ", i32(VERSION / 100), i32(VERSION % 100))
	case .indetermined:
		libc.snprintf(&title[0], size_of(title), "                     Public DOOM - v%i.%i                           ", i32(VERSION / 100), i32(VERSION % 100))
	}

	libc.printf("%s\n", &title[0])

	if devparm {
		libc.printf(D_DEVSTR)
	}

	if M_CheckParm("-cdrom") != 0 {
		libc.printf(D_CDROM)
		c_mkdir("c:\\doomdata")
		libc.strcpy(&basedefault[0], "c:/doomdata/default.cfg")
	}

	if p := M_CheckParm("-turbo"); p != 0 {
		scale := 200
		if p < myargc - 1 {
			scale = int(libc.atoi(myargv[p + 1]))
		}
		if scale < 10 {
			scale = 10
		}
		if scale > 400 {
			scale = 400
		}
		libc.printf("turbo scale: %i%%\n", i32(scale))
		forwardmove[0] = forwardmove[0] * fixed_t(scale) / 100
		forwardmove[1] = forwardmove[1] * fixed_t(scale) / 100
		sidemove[0] = sidemove[0] * fixed_t(scale) / 100
		sidemove[1] = sidemove[1] * fixed_t(scale) / 100
	}

	if p := M_CheckParm("-wart"); p != 0 {
		arg_p := ([^]u8)(myargv[p])
		arg_p[4] = 'p'

		switch gamemode {
		case .shareware, .retail, .registered:
			p1 := ([^]u8)(myargv[p + 1])
			p2 := ([^]u8)(myargv[p + 2])
			libc.snprintf(&file[0], size_of(file), "~" + DEVMAPS + "E%cM%c.wad", i32(p1[0]), i32(p2[0]))
			libc.printf("Warping to Episode %s, Map %s.\n", myargv[p + 1], myargv[p + 2])
		case .commercial, .indetermined:
			m := int(libc.atoi(myargv[p + 1]))
			if m < 10 {
				libc.snprintf(&file[0], size_of(file), "~" + DEVMAPS + "cdata/map0%i.wad", i32(m))
			} else {
				libc.snprintf(&file[0], size_of(file), "~" + DEVMAPS + "cdata/map%i.wad", i32(m))
			}
		}
		D_AddFile(cstring(&file[0]))
	}

	if p := M_CheckParm("-file"); p != 0 {
		modifiedgame = true
		p_idx := p + 1
		for p_idx != myargc && ((^u8)(myargv[p_idx]))^ != '-' {
			D_AddFile(myargv[p_idx])
			p_idx += 1
		}
	}

	p := M_CheckParm("-playdemo")
	if p == 0 {
		p = M_CheckParm("-timedemo")
	}
	if p != 0 && p < myargc - 1 {
		libc.snprintf(&file[0], size_of(file), "%s.lmp", myargv[p + 1])
		D_AddFile(cstring(&file[0]))
		libc.printf("Playing demo %s.lmp.\n", myargv[p + 1])
	}

	startskill = .sk_medium
	startepisode = 1
	startmap = 1
	autostart = false

	p = M_CheckParm("-skill")
	if p != 0 && p < myargc - 1 {
		startskill = skill_t(int(((^u8)(myargv[p + 1]))^) - int('1'))
		autostart = true
	}

	p = M_CheckParm("-episode")
	if p != 0 && p < myargc - 1 {
		startepisode = int(((^u8)(myargv[p + 1]))^) - int('0')
		startmap = 1
		autostart = true
	}

	p = M_CheckParm("-timer")
	if p != 0 && p < myargc - 1 && deathmatch != 0 {
		time := int(libc.atoi(myargv[p + 1]))
		libc.printf("Levels will end after %d minute", i32(time))
		if time > 1 {
			libc.printf("s")
		}
		libc.printf(".\n")
	}

	p = M_CheckParm("-avg")
	if p != 0 && p < myargc - 1 && deathmatch != 0 {
		libc.printf("Austin Virtual Gaming: Levels will end after 20 minutes\n")
	}

	p = M_CheckParm("-warp")
	if p != 0 && p < myargc - 1 {
		if gamemode == .commercial {
			startmap = int(libc.atoi(myargv[p + 1]))
		} else {
			startepisode = int(((^u8)(myargv[p + 1]))^) - int('0')
			startmap = int(((^u8)(myargv[p + 2]))^) - int('0')
		}
		autostart = true
	}

	libc.printf("V_Init: allocate screens.\n")
	V_Init()

	libc.printf("M_LoadDefaults: Load system defaults.\n")
	M_LoadDefaults()

	libc.printf("Z_Init: Init zone memory allocation daemon. \n")
	Z_Init()

	libc.printf("W_Init: Init WADfiles.\n")
	W_InitMultipleFiles(&wadfiles[0])

	if modifiedgame {
		names := [23]cstring{
			"e2m1", "e2m2", "e2m3", "e2m4", "e2m5", "e2m6", "e2m7", "e2m8", "e2m9",
			"e3m1", "e3m3", "e3m3", "e3m4", "e3m5", "e3m6", "e3m7", "e3m8", "e3m9",
			"dphoof", "bfgga0", "heada1", "cybra1", "spida1d1",
		}
		if gamemode == .shareware {
			I_Error("\nYou cannot -file with the shareware version. Register!")
		}
		if gamemode == .registered {
			for i := 0; i < 23; i += 1 {
				if W_CheckNumForName(names[i]) < 0 {
					I_Error("\nThis is not the registered version.")
				}
			}
		}
	}

	if modifiedgame {
		libc.printf(
			"===========================================================================\n" +
			"ATTENTION:  This version of DOOM has been modified.  If you would like to\n" +
			"get a copy of the original game, call 1-800-IDGAMES or see the readme file.\n" +
			"        You will not receive technical support for modified games.\n" +
			"                      press enter to continue\n" +
			"===========================================================================\n",
		)
		libc.getchar()
	}

	switch gamemode {
	case .shareware, .indetermined:
		libc.printf(
			"===========================================================================\n" +
			"                                Shareware!\n" +
			"===========================================================================\n",
		)
	case .registered, .retail, .commercial:
		libc.printf(
			"===========================================================================\n" +
			"                 Commercial product - do not distribute!\n" +
			"         Please report software piracy to the SPA: 1-800-388-PIR8\n" +
			"===========================================================================\n",
		)
	}

	libc.printf("M_Init: Init miscellaneous info.\n")
	M_Init()

	libc.printf("R_Init: Init DOOM refresh daemon - ")
	R_Init()

	libc.printf("\nP_Init: Init Playloop state.\n")
	P_Init()

	libc.printf("I_Init: Setting up machine state.\n")
	I_Init()

	libc.printf("D_CheckNetGame: Checking network game status.\n")
	D_CheckNetGame()

	libc.printf("S_Init: Setting up sound.\n")
	S_Init(snd_SfxVolume, snd_MusicVolume)

	libc.printf("HU_Init: Setting up heads up display.\n")
	HU_Init()

	libc.printf("ST_Init: Init status bar.\n")
	ST_Init()

	p = M_CheckParm("-statcopy")
	if p != 0 && p < myargc - 1 {
		statcopy = rawptr(uintptr(libc.atoi(myargv[p + 1])))
		libc.printf("External statistics registered.\n")
	}

	p = M_CheckParm("-record")
	if p != 0 && p < myargc - 1 {
		G_RecordDemo(myargv[p + 1])
		autostart = true
	}

	p = M_CheckParm("-playdemo")
	if p != 0 && p < myargc - 1 {
		singledemo = true
		G_DeferedPlayDemo(myargv[p + 1])
		D_DoomLoop()
	}

	p = M_CheckParm("-timedemo")
	if p != 0 && p < myargc - 1 {
		G_TimeDemo(myargv[p + 1])
		D_DoomLoop()
	}

	p = M_CheckParm("-loadgame")
	if p != 0 && p < myargc - 1 {
		ch := ((^u8)(myargv[p + 1]))^
		if M_CheckParm("-cdrom") != 0 {
			libc.snprintf(&file[0], size_of(file), "c:\\doomdata\\" + SAVEGAMENAME + "%c.dsg", i32(ch))
		} else {
			libc.snprintf(&file[0], size_of(file), SAVEGAMENAME + "%c.dsg", i32(ch))
		}
		G_LoadGame(cstring(&file[0]))
	}

	if gameaction != .ga_loadgame {
		if autostart || netgame {
			G_InitNew(startskill, startepisode, startmap)
		} else {
			D_StartTitle()
		}
	}

	D_DoomLoop()
}