// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;
use crate::i_system::*;

pub const FRACBITS: i32 = 16;
pub const FRACUNIT: i32 = 1 << FRACBITS;

pub type fixed_t = i32;

pub unsafe fn FixedMul(a: fixed_t, b: fixed_t) -> fixed_t {
    (((a as i64) * (b as i64)) >> FRACBITS) as fixed_t
}

pub unsafe fn FixedDiv(a: fixed_t, b: fixed_t) -> fixed_t {
    if (a.wrapping_abs() >> 14) >= b.wrapping_abs() {
        if (a ^ b) < 0 {
            return MININT;
        } else {
            return MAXINT;
        }
    }
    FixedDiv2(a, b)
}

pub unsafe fn FixedDiv2(a: fixed_t, b: fixed_t) -> fixed_t {
    let c: f64 = ((a as f64) / (b as f64)) * (FRACUNIT as f64);
    if c >= 2147483648.0 || c < -2147483648.0 {
        I_Error("FixedDiv: divide by zero");
    }
    c as fixed_t
}
