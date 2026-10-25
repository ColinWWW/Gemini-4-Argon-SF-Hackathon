// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::am_map::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_local::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::r_main::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;

pub const BONUSADD: i32 = 6;

pub static mut maxammo: [i32; NUMAMMO] = [200, 50, 300, 50];
pub static mut clipammo: [i32; NUMAMMO] = [10, 4, 20, 1];

pub unsafe fn P_GiveAmmo(player: *mut player_t, ammo: ammotype_t, mut num: i32) -> boolean {
    if ammo == am_noammo {
        return false_;
    }

    if ammo < 0 || (ammo as usize) > NUMAMMO {
        I_Error(&format!("P_GiveAmmo: bad type {}", ammo));
    }

    if (*player).ammo[ammo as usize] == (*player).maxammo[ammo as usize] {
        return false_;
    }

    if num != 0 {
        num *= clipammo[ammo as usize];
    } else {
        num = clipammo[ammo as usize] / 2;
    }

    if gameskill == sk_baby || gameskill == sk_nightmare {
        num <<= 1;
    }

    let oldammo = (*player).ammo[ammo as usize];
    (*player).ammo[ammo as usize] += num;

    if (*player).ammo[ammo as usize] > (*player).maxammo[ammo as usize] {
        (*player).ammo[ammo as usize] = (*player).maxammo[ammo as usize];
    }

    if oldammo != 0 {
        return true_;
    }

    match ammo {
        am_clip => {
            if (*player).readyweapon == wp_fist {
                if (*player).weaponowned[wp_chaingun as usize] != 0 {
                    (*player).pendingweapon = wp_chaingun;
                } else {
                    (*player).pendingweapon = wp_pistol;
                }
            }
        }
        am_shell => {
            if (*player).readyweapon == wp_fist || (*player).readyweapon == wp_pistol {
                if (*player).weaponowned[wp_shotgun as usize] != 0 {
                    (*player).pendingweapon = wp_shotgun;
                }
            }
        }
        am_cell => {
            if (*player).readyweapon == wp_fist || (*player).readyweapon == wp_pistol {
                if (*player).weaponowned[wp_plasma as usize] != 0 {
                    (*player).pendingweapon = wp_plasma;
                }
            }
        }
        am_misl => {
            if (*player).readyweapon == wp_fist {
                if (*player).weaponowned[wp_missile as usize] != 0 {
                    (*player).pendingweapon = wp_missile;
                }
            }
        }
        _ => {}
    }

    true_
}

pub unsafe fn P_GiveWeapon(
    player: *mut player_t,
    weapon: weapontype_t,
    dropped: boolean,
) -> boolean {
    if netgame != 0 && deathmatch != 2 && dropped == 0 {
        if (*player).weaponowned[weapon as usize] != 0 {
            return false_;
        }

        (*player).bonuscount += BONUSADD;
        (*player).weaponowned[weapon as usize] = true_;

        if deathmatch != 0 {
            P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 5);
        } else {
            P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 2);
        }
        (*player).pendingweapon = weapon;

        if player == (&mut players[consoleplayer as usize] as *mut player_t) {
            S_StartSound(std::ptr::null_mut(), sfx_wpnup);
        }
        return false_;
    }

    let gaveammo = if weaponinfo[weapon as usize].ammo != am_noammo {
        if dropped != 0 {
            P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 1)
        } else {
            P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 2)
        }
    } else {
        false_
    };

    let gaveweapon = if (*player).weaponowned[weapon as usize] != 0 {
        false_
    } else {
        (*player).weaponowned[weapon as usize] = true_;
        (*player).pendingweapon = weapon;
        true_
    };

    (gaveweapon != 0 || gaveammo != 0) as boolean
}

pub unsafe fn P_GiveBody(player: *mut player_t, num: i32) -> boolean {
    if (*player).health >= MAXHEALTH {
        return false_;
    }

    (*player).health += num;
    if (*player).health > MAXHEALTH {
        (*player).health = MAXHEALTH;
    }
    (*(*player).mo).health = (*player).health;

    true_
}

pub unsafe fn P_GiveArmor(player: *mut player_t, armortype: i32) -> boolean {
    let hits = armortype * 100;
    if (*player).armorpoints >= hits {
        return false_;
    }

    (*player).armortype = armortype;
    (*player).armorpoints = hits;

    true_
}

pub unsafe fn P_GiveCard(player: *mut player_t, card: card_t) {
    if (*player).cards[card as usize] != 0 {
        return;
    }
    (*player).bonuscount = BONUSADD;
    (*player).cards[card as usize] = 1;
}

pub unsafe fn P_GivePower(player: *mut player_t, power: i32) -> boolean {
    if power == pw_invulnerability {
        (*player).powers[power as usize] = INVULNTICS;
        return true_;
    }

    if power == pw_invisibility {
        (*player).powers[power as usize] = INVISTICS;
        (*(*player).mo).flags |= MF_SHADOW;
        return true_;
    }

    if power == pw_infrared {
        (*player).powers[power as usize] = INFRATICS;
        return true_;
    }

    if power == pw_ironfeet {
        (*player).powers[power as usize] = IRONTICS;
        return true_;
    }

    if power == pw_strength {
        P_GiveBody(player, 100);
        (*player).powers[power as usize] = 1;
        return true_;
    }

    if (*player).powers[power as usize] != 0 {
        return false_;
    }

    (*player).powers[power as usize] = 1;
    true_
}

pub unsafe fn P_TouchSpecialThing(special: *mut mobj_t, toucher: *mut mobj_t) {
    let delta = (*special).z - (*toucher).z;

    if delta > (*toucher).height || delta < -8 * FRACUNIT {
        return;
    }

    let mut sound = sfx_itemup;
    let player = (*toucher).player;

    if (*toucher).health <= 0 {
        return;
    }

    match (*special).sprite {
        SPR_ARM1 => {
            if P_GiveArmor(player, 1) == 0 {
                return;
            }
            (*player).message = b"Picked up the armor.\0".as_ptr();
        }
        SPR_ARM2 => {
            if P_GiveArmor(player, 2) == 0 {
                return;
            }
            (*player).message = b"Picked up the MegaArmor!\0".as_ptr();
        }
        SPR_BON1 => {
            (*player).health += 1;
            if (*player).health > 200 {
                (*player).health = 200;
            }
            (*(*player).mo).health = (*player).health;
            (*player).message = b"Picked up a health bonus.\0".as_ptr();
        }
        SPR_BON2 => {
            (*player).armorpoints += 1;
            if (*player).armorpoints > 200 {
                (*player).armorpoints = 200;
            }
            if (*player).armortype == 0 {
                (*player).armortype = 1;
            }
            (*player).message = b"Picked up an armor bonus.\0".as_ptr();
        }
        SPR_SOUL => {
            (*player).health += 100;
            if (*player).health > 200 {
                (*player).health = 200;
            }
            (*(*player).mo).health = (*player).health;
            (*player).message = b"Supercharge!\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_MEGA => {
            if gamemode != commercial {
                return;
            }
            (*player).health = 200;
            (*(*player).mo).health = (*player).health;
            P_GiveArmor(player, 2);
            (*player).message = b"MegaSphere!\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_BKEY => {
            if (*player).cards[it_bluecard as usize] == 0 {
                (*player).message = b"Picked up a blue keycard.\0".as_ptr();
            }
            P_GiveCard(player, it_bluecard);
            if netgame != 0 {
                return;
            }
        }
        SPR_YKEY => {
            if (*player).cards[it_yellowcard as usize] == 0 {
                (*player).message = b"Picked up a yellow keycard.\0".as_ptr();
            }
            P_GiveCard(player, it_yellowcard);
            if netgame != 0 {
                return;
            }
        }
        SPR_RKEY => {
            if (*player).cards[it_redcard as usize] == 0 {
                (*player).message = b"Picked up a red keycard.\0".as_ptr();
            }
            P_GiveCard(player, it_redcard);
            if netgame != 0 {
                return;
            }
        }
        SPR_BSKU => {
            if (*player).cards[it_blueskull as usize] == 0 {
                (*player).message = b"Picked up a blue skull key.\0".as_ptr();
            }
            P_GiveCard(player, it_blueskull);
            if netgame != 0 {
                return;
            }
        }
        SPR_YSKU => {
            if (*player).cards[it_yellowskull as usize] == 0 {
                (*player).message = b"Picked up a yellow skull key.\0".as_ptr();
            }
            P_GiveCard(player, it_yellowskull);
            if netgame != 0 {
                return;
            }
        }
        SPR_RSKU => {
            if (*player).cards[it_redskull as usize] == 0 {
                (*player).message = b"Picked up a red skull key.\0".as_ptr();
            }
            P_GiveCard(player, it_redskull);
            if netgame != 0 {
                return;
            }
        }
        SPR_STIM => {
            if P_GiveBody(player, 10) == 0 {
                return;
            }
            (*player).message = b"Picked up a stimpack.\0".as_ptr();
        }
        SPR_MEDI => {
            if P_GiveBody(player, 25) == 0 {
                return;
            }
            if (*player).health < 25 {
                (*player).message = b"Picked up a medikit that you REALLY need!\0".as_ptr();
            } else {
                (*player).message = b"Picked up a medikit.\0".as_ptr();
            }
        }
        SPR_PINV => {
            if P_GivePower(player, pw_invulnerability) == 0 {
                return;
            }
            (*player).message = b"Invulnerability!\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_PSTR => {
            if P_GivePower(player, pw_strength) == 0 {
                return;
            }
            (*player).message = b"Berserk!\0".as_ptr();
            if (*player).readyweapon != wp_fist {
                (*player).pendingweapon = wp_fist;
            }
            sound = sfx_getpow;
        }
        SPR_PINS => {
            if P_GivePower(player, pw_invisibility) == 0 {
                return;
            }
            (*player).message = b"Partial Invisibility\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_SUIT => {
            if P_GivePower(player, pw_ironfeet) == 0 {
                return;
            }
            (*player).message = b"Radiation Shielding Suit\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_PMAP => {
            if P_GivePower(player, pw_allmap) == 0 {
                return;
            }
            (*player).message = b"Computer Area Map\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_PVIS => {
            if P_GivePower(player, pw_infrared) == 0 {
                return;
            }
            (*player).message = b"Light Amplification Visor\0".as_ptr();
            sound = sfx_getpow;
        }
        SPR_CLIP => {
            if (*special).flags & MF_DROPPED != 0 {
                if P_GiveAmmo(player, am_clip, 0) == 0 {
                    return;
                }
            } else {
                if P_GiveAmmo(player, am_clip, 1) == 0 {
                    return;
                }
            }
            (*player).message = b"Picked up a clip.\0".as_ptr();
        }
        SPR_AMMO => {
            if P_GiveAmmo(player, am_clip, 5) == 0 {
                return;
            }
            (*player).message = b"Picked up a box of bullets.\0".as_ptr();
        }
        SPR_ROCK => {
            if P_GiveAmmo(player, am_misl, 1) == 0 {
                return;
            }
            (*player).message = b"Picked up a rocket.\0".as_ptr();
        }
        SPR_BROK => {
            if P_GiveAmmo(player, am_misl, 5) == 0 {
                return;
            }
            (*player).message = b"Picked up a box of rockets.\0".as_ptr();
        }
        SPR_CELL => {
            if P_GiveAmmo(player, am_cell, 1) == 0 {
                return;
            }
            (*player).message = b"Picked up an energy cell.\0".as_ptr();
        }
        SPR_CELP => {
            if P_GiveAmmo(player, am_cell, 5) == 0 {
                return;
            }
            (*player).message = b"Picked up an energy cell pack.\0".as_ptr();
        }
        SPR_SHEL => {
            if P_GiveAmmo(player, am_shell, 1) == 0 {
                return;
            }
            (*player).message = b"Picked up 4 shotgun shells.\0".as_ptr();
        }
        SPR_SBOX => {
            if P_GiveAmmo(player, am_shell, 5) == 0 {
                return;
            }
            (*player).message = b"Picked up a box of shotgun shells.\0".as_ptr();
        }
        SPR_BPAK => {
            if (*player).backpack == 0 {
                for i in 0..NUMAMMO {
                    (*player).maxammo[i] *= 2;
                }
                (*player).backpack = true_;
            }
            for i in 0..NUMAMMO {
                P_GiveAmmo(player, i as ammotype_t, 1);
            }
            (*player).message = b"Picked up a backpack full of ammo!\0".as_ptr();
        }
        SPR_BFUG => {
            if P_GiveWeapon(player, wp_bfg, false_) == 0 {
                return;
            }
            (*player).message = b"You got the BFG9000!  Oh, yes.\0".as_ptr();
            sound = sfx_wpnup;
        }
        SPR_MGUN => {
            if P_GiveWeapon(player, wp_chaingun, (*special).flags & MF_DROPPED) == 0 {
                return;
            }
            (*player).message = b"You got the chaingun!\0".as_ptr();
            sound = sfx_wpnup;
        }
        SPR_CSAW => {
            if P_GiveWeapon(player, wp_chainsaw, false_) == 0 {
                return;
            }
            (*player).message = b"A chainsaw!  Find some meat!\0".as_ptr();
            sound = sfx_wpnup;
        }
        SPR_LAUN => {
            if P_GiveWeapon(player, wp_missile, false_) == 0 {
                return;
            }
            (*player).message = b"You got the rocket launcher!\0".as_ptr();
            sound = sfx_wpnup;
        }
        SPR_PLAS => {
            if P_GiveWeapon(player, wp_plasma, false_) == 0 {
                return;
            }
            (*player).message = b"You got the plasma gun!\0".as_ptr();
            sound = sfx_wpnup;
        }
        SPR_SHOT => {
            if P_GiveWeapon(player, wp_shotgun, (*special).flags & MF_DROPPED) == 0 {
                return;
            }
            (*player).message = b"You got the shotgun!\0".as_ptr();
            sound = sfx_wpnup;
        }
        SPR_SGN2 => {
            if P_GiveWeapon(player, wp_supershotgun, (*special).flags & MF_DROPPED) == 0 {
                return;
            }
            (*player).message = b"You got the super shotgun!\0".as_ptr();
            sound = sfx_wpnup;
        }
        _ => {
            I_Error("P_SpecialThing: Unknown gettable thing");
        }
    }

    if (*special).flags & MF_COUNTITEM != 0 {
        (*player).itemcount += 1;
    }
    P_RemoveMobj(special);
    (*player).bonuscount += BONUSADD;
    if player == (&mut players[consoleplayer as usize] as *mut player_t) {
        S_StartSound(std::ptr::null_mut(), sound);
    }
}

pub unsafe fn P_KillMobj(source: *mut mobj_t, target: *mut mobj_t) {
    (*target).flags &= !(MF_SHOOTABLE | MF_FLOAT | MF_SKULLFLY);

    if (*target).type_ != MT_SKULL {
        (*target).flags &= !MF_NOGRAVITY;
    }

    (*target).flags |= MF_CORPSE | MF_DROPOFF;
    (*target).height >>= 2;

    if !source.is_null() && !(*source).player.is_null() {
        if (*target).flags & MF_COUNTKILL != 0 {
            (*(*source).player).killcount += 1;
        }
        if !(*target).player.is_null() {
            let pidx = ((*target).player as usize - players.as_ptr() as usize)
                / std::mem::size_of::<player_t>();
            (*(*source).player).frags[pidx] += 1;
        }
    } else if netgame == 0 && ((*target).flags & MF_COUNTKILL != 0) {
        players[0].killcount += 1;
    }

    if !(*target).player.is_null() {
        if source.is_null() {
            let pidx = ((*target).player as usize - players.as_ptr() as usize)
                / std::mem::size_of::<player_t>();
            (*(*target).player).frags[pidx] += 1;
        }

        (*target).flags &= !MF_SOLID;
        (*(*target).player).playerstate = PST_DEAD;
        P_DropWeapon((*target).player);

        if (*target).player == (&mut players[consoleplayer as usize] as *mut player_t)
            && automapactive != 0
        {
            AM_Stop();
        }
    }

    if (*target).health < -(*(*target).info).spawnhealth && (*(*target).info).xdeathstate != 0 {
        P_SetMobjState(target, (*(*target).info).xdeathstate);
    } else {
        P_SetMobjState(target, (*(*target).info).deathstate);
    }
    (*target).tics -= P_Random() & 3;

    if (*target).tics < 1 {
        (*target).tics = 1;
    }

    let item = match (*target).type_ {
        MT_WOLFSS | MT_POSSESSED => MT_CLIP,
        MT_SHOTGUY => MT_SHOTGUN,
        MT_CHAINGUY => MT_CHAINGUN,
        _ => return,
    };

    let mo = P_SpawnMobj((*target).x, (*target).y, ONFLOORZ, item);
    (*mo).flags |= MF_DROPPED;
}

pub unsafe fn P_DamageMobj(
    target: *mut mobj_t,
    inflictor: *mut mobj_t,
    source: *mut mobj_t,
    mut damage: i32,
) {
    if (*target).flags & MF_SHOOTABLE == 0 {
        return;
    }
    if (*target).health <= 0 {
        return;
    }

    if (*target).flags & MF_SKULLFLY != 0 {
        (*target).momx = 0;
        (*target).momy = 0;
        (*target).momz = 0;
    }

    let player = (*target).player;
    if !player.is_null() && gameskill == sk_baby {
        damage >>= 1;
    }

    if !inflictor.is_null()
        && ((*target).flags & MF_NOCLIP == 0)
        && (source.is_null()
            || (*source).player.is_null()
            || (*(*source).player).readyweapon != wp_chainsaw)
    {
        let mut ang =
            R_PointToAngle2((*inflictor).x, (*inflictor).y, (*target).x, (*target).y);

        let mut thrust = damage * (FRACUNIT >> 3) * 100 / (*(*target).info).mass;

        if damage < 40
            && damage > (*target).health
            && (*target).z - (*inflictor).z > 64 * FRACUNIT
            && (P_Random() & 1 != 0)
        {
            ang = ang.wrapping_add(ANG180);
            thrust *= 4;
        }

        let ang_idx = (ang >> ANGLETOFINESHIFT) as usize;
        (*target).momx += FixedMul(thrust, finecosine(ang_idx));
        (*target).momy += FixedMul(thrust, finesine[ang_idx]);
    }

    if !player.is_null() {
        if (*(*(*target).subsector).sector).special == 11 && damage >= (*target).health {
            damage = (*target).health - 1;
        }

        if damage < 1000
            && (((*player).cheats & CF_GODMODE != 0)
                || (*player).powers[pw_invulnerability as usize] != 0)
        {
            return;
        }

        if (*player).armortype != 0 {
            let mut saved = if (*player).armortype == 1 {
                damage / 3
            } else {
                damage / 2
            };

            if (*player).armorpoints <= saved {
                saved = (*player).armorpoints;
                (*player).armortype = 0;
            }
            (*player).armorpoints -= saved;
            damage -= saved;
        }
        (*player).health -= damage;
        if (*player).health < 0 {
            (*player).health = 0;
        }

        (*player).attacker = source;
        (*player).damagecount += damage;

        if (*player).damagecount > 100 {
            (*player).damagecount = 100;
        }

        let temp = if damage < 100 { damage } else { 100 };

        if player == (&mut players[consoleplayer as usize] as *mut player_t) {
            I_Tactile(40, 10, 40 + temp * 2);
        }
    }

    (*target).health -= damage;
    if (*target).health <= 0 {
        P_KillMobj(source, target);
        return;
    }

    if P_Random() < (*(*target).info).painchance && ((*target).flags & MF_SKULLFLY == 0) {
        (*target).flags |= MF_JUSTHIT;
        P_SetMobjState(target, (*(*target).info).painstate);
    }

    (*target).reactiontime = 0;

    if ((*target).threshold == 0 || (*target).type_ == MT_VILE)
        && !source.is_null()
        && source != target
        && (*source).type_ != MT_VILE
    {
        (*target).target = source;
        (*target).threshold = BASETHRESHOLD;
        if (*target).state == (&mut states[(*(*target).info).spawnstate as usize] as *mut state_t)
            && (*(*target).info).seestate != S_NULL
        {
            P_SetMobjState(target, (*(*target).info).seestate);
        }
    }
}
