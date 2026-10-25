// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

STOPSPEED :: 0x1000
FRICTION  :: 0xe800

itemrespawnque: [ITEMQUESIZE]mapthing_t
itemrespawntime: [ITEMQUESIZE]int
iquehead: int
iquetail: int

P_SetMobjState :: proc(mobj: ^mobj_t, state_in: statenum_t) -> boolean {
	state := state_in
	for {
		if state == .S_NULL {
			mobj.state = nil
			P_RemoveMobj(mobj)
			return false
		}

		st := &states[state]
		mobj.state = st
		mobj.tics = st.tics
		mobj.sprite = st.sprite
		mobj.frame = st.frame

		if st.action.acp1 != nil {
			st.action.acp1(mobj)
		}

		state = st.nextstate
		if mobj.tics != 0 {
			break
		}
	}

	return true
}

P_ExplodeMissile :: proc(mo: ^mobj_t) {
	mo.momx = 0
	mo.momy = 0
	mo.momz = 0

	P_SetMobjState(mo, mobjinfo[mo.type].deathstate)

	mo.tics -= P_Random() & 3
	if mo.tics < 1 {
		mo.tics = 1
	}

	mo.flags &= ~int(MF_MISSILE)

	if mo.info.deathsound != .sfx_None {
		S_StartSound(mo, int(mo.info.deathsound))
	}
}

P_XYMovement :: proc(mo: ^mobj_t) {
	if mo.momx == 0 && mo.momy == 0 {
		if (mo.flags & MF_SKULLFLY) != 0 {
			mo.flags &= ~int(MF_SKULLFLY)
			mo.momx = 0
			mo.momy = 0
			mo.momz = 0

			P_SetMobjState(mo, mo.info.spawnstate)
		}
		return
	}

	player := mo.player

	if mo.momx > MAXMOVE {
		mo.momx = MAXMOVE
	} else if mo.momx < -MAXMOVE {
		mo.momx = -MAXMOVE
	}

	if mo.momy > MAXMOVE {
		mo.momy = MAXMOVE
	} else if mo.momy < -MAXMOVE {
		mo.momy = -MAXMOVE
	}

	xmove := mo.momx
	ymove := mo.momy

	for {
		ptryx, ptryy: fixed_t
		if xmove > MAXMOVE / 2 || ymove > MAXMOVE / 2 {
			ptryx = mo.x + xmove / 2
			ptryy = mo.y + ymove / 2
			xmove >>= 1
			ymove >>= 1
		} else {
			ptryx = mo.x + xmove
			ptryy = mo.y + ymove
			xmove = 0
			ymove = 0
		}

		if !P_TryMove(mo, ptryx, ptryy) {
			if mo.player != nil {
				P_SlideMove(mo)
			} else if (mo.flags & MF_MISSILE) != 0 {
				if ceilingline != nil && ceilingline.backsector != nil && int(ceilingline.backsector.ceilingpic) == skyflatnum {
					P_RemoveMobj(mo)
					return
				}
				P_ExplodeMissile(mo)
			} else {
				mo.momx = 0
				mo.momy = 0
			}
		}

		if xmove == 0 && ymove == 0 {
			break
		}
	}

	if player != nil && (player.cheats & CF_NOMOMENTUM) != 0 {
		mo.momx = 0
		mo.momy = 0
		return
	}

	if (mo.flags & (MF_MISSILE | MF_SKULLFLY)) != 0 {
		return
	}

	if mo.z > mo.floorz {
		return
	}

	if (mo.flags & MF_CORPSE) != 0 {
		if mo.momx > FRACUNIT / 4 || mo.momx < -FRACUNIT / 4 || mo.momy > FRACUNIT / 4 || mo.momy < -FRACUNIT / 4 {
			if mo.floorz != mo.subsector.sector.floorheight {
				return
			}
		}
	}

	if mo.momx > -STOPSPEED && mo.momx < STOPSPEED && mo.momy > -STOPSPEED && mo.momy < STOPSPEED &&
	   (player == nil || (player.cmd.forwardmove == 0 && player.cmd.sidemove == 0)) {
		if player != nil {
			state_idx := int(uintptr(player.mo.state) - uintptr(&states[0])) / size_of(state_t)
			if uint(state_idx - int(statenum_t.S_PLAY_RUN1)) < 4 {
				P_SetMobjState(player.mo, .S_PLAY)
			}
		}
		mo.momx = 0
		mo.momy = 0
	} else {
		mo.momx = FixedMul(mo.momx, FRICTION)
		mo.momy = FixedMul(mo.momy, FRICTION)
	}
}

P_ZMovement :: proc(mo: ^mobj_t) {
	if mo.player != nil && mo.z < mo.floorz {
		mo.player.viewheight -= mo.floorz - mo.z
		mo.player.deltaviewheight = (VIEWHEIGHT - mo.player.viewheight) >> 3
	}

	mo.z += mo.momz

	if (mo.flags & MF_FLOAT) != 0 && mo.target != nil {
		if (mo.flags & MF_SKULLFLY) == 0 && (mo.flags & MF_INFLOAT) == 0 {
			dist := P_AproxDistance(mo.x - mo.target.x, mo.y - mo.target.y)
			delta := (mo.target.z + (mo.height >> 1)) - mo.z

			if delta < 0 && dist < -(delta * 3) {
				mo.z -= FLOATSPEED
			} else if delta > 0 && dist < (delta * 3) {
				mo.z += FLOATSPEED
			}
		}
	}

	if mo.z <= mo.floorz {
		if (mo.flags & MF_SKULLFLY) != 0 {
			mo.momz = -mo.momz
		}

		if mo.momz < 0 {
			if mo.player != nil && mo.momz < -GRAVITY * 8 {
				mo.player.deltaviewheight = mo.momz >> 3
				S_StartSound(mo, int(sfxenum_t.sfx_oof))
			}
			mo.momz = 0
		}
		mo.z = mo.floorz

		if (mo.flags & MF_MISSILE) != 0 && (mo.flags & MF_NOCLIP) == 0 {
			P_ExplodeMissile(mo)
			return
		}
	} else if (mo.flags & MF_NOGRAVITY) == 0 {
		if mo.momz == 0 {
			mo.momz = -GRAVITY * 2
		} else {
			mo.momz -= GRAVITY
		}
	}

	if mo.z + mo.height > mo.ceilingz {
		if mo.momz > 0 {
			mo.momz = 0
		}
		mo.z = mo.ceilingz - mo.height

		if (mo.flags & MF_SKULLFLY) != 0 {
			mo.momz = -mo.momz
		}

		if (mo.flags & MF_MISSILE) != 0 && (mo.flags & MF_NOCLIP) == 0 {
			P_ExplodeMissile(mo)
			return
		}
	}
}

P_NightmareRespawn :: proc(mobj: ^mobj_t) {
	x := fixed_t(i32(mobj.spawnpoint.x) << FRACBITS)
	y := fixed_t(i32(mobj.spawnpoint.y) << FRACBITS)

	if !P_CheckPosition(mobj, x, y) {
		return
	}

	mo := P_SpawnMobj(mobj.x, mobj.y, mobj.subsector.sector.floorheight, .MT_TFOG)
	S_StartSound(mo, int(sfxenum_t.sfx_telept))

	ss := R_PointInSubsector(x, y)
	mo = P_SpawnMobj(x, y, ss.sector.floorheight, .MT_TFOG)
	S_StartSound(mo, int(sfxenum_t.sfx_telept))

	mthing := &mobj.spawnpoint

	z: fixed_t
	if (mobj.info.flags & MF_SPAWNCEILING) != 0 {
		z = ONCEILINGZ
	} else {
		z = ONFLOORZ
	}

	mo = P_SpawnMobj(x, y, z, mobj.type)
	mo.spawnpoint = mobj.spawnpoint
	mo.angle = ANG45 * u32(int(mthing.angle) / 45)

	if (int(mthing.options) & MTF_AMBUSH) != 0 {
		mo.flags |= MF_AMBUSH
	}

	mo.reactiontime = 18

	P_RemoveMobj(mobj)
}

P_MobjThinker :: proc(mobj_raw: rawptr) {
	mobj := (^mobj_t)(mobj_raw)

	if mobj.momx != 0 || mobj.momy != 0 || (mobj.flags & MF_SKULLFLY) != 0 {
		P_XYMovement(mobj)
		if mobj.thinker.function.int_val == -1 {
			return
		}
	}

	if mobj.z != mobj.floorz || mobj.momz != 0 {
		P_ZMovement(mobj)
		if mobj.thinker.function.int_val == -1 {
			return
		}
	}

	if mobj.tics != -1 {
		mobj.tics -= 1
		if mobj.tics == 0 {
			if !P_SetMobjState(mobj, mobj.state.nextstate) {
				return
			}
		}
	} else {
		if (mobj.flags & MF_COUNTKILL) == 0 {
			return
		}
		if !respawnmonsters {
			return
		}
		mobj.movecount += 1
		if mobj.movecount < 12 * 35 {
			return
		}
		if (leveltime & 31) != 0 {
			return
		}
		if P_Random() > 4 {
			return
		}
		P_NightmareRespawn(mobj)
	}
}

P_SpawnMobj :: proc(x: fixed_t, y: fixed_t, z: fixed_t, type: mobjtype_t) -> ^mobj_t {
	mobj := (^mobj_t)(Z_Malloc(size_of(mobj_t), PU_LEVEL, nil))
	libc.memset(mobj, 0, size_of(mobj_t))
	info := &mobjinfo[type]

	mobj.type = type
	mobj.info = info
	mobj.x = x
	mobj.y = y
	mobj.radius = fixed_t(info.radius)
	mobj.height = fixed_t(info.height)
	mobj.flags = info.flags
	mobj.health = info.spawnhealth

	if gameskill != .sk_nightmare {
		mobj.reactiontime = info.reactiontime
	}

	mobj.lastlook = P_Random() % MAXPLAYERS
	st := &states[info.spawnstate]

	mobj.state = st
	mobj.tics = st.tics
	mobj.sprite = st.sprite
	mobj.frame = st.frame

	P_SetThingPosition(mobj)

	mobj.floorz = mobj.subsector.sector.floorheight
	mobj.ceilingz = mobj.subsector.sector.ceilingheight

	if z == ONFLOORZ {
		mobj.z = mobj.floorz
	} else if z == ONCEILINGZ {
		mobj.z = mobj.ceilingz - fixed_t(mobj.info.height)
	} else {
		mobj.z = z
	}

	mobj.thinker.function.acp1 = P_MobjThinker
	P_AddThinker(&mobj.thinker)

	return mobj
}

P_RemoveMobj :: proc(mobj: ^mobj_t) {
	if (mobj.flags & MF_SPECIAL) != 0 && (mobj.flags & MF_DROPPED) == 0 && mobj.type != .MT_INV && mobj.type != .MT_INS {
		itemrespawnque[iquehead] = mobj.spawnpoint
		itemrespawntime[iquehead] = leveltime
		iquehead = (iquehead + 1) & (ITEMQUESIZE - 1)

		if iquehead == iquetail {
			iquetail = (iquetail + 1) & (ITEMQUESIZE - 1)
		}
	}

	P_UnsetThingPosition(mobj)
	S_StopSound(mobj)
	P_RemoveThinker((^thinker_t)(mobj))
}

P_RespawnSpecials :: proc() {
	if deathmatch != 2 {
		return
	}
	if iquehead == iquetail {
		return
	}
	if leveltime - itemrespawntime[iquetail] < 30 * 35 {
		return
	}

	mthing := &itemrespawnque[iquetail]

	x := fixed_t(i32(mthing.x) << FRACBITS)
	y := fixed_t(i32(mthing.y) << FRACBITS)

	ss := R_PointInSubsector(x, y)
	mo := P_SpawnMobj(x, y, ss.sector.floorheight, .MT_IFOG)
	S_StartSound(mo, int(sfxenum_t.sfx_itmbk))

	i := 0
	for ; i < NUMMOBJTYPES; i += 1 {
		if int(mthing.type) == mobjinfo[i].doomednum {
			break
		}
	}

	z: fixed_t
	if (mobjinfo[i].flags & MF_SPAWNCEILING) != 0 {
		z = ONCEILINGZ
	} else {
		z = ONFLOORZ
	}

	mo = P_SpawnMobj(x, y, z, mobjtype_t(i))
	mo.spawnpoint = mthing^
	mo.angle = ANG45 * u32(int(mthing.angle) / 45)

	iquetail = (iquetail + 1) & (ITEMQUESIZE - 1)
}

P_SpawnPlayer :: proc(mthing: ^mapthing_t) {
	if !playeringame[mthing.type - 1] {
		return
	}

	p := &players[mthing.type - 1]

	if p.playerstate == .PST_REBORN {
		G_PlayerReborn(int(mthing.type - 1))
	}

	x := fixed_t(i32(mthing.x) << FRACBITS)
	y := fixed_t(i32(mthing.y) << FRACBITS)
	z := ONFLOORZ
	mobj := P_SpawnMobj(x, y, z, .MT_PLAYER)

	if mthing.type > 1 {
		mobj.flags |= int(mthing.type - 1) << MF_TRANSSHIFT
	}

	mobj.angle = ANG45 * u32(int(mthing.angle) / 45)
	mobj.player = p
	mobj.health = p.health

	p.mo = mobj
	p.playerstate = .PST_LIVE
	p.refire = 0
	p.message = nil
	p.damagecount = 0
	p.bonuscount = 0
	p.extralight = 0
	p.fixedcolormap = 0
	p.viewheight = VIEWHEIGHT

	P_SetupPsprites(p)

	if deathmatch != 0 {
		for i := 0; i < NUMCARDS; i += 1 {
			p.cards[i] = true
		}
	}

	if int(mthing.type - 1) == consoleplayer {
		ST_Start()
		HU_Start()
	}
}

P_SpawnMapThing :: proc(mthing: ^mapthing_t) {
	if mthing.type == 11 {
		if uintptr(deathmatch_p) < uintptr(&deathmatchstarts[0]) + 10 * size_of(mapthing_t) {
			deathmatch_p[0] = mthing^
			deathmatch_p = &deathmatch_p[1]
		}
		return
	}

	if mthing.type <= 4 {
		playerstarts[mthing.type - 1] = mthing^
		if deathmatch == 0 {
			P_SpawnPlayer(mthing)
		}
		return
	}

	if !netgame && (mthing.options & 16) != 0 {
		return
	}

	bit: int
	if gameskill == .sk_baby {
		bit = 1
	} else if gameskill == .sk_nightmare {
		bit = 4
	} else {
		bit = 1 << (uint(gameskill) - 1)
	}

	if (int(mthing.options) & bit) == 0 {
		return
	}

	i := 0
	for ; i < NUMMOBJTYPES; i += 1 {
		if int(mthing.type) == mobjinfo[i].doomednum {
			break
		}
	}

	if i == NUMMOBJTYPES {
		I_Error("P_SpawnMapThing: Unknown type %i at (%i, %i)", i32(mthing.type), i32(mthing.x), i32(mthing.y))
	}

	if deathmatch != 0 && (mobjinfo[i].flags & MF_NOTDMATCH) != 0 {
		return
	}

	if nomonsters && (mobjtype_t(i) == .MT_SKULL || (mobjinfo[i].flags & MF_COUNTKILL) != 0) {
		return
	}

	x := fixed_t(i32(mthing.x) << FRACBITS)
	y := fixed_t(i32(mthing.y) << FRACBITS)

	z: fixed_t
	if (mobjinfo[i].flags & MF_SPAWNCEILING) != 0 {
		z = ONCEILINGZ
	} else {
		z = ONFLOORZ
	}

	mobj := P_SpawnMobj(x, y, z, mobjtype_t(i))
	mobj.spawnpoint = mthing^

	if mobj.tics > 0 {
		mobj.tics = 1 + (P_Random() % mobj.tics)
	}
	if (mobj.flags & MF_COUNTKILL) != 0 {
		totalkills += 1
	}
	if (mobj.flags & MF_COUNTITEM) != 0 {
		totalitems += 1
	}

	mobj.angle = ANG45 * u32(int(mthing.angle) / 45)
	if (int(mthing.options) & MTF_AMBUSH) != 0 {
		mobj.flags |= MF_AMBUSH
	}
}

P_SpawnPuff :: proc(x: fixed_t, y: fixed_t, z_in: fixed_t) {
	r1 := P_Random()
	r2 := P_Random()
	z := z_in + fixed_t(i32(r1 - r2) << 10)

	th := P_SpawnMobj(x, y, z, .MT_PUFF)
	th.momz = FRACUNIT
	th.tics -= P_Random() & 3

	if th.tics < 1 {
		th.tics = 1
	}

	if attackrange == MELEERANGE {
		P_SetMobjState(th, .S_PUFF3)
	}
}

P_SpawnBlood :: proc(x: fixed_t, y: fixed_t, z_in: fixed_t, damage: int) {
	r1 := P_Random()
	r2 := P_Random()
	z := z_in + fixed_t(i32(r1 - r2) << 10)
	th := P_SpawnMobj(x, y, z, .MT_BLOOD)
	th.momz = FRACUNIT * 2
	th.tics -= P_Random() & 3

	if th.tics < 1 {
		th.tics = 1
	}

	if damage <= 12 && damage >= 9 {
		P_SetMobjState(th, .S_BLOOD2)
	} else if damage < 9 {
		P_SetMobjState(th, .S_BLOOD3)
	}
}

P_CheckMissileSpawn :: proc(th: ^mobj_t) {
	th.tics -= P_Random() & 3
	if th.tics < 1 {
		th.tics = 1
	}

	th.x += (th.momx >> 1)
	th.y += (th.momy >> 1)
	th.z += (th.momz >> 1)

	if !P_TryMove(th, th.x, th.y) {
		P_ExplodeMissile(th)
	}
}

P_SpawnMissile :: proc(source: ^mobj_t, dest: ^mobj_t, type: mobjtype_t) -> ^mobj_t {
	th := P_SpawnMobj(source.x, source.y, source.z + 4 * 8 * FRACUNIT, type)

	if th.info.seesound != .sfx_None {
		S_StartSound(th, int(th.info.seesound))
	}

	th.target = source
	an := R_PointToAngle2(source.x, source.y, dest.x, dest.y)

	if (dest.flags & MF_SHADOW) != 0 {
		r1 := P_Random()
		r2 := P_Random()
		an += angle_t(i32(r1 - r2) << 20)
	}

	th.angle = an
	an >>= ANGLETOFINESHIFT
	th.momx = FixedMul(fixed_t(th.info.speed), finecosine[an])
	th.momy = FixedMul(fixed_t(th.info.speed), finesine[an])

	dist := int(P_AproxDistance(dest.x - source.x, dest.y - source.y))
	dist = dist / th.info.speed

	if dist < 1 {
		dist = 1
	}

	th.momz = (dest.z - source.z) / fixed_t(dist)
	P_CheckMissileSpawn(th)

	return th
}

P_SpawnPlayerMissile :: proc(source: ^mobj_t, type: mobjtype_t) {
	an := source.angle
	slope := P_AimLineAttack(source, an, 16 * 64 * FRACUNIT)

	if linetarget == nil {
		an += 1 << 26
		slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT)

		if linetarget == nil {
			an -= 2 << 26
			slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT)
		}

		if linetarget == nil {
			an = source.angle
			slope = 0
		}
	}

	x := source.x
	y := source.y
	z := source.z + 4 * 8 * FRACUNIT

	th := P_SpawnMobj(x, y, z, type)

	if th.info.seesound != .sfx_None {
		S_StartSound(th, int(th.info.seesound))
	}

	th.target = source
	th.angle = an
	th.momx = FixedMul(fixed_t(th.info.speed), finecosine[an >> ANGLETOFINESHIFT])
	th.momy = FixedMul(fixed_t(th.info.speed), finesine[an >> ANGLETOFINESHIFT])
	th.momz = FixedMul(fixed_t(th.info.speed), slope)

	P_CheckMissileSpawn(th)
}
