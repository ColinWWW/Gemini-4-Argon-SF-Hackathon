// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

BONUSADD :: 6

maxammo  := [NUMAMMO]int{200, 50, 300, 50}
clipammo := [NUMAMMO]int{10, 4, 20, 1}

P_GiveAmmo :: proc(player: ^player_t, ammo: ammotype_t, num_in: int) -> boolean {
	num := num_in
	if ammo == .am_noammo {
		return false
	}

	if int(ammo) < 0 || int(ammo) > NUMAMMO {
		I_Error("P_GiveAmmo: bad type %i", i32(ammo))
	}

	if player.ammo[ammo] == player.maxammo[ammo] {
		return false
	}

	if num != 0 {
		num *= clipammo[ammo]
	} else {
		num = clipammo[ammo] / 2
	}

	if gameskill == .sk_baby || gameskill == .sk_nightmare {
		num <<= 1
	}

	oldammo := player.ammo[ammo]
	player.ammo[ammo] += num

	if player.ammo[ammo] > player.maxammo[ammo] {
		player.ammo[ammo] = player.maxammo[ammo]
	}

	if oldammo != 0 {
		return true
	}

	#partial switch ammo {
	case .am_clip:
		if player.readyweapon == .wp_fist {
			if player.weaponowned[weapontype_t.wp_chaingun] {
				player.pendingweapon = .wp_chaingun
			} else {
				player.pendingweapon = .wp_pistol
			}
		}
	case .am_shell:
		if player.readyweapon == .wp_fist || player.readyweapon == .wp_pistol {
			if player.weaponowned[weapontype_t.wp_shotgun] {
				player.pendingweapon = .wp_shotgun
			}
		}
	case .am_cell:
		if player.readyweapon == .wp_fist || player.readyweapon == .wp_pistol {
			if player.weaponowned[weapontype_t.wp_plasma] {
				player.pendingweapon = .wp_plasma
			}
		}
	case .am_misl:
		if player.readyweapon == .wp_fist {
			if player.weaponowned[weapontype_t.wp_missile] {
				player.pendingweapon = .wp_missile
			}
		}
	}

	return true
}

P_GiveWeapon :: proc(player: ^player_t, weapon: weapontype_t, dropped: boolean) -> boolean {
	if netgame && deathmatch != 2 && !dropped {
		if player.weaponowned[weapon] {
			return false
		}

		player.bonuscount += BONUSADD
		player.weaponowned[weapon] = true

		if deathmatch != 0 {
			P_GiveAmmo(player, weaponinfo[weapon].ammo, 5)
		} else {
			P_GiveAmmo(player, weaponinfo[weapon].ammo, 2)
		}
		player.pendingweapon = weapon

		if player == &players[consoleplayer] {
			S_StartSound(nil, int(sfxenum_t.sfx_wpnup))
		}
		return false
	}

	gaveammo: boolean
	if weaponinfo[weapon].ammo != .am_noammo {
		if dropped {
			gaveammo = P_GiveAmmo(player, weaponinfo[weapon].ammo, 1)
		} else {
			gaveammo = P_GiveAmmo(player, weaponinfo[weapon].ammo, 2)
		}
	} else {
		gaveammo = false
	}

	gaveweapon: boolean
	if player.weaponowned[weapon] {
		gaveweapon = false
	} else {
		gaveweapon = true
		player.weaponowned[weapon] = true
		player.pendingweapon = weapon
	}

	return gaveweapon || gaveammo
}

P_GiveBody :: proc(player: ^player_t, num: int) -> boolean {
	if player.health >= MAXHEALTH {
		return false
	}

	player.health += num
	if player.health > MAXHEALTH {
		player.health = MAXHEALTH
	}
	player.mo.health = player.health

	return true
}

P_GiveArmor :: proc(player: ^player_t, armortype: int) -> boolean {
	hits := armortype * 100
	if player.armorpoints >= hits {
		return false
	}

	player.armortype = armortype
	player.armorpoints = hits

	return true
}

P_GiveCard :: proc(player: ^player_t, card: card_t) {
	if player.cards[card] {
		return
	}
	player.bonuscount = BONUSADD
	player.cards[card] = true
}

P_GivePower :: proc(player: ^player_t, power: int) -> boolean {
	if power == pw_invulnerability {
		player.powers[power] = INVULNTICS
		return true
	}

	if power == pw_invisibility {
		player.powers[power] = INVISTICS
		player.mo.flags |= MF_SHADOW
		return true
	}

	if power == pw_infrared {
		player.powers[power] = INFRATICS
		return true
	}

	if power == pw_ironfeet {
		player.powers[power] = IRONTICS
		return true
	}

	if power == pw_strength {
		P_GiveBody(player, 100)
		player.powers[power] = 1
		return true
	}

	if player.powers[power] != 0 {
		return false
	}

	player.powers[power] = 1
	return true
}

P_TouchSpecialThing :: proc(special: ^mobj_t, toucher: ^mobj_t) {
	delta := special.z - toucher.z

	if delta > toucher.height || delta < -8 * FRACUNIT {
		return
	}

	sound := int(sfxenum_t.sfx_itemup)
	player := toucher.player

	if toucher.health <= 0 {
		return
	}

	#partial switch special.sprite {
	case .SPR_ARM1:
		if !P_GiveArmor(player, 1) { return }
		player.message = GOTARMOR
	case .SPR_ARM2:
		if !P_GiveArmor(player, 2) { return }
		player.message = GOTMEGA
	case .SPR_BON1:
		player.health += 1
		if player.health > 200 { player.health = 200 }
		player.mo.health = player.health
		player.message = GOTHTHBONUS
	case .SPR_BON2:
		player.armorpoints += 1
		if player.armorpoints > 200 { player.armorpoints = 200 }
		if player.armortype == 0 { player.armortype = 1 }
		player.message = GOTARMBONUS
	case .SPR_SOUL:
		player.health += 100
		if player.health > 200 { player.health = 200 }
		player.mo.health = player.health
		player.message = GOTSUPER
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_MEGA:
		if gamemode != .commercial { return }
		player.health = 200
		player.mo.health = player.health
		P_GiveArmor(player, 2)
		player.message = GOTMSPHERE
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_BKEY:
		if !player.cards[card_t.it_bluecard] { player.message = GOTBLUECARD }
		P_GiveCard(player, .it_bluecard)
		if netgame { return }
	case .SPR_YKEY:
		if !player.cards[card_t.it_yellowcard] { player.message = GOTYELWCARD }
		P_GiveCard(player, .it_yellowcard)
		if netgame { return }
	case .SPR_RKEY:
		if !player.cards[card_t.it_redcard] { player.message = GOTREDCARD }
		P_GiveCard(player, .it_redcard)
		if netgame { return }
	case .SPR_BSKU:
		if !player.cards[card_t.it_blueskull] { player.message = GOTBLUESKUL }
		P_GiveCard(player, .it_blueskull)
		if netgame { return }
	case .SPR_YSKU:
		if !player.cards[card_t.it_yellowskull] { player.message = GOTYELWSKUL }
		P_GiveCard(player, .it_yellowskull)
		if netgame { return }
	case .SPR_RSKU:
		if !player.cards[card_t.it_redskull] { player.message = GOTREDSKULL }
		P_GiveCard(player, .it_redskull)
		if netgame { return }
	case .SPR_STIM:
		if !P_GiveBody(player, 10) { return }
		player.message = GOTSTIM
	case .SPR_MEDI:
		if !P_GiveBody(player, 25) { return }
		if player.health < 25 {
			player.message = GOTMEDINEED
		} else {
			player.message = GOTMEDIKIT
		}
	case .SPR_PINV:
		if !P_GivePower(player, pw_invulnerability) { return }
		player.message = GOTINVUL
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_PSTR:
		if !P_GivePower(player, pw_strength) { return }
		player.message = GOTBERSERK
		if player.readyweapon != .wp_fist {
			player.pendingweapon = .wp_fist
		}
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_PINS:
		if !P_GivePower(player, pw_invisibility) { return }
		player.message = GOTINVIS
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_SUIT:
		if !P_GivePower(player, pw_ironfeet) { return }
		player.message = GOTSUIT
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_PMAP:
		if !P_GivePower(player, pw_allmap) { return }
		player.message = GOTMAP
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_PVIS:
		if !P_GivePower(player, pw_infrared) { return }
		player.message = GOTVISOR
		sound = int(sfxenum_t.sfx_getpow)
	case .SPR_CLIP:
		if (special.flags & MF_DROPPED) != 0 {
			if !P_GiveAmmo(player, .am_clip, 0) { return }
		} else {
			if !P_GiveAmmo(player, .am_clip, 1) { return }
		}
		player.message = GOTCLIP
	case .SPR_AMMO:
		if !P_GiveAmmo(player, .am_clip, 5) { return }
		player.message = GOTCLIPBOX
	case .SPR_ROCK:
		if !P_GiveAmmo(player, .am_misl, 1) { return }
		player.message = GOTROCKET
	case .SPR_BROK:
		if !P_GiveAmmo(player, .am_misl, 5) { return }
		player.message = GOTROCKBOX
	case .SPR_CELL:
		if !P_GiveAmmo(player, .am_cell, 1) { return }
		player.message = GOTCELL
	case .SPR_CELP:
		if !P_GiveAmmo(player, .am_cell, 5) { return }
		player.message = GOTCELLBOX
	case .SPR_SHEL:
		if !P_GiveAmmo(player, .am_shell, 1) { return }
		player.message = GOTSHELLS
	case .SPR_SBOX:
		if !P_GiveAmmo(player, .am_shell, 5) { return }
		player.message = GOTSHELLBOX
	case .SPR_BPAK:
		if !player.backpack {
			for i := 0; i < NUMAMMO; i += 1 {
				player.maxammo[i] *= 2
			}
			player.backpack = true
		}
		for i := 0; i < NUMAMMO; i += 1 {
			P_GiveAmmo(player, ammotype_t(i), 1)
		}
		player.message = GOTBACKPACK
	case .SPR_BFUG:
		if !P_GiveWeapon(player, .wp_bfg, false) { return }
		player.message = GOTBFG9000
		sound = int(sfxenum_t.sfx_wpnup)
	case .SPR_MGUN:
		if !P_GiveWeapon(player, .wp_chaingun, (special.flags & MF_DROPPED) != 0) { return }
		player.message = GOTCHAINGUN
		sound = int(sfxenum_t.sfx_wpnup)
	case .SPR_CSAW:
		if !P_GiveWeapon(player, .wp_chainsaw, false) { return }
		player.message = GOTCHAINSAW
		sound = int(sfxenum_t.sfx_wpnup)
	case .SPR_LAUN:
		if !P_GiveWeapon(player, .wp_missile, false) { return }
		player.message = GOTLAUNCHER
		sound = int(sfxenum_t.sfx_wpnup)
	case .SPR_PLAS:
		if !P_GiveWeapon(player, .wp_plasma, false) { return }
		player.message = GOTPLASMA
		sound = int(sfxenum_t.sfx_wpnup)
	case .SPR_SHOT:
		if !P_GiveWeapon(player, .wp_shotgun, (special.flags & MF_DROPPED) != 0) { return }
		player.message = GOTSHOTGUN
		sound = int(sfxenum_t.sfx_wpnup)
	case .SPR_SGN2:
		if !P_GiveWeapon(player, .wp_supershotgun, (special.flags & MF_DROPPED) != 0) { return }
		player.message = GOTSHOTGUN2
		sound = int(sfxenum_t.sfx_wpnup)
	case:
		I_Error("P_SpecialThing: Unknown gettable thing")
	}

	if (special.flags & MF_COUNTITEM) != 0 {
		player.itemcount += 1
	}
	P_RemoveMobj(special)
	player.bonuscount += BONUSADD
	if player == &players[consoleplayer] {
		S_StartSound(nil, sound)
	}
}

P_KillMobj :: proc(source: ^mobj_t, target: ^mobj_t) {
	target.flags &= ~int(MF_SHOOTABLE | MF_FLOAT | MF_SKULLFLY)

	if target.type != .MT_SKULL {
		target.flags &= ~int(MF_NOGRAVITY)
	}

	target.flags |= MF_CORPSE | MF_DROPOFF
	target.height >>= 2

	if source != nil && source.player != nil {
		if (target.flags & MF_COUNTKILL) != 0 {
			source.player.killcount += 1
		}
		if target.player != nil {
			pidx := int(uintptr(target.player) - uintptr(&players[0])) / size_of(player_t)
			source.player.frags[pidx] += 1
		}
	} else if !netgame && (target.flags & MF_COUNTKILL) != 0 {
		players[0].killcount += 1
	}

	if target.player != nil {
		if source == nil {
			pidx := int(uintptr(target.player) - uintptr(&players[0])) / size_of(player_t)
			target.player.frags[pidx] += 1
		}
		target.flags &= ~int(MF_SOLID)
		target.player.playerstate = .PST_DEAD
		P_DropWeapon(target.player)

		if target.player == &players[consoleplayer] && automapactive {
			AM_Stop()
		}
	}

	if target.health < -target.info.spawnhealth && target.info.xdeathstate != .S_NULL {
		P_SetMobjState(target, target.info.xdeathstate)
	} else {
		P_SetMobjState(target, target.info.deathstate)
	}
	target.tics -= P_Random() & 3

	if target.tics < 1 {
		target.tics = 1
	}

	item: mobjtype_t
	#partial switch target.type {
	case .MT_WOLFSS, .MT_POSSESSED:
		item = .MT_CLIP
	case .MT_SHOTGUY:
		item = .MT_SHOTGUN
	case .MT_CHAINGUY:
		item = .MT_CHAINGUN
	case:
		return
	}

	mo := P_SpawnMobj(target.x, target.y, ONFLOORZ, item)
	mo.flags |= MF_DROPPED
}

P_DamageMobj :: proc(target: ^mobj_t, inflictor: ^mobj_t, source: ^mobj_t, damage_in: int) {
	damage := damage_in
	if (target.flags & MF_SHOOTABLE) == 0 {
		return
	}

	if target.health <= 0 {
		return
	}

	if (target.flags & MF_SKULLFLY) != 0 {
		target.momx = 0
		target.momy = 0
		target.momz = 0
	}

	player := target.player
	if player != nil && gameskill == .sk_baby {
		damage >>= 1
	}

	if inflictor != nil && (target.flags & MF_NOCLIP) == 0 && (source == nil || source.player == nil || source.player.readyweapon != .wp_chainsaw) {
		ang := R_PointToAngle2(inflictor.x, inflictor.y, target.x, target.y)

		thrust := fixed_t(damage * (FRACUNIT >> 3) * 100 / target.info.mass)

		if damage < 40 && damage > target.health && target.z - inflictor.z > 64 * FRACUNIT && (P_Random() & 1) != 0 {
			ang += ANG180
			thrust *= 4
		}

		ang >>= ANGLETOFINESHIFT
		target.momx += FixedMul(thrust, finecosine[ang])
		target.momy += FixedMul(thrust, finesine[ang])
	}

	if player != nil {
		if target.subsector.sector.special == 11 && damage >= target.health {
			damage = target.health - 1
		}

		if damage < 1000 && ((player.cheats & CF_GODMODE) != 0 || player.powers[pw_invulnerability] != 0) {
			return
		}

		if player.armortype != 0 {
			saved: int
			if player.armortype == 1 {
				saved = damage / 3
			} else {
				saved = damage / 2
			}

			if player.armorpoints <= saved {
				saved = player.armorpoints
				player.armortype = 0
			}
			player.armorpoints -= saved
			damage -= saved
		}
		player.health -= damage
		if player.health < 0 {
			player.health = 0
		}

		player.attacker = source
		player.damagecount += damage

		if player.damagecount > 100 {
			player.damagecount = 100
		}

		temp := damage < 100 ? damage : 100

		if player == &players[consoleplayer] {
			I_Tactile(40, 10, 40 + temp * 2)
		}
	}

	target.health -= damage
	if target.health <= 0 {
		P_KillMobj(source, target)
		return
	}

	if P_Random() < target.info.painchance && (target.flags & MF_SKULLFLY) == 0 {
		target.flags |= MF_JUSTHIT
		P_SetMobjState(target, target.info.painstate)
	}

	target.reactiontime = 0

	if (target.threshold == 0 || target.type == .MT_VILE) && source != nil && source != target && source.type != .MT_VILE {
		target.target = source
		target.threshold = BASETHRESHOLD
		if target.state == &states[target.info.spawnstate] && target.info.seestate != .S_NULL {
			P_SetMobjState(target, target.info.seestate)
		}
	}
}
