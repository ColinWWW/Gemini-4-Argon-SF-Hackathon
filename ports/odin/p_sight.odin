// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

sightzstart: fixed_t
topslope: fixed_t
bottomslope: fixed_t

strace: divline_t
t2x: fixed_t
t2y: fixed_t

sightcounts: [2]int

P_DivlineSide :: proc(x: fixed_t, y: fixed_t, node: ^divline_t) -> int {
	if node.dx == 0 {
		if x == node.x {
			return 2
		}
		if x <= node.x {
			return node.dy > 0 ? 1 : 0
		}
		return node.dy < 0 ? 1 : 0
	}

	if node.dy == 0 {
		if x == node.y {
			return 2
		}
		if y <= node.y {
			return node.dx < 0 ? 1 : 0
		}
		return node.dx > 0 ? 1 : 0
	}

	dx := x - node.x
	dy := y - node.y

	left := (node.dy >> FRACBITS) * (dx >> FRACBITS)
	right := (dy >> FRACBITS) * (node.dx >> FRACBITS)

	if right < left {
		return 0
	}
	if left == right {
		return 2
	}
	return 1
}

P_InterceptVector2 :: proc(v2: ^divline_t, v1: ^divline_t) -> fixed_t {
	den := FixedMul(v1.dy >> 8, v2.dx) - FixedMul(v1.dx >> 8, v2.dy)
	if den == 0 {
		return 0
	}
	num := FixedMul((v1.x - v2.x) >> 8, v1.dy) + FixedMul((v2.y - v1.y) >> 8, v1.dx)
	frac := FixedDiv(num, den)
	return frac
}

P_CrossSubsector :: proc(num: int) -> boolean {
	if num >= numsubsectors {
		I_Error("P_CrossSubsector: ss %i with numss = %i", i32(num), i32(numsubsectors))
	}

	sub := &subsectors[num]
	count := int(sub.numlines)
	seg := &segs[sub.firstline]

	for ; count != 0; count -= 1 {
		line := seg.linedef
		seg = &segs[int(uintptr(seg) - uintptr(&segs[0])) / size_of(seg_t) + 1]

		if line.validcount == validcount {
			continue
		}

		line.validcount = validcount

		v1 := line.v1
		v2 := line.v2
		s1 := P_DivlineSide(v1.x, v1.y, &strace)
		s2 := P_DivlineSide(v2.x, v2.y, &strace)

		if s1 == s2 {
			continue
		}

		divl := divline_t{
			x = v1.x,
			y = v1.y,
			dx = v2.x - v1.x,
			dy = v2.y - v1.y,
		}
		s1 = P_DivlineSide(strace.x, strace.y, &divl)
		s2 = P_DivlineSide(t2x, t2y, &divl)

		if s1 == s2 {
			continue
		}

		if (int(line.flags) & ML_TWOSIDED) == 0 {
			return false
		}

		cur_seg := &segs[int(uintptr(seg) - uintptr(&segs[0])) / size_of(seg_t) - 1]
		front := cur_seg.frontsector
		back := cur_seg.backsector

		if front.floorheight == back.floorheight && front.ceilingheight == back.ceilingheight {
			continue
		}

		loc_opentop: fixed_t
		if front.ceilingheight < back.ceilingheight {
			loc_opentop = front.ceilingheight
		} else {
			loc_opentop = back.ceilingheight
		}

		loc_openbottom: fixed_t
		if front.floorheight > back.floorheight {
			loc_openbottom = front.floorheight
		} else {
			loc_openbottom = back.floorheight
		}

		if loc_openbottom >= loc_opentop {
			return false
		}

		frac := P_InterceptVector2(&strace, &divl)

		if front.floorheight != back.floorheight {
			slope := FixedDiv(loc_openbottom - sightzstart, frac)
			if slope > bottomslope {
				bottomslope = slope
			}
		}

		if front.ceilingheight != back.ceilingheight {
			slope := FixedDiv(loc_opentop - sightzstart, frac)
			if slope < topslope {
				topslope = slope
			}
		}

		if topslope <= bottomslope {
			return false
		}
	}

	return true
}

P_CrossBSPNode :: proc(bspnum: int) -> boolean {
	if (bspnum & NF_SUBSECTOR) != 0 {
		if bspnum == -1 {
			return P_CrossSubsector(0)
		} else {
			return P_CrossSubsector(bspnum & (~int(NF_SUBSECTOR)))
		}
	}

	bsp := &nodes[bspnum]

	side := P_DivlineSide(strace.x, strace.y, (^divline_t)(bsp))
	if side == 2 {
		side = 0
	}

	if !P_CrossBSPNode(int(bsp.children[side])) {
		return false
	}

	if side == P_DivlineSide(t2x, t2y, (^divline_t)(bsp)) {
		return true
	}

	return P_CrossBSPNode(int(bsp.children[side ~ 1]))
}

P_CheckSight :: proc(t1: ^mobj_t, t2: ^mobj_t) -> boolean {
	s1 := int(uintptr(t1.subsector.sector) - uintptr(&sectors[0])) / size_of(sector_t)
	s2 := int(uintptr(t2.subsector.sector) - uintptr(&sectors[0])) / size_of(sector_t)
	pnum := s1 * numsectors + s2
	bytenum := pnum >> 3
	bitnum := byte(1 << uint(pnum & 7))

	if (rejectmatrix[bytenum] & bitnum) != 0 {
		sightcounts[0] += 1
		return false
	}

	sightcounts[1] += 1

	validcount += 1

	sightzstart = t1.z + t1.height - (t1.height >> 2)
	topslope = (t2.z + t2.height) - sightzstart
	bottomslope = t2.z - sightzstart

	strace.x = t1.x
	strace.y = t1.y
	t2x = t2.x
	t2y = t2.y
	strace.dx = t2.x - t1.x
	strace.dy = t2.y - t1.y

	return P_CrossBSPNode(numnodes - 1)
}
