// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

FIELDOFVIEW :: 2048

viewangleoffset: angle_t
validcount: int = 1

fixedcolormap: [^]lighttable_t

centerx: int
centery: int

centerxfrac: fixed_t
centeryfrac: fixed_t
projection: fixed_t

framecount: int
sscount: int
linecount: int
loopcount: int

viewx: fixed_t
viewy: fixed_t
viewz: fixed_t

viewangle: angle_t

viewcos: fixed_t
viewsin: fixed_t

viewplayer: ^player_t

detailshift: int

clipangle: angle_t

viewangletox: [FINEANGLES / 2]int
xtoviewangle: [SCREENWIDTH + 1]angle_t

finecosine: [^]fixed_t = &finesine[FINEANGLES / 4]

scalelight: [LIGHTLEVELS][MAXLIGHTSCALE][^]lighttable_t
scalelightfixed: [MAXLIGHTSCALE][^]lighttable_t
zlight: [LIGHTLEVELS][MAXLIGHTZ][^]lighttable_t

extralight: int

colfunc: #type proc()
basecolfunc: #type proc()
fuzzcolfunc: #type proc()
transcolfunc: #type proc()
spanfunc: #type proc()

setsizeneeded: boolean
setblocks: int
setdetail: int

DISTMAP :: 2

R_AddPointToBox :: proc(x: int, y: int, box: [^]fixed_t) {
	if fixed_t(x) < box[BOXLEFT] {
		box[BOXLEFT] = fixed_t(x)
	}
	if fixed_t(x) > box[BOXRIGHT] {
		box[BOXRIGHT] = fixed_t(x)
	}
	if fixed_t(y) < box[BOXBOTTOM] {
		box[BOXBOTTOM] = fixed_t(y)
	}
	if fixed_t(y) > box[BOXTOP] {
		box[BOXTOP] = fixed_t(y)
	}
}

R_PointOnSide :: proc(x: fixed_t, y: fixed_t, node: ^node_t) -> int {
	if node.dx == 0 {
		if x <= node.x {
			return node.dy > 0 ? 1 : 0
		}
		return node.dy < 0 ? 1 : 0
	}
	if node.dy == 0 {
		if y <= node.y {
			return node.dx < 0 ? 1 : 0
		}
		return node.dx > 0 ? 1 : 0
	}

	dx := x - node.x
	dy := y - node.y

	if (u32(node.dy ~ node.dx ~ dx ~ dy) & 0x80000000) != 0 {
		if (u32(node.dy ~ dx) & 0x80000000) != 0 {
			return 1
		}
		return 0
	}

	left := FixedMul(node.dy >> FRACBITS, dx)
	right := FixedMul(dy, node.dx >> FRACBITS)

	if right < left {
		return 0
	}
	return 1
}

R_PointOnSegSide :: proc(x: fixed_t, y: fixed_t, line: ^seg_t) -> int {
	lx := line.v1.x
	ly := line.v1.y

	ldx := line.v2.x - lx
	ldy := line.v2.y - ly

	if ldx == 0 {
		if x <= lx {
			return ldy > 0 ? 1 : 0
		}
		return ldy < 0 ? 1 : 0
	}
	if ldy == 0 {
		if y <= ly {
			return ldx < 0 ? 1 : 0
		}
		return ldx > 0 ? 1 : 0
	}

	dx := x - lx
	dy := y - ly

	if (u32(ldy ~ ldx ~ dx ~ dy) & 0x80000000) != 0 {
		if (u32(ldy ~ dx) & 0x80000000) != 0 {
			return 1
		}
		return 0
	}

	left := FixedMul(ldy >> FRACBITS, dx)
	right := FixedMul(dy, ldx >> FRACBITS)

	if right < left {
		return 0
	}
	return 1
}

SlopeDiv :: proc(num: u32, den: u32) -> int {
	if den < 512 {
		return SLOPERANGE
	}
	ans := (num << 3) / (den >> 8)
	return ans <= SLOPERANGE ? int(ans) : SLOPERANGE
}

R_PointToAngle :: proc(x_in: fixed_t, y_in: fixed_t) -> angle_t {
	x := x_in - viewx
	y := y_in - viewy

	if x == 0 && y == 0 {
		return 0
	}

	if x >= 0 {
		if y >= 0 {
			if x > y {
				return tantoangle[SlopeDiv(u32(y), u32(x))]
			} else {
				return ANG90 - 1 - tantoangle[SlopeDiv(u32(x), u32(y))]
			}
		} else {
			y = -y
			if x > y {
				return -tantoangle[SlopeDiv(u32(y), u32(x))]
			} else {
				return ANG270 + tantoangle[SlopeDiv(u32(x), u32(y))]
			}
		}
	} else {
		x = -x
		if y >= 0 {
			if x > y {
				return ANG180 - 1 - tantoangle[SlopeDiv(u32(y), u32(x))]
			} else {
				return ANG90 + tantoangle[SlopeDiv(u32(x), u32(y))]
			}
		} else {
			y = -y
			if x > y {
				return ANG180 + tantoangle[SlopeDiv(u32(y), u32(x))]
			} else {
				return ANG270 - 1 - tantoangle[SlopeDiv(u32(x), u32(y))]
			}
		}
	}
}

R_PointToAngle2 :: proc(x1: fixed_t, y1: fixed_t, x2: fixed_t, y2: fixed_t) -> angle_t {
	viewx = x1
	viewy = y1
	return R_PointToAngle(x2, y2)
}

R_PointToDist :: proc(x: fixed_t, y: fixed_t) -> fixed_t {
	dx := abs(x - viewx)
	dy := abs(y - viewy)

	if dy > dx {
		temp := dx
		dx = dy
		dy = temp
	}

	angle := (tantoangle[FixedDiv(dy, dx) >> DBITS] + ANG90) >> ANGLETOFINESHIFT
	dist := FixedDiv(dx, finesine[angle])
	return dist
}

R_InitPointToAngle :: proc() {
}

R_ScaleFromGlobalAngle :: proc(visangle: angle_t) -> fixed_t {
	anglea := ANG90 + (visangle - viewangle)
	angleb := ANG90 + (visangle - rw_normalangle)

	sinea := finesine[anglea >> ANGLETOFINESHIFT]
	sineb := finesine[angleb >> ANGLETOFINESHIFT]
	num := FixedMul(projection, sineb) << uint(detailshift)
	den := FixedMul(rw_distance, sinea)

	scale: fixed_t
	if den > (num >> 16) {
		scale = FixedDiv(num, den)
		if scale > 64 * FRACUNIT {
			scale = 64 * FRACUNIT
		} else if scale < 256 {
			scale = 256
		}
	} else {
		scale = 64 * FRACUNIT
	}

	return scale
}

R_InitTables :: proc() {
}

R_InitTextureMapping :: proc() {
	focallength := FixedDiv(centerxfrac, finetangent[FINEANGLES / 4 + FIELDOFVIEW / 2])

	for i := 0; i < FINEANGLES / 2; i += 1 {
		t: int
		if finetangent[i] > FRACUNIT * 2 {
			t = -1
		} else if finetangent[i] < -FRACUNIT * 2 {
			t = viewwidth + 1
		} else {
			tf := FixedMul(finetangent[i], focallength)
			t = int((centerxfrac - tf + FRACUNIT - 1) >> FRACBITS)
			if t < -1 {
				t = -1
			} else if t > viewwidth + 1 {
				t = viewwidth + 1
			}
		}
		viewangletox[i] = t
	}

	for x := 0; x <= viewwidth; x += 1 {
		i := 0
		for viewangletox[i] > x {
			i += 1
		}
		xtoviewangle[x] = angle_t(i << ANGLETOFINESHIFT) - ANG90
	}

	for i := 0; i < FINEANGLES / 2; i += 1 {
		if viewangletox[i] == -1 {
			viewangletox[i] = 0
		} else if viewangletox[i] == viewwidth + 1 {
			viewangletox[i] = viewwidth
		}
	}

	clipangle = xtoviewangle[0]
}

R_InitLightTables :: proc() {
	for i := 0; i < LIGHTLEVELS; i += 1 {
		startmap := ((LIGHTLEVELS - 1 - i) * 2) * NUMCOLORMAPS / LIGHTLEVELS
		for j := 0; j < MAXLIGHTZ; j += 1 {
			scale := int(FixedDiv(SCREENWIDTH / 2 * FRACUNIT, fixed_t((j + 1) << LIGHTZSHIFT)))
			scale >>= LIGHTSCALESHIFT
			level := startmap - scale / DISTMAP

			if level < 0 {
				level = 0
			}
			if level >= NUMCOLORMAPS {
				level = NUMCOLORMAPS - 1
			}

			zlight[i][j] = &colormaps[level * 256]
		}
	}
}

R_SetViewSize :: proc(blocks: int, detail: int) {
	setsizeneeded = true
	setblocks = blocks
	setdetail = detail
}

R_ExecuteSetViewSize :: proc() {
	setsizeneeded = false

	if setblocks == 11 {
		scaledviewwidth = SCREENWIDTH
		viewheight = SCREENHEIGHT
	} else {
		scaledviewwidth = setblocks * 32
		viewheight = (setblocks * 168 / 10) & ~int(7)
	}

	detailshift = setdetail
	viewwidth = scaledviewwidth >> uint(detailshift)

	centery = viewheight / 2
	centerx = viewwidth / 2
	centerxfrac = fixed_t(i32(centerx) << FRACBITS)
	centeryfrac = fixed_t(i32(centery) << FRACBITS)
	projection = centerxfrac

	if detailshift == 0 {
		basecolfunc = R_DrawColumn
		colfunc = basecolfunc
		fuzzcolfunc = R_DrawFuzzColumn
		transcolfunc = R_DrawTranslatedColumn
		spanfunc = R_DrawSpan
	} else {
		basecolfunc = R_DrawColumnLow
		colfunc = basecolfunc
		fuzzcolfunc = R_DrawFuzzColumn
		transcolfunc = R_DrawTranslatedColumn
		spanfunc = R_DrawSpanLow
	}

	R_InitBuffer(scaledviewwidth, viewheight)
	R_InitTextureMapping()

	pspritescale = fixed_t(FRACUNIT * viewwidth / SCREENWIDTH)
	pspriteiscale = fixed_t(FRACUNIT * SCREENWIDTH / viewwidth)

	for i := 0; i < viewwidth; i += 1 {
		screenheightarray[i] = i16(viewheight)
	}

	for i := 0; i < viewheight; i += 1 {
		dy := fixed_t((i32(i - viewheight / 2) << FRACBITS) + FRACUNIT / 2)
		dy = abs(dy)
		yslope[i] = FixedDiv(fixed_t((viewwidth << uint(detailshift)) / 2 * FRACUNIT), dy)
	}

	for i := 0; i < viewwidth; i += 1 {
		cosadj := abs(finecosine[xtoviewangle[i] >> ANGLETOFINESHIFT])
		distscale[i] = FixedDiv(FRACUNIT, cosadj)
	}

	for i := 0; i < LIGHTLEVELS; i += 1 {
		startmap := ((LIGHTLEVELS - 1 - i) * 2) * NUMCOLORMAPS / LIGHTLEVELS
		for j := 0; j < MAXLIGHTSCALE; j += 1 {
			level := startmap - j * SCREENWIDTH / (viewwidth << uint(detailshift)) / DISTMAP

			if level < 0 {
				level = 0
			}
			if level >= NUMCOLORMAPS {
				level = NUMCOLORMAPS - 1
			}

			scalelight[i][j] = &colormaps[level * 256]
		}
	}
}

R_Init :: proc() {
	finecosine = &finesine[FINEANGLES / 4]
	R_InitData()
	libc.printf("\nR_InitData")
	R_InitPointToAngle()
	libc.printf("\nR_InitPointToAngle")
	R_InitTables()
	libc.printf("\nR_InitTables")

	R_SetViewSize(screenblocks, detailLevel)
	R_InitPlanes()
	libc.printf("\nR_InitPlanes")
	R_InitLightTables()
	libc.printf("\nR_InitLightTables")
	R_InitSkyMap()
	libc.printf("\nR_InitSkyMap")
	R_InitTranslationTables()
	libc.printf("\nR_InitTranslationsTables")

	framecount = 0
}

R_PointInSubsector :: proc(x: fixed_t, y: fixed_t) -> ^subsector_t {
	if numnodes == 0 {
		return &subsectors[0]
	}

	nodenum := numnodes - 1

	for (nodenum & NF_SUBSECTOR) == 0 {
		node := &nodes[nodenum]
		side := R_PointOnSide(x, y, node)
		nodenum = int(node.children[side])
	}

	return &subsectors[nodenum & ~int(NF_SUBSECTOR)]
}

R_SetupFrame :: proc(player: ^player_t) {
	viewplayer = player
	viewx = player.mo.x
	viewy = player.mo.y
	viewangle = player.mo.angle + viewangleoffset
	extralight = player.extralight

	viewz = player.viewz

	viewsin = finesine[viewangle >> ANGLETOFINESHIFT]
	viewcos = finecosine[viewangle >> ANGLETOFINESHIFT]

	sscount = 0

	if player.fixedcolormap != 0 {
		fixedcolormap = &colormaps[player.fixedcolormap * 256 * size_of(lighttable_t)]
		walllights = &scalelightfixed[0]

		for i := 0; i < MAXLIGHTSCALE; i += 1 {
			scalelightfixed[i] = fixedcolormap
		}
	} else {
		fixedcolormap = nil
	}

	framecount += 1
	validcount += 1
}

R_RenderPlayerView :: proc(player: ^player_t) {
	R_SetupFrame(player)

	R_ClearClipSegs()
	R_ClearDrawSegs()
	R_ClearPlanes()
	R_ClearSprites()

	NetUpdate()

	R_RenderBSPNode(numnodes - 1)

	NetUpdate()

	R_DrawPlanes()

	NetUpdate()

	R_DrawMasked()

	NetUpdate()
}
