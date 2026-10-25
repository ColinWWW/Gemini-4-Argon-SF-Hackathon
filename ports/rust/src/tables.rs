// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::m_fixed::*;
pub use crate::tables_data::*;

pub const PI: f64 = 3.141592657;

pub const FINEANGLES: usize = 8192;
pub const FINEMASK: usize = FINEANGLES - 1;
pub const ANGLETOFINESHIFT: u32 = 19;

pub const ANG45: u32 = 0x20000000;
pub const ANG90: u32 = 0x40000000;
pub const ANG180: u32 = 0x80000000;
pub const ANG270: u32 = 0xc0000000;

pub const SLOPERANGE: usize = 2048;
pub const SLOPEBITS: u32 = 11;
pub const DBITS: u32 = (FRACBITS as u32) - SLOPEBITS;

pub type angle_t = u32;

#[inline]
pub unsafe fn finecosine(idx: usize) -> fixed_t {
    finesine[idx + FINEANGLES / 4]
}
