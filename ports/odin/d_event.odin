// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

evtype_t :: enum i32 {
	ev_keydown,
	ev_keyup,
	ev_mouse,
	ev_joystick,
}

event_t :: struct {
	type:  evtype_t,
	data1: int,
	data2: int,
	data3: int,
}

gameaction_t :: enum i32 {
	ga_nothing,
	ga_loadlevel,
	ga_newgame,
	ga_loadgame,
	ga_savegame,
	ga_playdemo,
	ga_completed,
	ga_victory,
	ga_worlddone,
	ga_screenshot,
}

BT_ATTACK      :: 1
BT_USE         :: 2
BT_SPECIAL     :: 128
BT_SPECIALMASK :: 3
BT_CHANGE      :: 4
BT_WEAPONMASK  :: 8 + 16 + 32
BT_WEAPONSHIFT :: 3
BTS_PAUSE      :: 1
BTS_SAVEGAME   :: 2
BTS_SAVEMASK   :: 4 + 8 + 16
BTS_SAVESHIFT  :: 2

MAXEVENTS :: 64
