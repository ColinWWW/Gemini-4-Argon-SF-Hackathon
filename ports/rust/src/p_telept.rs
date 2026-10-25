// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::d_think::*;
use crate::info::*;
use crate::p_map::*;
use crate::p_mobj::*;
use crate::p_tick::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::tables::*;

pub unsafe fn EV_Teleport(line: *mut line_t, side: i32, thing: *mut mobj_t) -> i32 {
    if (*thing).flags & MF_MISSILE != 0 {
        return 0;
    }

    if side == 1 {
        return 0;
    }

    let tag = (*line).tag;
    for i in 0..(numsectors as usize) {
        if (*sectors.add(i)).tag == tag {
            let mut thinker = thinkercap.next;
            while thinker != &mut thinkercap {
                if (*thinker).function != actionf_t::acp1(P_MobjThinker) {
                    thinker = (*thinker).next;
                    continue;
                }

                let m = thinker as *mut mobj_t;
                if (*m).type_ != MT_TELEPORTMAN {
                    thinker = (*thinker).next;
                    continue;
                }

                let sector = (*(*m).subsector).sector;
                let sec_idx =
                    (sector as usize - sectors as usize) / std::mem::size_of::<sector_t>();
                if sec_idx != i {
                    thinker = (*thinker).next;
                    continue;
                }

                let oldx = (*thing).x;
                let oldy = (*thing).y;
                let oldz = (*thing).z;

                if P_TeleportMove(thing, (*m).x, (*m).y) == 0 {
                    return 0;
                }

                (*thing).z = (*thing).floorz;
                if !(*thing).player.is_null() {
                    (*(*thing).player).viewz = (*thing).z + (*(*thing).player).viewheight;
                }

                let fog = P_SpawnMobj(oldx, oldy, oldz, MT_TFOG);
                S_StartSound(fog as *mut c_void, sfx_telept);
                let an = ((*m).angle >> ANGLETOFINESHIFT) as usize;
                let fog2 = P_SpawnMobj(
                    (*m).x + 20 * finecosine(an),
                    (*m).y + 20 * finesine[an],
                    (*thing).z,
                    MT_TFOG,
                );

                S_StartSound(fog2 as *mut c_void, sfx_telept);

                if !(*thing).player.is_null() {
                    (*thing).reactiontime = 18;
                }

                (*thing).angle = (*m).angle;
                (*thing).momx = 0;
                (*thing).momy = 0;
                (*thing).momz = 0;
                return 1;
            }
        }
    }
    0
}
