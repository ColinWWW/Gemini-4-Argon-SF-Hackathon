// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::p_pspr::*;

pub type playerstate_t = i32;
pub const PST_LIVE: playerstate_t = 0;
pub const PST_DEAD: playerstate_t = 1;
pub const PST_REBORN: playerstate_t = 2;

pub type cheat_t = i32;
pub const CF_NOCLIP: i32 = 1;
pub const CF_GODMODE: i32 = 2;
pub const CF_NOMOMENTUM: i32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct player_t {
    pub mo: *mut mobj_t,
    pub playerstate: playerstate_t,
    pub cmd: ticcmd_t,
    pub viewz: fixed_t,
    pub viewheight: fixed_t,
    pub deltaviewheight: fixed_t,
    pub bob: fixed_t,
    pub health: i32,
    pub armorpoints: i32,
    pub armortype: i32,
    pub powers: [i32; NUMPOWERS],
    pub cards: [boolean; NUMCARDS],
    pub backpack: boolean,
    pub frags: [i32; MAXPLAYERS],
    pub readyweapon: weapontype_t,
    pub pendingweapon: weapontype_t,
    pub weaponowned: [boolean; NUMWEAPONS],
    pub ammo: [i32; NUMAMMO],
    pub maxammo: [i32; NUMAMMO],
    pub attackdown: i32,
    pub usedown: i32,
    pub cheats: i32,
    pub refire: i32,
    pub killcount: i32,
    pub itemcount: i32,
    pub secretcount: i32,
    pub message: *const u8,
    pub damagecount: i32,
    pub bonuscount: i32,
    pub attacker: *mut mobj_t,
    pub extralight: i32,
    pub fixedcolormap: i32,
    pub colormap: i32,
    pub psprites: [pspdef_t; NUMPSPRITES],
    pub didsecret: boolean,
}

impl Default for player_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct wbplayerstruct_t {
    pub in_: boolean,
    pub skills: i32,
    pub sitems: i32,
    pub ssecret: i32,
    pub stime: i32,
    pub frags: [i32; 4],
    pub score: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct wbstartstruct_t {
    pub epsd: i32,
    pub didsecret: boolean,
    pub last: i32,
    pub next: i32,
    pub maxkills: i32,
    pub maxitems: i32,
    pub maxsecret: i32,
    pub maxfrags: i32,
    pub partime: i32,
    pub pnum: i32,
    pub plyr: [wbplayerstruct_t; MAXPLAYERS],
}
