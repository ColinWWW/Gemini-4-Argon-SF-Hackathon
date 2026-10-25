// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ticcmd_t {
    pub forwardmove: i8,
    pub sidemove: i8,
    pub angleturn: i16,
    pub consistancy: i16,
    pub chatchar: byte,
    pub buttons: byte,
}
