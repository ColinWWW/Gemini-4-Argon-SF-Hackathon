// Based on id Software's linuxdoom-1.10 (GNU GPL)

pub use crate::m_fixed::*;
pub use crate::p_ceilng::*;
pub use crate::p_doors::*;
pub use crate::p_enemy::*;
pub use crate::p_floor::*;
pub use crate::p_inter::*;
pub use crate::p_lights::*;
pub use crate::p_map::*;
pub use crate::p_maputl::*;
pub use crate::p_mobj::*;
pub use crate::p_plats::*;
pub use crate::p_pspr::*;
pub use crate::p_setup::*;
pub use crate::p_sight::*;
pub use crate::p_spec::*;
pub use crate::p_switch::*;
pub use crate::p_telept::*;
pub use crate::p_tick::*;
pub use crate::p_user::*;
pub use crate::r_local::*;

pub const FLOATSPEED: fixed_t = FRACUNIT * 4;
pub const MAXHEALTH: i32 = 100;
pub const VIEWHEIGHT: fixed_t = 41 * FRACUNIT;

pub const MAPBLOCKUNITS: i32 = 128;
pub const MAPBLOCKSIZE: fixed_t = MAPBLOCKUNITS * FRACUNIT;
pub const MAPBLOCKSHIFT: i32 = FRACBITS + 7;
pub const MAPBMASK: fixed_t = MAPBLOCKSIZE - 1;
pub const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;

pub const PLAYERRADIUS: fixed_t = 16 * FRACUNIT;
pub const MAXRADIUS: fixed_t = 32 * FRACUNIT;
pub const GRAVITY: fixed_t = FRACUNIT;
pub const MAXMOVE: fixed_t = 30 * FRACUNIT;

pub const USERANGE: fixed_t = 64 * FRACUNIT;
pub const MELEERANGE: fixed_t = 64 * FRACUNIT;
pub const MISSILERANGE: fixed_t = 32 * 64 * FRACUNIT;

pub const BASETHRESHOLD: i32 = 100;

pub const ONFLOORZ: fixed_t = crate::doomtype::MININT;
pub const ONCEILINGZ: fixed_t = crate::doomtype::MAXINT;

pub const ITEMQUESIZE: usize = 128;
pub const MAXINTERCEPTS: usize = 128;

pub const PT_ADDLINES: i32 = 1;
pub const PT_ADDTHINGS: i32 = 2;
pub const PT_EARLYOUT: i32 = 4;
