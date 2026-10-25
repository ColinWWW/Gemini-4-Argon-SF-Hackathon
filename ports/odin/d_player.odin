// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

playerstate_t :: enum i32 {
	PST_LIVE,
	PST_DEAD,
	PST_REBORN,
}

CF_NOCLIP     :: 1
CF_GODMODE    :: 2
CF_NOMOMENTUM :: 4

player_t :: struct {
	mo:               ^mobj_t,
	playerstate:      playerstate_t,
	cmd:              ticcmd_t,
	viewz:            fixed_t,
	viewheight:       fixed_t,
	deltaviewheight:  fixed_t,
	bob:              fixed_t,
	health:           int,
	armorpoints:      int,
	armortype:        int,
	powers:           [NUMPOWERS]int,
	cards:            [NUMCARDS]boolean,
	backpack:         boolean,
	frags:            [MAXPLAYERS]int,
	readyweapon:      weapontype_t,
	pendingweapon:    weapontype_t,
	weaponowned:      [NUMWEAPONS]boolean,
	ammo:             [NUMAMMO]int,
	maxammo:          [NUMAMMO]int,
	attackdown:       int,
	usedown:          int,
	cheats:           int,
	refire:           int,
	killcount:        int,
	itemcount:        int,
	secretcount:      int,
	message:          cstring,
	damagecount:      int,
	bonuscount:       int,
	attacker:         ^mobj_t,
	extralight:       int,
	fixedcolormap:    int,
	colormap:         int,
	psprites:         [NUMPSPRITES]pspdef_t,
	didsecret:        boolean,
}

wbplayerstruct_t :: struct {
	in_game: boolean,
	skills:  int,
	sitems:  int,
	ssecret: int,
	stime:   int,
	frags:   [4]int,
	score:   int,
}

wbstartstruct_t :: struct {
	epsd:      int,
	didsecret: boolean,
	last:      int,
	next:      int,
	maxkills:  int,
	maxitems:  int,
	maxsecret: int,
	maxfrags:  int,
	partime:   int,
	pnum:      int,
	plyr:      [MAXPLAYERS]wbplayerstruct_t,
}
