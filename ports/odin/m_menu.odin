// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

mouseSensitivity: int
showMessages: int
detailLevel: int
screenblocks: int
screenSize: int
quickSaveSlot: int
messageToPrint: int
messageString: cstring
messx: int
messy: int
messageLastMenuActive: int
messageNeedsInput: boolean
messageRoutine: #type proc(response: int)

gammamsg := [5]cstring{
	GAMMALVL0,
	GAMMALVL1,
	GAMMALVL2,
	GAMMALVL3,
	GAMMALVL4,
}

saveStringEnter: int
saveSlot: int
saveCharIndex: int
saveOldString: [SAVESTRINGSIZE]u8

inhelpscreens: boolean
menuactive: boolean

SKULLXOFF  :: -32
LINEHEIGHT :: 16

savegamestrings: [10][SAVESTRINGSIZE]u8
endstring: [160]u8
m_tempstring: [80]u8

menuitem_t :: struct {
	status:   i16,
	name:     cstring,
	routine:  #type proc(choice: int),
	alphaKey: u8,
}

menu_t :: struct {
	numitems:  i16,
	prevMenu:  ^menu_t,
	menuitems: [^]menuitem_t,
	routine:   #type proc(),
	x:         i16,
	y:         i16,
	lastOn:    i16,
}

itemOn: i16
skullAnimCounter: i16
whichSkull: i16

skullName := [2]cstring{"M_SKULL1", "M_SKULL2"}
currentMenu: ^menu_t

newgame_idx  :: 0
options_idx  :: 1
loadgame_idx :: 2
savegame_idx :: 3
readthis_idx :: 4
quitdoom_idx :: 5
main_end     :: 6

MainMenu := [main_end]menuitem_t{
	{1, "M_NGAME", M_NewGame, 'n'},
	{1, "M_OPTION", M_Options, 'o'},
	{1, "M_LOADG", M_LoadGame, 'l'},
	{1, "M_SAVEG", M_SaveGame, 's'},
	{1, "M_RDTHIS", M_ReadThis, 'r'},
	{1, "M_QUITG", M_QuitDOOM, 'q'},
}

MainDef := menu_t{
	main_end,
	nil,
	&MainMenu[0],
	M_DrawMainMenu,
	97, 64,
	0,
}

ep1    :: 0
ep2    :: 1
ep3    :: 2
ep4    :: 3
ep_end :: 4

EpisodeMenu := [ep_end]menuitem_t{
	{1, "M_EPI1", M_Episode, 'k'},
	{1, "M_EPI2", M_Episode, 't'},
	{1, "M_EPI3", M_Episode, 'i'},
	{1, "M_EPI4", M_Episode, 't'},
}

EpiDef := menu_t{
	ep_end,
	&MainDef,
	&EpisodeMenu[0],
	M_DrawEpisode,
	48, 63,
	ep1,
}

killthings :: 0
toorough   :: 1
hurtme     :: 2
violence   :: 3
nightmare  :: 4
newg_end   :: 5

NewGameMenu := [newg_end]menuitem_t{
	{1, "M_JKILL", M_ChooseSkill, 'i'},
	{1, "M_ROUGH", M_ChooseSkill, 'h'},
	{1, "M_HURT", M_ChooseSkill, 'h'},
	{1, "M_ULTRA", M_ChooseSkill, 'u'},
	{1, "M_NMARE", M_ChooseSkill, 'n'},
}

NewDef := menu_t{
	newg_end,
	&EpiDef,
	&NewGameMenu[0],
	M_DrawNewGame,
	48, 63,
	hurtme,
}

endgame       :: 0
messages      :: 1
detail        :: 2
scrnsize      :: 3
option_empty1 :: 4
mousesens     :: 5
option_empty2 :: 6
soundvol      :: 7
opt_end       :: 8

OptionsMenu := [opt_end]menuitem_t{
	{1, "M_ENDGAM", M_EndGame, 'e'},
	{1, "M_MESSG", M_ChangeMessages, 'm'},
	{1, "M_DETAIL", M_ChangeDetail, 'g'},
	{2, "M_SCRNSZ", M_SizeDisplay, 's'},
	{-1, "", nil, 0},
	{2, "M_MSENS", M_ChangeSensitivity, 'm'},
	{-1, "", nil, 0},
	{1, "M_SVOL", M_Sound, 's'},
}

OptionsDef := menu_t{
	opt_end,
	&MainDef,
	&OptionsMenu[0],
	M_DrawOptions,
	60, 37,
	0,
}

rdthsempty1 :: 0
read1_end   :: 1

ReadMenu1 := [read1_end]menuitem_t{
	{1, "", M_ReadThis2, 0},
}

ReadDef1 := menu_t{
	read1_end,
	&MainDef,
	&ReadMenu1[0],
	M_DrawReadThis1,
	280, 185,
	0,
}

rdthsempty2 :: 0
read2_end   :: 1

ReadMenu2 := [read2_end]menuitem_t{
	{1, "", M_FinishReadThis, 0},
}

ReadDef2 := menu_t{
	read2_end,
	&ReadDef1,
	&ReadMenu2[0],
	M_DrawReadThis2,
	330, 175,
	0,
}

sfx_vol    :: 0
sfx_empty1 :: 1
music_vol  :: 2
sfx_empty2 :: 3
sound_end  :: 4

SoundMenu := [sound_end]menuitem_t{
	{2, "M_SFXVOL", M_SfxVol, 's'},
	{-1, "", nil, 0},
	{2, "M_MUSVOL", M_MusicVol, 'm'},
	{-1, "", nil, 0},
}

SoundDef := menu_t{
	sound_end,
	&OptionsDef,
	&SoundMenu[0],
	M_DrawSound,
	80, 64,
	0,
}

load1    :: 0
load2    :: 1
load3    :: 2
load4    :: 3
load5    :: 4
load6    :: 5
load_end :: 6

LoadMenu := [load_end]menuitem_t{
	{1, "", M_LoadSelect, '1'},
	{1, "", M_LoadSelect, '2'},
	{1, "", M_LoadSelect, '3'},
	{1, "", M_LoadSelect, '4'},
	{1, "", M_LoadSelect, '5'},
	{1, "", M_LoadSelect, '6'},
}

LoadDef := menu_t{
	load_end,
	&MainDef,
	&LoadMenu[0],
	M_DrawLoad,
	80, 54,
	0,
}

SaveMenu := [load_end]menuitem_t{
	{1, "", M_SaveSelect, '1'},
	{1, "", M_SaveSelect, '2'},
	{1, "", M_SaveSelect, '3'},
	{1, "", M_SaveSelect, '4'},
	{1, "", M_SaveSelect, '5'},
	{1, "", M_SaveSelect, '6'},
}

SaveDef := menu_t{
	load_end,
	&MainDef,
	&SaveMenu[0],
	M_DrawSave,
	80, 54,
	0,
}

epi: int
detailNames := [2]cstring{"M_GDHIGH", "M_GDLOW"}
msgNames    := [2]cstring{"M_MSGOFF", "M_MSGON"}

quitsounds := [8]sfxenum_t{
	.sfx_pldeth,
	.sfx_dmpain,
	.sfx_popain,
	.sfx_slop,
	.sfx_telept,
	.sfx_posit1,
	.sfx_posit3,
	.sfx_sgtatk,
}

quitsounds2 := [8]sfxenum_t{
	.sfx_vilact,
	.sfx_getpow,
	.sfx_boscub,
	.sfx_slop,
	.sfx_skeswg,
	.sfx_kntdth,
	.sfx_bspact,
	.sfx_sgtatk,
}

m_joywait: int = 0
m_mousewait: int = 0
m_mousey: int = 0
m_lasty: int = 0
m_mousex: int = 0
m_lastx: int = 0

M_ReadSaveStrings :: proc() {
	name: [256]u8

	for i := 0; i < load_end; i += 1 {
		if M_CheckParm("-cdrom") != 0 {
			libc.snprintf(&name[0], len(name), "c:\\doomdata\\" + SAVEGAMENAME + "%d.dsg", i32(i))
		} else {
			libc.snprintf(&name[0], len(name), SAVEGAMENAME + "%d.dsg", i32(i))
		}

		f := libc.fopen(cstring(&name[0]), "rb")
		if f == nil {
			libc.strcpy(&savegamestrings[i][0], EMPTYSTRING)
			LoadMenu[i].status = 0
			continue
		}
		libc.fread(&savegamestrings[i][0], 1, SAVESTRINGSIZE, f)
		libc.fclose(f)
		LoadMenu[i].status = 1
	}
}

M_DrawLoad :: proc() {
	V_DrawPatchDirect(72, 28, 0, (^patch_t)(W_CacheLumpName("M_LOADG", PU_CACHE)))
	for i := 0; i < load_end; i += 1 {
		M_DrawSaveLoadBorder(int(LoadDef.x), int(LoadDef.y) + LINEHEIGHT * i)
		M_WriteText(int(LoadDef.x), int(LoadDef.y) + LINEHEIGHT * i, cstring(&savegamestrings[i][0]))
	}
}

M_DrawSaveLoadBorder :: proc(x_in: int, y: int) {
	x := x_in
	V_DrawPatchDirect(x - 8, y + 7, 0, (^patch_t)(W_CacheLumpName("M_LSLEFT", PU_CACHE)))

	for i := 0; i < 24; i += 1 {
		V_DrawPatchDirect(x, y + 7, 0, (^patch_t)(W_CacheLumpName("M_LSCNTR", PU_CACHE)))
		x += 8
	}

	V_DrawPatchDirect(x, y + 7, 0, (^patch_t)(W_CacheLumpName("M_LSRGHT", PU_CACHE)))
}

M_LoadSelect :: proc(choice: int) {
	name: [256]u8
	if M_CheckParm("-cdrom") != 0 {
		libc.snprintf(&name[0], len(name), "c:\\doomdata\\" + SAVEGAMENAME + "%d.dsg", i32(choice))
	} else {
		libc.snprintf(&name[0], len(name), SAVEGAMENAME + "%d.dsg", i32(choice))
	}
	G_LoadGame(cstring(&name[0]))
	M_ClearMenus()
}

M_LoadGame :: proc(choice: int) {
	if netgame {
		M_StartMessage(LOADNET, nil, false)
		return
	}

	M_SetupNextMenu(&LoadDef)
	M_ReadSaveStrings()
}

M_DrawSave :: proc() {
	V_DrawPatchDirect(72, 28, 0, (^patch_t)(W_CacheLumpName("M_SAVEG", PU_CACHE)))
	for i := 0; i < load_end; i += 1 {
		M_DrawSaveLoadBorder(int(LoadDef.x), int(LoadDef.y) + LINEHEIGHT * i)
		M_WriteText(int(LoadDef.x), int(LoadDef.y) + LINEHEIGHT * i, cstring(&savegamestrings[i][0]))
	}

	if saveStringEnter != 0 {
		i := M_StringWidth(cstring(&savegamestrings[saveSlot][0]))
		M_WriteText(int(LoadDef.x) + i, int(LoadDef.y) + LINEHEIGHT * saveSlot, "_")
	}
}

M_DoSave :: proc(slot: int) {
	G_SaveGame(slot, cstring(&savegamestrings[slot][0]))
	M_ClearMenus()

	if quickSaveSlot == -2 {
		quickSaveSlot = slot
	}
}

M_SaveSelect :: proc(choice: int) {
	saveStringEnter = 1
	saveSlot = choice
	libc.strcpy(&saveOldString[0], cstring(&savegamestrings[choice][0]))
	if libc.strcmp(cstring(&savegamestrings[choice][0]), EMPTYSTRING) == 0 {
		savegamestrings[choice][0] = 0
	}
	saveCharIndex = int(libc.strlen(cstring(&savegamestrings[choice][0])))
}

M_SaveGame :: proc(choice: int) {
	if !usergame {
		M_StartMessage(SAVEDEAD, nil, false)
		return
	}

	if gamestate != .GS_LEVEL {
		return
	}

	M_SetupNextMenu(&SaveDef)
	M_ReadSaveStrings()
}

M_QuickSaveResponse :: proc(ch: int) {
	if ch == 'y' {
		M_DoSave(quickSaveSlot)
		S_StartSound(nil, int(sfxenum_t.sfx_swtchx))
	}
}

M_QuickSave :: proc() {
	if !usergame {
		S_StartSound(nil, int(sfxenum_t.sfx_oof))
		return
	}

	if gamestate != .GS_LEVEL {
		return
	}

	if quickSaveSlot < 0 {
		M_StartControlPanel()
		M_ReadSaveStrings()
		M_SetupNextMenu(&SaveDef)
		quickSaveSlot = -2
		return
	}
	libc.snprintf(&m_tempstring[0], len(m_tempstring), QSPROMPT, &savegamestrings[quickSaveSlot][0])
	M_StartMessage(cstring(&m_tempstring[0]), M_QuickSaveResponse, true)
}

M_QuickLoadResponse :: proc(ch: int) {
	if ch == 'y' {
		M_LoadSelect(quickSaveSlot)
		S_StartSound(nil, int(sfxenum_t.sfx_swtchx))
	}
}

M_QuickLoad :: proc() {
	if netgame {
		M_StartMessage(QLOADNET, nil, false)
		return
	}

	if quickSaveSlot < 0 {
		M_StartMessage(QSAVESPOT, nil, false)
		return
	}
	libc.snprintf(&m_tempstring[0], len(m_tempstring), QLPROMPT, &savegamestrings[quickSaveSlot][0])
	M_StartMessage(cstring(&m_tempstring[0]), M_QuickLoadResponse, true)
}

M_DrawReadThis1 :: proc() {
	inhelpscreens = true
	switch gamemode {
	case .commercial:
		V_DrawPatchDirect(0, 0, 0, (^patch_t)(W_CacheLumpName("HELP", PU_CACHE)))
	case .shareware, .registered, .retail:
		V_DrawPatchDirect(0, 0, 0, (^patch_t)(W_CacheLumpName("HELP1", PU_CACHE)))
	case .indetermined:
	}
}

M_DrawReadThis2 :: proc() {
	inhelpscreens = true
	switch gamemode {
	case .retail, .commercial:
		V_DrawPatchDirect(0, 0, 0, (^patch_t)(W_CacheLumpName("CREDIT", PU_CACHE)))
	case .shareware, .registered:
		V_DrawPatchDirect(0, 0, 0, (^patch_t)(W_CacheLumpName("HELP2", PU_CACHE)))
	case .indetermined:
	}
}

M_DrawSound :: proc() {
	V_DrawPatchDirect(60, 38, 0, (^patch_t)(W_CacheLumpName("M_SVOL", PU_CACHE)))
	M_DrawThermo(int(SoundDef.x), int(SoundDef.y) + LINEHEIGHT * (sfx_vol + 1), 16, snd_SfxVolume)
	M_DrawThermo(int(SoundDef.x), int(SoundDef.y) + LINEHEIGHT * (music_vol + 1), 16, snd_MusicVolume)
}

M_Sound :: proc(choice: int) {
	M_SetupNextMenu(&SoundDef)
}

M_SfxVol :: proc(choice: int) {
	switch choice {
	case 0:
		if snd_SfxVolume != 0 {
			snd_SfxVolume -= 1
		}
	case 1:
		if snd_SfxVolume < 15 {
			snd_SfxVolume += 1
		}
	}
	S_SetSfxVolume(snd_SfxVolume)
}

M_MusicVol :: proc(choice: int) {
	switch choice {
	case 0:
		if snd_MusicVolume != 0 {
			snd_MusicVolume -= 1
		}
	case 1:
		if snd_MusicVolume < 15 {
			snd_MusicVolume += 1
		}
	}
	S_SetMusicVolume(snd_MusicVolume)
}

M_DrawMainMenu :: proc() {
	V_DrawPatchDirect(94, 2, 0, (^patch_t)(W_CacheLumpName("M_DOOM", PU_CACHE)))
}

M_DrawNewGame :: proc() {
	V_DrawPatchDirect(96, 14, 0, (^patch_t)(W_CacheLumpName("M_NEWG", PU_CACHE)))
	V_DrawPatchDirect(54, 38, 0, (^patch_t)(W_CacheLumpName("M_SKILL", PU_CACHE)))
}

M_NewGame :: proc(choice: int) {
	if netgame && !demoplayback {
		M_StartMessage(NEWGAME, nil, false)
		return
	}

	if gamemode == .commercial {
		M_SetupNextMenu(&NewDef)
	} else {
		M_SetupNextMenu(&EpiDef)
	}
}

M_DrawEpisode :: proc() {
	V_DrawPatchDirect(54, 38, 0, (^patch_t)(W_CacheLumpName("M_EPISOD", PU_CACHE)))
}

M_VerifyNightmare :: proc(ch: int) {
	if ch != 'y' {
		return
	}
	G_DeferedInitNew(skill_t(nightmare), epi + 1, 1)
	M_ClearMenus()
}

M_ChooseSkill :: proc(choice: int) {
	if choice == nightmare {
		M_StartMessage(NIGHTMARE, M_VerifyNightmare, true)
		return
	}
	G_DeferedInitNew(skill_t(choice), epi + 1, 1)
	M_ClearMenus()
}

M_Episode :: proc(choice_in: int) {
	choice := choice_in
	if gamemode == .shareware && choice != 0 {
		M_StartMessage(SWSTRING, nil, false)
		M_SetupNextMenu(&ReadDef1)
		return
	}

	if gamemode == .registered && choice > 2 {
		libc.fprintf(libc.stderr, "M_Episode: 4th episode requires UltimateDOOM\n")
		choice = 0
	}

	epi = choice
	M_SetupNextMenu(&NewDef)
}

M_DrawOptions :: proc() {
	V_DrawPatchDirect(108, 15, 0, (^patch_t)(W_CacheLumpName("M_OPTTTL", PU_CACHE)))
	V_DrawPatchDirect(int(OptionsDef.x) + 175, int(OptionsDef.y) + LINEHEIGHT * detail, 0, (^patch_t)(W_CacheLumpName(detailNames[detailLevel], PU_CACHE)))
	V_DrawPatchDirect(int(OptionsDef.x) + 120, int(OptionsDef.y) + LINEHEIGHT * messages, 0, (^patch_t)(W_CacheLumpName(msgNames[showMessages], PU_CACHE)))
	M_DrawThermo(int(OptionsDef.x), int(OptionsDef.y) + LINEHEIGHT * (mousesens + 1), 10, mouseSensitivity)
	M_DrawThermo(int(OptionsDef.x), int(OptionsDef.y) + LINEHEIGHT * (scrnsize + 1), 9, screenSize)
}

M_Options :: proc(choice: int) {
	M_SetupNextMenu(&OptionsDef)
}

M_ChangeMessages :: proc(choice: int) {
	showMessages = 1 - showMessages
	if showMessages == 0 {
		players[consoleplayer].message = MSGOFF
	} else {
		players[consoleplayer].message = MSGON
	}
	message_dontfuckwithme = true
}

M_EndGameResponse :: proc(ch: int) {
	if ch != 'y' {
		return
	}
	currentMenu.lastOn = itemOn
	M_ClearMenus()
	D_StartTitle()
}

M_EndGame :: proc(choice: int) {
	if !usergame {
		S_StartSound(nil, int(sfxenum_t.sfx_oof))
		return
	}

	if netgame {
		M_StartMessage(NETEND, nil, false)
		return
	}

	M_StartMessage(ENDGAME, M_EndGameResponse, true)
}

M_ReadThis :: proc(choice: int) {
	M_SetupNextMenu(&ReadDef1)
}

M_ReadThis2 :: proc(choice: int) {
	M_SetupNextMenu(&ReadDef2)
}

M_FinishReadThis :: proc(choice: int) {
	M_SetupNextMenu(&MainDef)
}

M_QuitResponse :: proc(ch: int) {
	if ch != 'y' {
		return
	}
	if !netgame {
		if gamemode == .commercial {
			S_StartSound(nil, int(quitsounds2[(gametic >> 2) & 7]))
		} else {
			S_StartSound(nil, int(quitsounds[(gametic >> 2) & 7]))
		}
		I_WaitVBL(105)
	}
	I_Quit()
}

M_QuitDOOM :: proc(choice: int) {
	if language != .english {
		libc.snprintf(&endstring[0], len(endstring), "%s\n\n" + DOSY, endmsg[0])
	} else {
		libc.snprintf(&endstring[0], len(endstring), "%s\n\n" + DOSY, endmsg[(gametic % (NUM_QUITMESSAGES - 2)) + 1])
	}
	M_StartMessage(cstring(&endstring[0]), M_QuitResponse, true)
}

M_ChangeSensitivity :: proc(choice: int) {
	switch choice {
	case 0:
		if mouseSensitivity != 0 {
			mouseSensitivity -= 1
		}
	case 1:
		if mouseSensitivity < 9 {
			mouseSensitivity += 1
		}
	}
}

M_ChangeDetail :: proc(choice: int) {
	detailLevel = 1 - detailLevel
	libc.fprintf(libc.stderr, "M_ChangeDetail: low detail mode n.a.\n")
}

M_SizeDisplay :: proc(choice: int) {
	switch choice {
	case 0:
		if screenSize > 0 {
			screenblocks -= 1
			screenSize -= 1
		}
	case 1:
		if screenSize < 8 {
			screenblocks += 1
			screenSize += 1
		}
	}
	R_SetViewSize(screenblocks, detailLevel)
}

M_DrawThermo :: proc(x: int, y: int, thermWidth: int, thermDot: int) {
	xx := x
	V_DrawPatchDirect(xx, y, 0, (^patch_t)(W_CacheLumpName("M_THERML", PU_CACHE)))
	xx += 8
	for i := 0; i < thermWidth; i += 1 {
		V_DrawPatchDirect(xx, y, 0, (^patch_t)(W_CacheLumpName("M_THERMM", PU_CACHE)))
		xx += 8
	}
	V_DrawPatchDirect(xx, y, 0, (^patch_t)(W_CacheLumpName("M_THERMR", PU_CACHE)))
	V_DrawPatchDirect((x + 8) + thermDot * 8, y, 0, (^patch_t)(W_CacheLumpName("M_THERMO", PU_CACHE)))
}

M_DrawEmptyCell :: proc(menu: ^menu_t, item: int) {
	V_DrawPatchDirect(int(menu.x) - 10, int(menu.y) + item * LINEHEIGHT - 1, 0, (^patch_t)(W_CacheLumpName("M_CELL1", PU_CACHE)))
}

M_DrawSelCell :: proc(menu: ^menu_t, item: int) {
	V_DrawPatchDirect(int(menu.x) - 10, int(menu.y) + item * LINEHEIGHT - 1, 0, (^patch_t)(W_CacheLumpName("M_CELL2", PU_CACHE)))
}

M_StartMessage :: proc(str: cstring, routine: #type proc(response: int), input: boolean) {
	messageLastMenuActive = menuactive ? 1 : 0
	messageToPrint = 1
	messageString = str
	messageRoutine = routine
	messageNeedsInput = input
	menuactive = true
}

M_StopMessage :: proc() {
	menuactive = messageLastMenuActive != 0
	messageToPrint = 0
}

M_StringWidth :: proc(str: cstring) -> int {
	w := 0
	p := ([^]u8)(str)
	length := int(libc.strlen(str))
	for i := 0; i < length; i += 1 {
		c := int(libc.toupper(i32(p[i]))) - int(HU_FONTSTART)
		if c < 0 || c >= HU_FONTSIZE {
			w += 4
		} else {
			w += int(SHORT(hu_font[c].width))
		}
	}
	return w
}

M_StringHeight :: proc(str: cstring) -> int {
	height := int(SHORT(hu_font[0].height))
	h := height
	p := ([^]u8)(str)
	length := int(libc.strlen(str))
	for i := 0; i < length; i += 1 {
		if p[i] == '\n' {
			h += height
		}
	}
	return h
}

M_WriteText :: proc(x: int, y: int, str: cstring) {
	ch := ([^]u8)(str)
	cx := x
	cy := y

	for {
		c := int(ch[0])
		ch = &ch[1]
		if c == 0 {
			break
		}
		if c == '\n' {
			cx = x
			cy += 12
			continue
		}

		c = int(libc.toupper(i32(c))) - int(HU_FONTSTART)
		if c < 0 || c >= HU_FONTSIZE {
			cx += 4
			continue
		}

		w := int(SHORT(hu_font[c].width))
		if cx + w > SCREENWIDTH {
			break
		}
		V_DrawPatchDirect(cx, cy, 0, hu_font[c])
		cx += w
	}
}

M_Responder :: proc(ev: ^event_t) -> boolean {
	ch := -1

	if ev.type == .ev_joystick && m_joywait < I_GetTime() {
		if ev.data3 == -1 {
			ch = KEY_UPARROW
			m_joywait = I_GetTime() + 5
		} else if ev.data3 == 1 {
			ch = KEY_DOWNARROW
			m_joywait = I_GetTime() + 5
		}

		if ev.data2 == -1 {
			ch = KEY_LEFTARROW
			m_joywait = I_GetTime() + 2
		} else if ev.data2 == 1 {
			ch = KEY_RIGHTARROW
			m_joywait = I_GetTime() + 2
		}

		if (ev.data1 & 1) != 0 {
			ch = KEY_ENTER
			m_joywait = I_GetTime() + 5
		}
		if (ev.data1 & 2) != 0 {
			ch = KEY_BACKSPACE
			m_joywait = I_GetTime() + 5
		}
	} else {
		if ev.type == .ev_mouse && m_mousewait < I_GetTime() {
			m_mousey += ev.data3
			if m_mousey < m_lasty - 30 {
				ch = KEY_DOWNARROW
				m_mousewait = I_GetTime() + 5
				m_lasty -= 30
				m_mousey = m_lasty
			} else if m_mousey > m_lasty + 30 {
				ch = KEY_UPARROW
				m_mousewait = I_GetTime() + 5
				m_lasty += 30
				m_mousey = m_lasty
			}

			m_mousex += ev.data2
			if m_mousex < m_lastx - 30 {
				ch = KEY_LEFTARROW
				m_mousewait = I_GetTime() + 5
				m_lastx -= 30
				m_mousex = m_lastx
			} else if m_mousex > m_lastx + 30 {
				ch = KEY_RIGHTARROW
				m_mousewait = I_GetTime() + 5
				m_lastx += 30
				m_mousex = m_lastx
			}

			if (ev.data1 & 1) != 0 {
				ch = KEY_ENTER
				m_mousewait = I_GetTime() + 15
			}
			if (ev.data1 & 2) != 0 {
				ch = KEY_BACKSPACE
				m_mousewait = I_GetTime() + 15
			}
		} else if ev.type == .ev_keydown {
			ch = ev.data1
		}
	}

	if ch == -1 {
		return false
	}

	if saveStringEnter != 0 {
		switch ch {
		case KEY_BACKSPACE:
			if saveCharIndex > 0 {
				saveCharIndex -= 1
				savegamestrings[saveSlot][saveCharIndex] = 0
			}
		case KEY_ESCAPE:
			saveStringEnter = 0
			libc.strcpy(&savegamestrings[saveSlot][0], cstring(&saveOldString[0]))
		case KEY_ENTER:
			saveStringEnter = 0
			if savegamestrings[saveSlot][0] != 0 {
				M_DoSave(saveSlot)
			}
		case:
			ch = int(libc.toupper(i32(ch)))
			if ch != 32 {
				if ch - int(HU_FONTSTART) < 0 || ch - int(HU_FONTSTART) >= HU_FONTSIZE {
					break
				}
			}
			if ch >= 32 && ch <= 127 && saveCharIndex < SAVESTRINGSIZE - 1 && M_StringWidth(cstring(&savegamestrings[saveSlot][0])) < (SAVESTRINGSIZE - 2) * 8 {
				savegamestrings[saveSlot][saveCharIndex] = u8(ch)
				saveCharIndex += 1
				savegamestrings[saveSlot][saveCharIndex] = 0
			}
		}
		return true
	}

	if messageToPrint != 0 {
		if messageNeedsInput && !(ch == ' ' || ch == 'n' || ch == 'y' || ch == KEY_ESCAPE) {
			return false
		}

		menuactive = messageLastMenuActive != 0
		messageToPrint = 0
		if messageRoutine != nil {
			messageRoutine(ch)
		}

		menuactive = false
		S_StartSound(nil, int(sfxenum_t.sfx_swtchx))
		return true
	}

	if devparm && ch == KEY_F1 {
		G_ScreenShot()
		return true
	}

	if !menuactive {
		switch ch {
		case KEY_MINUS:
			if automapactive || chat_on {
				return false
			}
			M_SizeDisplay(0)
			S_StartSound(nil, int(sfxenum_t.sfx_stnmov))
			return true
		case KEY_EQUALS:
			if automapactive || chat_on {
				return false
			}
			M_SizeDisplay(1)
			S_StartSound(nil, int(sfxenum_t.sfx_stnmov))
			return true
		case KEY_F1:
			M_StartControlPanel()
			if gamemode == .retail {
				currentMenu = &ReadDef2
			} else {
				currentMenu = &ReadDef1
			}
			itemOn = 0
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			return true
		case KEY_F2:
			M_StartControlPanel()
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			M_SaveGame(0)
			return true
		case KEY_F3:
			M_StartControlPanel()
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			M_LoadGame(0)
			return true
		case KEY_F4:
			M_StartControlPanel()
			currentMenu = &SoundDef
			itemOn = sfx_vol
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			return true
		case KEY_F5:
			M_ChangeDetail(0)
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			return true
		case KEY_F6:
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			M_QuickSave()
			return true
		case KEY_F7:
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			M_EndGame(0)
			return true
		case KEY_F8:
			M_ChangeMessages(0)
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			return true
		case KEY_F9:
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			M_QuickLoad()
			return true
		case KEY_F10:
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			M_QuitDOOM(0)
			return true
		case KEY_F11:
			usegamma += 1
			if usegamma > 4 {
				usegamma = 0
			}
			players[consoleplayer].message = gammamsg[usegamma]
			I_SetPalette(([^]byte)(W_CacheLumpName("PLAYPAL", PU_CACHE)))
			return true
		}
	}

	if !menuactive {
		if ch == KEY_ESCAPE {
			M_StartControlPanel()
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
			return true
		}
		return false
	}

	switch ch {
	case KEY_DOWNARROW:
		for {
			if itemOn + 1 > currentMenu.numitems - 1 {
				itemOn = 0
			} else {
				itemOn += 1
			}
			S_StartSound(nil, int(sfxenum_t.sfx_pstop))
			if currentMenu.menuitems[itemOn].status != -1 {
				break
			}
		}
		return true
	case KEY_UPARROW:
		for {
			if itemOn == 0 {
				itemOn = currentMenu.numitems - 1
			} else {
				itemOn -= 1
			}
			S_StartSound(nil, int(sfxenum_t.sfx_pstop))
			if currentMenu.menuitems[itemOn].status != -1 {
				break
			}
		}
		return true
	case KEY_LEFTARROW:
		if currentMenu.menuitems[itemOn].routine != nil && currentMenu.menuitems[itemOn].status == 2 {
			S_StartSound(nil, int(sfxenum_t.sfx_stnmov))
			currentMenu.menuitems[itemOn].routine(0)
		}
		return true
	case KEY_RIGHTARROW:
		if currentMenu.menuitems[itemOn].routine != nil && currentMenu.menuitems[itemOn].status == 2 {
			S_StartSound(nil, int(sfxenum_t.sfx_stnmov))
			currentMenu.menuitems[itemOn].routine(1)
		}
		return true
	case KEY_ENTER:
		if currentMenu.menuitems[itemOn].routine != nil && currentMenu.menuitems[itemOn].status != 0 {
			currentMenu.lastOn = itemOn
			if currentMenu.menuitems[itemOn].status == 2 {
				currentMenu.menuitems[itemOn].routine(1)
				S_StartSound(nil, int(sfxenum_t.sfx_stnmov))
			} else {
				currentMenu.menuitems[itemOn].routine(int(itemOn))
				S_StartSound(nil, int(sfxenum_t.sfx_pistol))
			}
		}
		return true
	case KEY_ESCAPE:
		currentMenu.lastOn = itemOn
		M_ClearMenus()
		S_StartSound(nil, int(sfxenum_t.sfx_swtchx))
		return true
	case KEY_BACKSPACE:
		currentMenu.lastOn = itemOn
		if currentMenu.prevMenu != nil {
			currentMenu = currentMenu.prevMenu
			itemOn = currentMenu.lastOn
			S_StartSound(nil, int(sfxenum_t.sfx_swtchn))
		}
		return true
	case:
		for i := int(itemOn) + 1; i < int(currentMenu.numitems); i += 1 {
			if int(currentMenu.menuitems[i].alphaKey) == ch {
				itemOn = i16(i)
				S_StartSound(nil, int(sfxenum_t.sfx_pstop))
				return true
			}
		}
		for i := 0; i <= int(itemOn); i += 1 {
			if int(currentMenu.menuitems[i].alphaKey) == ch {
				itemOn = i16(i)
				S_StartSound(nil, int(sfxenum_t.sfx_pstop))
				return true
			}
		}
	}

	return false
}

M_StartControlPanel :: proc() {
	if menuactive {
		return
	}
	menuactive = true
	currentMenu = &MainDef
	itemOn = currentMenu.lastOn
}

M_Drawer :: proc() {
	str: [40]u8

	inhelpscreens = false

	if messageToPrint != 0 {
		start := 0
		y := 100 - M_StringHeight(messageString) / 2
		msg_bytes := ([^]u8)(messageString)
		for msg_bytes[start] != 0 {
			rem_len := int(libc.strlen(cstring(&msg_bytes[start])))
			i := 0
			for ; i < rem_len; i += 1 {
				if msg_bytes[start + i] == '\n' {
					libc.memset(&str[0], 0, 40)
					libc.strncpy(&str[0], cstring(&msg_bytes[start]), uint(i))
					start += i + 1
					break
				}
			}

			if i == rem_len {
				libc.strcpy(&str[0], cstring(&msg_bytes[start]))
				start += i
			}

			x := 160 - M_StringWidth(cstring(&str[0])) / 2
			M_WriteText(x, y, cstring(&str[0]))
			y += int(SHORT(hu_font[0].height))
		}
		return
	}

	if !menuactive {
		return
	}

	if currentMenu.routine != nil {
		currentMenu.routine()
	}

	x := int(currentMenu.x)
	y := int(currentMenu.y)
	max_items := int(currentMenu.numitems)

	for i := 0; i < max_items; i += 1 {
		if currentMenu.menuitems[i].name != nil && ((^u8)(currentMenu.menuitems[i].name))^ != 0 {
			V_DrawPatchDirect(x, y, 0, (^patch_t)(W_CacheLumpName(currentMenu.menuitems[i].name, PU_CACHE)))
		}
		y += LINEHEIGHT
	}

	V_DrawPatchDirect(x + SKULLXOFF, int(currentMenu.y) - 5 + int(itemOn) * LINEHEIGHT, 0, (^patch_t)(W_CacheLumpName(skullName[whichSkull], PU_CACHE)))
}

M_ClearMenus :: proc() {
	menuactive = false
}

M_SetupNextMenu :: proc(menudef: ^menu_t) {
	currentMenu = menudef
	itemOn = currentMenu.lastOn
}

M_Ticker :: proc() {
	skullAnimCounter -= 1
	if skullAnimCounter <= 0 {
		whichSkull ~= 1
		skullAnimCounter = 8
	}
}

M_Init :: proc() {
	currentMenu = &MainDef
	menuactive = false
	itemOn = currentMenu.lastOn
	whichSkull = 0
	skullAnimCounter = 10
	screenSize = screenblocks - 3
	messageToPrint = 0
	messageString = nil
	messageLastMenuActive = menuactive ? 1 : 0
	quickSaveSlot = -1

	switch gamemode {
	case .commercial:
		MainMenu[readthis_idx] = MainMenu[quitdoom_idx]
		MainDef.numitems -= 1
		MainDef.y += 8
		NewDef.prevMenu = &MainDef
		ReadDef1.routine = M_DrawReadThis1
		ReadDef1.x = 330
		ReadDef1.y = 165
		ReadMenu1[0].routine = M_FinishReadThis
	case .shareware, .registered:
		EpiDef.numitems -= 1
	case .retail, .indetermined:
	}
}