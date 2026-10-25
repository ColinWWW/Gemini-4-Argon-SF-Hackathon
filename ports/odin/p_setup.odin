// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

numvertexes: int
vertexes: [^]vertex_t

numsegs: int
segs: [^]seg_t

numsectors: int
sectors: [^]sector_t

numsubsectors: int
subsectors: [^]subsector_t

numnodes: int
nodes: [^]node_t

numlines: int
lines: [^]line_t

numsides: int
sides: [^]side_t

bmapwidth: int
bmapheight: int
blockmap: [^]i16
blockmaplump: [^]i16
bmaporgx: fixed_t
bmaporgy: fixed_t
blocklinks: [^]^mobj_t

rejectmatrix: [^]byte

MAX_DEATHMATCH_STARTS :: 10
deathmatchstarts: [MAX_DEATHMATCH_STARTS]mapthing_t
deathmatch_p: [^]mapthing_t
playerstarts: [MAXPLAYERS]mapthing_t

P_LoadVertexes :: proc(lump: int) {
	numvertexes = W_LumpLength(lump) / size_of(mapvertex_t)
	vertexes = ([^]vertex_t)(Z_Malloc(numvertexes * size_of(vertex_t), PU_LEVEL, nil))
	data := W_CacheLumpNum(lump, PU_STATIC)

	ml := ([^]mapvertex_t)(data)
	for i := 0; i < numvertexes; i += 1 {
		vertexes[i].x = fixed_t(i32(SHORT(ml[i].x)) << FRACBITS)
		vertexes[i].y = fixed_t(i32(SHORT(ml[i].y)) << FRACBITS)
	}

	Z_Free(data)
}

P_LoadSegs :: proc(lump: int) {
	numsegs = W_LumpLength(lump) / size_of(mapseg_t)
	segs = ([^]seg_t)(Z_Malloc(numsegs * size_of(seg_t), PU_LEVEL, nil))
	libc.memset(segs, 0, uint(numsegs * size_of(seg_t)))
	data := W_CacheLumpNum(lump, PU_STATIC)

	ml := ([^]mapseg_t)(data)
	for i := 0; i < numsegs; i += 1 {
		li := &segs[i]
		li.v1 = &vertexes[u16(SHORT(ml[i].v1))]
		li.v2 = &vertexes[u16(SHORT(ml[i].v2))]

		li.angle = angle_t(i32(SHORT(ml[i].angle)) << 16)
		li.offset = fixed_t(i32(SHORT(ml[i].offset)) << 16)
		linedef := int(SHORT(ml[i].linedef))
		ldef := &lines[linedef]
		li.linedef = ldef
		side := int(SHORT(ml[i].side))
		li.sidedef = &sides[ldef.sidenum[side]]
		li.frontsector = sides[ldef.sidenum[side]].sector
		if (int(ldef.flags) & ML_TWOSIDED) != 0 {
			li.backsector = sides[ldef.sidenum[side ~ 1]].sector
		} else {
			li.backsector = nil
		}
	}

	Z_Free(data)
}

P_LoadSubsectors :: proc(lump: int) {
	numsubsectors = W_LumpLength(lump) / size_of(mapsubsector_t)
	subsectors = ([^]subsector_t)(Z_Malloc(numsubsectors * size_of(subsector_t), PU_LEVEL, nil))
	data := W_CacheLumpNum(lump, PU_STATIC)

	ms := ([^]mapsubsector_t)(data)
	libc.memset(subsectors, 0, uint(numsubsectors * size_of(subsector_t)))

	for i := 0; i < numsubsectors; i += 1 {
		subsectors[i].numlines = SHORT(ms[i].numsegs)
		subsectors[i].firstline = SHORT(ms[i].firstseg)
	}

	Z_Free(data)
}

P_LoadSectors :: proc(lump: int) {
	numsectors = W_LumpLength(lump) / size_of(mapsector_t)
	sectors = ([^]sector_t)(Z_Malloc(numsectors * size_of(sector_t), PU_LEVEL, nil))
	libc.memset(sectors, 0, uint(numsectors * size_of(sector_t)))
	data := W_CacheLumpNum(lump, PU_STATIC)

	ms := ([^]mapsector_t)(data)
	for i := 0; i < numsectors; i += 1 {
		ss := &sectors[i]
		ss.floorheight = fixed_t(i32(SHORT(ms[i].floorheight)) << FRACBITS)
		ss.ceilingheight = fixed_t(i32(SHORT(ms[i].ceilingheight)) << FRACBITS)
		ss.floorpic = i16(R_FlatNumForName(cstring(&ms[i].floorpic[0])))
		ss.ceilingpic = i16(R_FlatNumForName(cstring(&ms[i].ceilingpic[0])))
		ss.lightlevel = SHORT(ms[i].lightlevel)
		ss.special = SHORT(ms[i].special)
		ss.tag = SHORT(ms[i].tag)
		ss.thinglist = nil
	}

	Z_Free(data)
}

P_LoadNodes :: proc(lump: int) {
	numnodes = W_LumpLength(lump) / size_of(mapnode_t)
	nodes = ([^]node_t)(Z_Malloc(numnodes * size_of(node_t), PU_LEVEL, nil))
	data := W_CacheLumpNum(lump, PU_STATIC)

	mn := ([^]mapnode_t)(data)
	for i := 0; i < numnodes; i += 1 {
		no := &nodes[i]
		no.x = fixed_t(i32(SHORT(mn[i].x)) << FRACBITS)
		no.y = fixed_t(i32(SHORT(mn[i].y)) << FRACBITS)
		no.dx = fixed_t(i32(SHORT(mn[i].dx)) << FRACBITS)
		no.dy = fixed_t(i32(SHORT(mn[i].dy)) << FRACBITS)
		for j := 0; j < 2; j += 1 {
			no.children[j] = u16(SHORT(i16(mn[i].children[j])))
			for k := 0; k < 4; k += 1 {
				no.bbox[j][k] = fixed_t(i32(SHORT(mn[i].bbox[j][k])) << FRACBITS)
			}
		}
	}

	Z_Free(data)
}

P_LoadThings :: proc(lump: int) {
	data := W_CacheLumpNum(lump, PU_STATIC)
	numthings := W_LumpLength(lump) / size_of(mapthing_t)

	mt := ([^]mapthing_t)(data)
	for i := 0; i < numthings; i += 1 {
		spawn := true

		if gamemode != .commercial {
			switch mt[i].type {
			case 68, 64, 88, 89, 69, 67, 71, 65, 66, 84:
				spawn = false
			}
		}
		if !spawn {
			break
		}

		mt[i].x = SHORT(mt[i].x)
		mt[i].y = SHORT(mt[i].y)
		mt[i].angle = SHORT(mt[i].angle)
		mt[i].type = SHORT(mt[i].type)
		mt[i].options = SHORT(mt[i].options)

		P_SpawnMapThing(&mt[i])
	}

	Z_Free(data)
}

P_LoadLineDefs :: proc(lump: int) {
	numlines = W_LumpLength(lump) / size_of(maplinedef_t)
	lines = ([^]line_t)(Z_Malloc(numlines * size_of(line_t), PU_LEVEL, nil))
	libc.memset(lines, 0, uint(numlines * size_of(line_t)))
	data := W_CacheLumpNum(lump, PU_STATIC)

	mld := ([^]maplinedef_t)(data)
	for i := 0; i < numlines; i += 1 {
		ld := &lines[i]
		ld.flags = SHORT(mld[i].flags)
		ld.special = SHORT(mld[i].special)
		ld.tag = SHORT(mld[i].tag)
		v1 := &vertexes[u16(SHORT(mld[i].v1))]
		v2 := &vertexes[u16(SHORT(mld[i].v2))]
		ld.v1 = v1
		ld.v2 = v2
		ld.dx = v2.x - v1.x
		ld.dy = v2.y - v1.y

		if ld.dx == 0 {
			ld.slopetype = .ST_VERTICAL
		} else if ld.dy == 0 {
			ld.slopetype = .ST_HORIZONTAL
		} else {
			if FixedDiv(ld.dy, ld.dx) > 0 {
				ld.slopetype = .ST_POSITIVE
			} else {
				ld.slopetype = .ST_NEGATIVE
			}
		}

		if v1.x < v2.x {
			ld.bbox[BOXLEFT] = v1.x
			ld.bbox[BOXRIGHT] = v2.x
		} else {
			ld.bbox[BOXLEFT] = v2.x
			ld.bbox[BOXRIGHT] = v1.x
		}

		if v1.y < v2.y {
			ld.bbox[BOXBOTTOM] = v1.y
			ld.bbox[BOXTOP] = v2.y
		} else {
			ld.bbox[BOXBOTTOM] = v2.y
			ld.bbox[BOXTOP] = v1.y
		}

		ld.sidenum[0] = SHORT(mld[i].sidenum[0])
		ld.sidenum[1] = SHORT(mld[i].sidenum[1])

		if ld.sidenum[0] != -1 {
			ld.frontsector = sides[ld.sidenum[0]].sector
		} else {
			ld.frontsector = nil
		}

		if ld.sidenum[1] != -1 {
			ld.backsector = sides[ld.sidenum[1]].sector
		} else {
			ld.backsector = nil
		}
	}

	Z_Free(data)
}

P_LoadSideDefs :: proc(lump: int) {
	numsides = W_LumpLength(lump) / size_of(mapsidedef_t)
	sides = ([^]side_t)(Z_Malloc(numsides * size_of(side_t), PU_LEVEL, nil))
	libc.memset(sides, 0, uint(numsides * size_of(side_t)))
	data := W_CacheLumpNum(lump, PU_STATIC)

	msd := ([^]mapsidedef_t)(data)
	for i := 0; i < numsides; i += 1 {
		sd := &sides[i]
		sd.textureoffset = fixed_t(i32(SHORT(msd[i].textureoffset)) << FRACBITS)
		sd.rowoffset = fixed_t(i32(SHORT(msd[i].rowoffset)) << FRACBITS)
		sd.toptexture = i16(R_TextureNumForName(cstring(&msd[i].toptexture[0])))
		sd.bottomtexture = i16(R_TextureNumForName(cstring(&msd[i].bottomtexture[0])))
		sd.midtexture = i16(R_TextureNumForName(cstring(&msd[i].midtexture[0])))
		sd.sector = &sectors[SHORT(msd[i].sector)]
	}

	Z_Free(data)
}

P_LoadBlockMap :: proc(lump: int) {
	blockmaplump = ([^]i16)(W_CacheLumpNum(lump, PU_LEVEL))
	blockmap = &blockmaplump[4]
	count := W_LumpLength(lump) / 2

	for i := 0; i < count; i += 1 {
		blockmaplump[i] = SHORT(blockmaplump[i])
	}

	bmaporgx = fixed_t(i32(blockmaplump[0]) << FRACBITS)
	bmaporgy = fixed_t(i32(blockmaplump[1]) << FRACBITS)
	bmapwidth = int(blockmaplump[2])
	bmapheight = int(blockmaplump[3])

	link_size := size_of(^mobj_t) * bmapwidth * bmapheight
	blocklinks = ([^]^mobj_t)(Z_Malloc(link_size, PU_LEVEL, nil))
	libc.memset(blocklinks, 0, uint(link_size))
}

P_GroupLines :: proc() {
	bbox: [4]fixed_t

	for i := 0; i < numsubsectors; i += 1 {
		ss := &subsectors[i]
		seg := &segs[ss.firstline]
		ss.sector = seg.sidedef.sector
	}

	total := 0
	for i := 0; i < numlines; i += 1 {
		li := &lines[i]
		total += 1
		li.frontsector.linecount += 1

		if li.backsector != nil && li.backsector != li.frontsector {
			li.backsector.linecount += 1
			total += 1
		}
	}

	linebuffer := ([^]^line_t)(Z_Malloc(total * size_of(^line_t), PU_LEVEL, nil))
	for i := 0; i < numsectors; i += 1 {
		sector := &sectors[i]
		M_ClearBox(&bbox[0])
		sector.lines = linebuffer
		for j := 0; j < numlines; j += 1 {
			li := &lines[j]
			if li.frontsector == sector || li.backsector == sector {
				linebuffer[0] = li
				linebuffer = &linebuffer[1]
				M_AddToBox(&bbox[0], li.v1.x, li.v1.y)
				M_AddToBox(&bbox[0], li.v2.x, li.v2.y)
			}
		}
		if int(uintptr(linebuffer) - uintptr(sector.lines)) / size_of(^line_t) != sector.linecount {
			I_Error("P_GroupLines: miscounted")
		}

		sector.soundorg.x = (bbox[BOXRIGHT] + bbox[BOXLEFT]) / 2
		sector.soundorg.y = (bbox[BOXTOP] + bbox[BOXBOTTOM]) / 2

		block := int((bbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT)
		block = block >= bmapheight ? bmapheight - 1 : block
		sector.blockbox[BOXTOP] = block

		block = int((bbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT)
		block = block < 0 ? 0 : block
		sector.blockbox[BOXBOTTOM] = block

		block = int((bbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT)
		block = block >= bmapwidth ? bmapwidth - 1 : block
		sector.blockbox[BOXRIGHT] = block

		block = int((bbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT)
		block = block < 0 ? 0 : block
		sector.blockbox[BOXLEFT] = block
	}
}

P_SetupLevel :: proc(episode: int, map_num: int, playermask: int, skill: skill_t) {
	lumpname: [9]u8

	totalkills = 0
	totalitems = 0
	totalsecret = 0
	wminfo.maxfrags = 0
	wminfo.partime = 180
	for i := 0; i < MAXPLAYERS; i += 1 {
		players[i].killcount = 0
		players[i].secretcount = 0
		players[i].itemcount = 0
	}

	players[consoleplayer].viewz = 1

	S_Start()

	Z_FreeTags(PU_LEVEL, PU_PURGELEVEL - 1)

	P_InitThinkers()

	W_Reload()

	if gamemode == .commercial {
		if map_num < 10 {
			libc.snprintf(&lumpname[0], len(lumpname), "map0%i", i32(map_num))
		} else {
			libc.snprintf(&lumpname[0], len(lumpname), "map%i", i32(map_num))
		}
	} else {
		lumpname[0] = 'E'
		lumpname[1] = u8('0' + episode)
		lumpname[2] = 'M'
		lumpname[3] = u8('0' + map_num)
		lumpname[4] = 0
	}

	lumpnum := W_GetNumForName(cstring(&lumpname[0]))

	leveltime = 0

	P_LoadBlockMap(lumpnum + ML_BLOCKMAP)
	P_LoadVertexes(lumpnum + ML_VERTEXES)
	P_LoadSectors(lumpnum + ML_SECTORS)
	P_LoadSideDefs(lumpnum + ML_SIDEDEFS)

	P_LoadLineDefs(lumpnum + ML_LINEDEFS)
	P_LoadSubsectors(lumpnum + ML_SSECTORS)
	P_LoadNodes(lumpnum + ML_NODES)
	P_LoadSegs(lumpnum + ML_SEGS)

	rejectmatrix = ([^]byte)(W_CacheLumpNum(lumpnum + ML_REJECT, PU_LEVEL))
	P_GroupLines()

	bodyqueslot = 0
	deathmatch_p = &deathmatchstarts[0]
	P_LoadThings(lumpnum + ML_THINGS)

	if deathmatch != 0 {
		for i := 0; i < MAXPLAYERS; i += 1 {
			if playeringame[i] {
				players[i].mo = nil
				G_DeathMatchSpawnPlayer(i)
			}
		}
	}

	iquehead = 0
	iquetail = 0

	P_SpawnSpecials()

	if precache {
		R_PrecacheLevel()
	}
}

P_Init :: proc() {
	P_InitSwitchList()
	P_InitPicAnims()
	R_InitSprites(&sprnames[0])
}