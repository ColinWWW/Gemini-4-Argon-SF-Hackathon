// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::m_fixed::*;

pub const SKYFLATNAME: &str = "F_SKY1";
pub const ANGLETOSKYSHIFT: u32 = 22;

pub static mut skyflatnum: i32 = 0;
pub static mut skytexture: i32 = 0;
pub static mut skytexturemid: i32 = 0;

pub unsafe fn R_InitSkyMap() {
    skytexturemid = 100 * FRACUNIT;
}
