// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_think::*;
use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_fixed::*;
use crate::p_mobj::*;
use crate::tables::*;

pub const SIL_NONE: i32 = 0;
pub const SIL_BOTTOM: i32 = 1;
pub const SIL_TOP: i32 = 2;
pub const SIL_BOTH: i32 = 3;

pub const MAXDRAWSEGS: usize = 256;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct vertex_t {
    pub x: fixed_t,
    pub y: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct degenmobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sector_t {
    pub floorheight: fixed_t,
    pub ceilingheight: fixed_t,
    pub floorpic: i16,
    pub ceilingpic: i16,
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
    pub soundtraversed: i32,
    pub soundtarget: *mut mobj_t,
    pub blockbox: [i32; 4],
    pub soundorg: degenmobj_t,
    pub validcount: i32,
    pub thinglist: *mut mobj_t,
    pub specialdata: *mut std::ffi::c_void,
    pub linecount: i32,
    pub lines: *mut *mut line_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct side_t {
    pub textureoffset: fixed_t,
    pub rowoffset: fixed_t,
    pub toptexture: i16,
    pub bottomtexture: i16,
    pub midtexture: i16,
    pub sector: *mut sector_t,
}

pub type slopetype_t = i32;
pub const ST_HORIZONTAL: slopetype_t = 0;
pub const ST_VERTICAL: slopetype_t = 1;
pub const ST_POSITIVE: slopetype_t = 2;
pub const ST_NEGATIVE: slopetype_t = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
    pub bbox: [fixed_t; 4],
    pub slopetype: slopetype_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: i32,
    pub specialdata: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subsector_t {
    pub sector: *mut sector_t,
    pub numlines: i16,
    pub firstline: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub offset: fixed_t,
    pub angle: angle_t,
    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub bbox: [[fixed_t; 4]; 2],
    pub children: [u16; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct post_t {
    pub topdelta: byte,
    pub length: byte,
}

pub type column_t = post_t;
pub type lighttable_t = byte;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drawseg_t {
    pub curline: *mut seg_t,
    pub x1: i32,
    pub x2: i32,
    pub scale1: fixed_t,
    pub scale2: fixed_t,
    pub scalestep: fixed_t,
    pub silhouette: i32,
    pub bsilheight: fixed_t,
    pub tsilheight: fixed_t,
    pub sprtopclip: *mut i16,
    pub sprbottomclip: *mut i16,
    pub maskedtexturecol: *mut i16,
}

impl Default for drawseg_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct patch_t {
    pub width: i16,
    pub height: i16,
    pub leftoffset: i16,
    pub topoffset: i16,
    pub columnofs: [i32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vissprite_t {
    pub prev: *mut vissprite_t,
    pub next: *mut vissprite_t,
    pub x1: i32,
    pub x2: i32,
    pub gx: fixed_t,
    pub gy: fixed_t,
    pub gz: fixed_t,
    pub gzt: fixed_t,
    pub startfrac: fixed_t,
    pub scale: fixed_t,
    pub xiscale: fixed_t,
    pub texturemid: fixed_t,
    pub patch: i32,
    pub colormap: *mut lighttable_t,
    pub mobjflags: i32,
}

impl Default for vissprite_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spriteframe_t {
    pub rotate: boolean,
    pub lump: [i16; 8],
    pub flip: [byte; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spritedef_t {
    pub numframes: i32,
    pub spriteframes: *mut spriteframe_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visplane_t {
    pub height: fixed_t,
    pub picnum: i32,
    pub lightlevel: i32,
    pub minx: i32,
    pub maxx: i32,
    pub pad1: byte,
    pub top: [byte; SCREENWIDTH as usize],
    pub pad2: byte,
    pub pad3: byte,
    pub bottom: [byte; SCREENWIDTH as usize],
    pub pad4: byte,
}

impl Default for visplane_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
