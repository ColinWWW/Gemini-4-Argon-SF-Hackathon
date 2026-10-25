// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

LOWERSPEED   :: FRACUNIT * 6
RAISESPEED   :: FRACUNIT * 6
WEAPONBOTTOM :: 128 * FRACUNIT
WEAPONTOP    :: 32 * FRACUNIT
BFGCELLS     :: 40

swingx: fixed_t
swingy: fixed_t
bulletslope: fixed_t

P_SetPsprite :: proc(player: ^player_t, position: int, stnum_in: statenum_t) {
	stnum := stnum_in
	psp := &player.psprites[position]

	for {
		if stnum == .S_NULL {
			psp.state = nil
			break
		}

		state := &states[stnum]
		psp.state = state
		psp.tics = state.tics

		if state.misc1 != 0 {
			psp.sx = fixed_t(i32(state.misc1) << FRACBITS)
			psp.sy = fixed_t(i32(state.misc2) << FRACBITS)
		}

		if state.action.acp2 != nil {
			state.action.acp2(player, psp)
			if psp.state == nil {
				break
			}
		}

		stnum = psp.state.nextstate
		if psp.tics != 0 {
			break
		}
	}
}

P_CalcSwing :: proc(player: ^player_t) {
	swing := player.bob

	angle := (FINEANGLES / 70 * leveltime) & FINEMASK
	swingx = FixedMul(swing, finesine[angle])

	angle = (FINEANGLES / 70 * leveltime + FINEANGLES / 2) & FINEMASK
	swingy = -FixedMul(swingx, finesine[angle])
}

P_BringUpWeapon :: proc(player: ^player_t) {
	if player.pendingweapon == .wp_nochange {
		player.pendingweapon = player.readyweapon
	}

	if player.pendingweapon == .wp_chainsaw {
		S_StartSound(player.mo, int(sfxenum_t.sfx_sawup))
	}

	newstate := statenum_t(weaponinfo[player.pendingweapon].upstate)

	player.pendingweapon = .wp_nochange
	player.psprites[ps_weapon].sy = WEAPONBOTTOM

	P_SetPsprite(player, ps_weapon, newstate)
}

P_CheckAmmo :: proc(player: ^player_t) -> boolean {
	ammo := weaponinfo[player.readyweapon].ammo

	count: int
	if player.readyweapon == .wp_bfg {
		count = BFGCELLS
	} else if player.readyweapon == .wp_supershotgun {
		count = 2
	} else {
		count = 1
	}

	if ammo == .am_noammo || player.ammo[ammo] >= count {
		return true
	}

	for {
		if player.weaponowned[weapontype_t.wp_plasma] && player.ammo[ammotype_t.am_cell] != 0 && gamemode != .shareware {
			player.pendingweapon = .wp_plasma
		} else if player.weaponowned[weapontype_t.wp_supershotgun] && player.ammo[ammotype_t.am_shell] > 2 && gamemode == .commercial {
			player.pendingweapon = .wp_supershotgun
		} else if player.weaponowned[weapontype_t.wp_chaingun] && player.ammo[ammotype_t.am_clip] != 0 {
			player.pendingweapon = .wp_chaingun
		} else if player.weaponowned[weapontype_t.wp_shotgun] && player.ammo[ammotype_t.am_shell] != 0 {
			player.pendingweapon = .wp_shotgun
		} else if player.ammo[ammotype_t.am_clip] != 0 {
			player.pendingweapon = .wp_pistol
		} else if player.weaponowned[weapontype_t.wp_chainsaw] {
			player.pendingweapon = .wp_chainsaw
		} else if player.weaponowned[weapontype_t.wp_missile] && player.ammo[ammotype_t.am_misl] != 0 {
			player.pendingweapon = .wp_missile
		} else if player.weaponowned[weapontype_t.wp_bfg] && player.ammo[ammotype_t.am_cell] > 40 && gamemode != .shareware {
			player.pendingweapon = .wp_bfg
		} else {
			player.pendingweapon = .wp_fist
		}

		if player.pendingweapon != .wp_nochange {
			break
		}
	}

	P_SetPsprite(player, ps_weapon, statenum_t(weaponinfo[player.readyweapon].downstate))

	return false
}

P_FireWeapon :: proc(player: ^player_t) {
	if !P_CheckAmmo(player) {
		return
	}

	P_SetMobjState(player.mo, .S_PLAY_ATK1)
	newstate := statenum_t(weaponinfo[player.readyweapon].atkstate)
	P_SetPsprite(player, ps_weapon, newstate)
	P_NoiseAlert(player.mo, player.mo)
}

P_DropWeapon :: proc(player: ^player_t) {
	P_SetPsprite(player, ps_weapon, statenum_t(weaponinfo[player.readyweapon].downstate))
}

A_WeaponReady :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	psp := (^pspdef_t)(psp_raw)

	if player.mo.state == &states[statenum_t.S_PLAY_ATK1] || player.mo.state == &states[statenum_t.S_PLAY_ATK2] {
		P_SetMobjState(player.mo, .S_PLAY)
	}

	if player.readyweapon == .wp_chainsaw && psp.state == &states[statenum_t.S_SAW] {
		S_StartSound(player.mo, int(sfxenum_t.sfx_sawidl))
	}

	if player.pendingweapon != .wp_nochange || player.health == 0 {
		newstate := statenum_t(weaponinfo[player.readyweapon].downstate)
		P_SetPsprite(player, ps_weapon, newstate)
		return
	}

	if (player.cmd.buttons & BT_ATTACK) != 0 {
		if player.attackdown == 0 || (player.readyweapon != .wp_missile && player.readyweapon != .wp_bfg) {
			player.attackdown = 1
			P_FireWeapon(player)
			return
		}
	} else {
		player.attackdown = 0
	}

	angle := (128 * leveltime) & FINEMASK
	psp.sx = FRACUNIT + FixedMul(player.bob, finecosine[angle])
	angle &= FINEANGLES / 2 - 1
	psp.sy = WEAPONTOP + FixedMul(player.bob, finesine[angle])
}

A_ReFire :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)

	if (player.cmd.buttons & BT_ATTACK) != 0 && player.pendingweapon == .wp_nochange && player.health != 0 {
		player.refire += 1
		P_FireWeapon(player)
	} else {
		player.refire = 0
		P_CheckAmmo(player)
	}
}

A_CheckReload :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	P_CheckAmmo(player)
}

A_Lower :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	psp := (^pspdef_t)(psp_raw)

	psp.sy += LOWERSPEED

	if psp.sy < WEAPONBOTTOM {
		return
	}

	if player.playerstate == .PST_DEAD {
		psp.sy = WEAPONBOTTOM
		return
	}

	if player.health == 0 {
		P_SetPsprite(player, ps_weapon, .S_NULL)
		return
	}

	player.readyweapon = player.pendingweapon
	P_BringUpWeapon(player)
}

A_Raise :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	psp := (^pspdef_t)(psp_raw)

	psp.sy -= RAISESPEED

	if psp.sy > WEAPONTOP {
		return
	}

	psp.sy = WEAPONTOP

	newstate := statenum_t(weaponinfo[player.readyweapon].readystate)
	P_SetPsprite(player, ps_weapon, newstate)
}

A_GunFlash :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	P_SetMobjState(player.mo, .S_PLAY_ATK2)
	P_SetPsprite(player, ps_flash, statenum_t(weaponinfo[player.readyweapon].flashstate))
}

A_Punch :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	damage := (P_Random() % 10 + 1) << 1

	if player.powers[pw_strength] != 0 {
		damage *= 10
	}

	angle := player.mo.angle
	r1 := P_Random()
	r2 := P_Random()
	angle += angle_t(i32(r1 - r2) << 18)
	slope := P_AimLineAttack(player.mo, angle, MELEERANGE)
	P_LineAttack(player.mo, angle, MELEERANGE, slope, damage)

	if linetarget != nil {
		S_StartSound(player.mo, int(sfxenum_t.sfx_punch))
		player.mo.angle = R_PointToAngle2(player.mo.x, player.mo.y, linetarget.x, linetarget.y)
	}
}

A_Saw :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	damage := 2 * (P_Random() % 10 + 1)
	angle := player.mo.angle
	r1 := P_Random()
	r2 := P_Random()
	angle += angle_t(i32(r1 - r2) << 18)

	slope := P_AimLineAttack(player.mo, angle, MELEERANGE + 1)
	P_LineAttack(player.mo, angle, MELEERANGE + 1, slope, damage)

	if linetarget == nil {
		S_StartSound(player.mo, int(sfxenum_t.sfx_sawful))
		return
	}
	S_StartSound(player.mo, int(sfxenum_t.sfx_sawhit))

	angle = R_PointToAngle2(player.mo.x, player.mo.y, linetarget.x, linetarget.y)
	if angle - player.mo.angle > ANG180 {
		if i32(angle - player.mo.angle) < -i32(ANG90 / 20) {
			player.mo.angle = angle + ANG90 / 21
		} else {
			player.mo.angle -= ANG90 / 20
		}
	} else {
		if angle - player.mo.angle > ANG90 / 20 {
			player.mo.angle = angle - ANG90 / 21
		} else {
			player.mo.angle += ANG90 / 20
		}
	}
	player.mo.flags |= MF_JUSTATTACKED
}

A_FireMissile :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	player.ammo[weaponinfo[player.readyweapon].ammo] -= 1
	P_SpawnPlayerMissile(player.mo, .MT_ROCKET)
}

A_FireBFG :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	player.ammo[weaponinfo[player.readyweapon].ammo] -= BFGCELLS
	P_SpawnPlayerMissile(player.mo, .MT_BFG)
}

A_FirePlasma :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	player.ammo[weaponinfo[player.readyweapon].ammo] -= 1

	P_SetPsprite(player, ps_flash, statenum_t(weaponinfo[player.readyweapon].flashstate + (P_Random() & 1)))
	P_SpawnPlayerMissile(player.mo, .MT_PLASMA)
}

P_BulletSlope :: proc(mo: ^mobj_t) {
	an := mo.angle
	bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT)

	if linetarget == nil {
		an += 1 << 26
		bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT)
		if linetarget == nil {
			an -= 2 << 26
			bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT)
		}
	}
}

P_GunShot :: proc(mo: ^mobj_t, accurate: boolean) {
	damage := 5 * (P_Random() % 3 + 1)
	angle := mo.angle

	if !accurate {
		r1 := P_Random()
		r2 := P_Random()
		angle += angle_t(i32(r1 - r2) << 18)
	}

	P_LineAttack(mo, angle, MISSILERANGE, bulletslope, damage)
}

A_FirePistol :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_pistol))

	P_SetMobjState(player.mo, .S_PLAY_ATK2)
	player.ammo[weaponinfo[player.readyweapon].ammo] -= 1

	P_SetPsprite(player, ps_flash, statenum_t(weaponinfo[player.readyweapon].flashstate))

	P_BulletSlope(player.mo)
	P_GunShot(player.mo, player.refire == 0)
}

A_FireShotgun :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_shotgn))
	P_SetMobjState(player.mo, .S_PLAY_ATK2)

	player.ammo[weaponinfo[player.readyweapon].ammo] -= 1

	P_SetPsprite(player, ps_flash, statenum_t(weaponinfo[player.readyweapon].flashstate))

	P_BulletSlope(player.mo)

	for i := 0; i < 7; i += 1 {
		P_GunShot(player.mo, false)
	}
}

A_FireShotgun2 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_dshtgn))
	P_SetMobjState(player.mo, .S_PLAY_ATK2)

	player.ammo[weaponinfo[player.readyweapon].ammo] -= 2

	P_SetPsprite(player, ps_flash, statenum_t(weaponinfo[player.readyweapon].flashstate))

	P_BulletSlope(player.mo)

	for i := 0; i < 20; i += 1 {
		damage := 5 * (P_Random() % 3 + 1)
		angle := player.mo.angle
		r1 := P_Random()
		r2 := P_Random()
		angle += angle_t(i32(r1 - r2) << 19)
		r3 := P_Random()
		r4 := P_Random()
		P_LineAttack(player.mo, angle, MISSILERANGE, bulletslope + fixed_t(i32(r3 - r4) << 5), damage)
	}
}

A_FireCGun :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	psp := (^pspdef_t)(psp_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_pistol))

	if player.ammo[weaponinfo[player.readyweapon].ammo] == 0 {
		return
	}

	P_SetMobjState(player.mo, .S_PLAY_ATK2)
	player.ammo[weaponinfo[player.readyweapon].ammo] -= 1

	state_offset := int(uintptr(psp.state) - uintptr(&states[statenum_t.S_CHAIN1])) / size_of(state_t)
	P_SetPsprite(player, ps_flash, statenum_t(weaponinfo[player.readyweapon].flashstate + state_offset))

	P_BulletSlope(player.mo)
	P_GunShot(player.mo, player.refire == 0)
}

A_Light0 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	player.extralight = 0
}

A_Light1 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	player.extralight = 1
}

A_Light2 :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	player.extralight = 2
}

A_BFGSpray :: proc(mo_raw: rawptr) {
	mo := (^mobj_t)(mo_raw)

	for i := 0; i < 40; i += 1 {
		an := mo.angle - ANG90 / 2 + ANG90 / 40 * angle_t(i)

		P_AimLineAttack(mo.target, an, 16 * 64 * FRACUNIT)

		if linetarget == nil {
			continue
		}

		P_SpawnMobj(linetarget.x, linetarget.y, linetarget.z + (linetarget.height >> 2), .MT_EXTRABFG)

		damage := 0
		for j := 0; j < 15; j += 1 {
			damage += (P_Random() & 7) + 1
		}

		P_DamageMobj(linetarget, mo.target, mo.target, damage)
	}
}

A_BFGsound :: proc(player_raw: rawptr, psp_raw: rawptr) {
	player := (^player_t)(player_raw)
	S_StartSound(player.mo, int(sfxenum_t.sfx_bfg))
}

P_SetupPsprites :: proc(player: ^player_t) {
	for i := 0; i < NUMPSPRITES; i += 1 {
		player.psprites[i].state = nil
	}

	player.pendingweapon = player.readyweapon
	P_BringUpWeapon(player)
}

P_MovePsprites :: proc(player: ^player_t) {
	for i := 0; i < NUMPSPRITES; i += 1 {
		psp := &player.psprites[i]
		if psp.state != nil {
			if psp.tics != -1 {
				psp.tics -= 1
				if psp.tics == 0 {
					P_SetPsprite(player, i, psp.state.nextstate)
				}
			}
		}
	}

	player.psprites[ps_flash].sx = player.psprites[ps_weapon].sx
	player.psprites[ps_flash].sy = player.psprites[ps_weapon].sy
}
