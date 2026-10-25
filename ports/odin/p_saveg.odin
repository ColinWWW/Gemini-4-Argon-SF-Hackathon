// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

save_p: [^]byte

PADSAVEP :: proc() {
	offset := (4 - (uintptr(save_p) & 3)) & 3
	save_p = &save_p[offset]
}

P_ArchivePlayers :: proc() {
	for i := 0; i < MAXPLAYERS; i += 1 {
		if !playeringame[i] {
			continue
		}

		PADSAVEP()

		dest := (^player_t)(save_p)
		libc.memcpy(dest, &players[i], size_of(player_t))
		save_p = &save_p[size_of(player_t)]
		for j := 0; j < NUMPSPRITES; j += 1 {
			if dest.psprites[j].state != nil {
				idx := (uintptr(dest.psprites[j].state) - uintptr(&states[0])) / size_of(state_t)
				dest.psprites[j].state = (^state_t)(idx)
			}
		}
	}
}

P_UnArchivePlayers :: proc() {
	for i := 0; i < MAXPLAYERS; i += 1 {
		if !playeringame[i] {
			continue
		}

		PADSAVEP()

		libc.memcpy(&players[i], save_p, size_of(player_t))
		save_p = &save_p[size_of(player_t)]

		players[i].mo = nil
		players[i].message = nil
		players[i].attacker = nil

		for j := 0; j < NUMPSPRITES; j += 1 {
			if players[i].psprites[j].state != nil {
				idx := int(uintptr(players[i].psprites[j].state))
				players[i].psprites[j].state = &states[idx]
			}
		}
	}
}

P_ArchiveWorld :: proc() {
	put := ([^]i16)(save_p)

	for i := 0; i < numsectors; i += 1 {
		sec := &sectors[i]
		put[0] = i16(sec.floorheight >> FRACBITS); put = &put[1]
		put[0] = i16(sec.ceilingheight >> FRACBITS); put = &put[1]
		put[0] = sec.floorpic; put = &put[1]
		put[0] = sec.ceilingpic; put = &put[1]
		put[0] = sec.lightlevel; put = &put[1]
		put[0] = sec.special; put = &put[1]
		put[0] = sec.tag; put = &put[1]
	}

	for i := 0; i < numlines; i += 1 {
		li := &lines[i]
		put[0] = li.flags; put = &put[1]
		put[0] = li.special; put = &put[1]
		put[0] = li.tag; put = &put[1]
		for j := 0; j < 2; j += 1 {
			if li.sidenum[j] == -1 {
				continue
			}
			si := &sides[li.sidenum[j]]
			put[0] = i16(si.textureoffset >> FRACBITS); put = &put[1]
			put[0] = i16(si.rowoffset >> FRACBITS); put = &put[1]
			put[0] = si.toptexture; put = &put[1]
			put[0] = si.bottomtexture; put = &put[1]
			put[0] = si.midtexture; put = &put[1]
		}
	}

	save_p = ([^]byte)(put)
}

P_UnArchiveWorld :: proc() {
	get := ([^]i16)(save_p)

	for i := 0; i < numsectors; i += 1 {
		sec := &sectors[i]
		sec.floorheight = fixed_t(i32(get[0]) << FRACBITS); get = &get[1]
		sec.ceilingheight = fixed_t(i32(get[0]) << FRACBITS); get = &get[1]
		sec.floorpic = get[0]; get = &get[1]
		sec.ceilingpic = get[0]; get = &get[1]
		sec.lightlevel = get[0]; get = &get[1]
		sec.special = get[0]; get = &get[1]
		sec.tag = get[0]; get = &get[1]
		sec.specialdata = nil
		sec.soundtarget = nil
	}

	for i := 0; i < numlines; i += 1 {
		li := &lines[i]
		li.flags = get[0]; get = &get[1]
		li.special = get[0]; get = &get[1]
		li.tag = get[0]; get = &get[1]
		for j := 0; j < 2; j += 1 {
			if li.sidenum[j] == -1 {
				continue
			}
			si := &sides[li.sidenum[j]]
			si.textureoffset = fixed_t(i32(get[0]) << FRACBITS); get = &get[1]
			si.rowoffset = fixed_t(i32(get[0]) << FRACBITS); get = &get[1]
			si.toptexture = get[0]; get = &get[1]
			si.bottomtexture = get[0]; get = &get[1]
			si.midtexture = get[0]; get = &get[1]
		}
	}

	save_p = ([^]byte)(get)
}

tc_end  :: 0
tc_mobj :: 1

P_ArchiveThinkers :: proc() {
	for th := thinkercap.next; th != &thinkercap; th = th.next {
		if th.function.acp1 == P_MobjThinker {
			save_p[0] = tc_mobj
			save_p = &save_p[1]
			PADSAVEP()
			mobj := (^mobj_t)(save_p)
			libc.memcpy(mobj, th, size_of(mobj_t))
			save_p = &save_p[size_of(mobj_t)]
			mobj.state = (^state_t)((uintptr(mobj.state) - uintptr(&states[0])) / size_of(state_t))

			if mobj.player != nil {
				mobj.player = (^player_t)((uintptr(mobj.player) - uintptr(&players[0])) / size_of(player_t) + 1)
			}
			continue
		}
	}

	save_p[0] = tc_end
	save_p = &save_p[1]
}

P_UnArchiveThinkers :: proc() {
	currentthinker := thinkercap.next
	for currentthinker != &thinkercap {
		next := currentthinker.next
		if currentthinker.function.acp1 == P_MobjThinker {
			P_RemoveMobj((^mobj_t)(currentthinker))
		} else {
			Z_Free(currentthinker)
		}
		currentthinker = next
	}
	P_InitThinkers()

	for {
		tclass := save_p[0]
		save_p = &save_p[1]
		switch tclass {
		case tc_end:
			return
		case tc_mobj:
			PADSAVEP()
			mobj := (^mobj_t)(Z_Malloc(size_of(mobj_t), PU_LEVEL, nil))
			libc.memcpy(mobj, save_p, size_of(mobj_t))
			save_p = &save_p[size_of(mobj_t)]
			mobj.state = &states[int(uintptr(mobj.state))]
			mobj.target = nil
			if mobj.player != nil {
				pidx := int(uintptr(mobj.player)) - 1
				mobj.player = &players[pidx]
				mobj.player.mo = mobj
			}
			P_SetThingPosition(mobj)
			mobj.info = &mobjinfo[mobj.type]
			mobj.floorz = mobj.subsector.sector.floorheight
			mobj.ceilingz = mobj.subsector.sector.ceilingheight
			mobj.thinker.function.acp1 = P_MobjThinker
			P_AddThinker(&mobj.thinker)
		case:
			I_Error("Unknown tclass %i in savegame", i32(tclass))
		}
	}
}

tc_ceiling     :: 0
tc_door        :: 1
tc_floor       :: 2
tc_plat        :: 3
tc_flash       :: 4
tc_strobe      :: 5
tc_glow        :: 6
tc_endspecials :: 7

P_ArchiveSpecials :: proc() {
	for th := thinkercap.next; th != &thinkercap; th = th.next {
		if th.function.acv == nil {
			i := 0
			for ; i < MAXCEILINGS; i += 1 {
				if activeceilings[i] == (^ceiling_t)(th) {
					break
				}
			}
			if i < MAXCEILINGS {
				save_p[0] = tc_ceiling
				save_p = &save_p[1]
				PADSAVEP()
				ceiling := (^ceiling_t)(save_p)
				libc.memcpy(ceiling, th, size_of(ceiling_t))
				save_p = &save_p[size_of(ceiling_t)]
				ceiling.sector = (^sector_t)((uintptr(ceiling.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			}
			continue
		}

		if th.function.acp1 == T_MoveCeiling {
			save_p[0] = tc_ceiling
			save_p = &save_p[1]
			PADSAVEP()
			ceiling := (^ceiling_t)(save_p)
			libc.memcpy(ceiling, th, size_of(ceiling_t))
			save_p = &save_p[size_of(ceiling_t)]
			ceiling.sector = (^sector_t)((uintptr(ceiling.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}

		if th.function.acp1 == T_VerticalDoor {
			save_p[0] = tc_door
			save_p = &save_p[1]
			PADSAVEP()
			door := (^vldoor_t)(save_p)
			libc.memcpy(door, th, size_of(vldoor_t))
			save_p = &save_p[size_of(vldoor_t)]
			door.sector = (^sector_t)((uintptr(door.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}

		if th.function.acp1 == T_MoveFloor {
			save_p[0] = tc_floor
			save_p = &save_p[1]
			PADSAVEP()
			floor := (^floormove_t)(save_p)
			libc.memcpy(floor, th, size_of(floormove_t))
			save_p = &save_p[size_of(floormove_t)]
			floor.sector = (^sector_t)((uintptr(floor.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}

		if th.function.acp1 == T_PlatRaise {
			save_p[0] = tc_plat
			save_p = &save_p[1]
			PADSAVEP()
			plat := (^plat_t)(save_p)
			libc.memcpy(plat, th, size_of(plat_t))
			save_p = &save_p[size_of(plat_t)]
			plat.sector = (^sector_t)((uintptr(plat.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}

		if th.function.acp1 == T_LightFlash {
			save_p[0] = tc_flash
			save_p = &save_p[1]
			PADSAVEP()
			flash := (^lightflash_t)(save_p)
			libc.memcpy(flash, th, size_of(lightflash_t))
			save_p = &save_p[size_of(lightflash_t)]
			flash.sector = (^sector_t)((uintptr(flash.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}

		if th.function.acp1 == T_StrobeFlash {
			save_p[0] = tc_strobe
			save_p = &save_p[1]
			PADSAVEP()
			strobe := (^strobe_t)(save_p)
			libc.memcpy(strobe, th, size_of(strobe_t))
			save_p = &save_p[size_of(strobe_t)]
			strobe.sector = (^sector_t)((uintptr(strobe.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}

		if th.function.acp1 == T_Glow {
			save_p[0] = tc_glow
			save_p = &save_p[1]
			PADSAVEP()
			glow := (^glow_t)(save_p)
			libc.memcpy(glow, th, size_of(glow_t))
			save_p = &save_p[size_of(glow_t)]
			glow.sector = (^sector_t)((uintptr(glow.sector) - uintptr(&sectors[0])) / size_of(sector_t))
			continue
		}
	}

	save_p[0] = tc_endspecials
	save_p = &save_p[1]
}

P_UnArchiveSpecials :: proc() {
	for {
		tclass := save_p[0]
		save_p = &save_p[1]
		switch tclass {
		case tc_endspecials:
			return
		case tc_ceiling:
			PADSAVEP()
			ceiling := (^ceiling_t)(Z_Malloc(size_of(ceiling_t), PU_LEVEL, nil))
			libc.memcpy(ceiling, save_p, size_of(ceiling_t))
			save_p = &save_p[size_of(ceiling_t)]
			ceiling.sector = &sectors[int(uintptr(ceiling.sector))]
			ceiling.sector.specialdata = ceiling
			if ceiling.thinker.function.acp1 != nil {
				ceiling.thinker.function.acp1 = T_MoveCeiling
			}
			P_AddThinker(&ceiling.thinker)
			P_AddActiveCeiling(ceiling)
		case tc_door:
			PADSAVEP()
			door := (^vldoor_t)(Z_Malloc(size_of(vldoor_t), PU_LEVEL, nil))
			libc.memcpy(door, save_p, size_of(vldoor_t))
			save_p = &save_p[size_of(vldoor_t)]
			door.sector = &sectors[int(uintptr(door.sector))]
			door.sector.specialdata = door
			door.thinker.function.acp1 = T_VerticalDoor
			P_AddThinker(&door.thinker)
		case tc_floor:
			PADSAVEP()
			floor := (^floormove_t)(Z_Malloc(size_of(floormove_t), PU_LEVEL, nil))
			libc.memcpy(floor, save_p, size_of(floormove_t))
			save_p = &save_p[size_of(floormove_t)]
			floor.sector = &sectors[int(uintptr(floor.sector))]
			floor.sector.specialdata = floor
			floor.thinker.function.acp1 = T_MoveFloor
			P_AddThinker(&floor.thinker)
		case tc_plat:
			PADSAVEP()
			plat := (^plat_t)(Z_Malloc(size_of(plat_t), PU_LEVEL, nil))
			libc.memcpy(plat, save_p, size_of(plat_t))
			save_p = &save_p[size_of(plat_t)]
			plat.sector = &sectors[int(uintptr(plat.sector))]
			plat.sector.specialdata = plat
			if plat.thinker.function.acp1 != nil {
				plat.thinker.function.acp1 = T_PlatRaise
			}
			P_AddThinker(&plat.thinker)
			P_AddActivePlat(plat)
		case tc_flash:
			PADSAVEP()
			flash := (^lightflash_t)(Z_Malloc(size_of(lightflash_t), PU_LEVEL, nil))
			libc.memcpy(flash, save_p, size_of(lightflash_t))
			save_p = &save_p[size_of(lightflash_t)]
			flash.sector = &sectors[int(uintptr(flash.sector))]
			flash.thinker.function.acp1 = T_LightFlash
			P_AddThinker(&flash.thinker)
		case tc_strobe:
			PADSAVEP()
			strobe := (^strobe_t)(Z_Malloc(size_of(strobe_t), PU_LEVEL, nil))
			libc.memcpy(strobe, save_p, size_of(strobe_t))
			save_p = &save_p[size_of(strobe_t)]
			strobe.sector = &sectors[int(uintptr(strobe.sector))]
			strobe.thinker.function.acp1 = T_StrobeFlash
			P_AddThinker(&strobe.thinker)
		case tc_glow:
			PADSAVEP()
			glow := (^glow_t)(Z_Malloc(size_of(glow_t), PU_LEVEL, nil))
			libc.memcpy(glow, save_p, size_of(glow_t))
			save_p = &save_p[size_of(glow_t)]
			glow.sector = &sectors[int(uintptr(glow.sector))]
			glow.thinker.function.acp1 = T_Glow
			P_AddThinker(&glow.thinker)
		case:
			I_Error("P_UnarchiveSpecials:Unknown tclass %i in savegame", i32(tclass))
		}
	}
}
