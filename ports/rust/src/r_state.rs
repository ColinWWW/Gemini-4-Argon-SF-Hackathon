// Based on id Software's linuxdoom-1.10 (GNU GPL)

pub use crate::p_setup::{
    lines, nodes, numlines, numnodes, numsectors, numsegs, numsides, numsubsectors, numvertexes,
    sectors, segs, sides, subsectors, vertexes,
};
pub use crate::r_bsp::{curline, ds_p, drawsegs, frontsector, backsector, linedef, sidedef};
pub use crate::r_data::{
    colormaps, firstflat, firstspritelump, flattranslation, lastspritelump, numspritelumps,
    spriteoffset, spritetopoffset, spritewidth, textureheight, texturetranslation,
};
pub use crate::r_draw::{scaledviewwidth, viewheight, viewwidth};
pub use crate::r_main::{
    clipangle, sscount, viewangle, viewangletox, viewplayer, viewx, viewy, viewz, xtoviewangle,
};
pub use crate::r_plane::{ceilingplane, floorplane};
pub use crate::r_segs::{rw_angle1, rw_distance, rw_normalangle};
pub use crate::r_things::{numsprites, sprites};
