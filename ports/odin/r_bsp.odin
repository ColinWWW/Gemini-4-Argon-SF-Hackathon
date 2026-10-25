// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

curline: ^seg_t
sidedef: ^side_t
linedef: ^line_t
frontsector: ^sector_t
backsector: ^sector_t

drawsegs: [MAXDRAWSEGS]drawseg_t
ds_p: [^]drawseg_t

cliprange_t :: struct {
	first: int,
	last:  int,
}

MAXSEGS :: 32
newend: [^]cliprange_t
solidsegs: [MAXSEGS]cliprange_t

checkcoord := [12][4]int{
	{3, 0, 2, 1},
	{3, 0, 2, 0},
	{3, 1, 2, 0},
	{0, 0, 0, 0},
	{2, 0, 2, 1},
	{0, 0, 0, 0},
	{3, 1, 3, 0},
	{0, 0, 0, 0},
	{2, 0, 3, 1},
	{2, 1, 3, 1},
	{2, 1, 3, 0},
	{0, 0, 0, 0},
}

R_ClearDrawSegs :: proc() {
	ds_p = &drawsegs[0]
}

R_ClipSolidWallSegment :: proc(first: int, last: int) {
	start := ([^]cliprange_t)(&solidsegs[0])
	for start[0].last < first - 1 {
		start = &start[1]
	}

	if first < start[0].first {
		if last < start[0].first - 1 {
			R_StoreWallRange(first, last)
			next := newend
			newend = &newend[1]

			for next != start {
				prev := ([^]cliprange_t)(uintptr(next) - size_of(cliprange_t))
				next[0] = prev[0]
				next = prev
			}
			next[0].first = first
			next[0].last = last
			return
		}

		R_StoreWallRange(first, start[0].first - 1)
		start[0].first = first
	}

	if last <= start[0].last {
		return
	}

	next := start
	crunch := false
	for last >= next[1].first - 1 {
		R_StoreWallRange(next[0].last + 1, next[1].first - 1)
		next = &next[1]

		if last <= next[0].last {
			start[0].last = next[0].last
			crunch = true
			break
		}
	}

	if !crunch {
		R_StoreWallRange(next[0].last + 1, last)
		start[0].last = last
	}

	if next == start {
		return
	}

	for {
		old_next := next
		next = &next[1]
		if old_next == newend {
			break
		}
		start = &start[1]
		start[0] = next[0]
	}

	newend = &start[1]
}

R_ClipPassWallSegment :: proc(first: int, last: int) {
	start := ([^]cliprange_t)(&solidsegs[0])
	for start[0].last < first - 1 {
		start = &start[1]
	}

	if first < start[0].first {
		if last < start[0].first - 1 {
			R_StoreWallRange(first, last)
			return
		}
		R_StoreWallRange(first, start[0].first - 1)
	}

	if last <= start[0].last {
		return
	}

	for last >= start[1].first - 1 {
		R_StoreWallRange(start[0].last + 1, start[1].first - 1)
		start = &start[1]

		if last <= start[0].last {
			return
		}
	}

	R_StoreWallRange(start[0].last + 1, last)
}

R_ClearClipSegs :: proc() {
	solidsegs[0].first = -0x7fffffff
	solidsegs[0].last = -1
	solidsegs[1].first = viewwidth
	solidsegs[1].last = 0x7fffffff
	newend = &solidsegs[2]
}

R_AddLine :: proc(line: ^seg_t) {
	curline = line

	angle1 := R_PointToAngle(line.v1.x, line.v1.y)
	angle2 := R_PointToAngle(line.v2.x, line.v2.y)

	span := angle1 - angle2

	if span >= ANG180 {
		return
	}

	rw_angle1 = int(angle1)
	angle1 -= viewangle
	angle2 -= viewangle

	tspan := angle1 + clipangle
	if tspan > 2 * clipangle {
		tspan -= 2 * clipangle
		if tspan >= span {
			return
		}
		angle1 = clipangle
	}
	tspan = clipangle - angle2
	if tspan > 2 * clipangle {
		tspan -= 2 * clipangle
		if tspan >= span {
			return
		}
		angle2 = -clipangle
	}

	angle1 = (angle1 + ANG90) >> ANGLETOFINESHIFT
	angle2 = (angle2 + ANG90) >> ANGLETOFINESHIFT
	x1 := viewangletox[angle1]
	x2 := viewangletox[angle2]

	if x1 == x2 {
		return
	}

	backsector = line.backsector

	if backsector == nil {
		R_ClipSolidWallSegment(x1, x2 - 1)
		return
	}

	if backsector.ceilingheight <= frontsector.floorheight || backsector.floorheight >= frontsector.ceilingheight {
		R_ClipSolidWallSegment(x1, x2 - 1)
		return
	}

	if backsector.ceilingheight != frontsector.ceilingheight || backsector.floorheight != frontsector.floorheight {
		R_ClipPassWallSegment(x1, x2 - 1)
		return
	}

	if backsector.ceilingpic == frontsector.ceilingpic &&
	   backsector.floorpic == frontsector.floorpic &&
	   backsector.lightlevel == frontsector.lightlevel &&
	   curline.sidedef.midtexture == 0 {
		return
	}

	R_ClipPassWallSegment(x1, x2 - 1)
}

R_CheckBBox :: proc(bspcoord: [^]fixed_t) -> boolean {
	boxx, boxy: int
	if viewx <= bspcoord[BOXLEFT] {
		boxx = 0
	} else if viewx < bspcoord[BOXRIGHT] {
		boxx = 1
	} else {
		boxx = 2
	}

	if viewy >= bspcoord[BOXTOP] {
		boxy = 0
	} else if viewy > bspcoord[BOXBOTTOM] {
		boxy = 1
	} else {
		boxy = 2
	}

	boxpos := (boxy << 2) + boxx
	if boxpos == 5 {
		return true
	}

	x1 := bspcoord[checkcoord[boxpos][0]]
	y1 := bspcoord[checkcoord[boxpos][1]]
	x2 := bspcoord[checkcoord[boxpos][2]]
	y2 := bspcoord[checkcoord[boxpos][3]]

	angle1 := R_PointToAngle(x1, y1) - viewangle
	angle2 := R_PointToAngle(x2, y2) - viewangle

	span := angle1 - angle2

	if span >= ANG180 {
		return true
	}

	tspan := angle1 + clipangle
	if tspan > 2 * clipangle {
		tspan -= 2 * clipangle
		if tspan >= span {
			return false
		}
		angle1 = clipangle
	}
	tspan = clipangle - angle2
	if tspan > 2 * clipangle {
		tspan -= 2 * clipangle
		if tspan >= span {
			return false
		}
		angle2 = -clipangle
	}

	angle1 = (angle1 + ANG90) >> ANGLETOFINESHIFT
	angle2 = (angle2 + ANG90) >> ANGLETOFINESHIFT
	sx1 := viewangletox[angle1]
	sx2 := viewangletox[angle2]

	if sx1 == sx2 {
		return false
	}
	sx2 -= 1

	start := ([^]cliprange_t)(&solidsegs[0])
	for start[0].last < sx2 {
		start = &start[1]
	}

	if sx1 >= start[0].first && sx2 <= start[0].last {
		return false
	}

	return true
}

R_Subsector :: proc(num: int) {
	if num >= numsubsectors {
		I_Error("R_Subsector: ss %i with numss = %i", i32(num), i32(numsubsectors))
	}

	sscount += 1
	sub := &subsectors[num]
	frontsector = sub.sector
	count := int(sub.numlines)
	line_idx := int(sub.firstline)

	if frontsector.floorheight < viewz {
		floorplane = R_FindPlane(frontsector.floorheight, int(frontsector.floorpic), int(frontsector.lightlevel))
	} else {
		floorplane = nil
	}

	if frontsector.ceilingheight > viewz || int(frontsector.ceilingpic) == skyflatnum {
		ceilingplane = R_FindPlane(frontsector.ceilingheight, int(frontsector.ceilingpic), int(frontsector.lightlevel))
	} else {
		ceilingplane = nil
	}

	R_AddSprites(frontsector)

	for count > 0 {
		count -= 1
		R_AddLine(&segs[line_idx])
		line_idx += 1
	}
}

R_RenderBSPNode :: proc(bspnum: int) {
	if (bspnum & NF_SUBSECTOR) != 0 {
		if bspnum == -1 {
			R_Subsector(0)
		} else {
			R_Subsector(bspnum & (~int(NF_SUBSECTOR)))
		}
		return
	}

	bsp := &nodes[bspnum]
	side := R_PointOnSide(viewx, viewy, bsp)

	R_RenderBSPNode(int(bsp.children[side]))

	if R_CheckBBox(&bsp.bbox[side ~ 1][0]) {
		R_RenderBSPNode(int(bsp.children[side ~ 1]))
	}
}
