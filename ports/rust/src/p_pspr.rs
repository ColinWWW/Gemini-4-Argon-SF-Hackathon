// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_event::*;
use crate::d_items::*;
use crate::d_player::*;
use crate::d_think::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::m_random::*;
use crate::p_enemy::*;
use crate::p_inter::*;
use crate::p_local::*;
use crate::p_map::*;
use crate::p_mobj::*;
use crate::r_main::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;

pub const FF_FULLBRIGHT: i32 = 0x8000;
pub const FF_FRAMEMASK: i32 = 0x7fff;

pub type psprnum_t = usize;
pub const ps_weapon: usize = 0;
pub const ps_flash: usize = 1;
pub const NUMPSPRITES: usize = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pspdef_t {
    pub state: *mut state_t,
    pub tics: i32,
    pub sx: fixed_t,
    pub sy: fixed_t,
}

impl Default for pspdef_t {
    fn default() -> Self {
        Self {
            state: std::ptr::null_mut(),
            tics: 0,
            sx: 0,
            sy: 0,
        }
    }
}

pub const LOWERSPEED: fixed_t = FRACUNIT * 6;
pub const RAISESPEED: fixed_t = FRACUNIT * 6;
pub const WEAPONBOTTOM: fixed_t = 128 * FRACUNIT;
pub const WEAPONTOP: fixed_t = 32 * FRACUNIT;
pub const BFGCELLS: i32 = 40;

pub unsafe fn P_SetPsprite(player: *mut player_t, position: usize, mut stnum: statenum_t) {
    let psp = &mut (*player).psprites[position] as *mut pspdef_t;

    loop {
        if stnum == 0 {
            (*psp).state = std::ptr::null_mut();
            break;
        }

        let state = &mut states[stnum as usize] as *mut state_t;
        (*psp).state = state;
        (*psp).tics = (*state).tics;

        if (*state).misc1 != 0 {
            (*psp).sx = (*state).misc1 << FRACBITS;
            (*psp).sy = (*state).misc2 << FRACBITS;
        }

        if let actionf_t::acp2(f) = (*state).action {
            f(player as *mut c_void, psp as *mut c_void);
            if (*psp).state.is_null() {
                break;
            }
        }

        stnum = (*(*psp).state).nextstate;
        if (*psp).tics != 0 {
            break;
        }
    }
}

pub static mut swingx: fixed_t = 0;
pub static mut swingy: fixed_t = 0;

pub unsafe fn P_CalcSwing(player: *mut player_t) {
    let swing = (*player).bob;

    let mut angle = ((FINEANGLES as i32) / 70 * leveltime) as usize & FINEMASK;
    swingx = FixedMul(swing, finesine[angle]);

    angle = ((FINEANGLES as i32) / 70 * leveltime + (FINEANGLES as i32) / 2) as usize & FINEMASK;
    swingy = -FixedMul(swingx, finesine[angle]);
}

pub unsafe fn P_BringUpWeapon(player: *mut player_t) {
    if (*player).pendingweapon == wp_nochange {
        (*player).pendingweapon = (*player).readyweapon;
    }

    if (*player).pendingweapon == wp_chainsaw {
        S_StartSound((*player).mo as *mut c_void, sfx_sawup);
    }

    let newstate = weaponinfo[(*player).pendingweapon as usize].upstate;
    (*player).pendingweapon = wp_nochange;
    (*player).psprites[ps_weapon].sy = WEAPONBOTTOM;

    P_SetPsprite(player, ps_weapon, newstate);
}

pub unsafe fn P_CheckAmmo(player: *mut player_t) -> boolean {
    let ammo = weaponinfo[(*player).readyweapon as usize].ammo;

    let count = if (*player).readyweapon == wp_bfg {
        BFGCELLS
    } else if (*player).readyweapon == wp_supershotgun {
        2
    } else {
        1
    };

    if ammo == am_noammo || (*player).ammo[ammo as usize] >= count {
        return true_;
    }

    loop {
        if (*player).weaponowned[wp_plasma as usize] != 0
            && (*player).ammo[am_cell as usize] != 0
            && gamemode != shareware
        {
            (*player).pendingweapon = wp_plasma;
        } else if (*player).weaponowned[wp_supershotgun as usize] != 0
            && (*player).ammo[am_shell as usize] > 2
            && gamemode == commercial
        {
            (*player).pendingweapon = wp_supershotgun;
        } else if (*player).weaponowned[wp_chaingun as usize] != 0
            && (*player).ammo[am_clip as usize] != 0
        {
            (*player).pendingweapon = wp_chaingun;
        } else if (*player).weaponowned[wp_shotgun as usize] != 0
            && (*player).ammo[am_shell as usize] != 0
        {
            (*player).pendingweapon = wp_shotgun;
        } else if (*player).ammo[am_clip as usize] != 0 {
            (*player).pendingweapon = wp_pistol;
        } else if (*player).weaponowned[wp_chainsaw as usize] != 0 {
            (*player).pendingweapon = wp_chainsaw;
        } else if (*player).weaponowned[wp_missile as usize] != 0
            && (*player).ammo[am_misl as usize] != 0
        {
            (*player).pendingweapon = wp_missile;
        } else if (*player).weaponowned[wp_bfg as usize] != 0
            && (*player).ammo[am_cell as usize] > 40
            && gamemode != shareware
        {
            (*player).pendingweapon = wp_bfg;
        } else {
            (*player).pendingweapon = wp_fist;
        }

        if (*player).pendingweapon != wp_nochange {
            break;
        }
    }

    P_SetPsprite(
        player,
        ps_weapon,
        weaponinfo[(*player).readyweapon as usize].downstate,
    );

    false_
}

pub unsafe fn P_FireWeapon(player: *mut player_t) {
    if P_CheckAmmo(player) == 0 {
        return;
    }

    P_SetMobjState((*player).mo, S_PLAY_ATK1);
    let newstate = weaponinfo[(*player).readyweapon as usize].atkstate;
    P_SetPsprite(player, ps_weapon, newstate);
    P_NoiseAlert((*player).mo, (*player).mo);
}

pub unsafe fn P_DropWeapon(player: *mut player_t) {
    P_SetPsprite(
        player,
        ps_weapon,
        weaponinfo[(*player).readyweapon as usize].downstate,
    );
}

pub unsafe fn A_WeaponReady(player_ptr: *mut c_void, psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    let psp = psp_ptr as *mut pspdef_t;

    if (*(*player).mo).state == (&mut states[S_PLAY_ATK1 as usize] as *mut state_t)
        || (*(*player).mo).state == (&mut states[S_PLAY_ATK2 as usize] as *mut state_t)
    {
        P_SetMobjState((*player).mo, S_PLAY);
    }

    if (*player).readyweapon == wp_chainsaw
        && (*psp).state == (&mut states[S_SAW as usize] as *mut state_t)
    {
        S_StartSound((*player).mo as *mut c_void, sfx_sawidl);
    }

    if (*player).pendingweapon != wp_nochange || (*player).health == 0 {
        let newstate = weaponinfo[(*player).readyweapon as usize].downstate;
        P_SetPsprite(player, ps_weapon, newstate);
        return;
    }

    if (*player).cmd.buttons & BT_ATTACK != 0 {
        if (*player).attackdown == 0
            || ((*player).readyweapon != wp_missile && (*player).readyweapon != wp_bfg)
        {
            (*player).attackdown = true_;
            P_FireWeapon(player);
            return;
        }
    } else {
        (*player).attackdown = false_;
    }

    let mut angle = ((128 * leveltime) as usize) & FINEMASK;
    (*psp).sx = FRACUNIT + FixedMul((*player).bob, finecosine(angle));
    angle &= FINEANGLES / 2 - 1;
    (*psp).sy = WEAPONTOP + FixedMul((*player).bob, finesine[angle]);
}

pub unsafe fn A_ReFire(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    if ((*player).cmd.buttons & BT_ATTACK != 0)
        && (*player).pendingweapon == wp_nochange
        && (*player).health != 0
    {
        (*player).refire += 1;
        P_FireWeapon(player);
    } else {
        (*player).refire = 0;
        P_CheckAmmo(player);
    }
}

pub unsafe fn A_CheckReload(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    P_CheckAmmo(player);
}

pub unsafe fn A_Lower(player_ptr: *mut c_void, psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    let psp = psp_ptr as *mut pspdef_t;

    (*psp).sy += LOWERSPEED;

    if (*psp).sy < WEAPONBOTTOM {
        return;
    }

    if (*player).playerstate == PST_DEAD {
        (*psp).sy = WEAPONBOTTOM;
        return;
    }

    if (*player).health == 0 {
        P_SetPsprite(player, ps_weapon, S_NULL);
        return;
    }

    (*player).readyweapon = (*player).pendingweapon;
    P_BringUpWeapon(player);
}

pub unsafe fn A_Raise(player_ptr: *mut c_void, psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    let psp = psp_ptr as *mut pspdef_t;

    (*psp).sy -= RAISESPEED;

    if (*psp).sy > WEAPONTOP {
        return;
    }

    (*psp).sy = WEAPONTOP;
    let newstate = weaponinfo[(*player).readyweapon as usize].readystate;
    P_SetPsprite(player, ps_weapon, newstate);
}

pub unsafe fn A_GunFlash(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    P_SetPsprite(
        player,
        ps_flash,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );
}

pub unsafe fn A_Punch(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    let mut damage = ((P_Random() % 10) + 1) << 1;

    if (*player).powers[pw_strength as usize] != 0 {
        damage *= 10;
    }

    let mut angle = (*(*player).mo).angle;
    let r1 = P_Random();
    let r2 = P_Random();
    angle = angle.wrapping_add(((r1 - r2) << 18) as u32);
    let slope = P_AimLineAttack((*player).mo, angle, MELEERANGE);
    P_LineAttack((*player).mo, angle, MELEERANGE, slope, damage);

    if !linetarget.is_null() {
        S_StartSound((*player).mo as *mut c_void, sfx_punch);
        (*(*player).mo).angle = R_PointToAngle2(
            (*(*player).mo).x,
            (*(*player).mo).y,
            (*linetarget).x,
            (*linetarget).y,
        );
    }
}

pub unsafe fn A_Saw(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    let damage = 2 * ((P_Random() % 10) + 1);
    let mut angle = (*(*player).mo).angle;
    let r1 = P_Random();
    let r2 = P_Random();
    angle = angle.wrapping_add(((r1 - r2) << 18) as u32);

    let slope = P_AimLineAttack((*player).mo, angle, MELEERANGE + 1);
    P_LineAttack((*player).mo, angle, MELEERANGE + 1, slope, damage);

    if linetarget.is_null() {
        S_StartSound((*player).mo as *mut c_void, sfx_sawful);
        return;
    }
    S_StartSound((*player).mo as *mut c_void, sfx_sawhit);

    angle = R_PointToAngle2(
        (*(*player).mo).x,
        (*(*player).mo).y,
        (*linetarget).x,
        (*linetarget).y,
    );
    let diff = angle.wrapping_sub((*(*player).mo).angle);
    if diff > ANG180 {
        if (diff as i32) < -((ANG90 / 20) as i32) {
            (*(*player).mo).angle = angle.wrapping_add(ANG90 / 21);
        } else {
            (*(*player).mo).angle = (*(*player).mo).angle.wrapping_sub(ANG90 / 20);
        }
    } else {
        if diff > ANG90 / 20 {
            (*(*player).mo).angle = angle.wrapping_sub(ANG90 / 21);
        } else {
            (*(*player).mo).angle = (*(*player).mo).angle.wrapping_add(ANG90 / 20);
        }
    }
    (*(*player).mo).flags |= MF_JUSTATTACKED;
}

pub unsafe fn A_FireMissile(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= 1;
    P_SpawnPlayerMissile((*player).mo, MT_ROCKET);
}

pub unsafe fn A_FireBFG(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= BFGCELLS;
    P_SpawnPlayerMissile((*player).mo, MT_BFG);
}

pub unsafe fn A_FirePlasma(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= 1;

    P_SetPsprite(
        player,
        ps_flash,
        weaponinfo[(*player).readyweapon as usize].flashstate + (P_Random() & 1),
    );

    P_SpawnPlayerMissile((*player).mo, MT_PLASMA);
}

pub static mut bulletslope: fixed_t = 0;

pub unsafe fn P_BulletSlope(mo: *mut mobj_t) {
    let mut an = (*mo).angle;
    bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT);

    if linetarget.is_null() {
        an = an.wrapping_add(1 << 26);
        bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT);
        if linetarget.is_null() {
            an = an.wrapping_sub(2 << 26);
            bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT);
        }
    }
}

pub unsafe fn P_GunShot(mo: *mut mobj_t, accurate: boolean) {
    let damage = 5 * ((P_Random() % 3) + 1);
    let mut angle = (*mo).angle;

    if accurate == 0 {
        let r1 = P_Random();
        let r2 = P_Random();
        angle = angle.wrapping_add(((r1 - r2) << 18) as u32);
    }

    P_LineAttack(mo, angle, MISSILERANGE, bulletslope, damage);
}

pub unsafe fn A_FirePistol(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_pistol);

    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= 1;

    P_SetPsprite(
        player,
        ps_flash,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );

    P_BulletSlope((*player).mo);
    P_GunShot((*player).mo, ((*player).refire == 0) as boolean);
}

pub unsafe fn A_FireShotgun(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_shotgn);
    P_SetMobjState((*player).mo, S_PLAY_ATK2);

    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= 1;

    P_SetPsprite(
        player,
        ps_flash,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );

    P_BulletSlope((*player).mo);

    for _ in 0..7 {
        P_GunShot((*player).mo, false_);
    }
}

pub unsafe fn A_FireShotgun2(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_dshtgn);
    P_SetMobjState((*player).mo, S_PLAY_ATK2);

    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= 2;

    P_SetPsprite(
        player,
        ps_flash,
        weaponinfo[(*player).readyweapon as usize].flashstate,
    );

    P_BulletSlope((*player).mo);

    for _ in 0..20 {
        let damage = 5 * ((P_Random() % 3) + 1);
        let mut angle = (*(*player).mo).angle;
        let r1 = P_Random();
        let r2 = P_Random();
        angle = angle.wrapping_add(((r1 - r2) << 19) as u32);
        let r3 = P_Random();
        let r4 = P_Random();
        P_LineAttack(
            (*player).mo,
            angle,
            MISSILERANGE,
            bulletslope + ((r3 - r4) << 5),
            damage,
        );
    }
}

pub unsafe fn A_FireCGun(player_ptr: *mut c_void, psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    let psp = psp_ptr as *mut pspdef_t;

    S_StartSound((*player).mo as *mut c_void, sfx_pistol);

    if (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] == 0 {
        return;
    }

    P_SetMobjState((*player).mo, S_PLAY_ATK2);
    (*player).ammo[weaponinfo[(*player).readyweapon as usize].ammo as usize] -= 1;

    let offset = (((*psp).state as usize) - (&states[S_CHAIN1 as usize] as *const state_t as usize))
        / std::mem::size_of::<state_t>();

    P_SetPsprite(
        player,
        ps_flash,
        weaponinfo[(*player).readyweapon as usize].flashstate + (offset as i32),
    );

    P_BulletSlope((*player).mo);
    P_GunShot((*player).mo, ((*player).refire == 0) as boolean);
}

pub unsafe fn A_Light0(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    (*player).extralight = 0;
}

pub unsafe fn A_Light1(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    (*player).extralight = 1;
}

pub unsafe fn A_Light2(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    (*player).extralight = 2;
}

pub unsafe fn A_BFGSpray(mo_ptr: *mut c_void) {
    let mo = mo_ptr as *mut mobj_t;
    for i in 0..40 {
        let an = (*mo)
            .angle
            .wrapping_sub(ANG90 / 2)
            .wrapping_add((ANG90 / 40) * (i as u32));

        P_AimLineAttack((*mo).target, an, 16 * 64 * FRACUNIT);

        if linetarget.is_null() {
            continue;
        }

        P_SpawnMobj(
            (*linetarget).x,
            (*linetarget).y,
            (*linetarget).z + ((*linetarget).height >> 2),
            MT_EXTRABFG,
        );

        let mut damage = 0;
        for _ in 0..15 {
            damage += (P_Random() & 7) + 1;
        }

        P_DamageMobj(linetarget, (*mo).target, (*mo).target, damage);
    }
}

pub unsafe fn A_BFGsound(player_ptr: *mut c_void, _psp_ptr: *mut c_void) {
    let player = player_ptr as *mut player_t;
    S_StartSound((*player).mo as *mut c_void, sfx_bfg);
}

pub unsafe fn P_SetupPsprites(player: *mut player_t) {
    for i in 0..NUMPSPRITES {
        (*player).psprites[i].state = std::ptr::null_mut();
    }

    (*player).pendingweapon = (*player).readyweapon;
    P_BringUpWeapon(player);
}

pub unsafe fn P_MovePsprites(player: *mut player_t) {
    for i in 0..NUMPSPRITES {
        let psp = &mut (*player).psprites[i];
        if !psp.state.is_null() {
            if psp.tics != -1 {
                psp.tics -= 1;
                if psp.tics == 0 {
                    let next = (*psp.state).nextstate;
                    P_SetPsprite(player, i, next);
                }
            }
        }
    }

    (*player).psprites[ps_flash].sx = (*player).psprites[ps_weapon].sx;
    (*player).psprites[ps_flash].sy = (*player).psprites[ps_weapon].sy;
}
