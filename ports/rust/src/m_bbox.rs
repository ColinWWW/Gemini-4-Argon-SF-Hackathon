// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;
use crate::m_fixed::*;

pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

pub unsafe fn M_ClearBox(box_: *mut fixed_t) {
    *box_.add(BOXTOP) = MININT;
    *box_.add(BOXRIGHT) = MININT;
    *box_.add(BOXBOTTOM) = MAXINT;
    *box_.add(BOXLEFT) = MAXINT;
}

pub unsafe fn M_AddToBox(box_: *mut fixed_t, x: fixed_t, y: fixed_t) {
    if x < *box_.add(BOXLEFT) {
        *box_.add(BOXLEFT) = x;
    } else if x > *box_.add(BOXRIGHT) {
        *box_.add(BOXRIGHT) = x;
    }
    if y < *box_.add(BOXBOTTOM) {
        *box_.add(BOXBOTTOM) = y;
    } else if y > *box_.add(BOXTOP) {
        *box_.add(BOXTOP) = y;
    }
}
