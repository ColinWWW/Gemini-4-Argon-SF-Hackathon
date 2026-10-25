// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_bbox::*;
use crate::m_fixed::*;
use crate::m_swap::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_spec::*;
use crate::p_switch::*;
use crate::p_tick::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_things::*;
use crate::s_sound::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub static mut numvertexes: i32 = 0;
pub static mut vertexes: *mut vertex_t = std::ptr::null_mut();

pub static mut numsegs: i32 = 0;
pub static mut segs: *mut seg_t = std::ptr::null_mut();

pub static mut numsectors: i32 = 0;
pub static mut sectors: *mut sector_t = std::ptr::null_mut();

pub static mut numsubsectors: i32 = 0;
pub static mut subsectors: *mut subsector_t = std::ptr::null_mut();

pub static mut numnodes: i32 = 0;
pub static mut nodes: *mut node_t = std::ptr::null_mut();

pub static mut numlines: i32 = 0;
pub static mut lines: *mut line_t = std::ptr::null_mut();

pub static mut numsides: i32 = 0;
pub static mut sides: *mut side_t = std::ptr::null_mut();

pub static mut bmapwidth: i32 = 0;
pub static mut bmapheight: i32 = 0;
pub static mut blockmap: *mut i16 = std::ptr::null_mut();
pub static mut blockmaplump: *mut i16 = std::ptr::null_mut();
pub static mut bmaporgx: fixed_t = 0;
pub static mut bmaporgy: fixed_t = 0;
pub static mut blocklinks: *mut *mut mobj_t = std::ptr::null_mut();

pub static mut rejectmatrix: *mut byte = std::ptr::null_mut();

pub const MAX_DEATHMATCH_STARTS: usize = 10;
pub static mut deathmatchstarts: [mapthing_t; MAX_DEATHMATCH_STARTS] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    type_: 0,
    options: 0,
}; MAX_DEATHMATCH_STARTS];
pub static mut deathmatch_p: *mut mapthing_t = std::ptr::null_mut();
pub static mut playerstarts: [mapthing_t; MAXPLAYERS] = [mapthing_t {
    x: 0,
    y: 0,
    angle: 0,
    type_: 0,
    options: 0,
}; MAXPLAYERS];

pub unsafe fn P_LoadVertexes(lump: i32) {
    numvertexes = W_LumpLength(lump) / (std::mem::size_of::<mapvertex_t>() as i32);
    vertexes = Z_Malloc(
        numvertexes * (std::mem::size_of::<vertex_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut vertex_t;

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let ml = data as *const mapvertex_t;
    let li = vertexes;

    for i in 0..(numvertexes as usize) {
        let m = *ml.add(i);
        (*li.add(i)).x = (SHORT(m.x) as fixed_t) << FRACBITS;
        (*li.add(i)).y = (SHORT(m.y) as fixed_t) << FRACBITS;
    }

    Z_Free(data);
}

pub unsafe fn P_LoadSegs(lump: i32) {
    numsegs = W_LumpLength(lump) / (std::mem::size_of::<mapseg_t>() as i32);
    segs = Z_Malloc(
        numsegs * (std::mem::size_of::<seg_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut seg_t;
    std::ptr::write_bytes(segs, 0, numsegs as usize);

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let ml = data as *const mapseg_t;
    let li = segs;

    for i in 0..(numsegs as usize) {
        let m = *ml.add(i);
        let seg = li.add(i);
        (*seg).v1 = vertexes.add(SHORT(m.v1) as usize);
        (*seg).v2 = vertexes.add(SHORT(m.v2) as usize);
        (*seg).angle = ((SHORT(m.angle) as i32) << 16) as u32;
        (*seg).offset = (SHORT(m.offset) as fixed_t) << 16;
        let linedef_idx = SHORT(m.linedef) as usize;
        let ldef = lines.add(linedef_idx);
        (*seg).linedef = ldef;
        let side = SHORT(m.side) as usize;
        (*seg).sidedef = sides.add((*ldef).sidenum[side] as usize);
        (*seg).frontsector = (*sides.add((*ldef).sidenum[side] as usize)).sector;
        if ((*ldef).flags as i32) & ML_TWOSIDED != 0 {
            (*seg).backsector = (*sides.add((*ldef).sidenum[side ^ 1] as usize)).sector;
        } else {
            (*seg).backsector = std::ptr::null_mut();
        }
    }

    Z_Free(data);
}

pub unsafe fn P_LoadSubsectors(lump: i32) {
    numsubsectors = W_LumpLength(lump) / (std::mem::size_of::<mapsubsector_t>() as i32);
    subsectors = Z_Malloc(
        numsubsectors * (std::mem::size_of::<subsector_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut subsector_t;
    std::ptr::write_bytes(subsectors, 0, numsubsectors as usize);

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let ms = data as *const mapsubsector_t;
    let ss = subsectors;

    for i in 0..(numsubsectors as usize) {
        let m = *ms.add(i);
        (*ss.add(i)).numlines = SHORT(m.numsegs);
        (*ss.add(i)).firstline = SHORT(m.firstseg);
    }

    Z_Free(data);
}

pub unsafe fn P_LoadSectors(lump: i32) {
    numsectors = W_LumpLength(lump) / (std::mem::size_of::<mapsector_t>() as i32);
    sectors = Z_Malloc(
        numsectors * (std::mem::size_of::<sector_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut sector_t;
    std::ptr::write_bytes(sectors, 0, numsectors as usize);

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let ms = data as *const mapsector_t;
    let ss = sectors;

    for i in 0..(numsectors as usize) {
        let m = *ms.add(i);
        let sec = ss.add(i);
        (*sec).floorheight = (SHORT(m.floorheight) as fixed_t) << FRACBITS;
        (*sec).ceilingheight = (SHORT(m.ceilingheight) as fixed_t) << FRACBITS;
        (*sec).floorpic = R_FlatNumForName(m.floorpic.as_ptr()) as i16;
        (*sec).ceilingpic = R_FlatNumForName(m.ceilingpic.as_ptr()) as i16;
        (*sec).lightlevel = SHORT(m.lightlevel);
        (*sec).special = SHORT(m.special);
        (*sec).tag = SHORT(m.tag);
        (*sec).thinglist = std::ptr::null_mut();
    }

    Z_Free(data);
}

pub unsafe fn P_LoadNodes(lump: i32) {
    numnodes = W_LumpLength(lump) / (std::mem::size_of::<mapnode_t>() as i32);
    nodes = Z_Malloc(
        numnodes * (std::mem::size_of::<node_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut node_t;

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let mn = data as *const mapnode_t;
    let no = nodes;

    for i in 0..(numnodes as usize) {
        let m = *mn.add(i);
        let n = no.add(i);
        (*n).x = (SHORT(m.x) as fixed_t) << FRACBITS;
        (*n).y = (SHORT(m.y) as fixed_t) << FRACBITS;
        (*n).dx = (SHORT(m.dx) as fixed_t) << FRACBITS;
        (*n).dy = (SHORT(m.dy) as fixed_t) << FRACBITS;
        for j in 0..2 {
            (*n).children[j] = USHORT(m.children[j]);
            for k in 0..4 {
                (*n).bbox[j][k] = (SHORT(m.bbox[j][k]) as fixed_t) << FRACBITS;
            }
        }
    }

    Z_Free(data);
}

pub unsafe fn P_LoadThings(lump: i32) {
    let data = W_CacheLumpNum(lump, PU_STATIC);
    let numthings = W_LumpLength(lump) / (std::mem::size_of::<mapthing_t>() as i32);

    let mt_ptr = data as *mut mapthing_t;
    for i in 0..(numthings as usize) {
        let mt = mt_ptr.add(i);
        let mut spawn = true;

        if gamemode != commercial {
            match SHORT((*mt).type_) {
                68 | 64 | 88 | 89 | 69 | 67 | 71 | 65 | 66 | 84 => {
                    spawn = false;
                }
                _ => {}
            }
        }
        if !spawn {
            break;
        }

        (*mt).x = SHORT((*mt).x);
        (*mt).y = SHORT((*mt).y);
        (*mt).angle = SHORT((*mt).angle);
        (*mt).type_ = SHORT((*mt).type_);
        (*mt).options = SHORT((*mt).options);

        P_SpawnMapThing(mt);
    }

    Z_Free(data);
}

pub unsafe fn P_LoadLineDefs(lump: i32) {
    numlines = W_LumpLength(lump) / (std::mem::size_of::<maplinedef_t>() as i32);
    lines = Z_Malloc(
        numlines * (std::mem::size_of::<line_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut line_t;
    std::ptr::write_bytes(lines, 0, numlines as usize);

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let mld = data as *const maplinedef_t;
    let ld_base = lines;

    for i in 0..(numlines as usize) {
        let m = *mld.add(i);
        let ld = ld_base.add(i);
        (*ld).flags = SHORT(m.flags);
        (*ld).special = SHORT(m.special);
        (*ld).tag = SHORT(m.tag);
        let v1 = vertexes.add(SHORT(m.v1) as usize);
        let v2 = vertexes.add(SHORT(m.v2) as usize);
        (*ld).v1 = v1;
        (*ld).v2 = v2;
        (*ld).dx = (*v2).x - (*v1).x;
        (*ld).dy = (*v2).y - (*v1).y;

        if (*ld).dx == 0 {
            (*ld).slopetype = ST_VERTICAL;
        } else if (*ld).dy == 0 {
            (*ld).slopetype = ST_HORIZONTAL;
        } else {
            if FixedDiv((*ld).dy, (*ld).dx) > 0 {
                (*ld).slopetype = ST_POSITIVE;
            } else {
                (*ld).slopetype = ST_NEGATIVE;
            }
        }

        if (*v1).x < (*v2).x {
            (*ld).bbox[BOXLEFT] = (*v1).x;
            (*ld).bbox[BOXRIGHT] = (*v2).x;
        } else {
            (*ld).bbox[BOXLEFT] = (*v2).x;
            (*ld).bbox[BOXRIGHT] = (*v1).x;
        }

        if (*v1).y < (*v2).y {
            (*ld).bbox[BOXBOTTOM] = (*v1).y;
            (*ld).bbox[BOXTOP] = (*v2).y;
        } else {
            (*ld).bbox[BOXBOTTOM] = (*v2).y;
            (*ld).bbox[BOXTOP] = (*v1).y;
        }

        (*ld).sidenum[0] = SHORT(m.sidenum[0]);
        (*ld).sidenum[1] = SHORT(m.sidenum[1]);

        if (*ld).sidenum[0] != -1 {
            (*ld).frontsector = (*sides.add((*ld).sidenum[0] as usize)).sector;
        } else {
            (*ld).frontsector = std::ptr::null_mut();
        }

        if (*ld).sidenum[1] != -1 {
            (*ld).backsector = (*sides.add((*ld).sidenum[1] as usize)).sector;
        } else {
            (*ld).backsector = std::ptr::null_mut();
        }
    }

    Z_Free(data);
}

pub unsafe fn P_LoadSideDefs(lump: i32) {
    numsides = W_LumpLength(lump) / (std::mem::size_of::<mapsidedef_t>() as i32);
    sides = Z_Malloc(
        numsides * (std::mem::size_of::<side_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut side_t;
    std::ptr::write_bytes(sides, 0, numsides as usize);

    let data = W_CacheLumpNum(lump, PU_STATIC);
    let msd = data as *const mapsidedef_t;
    let sd_base = sides;

    for i in 0..(numsides as usize) {
        let m = *msd.add(i);
        let sd = sd_base.add(i);
        (*sd).textureoffset = (SHORT(m.textureoffset) as fixed_t) << FRACBITS;
        (*sd).rowoffset = (SHORT(m.rowoffset) as fixed_t) << FRACBITS;
        (*sd).toptexture = R_TextureNumForName(m.toptexture.as_ptr()) as i16;
        (*sd).bottomtexture = R_TextureNumForName(m.bottomtexture.as_ptr()) as i16;
        (*sd).midtexture = R_TextureNumForName(m.midtexture.as_ptr()) as i16;
        (*sd).sector = sectors.add(SHORT(m.sector) as usize);
    }

    Z_Free(data);
}

pub unsafe fn P_LoadBlockMap(lump: i32) {
    blockmaplump = W_CacheLumpNum(lump, PU_LEVEL) as *mut i16;
    blockmap = blockmaplump.add(4);
    let count = W_LumpLength(lump) / 2;

    for i in 0..(count as usize) {
        *blockmaplump.add(i) = SHORT(*blockmaplump.add(i));
    }

    bmaporgx = (*blockmaplump.add(0) as fixed_t) << FRACBITS;
    bmaporgy = (*blockmaplump.add(1) as fixed_t) << FRACBITS;
    bmapwidth = *blockmaplump.add(2) as i32;
    bmapheight = *blockmaplump.add(3) as i32;

    let links_size = (std::mem::size_of::<*mut mobj_t>() as i32) * bmapwidth * bmapheight;
    blocklinks = Z_Malloc(links_size, PU_LEVEL, std::ptr::null_mut()) as *mut *mut mobj_t;
    std::ptr::write_bytes(blocklinks, 0, (bmapwidth * bmapheight) as usize);
}

pub unsafe fn P_GroupLines() {
    for i in 0..(numsubsectors as usize) {
        let ss = subsectors.add(i);
        let seg = segs.add((*ss).firstline as usize);
        (*ss).sector = (*(*seg).sidedef).sector;
    }

    let mut total = 0i32;
    for i in 0..(numlines as usize) {
        let li = lines.add(i);
        total += 1;
        (*(*li).frontsector).linecount += 1;

        if !(*li).backsector.is_null() && (*li).backsector != (*li).frontsector {
            (*(*li).backsector).linecount += 1;
            total += 1;
        }
    }

    let mut linebuffer = Z_Malloc(
        total * (std::mem::size_of::<*mut line_t>() as i32),
        PU_LEVEL,
        std::ptr::null_mut(),
    ) as *mut *mut line_t;

    for i in 0..(numsectors as usize) {
        let sector = sectors.add(i);
        let mut bbox = [0; 4];
        M_ClearBox(bbox.as_mut_ptr());
        (*sector).lines = linebuffer;

        for j in 0..(numlines as usize) {
            let li = lines.add(j);
            if (*li).frontsector == sector || (*li).backsector == sector {
                *linebuffer = li;
                linebuffer = linebuffer.add(1);
                M_AddToBox(bbox.as_mut_ptr(), (*(*li).v1).x, (*(*li).v1).y);
                M_AddToBox(bbox.as_mut_ptr(), (*(*li).v2).x, (*(*li).v2).y);
            }
        }

        let counted = ((linebuffer as usize - (*sector).lines as usize)
            / std::mem::size_of::<*mut line_t>()) as i32;
        if counted != (*sector).linecount {
            I_Error("P_GroupLines: miscounted");
        }

        (*sector).soundorg.x = (bbox[BOXRIGHT] + bbox[BOXLEFT]) / 2;
        (*sector).soundorg.y = (bbox[BOXTOP] + bbox[BOXBOTTOM]) / 2;

        let mut block = (bbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
        if block >= bmapheight {
            block = bmapheight - 1;
        }
        (*sector).blockbox[BOXTOP] = block;

        block = (bbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
        if block < 0 {
            block = 0;
        }
        (*sector).blockbox[BOXBOTTOM] = block;

        block = (bbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
        if block >= bmapwidth {
            block = bmapwidth - 1;
        }
        (*sector).blockbox[BOXRIGHT] = block;

        block = (bbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
        if block < 0 {
            block = 0;
        }
        (*sector).blockbox[BOXLEFT] = block;
    }
}

pub unsafe fn P_SetupLevel(episode: i32, map: i32, _playermask: i32, _skill: skill_t) {
    totalkills = 0;
    totalitems = 0;
    totalsecret = 0;
    wminfo.maxfrags = 0;
    wminfo.partime = 180;

    for i in 0..MAXPLAYERS {
        players[i].killcount = 0;
        players[i].secretcount = 0;
        players[i].itemcount = 0;
    }

    players[consoleplayer as usize].viewz = 1;

    S_Start();

    Z_FreeTags(PU_LEVEL, PU_PURGELEVEL - 1);

    P_InitThinkers();

    W_Reload();

    let lumpname = if gamemode == commercial {
        if map < 10 {
            format!("map0{}", map)
        } else {
            format!("map{}", map)
        }
    } else {
        format!("E{}M{}", episode, map)
    };

    let lumpnum = W_GetNumForNameStr(&lumpname);

    leveltime = 0;

    P_LoadBlockMap(lumpnum + ML_BLOCKMAP);
    P_LoadVertexes(lumpnum + ML_VERTEXES);
    P_LoadSectors(lumpnum + ML_SECTORS);
    P_LoadSideDefs(lumpnum + ML_SIDEDEFS);

    P_LoadLineDefs(lumpnum + ML_LINEDEFS);
    P_LoadSubsectors(lumpnum + ML_SSECTORS);
    P_LoadNodes(lumpnum + ML_NODES);
    P_LoadSegs(lumpnum + ML_SEGS);

    rejectmatrix = W_CacheLumpNum(lumpnum + ML_REJECT, PU_LEVEL) as *mut byte;
    P_GroupLines();

    bodyqueslot = 0;
    deathmatch_p = deathmatchstarts.as_mut_ptr();
    P_LoadThings(lumpnum + ML_THINGS);

    if deathmatch != 0 {
        for i in 0..MAXPLAYERS {
            if playeringame[i] != 0 {
                players[i].mo = std::ptr::null_mut();
                G_DeathMatchSpawnPlayer(i as i32);
            }
        }
    }

    iquehead = 0;
    iquetail = 0;

    P_SpawnSpecials();

    if precache != 0 {
        R_PrecacheLevel();
    }
}

pub unsafe fn P_Init() {
    P_InitSwitchList();
    P_InitPicAnims();
    R_InitSprites(&sprnames);
}
