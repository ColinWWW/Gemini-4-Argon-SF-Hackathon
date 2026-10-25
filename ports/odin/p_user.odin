// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

INVERSECOLORMAP :: 32
MAXBOB          :: 0x100000

onground: boolean

P_Thrust :: proc(player: ^player_t, angle_in: angle_t, move: fixed_t) {
	angle := angle_in >> ANGLETOFINESHIFT
	player.mo.momx += FixedMul(move, finecosine[angle])
	player.mo.momy += FixedMul(move, finesine[angle])
}

P_CalcHeight :: proc(player: ^player_t) {
	player.bob = FixedMul(player.mo.momx, player.mo.momx) + FixedMul(player.mo.momy, player.mo.momy)
	player.bob >>= 2

	if player.bob > MAXBOB {
		player.bob = MAXBOB
	}

	if (player.cheats & CF_NOMOMENTUM) != 0 || !onground {
		player.viewz = player.mo.z + VIEWHEIGHT
		if player.viewz > player.mo.ceilingz - 4 * FRACUNIT {
			player.viewz = player.mo.ceilingz - 4 * FRACUNIT
		}
		player.viewz = player.mo.z + player.viewheight
		return
	}

	angle := (FINEANGLES / 20 * leveltime) & FINEMASK
	bob := FixedMul(player.bob / 2, finesine[angle])

	if player.playerstate == .PST_LIVE {
		player.viewheight += player.deltaviewheight

		if player.viewheight > VIEWHEIGHT {
			player.viewheight = VIEWHEIGHT
			player.deltaviewheight = 0
		}

		if player.viewheight < VIEWHEIGHT / 2 {
			player.viewheight = VIEWHEIGHT / 2
			if player.deltaviewheight <= 0 {
				player.deltaviewheight = 1
			}
		}

		if player.deltaviewheight != 0 {
			player.deltaviewheight += FRACUNIT / 4
			if player.deltaviewheight == 0 {
				player.deltaviewheight = 1
			}
		}
	}
	player.viewz = player.mo.z + player.viewheight + bob

	if player.viewz > player.mo.ceilingz - 4 * FRACUNIT {
		player.viewz = player.mo.ceilingz - 4 * FRACUNIT
	}
}

P_MovePlayer :: proc(player: ^player_t) {
	cmd := &player.cmd

	player.mo.angle += angle_t(i32(cmd.angleturn) << 16)

	onground = (player.mo.z <= player.mo.floorz)

	if cmd.forwardmove != 0 && onground {
		P_Thrust(player, player.mo.angle, fixed_t(cmd.forwardmove) * 2048)
	}

	if cmd.sidemove != 0 && onground {
		P_Thrust(player, player.mo.angle - ANG90, fixed_t(cmd.sidemove) * 2048)
	}

	if (cmd.forwardmove != 0 || cmd.sidemove != 0) && player.mo.state == &states[statenum_t.S_PLAY] {
		P_SetMobjState(player.mo, .S_PLAY_RUN1)
	}
}

ANG5 :: ANG90 / 18

P_DeathThink :: proc(player: ^player_t) {
	P_MovePsprites(player)

	if player.viewheight > 6 * FRACUNIT {
		player.viewheight -= FRACUNIT
	}

	if player.viewheight < 6 * FRACUNIT {
		player.viewheight = 6 * FRACUNIT
	}

	player.deltaviewheight = 0
	onground = (player.mo.z <= player.mo.floorz)
	P_CalcHeight(player)

	if player.attacker != nil && player.attacker != player.mo {
		angle := R_PointToAngle2(player.mo.x, player.mo.y, player.attacker.x, player.attacker.y)
		delta := angle - player.mo.angle

		if delta < ANG5 || delta > (0xFFFFFFFF - ANG5 + 1) {
			player.mo.angle = angle
			if player.damagecount != 0 {
				player.damagecount -= 1
			}
		} else if delta < ANG180 {
			player.mo.angle += ANG5
		} else {
			player.mo.angle -= ANG5
		}
	} else if player.damagecount != 0 {
		player.damagecount -= 1
	}

	if (player.cmd.buttons & BT_USE) != 0 {
		player.playerstate = .PST_REBORN
	}
}

P_PlayerThink :: proc(player: ^player_t) {
	if (player.cheats & CF_NOCLIP) != 0 {
		player.mo.flags |= MF_NOCLIP
	} else {
		player.mo.flags &= ~int(MF_NOCLIP)
	}

	cmd := &player.cmd
	if (player.mo.flags & MF_JUSTATTACKED) != 0 {
		cmd.angleturn = 0
		cmd.forwardmove = 0xc800 / 512
		cmd.sidemove = 0
		player.mo.flags &= ~int(MF_JUSTATTACKED)
	}

	if player.playerstate == .PST_DEAD {
		P_DeathThink(player)
		return
	}

	if player.mo.reactiontime != 0 {
		player.mo.reactiontime -= 1
	} else {
		P_MovePlayer(player)
	}

	P_CalcHeight(player)

	if player.mo.subsector.sector.special != 0 {
		P_PlayerInSpecialSector(player)
	}

	if (cmd.buttons & BT_SPECIAL) != 0 {
		cmd.buttons = 0
	}

	if (cmd.buttons & BT_CHANGE) != 0 {
		newweapon := weapontype_t((cmd.buttons & BT_WEAPONMASK) >> BT_WEAPONSHIFT)

		if newweapon == .wp_fist && player.weaponowned[weapontype_t.wp_chainsaw] && !(player.readyweapon == .wp_chainsaw && player.powers[pw_strength] != 0) {
			newweapon = .wp_chainsaw
		}

		if gamemode == .commercial && newweapon == .wp_shotgun && player.weaponowned[weapontype_t.wp_supershotgun] && player.readyweapon != .wp_supershotgun {
			newweapon = .wp_supershotgun
		}

		if player.weaponowned[newweapon] && newweapon != player.readyweapon {
			if (newweapon != .wp_plasma && newweapon != .wp_bfg) || gamemode != .shareware {
				player.pendingweapon = newweapon
			}
		}
	}

	if (cmd.buttons & BT_USE) != 0 {
		if player.usedown == 0 {
			P_UseLines(player)
			player.usedown = 1
		}
	} else {
		player.usedown = 0
	}

	P_MovePsprites(player)

	if player.powers[pw_strength] != 0 {
		player.powers[pw_strength] += 1
	}

	if player.powers[pw_invulnerability] != 0 {
		player.powers[pw_invulnerability] -= 1
	}

	if player.powers[pw_invisibility] != 0 {
		player.powers[pw_invisibility] -= 1
		if player.powers[pw_invisibility] == 0 {
			player.mo.flags &= ~int(MF_SHADOW)
		}
	}

	if player.powers[pw_infrared] != 0 {
		player.powers[pw_infrared] -= 1
	}

	if player.powers[pw_ironfeet] != 0 {
		player.powers[pw_ironfeet] -= 1
	}

	if player.damagecount != 0 {
		player.damagecount -= 1
	}

	if player.bonuscount != 0 {
		player.bonuscount -= 1
	}

	if player.powers[pw_invulnerability] != 0 {
		if player.powers[pw_invulnerability] > 4 * 32 || (player.powers[pw_invulnerability] & 8) != 0 {
			player.fixedcolormap = INVERSECOLORMAP
		} else {
			player.fixedcolormap = 0
		}
	} else if player.powers[pw_infrared] != 0 {
		if player.powers[pw_infrared] > 4 * 32 || (player.powers[pw_infrared] & 8) != 0 {
			player.fixedcolormap = 1
		} else {
			player.fixedcolormap = 0
		}
	} else {
		player.fixedcolormap = 0
	}
}