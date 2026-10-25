// Based on id Software's linuxdoom-1.10 (GNU GPL)

pub const ML_LABEL: i32 = 0;
pub const ML_THINGS: i32 = 1;
pub const ML_LINEDEFS: i32 = 2;
pub const ML_SIDEDEFS: i32 = 3;
pub const ML_VERTEXES: i32 = 4;
pub const ML_SEGS: i32 = 5;
pub const ML_SSECTORS: i32 = 6;
pub const ML_NODES: i32 = 7;
pub const ML_SECTORS: i32 = 8;
pub const ML_REJECT: i32 = 9;
pub const ML_BLOCKMAP: i32 = 10;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mapvertex_t {
    pub x: i16,
    pub y: i16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mapsidedef_t {
    pub textureoffset: i16,
    pub rowoffset: i16,
    pub toptexture: [u8; 8],
    pub bottomtexture: [u8; 8],
    pub midtexture: [u8; 8],
    pub sector: i16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct maplinedef_t {
    pub v1: i16,
    pub v2: i16,
    pub flags: i16,
    pub special: i16,
    pub tag: i16,
    pub sidenum: [i16; 2],
}

pub const ML_BLOCKING: i32 = 1;
pub const ML_BLOCKMONSTERS: i32 = 2;
pub const ML_TWOSIDED: i32 = 4;
pub const ML_DONTPEGTOP: i32 = 8;
pub const ML_DONTPEGBOTTOM: i32 = 16;
pub const ML_SECRET: i32 = 32;
pub const ML_SOUNDBLOCK: i32 = 64;
pub const ML_DONTDRAW: i32 = 128;
pub const ML_MAPPED: i32 = 256;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mapsector_t {
    pub floorheight: i16,
    pub ceilingheight: i16,
    pub floorpic: [u8; 8],
    pub ceilingpic: [u8; 8],
    pub lightlevel: i16,
    pub special: i16,
    pub tag: i16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mapsubsector_t {
    pub numsegs: i16,
    pub firstseg: i16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mapseg_t {
    pub v1: i16,
    pub v2: i16,
    pub angle: i16,
    pub linedef: i16,
    pub side: i16,
    pub offset: i16,
}

pub const NF_SUBSECTOR: i32 = 0x8000;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct mapnode_t {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox: [[i16; 4]; 2],
    pub children: [u16; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct mapthing_t {
    pub x: i16,
    pub y: i16,
    pub angle: i16,
    pub type_: i16,
    pub options: i16,
}
