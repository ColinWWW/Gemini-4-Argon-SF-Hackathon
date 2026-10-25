// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

dirtype_t :: enum i32 {
	DI_EAST,
	DI_NORTHEAST,
	DI_NORTH,
	DI_NORTHWEST,
	DI_WEST,
	DI_SOUTHWEST,
	DI_SOUTH,
	DI_SOUTHEAST,
	DI_NODIR,
	NUMDIRS,
}

opposite := [9]dirtype_t{
	.DI_WEST, .DI_SOUTHWEST, .DI_SOUTH, .DI_SOUTHEAST,
	.DI_EAST, .DI_NORTHEAST, .DI_NORTH, .DI_NORTHWEST, .DI_NODIR,
}

diags := [4]dirtype_t{
	.DI_NORTHWEST, .DI_NORTHEAST, .DI_SOUTHWEST, .DI_SOUTHEAST,
}

soundtarget: ^mobj_t

xspeed := [8]fixed_t{FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000, 0, 47000}
yspeed := [8]fixed_t{0, 47000, FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000}

TRACEANGLE: angle_t = 0xc000000

corpsehit: ^mobj_t
vileobj: ^mobj_t
viletryx: fixed_t
viletryy: fixed_t

braintargets: [32]^mobj_t
numbraintargets: int
braintargeton: int
brain_easy: int = 0

P_RecursiveSound :: proc(sec: ^sector_t, soundblocks: int) {
	if sec.validcount == validcount && sec.soundtraversed <= soundblocks + 1 {
		return
	}

	sec.validcount = validcount
	sec.soundtraversed = soundblocks + 1
	sec.soundtarget = soundtarget

	for i := 0; i < sec.linecount; i += 1 {
		check := sec.lines[i]
		if (int(check.flags) & ML_TWOSIDED) == 0 {
			continue
		}

		P_LineOpening(check)

		if openrange <= 0 {
			continue
		}

		other: ^sector_t
		if sides[check.sidenum[0]].sector == sec {
			other = sides[check.sidenum[1]].sector
		} else {
			other = sides[check.sidenum[0]].sector
		}

		if (int(check.flags) & ML_SOUNDBLOCK) != 0 {
			if soundblocks == 0 {
				P_RecursiveSound(other, 1)
			}
		} else {
			P_RecursiveSound(other, soundblocks)
		}
	}
}

P_NoiseAlert :: proc(target: ^mobj_t, emmiter: ^mobj_t) {
	soundtarget = target
	validcount += 1
	P_RecursiveSound(emmiter.subsector.sector, 0)
}

P_CheckMeleeRange :: proc(actor: ^mobj_t) -> boolean {
	if actor.target == nil {
		return false
	}

	pl := actor.target
	dist := P_AproxDistance(pl.x - actor.x, pl.y - actor.y)

	if dist >= MELEERANGE - 20 * FRACUNIT + fixed_t(pl.info.radius) {
		return false
	}

	if !P_CheckSight(actor, actor.target) {
		return false
	}

	return true
}

P_CheckMissileRange :: proc(actor: ^mobj_t) -> boolean {
	if !P_CheckSight(actor, actor.target) {
		return false
	}

	if (actor.flags & MF_JUSTHIT) != 0 {
		actor.flags &= ~int(MF_JUSTHIT)
		return true
	}

	if actor.reactiontime != 0 {
		return false
	}

	dist := int(P_AproxDistance(actor.x - actor.target.x, actor.y - actor.target.y) - 64 * FRACUNIT)

	if actor.info.meleestate == .S_NULL {
		dist -= 128 * FRACUNIT
	}

	dist >>= 16

	if actor.type == .MT_VILE {
		if dist > 14 * 64 {
			return false
		}
	}

	if actor.type == .MT_UNDEAD {
		if dist < 196 {
			return false
		}
		dist >>= 1
	}

	if actor.type == .MT_CYBORG || actor.type == .MT_SPIDER || actor.type == .MT_SKULL {
		dist >>= 1
	}

	if dist > 200 {
		dist = 200
	}

	if actor.type == .MT_CYBORG && dist > 160 {
		dist = 160
	}

	if P_Random() < dist {
		return false
	}

	return true
}

P_Move :: proc(actor: ^mobj_t) -> boolean {
	if actor.movedir == int(dirtype_t.DI_NODIR) {
		return false
	}

	if uint(actor.movedir) >= 8 {
		I_Error("Weird actor->movedir!")
	}

	tryx := actor.x + fixed_t(actor.info.speed) * xspeed[actor.movedir]
	tryy := actor.y + fixed_t(actor.info.speed) * yspeed[actor.movedir]

	try_ok := P_TryMove(actor, tryx, tryy)

	if !try_ok {
		if (actor.flags & MF_FLOAT) != 0 && floatok {
			if actor.z < tmfloorz {
				actor.z += FLOATSPEED
			} else {
				actor.z -= FLOATSPEED
			}
			actor.flags |= MF_INFLOAT
			return true
		}

		if numspechit == 0 {
			return false
		}

		actor.movedir = int(dirtype_t.DI_NODIR)
		good := false
		for numspechit > 0 {
			numspechit -= 1
			ld := spechit[numspechit]
			if P_UseSpecialLine(actor, ld, 0) {
				good = true
			}
		}
		return good
	} else {
		actor.flags &= ~int(MF_INFLOAT)
	}

	if (actor.flags & MF_FLOAT) == 0 {
		actor.z = actor.floorz
	}
	return true
}

P_TryWalk :: proc(actor: ^mobj_t) -> boolean {
	if !P_Move(actor) {
		return false
	}
	actor.movecount = P_Random() & 15
	return true
}

P_NewChaseDir :: proc(actor: ^mobj_t) {
	d: [3]dirtype_t

	if actor.target == nil {
		I_Error("P_NewChaseDir: called with no target")
	}

	olddir := dirtype_t(actor.movedir)
	turnaround := opposite[olddir]

	deltax := actor.target.x - actor.x
	deltay := actor.target.y - actor.y

	if deltax > 10 * FRACUNIT {
		d[1] = .DI_EAST
	} else if deltax < -10 * FRACUNIT {
		d[1] = .DI_WEST
	} else {
		d[1] = .DI_NODIR
	}

	if deltay < -10 * FRACUNIT {
		d[2] = .DI_SOUTH
	} else if deltay > 10 * FRACUNIT {
		d[2] = .DI_NORTH
	} else {
		d[2] = .DI_NODIR
	}

	if d[1] != .DI_NODIR && d[2] != .DI_NODIR {
		idx := ((deltay < 0 ? 1 : 0) << 1) + (deltax > 0 ? 1 : 0)
		actor.movedir = int(diags[idx])
		if dirtype_t(actor.movedir) != turnaround && P_TryWalk(actor) {
			return
		}
	}

	if P_Random() > 200 || abs(deltay) > abs(deltax) {
		tdir := d[1]
		d[1] = d[2]
		d[2] = tdir
	}

	if d[1] == turnaround {
		d[1] = .DI_NODIR
	}
	if d[2] == turnaround {
		d[2] = .DI_NODIR
	}

	if d[1] != .DI_NODIR {
		actor.movedir = int(d[1])
		if P_TryWalk(actor) {
			return
		}
	}

	if d[2] != .DI_NODIR {
		actor.movedir = int(d[2])
		if P_TryWalk(actor) {
			return
		}
	}

	if olddir != .DI_NODIR {
		actor.movedir = int(olddir)
		if P_TryWalk(actor) {
			return
		}
	}

	if (P_Random() & 1) != 0 {
		for tdir := int(dirtype_t.DI_EAST); tdir <= int(dirtype_t.DI_SOUTHEAST); tdir += 1 {
			if dirtype_t(tdir) != turnaround {
				actor.movedir = tdir
				if P_TryWalk(actor) {
					return
				}
			}
		}
	} else {
		for tdir := int(dirtype_t.DI_SOUTHEAST); tdir != (int(dirtype_t.DI_EAST) - 1); tdir -= 1 {
			if dirtype_t(tdir) != turnaround {
				actor.movedir = tdir
				if P_TryWalk(actor) {
					return
				}
			}
		}
	}

	if turnaround != .DI_NODIR {
		actor.movedir = int(turnaround)
		if P_TryWalk(actor) {
			return
		}
	}

	actor.movedir = int(dirtype_t.DI_NODIR)
}

P_LookForPlayers :: proc(actor: ^mobj_t, allaround: boolean) -> boolean {
	c := 0
	stop := (actor.lastlook - 1) & 3

	for ; ; actor.lastlook = (actor.lastlook + 1) & 3 {
		if !playeringame[actor.lastlook] {
			continue
		}

		old_c := c
		c += 1
		if old_c == 2 || actor.lastlook == stop {
			return false
		}

		player := &players[actor.lastlook]

		if player.health <= 0 {
			continue
		}

		if !P_CheckSight(actor, player.mo) {
			continue
		}

		if !allaround {
			an := R_PointToAngle2(actor.x, actor.y, player.mo.x, player.mo.y) - actor.angle
			if an > ANG90 && an < ANG270 {
				dist := P_AproxDistance(player.mo.x - actor.x, player.mo.y - actor.y)
				if dist > MELEERANGE {
					continue
				}
			}
		}

		actor.target = player.mo
		return true
	}
}

A_KeenDie :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	junk: line_t

	A_Fall(mo)

	for th := thinkercap.next; th != &thinkercap; th = th.next {
		if th.function.acp1 != P_MobjThinker {
			continue
		}

		mo2 := (^mobj_t)(th)
		if mo2 != mo && mo2.type == mo.type && mo2.health > 0 {
			return
		}
	}

	junk.tag = 666
	EV_DoDoor(&junk, .open)
}

A_Look :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	actor.threshold = 0
	targ := actor.subsector.sector.soundtarget

	seeyou := false
	if targ != nil && (targ.flags & MF_SHOOTABLE) != 0 {
		actor.target = targ
		if (actor.flags & MF_AMBUSH) != 0 {
			if P_CheckSight(actor, actor.target) {
				seeyou = true
			}
		} else {
			seeyou = true
		}
	}

	if !seeyou {
		if !P_LookForPlayers(actor, false) {
			return
		}
	}

	if actor.info.seesound != .sfx_None {
		sound: int
		#partial switch actor.info.seesound {
		case .sfx_posit1, .sfx_posit2, .sfx_posit3:
			sound = int(sfxenum_t.sfx_posit1) + P_Random() % 3
		case .sfx_bgsit1, .sfx_bgsit2:
			sound = int(sfxenum_t.sfx_bgsit1) + P_Random() % 2
		case:
			sound = int(actor.info.seesound)
		}

		if actor.type == .MT_SPIDER || actor.type == .MT_CYBORG {
			S_StartSound(nil, sound)
		} else {
			S_StartSound(actor, sound)
		}
	}

	P_SetMobjState(actor, actor.info.seestate)
}

A_Chase :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)

	if actor.reactiontime != 0 {
		actor.reactiontime -= 1
	}

	if actor.threshold != 0 {
		if actor.target == nil || actor.target.health <= 0 {
			actor.threshold = 0
		} else {
			actor.threshold -= 1
		}
	}

	if actor.movedir < 8 {
		actor.angle &= (7 << 29)
		delta := i32(actor.angle - (angle_t(actor.movedir) << 29))
		if delta > 0 {
			actor.angle -= ANG90 / 2
		} else if delta < 0 {
			actor.angle += ANG90 / 2
		}
	}

	if actor.target == nil || (actor.target.flags & MF_SHOOTABLE) == 0 {
		if P_LookForPlayers(actor, true) {
			return
		}
		P_SetMobjState(actor, actor.info.spawnstate)
		return
	}

	if (actor.flags & MF_JUSTATTACKED) != 0 {
		actor.flags &= ~int(MF_JUSTATTACKED)
		if gameskill != .sk_nightmare && !fastparm {
			P_NewChaseDir(actor)
		}
		return
	}

	if actor.info.meleestate != .S_NULL && P_CheckMeleeRange(actor) {
		if actor.info.attacksound != .sfx_None {
			S_StartSound(actor, int(actor.info.attacksound))
		}
		P_SetMobjState(actor, actor.info.meleestate)
		return
	}

	if actor.info.missilestate != .S_NULL {
		if !(gameskill < .sk_nightmare && !fastparm && actor.movecount != 0) {
			if P_CheckMissileRange(actor) {
				P_SetMobjState(actor, actor.info.missilestate)
				actor.flags |= MF_JUSTATTACKED
				return
			}
		}
	}

	if netgame && actor.threshold == 0 && !P_CheckSight(actor, actor.target) {
		if P_LookForPlayers(actor, true) {
			return
		}
	}

	actor.movecount -= 1
	if actor.movecount < 0 || !P_Move(actor) {
		P_NewChaseDir(actor)
	}

	if actor.info.activesound != .sfx_None && P_Random() < 3 {
		S_StartSound(actor, int(actor.info.activesound))
	}
}

A_FaceTarget :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	actor.flags &= ~int(MF_AMBUSH)

	actor.angle = R_PointToAngle2(actor.x, actor.y, actor.target.x, actor.target.y)

	if (actor.target.flags & MF_SHADOW) != 0 {
		r1 := P_Random()
		r2 := P_Random()
		actor.angle += angle_t(i32(r1 - r2) << 21)
	}
}

A_PosAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)
	angle := actor.angle
	slope := P_AimLineAttack(actor, angle, MISSILERANGE)

	S_StartSound(actor, int(sfxenum_t.sfx_pistol))
	r1 := P_Random()
	r2 := P_Random()
	angle += angle_t(i32(r1 - r2) << 20)
	damage := ((P_Random() % 5) + 1) * 3
	P_LineAttack(actor, angle, MISSILERANGE, slope, damage)
}

A_SPosAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	S_StartSound(actor, int(sfxenum_t.sfx_shotgn))
	A_FaceTarget(actor)
	bangle := actor.angle
	slope := P_AimLineAttack(actor, bangle, MISSILERANGE)

	for i := 0; i < 3; i += 1 {
		r1 := P_Random()
		r2 := P_Random()
		angle := bangle + angle_t(i32(r1 - r2) << 20)
		damage := ((P_Random() % 5) + 1) * 3
		P_LineAttack(actor, angle, MISSILERANGE, slope, damage)
	}
}

A_CPosAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	S_StartSound(actor, int(sfxenum_t.sfx_shotgn))
	A_FaceTarget(actor)
	bangle := actor.angle
	slope := P_AimLineAttack(actor, bangle, MISSILERANGE)

	r1 := P_Random()
	r2 := P_Random()
	angle := bangle + angle_t(i32(r1 - r2) << 20)
	damage := ((P_Random() % 5) + 1) * 3
	P_LineAttack(actor, angle, MISSILERANGE, slope, damage)
}

A_CPosRefire :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_FaceTarget(actor)

	if P_Random() < 40 {
		return
	}

	if actor.target == nil || actor.target.health <= 0 || !P_CheckSight(actor, actor.target) {
		P_SetMobjState(actor, actor.info.seestate)
	}
}

A_SpidRefire :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_FaceTarget(actor)

	if P_Random() < 10 {
		return
	}

	if actor.target == nil || actor.target.health <= 0 || !P_CheckSight(actor, actor.target) {
		P_SetMobjState(actor, actor.info.seestate)
	}
}

A_BspiAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}
	A_FaceTarget(actor)
	P_SpawnMissile(actor, actor.target, .MT_ARACHPLAZ)
}

A_TroopAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)
	if P_CheckMeleeRange(actor) {
		S_StartSound(actor, int(sfxenum_t.sfx_claw))
		damage := (P_Random() % 8 + 1) * 3
		P_DamageMobj(actor.target, actor, actor, damage)
		return
	}

	P_SpawnMissile(actor, actor.target, .MT_TROOPSHOT)
}

A_SargAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)
	if P_CheckMeleeRange(actor) {
		damage := ((P_Random() % 10) + 1) * 4
		P_DamageMobj(actor.target, actor, actor, damage)
	}
}

A_HeadAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)
	if P_CheckMeleeRange(actor) {
		damage := (P_Random() % 6 + 1) * 10
		P_DamageMobj(actor.target, actor, actor, damage)
		return
	}

	P_SpawnMissile(actor, actor.target, .MT_HEADSHOT)
}

A_CyberAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}
	A_FaceTarget(actor)
	P_SpawnMissile(actor, actor.target, .MT_ROCKET)
}

A_BruisAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	if P_CheckMeleeRange(actor) {
		S_StartSound(actor, int(sfxenum_t.sfx_claw))
		damage := (P_Random() % 8 + 1) * 10
		P_DamageMobj(actor.target, actor, actor, damage)
		return
	}

	P_SpawnMissile(actor, actor.target, .MT_BRUISERSHOT)
}

A_SkelMissile :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)
	actor.z += 16 * FRACUNIT
	mo := P_SpawnMissile(actor, actor.target, .MT_TRACER)
	actor.z -= 16 * FRACUNIT

	mo.x += mo.momx
	mo.y += mo.momy
	mo.tracer = actor.target
}

A_Tracer :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)

	if (gametic & 3) != 0 {
		return
	}

	P_SpawnPuff(actor.x, actor.y, actor.z)

	th := P_SpawnMobj(actor.x - actor.momx, actor.y - actor.momy, actor.z, .MT_SMOKE)

	th.momz = FRACUNIT
	th.tics -= P_Random() & 3
	if th.tics < 1 {
		th.tics = 1
	}

	dest := actor.tracer
	if dest == nil || dest.health <= 0 {
		return
	}

	exact := R_PointToAngle2(actor.x, actor.y, dest.x, dest.y)

	if exact != actor.angle {
		if exact - actor.angle > 0x80000000 {
			actor.angle -= TRACEANGLE
			if exact - actor.angle < 0x80000000 {
				actor.angle = exact
			}
		} else {
			actor.angle += TRACEANGLE
			if exact - actor.angle > 0x80000000 {
				actor.angle = exact
			}
		}
	}

	exact = actor.angle >> ANGLETOFINESHIFT
	actor.momx = FixedMul(fixed_t(actor.info.speed), finecosine[exact])
	actor.momy = FixedMul(fixed_t(actor.info.speed), finesine[exact])

	dist := int(P_AproxDistance(dest.x - actor.x, dest.y - actor.y))
	dist = dist / actor.info.speed

	if dist < 1 {
		dist = 1
	}
	slope := (dest.z + 40 * FRACUNIT - actor.z) / fixed_t(dist)

	if slope < actor.momz {
		actor.momz -= FRACUNIT / 8
	} else {
		actor.momz += FRACUNIT / 8
	}
}

A_SkelWhoosh :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}
	A_FaceTarget(actor)
	S_StartSound(actor, int(sfxenum_t.sfx_skeswg))
}

A_SkelFist :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)

	if P_CheckMeleeRange(actor) {
		damage := ((P_Random() % 10) + 1) * 6
		S_StartSound(actor, int(sfxenum_t.sfx_skepch))
		P_DamageMobj(actor.target, actor, actor, damage)
	}
}

PIT_VileCheck :: proc(thing: ^mobj_t) -> boolean {
	if (thing.flags & MF_CORPSE) == 0 {
		return true
	}
	if thing.tics != -1 {
		return true
	}
	if thing.info.raisestate == .S_NULL {
		return true
	}

	maxdist := fixed_t(thing.info.radius + mobjinfo[mobjtype_t.MT_VILE].radius)

	if abs(thing.x - viletryx) > maxdist || abs(thing.y - viletryy) > maxdist {
		return true
	}

	corpsehit = thing
	corpsehit.momx = 0
	corpsehit.momy = 0
	corpsehit.height <<= 2
	check := P_CheckPosition(corpsehit, corpsehit.x, corpsehit.y)
	corpsehit.height >>= 2

	if !check {
		return true
	}

	return false
}

A_VileChase :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)

	if actor.movedir != int(dirtype_t.DI_NODIR) {
		viletryx = actor.x + fixed_t(actor.info.speed) * xspeed[actor.movedir]
		viletryy = actor.y + fixed_t(actor.info.speed) * yspeed[actor.movedir]

		xl := int((viletryx - bmaporgx - MAXRADIUS * 2) >> MAPBLOCKSHIFT)
		xh := int((viletryx - bmaporgx + MAXRADIUS * 2) >> MAPBLOCKSHIFT)
		yl := int((viletryy - bmaporgy - MAXRADIUS * 2) >> MAPBLOCKSHIFT)
		yh := int((viletryy - bmaporgy + MAXRADIUS * 2) >> MAPBLOCKSHIFT)

		vileobj = actor
		for bx := xl; bx <= xh; bx += 1 {
			for by := yl; by <= yh; by += 1 {
				if !P_BlockThingsIterator(bx, by, PIT_VileCheck) {
					temp := actor.target
					actor.target = corpsehit
					A_FaceTarget(actor)
					actor.target = temp

					P_SetMobjState(actor, .S_VILE_HEAL1)
					S_StartSound(corpsehit, int(sfxenum_t.sfx_slop))
					info := corpsehit.info

					P_SetMobjState(corpsehit, info.raisestate)
					corpsehit.height <<= 2
					corpsehit.flags = info.flags
					corpsehit.health = info.spawnhealth
					corpsehit.target = nil

					return
				}
			}
		}
	}

	A_Chase(actor)
}

A_VileStart :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	S_StartSound(actor, int(sfxenum_t.sfx_vilatk))
}

A_StartFire :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	S_StartSound(actor, int(sfxenum_t.sfx_flamst))
	A_Fire(actor)
}

A_FireCrackle :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	S_StartSound(actor, int(sfxenum_t.sfx_flame))
	A_Fire(actor)
}

A_Fire :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	dest := actor.tracer
	if dest == nil {
		return
	}

	if !P_CheckSight(actor.target, dest) {
		return
	}

	an := dest.angle >> ANGLETOFINESHIFT

	P_UnsetThingPosition(actor)
	actor.x = dest.x + FixedMul(24 * FRACUNIT, finecosine[an])
	actor.y = dest.y + FixedMul(24 * FRACUNIT, finesine[an])
	actor.z = dest.z
	P_SetThingPosition(actor)
}

A_VileTarget :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)

	// Preserving original bug: actor->target->x passed for both x and y!
	fog := P_SpawnMobj(actor.target.x, actor.target.x, actor.target.z, .MT_FIRE)

	actor.tracer = fog
	fog.target = actor
	fog.tracer = actor.target
	A_Fire(fog)
}

A_VileAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	A_FaceTarget(actor)

	if !P_CheckSight(actor, actor.target) {
		return
	}

	S_StartSound(actor, int(sfxenum_t.sfx_barexp))
	P_DamageMobj(actor.target, actor, actor, 20)
	actor.target.momz = 1000 * FRACUNIT / fixed_t(actor.target.info.mass)

	an := actor.angle >> ANGLETOFINESHIFT

	fire := actor.tracer
	if fire == nil {
		return
	}

	fire.x = actor.target.x - FixedMul(24 * FRACUNIT, finecosine[an])
	fire.y = actor.target.y - FixedMul(24 * FRACUNIT, finesine[an])
	P_RadiusAttack(fire, actor, 70)
}

FATSPREAD :: ANG90 / 8

A_FatRaise :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_FaceTarget(actor)
	S_StartSound(actor, int(sfxenum_t.sfx_manatk))
}

A_FatAttack1 :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_FaceTarget(actor)
	actor.angle += FATSPREAD
	P_SpawnMissile(actor, actor.target, .MT_FATSHOT)

	mo := P_SpawnMissile(actor, actor.target, .MT_FATSHOT)
	mo.angle += FATSPREAD
	an := mo.angle >> ANGLETOFINESHIFT
	mo.momx = FixedMul(fixed_t(mo.info.speed), finecosine[an])
	mo.momy = FixedMul(fixed_t(mo.info.speed), finesine[an])
}

A_FatAttack2 :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_FaceTarget(actor)
	actor.angle -= FATSPREAD
	P_SpawnMissile(actor, actor.target, .MT_FATSHOT)

	mo := P_SpawnMissile(actor, actor.target, .MT_FATSHOT)
	mo.angle -= FATSPREAD * 2
	an := mo.angle >> ANGLETOFINESHIFT
	mo.momx = FixedMul(fixed_t(mo.info.speed), finecosine[an])
	mo.momy = FixedMul(fixed_t(mo.info.speed), finesine[an])
}

A_FatAttack3 :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_FaceTarget(actor)

	mo := P_SpawnMissile(actor, actor.target, .MT_FATSHOT)
	mo.angle -= FATSPREAD / 2
	an := mo.angle >> ANGLETOFINESHIFT
	mo.momx = FixedMul(fixed_t(mo.info.speed), finecosine[an])
	mo.momy = FixedMul(fixed_t(mo.info.speed), finesine[an])

	mo = P_SpawnMissile(actor, actor.target, .MT_FATSHOT)
	mo.angle += FATSPREAD / 2
	an = mo.angle >> ANGLETOFINESHIFT
	mo.momx = FixedMul(fixed_t(mo.info.speed), finecosine[an])
	mo.momy = FixedMul(fixed_t(mo.info.speed), finesine[an])
}

SKULLSPEED :: 20 * FRACUNIT

A_SkullAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}

	dest := actor.target
	actor.flags |= MF_SKULLFLY

	S_StartSound(actor, int(actor.info.attacksound))
	A_FaceTarget(actor)
	an := actor.angle >> ANGLETOFINESHIFT
	actor.momx = FixedMul(SKULLSPEED, finecosine[an])
	actor.momy = FixedMul(SKULLSPEED, finesine[an])
	dist := int(P_AproxDistance(dest.x - actor.x, dest.y - actor.y))
	dist = dist / SKULLSPEED

	if dist < 1 {
		dist = 1
	}
	actor.momz = (dest.z + (dest.height >> 1) - actor.z) / fixed_t(dist)
}

A_PainShootSkull :: proc(actor: ^mobj_t, angle: angle_t) {
	count := 0

	currentthinker := thinkercap.next
	for currentthinker != &thinkercap {
		if currentthinker.function.acp1 == P_MobjThinker && ((^mobj_t)(currentthinker)).type == .MT_SKULL {
			count += 1
		}
		currentthinker = currentthinker.next
	}

	if count > 20 {
		return
	}

	an := angle >> ANGLETOFINESHIFT
	prestep := fixed_t(4 * FRACUNIT + 3 * (actor.info.radius + mobjinfo[mobjtype_t.MT_SKULL].radius) / 2)

	x := actor.x + FixedMul(prestep, finecosine[an])
	y := actor.y + FixedMul(prestep, finesine[an])
	z := actor.z + 8 * FRACUNIT

	newmobj := P_SpawnMobj(x, y, z, .MT_SKULL)

	if !P_TryMove(newmobj, newmobj.x, newmobj.y) {
		P_DamageMobj(newmobj, actor, actor, 10000)
		return
	}

	newmobj.target = actor.target
	A_SkullAttack(newmobj)
}

A_PainAttack :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.target == nil {
		return
	}
	A_FaceTarget(actor)
	A_PainShootSkull(actor, actor.angle)
}

A_PainDie :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	A_Fall(actor)
	A_PainShootSkull(actor, actor.angle + ANG90)
	A_PainShootSkull(actor, actor.angle + ANG180)
	A_PainShootSkull(actor, actor.angle + ANG270)
}

A_Scream :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	sound: int

	#partial switch actor.info.deathsound {
	case .sfx_None:
		return
	case .sfx_podth1, .sfx_podth2, .sfx_podth3:
		sound = int(sfxenum_t.sfx_podth1) + P_Random() % 3
	case .sfx_bgdth1, .sfx_bgdth2:
		sound = int(sfxenum_t.sfx_bgdth1) + P_Random() % 2
	case:
		sound = int(actor.info.deathsound)
	}

	if actor.type == .MT_SPIDER || actor.type == .MT_CYBORG {
		S_StartSound(nil, sound)
	} else {
		S_StartSound(actor, sound)
	}
}

A_XScream :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	S_StartSound(actor, int(sfxenum_t.sfx_slop))
}

A_Pain :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	if actor.info.painsound != .sfx_None {
		S_StartSound(actor, int(actor.info.painsound))
	}
}

A_Fall :: proc(actor_raw: rawptr) {
	actor := (^mobj_t)(actor_raw)
	actor.flags &= ~int(MF_SOLID)
}

A_Explode :: proc(thingy_raw: rawptr) {
	thingy := (^mobj_t)(thingy_raw)
	P_RadiusAttack(thingy, thingy.target, 128)
}

A_BossDeath :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	junk: line_t

	if gamemode == .commercial {
		if gamemap != 7 {
			return
		}
		if mo.type != .MT_FATSO && mo.type != .MT_BABY {
			return
		}
	} else {
		switch gameepisode {
		case 1:
			if gamemap != 8 { return }
			if mo.type != .MT_BRUISER { return }
		case 2:
			if gamemap != 8 { return }
			if mo.type != .MT_CYBORG { return }
		case 3:
			if gamemap != 8 { return }
			if mo.type != .MT_SPIDER { return }
		case 4:
			switch gamemap {
			case 6:
				if mo.type != .MT_CYBORG { return }
			case 8:
				if mo.type != .MT_SPIDER { return }
			case:
				return
			}
		case:
			if gamemap != 8 { return }
		}
	}

	i := 0
	for ; i < MAXPLAYERS; i += 1 {
		if playeringame[i] && players[i].health > 0 {
			break
		}
	}

	if i == MAXPLAYERS {
		return
	}

	for th := thinkercap.next; th != &thinkercap; th = th.next {
		if th.function.acp1 != P_MobjThinker {
			continue
		}
		mo2 := (^mobj_t)(th)
		if mo2 != mo && mo2.type == mo.type && mo2.health > 0 {
			return
		}
	}

	if gamemode == .commercial {
		if gamemap == 7 {
			if mo.type == .MT_FATSO {
				junk.tag = 666
				EV_DoFloor(&junk, .lowerFloorToLowest)
				return
			}
			if mo.type == .MT_BABY {
				junk.tag = 667
				EV_DoFloor(&junk, .raiseToTexture)
				return
			}
		}
	} else {
		switch gameepisode {
		case 1:
			junk.tag = 666
			EV_DoFloor(&junk, .lowerFloorToLowest)
			return
		case 4:
			switch gamemap {
			case 6:
				junk.tag = 666
				EV_DoDoor(&junk, .blazeOpen)
				return
			case 8:
				junk.tag = 666
				EV_DoFloor(&junk, .lowerFloorToLowest)
				return
			}
		}
	}

	G_ExitLevel()
}

A_Hoof :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	S_StartSound(mo, int(sfxenum_t.sfx_hoof))
	A_Chase(mo)
}

A_Metal :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	S_StartSound(mo, int(sfxenum_t.sfx_metal))
	A_Chase(mo)
}

A_BabyMetal :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	S_StartSound(mo, int(sfxenum_t.sfx_bspwlk))
	A_Chase(mo)
}

A_OpenShotgun2 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_dbopn))
}

A_LoadShotgun2 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_dbload))
}

A_CloseShotgun2 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_dbcls))
	A_ReFire(player_raw, psp_raw)
}

A_BrainAwake :: proc(mo_raw: rawptr) {
	numbraintargets = 0
	braintargeton = 0

	for thinker := thinkercap.next; thinker != &thinkercap; thinker = thinker.next {
		if thinker.function.acp1 != P_MobjThinker {
			continue
		}
		m := (^mobj_t)(thinker)
		if m.type == .MT_BOSSTARGET {
			braintargets[numbraintargets] = m
			numbraintargets += 1
		}
	}

	S_StartSound(nil, int(sfxenum_t.sfx_bossit))
}

A_BrainPain :: proc(mo_raw: rawptr) {
	S_StartSound(nil, int(sfxenum_t.sfx_bospn))
}

A_BrainScream :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)

	for x := mo.x - 196 * FRACUNIT; x < mo.x + 320 * FRACUNIT; x += FRACUNIT * 8 {
		y := mo.y - 320 * FRACUNIT
		z := fixed_t(128 + P_Random() * 2 * FRACUNIT)
		th := P_SpawnMobj(x, y, z, .MT_ROCKET)
		th.momz = fixed_t(P_Random() * 512)

		P_SetMobjState(th, .S_BRAINEXPLODE1)

		th.tics -= P_Random() & 7
		if th.tics < 1 {
			th.tics = 1
		}
	}

	S_StartSound(nil, int(sfxenum_t.sfx_bosdth))
}

A_BrainExplode :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	r1 := P_Random()
	r2 := P_Random()
	x := mo.x + fixed_t((r1 - r2) * 2048)
	y := mo.y
	z := fixed_t(128 + P_Random() * 2 * FRACUNIT)
	th := P_SpawnMobj(x, y, z, .MT_ROCKET)
	th.momz = fixed_t(P_Random() * 512)

	P_SetMobjState(th, .S_BRAINEXPLODE1)

	th.tics -= P_Random() & 7
	if th.tics < 1 {
		th.tics = 1
	}
}

A_BrainDie :: proc(mo_raw: rawptr) {
	G_ExitLevel()
}

A_BrainSpit :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	brain_easy ~= 1
	if gameskill <= .sk_easy && brain_easy == 0 {
		return
	}

	targ := braintargets[braintargeton]
	braintargeton = (braintargeton + 1) % numbraintargets

	newmobj := P_SpawnMissile(mo, targ, .MT_SPAWNSHOT)
	newmobj.target = targ
	newmobj.reactiontime = int((targ.y - mo.y) / newmobj.momy) / newmobj.state.tics

	S_StartSound(nil, int(sfxenum_t.sfx_bospit))
}

A_SpawnSound :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	S_StartSound(mo, int(sfxenum_t.sfx_boscub))
	A_SpawnFly(mo)
}

A_SpawnFly :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	mo.reactiontime -= 1
	if mo.reactiontime != 0 {
		return
	}

	targ := mo.target

	fog := P_SpawnMobj(targ.x, targ.y, targ.z, .MT_SPAWNFIRE)
	S_StartSound(fog, int(sfxenum_t.sfx_telept))

	r := P_Random()

	type: mobjtype_t
	if r < 50 {
		type = .MT_TROOP
	} else if r < 90 {
		type = .MT_SERGEANT
	} else if r < 120 {
		type = .MT_SHADOWS
	} else if r < 130 {
		type = .MT_PAIN
	} else if r < 160 {
		type = .MT_HEAD
	} else if r < 162 {
		type = .MT_VILE
	} else if r < 172 {
		type = .MT_UNDEAD
	} else if r < 192 {
		type = .MT_BABY
	} else if r < 222 {
		type = .MT_FATSO
	} else if r < 246 {
		type = .MT_KNIGHT
	} else {
		type = .MT_BRUISER
	}

	newmobj := P_SpawnMobj(targ.x, targ.y, targ.z, type)
	if P_LookForPlayers(newmobj, true) {
		P_SetMobjState(newmobj, newmobj.info.seestate)
	}

	P_TeleportMove(newmobj, newmobj.x, newmobj.y)

	P_RemoveMobj(mo)
}

A_PlayerScream :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)
	sound := int(sfxenum_t.sfx_pldeth)

	if gamemode == .commercial && mo.health < -50 {
		sound = int(sfxenum_t.sfx_pdiehi)
	}

	S_StartSound(mo, sound)
}
