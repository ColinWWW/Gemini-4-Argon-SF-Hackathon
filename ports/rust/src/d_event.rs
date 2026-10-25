// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_main::{eventhead, events, eventtail};
use crate::g_game::gameaction;

pub type evtype_t = i32;
pub const ev_keydown: evtype_t = 0;
pub const ev_keyup: evtype_t = 1;
pub const ev_mouse: evtype_t = 2;
pub const ev_joystick: evtype_t = 3;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct event_t {
    pub type_: evtype_t,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
}

pub type gameaction_t = i32;
pub const ga_nothing: gameaction_t = 0;
pub const ga_loadlevel: gameaction_t = 1;
pub const ga_newgame: gameaction_t = 2;
pub const ga_loadgame: gameaction_t = 3;
pub const ga_savegame: gameaction_t = 4;
pub const ga_playdemo: gameaction_t = 5;
pub const ga_completed: gameaction_t = 6;
pub const ga_victory: gameaction_t = 7;
pub const ga_worlddone: gameaction_t = 8;
pub const ga_screenshot: gameaction_t = 9;

pub type buttoncode_t = u8;
pub const BT_ATTACK: u8 = 1;
pub const BT_USE: u8 = 2;
pub const BT_SPECIAL: u8 = 128;
pub const BT_SPECIALMASK: u8 = 3;
pub const BT_CHANGE: u8 = 4;
pub const BT_WEAPONMASK: u8 = 8 + 16 + 32;
pub const BT_WEAPONSHIFT: u8 = 3;
pub const BTS_PAUSE: u8 = 1;
pub const BTS_SAVEGAME: u8 = 2;
pub const BTS_SAVEMASK: u8 = 4 + 8 + 16;
pub const BTS_SAVESHIFT: u8 = 2;

pub const MAXEVENTS: usize = 64;
