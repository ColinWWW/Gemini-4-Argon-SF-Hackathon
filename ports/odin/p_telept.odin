// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

EV_Teleport :: proc(line: ^line_t, side: int, thing: ^mobj_t) -> int {
	if (thing.flags & MF_MISSILE) != 0 {
		return 0
	}
	if side == 1 {
		return 0
	}

	tag := line.tag
	for i := 0; i < numsectors; i += 1 {
		if sectors[i].tag == tag {
			for thinker := thinkercap.next; thinker != &thinkercap; thinker = thinker.next {
				if thinker.function.acp1 != P_MobjThinker {
					continue
				}
				m := (^mobj_t)(thinker)
				if m.type != .MT_TELEPORTMAN {
					continue
				}
				sector := m.subsector.sector
				if int(uintptr(sector) - uintptr(&sectors[0])) / size_of(sector_t) != i {
					continue
				}

				oldx := thing.x
				oldy := thing.y
				oldz := thing.z

				if !P_TeleportMove(thing, m.x, m.y) {
					return 0
				}

				thing.z = thing.floorz
				if thing.player != nil {
					thing.player.viewz = thing.z + thing.player.viewheight
				}

				fog := P_SpawnMobj(oldx, oldy, oldz, .MT_TFOG)
				S_StartSound(fog, int(sfxenum_t.sfx_telept))
				an := m.angle >> ANGLETOFINESHIFT
				fog = P_SpawnMobj(m.x + 20 * finecosine[an], m.y + 20 * finesine[an], thing.z, .MT_TFOG)
				S_StartSound(fog, int(sfxenum_t.sfx_telept))

				if thing.player != nil {
					thing.reactiontime = 18
				}

				thing.angle = m.angle
				thing.momx = 0
				thing.momy = 0
				thing.momz = 0
				return 1
			}
		}
	}
	return 0
}
