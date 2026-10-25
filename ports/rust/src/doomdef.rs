// Based on id Software's linuxdoom-1.10 (GNU GPL)

pub const VERSION: i32 = 110;

pub type GameMode_t = i32;
pub const shareware: GameMode_t = 0;
pub const registered: GameMode_t = 1;
pub const commercial: GameMode_t = 2;
pub const retail: GameMode_t = 3;
pub const indetermined: GameMode_t = 4;

pub type GameMission_t = i32;
pub const doom: GameMission_t = 0;
pub const doom2: GameMission_t = 1;
pub const pack_tnt: GameMission_t = 2;
pub const pack_plut: GameMission_t = 3;
pub const none: GameMission_t = 4;

pub type Language_t = i32;
pub const english: Language_t = 0;
pub const french: Language_t = 1;
pub const german: Language_t = 2;
pub const unknown: Language_t = 3;

pub const SNDSERV: i32 = 1;

pub const BASE_WIDTH: i32 = 320;
pub const SCREEN_MUL: i32 = 1;
pub const INV_ASPECT_RATIO: f64 = 0.625;
pub const SCREENWIDTH: i32 = 320;
pub const SCREENHEIGHT: i32 = 200;

pub const MAXPLAYERS: usize = 4;
pub const TICRATE: i32 = 35;

pub type gamestate_t = i32;
pub const GS_LEVEL: gamestate_t = 0;
pub const GS_INTERMISSION: gamestate_t = 1;
pub const GS_FINALE: gamestate_t = 2;
pub const GS_DEMOSCREEN: gamestate_t = 3;

pub const MTF_EASY: i32 = 1;
pub const MTF_NORMAL: i32 = 2;
pub const MTF_HARD: i32 = 4;
pub const MTF_AMBUSH: i32 = 8;

pub type skill_t = i32;
pub const sk_baby: skill_t = 0;
pub const sk_easy: skill_t = 1;
pub const sk_medium: skill_t = 2;
pub const sk_hard: skill_t = 3;
pub const sk_nightmare: skill_t = 4;

pub type card_t = i32;
pub const it_bluecard: card_t = 0;
pub const it_yellowcard: card_t = 1;
pub const it_redcard: card_t = 2;
pub const it_blueskull: card_t = 3;
pub const it_yellowskull: card_t = 4;
pub const it_redskull: card_t = 5;
pub const NUMCARDS: usize = 6;

pub type weapontype_t = i32;
pub const wp_fist: weapontype_t = 0;
pub const wp_pistol: weapontype_t = 1;
pub const wp_shotgun: weapontype_t = 2;
pub const wp_chaingun: weapontype_t = 3;
pub const wp_missile: weapontype_t = 4;
pub const wp_plasma: weapontype_t = 5;
pub const wp_bfg: weapontype_t = 6;
pub const wp_chainsaw: weapontype_t = 7;
pub const wp_supershotgun: weapontype_t = 8;
pub const NUMWEAPONS: usize = 9;
pub const wp_nochange: weapontype_t = 9;

pub type ammotype_t = i32;
pub const am_clip: ammotype_t = 0;
pub const am_shell: ammotype_t = 1;
pub const am_cell: ammotype_t = 2;
pub const am_misl: ammotype_t = 3;
pub const NUMAMMO: usize = 4;
pub const am_noammo: ammotype_t = 5;

pub type powertype_t = i32;
pub const pw_invulnerability: powertype_t = 0;
pub const pw_strength: powertype_t = 1;
pub const pw_invisibility: powertype_t = 2;
pub const pw_ironfeet: powertype_t = 3;
pub const pw_allmap: powertype_t = 4;
pub const pw_infrared: powertype_t = 5;
pub const NUMPOWERS: usize = 6;

pub type powerduration_t = i32;
pub const INVULNTICS: i32 = 30 * TICRATE;
pub const INVISTICS: i32 = 60 * TICRATE;
pub const INFRATICS: i32 = 120 * TICRATE;
pub const IRONTICS: i32 = 60 * TICRATE;

pub const KEY_RIGHTARROW: i32 = 0xae;
pub const KEY_LEFTARROW: i32 = 0xac;
pub const KEY_UPARROW: i32 = 0xad;
pub const KEY_DOWNARROW: i32 = 0xaf;
pub const KEY_ESCAPE: i32 = 27;
pub const KEY_ENTER: i32 = 13;
pub const KEY_TAB: i32 = 9;
pub const KEY_F1: i32 = 0x80 + 0x3b;
pub const KEY_F2: i32 = 0x80 + 0x3c;
pub const KEY_F3: i32 = 0x80 + 0x3d;
pub const KEY_F4: i32 = 0x80 + 0x3e;
pub const KEY_F5: i32 = 0x80 + 0x3f;
pub const KEY_F6: i32 = 0x80 + 0x40;
pub const KEY_F7: i32 = 0x80 + 0x41;
pub const KEY_F8: i32 = 0x80 + 0x42;
pub const KEY_F9: i32 = 0x80 + 0x43;
pub const KEY_F10: i32 = 0x80 + 0x44;
pub const KEY_F11: i32 = 0x80 + 0x57;
pub const KEY_F12: i32 = 0x80 + 0x58;

pub const KEY_BACKSPACE: i32 = 127;
pub const KEY_PAUSE: i32 = 0xff;

pub const KEY_EQUALS: i32 = 0x3d;
pub const KEY_MINUS: i32 = 0x2d;

pub const KEY_RSHIFT: i32 = 0x80 + 0x36;
pub const KEY_RCTRL: i32 = 0x80 + 0x1d;
pub const KEY_RALT: i32 = 0x80 + 0x38;
pub const KEY_LALT: i32 = KEY_RALT;
