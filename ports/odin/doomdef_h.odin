// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

VERSION :: 110

GameMode_t :: enum i32 {
	shareware,
	registered,
	commercial,
	retail,
	indetermined,
}

GameMission_t :: enum i32 {
	doom,
	doom2,
	pack_tnt,
	pack_plut,
	none,
}

Language_t :: enum i32 {
	english,
	french,
	german,
	unknown,
}

SNDSERV :: 1

BASE_WIDTH       :: 320
SCREEN_MUL       :: 1
INV_ASPECT_RATIO :: 0.625

SCREENWIDTH  :: 320
SCREENHEIGHT :: 200

MAXPLAYERS :: 4
TICRATE    :: 35

gamestate_t :: enum i32 {
	GS_LEVEL,
	GS_INTERMISSION,
	GS_FINALE,
	GS_DEMOSCREEN,
}

MTF_EASY   :: 1
MTF_NORMAL :: 2
MTF_HARD   :: 4
MTF_AMBUSH :: 8

skill_t :: enum i32 {
	sk_baby,
	sk_easy,
	sk_medium,
	sk_hard,
	sk_nightmare,
}

card_t :: enum i32 {
	it_bluecard,
	it_yellowcard,
	it_redcard,
	it_blueskull,
	it_yellowskull,
	it_redskull,
	NUMCARDS,
}
NUMCARDS :: int(card_t.NUMCARDS)

weapontype_t :: enum i32 {
	wp_fist,
	wp_pistol,
	wp_shotgun,
	wp_chaingun,
	wp_missile,
	wp_plasma,
	wp_bfg,
	wp_chainsaw,
	wp_supershotgun,
	NUMWEAPONS,
	wp_nochange,
}
NUMWEAPONS :: int(weapontype_t.NUMWEAPONS)

ammotype_t :: enum i32 {
	am_clip,
	am_shell,
	am_cell,
	am_misl,
	NUMAMMO,
	am_noammo,
}
NUMAMMO :: int(ammotype_t.NUMAMMO)

powertype_t :: enum i32 {
	pw_invulnerability,
	pw_strength,
	pw_invisibility,
	pw_ironfeet,
	pw_allmap,
	pw_infrared,
	NUMPOWERS,
}
pw_invulnerability :: int(powertype_t.pw_invulnerability)
pw_strength        :: int(powertype_t.pw_strength)
pw_invisibility    :: int(powertype_t.pw_invisibility)
pw_ironfeet        :: int(powertype_t.pw_ironfeet)
pw_allmap          :: int(powertype_t.pw_allmap)
pw_infrared        :: int(powertype_t.pw_infrared)
NUMPOWERS          :: int(powertype_t.NUMPOWERS)

INVULNTICS :: 30 * TICRATE
INVISTICS  :: 60 * TICRATE
INFRATICS  :: 120 * TICRATE
IRONTICS   :: 60 * TICRATE

KEY_RIGHTARROW :: 0xae
KEY_LEFTARROW  :: 0xac
KEY_UPARROW    :: 0xad
KEY_DOWNARROW  :: 0xaf
KEY_ESCAPE     :: 27
KEY_ENTER      :: 13
KEY_TAB        :: 9
KEY_F1         :: 0x80 + 0x3b
KEY_F2         :: 0x80 + 0x3c
KEY_F3         :: 0x80 + 0x3d
KEY_F4         :: 0x80 + 0x3e
KEY_F5         :: 0x80 + 0x3f
KEY_F6         :: 0x80 + 0x40
KEY_F7         :: 0x80 + 0x41
KEY_F8         :: 0x80 + 0x42
KEY_F9         :: 0x80 + 0x43
KEY_F10        :: 0x80 + 0x44
KEY_F11        :: 0x80 + 0x57
KEY_F12        :: 0x80 + 0x58

KEY_BACKSPACE  :: 127
KEY_PAUSE      :: 0xff

KEY_EQUALS     :: 0x3d
KEY_MINUS      :: 0x2d

KEY_RSHIFT     :: 0x80 + 0x36
KEY_RCTRL      :: 0x80 + 0x1d
KEY_RALT       :: 0x80 + 0x38
KEY_LALT       :: KEY_RALT
