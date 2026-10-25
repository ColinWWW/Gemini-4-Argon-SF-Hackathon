// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_event::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
use crate::hu_stuff::*;
use crate::info::*;
use crate::m_swap::*;
use crate::p_pspr::*;
use crate::r_defs::*;
use crate::r_state::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;

pub static mut finalestage: i32 = 0;
pub static mut finalecount: i32 = 0;

pub const TEXTSPEED: i32 = 3;
pub const TEXTWAIT: i32 = 250;

pub static mut finaletext: &'static str = "";
pub static mut finaleflat: &'static str = "";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct castinfo_t {
    pub name: Option<&'static str>,
    pub type_: mobjtype_t,
}

pub static mut castorder: [castinfo_t; 18] = [
    castinfo_t {
        name: Some(CC_ZOMBIE),
        type_: MT_POSSESSED,
    },
    castinfo_t {
        name: Some(CC_SHOTGUN),
        type_: MT_SHOTGUY,
    },
    castinfo_t {
        name: Some(CC_HEAVY),
        type_: MT_CHAINGUY,
    },
    castinfo_t {
        name: Some(CC_IMP),
        type_: MT_TROOP,
    },
    castinfo_t {
        name: Some(CC_DEMON),
        type_: MT_SERGEANT,
    },
    castinfo_t {
        name: Some(CC_LOST),
        type_: MT_SKULL,
    },
    castinfo_t {
        name: Some(CC_CACO),
        type_: MT_HEAD,
    },
    castinfo_t {
        name: Some(CC_HELL),
        type_: MT_KNIGHT,
    },
    castinfo_t {
        name: Some(CC_BARON),
        type_: MT_BRUISER,
    },
    castinfo_t {
        name: Some(CC_ARACH),
        type_: MT_BABY,
    },
    castinfo_t {
        name: Some(CC_PAIN),
        type_: MT_PAIN,
    },
    castinfo_t {
        name: Some(CC_REVEN),
        type_: MT_UNDEAD,
    },
    castinfo_t {
        name: Some(CC_MANCU),
        type_: MT_FATSO,
    },
    castinfo_t {
        name: Some(CC_ARCH),
        type_: MT_VILE,
    },
    castinfo_t {
        name: Some(CC_SPIDER),
        type_: MT_SPIDER,
    },
    castinfo_t {
        name: Some(CC_CYBER),
        type_: MT_CYBORG,
    },
    castinfo_t {
        name: Some(CC_HERO),
        type_: MT_PLAYER,
    },
    castinfo_t {
        name: None,
        type_: 0,
    },
];

pub static mut castnum: usize = 0;
pub static mut casttics: i32 = 0;
pub static mut caststate: *mut state_t = std::ptr::null_mut();
pub static mut castdeath: boolean = 0;
pub static mut castframes: i32 = 0;
pub static mut castonmelee: i32 = 0;
pub static mut castattacking: boolean = 0;

pub unsafe fn F_StartFinale() {
    gameaction = ga_nothing;
    gamestate = GS_FINALE;
    viewactive = false_;
    automapactive = false_;

    match gamemode {
        shareware | registered | retail => {
            S_ChangeMusic(mus_victor, true_);
            match gameepisode {
                1 => {
                    finaleflat = "FLOOR4_8";
                    finaletext = E1TEXT;
                }
                2 => {
                    finaleflat = "SFLR6_1";
                    finaletext = E2TEXT;
                }
                3 => {
                    finaleflat = "MFLR8_4";
                    finaletext = E3TEXT;
                }
                4 => {
                    finaleflat = "MFLR8_3";
                    finaletext = E4TEXT;
                }
                _ => {}
            }
        }
        commercial => {
            S_ChangeMusic(mus_read_m, true_);
            match gamemap {
                6 => {
                    finaleflat = "SLIME16";
                    finaletext = C1TEXT;
                }
                11 => {
                    finaleflat = "RROCK14";
                    finaletext = C2TEXT;
                }
                20 => {
                    finaleflat = "RROCK07";
                    finaletext = C3TEXT;
                }
                30 => {
                    finaleflat = "RROCK17";
                    finaletext = C4TEXT;
                }
                15 => {
                    finaleflat = "RROCK13";
                    finaletext = C5TEXT;
                }
                31 => {
                    finaleflat = "RROCK19";
                    finaletext = C6TEXT;
                }
                _ => {}
            }
        }
        _ => {
            S_ChangeMusic(mus_read_m, true_);
            finaleflat = "F_SKY1";
            finaletext = C1TEXT;
        }
    }

    finalestage = 0;
    finalecount = 0;
}

pub unsafe fn F_Responder(event: *mut event_t) -> boolean {
    if finalestage == 2 {
        return F_CastResponder(event);
    }
    false_
}

pub unsafe fn F_Ticker() {
    if gamemode == commercial && finalecount > 50 {
        let mut i = 0;
        while i < MAXPLAYERS {
            if players[i].cmd.buttons != 0 {
                break;
            }
            i += 1;
        }

        if i < MAXPLAYERS {
            if gamemap == 30 {
                F_StartCast();
            } else {
                gameaction = ga_worlddone;
            }
        }
    }

    finalecount += 1;

    if finalestage == 2 {
        F_CastTicker();
        return;
    }

    if gamemode == commercial {
        return;
    }

    if finalestage == 0 && finalecount > (finaletext.len() as i32) * TEXTSPEED + TEXTWAIT {
        finalecount = 0;
        finalestage = 1;
        crate::d_main::wipegamestate = -1;
        if gameepisode == 3 {
            S_StartMusic(mus_bunny);
        }
    }
}

pub unsafe fn F_TextWrite() {
    let src = W_CacheLumpNameStr(finaleflat, PU_CACHE) as *mut byte;
    let mut dest = screens[0];

    for y in 0..SCREENHEIGHT {
        for _ in 0..(SCREENWIDTH / 64) {
            std::ptr::copy_nonoverlapping(src.add(((y & 63) << 6) as usize), dest, 64);
            dest = dest.add(64);
        }
        if (SCREENWIDTH & 63) != 0 {
            std::ptr::copy_nonoverlapping(
                src.add(((y & 63) << 6) as usize),
                dest,
                (SCREENWIDTH & 63) as usize,
            );
            dest = dest.add((SCREENWIDTH & 63) as usize);
        }
    }

    V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);

    let mut cx = 10;
    let mut cy = 10;
    let bytes = finaletext.as_bytes();
    let mut idx = 0;

    let mut count = (finalecount - 10) / TEXTSPEED;
    if count < 0 {
        count = 0;
    }

    while count > 0 {
        count -= 1;
        if idx >= bytes.len() {
            break;
        }
        let c = bytes[idx];
        idx += 1;
        if c == 0 {
            break;
        }
        if c == b'\n' {
            cx = 10;
            cy += 11;
            continue;
        }

        let glyph = (c.to_ascii_uppercase() as i32) - (HU_FONTSTART as i32);
        if glyph < 0 || (glyph as usize) > HU_FONTSIZE {
            cx += 4;
            continue;
        }

        let w = SHORT((*hu_font[glyph as usize]).width) as i32;
        if cx + w > SCREENWIDTH {
            break;
        }
        V_DrawPatch(cx, cy, 0, hu_font[glyph as usize]);
        cx += w;
    }
}

pub unsafe fn F_StartCast() {
    crate::d_main::wipegamestate = -1;
    castnum = 0;
    caststate = &mut states[mobjinfo[castorder[castnum].type_ as usize].seestate as usize];
    casttics = (*caststate).tics;
    castdeath = false_;
    finalestage = 2;
    castframes = 0;
    castonmelee = 0;
    castattacking = false_;
    S_ChangeMusic(mus_evil, true_);
}

pub unsafe fn F_CastTicker() {
    casttics -= 1;
    if casttics > 0 {
        return;
    }

    if (*caststate).tics == -1 || (*caststate).nextstate == S_NULL {
        castnum += 1;
        castdeath = false_;
        if castorder[castnum].name.is_none() {
            castnum = 0;
        }
        if mobjinfo[castorder[castnum].type_ as usize].seesound != 0 {
            S_StartSound(
                std::ptr::null_mut(),
                mobjinfo[castorder[castnum].type_ as usize].seesound,
            );
        }
        caststate = &mut states[mobjinfo[castorder[castnum].type_ as usize].seestate as usize];
        castframes = 0;
    } else {
        if caststate == (&mut states[S_PLAY_ATK1 as usize] as *mut state_t) {
            castattacking = false_;
            castframes = 0;
            caststate = &mut states[mobjinfo[castorder[castnum].type_ as usize].seestate as usize];
            casttics = (*caststate).tics;
            if casttics == -1 {
                casttics = 15;
            }
            return;
        }
        let st = (*caststate).nextstate;
        caststate = &mut states[st as usize];
        castframes += 1;

        let sfx = match st {
            S_PLAY_ATK1 => sfx_dshtgn,
            S_POSS_ATK2 => sfx_pistol,
            S_SPOS_ATK2 => sfx_shotgn,
            S_VILE_ATK2 => sfx_vilatk,
            S_SKEL_FIST2 => sfx_skeswg,
            S_SKEL_FIST4 => sfx_skepch,
            S_SKEL_MISS2 => sfx_skeatk,
            S_FATT_ATK8 | S_FATT_ATK5 | S_FATT_ATK2 => sfx_firsht,
            S_CPOS_ATK2 | S_CPOS_ATK3 | S_CPOS_ATK4 => sfx_shotgn,
            S_TROO_ATK3 => sfx_claw,
            S_SARG_ATK2 => sfx_sgtatk,
            S_BOSS_ATK2 | S_BOS2_ATK2 | S_HEAD_ATK2 => sfx_firsht,
            S_SKULL_ATK2 => sfx_sklatk,
            S_SPID_ATK2 | S_SPID_ATK3 => sfx_shotgn,
            S_BSPI_ATK2 => sfx_plasma,
            S_CYBER_ATK2 | S_CYBER_ATK4 | S_CYBER_ATK6 => sfx_rlaunc,
            S_PAIN_ATK3 => sfx_sklatk,
            _ => 0,
        };

        if sfx != 0 {
            S_StartSound(std::ptr::null_mut(), sfx);
        }
    }

    if castframes == 12 {
        castattacking = true_;
        if castonmelee != 0 {
            caststate =
                &mut states[mobjinfo[castorder[castnum].type_ as usize].meleestate as usize];
        } else {
            caststate =
                &mut states[mobjinfo[castorder[castnum].type_ as usize].missilestate as usize];
        }
        castonmelee ^= 1;
        if caststate == (&mut states[S_NULL as usize] as *mut state_t) {
            if castonmelee != 0 {
                caststate =
                    &mut states[mobjinfo[castorder[castnum].type_ as usize].meleestate as usize];
            } else {
                caststate =
                    &mut states[mobjinfo[castorder[castnum].type_ as usize].missilestate as usize];
            }
        }
    }

    if castattacking != 0 {
        if castframes == 24
            || caststate
                == (&mut states[mobjinfo[castorder[castnum].type_ as usize].seestate as usize]
                    as *mut state_t)
        {
            castattacking = false_;
            castframes = 0;
            caststate = &mut states[mobjinfo[castorder[castnum].type_ as usize].seestate as usize];
        }
    }

    casttics = (*caststate).tics;
    if casttics == -1 {
        casttics = 15;
    }
}

pub unsafe fn F_CastResponder(ev: *mut event_t) -> boolean {
    if (*ev).type_ != ev_keydown {
        return false_;
    }
    if castdeath != 0 {
        return true_;
    }

    castdeath = true_;
    caststate = &mut states[mobjinfo[castorder[castnum].type_ as usize].deathstate as usize];
    casttics = (*caststate).tics;
    castframes = 0;
    castattacking = false_;
    if mobjinfo[castorder[castnum].type_ as usize].deathsound != 0 {
        S_StartSound(
            std::ptr::null_mut(),
            mobjinfo[castorder[castnum].type_ as usize].deathsound,
        );
    }

    true_
}

pub unsafe fn F_CastPrint(text: &str) {
    let mut width = 0;
    for &b in text.as_bytes() {
        if b == 0 {
            break;
        }
        let c = (b.to_ascii_uppercase() as i32) - (HU_FONTSTART as i32);
        if c < 0 || (c as usize) > HU_FONTSIZE {
            width += 4;
            continue;
        }
        width += SHORT((*hu_font[c as usize]).width) as i32;
    }

    let mut cx = 160 - width / 2;
    for &b in text.as_bytes() {
        if b == 0 {
            break;
        }
        let c = (b.to_ascii_uppercase() as i32) - (HU_FONTSTART as i32);
        if c < 0 || (c as usize) > HU_FONTSIZE {
            cx += 4;
            continue;
        }
        let w = SHORT((*hu_font[c as usize]).width) as i32;
        V_DrawPatch(cx, 180, 0, hu_font[c as usize]);
        cx += w;
    }
}

pub unsafe fn F_CastDrawer() {
    V_DrawPatch(
        0,
        0,
        0,
        W_CacheLumpNameStr("BOSSBACK", PU_CACHE) as *mut patch_t,
    );

    if let Some(name) = castorder[castnum].name {
        F_CastPrint(name);
    }

    let sprdef = sprites.add((*caststate).sprite as usize);
    let sprframe = (*sprdef)
        .spriteframes
        .add(((*caststate).frame & FF_FRAMEMASK) as usize);
    let lump = (*sprframe).lump[0] as i32;
    let flip = (*sprframe).flip[0] as boolean;

    let patch = W_CacheLumpNum(lump + firstspritelump, PU_CACHE) as *mut patch_t;
    if flip != 0 {
        V_DrawPatchFlipped(160, 170, 0, patch);
    } else {
        V_DrawPatch(160, 170, 0, patch);
    }
}

pub unsafe fn F_DrawPatchCol(x: i32, patch: *mut patch_t, col: i32) {
    let columnofs_ptr = (*patch).columnofs.as_ptr();
    let mut column = (patch as *const byte).add(LONG(*columnofs_ptr.add(col as usize)) as usize)
        as *const column_t;
    let desttop = screens[0].add(x as usize);

    while (*column).topdelta != 0xff {
        let mut source = (column as *const byte).add(3);
        let mut dest = desttop.add((((*column).topdelta as i32) * SCREENWIDTH) as usize);
        let mut count = (*column).length as i32;

        while count > 0 {
            count -= 1;
            *dest = *source;
            source = source.add(1);
            dest = dest.add(SCREENWIDTH as usize);
        }
        column = (column as *const byte).add(((*column).length as usize) + 4) as *const column_t;
    }
}

static mut laststage: i32 = 0;

pub unsafe fn F_BunnyScroll() {
    let p1 = W_CacheLumpNameStr("PFUB2", PU_LEVEL) as *mut patch_t;
    let p2 = W_CacheLumpNameStr("PFUB1", PU_LEVEL) as *mut patch_t;

    V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);

    let mut scrolled = 320 - (finalecount - 230) / 2;
    if scrolled > 320 {
        scrolled = 320;
    }
    if scrolled < 0 {
        scrolled = 0;
    }

    for x in 0..SCREENWIDTH {
        if x + scrolled < 320 {
            F_DrawPatchCol(x, p1, x + scrolled);
        } else {
            F_DrawPatchCol(x, p2, x + scrolled - 320);
        }
    }

    if finalecount < 1130 {
        return;
    }
    if finalecount < 1180 {
        V_DrawPatch(
            (SCREENWIDTH - 13 * 8) / 2,
            (SCREENHEIGHT - 8 * 8) / 2,
            0,
            W_CacheLumpNameStr("END0", PU_CACHE) as *mut patch_t,
        );
        laststage = 0;
        return;
    }

    let mut stage = (finalecount - 1180) / 5;
    if stage > 6 {
        stage = 6;
    }
    if stage > laststage {
        S_StartSound(std::ptr::null_mut(), sfx_pistol);
        laststage = stage;
    }

    let name = format!("END{}", stage);
    V_DrawPatch(
        (SCREENWIDTH - 13 * 8) / 2,
        (SCREENHEIGHT - 8 * 8) / 2,
        0,
        W_CacheLumpNameStr(&name, PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn F_Drawer() {
    if finalestage == 2 {
        F_CastDrawer();
        return;
    }

    if finalestage == 0 {
        F_TextWrite();
    } else {
        match gameepisode {
            1 => {
                if gamemode == retail {
                    V_DrawPatch(
                        0,
                        0,
                        0,
                        W_CacheLumpNameStr("CREDIT", PU_CACHE) as *mut patch_t,
                    );
                } else {
                    V_DrawPatch(
                        0,
                        0,
                        0,
                        W_CacheLumpNameStr("HELP2", PU_CACHE) as *mut patch_t,
                    );
                }
            }
            2 => {
                V_DrawPatch(
                    0,
                    0,
                    0,
                    W_CacheLumpNameStr("VICTORY2", PU_CACHE) as *mut patch_t,
                );
            }
            3 => {
                F_BunnyScroll();
            }
            4 => {
                V_DrawPatch(
                    0,
                    0,
                    0,
                    W_CacheLumpNameStr("ENDPIC", PU_CACHE) as *mut patch_t,
                );
            }
            _ => {}
        }
    }
}
