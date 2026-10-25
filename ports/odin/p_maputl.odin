// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

opentop: fixed_t
openbottom: fixed_t
openrange: fixed_t
lowfloor: fixed_t

intercepts: [MAXINTERCEPTS]intercept_t
intercept_p: [^]intercept_t

trace: divline_t
earlyout: boolean
ptflags: int

P_AproxDistance :: proc(dx_in: fixed_t, dy_in: fixed_t) -> fixed_t {
	dx := abs(dx_in)
	dy := abs(dy_in)
	if dx < dy {
		return dx + dy - (dx >> 1)
	}
	return dx + dy - (dy >> 1)
}

P_PointOnLineSide :: proc(x: fixed_t, y: fixed_t, line: ^line_t) -> int {
	if line.dx == 0 {
		if x <= line.v1.x {
			return line.dy > 0 ? 1 : 0
		}
		return line.dy < 0 ? 1 : 0
	}
	if line.dy == 0 {
		if y <= line.v1.y {
			return line.dx < 0 ? 1 : 0
		}
		return line.dx > 0 ? 1 : 0
	}

	dx := x - line.v1.x
	dy := y - line.v1.y

	left := FixedMul(line.dy >> FRACBITS, dx)
	right := FixedMul(dy, line.dx >> FRACBITS)

	if right < left {
		return 0
	}
	return 1
}

P_BoxOnLineSide :: proc(tmbox: [^]fixed_t, ld: ^line_t) -> int {
	p1, p2: int

	switch ld.slopetype {
	case .ST_HORIZONTAL:
		p1 = tmbox[BOXTOP] > ld.v1.y ? 1 : 0
		p2 = tmbox[BOXBOTTOM] > ld.v1.y ? 1 : 0
		if ld.dx < 0 {
			p1 ~= 1
			p2 ~= 1
		}
	case .ST_VERTICAL:
		p1 = tmbox[BOXRIGHT] < ld.v1.x ? 1 : 0
		p2 = tmbox[BOXLEFT] < ld.v1.x ? 1 : 0
		if ld.dy < 0 {
			p1 ~= 1
			p2 ~= 1
		}
	case .ST_POSITIVE:
		p1 = P_PointOnLineSide(tmbox[BOXLEFT], tmbox[BOXTOP], ld)
		p2 = P_PointOnLineSide(tmbox[BOXRIGHT], tmbox[BOXBOTTOM], ld)
	case .ST_NEGATIVE:
		p1 = P_PointOnLineSide(tmbox[BOXRIGHT], tmbox[BOXTOP], ld)
		p2 = P_PointOnLineSide(tmbox[BOXLEFT], tmbox[BOXBOTTOM], ld)
	}

	if p1 == p2 {
		return p1
	}
	return -1
}

P_PointOnDivlineSide :: proc(x: fixed_t, y: fixed_t, line: ^divline_t) -> int {
	if line.dx == 0 {
		if x <= line.x {
			return line.dy > 0 ? 1 : 0
		}
		return line.dy < 0 ? 1 : 0
	}
	if line.dy == 0 {
		if y <= line.y {
			return line.dx < 0 ? 1 : 0
		}
		return line.dx > 0 ? 1 : 0
	}

	dx := x - line.x
	dy := y - line.y

	if (u32(line.dy ~ line.dx ~ dx ~ dy) & 0x80000000) != 0 {
		if (u32(line.dy ~ dx) & 0x80000000) != 0 {
			return 1
		}
		return 0
	}

	left := FixedMul(line.dy >> 8, dx >> 8)
	right := FixedMul(dy >> 8, line.dx >> 8)

	if right < left {
		return 0
	}
	return 1
}

P_MakeDivline :: proc(li: ^line_t, dl: ^divline_t) {
	dl.x = li.v1.x
	dl.y = li.v1.y
	dl.dx = li.dx
	dl.dy = li.dy
}

P_InterceptVector :: proc(v2: ^divline_t, v1: ^divline_t) -> fixed_t {
	den := FixedMul(v1.dy >> 8, v2.dx) - FixedMul(v1.dx >> 8, v2.dy)
	if den == 0 {
		return 0
	}

	num := FixedMul((v1.x - v2.x) >> 8, v1.dy) + FixedMul((v2.y - v1.y) >> 8, v1.dx)
	frac := FixedDiv(num, den)
	return frac
}

P_LineOpening :: proc(linedef: ^line_t) {
	if linedef.sidenum[1] == -1 {
		openrange = 0
		return
	}

	front := linedef.frontsector
	back := linedef.backsector

	if front.ceilingheight < back.ceilingheight {
		opentop = front.ceilingheight
	} else {
		opentop = back.ceilingheight
	}

	if front.floorheight > back.floorheight {
		openbottom = front.floorheight
		lowfloor = back.floorheight
	} else {
		openbottom = back.floorheight
		lowfloor = front.floorheight
	}

	openrange = opentop - openbottom
}

P_UnsetThingPosition :: proc(thing: ^mobj_t) {
	if (thing.flags & MF_NOSECTOR) == 0 {
		if thing.snext != nil {
			thing.snext.sprev = thing.sprev
		}
		if thing.sprev != nil {
			thing.sprev.snext = thing.snext
		} else {
			thing.subsector.sector.thinglist = thing.snext
		}
	}

	if (thing.flags & MF_NOBLOCKMAP) == 0 {
		if thing.bnext != nil {
			thing.bnext.bprev = thing.bprev
		}
		if thing.bprev != nil {
			thing.bprev.bnext = thing.bnext
		} else {
			blockx := int((thing.x - bmaporgx) >> MAPBLOCKSHIFT)
			blocky := int((thing.y - bmaporgy) >> MAPBLOCKSHIFT)

			if blockx >= 0 && blockx < bmapwidth && blocky >= 0 && blocky < bmapheight {
				blocklinks[blocky * bmapwidth + blockx] = thing.bnext
			}
		}
	}
}

P_SetThingPosition :: proc(thing: ^mobj_t) {
	ss := R_PointInSubsector(thing.x, thing.y)
	thing.subsector = ss

	if (thing.flags & MF_NOSECTOR) == 0 {
		sec := ss.sector
		thing.sprev = nil
		thing.snext = sec.thinglist

		if sec.thinglist != nil {
			sec.thinglist.sprev = thing
		}
		sec.thinglist = thing
	}

	if (thing.flags & MF_NOBLOCKMAP) == 0 {
		blockx := int((thing.x - bmaporgx) >> MAPBLOCKSHIFT)
		blocky := int((thing.y - bmaporgy) >> MAPBLOCKSHIFT)

		if blockx >= 0 && blockx < bmapwidth && blocky >= 0 && blocky < bmapheight {
			link := &blocklinks[blocky * bmapwidth + blockx]
			thing.bprev = nil
			thing.bnext = link^
			if link^ != nil {
				link^.bprev = thing
			}
			link^ = thing
		} else {
			thing.bnext = nil
			thing.bprev = nil
		}
	}
}

P_BlockLinesIterator :: proc(x: int, y: int, func: #type proc(^line_t) -> boolean) -> boolean {
	if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
		return true
	}

	offset := y * bmapwidth + x
	offset = int(blockmap[offset])

	for list := &blockmaplump[offset]; list[0] != -1; list = &list[1] {
		ld := &lines[list[0]]
		if ld.validcount == validcount {
			continue
		}
		ld.validcount = validcount
		if !func(ld) {
			return false
		}
	}
	return true
}

P_BlockThingsIterator :: proc(x: int, y: int, func: #type proc(^mobj_t) -> boolean) -> boolean {
	if x < 0 || y < 0 || x >= bmapwidth || y >= bmapheight {
		return true
	}

	for mobj := blocklinks[y * bmapwidth + x]; mobj != nil; mobj = mobj.bnext {
		if !func(mobj) {
			return false
		}
	}
	return true
}

PIT_AddLineIntercepts :: proc(ld: ^line_t) -> boolean {
	s1, s2: int
	if trace.dx > FRACUNIT * 16 || trace.dy > FRACUNIT * 16 || trace.dx < -FRACUNIT * 16 || trace.dy < -FRACUNIT * 16 {
		s1 = P_PointOnDivlineSide(ld.v1.x, ld.v1.y, &trace)
		s2 = P_PointOnDivlineSide(ld.v2.x, ld.v2.y, &trace)
	} else {
		s1 = P_PointOnLineSide(trace.x, trace.y, ld)
		s2 = P_PointOnLineSide(trace.x + trace.dx, trace.y + trace.dy, ld)
	}

	if s1 == s2 {
		return true
	}

	dl: divline_t
	P_MakeDivline(ld, &dl)
	frac := P_InterceptVector(&trace, &dl)

	if frac < 0 {
		return true
	}

	if earlyout && frac < FRACUNIT && ld.backsector == nil {
		return false
	}

	intercept_p[0].frac = frac
	intercept_p[0].isaline = true
	intercept_p[0].d.line = ld
	intercept_p = &intercept_p[1]

	return true
}

PIT_AddThingIntercepts :: proc(thing: ^mobj_t) -> boolean {
	tracepositive := (trace.dx ~ trace.dy) > 0

	x1, y1, x2, y2: fixed_t
	if tracepositive {
		x1 = thing.x - thing.radius
		y1 = thing.y + thing.radius
		x2 = thing.x + thing.radius
		y2 = thing.y - thing.radius
	} else {
		x1 = thing.x - thing.radius
		y1 = thing.y - thing.radius
		x2 = thing.x + thing.radius
		y2 = thing.y + thing.radius
	}

	s1 := P_PointOnDivlineSide(x1, y1, &trace)
	s2 := P_PointOnDivlineSide(x2, y2, &trace)

	if s1 == s2 {
		return true
	}

	dl := divline_t{
		x = x1,
		y = y1,
		dx = x2 - x1,
		dy = y2 - y1,
	}

	frac := P_InterceptVector(&trace, &dl)

	if frac < 0 {
		return true
	}

	intercept_p[0].frac = frac
	intercept_p[0].isaline = false
	intercept_p[0].d.thing = thing
	intercept_p = &intercept_p[1]

	return true
}

P_TraverseIntercepts :: proc(func: traverser_t, maxfrac: fixed_t) -> boolean {
	count := int(uintptr(intercept_p) - uintptr(&intercepts[0])) / size_of(intercept_t)
	in_ptr: ^intercept_t = nil

	for count > 0 {
		count -= 1
		dist := fixed_t(MAXINT)
		for scan := ([^]intercept_t)(&intercepts[0]); scan != intercept_p; scan = &scan[1] {
			if scan[0].frac < dist {
				dist = scan[0].frac
				in_ptr = &scan[0]
			}
		}

		if dist > maxfrac {
			return true
		}

		if !func(in_ptr) {
			return false
		}

		in_ptr.frac = fixed_t(MAXINT)
	}

	return true
}

P_PathTraverse :: proc(x1_in: fixed_t, y1_in: fixed_t, x2_in: fixed_t, y2_in: fixed_t, flags: int, trav: traverser_t) -> boolean {
	x1 := x1_in
	y1 := y1_in
	x2 := x2_in
	y2 := y2_in

	earlyout = (flags & PT_EARLYOUT) != 0

	validcount += 1
	intercept_p = &intercepts[0]

	if ((x1 - bmaporgx) & (MAPBLOCKSIZE - 1)) == 0 {
		x1 += FRACUNIT
	}
	if ((y1 - bmaporgy) & (MAPBLOCKSIZE - 1)) == 0 {
		y1 += FRACUNIT
	}

	trace.x = x1
	trace.y = y1
	trace.dx = x2 - x1
	trace.dy = y2 - y1

	x1 -= bmaporgx
	y1 -= bmaporgy
	xt1 := int(x1 >> MAPBLOCKSHIFT)
	yt1 := int(y1 >> MAPBLOCKSHIFT)

	x2 -= bmaporgx
	y2 -= bmaporgy
	xt2 := int(x2 >> MAPBLOCKSHIFT)
	yt2 := int(y2 >> MAPBLOCKSHIFT)

	mapxstep: int
	mapystep: int
	partial: fixed_t
	xstep: fixed_t
	ystep: fixed_t

	if xt2 > xt1 {
		mapxstep = 1
		partial = FRACUNIT - ((x1 >> MAPBTOFRAC) & (FRACUNIT - 1))
		ystep = FixedDiv(y2 - y1, abs(x2 - x1))
	} else if xt2 < xt1 {
		mapxstep = -1
		partial = (x1 >> MAPBTOFRAC) & (FRACUNIT - 1)
		ystep = FixedDiv(y2 - y1, abs(x2 - x1))
	} else {
		mapxstep = 0
		partial = FRACUNIT
		ystep = 256 * FRACUNIT
	}

	yintercept := (y1 >> MAPBTOFRAC) + FixedMul(partial, ystep)

	if yt2 > yt1 {
		mapystep = 1
		partial = FRACUNIT - ((y1 >> MAPBTOFRAC) & (FRACUNIT - 1))
		xstep = FixedDiv(x2 - x1, abs(y2 - y1))
	} else if yt2 < yt1 {
		mapystep = -1
		partial = (y1 >> MAPBTOFRAC) & (FRACUNIT - 1)
		xstep = FixedDiv(x2 - x1, abs(y2 - y1))
	} else {
		mapystep = 0
		partial = FRACUNIT
		xstep = 256 * FRACUNIT
	}
	xintercept := (x1 >> MAPBTOFRAC) + FixedMul(partial, xstep)

	mapx := xt1
	mapy := yt1

	for count := 0; count < 64; count += 1 {
		if (flags & PT_ADDLINES) != 0 {
			if !P_BlockLinesIterator(mapx, mapy, PIT_AddLineIntercepts) {
				return false
			}
		}

		if (flags & PT_ADDTHINGS) != 0 {
			if !P_BlockThingsIterator(mapx, mapy, PIT_AddThingIntercepts) {
				return false
			}
		}

		if mapx == xt2 && mapy == yt2 {
			break
		}

		if int(yintercept >> FRACBITS) == mapy {
			yintercept += ystep
			mapx += mapxstep
		} else if int(xintercept >> FRACBITS) == mapx {
			xintercept += xstep
			mapy += mapystep
		}
	}

	return P_TraverseIntercepts(trav, FRACUNIT)
}
