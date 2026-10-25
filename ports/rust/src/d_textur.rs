// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomtype::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic_t {
    pub width: byte,
    pub height: byte,
    pub data: byte,
}
