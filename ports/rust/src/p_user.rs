// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_event::*;
use crate::d_player::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::info::*;
use crate::m_fixed::*;
use crate::p_local::*;
use crate::p_map::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::p_spec::*;
use crate::r_main::*;
use crate::tables::*;

pub const INVERSECOLORMAP: i32 = 32;
pub const MAXBOB: fixed_t = 0x100000;

pub static mut onground: boolean = 0;

pub unsafe fn P_Thrust(player: *mut player_t, angle: angle_t, move_: fixed_t) {
    let ang_idx = (angle >> ANGLETOFINESHIFT) as usize;
    (*(*player).mo).momx += FixedMul(move_, finecosine(ang_idx));
    (*(*player).mo).momy += FixedMul(move_, finesine[ang_idx]);
}

pub unsafe fn P_CalcHeight(player: *mut player_t) {
    (*player).bob = FixedMul((*(*player).mo).momx, (*(*player).mo).momx)
        + FixedMul((*(*player).mo).momy, (*(*player).mo).momy);

    (*player).bob >>= 2;

    if (*player).bob > MAXBOB {
        (*player).bob = MAXBOB;
    }

    if ((*player).cheats & CF_NOMOMENTUM != 0) || onground == 0 {
        (*player).viewz = (*(*player).mo).z + VIEWHEIGHT;
        if (*player).viewz > (*(*player).mo).ceilingz - 4 * FRACUNIT {
            (*player).viewz = (*(*player).mo).ceilingz - 4 * FRACUNIT;
        }
        (*player).viewz = (*(*player).mo).z + (*player).viewheight;
        return;
    }

    let angle = (((FINEANGLES as i32) / 20 * leveltime) as usize) & FINEMASK;
    let bob = FixedMul((*player).bob / 2, finesine[angle]);

    if (*player).playerstate == PST_LIVE {
        (*player).viewheight += (*player).deltaviewheight;

        if (*player).viewheight > VIEWHEIGHT {
            (*player).viewheight = VIEWHEIGHT;
            (*player).deltaviewheight = 0;
        }

        if (*player).viewheight < VIEWHEIGHT / 2 {
            (*player).viewheight = VIEWHEIGHT / 2;
            if (*player).deltaviewheight <= 0 {
                (*player).deltaviewheight = 1;
            }
        }

        if (*player).deltaviewheight != 0 {
            (*player).deltaviewheight += FRACUNIT / 4;
            if (*player).deltaviewheight == 0 {
                (*player).deltaviewheight = 1;
            }
        }
    }
    (*player).viewz = (*(*player).mo).z + (*player).viewheight + bob;

    if (*player).viewz > (*(*player).mo).ceilingz - 4 * FRACUNIT {
        (*player).viewz = (*(*player).mo).ceilingz - 4 * FRACUNIT;
    }
}

pub unsafe fn P_MovePlayer(player: *mut player_t) {
    let cmd = &mut (*player).cmd;

    (*(*player).mo).angle = (*(*player).mo)
        .angle
        .wrapping_add(((cmd.angleturn as i32) << 16) as u32);

    onground = ((*(*player).mo).z <= (*(*player).mo).floorz) as boolean;

    if cmd.forwardmove != 0 && onground != 0 {
        P_Thrust(player, (*(*player).mo).angle, (cmd.forwardmove as i32) * 2048);
    }

    if cmd.sidemove != 0 && onground != 0 {
        P_Thrust(
            player,
            (*(*player).mo).angle.wrapping_sub(ANG90),
            (cmd.sidemove as i32) * 2048,
        );
    }

    if (cmd.forwardmove != 0 || cmd.sidemove != 0)
        && (*(*player).mo).state == (&mut states[S_PLAY as usize] as *mut state_t)
    {
        P_SetMobjState((*player).mo, S_PLAY_RUN1);
    }
}

pub const ANG5: u32 = ANG90 / 18;

pub unsafe fn P_DeathThink(player: *mut player_t) {
    P_MovePsprites(player);

    if (*player).viewheight > 6 * FRACUNIT {
        (*player).viewheight -= FRACUNIT;
    }

    if (*player).viewheight < 6 * FRACUNIT {
        (*player).viewheight = 6 * FRACUNIT;
    }

    (*player).deltaviewheight = 0;
    onground = ((*(*player).mo).z <= (*(*player).mo).floorz) as boolean;
    P_CalcHeight(player);

    if !(*player).attacker.is_null() && (*player).attacker != (*player).mo {
        let angle = R_PointToAngle2(
            (*(*player).mo).x,
            (*(*player).mo).y,
            (*(*player).attacker).x,
            (*(*player).attacker).y,
        );

        let delta = angle.wrapping_sub((*(*player).mo).angle);

        if delta < ANG5 || delta > 0u32.wrapping_sub(ANG5) {
            (*(*player).mo).angle = angle;
            if (*player).damagecount != 0 {
                (*player).damagecount -= 1;
            }
        } else if delta < ANG180 {
            (*(*player).mo).angle = (*(*player).mo).angle.wrapping_add(ANG5);
        } else {
            (*(*player).mo).angle = (*(*player).mo).angle.wrapping_sub(ANG5);
        }
    } else if (*player).damagecount != 0 {
        (*player).damagecount -= 1;
    }

    if (*player).cmd.buttons & BT_USE != 0 {
        (*player).playerstate = PST_REBORN;
    }
}

pub unsafe fn P_PlayerThink(player: *mut player_t) {
    if (*player).cheats & CF_NOCLIP != 0 {
        (*(*player).mo).flags |= MF_NOCLIP;
    } else {
        (*(*player).mo).flags &= !MF_NOCLIP;
    }

    let cmd = &mut (*player).cmd;
    if (*(*player).mo).flags & MF_JUSTATTACKED != 0 {
        cmd.angleturn = 0;
        cmd.forwardmove = (0xc800 / 512) as i8;
        cmd.sidemove = 0;
        (*(*player).mo).flags &= !MF_JUSTATTACKED;
    }

    if (*player).playerstate == PST_DEAD {
        P_DeathThink(player);
        return;
    }

    if (*(*player).mo).reactiontime != 0 {
        (*(*player).mo).reactiontime -= 1;
    } else {
        P_MovePlayer(player);
    }

    P_CalcHeight(player);

    if (*(*(*(*player).mo).subsector).sector).special != 0 {
        P_PlayerInSpecialSector(player);
    }

    if cmd.buttons & BT_SPECIAL != 0 {
        cmd.buttons = 0;
    }

    if cmd.buttons & BT_CHANGE != 0 {
        let mut newweapon = ((cmd.buttons & BT_WEAPONMASK) >> BT_WEAPONSHIFT) as weapontype_t;

        if newweapon == wp_fist
            && (*player).weaponowned[wp_chainsaw as usize] != 0
            && !((*player).readyweapon == wp_chainsaw
                && (*player).powers[pw_strength as usize] != 0)
        {
            newweapon = wp_chainsaw;
        }

        if gamemode == commercial
            && newweapon == wp_shotgun
            && (*player).weaponowned[wp_supershotgun as usize] != 0
            && (*player).readyweapon != wp_supershotgun
        {
            newweapon = wp_supershotgun;
        }

        if (*player).weaponowned[newweapon as usize] != 0 && newweapon != (*player).readyweapon {
            if (newweapon != wp_plasma && newweapon != wp_bfg) || gamemode != shareware {
                (*player).pendingweapon = newweapon;
            }
        }
    }

    if cmd.buttons & BT_USE != 0 {
        if (*player).usedown == 0 {
            P_UseLines(player);
            (*player).usedown = true_;
        }
    } else {
        (*player).usedown = false_;
    }

    P_MovePsprites(player);

    if (*player).powers[pw_strength as usize] != 0 {
        (*player).powers[pw_strength as usize] += 1;
    }

    if (*player).powers[pw_invulnerability as usize] != 0 {
        (*player).powers[pw_invulnerability as usize] -= 1;
    }

    if (*player).powers[pw_invisibility as usize] != 0 {
        (*player).powers[pw_invisibility as usize] -= 1;
        if (*player).powers[pw_invisibility as usize] == 0 {
            (*(*player).mo).flags &= !MF_SHADOW;
        }
    }

    if (*player).powers[pw_infrared as usize] != 0 {
        (*player).powers[pw_infrared as usize] -= 1;
    }

    if (*player).powers[pw_ironfeet as usize] != 0 {
        (*player).powers[pw_ironfeet as usize] -= 1;
    }

    if (*player).damagecount != 0 {
        (*player).damagecount -= 1;
    }

    if (*player).bonuscount != 0 {
        (*player).bonuscount -= 1;
    }

    if (*player).powers[pw_invulnerability as usize] != 0 {
        if (*player).powers[pw_invulnerability as usize] > 4 * 32
            || ((*player).powers[pw_invulnerability as usize] & 8 != 0)
        {
            (*player).fixedcolormap = INVERSECOLORMAP;
        } else {
            (*player).fixedcolormap = 0;
        }
    } else if (*player).powers[pw_infrared as usize] != 0 {
        if (*player).powers[pw_infrared as usize] > 4 * 32
            || ((*player).powers[pw_infrared as usize] & 8 != 0)
        {
            (*player).fixedcolormap = 1;
        } else {
            (*player).fixedcolormap = 0;
        }
    } else {
        (*player).fixedcolormap = 0;
    }
}
