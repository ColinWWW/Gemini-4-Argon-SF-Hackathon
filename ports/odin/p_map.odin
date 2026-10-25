// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

tmbbox: [4]fixed_t
tmthing: ^mobj_t
tmflags: int
tmx: fixed_t
tmy: fixed_t

floatok: boolean

tmfloorz: fixed_t
tmceilingz: fixed_t
tmdropoffz: fixed_t

ceilingline: ^line_t

MAXSPECIALCROSS :: 8
spechit: [MAXSPECIALCROSS]^line_t
numspechit: int

bestslidefrac: fixed_t
secondslidefrac: fixed_t

bestslideline: ^line_t
secondslideline: ^line_t

slidemo: ^mobj_t

tmxmove: fixed_t
tmymove: fixed_t

linetarget: ^mobj_t
shootthing: ^mobj_t
shootz: fixed_t
la_damage: int
attackrange: fixed_t
aimslope: fixed_t

usething: ^mobj_t

bombsource: ^mobj_t
bombspot: ^mobj_t
bombdamage: int

crushchange: boolean
nofit: boolean

PIT_StompThing :: proc(thing: ^mobj_t) -> boolean {
	if (thing.flags & MF_SHOOTABLE) == 0 {
		return true
	}

	blockdist := thing.radius + tmthing.radius

	if abs(thing.x - tmx) >= blockdist || abs(thing.y - tmy) >= blockdist {
		return true
	}

	if thing == tmthing {
		return true
	}

	if tmthing.player == nil && gamemap != 30 {
		return false
	}

	P_DamageMobj(thing, tmthing, tmthing, 10000)

	return true
}

P_TeleportMove :: proc(thing: ^mobj_t, x: fixed_t, y: fixed_t) -> boolean {
	tmthing = thing
	tmflags = thing.flags

	tmx = x
	tmy = y

	tmbbox[BOXTOP] = y + tmthing.radius
	tmbbox[BOXBOTTOM] = y - tmthing.radius
	tmbbox[BOXRIGHT] = x + tmthing.radius
	tmbbox[BOXLEFT] = x - tmthing.radius

	newsubsec := R_PointInSubsector(x, y)
	ceilingline = nil

	tmfloorz = newsubsec.sector.floorheight
	tmdropoffz = tmfloorz
	tmceilingz = newsubsec.sector.ceilingheight

	validcount += 1
	numspechit = 0

	xl := int((tmbbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT)
	xh := int((tmbbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT)
	yl := int((tmbbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT)
	yh := int((tmbbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT)

	for bx := xl; bx <= xh; bx += 1 {
		for by := yl; by <= yh; by += 1 {
			if !P_BlockThingsIterator(bx, by, PIT_StompThing) {
				return false
			}
		}
	}

	P_UnsetThingPosition(thing)

	thing.floorz = tmfloorz
	thing.ceilingz = tmceilingz
	thing.x = x
	thing.y = y

	P_SetThingPosition(thing)

	return true
}

PIT_CheckLine :: proc(ld: ^line_t) -> boolean {
	if tmbbox[BOXRIGHT] <= ld.bbox[BOXLEFT] ||
	   tmbbox[BOXLEFT] >= ld.bbox[BOXRIGHT] ||
	   tmbbox[BOXTOP] <= ld.bbox[BOXBOTTOM] ||
	   tmbbox[BOXBOTTOM] >= ld.bbox[BOXTOP] {
		return true
	}

	if P_BoxOnLineSide(&tmbbox[0], ld) != -1 {
		return true
	}

	if ld.backsector == nil {
		return false
	}

	if (tmthing.flags & MF_MISSILE) == 0 {
		if (int(ld.flags) & ML_BLOCKING) != 0 {
			return false
		}
		if tmthing.player == nil && (int(ld.flags) & ML_BLOCKMONSTERS) != 0 {
			return false
		}
	}

	P_LineOpening(ld)

	if opentop < tmceilingz {
		tmceilingz = opentop
		ceilingline = ld
	}

	if openbottom > tmfloorz {
		tmfloorz = openbottom
	}

	if lowfloor < tmdropoffz {
		tmdropoffz = lowfloor
	}

	if ld.special != 0 {
		spechit[numspechit] = ld
		numspechit += 1
	}

	return true
}

PIT_CheckThing :: proc(thing: ^mobj_t) -> boolean {
	if (thing.flags & (MF_SOLID | MF_SPECIAL | MF_SHOOTABLE)) == 0 {
		return true
	}

	blockdist := thing.radius + tmthing.radius

	if abs(thing.x - tmx) >= blockdist || abs(thing.y - tmy) >= blockdist {
		return true
	}

	if thing == tmthing {
		return true
	}

	if (tmthing.flags & MF_SKULLFLY) != 0 {
		damage := ((P_Random() % 8) + 1) * tmthing.info.damage
		P_DamageMobj(thing, tmthing, tmthing, damage)

		tmthing.flags &= ~int(MF_SKULLFLY)
		tmthing.momx = 0
		tmthing.momy = 0
		tmthing.momz = 0

		P_SetMobjState(tmthing, tmthing.info.spawnstate)

		return false
	}

	if (tmthing.flags & MF_MISSILE) != 0 {
		if tmthing.z > thing.z + thing.height {
			return true
		}
		if tmthing.z + tmthing.height < thing.z {
			return true
		}

		if tmthing.target != nil && (tmthing.target.type == thing.type ||
		   (tmthing.target.type == .MT_KNIGHT && thing.type == .MT_BRUISER) ||
		   (tmthing.target.type == .MT_BRUISER && thing.type == .MT_KNIGHT)) {
			if thing == tmthing.target {
				return true
			}
			if thing.type != .MT_PLAYER {
				return false
			}
		}

		if (thing.flags & MF_SHOOTABLE) == 0 {
			return (thing.flags & MF_SOLID) == 0
		}

		damage := ((P_Random() % 8) + 1) * tmthing.info.damage
		P_DamageMobj(thing, tmthing, tmthing.target, damage)

		return false
	}

	if (thing.flags & MF_SPECIAL) != 0 {
		solid := (thing.flags & MF_SOLID) != 0
		if (tmflags & MF_PICKUP) != 0 {
			P_TouchSpecialThing(thing, tmthing)
		}
		return !solid
	}

	return (thing.flags & MF_SOLID) == 0
}

P_CheckPosition :: proc(thing: ^mobj_t, x: fixed_t, y: fixed_t) -> boolean {
	tmthing = thing
	tmflags = thing.flags

	tmx = x
	tmy = y

	tmbbox[BOXTOP] = y + tmthing.radius
	tmbbox[BOXBOTTOM] = y - tmthing.radius
	tmbbox[BOXRIGHT] = x + tmthing.radius
	tmbbox[BOXLEFT] = x - tmthing.radius

	newsubsec := R_PointInSubsector(x, y)
	ceilingline = nil

	tmfloorz = newsubsec.sector.floorheight
	tmdropoffz = tmfloorz
	tmceilingz = newsubsec.sector.ceilingheight

	validcount += 1
	numspechit = 0

	if (tmflags & MF_NOCLIP) != 0 {
		return true
	}

	xl := int((tmbbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT)
	xh := int((tmbbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT)
	yl := int((tmbbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT)
	yh := int((tmbbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT)

	for bx := xl; bx <= xh; bx += 1 {
		for by := yl; by <= yh; by += 1 {
			if !P_BlockThingsIterator(bx, by, PIT_CheckThing) {
				return false
			}
		}
	}

	xl = int((tmbbox[BOXLEFT] - bmaporgx) >> MAPBLOCKSHIFT)
	xh = int((tmbbox[BOXRIGHT] - bmaporgx) >> MAPBLOCKSHIFT)
	yl = int((tmbbox[BOXBOTTOM] - bmaporgy) >> MAPBLOCKSHIFT)
	yh = int((tmbbox[BOXTOP] - bmaporgy) >> MAPBLOCKSHIFT)

	for bx := xl; bx <= xh; bx += 1 {
		for by := yl; by <= yh; by += 1 {
			if !P_BlockLinesIterator(bx, by, PIT_CheckLine) {
				return false
			}
		}
	}

	return true
}

P_TryMove :: proc(thing: ^mobj_t, x: fixed_t, y: fixed_t) -> boolean {
	floatok = false
	if !P_CheckPosition(thing, x, y) {
		return false
	}

	if (thing.flags & MF_NOCLIP) == 0 {
		if tmceilingz - tmfloorz < thing.height {
			return false
		}

		floatok = true

		if (thing.flags & MF_TELEPORT) == 0 && tmceilingz - thing.z < thing.height {
			return false
		}

		if (thing.flags & MF_TELEPORT) == 0 && tmfloorz - thing.z > 24 * FRACUNIT {
			return false
		}

		if (thing.flags & (MF_DROPOFF | MF_FLOAT)) == 0 && tmfloorz - tmdropoffz > 24 * FRACUNIT {
			return false
		}
	}

	P_UnsetThingPosition(thing)

	oldx := thing.x
	oldy := thing.y
	thing.floorz = tmfloorz
	thing.ceilingz = tmceilingz
	thing.x = x
	thing.y = y

	P_SetThingPosition(thing)

	if (thing.flags & (MF_TELEPORT | MF_NOCLIP)) == 0 {
		for numspechit > 0 {
			numspechit -= 1
			ld := spechit[numspechit]
			side := P_PointOnLineSide(thing.x, thing.y, ld)
			oldside := P_PointOnLineSide(oldx, oldy, ld)
			if side != oldside {
				if ld.special != 0 {
					linenum := int(uintptr(ld) - uintptr(&lines[0])) / size_of(line_t)
					P_CrossSpecialLine(linenum, oldside, thing)
				}
			}
		}
	}

	return true
}

P_ThingHeightClip :: proc(thing: ^mobj_t) -> boolean {
	onfloor := (thing.z == thing.floorz)

	P_CheckPosition(thing, thing.x, thing.y)

	thing.floorz = tmfloorz
	thing.ceilingz = tmceilingz

	if onfloor {
		thing.z = thing.floorz
	} else {
		if thing.z + thing.height > thing.ceilingz {
			thing.z = thing.ceilingz - thing.height
		}
	}

	if thing.ceilingz - thing.floorz < thing.height {
		return false
	}

	return true
}

P_HitSlideLine :: proc(ld: ^line_t) {
	if ld.slopetype == .ST_HORIZONTAL {
		tmymove = 0
		return
	}

	if ld.slopetype == .ST_VERTICAL {
		tmxmove = 0
		return
	}

	side := P_PointOnLineSide(slidemo.x, slidemo.y, ld)

	lineangle := R_PointToAngle2(0, 0, ld.dx, ld.dy)

	if side == 1 {
		lineangle += ANG180
	}

	moveangle := R_PointToAngle2(0, 0, tmxmove, tmymove)
	deltaangle := moveangle - lineangle

	if deltaangle > ANG180 {
		deltaangle += ANG180
	}

	lineangle >>= ANGLETOFINESHIFT
	deltaangle >>= ANGLETOFINESHIFT

	movelen := P_AproxDistance(tmxmove, tmymove)
	newlen := FixedMul(movelen, finecosine[deltaangle])

	tmxmove = FixedMul(newlen, finecosine[lineangle])
	tmymove = FixedMul(newlen, finesine[lineangle])
}

PTR_SlideTraverse :: proc(in_ptr: ^intercept_t) -> boolean {
	if !in_ptr.isaline {
		I_Error("PTR_SlideTraverse: not a line?")
	}

	li := in_ptr.d.line

	isblocking := false
	if (int(li.flags) & ML_TWOSIDED) == 0 {
		if P_PointOnLineSide(slidemo.x, slidemo.y, li) != 0 {
			return true
		}
		isblocking = true
	}

	if !isblocking {
		P_LineOpening(li)
		if openrange < slidemo.height || opentop - slidemo.z < slidemo.height || openbottom - slidemo.z > 24 * FRACUNIT {
			isblocking = true
		} else {
			return true
		}
	}

	if in_ptr.frac < bestslidefrac {
		secondslidefrac = bestslidefrac
		secondslideline = bestslideline
		bestslidefrac = in_ptr.frac
		bestslideline = li
	}

	return false
}

P_SlideMove :: proc(mo: ^mobj_t) {
	slidemo = mo
	hitcount := 0

	for {
		hitcount += 1
		if hitcount == 3 {
			if !P_TryMove(mo, mo.x, mo.y + mo.momy) {
				P_TryMove(mo, mo.x + mo.momx, mo.y)
			}
			return
		}

		leadx, trailx: fixed_t
		if mo.momx > 0 {
			leadx = mo.x + mo.radius
			trailx = mo.x - mo.radius
		} else {
			leadx = mo.x - mo.radius
			trailx = mo.x + mo.radius
		}

		leady, traily: fixed_t
		if mo.momy > 0 {
			leady = mo.y + mo.radius
			traily = mo.y - mo.radius
		} else {
			leady = mo.y - mo.radius
			traily = mo.y + mo.radius
		}

		bestslidefrac = FRACUNIT + 1

		P_PathTraverse(leadx, leady, leadx + mo.momx, leady + mo.momy, PT_ADDLINES, PTR_SlideTraverse)
		P_PathTraverse(trailx, leady, trailx + mo.momx, leady + mo.momy, PT_ADDLINES, PTR_SlideTraverse)
		P_PathTraverse(leadx, traily, leadx + mo.momx, traily + mo.momy, PT_ADDLINES, PTR_SlideTraverse)

		if bestslidefrac == FRACUNIT + 1 {
			if !P_TryMove(mo, mo.x, mo.y + mo.momy) {
				P_TryMove(mo, mo.x + mo.momx, mo.y)
			}
			return
		}

		bestslidefrac -= 0x800
		if bestslidefrac > 0 {
			newx := FixedMul(mo.momx, bestslidefrac)
			newy := FixedMul(mo.momy, bestslidefrac)

			if !P_TryMove(mo, mo.x + newx, mo.y + newy) {
				if !P_TryMove(mo, mo.x, mo.y + mo.momy) {
					P_TryMove(mo, mo.x + mo.momx, mo.y)
				}
				return
			}
		}

		bestslidefrac = FRACUNIT - (bestslidefrac + 0x800)

		if bestslidefrac > FRACUNIT {
			bestslidefrac = FRACUNIT
		}

		if bestslidefrac <= 0 {
			return
		}

		tmxmove = FixedMul(mo.momx, bestslidefrac)
		tmymove = FixedMul(mo.momy, bestslidefrac)

		P_HitSlideLine(bestslideline)

		mo.momx = tmxmove
		mo.momy = tmymove

		if P_TryMove(mo, mo.x + tmxmove, mo.y + tmymove) {
			break
		}
	}
}

PTR_AimTraverse :: proc(in_ptr: ^intercept_t) -> boolean {
	if in_ptr.isaline {
		li := in_ptr.d.line

		if (int(li.flags) & ML_TWOSIDED) == 0 {
			return false
		}

		P_LineOpening(li)

		if openbottom >= opentop {
			return false
		}

		dist := FixedMul(attackrange, in_ptr.frac)

		if li.frontsector.floorheight != li.backsector.floorheight {
			slope := FixedDiv(openbottom - shootz, dist)
			if slope > bottomslope {
				bottomslope = slope
			}
		}

		if li.frontsector.ceilingheight != li.backsector.ceilingheight {
			slope := FixedDiv(opentop - shootz, dist)
			if slope < topslope {
				topslope = slope
			}
		}

		if topslope <= bottomslope {
			return false
		}

		return true
	}

	th := in_ptr.d.thing
	if th == shootthing {
		return true
	}

	if (th.flags & MF_SHOOTABLE) == 0 {
		return true
	}

	dist := FixedMul(attackrange, in_ptr.frac)
	thingtopslope := FixedDiv(th.z + th.height - shootz, dist)

	if thingtopslope < bottomslope {
		return true
	}

	thingbottomslope := FixedDiv(th.z - shootz, dist)

	if thingbottomslope > topslope {
		return true
	}

	if thingtopslope > topslope {
		thingtopslope = topslope
	}

	if thingbottomslope < bottomslope {
		thingbottomslope = bottomslope
	}

	aimslope = (thingtopslope + thingbottomslope) / 2
	linetarget = th

	return false
}

PTR_ShootTraverse :: proc(in_ptr: ^intercept_t) -> boolean {
	if in_ptr.isaline {
		li := in_ptr.d.line

		if li.special != 0 {
			P_ShootSpecialLine(shootthing, li)
		}

		hitline := false
		if (int(li.flags) & ML_TWOSIDED) == 0 {
			hitline = true
		}

		if !hitline {
			P_LineOpening(li)
			dist := FixedMul(attackrange, in_ptr.frac)

			if li.frontsector.floorheight != li.backsector.floorheight {
				slope := FixedDiv(openbottom - shootz, dist)
				if slope > aimslope {
					hitline = true
				}
			}

			if !hitline && li.frontsector.ceilingheight != li.backsector.ceilingheight {
				slope := FixedDiv(opentop - shootz, dist)
				if slope < aimslope {
					hitline = true
				}
			}

			if !hitline {
				return true
			}
		}

		frac := in_ptr.frac - FixedDiv(4 * FRACUNIT, attackrange)
		x := trace.x + FixedMul(trace.dx, frac)
		y := trace.y + FixedMul(trace.dy, frac)
		z := shootz + FixedMul(aimslope, FixedMul(frac, attackrange))

		if int(li.frontsector.ceilingpic) == skyflatnum {
			if z > li.frontsector.ceilingheight {
				return false
			}
			if li.backsector != nil && int(li.backsector.ceilingpic) == skyflatnum {
				return false
			}
		}

		P_SpawnPuff(x, y, z)
		return false
	}

	th := in_ptr.d.thing
	if th == shootthing {
		return true
	}

	if (th.flags & MF_SHOOTABLE) == 0 {
		return true
	}

	dist := FixedMul(attackrange, in_ptr.frac)
	thingtopslope := FixedDiv(th.z + th.height - shootz, dist)

	if thingtopslope < aimslope {
		return true
	}

	thingbottomslope := FixedDiv(th.z - shootz, dist)

	if thingbottomslope > aimslope {
		return true
	}

	frac := in_ptr.frac - FixedDiv(10 * FRACUNIT, attackrange)

	x := trace.x + FixedMul(trace.dx, frac)
	y := trace.y + FixedMul(trace.dy, frac)
	z := shootz + FixedMul(aimslope, FixedMul(frac, attackrange))

	if (in_ptr.d.thing.flags & MF_NOBLOOD) != 0 {
		P_SpawnPuff(x, y, z)
	} else {
		P_SpawnBlood(x, y, z, la_damage)
	}

	if la_damage != 0 {
		P_DamageMobj(th, shootthing, shootthing, la_damage)
	}

	return false
}

P_AimLineAttack :: proc(t1: ^mobj_t, angle_in: angle_t, distance: fixed_t) -> fixed_t {
	angle := angle_in >> ANGLETOFINESHIFT
	shootthing = t1

	x2 := t1.x + (distance >> FRACBITS) * finecosine[angle]
	y2 := t1.y + (distance >> FRACBITS) * finesine[angle]
	shootz = t1.z + (t1.height >> 1) + 8 * FRACUNIT

	topslope = 100 * FRACUNIT / 160
	bottomslope = -100 * FRACUNIT / 160

	attackrange = distance
	linetarget = nil

	P_PathTraverse(t1.x, t1.y, x2, y2, PT_ADDLINES | PT_ADDTHINGS, PTR_AimTraverse)

	if linetarget != nil {
		return aimslope
	}

	return 0
}

P_LineAttack :: proc(t1: ^mobj_t, angle_in: angle_t, distance: fixed_t, slope: fixed_t, damage: int) {
	angle := angle_in >> ANGLETOFINESHIFT
	shootthing = t1
	la_damage = damage
	x2 := t1.x + (distance >> FRACBITS) * finecosine[angle]
	y2 := t1.y + (distance >> FRACBITS) * finesine[angle]
	shootz = t1.z + (t1.height >> 1) + 8 * FRACUNIT
	attackrange = distance
	aimslope = slope

	P_PathTraverse(t1.x, t1.y, x2, y2, PT_ADDLINES | PT_ADDTHINGS, PTR_ShootTraverse)
}

PTR_UseTraverse :: proc(in_ptr: ^intercept_t) -> boolean {
	if in_ptr.d.line.special == 0 {
		P_LineOpening(in_ptr.d.line)
		if openrange <= 0 {
			S_StartSound(usething, int(sfxenum_t.sfx_noway))
			return false
		}
		return true
	}

	side := 0
	if P_PointOnLineSide(usething.x, usething.y, in_ptr.d.line) == 1 {
		side = 1
	}

	P_UseSpecialLine(usething, in_ptr.d.line, side)

	return false
}

P_UseLines :: proc(player: ^player_t) {
	usething = player.mo

	angle := player.mo.angle >> ANGLETOFINESHIFT

	x1 := player.mo.x
	y1 := player.mo.y
	x2 := x1 + (USERANGE >> FRACBITS) * finecosine[angle]
	y2 := y1 + (USERANGE >> FRACBITS) * finesine[angle]

	P_PathTraverse(x1, y1, x2, y2, PT_ADDLINES, PTR_UseTraverse)
}

PIT_RadiusAttack :: proc(thing: ^mobj_t) -> boolean {
	if (thing.flags & MF_SHOOTABLE) == 0 {
		return true
	}

	if thing.type == .MT_CYBORG || thing.type == .MT_SPIDER {
		return true
	}

	dx := abs(thing.x - bombspot.x)
	dy := abs(thing.y - bombspot.y)

	dist := dx > dy ? dx : dy
	dist = (dist - thing.radius) >> FRACBITS

	if dist < 0 {
		dist = 0
	}

	if int(dist) >= bombdamage {
		return true
	}

	if P_CheckSight(thing, bombspot) {
		P_DamageMobj(thing, bombspot, bombsource, bombdamage - int(dist))
	}

	return true
}

P_RadiusAttack :: proc(spot: ^mobj_t, source: ^mobj_t, damage: int) {
	dist := fixed_t(i32(damage + MAXRADIUS) << FRACBITS)
	yh := int((spot.y + dist - bmaporgy) >> MAPBLOCKSHIFT)
	yl := int((spot.y - dist - bmaporgy) >> MAPBLOCKSHIFT)
	xh := int((spot.x + dist - bmaporgx) >> MAPBLOCKSHIFT)
	xl := int((spot.x - dist - bmaporgx) >> MAPBLOCKSHIFT)
	bombspot = spot
	bombsource = source
	bombdamage = damage

	for y := yl; y <= yh; y += 1 {
		for x := xl; x <= xh; x += 1 {
			P_BlockThingsIterator(x, y, PIT_RadiusAttack)
		}
	}
}

PIT_ChangeSector :: proc(thing: ^mobj_t) -> boolean {
	if P_ThingHeightClip(thing) {
		return true
	}

	if thing.health <= 0 {
		P_SetMobjState(thing, .S_GIBS)
		thing.flags &= ~int(MF_SOLID)
		thing.height = 0
		thing.radius = 0
		return true
	}

	if (thing.flags & MF_DROPPED) != 0 {
		P_RemoveMobj(thing)
		return true
	}

	if (thing.flags & MF_SHOOTABLE) == 0 {
		return true
	}

	nofit = true

	if crushchange && (leveltime & 3) == 0 {
		P_DamageMobj(thing, nil, nil, 10)
		mo := P_SpawnMobj(thing.x, thing.y, thing.z + thing.height / 2, .MT_BLOOD)
		r1 := P_Random()
		r2 := P_Random()
		mo.momx = fixed_t(i32(r1 - r2) << 12)
		r3 := P_Random()
		r4 := P_Random()
		mo.momy = fixed_t(i32(r3 - r4) << 12)
	}

	return true
}

P_ChangeSector :: proc(sector: ^sector_t, crunch: boolean) -> boolean {
	nofit = false
	crushchange = crunch

	for x := sector.blockbox[BOXLEFT]; x <= sector.blockbox[BOXRIGHT]; x += 1 {
		for y := sector.blockbox[BOXBOTTOM]; y <= sector.blockbox[BOXTOP]; y += 1 {
			P_BlockThingsIterator(x, y, PIT_ChangeSector)
		}
	}

	return nofit
}
