// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::am_map::*;
use crate::d_event::*;
use crate::d_main::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::d_ticcmd::*;
use crate::doomdata::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
use crate::f_finale::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::info::*;
use crate::m_argv::*;
use crate::m_fixed::*;
use crate::m_menu::*;
use crate::m_misc::*;
use crate::m_random::*;
use crate::p_inter::*;
use crate::p_local::*;
use crate::p_map::*;
use crate::p_mobj::*;
use crate::p_saveg::*;
use crate::p_setup::*;
use crate::p_tick::*;
use crate::r_data::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_main::*;
use crate::r_sky::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::st_stuff::*;
use crate::tables::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::wi_stuff::*;
use crate::z_zone::*;
use crate::{c_atoi, c_strcat, c_strcmp, c_strcpy, c_str, write_c_str};

pub const SAVEGAMESIZE: usize = 0x2c000;
pub const SAVESTRINGSIZE: usize = 24;

pub static mut gameaction: gameaction_t = 0;
pub static mut gamestate: gamestate_t = 0;
pub static mut gameskill: skill_t = 0;
pub static mut respawnmonsters: boolean = 0;
pub static mut gameepisode: i32 = 0;
pub static mut gamemap: i32 = 0;

pub static mut paused: boolean = 0;
pub static mut sendpause: boolean = 0;
pub static mut sendsave: boolean = 0;
pub static mut usergame: boolean = 0;

pub static mut timingdemo: boolean = 0;
pub static mut nodrawers: boolean = 0;
pub static mut noblit: boolean = 0;
pub static mut starttime: i32 = 0;

pub static mut viewactive: boolean = 0;

pub static mut deathmatch: boolean = 0;
pub static mut netgame: boolean = 0;
pub static mut playeringame: [boolean; MAXPLAYERS] = [0; MAXPLAYERS];
pub static mut players: [player_t; MAXPLAYERS] = [player_t {
    mo: std::ptr::null_mut(),
    playerstate: 0,
    cmd: ticcmd_t {
        forwardmove: 0,
        sidemove: 0,
        angleturn: 0,
        consistancy: 0,
        chatchar: 0,
        buttons: 0,
    },
    viewz: 0,
    viewheight: 0,
    deltaviewheight: 0,
    bob: 0,
    health: 0,
    armorpoints: 0,
    armortype: 0,
    powers: [0; NUMPOWERS],
    cards: [0; NUMCARDS],
    backpack: 0,
    frags: [0; MAXPLAYERS],
    readyweapon: 0,
    pendingweapon: 0,
    weaponowned: [0; NUMWEAPONS],
    ammo: [0; NUMAMMO],
    maxammo: [0; NUMAMMO],
    attackdown: 0,
    usedown: 0,
    cheats: 0,
    refire: 0,
    killcount: 0,
    itemcount: 0,
    secretcount: 0,
    message: std::ptr::null(),
    damagecount: 0,
    bonuscount: 0,
    attacker: std::ptr::null_mut(),
    extralight: 0,
    fixedcolormap: 0,
    colormap: 0,
    psprites: [pspdef_t {
        state: std::ptr::null_mut(),
        tics: 0,
        sx: 0,
        sy: 0,
    }; NUMPSPRITES],
    didsecret: 0,
}; MAXPLAYERS];

pub static mut consoleplayer: i32 = 0;
pub static mut displayplayer: i32 = 0;
pub static mut gametic: i32 = 0;
pub static mut levelstarttic: i32 = 0;
pub static mut totalkills: i32 = 0;
pub static mut totalitems: i32 = 0;
pub static mut totalsecret: i32 = 0;

pub static mut demoname: [u8; 32] = [0; 32];
pub static mut demorecording: boolean = 0;
pub static mut demoplayback: boolean = 0;
pub static mut netdemo: boolean = 0;
pub static mut demobuffer: *mut byte = std::ptr::null_mut();
pub static mut demo_p: *mut byte = std::ptr::null_mut();
pub static mut demoend: *mut byte = std::ptr::null_mut();
pub static mut singledemo: boolean = 0;

pub static mut precache: boolean = true_;

pub static mut wminfo: wbstartstruct_t = wbstartstruct_t {
    epsd: 0,
    didsecret: 0,
    last: 0,
    next: 0,
    maxkills: 0,
    maxitems: 0,
    maxsecret: 0,
    maxfrags: 0,
    partime: 0,
    pnum: 0,
    plyr: [wbplayerstruct_t {
        in_: 0,
        skills: 0,
        sitems: 0,
        ssecret: 0,
        stime: 0,
        frags: [0; 4],
        score: 0,
    }; MAXPLAYERS],
};

pub static mut consistancy: [[i16; BACKUPTICS]; MAXPLAYERS] = [[0; BACKUPTICS]; MAXPLAYERS];
pub static mut savebuffer: *mut byte = std::ptr::null_mut();

pub static mut key_right: i32 = 0;
pub static mut key_left: i32 = 0;
pub static mut key_up: i32 = 0;
pub static mut key_down: i32 = 0;
pub static mut key_strafeleft: i32 = 0;
pub static mut key_straferight: i32 = 0;
pub static mut key_fire: i32 = 0;
pub static mut key_use: i32 = 0;
pub static mut key_strafe: i32 = 0;
pub static mut key_speed: i32 = 0;

pub static mut mousebfire: i32 = 0;
pub static mut mousebstrafe: i32 = 0;
pub static mut mousebforward: i32 = 0;

pub static mut joybfire: i32 = 0;
pub static mut joybstrafe: i32 = 0;
pub static mut joybuse: i32 = 0;
pub static mut joybspeed: i32 = 0;

pub const TURBOTHRESHOLD: i32 = 0x32;

pub static mut forwardmove: [fixed_t; 2] = [0x19, 0x32];
pub static mut sidemove: [fixed_t; 2] = [0x18, 0x28];
pub static mut angleturn: [fixed_t; 3] = [640, 1280, 320];

pub const SLOWTURNTICS: i32 = 6;
pub const NUMKEYS: usize = 256;

pub static mut gamekeydown: [boolean; NUMKEYS] = [0; NUMKEYS];
pub static mut turnheld: i32 = 0;

pub static mut mousearray: [boolean; 4] = [0; 4];
pub static mut mousex: i32 = 0;
pub static mut mousey: i32 = 0;

pub static mut dclicktime: i32 = 0;
pub static mut dclickstate: i32 = 0;
pub static mut dclicks: i32 = 0;
pub static mut dclicktime2: i32 = 0;
pub static mut dclickstate2: i32 = 0;
pub static mut dclicks2: i32 = 0;

pub static mut joyxmove: i32 = 0;
pub static mut joyymove: i32 = 0;
pub static mut joyarray: [boolean; 5] = [0; 5];

pub static mut savegameslot: i32 = 0;
pub static mut savedescription: [u8; 32] = [0; 32];

pub const BODYQUESIZE: usize = 32;
pub static mut bodyque: [*mut mobj_t; BODYQUESIZE] = [std::ptr::null_mut(); BODYQUESIZE];
pub static mut bodyqueslot: i32 = 0;

pub static mut statcopy: *mut c_void = std::ptr::null_mut();

#[inline]
unsafe fn mousebuttons_get(idx: i32) -> boolean {
    mousearray[(idx + 1) as usize]
}

#[inline]
unsafe fn mousebuttons_set(idx: i32, val: boolean) {
    mousearray[(idx + 1) as usize] = val;
}

#[inline]
unsafe fn joybuttons_get(idx: i32) -> boolean {
    joyarray[(idx + 1) as usize]
}

#[inline]
unsafe fn joybuttons_set(idx: i32, val: boolean) {
    joyarray[(idx + 1) as usize] = val;
}

pub unsafe fn G_CmdChecksum(cmd: *mut ticcmd_t) -> i32 {
    let mut sum = 0i32;
    let len = std::mem::size_of::<ticcmd_t>() / 4 - 1;
    let p = cmd as *const i32;
    for i in 0..len {
        sum = sum.wrapping_add(*p.add(i));
    }
    sum
}

pub unsafe fn G_BuildTiccmd(cmd: *mut ticcmd_t) {
    let base = I_BaseTiccmd();
    *cmd = *base;

    (*cmd).consistancy = consistancy[consoleplayer as usize][(maketic as usize) % BACKUPTICS];

    let strafe = gamekeydown[key_strafe as usize] != 0
        || mousebuttons_get(mousebstrafe) != 0
        || joybuttons_get(joybstrafe) != 0;
    let speed = ((gamekeydown[key_speed as usize] != 0) || (joybuttons_get(joybspeed) != 0))
        as usize;

    let mut forward = 0i32;
    let mut side = 0i32;

    if joyxmove < 0
        || joyxmove > 0
        || gamekeydown[key_right as usize] != 0
        || gamekeydown[key_left as usize] != 0
    {
        turnheld += ticdup;
    } else {
        turnheld = 0;
    }

    let tspeed = if turnheld < SLOWTURNTICS { 2 } else { speed };

    if strafe {
        if gamekeydown[key_right as usize] != 0 {
            side += sidemove[speed];
        }
        if gamekeydown[key_left as usize] != 0 {
            side -= sidemove[speed];
        }
        if joyxmove > 0 {
            side += sidemove[speed];
        }
        if joyxmove < 0 {
            side -= sidemove[speed];
        }
    } else {
        if gamekeydown[key_right as usize] != 0 {
            (*cmd).angleturn = (*cmd).angleturn.wrapping_sub(angleturn[tspeed] as i16);
        }
        if gamekeydown[key_left as usize] != 0 {
            (*cmd).angleturn = (*cmd).angleturn.wrapping_add(angleturn[tspeed] as i16);
        }
        if joyxmove > 0 {
            (*cmd).angleturn = (*cmd).angleturn.wrapping_sub(angleturn[tspeed] as i16);
        }
        if joyxmove < 0 {
            (*cmd).angleturn = (*cmd).angleturn.wrapping_add(angleturn[tspeed] as i16);
        }
    }

    if gamekeydown[key_up as usize] != 0 {
        forward += forwardmove[speed];
    }
    if gamekeydown[key_down as usize] != 0 {
        forward -= forwardmove[speed];
    }
    if joyymove < 0 {
        forward += forwardmove[speed];
    }
    if joyymove > 0 {
        forward -= forwardmove[speed];
    }
    if gamekeydown[key_straferight as usize] != 0 {
        side += sidemove[speed];
    }
    if gamekeydown[key_strafeleft as usize] != 0 {
        side -= sidemove[speed];
    }

    (*cmd).chatchar = HU_dequeueChatChar();

    if gamekeydown[key_fire as usize] != 0
        || mousebuttons_get(mousebfire) != 0
        || joybuttons_get(joybfire) != 0
    {
        (*cmd).buttons |= BT_ATTACK;
    }

    if gamekeydown[key_use as usize] != 0 || joybuttons_get(joybuse) != 0 {
        (*cmd).buttons |= BT_USE;
        dclicks = 0;
    }

    for i in 0..(NUMWEAPONS - 1) {
        if gamekeydown[(b'1' as usize) + i] != 0 {
            (*cmd).buttons |= BT_CHANGE;
            (*cmd).buttons |= (i as u8) << BT_WEAPONSHIFT;
            break;
        }
    }

    if mousebuttons_get(mousebforward) != 0 {
        forward += forwardmove[speed];
    }

    if mousebuttons_get(mousebforward) != dclickstate && dclicktime > 1 {
        dclickstate = mousebuttons_get(mousebforward);
        if dclickstate != 0 {
            dclicks += 1;
        }
        if dclicks == 2 {
            (*cmd).buttons |= BT_USE;
            dclicks = 0;
        } else {
            dclicktime = 0;
        }
    } else {
        dclicktime += ticdup;
        if dclicktime > 20 {
            dclicks = 0;
            dclickstate = 0;
        }
    }

    let bstrafe =
        ((mousebuttons_get(mousebstrafe) != 0) || (joybuttons_get(joybstrafe) != 0)) as boolean;
    if bstrafe != dclickstate2 && dclicktime2 > 1 {
        dclickstate2 = bstrafe;
        if dclickstate2 != 0 {
            dclicks2 += 1;
        }
        if dclicks2 == 2 {
            (*cmd).buttons |= BT_USE;
            dclicks2 = 0;
        } else {
            dclicktime2 = 0;
        }
    } else {
        dclicktime2 += ticdup;
        if dclicktime2 > 20 {
            dclicks2 = 0;
            dclickstate2 = 0;
        }
    }

    forward += mousey;
    if strafe {
        side += mousex * 2;
    } else {
        (*cmd).angleturn = (*cmd).angleturn.wrapping_sub((mousex * 0x8) as i16);
    }

    mousex = 0;
    mousey = 0;

    let maxplmove = forwardmove[1];
    if forward > maxplmove {
        forward = maxplmove;
    } else if forward < -maxplmove {
        forward = -maxplmove;
    }
    if side > maxplmove {
        side = maxplmove;
    } else if side < -maxplmove {
        side = -maxplmove;
    }

    (*cmd).forwardmove = (*cmd).forwardmove.wrapping_add(forward as i8);
    (*cmd).sidemove = (*cmd).sidemove.wrapping_add(side as i8);

    if sendpause != 0 {
        sendpause = false_;
        (*cmd).buttons = BT_SPECIAL | BTS_PAUSE;
    }

    if sendsave != 0 {
        sendsave = false_;
        (*cmd).buttons = BT_SPECIAL | BTS_SAVEGAME | ((savegameslot as u8) << BTS_SAVESHIFT);
    }
}

pub unsafe fn G_DoLoadLevel() {
    skyflatnum = R_FlatNumForNameStr(SKYFLATNAME);

    if gamemode == commercial || gamemode == pack_tnt || gamemode == pack_plut {
        skytexture = R_TextureNumForNameStr("SKY3");
        if gamemap < 12 {
            skytexture = R_TextureNumForNameStr("SKY1");
        } else if gamemap < 21 {
            skytexture = R_TextureNumForNameStr("SKY2");
        }
    }

    levelstarttic = gametic;

    if wipegamestate == GS_LEVEL {
        wipegamestate = -1;
    }

    gamestate = GS_LEVEL;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 && players[i].playerstate == PST_DEAD {
            players[i].playerstate = PST_REBORN;
        }
        players[i].frags = [0; MAXPLAYERS];
    }

    P_SetupLevel(gameepisode, gamemap, 0, gameskill);
    displayplayer = consoleplayer;
    starttime = I_GetTime();
    gameaction = ga_nothing;
    Z_CheckHeap();

    gamekeydown = [0; NUMKEYS];
    joyxmove = 0;
    joyymove = 0;
    mousex = 0;
    mousey = 0;
    sendpause = false_;
    sendsave = false_;
    paused = false_;
    mousebuttons_set(0, 0);
    mousebuttons_set(1, 0);
    joybuttons_set(0, 0);
    joybuttons_set(1, 0);
}

pub unsafe fn G_Responder(ev: *mut event_t) -> boolean {
    if gamestate == GS_LEVEL
        && (*ev).type_ == ev_keydown
        && (*ev).data1 == KEY_F12
        && (singledemo != 0 || deathmatch == 0)
    {
        loop {
            displayplayer += 1;
            if displayplayer == (MAXPLAYERS as i32) {
                displayplayer = 0;
            }
            if playeringame[displayplayer as usize] != 0 || displayplayer == consoleplayer {
                break;
            }
        }
        return true_;
    }

    if gameaction == ga_nothing
        && singledemo == 0
        && (demoplayback != 0 || gamestate == GS_DEMOSCREEN)
    {
        if (*ev).type_ == ev_keydown
            || ((*ev).type_ == ev_mouse && (*ev).data1 != 0)
            || ((*ev).type_ == ev_joystick && (*ev).data1 != 0)
        {
            M_StartControlPanel();
            return true_;
        }
        return false_;
    }

    if gamestate == GS_LEVEL {
        if HU_Responder(ev) != 0 {
            return true_;
        }
        if ST_Responder(ev) != 0 {
            return true_;
        }
        if AM_Responder(ev) != 0 {
            return true_;
        }
    }

    if gamestate == GS_FINALE {
        if F_Responder(ev) != 0 {
            return true_;
        }
    }

    match (*ev).type_ {
        ev_keydown => {
            if (*ev).data1 == KEY_PAUSE {
                sendpause = true_;
                return true_;
            }
            if ((*ev).data1 as usize) < NUMKEYS {
                gamekeydown[(*ev).data1 as usize] = true_;
            }
            return true_;
        }
        ev_keyup => {
            if ((*ev).data1 as usize) < NUMKEYS {
                gamekeydown[(*ev).data1 as usize] = false_;
            }
            return false_;
        }
        ev_mouse => {
            mousebuttons_set(0, (*ev).data1 & 1);
            mousebuttons_set(1, (*ev).data1 & 2);
            mousebuttons_set(2, (*ev).data1 & 4);
            mousex = (*ev).data2 * (mouseSensitivity + 5) / 10;
            mousey = (*ev).data3 * (mouseSensitivity + 5) / 10;
            return true_;
        }
        ev_joystick => {
            joybuttons_set(0, (*ev).data1 & 1);
            joybuttons_set(1, (*ev).data1 & 2);
            joybuttons_set(2, (*ev).data1 & 4);
            joybuttons_set(3, (*ev).data1 & 8);
            joyxmove = (*ev).data2;
            joyymove = (*ev).data3;
            return true_;
        }
        _ => {}
    }

    false_
}

static mut turbomessage: [u8; 80] = [0; 80];

pub unsafe fn G_Ticker() {
    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 && players[i].playerstate == PST_REBORN {
            G_DoReborn(i as i32);
        }
    }

    while gameaction != ga_nothing {
        match gameaction {
            ga_loadlevel => G_DoLoadLevel(),
            ga_newgame => G_DoNewGame(),
            ga_loadgame => G_DoLoadGame(),
            ga_savegame => G_DoSaveGame(),
            ga_playdemo => G_DoPlayDemo(),
            ga_completed => G_DoCompleted(),
            ga_victory => F_StartFinale(),
            ga_worlddone => G_DoWorldDone(),
            ga_screenshot => {
                M_ScreenShot();
                gameaction = ga_nothing;
            }
            _ => {}
        }
    }

    let buf = ((gametic / ticdup) as usize) % BACKUPTICS;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            let cmd = &mut players[i].cmd as *mut ticcmd_t;
            *cmd = netcmds[i][buf];

            if demoplayback != 0 {
                G_ReadDemoTiccmd(cmd);
            }
            if demorecording != 0 {
                G_WriteDemoTiccmd(cmd);
            }

            if ((*cmd).forwardmove as i32) > TURBOTHRESHOLD
                && (gametic & 31) == 0
                && ((gametic >> 5) & 3) == (i as i32)
            {
                let s = format!("{} is turbo!", c_str(player_names[i]));
                write_c_str(turbomessage.as_mut_ptr(), &s);
                players[consoleplayer as usize].message = turbomessage.as_ptr();
            }

            if netgame != 0 && netdemo == 0 && (gametic % ticdup) == 0 {
                if gametic > (BACKUPTICS as i32) && consistancy[i][buf] != (*cmd).consistancy {
                    I_Error(&format!(
                        "consistency failure ({} should be {})",
                        (*cmd).consistancy, consistancy[i][buf]
                    ));
                }
                if !players[i].mo.is_null() {
                    consistancy[i][buf] = (*players[i].mo).x as i16;
                } else {
                    consistancy[i][buf] = rndindex as i16;
                }
            }
        }
    }

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            if players[i].cmd.buttons & BT_SPECIAL != 0 {
                match players[i].cmd.buttons & BT_SPECIALMASK {
                    BTS_PAUSE => {
                        paused ^= 1;
                        if paused != 0 {
                            S_PauseSound();
                        } else {
                            S_ResumeSound();
                        }
                    }
                    BTS_SAVEGAME => {
                        if savedescription[0] == 0 {
                            write_c_str(savedescription.as_mut_ptr(), "NET GAME");
                        }
                        savegameslot =
                            ((players[i].cmd.buttons & BTS_SAVEMASK) >> BTS_SAVESHIFT) as i32;
                        gameaction = ga_savegame;
                    }
                    _ => {}
                }
            }
        }
    }

    match gamestate {
        GS_LEVEL => {
            P_Ticker();
            ST_Ticker();
            AM_Ticker();
            HU_Ticker();
        }
        GS_INTERMISSION => WI_Ticker(),
        GS_FINALE => F_Ticker(),
        GS_DEMOSCREEN => D_PageTicker(),
        _ => {}
    }
}

pub unsafe fn G_InitPlayer(player: i32) {
    G_PlayerReborn(player);
}

pub unsafe fn G_PlayerFinishLevel(player: i32) {
    let p = &mut players[player as usize];
    p.powers = [0; NUMPOWERS];
    p.cards = [0; NUMCARDS];
    (*p.mo).flags &= !MF_SHADOW;
    p.extralight = 0;
    p.fixedcolormap = 0;
    p.damagecount = 0;
    p.bonuscount = 0;
}

pub unsafe fn G_PlayerReborn(player: i32) {
    let pidx = player as usize;
    let frags = players[pidx].frags;
    let killcount = players[pidx].killcount;
    let itemcount = players[pidx].itemcount;
    let secretcount = players[pidx].secretcount;

    let p = &mut players[pidx];
    *p = player_t::default();

    p.frags = frags;
    p.killcount = killcount;
    p.itemcount = itemcount;
    p.secretcount = secretcount;

    p.usedown = true_;
    p.attackdown = true_;
    p.playerstate = PST_LIVE;
    p.health = MAXHEALTH;
    p.readyweapon = wp_pistol;
    p.pendingweapon = wp_pistol;
    p.weaponowned[wp_fist as usize] = true_;
    p.weaponowned[wp_pistol as usize] = true_;
    p.ammo[am_clip as usize] = 50;

    for i in 0..NUMAMMO {
        p.maxammo[i] = maxammo[i];
    }
}

pub unsafe fn G_CheckSpot(playernum: i32, mthing: *mut mapthing_t) -> boolean {
    let pidx = playernum as usize;
    if players[pidx].mo.is_null() {
        for i in 0..pidx {
            if (*players[i].mo).x == (((*mthing).x as fixed_t) << FRACBITS)
                && (*players[i].mo).y == (((*mthing).y as fixed_t) << FRACBITS)
            {
                return false_;
            }
        }
        return true_;
    }

    let x = ((*mthing).x as fixed_t) << FRACBITS;
    let y = ((*mthing).y as fixed_t) << FRACBITS;

    if P_CheckPosition(players[pidx].mo, x, y) == 0 {
        return false_;
    }

    if (bodyqueslot as usize) >= BODYQUESIZE {
        P_RemoveMobj(bodyque[(bodyqueslot as usize) % BODYQUESIZE]);
    }
    bodyque[(bodyqueslot as usize) % BODYQUESIZE] = players[pidx].mo;
    bodyqueslot += 1;

    let ss = R_PointInSubsector(x, y);
    let an = (ANG45.wrapping_mul(((*mthing).angle / 45) as u32) >> ANGLETOFINESHIFT) as usize;

    let mo = P_SpawnMobj(
        x + 20 * finecosine(an),
        y + 20 * finesine[an],
        (*(*ss).sector).floorheight,
        MT_TFOG,
    );

    if players[consoleplayer as usize].viewz != 1 {
        S_StartSound(mo as *mut c_void, sfx_telept);
    }

    true_
}

pub unsafe fn G_DeathMatchSpawnPlayer(playernum: i32) {
    let selections = ((deathmatch_p as usize - deathmatchstarts.as_ptr() as usize)
        / std::mem::size_of::<mapthing_t>()) as i32;
    if selections < 4 {
        I_Error(&format!(
            "Only {} deathmatch spots, 4 required",
            selections
        ));
    }

    for _ in 0..20 {
        let i = (P_Random() % selections) as usize;
        if G_CheckSpot(playernum, &mut deathmatchstarts[i]) != 0 {
            deathmatchstarts[i].type_ = (playernum + 1) as i16;
            P_SpawnPlayer(&mut deathmatchstarts[i]);
            return;
        }
    }

    P_SpawnPlayer(&mut playerstarts[playernum as usize]);
}

pub unsafe fn G_DoReborn(playernum: i32) {
    if netgame == 0 {
        gameaction = ga_loadlevel;
    } else {
        let pidx = playernum as usize;
        (*players[pidx].mo).player = std::ptr::null_mut();

        if deathmatch != 0 {
            G_DeathMatchSpawnPlayer(playernum);
            return;
        }

        if G_CheckSpot(playernum, &mut playerstarts[pidx]) != 0 {
            P_SpawnPlayer(&mut playerstarts[pidx]);
            return;
        }

        for i in 0..MAXPLAYERS {
            if G_CheckSpot(playernum, &mut playerstarts[i]) != 0 {
                playerstarts[i].type_ = (playernum + 1) as i16;
                P_SpawnPlayer(&mut playerstarts[i]);
                playerstarts[i].type_ = (i + 1) as i16;
                return;
            }
        }
        P_SpawnPlayer(&mut playerstarts[pidx]);
    }
}

pub unsafe fn G_ScreenShot() {
    gameaction = ga_screenshot;
}

pub static mut pars: [[i32; 10]; 4] = [
    [0; 10],
    [0, 30, 75, 120, 90, 165, 180, 180, 30, 165],
    [0, 90, 90, 90, 120, 90, 360, 240, 30, 170],
    [0, 90, 45, 90, 150, 90, 90, 165, 30, 135],
];

pub static mut cpars: [i32; 32] = [
    30, 90, 120, 120, 90, 150, 120, 120, 270, 90, 210, 150, 150, 150, 210, 150, 420, 150, 210, 150,
    240, 150, 180, 150, 150, 300, 330, 420, 300, 180, 120, 30,
];

pub static mut secretexit: boolean = 0;

pub unsafe fn G_ExitLevel() {
    secretexit = false_;
    gameaction = ga_completed;
}

pub unsafe fn G_SecretExitLevel() {
    if gamemode == commercial && W_CheckNumForNameStr("map31") < 0 {
        secretexit = false_;
    } else {
        secretexit = true_;
    }
    gameaction = ga_completed;
}

pub unsafe fn G_DoCompleted() {
    gameaction = ga_nothing;

    for i in 0..MAXPLAYERS {
        if playeringame[i] != 0 {
            G_PlayerFinishLevel(i as i32);
        }
    }

    if automapactive != 0 {
        AM_Stop();
    }

    if gamemode != commercial {
        match gamemap {
            8 => {
                gameaction = ga_victory;
                return;
            }
            9 => {
                for i in 0..MAXPLAYERS {
                    players[i].didsecret = true_;
                }
            }
            _ => {}
        }
    }

    if gamemap == 8 && gamemode != commercial {
        gameaction = ga_victory;
        return;
    }

    if gamemap == 9 && gamemode != commercial {
        for i in 0..MAXPLAYERS {
            players[i].didsecret = true_;
        }
    }

    wminfo.didsecret = players[consoleplayer as usize].didsecret;
    wminfo.epsd = gameepisode - 1;
    wminfo.last = gamemap - 1;

    if gamemode == commercial {
        if secretexit != 0 {
            match gamemap {
                15 => wminfo.next = 30,
                31 => wminfo.next = 31,
                _ => {}
            }
        } else {
            match gamemap {
                31 | 32 => wminfo.next = 15,
                _ => wminfo.next = gamemap,
            }
        }
    } else {
        if secretexit != 0 {
            wminfo.next = 8;
        } else if gamemap == 9 {
            match gameepisode {
                1 => wminfo.next = 3,
                2 => wminfo.next = 5,
                3 => wminfo.next = 6,
                4 => wminfo.next = 2,
                _ => {}
            }
        } else {
            wminfo.next = gamemap;
        }
    }

    wminfo.maxkills = totalkills;
    wminfo.maxitems = totalitems;
    wminfo.maxsecret = totalsecret;
    wminfo.maxfrags = 0;

    if gamemode == commercial {
        wminfo.partime = 35 * cpars[(gamemap - 1) as usize];
    } else {
        wminfo.partime = 35 * pars[gameepisode as usize][gamemap as usize];
    }
    wminfo.pnum = consoleplayer;

    for i in 0..MAXPLAYERS {
        wminfo.plyr[i].in_ = playeringame[i];
        wminfo.plyr[i].skills = players[i].killcount;
        wminfo.plyr[i].sitems = players[i].itemcount;
        wminfo.plyr[i].ssecret = players[i].secretcount;
        wminfo.plyr[i].stime = leveltime;
        wminfo.plyr[i].frags = players[i].frags;
    }

    gamestate = GS_INTERMISSION;
    viewactive = false_;
    automapactive = false_;

    if !statcopy.is_null() {
        std::ptr::copy_nonoverlapping(&wminfo, statcopy as *mut wbstartstruct_t, 1);
    }

    WI_Start(&mut wminfo);
}

pub unsafe fn G_WorldDone() {
    gameaction = ga_worlddone;

    if secretexit != 0 {
        players[consoleplayer as usize].didsecret = true_;
    }

    if gamemode == commercial {
        match gamemap {
            15 | 31 => {
                if secretexit != 0 {
                    F_StartFinale();
                }
            }
            6 | 11 | 20 | 30 => {
                F_StartFinale();
            }
            _ => {}
        }
    }
}

pub unsafe fn G_DoWorldDone() {
    gamestate = GS_LEVEL;
    gamemap = wminfo.next + 1;
    G_DoLoadLevel();
    gameaction = ga_nothing;
    viewactive = true_;
}

pub static mut savename: [u8; 256] = [0; 256];

pub unsafe fn G_LoadGame(name: *const u8) {
    c_strcpy(savename.as_mut_ptr(), name);
    gameaction = ga_loadgame;
}

pub const VERSIONSIZE: usize = 16;

pub unsafe fn G_DoLoadGame() {
    gameaction = ga_nothing;

    let _length = M_ReadFile(savename.as_ptr(), &mut savebuffer);
    save_p = savebuffer.add(SAVESTRINGSIZE);

    let mut vcheck = [0u8; VERSIONSIZE];
    let vs = format!("version {}", VERSION);
    write_c_str(vcheck.as_mut_ptr(), &vs);
    if c_strcmp(save_p, vcheck.as_ptr()) != 0 {
        return;
    }
    save_p = save_p.add(VERSIONSIZE);

    gameskill = *save_p as skill_t;
    save_p = save_p.add(1);
    gameepisode = *save_p as i32;
    save_p = save_p.add(1);
    gamemap = *save_p as i32;
    save_p = save_p.add(1);
    for i in 0..MAXPLAYERS {
        playeringame[i] = *save_p as boolean;
        save_p = save_p.add(1);
    }

    G_InitNew(gameskill, gameepisode, gamemap);

    let a = *save_p as i32;
    save_p = save_p.add(1);
    let b = *save_p as i32;
    save_p = save_p.add(1);
    let c = *save_p as i32;
    save_p = save_p.add(1);
    leveltime = (a << 16) + (b << 8) + c;

    P_UnArchivePlayers();
    P_UnArchiveWorld();
    P_UnArchiveThinkers();
    P_UnArchiveSpecials();

    if *save_p != 0x1d {
        I_Error("Bad savegame");
    }

    Z_Free(savebuffer as *mut c_void);

    if setsizeneeded != 0 {
        R_ExecuteSetViewSize();
    }

    R_FillBackScreen();
}

pub unsafe fn G_SaveGame(slot: i32, description: *const u8) {
    savegameslot = slot;
    c_strcpy(savedescription.as_mut_ptr(), description);
    sendsave = true_;
}

pub unsafe fn G_DoSaveGame() {
    let mut name = [0u8; 100];
    let s = if M_CheckParmStr("-cdrom") != 0 {
        format!("c:\\doomdata\\{}{}.dsg", SAVEGAMENAME, savegameslot)
    } else {
        format!("{}{}.dsg", SAVEGAMENAME, savegameslot)
    };
    write_c_str(name.as_mut_ptr(), &s);

    savebuffer = screens[1].add(0x4000);
    save_p = savebuffer;

    std::ptr::copy_nonoverlapping(savedescription.as_ptr(), save_p, SAVESTRINGSIZE);
    save_p = save_p.add(SAVESTRINGSIZE);

    let mut name2 = [0u8; VERSIONSIZE];
    let vs = format!("version {}", VERSION);
    write_c_str(name2.as_mut_ptr(), &vs);
    std::ptr::copy_nonoverlapping(name2.as_ptr(), save_p, VERSIONSIZE);
    save_p = save_p.add(VERSIONSIZE);

    *save_p = gameskill as byte;
    save_p = save_p.add(1);
    *save_p = gameepisode as byte;
    save_p = save_p.add(1);
    *save_p = gamemap as byte;
    save_p = save_p.add(1);
    for i in 0..MAXPLAYERS {
        *save_p = playeringame[i] as byte;
        save_p = save_p.add(1);
    }
    *save_p = (leveltime >> 16) as byte;
    save_p = save_p.add(1);
    *save_p = (leveltime >> 8) as byte;
    save_p = save_p.add(1);
    *save_p = leveltime as byte;
    save_p = save_p.add(1);

    P_ArchivePlayers();
    P_ArchiveWorld();
    P_ArchiveThinkers();
    P_ArchiveSpecials();

    *save_p = 0x1d;
    save_p = save_p.add(1);

    let length = (save_p as usize - savebuffer as usize) as i32;
    if (length as usize) > SAVEGAMESIZE {
        I_Error("Savegame buffer overrun");
    }
    M_WriteFile(name.as_ptr(), savebuffer as *const c_void, length);
    gameaction = ga_nothing;
    savedescription[0] = 0;

    players[consoleplayer as usize].message = b"game saved.\0".as_ptr();

    R_FillBackScreen();
}

pub static mut d_skill: skill_t = 0;
pub static mut d_episode: i32 = 0;
pub static mut d_map: i32 = 0;

pub unsafe fn G_DeferedInitNew(skill: skill_t, episode: i32, map: i32) {
    d_skill = skill;
    d_episode = episode;
    d_map = map;
    gameaction = ga_newgame;
}

pub unsafe fn G_DoNewGame() {
    demoplayback = false_;
    netdemo = false_;
    netgame = false_;
    deathmatch = false_;
    playeringame[1] = 0;
    playeringame[2] = 0;
    playeringame[3] = 0;
    respawnparm = false_;
    fastparm = false_;
    nomonsters = false_;
    consoleplayer = 0;
    G_InitNew(d_skill, d_episode, d_map);
    gameaction = ga_nothing;
}

pub unsafe fn G_InitNew(mut skill: skill_t, mut episode: i32, mut map: i32) {
    if paused != 0 {
        paused = false_;
        S_ResumeSound();
    }

    if skill > sk_nightmare {
        skill = sk_nightmare;
    }

    if episode < 1 {
        episode = 1;
    }

    if gamemode == retail {
        if episode > 4 {
            episode = 4;
        }
    } else if gamemode == shareware {
        if episode > 1 {
            episode = 1;
        }
    } else {
        if episode > 3 {
            episode = 3;
        }
    }

    if map < 1 {
        map = 1;
    }

    if map > 9 && gamemode != commercial {
        map = 9;
    }

    M_ClearRandom();

    if skill == sk_nightmare || respawnparm != 0 {
        respawnmonsters = true_;
    } else {
        respawnmonsters = false_;
    }

    if fastparm != 0 || (skill == sk_nightmare && gameskill != sk_nightmare) {
        for i in (S_SARG_RUN1 as usize)..=(S_SARG_PAIN2 as usize) {
            states[i].tics >>= 1;
        }
        mobjinfo[MT_BRUISERSHOT as usize].speed = 20 * FRACUNIT;
        mobjinfo[MT_HEADSHOT as usize].speed = 20 * FRACUNIT;
        mobjinfo[MT_TROOPSHOT as usize].speed = 20 * FRACUNIT;
    } else if skill != sk_nightmare && gameskill == sk_nightmare {
        for i in (S_SARG_RUN1 as usize)..=(S_SARG_PAIN2 as usize) {
            states[i].tics <<= 1;
        }
        mobjinfo[MT_BRUISERSHOT as usize].speed = 15 * FRACUNIT;
        mobjinfo[MT_HEADSHOT as usize].speed = 10 * FRACUNIT;
        mobjinfo[MT_TROOPSHOT as usize].speed = 10 * FRACUNIT;
    }

    for i in 0..MAXPLAYERS {
        players[i].playerstate = PST_REBORN;
    }

    usergame = true_;
    paused = false_;
    demoplayback = false_;
    automapactive = false_;
    viewactive = true_;
    gameepisode = episode;
    gamemap = map;
    gameskill = skill;

    viewactive = true_;

    if gamemode == commercial {
        skytexture = R_TextureNumForNameStr("SKY3");
        if gamemap < 12 {
            skytexture = R_TextureNumForNameStr("SKY1");
        } else if gamemap < 21 {
            skytexture = R_TextureNumForNameStr("SKY2");
        }
    } else {
        match episode {
            1 => skytexture = R_TextureNumForNameStr("SKY1"),
            2 => skytexture = R_TextureNumForNameStr("SKY2"),
            3 => skytexture = R_TextureNumForNameStr("SKY3"),
            4 => skytexture = R_TextureNumForNameStr("SKY4"),
            _ => {}
        }
    }

    G_DoLoadLevel();
}

pub const DEMOMARKER: byte = 0x80;

pub unsafe fn G_ReadDemoTiccmd(cmd: *mut ticcmd_t) {
    if *demo_p == DEMOMARKER {
        G_CheckDemoStatus();
        return;
    }
    (*cmd).forwardmove = *demo_p as i8;
    demo_p = demo_p.add(1);
    (*cmd).sidemove = *demo_p as i8;
    demo_p = demo_p.add(1);
    (*cmd).angleturn = ((*demo_p as i16) << 8) as i16;
    demo_p = demo_p.add(1);
    (*cmd).buttons = *demo_p;
    demo_p = demo_p.add(1);
}

pub unsafe fn G_WriteDemoTiccmd(cmd: *mut ticcmd_t) {
    if gamekeydown[b'q' as usize] != 0 {
        G_CheckDemoStatus();
    }
    *demo_p = (*cmd).forwardmove as byte;
    demo_p = demo_p.add(1);
    *demo_p = (*cmd).sidemove as byte;
    demo_p = demo_p.add(1);
    *demo_p = (((*cmd).angleturn + 128) >> 8) as byte;
    demo_p = demo_p.add(1);
    *demo_p = (*cmd).buttons;
    demo_p = demo_p.add(1);
    demo_p = demo_p.sub(4);
    if (demo_p as usize) > (demoend.sub(16) as usize) {
        G_CheckDemoStatus();
        return;
    }

    G_ReadDemoTiccmd(cmd);
}

pub unsafe fn G_RecordDemo(name: *const u8) {
    usergame = false_;
    c_strcpy(demoname.as_mut_ptr(), name);
    c_strcat(demoname.as_mut_ptr(), b".lmp\0".as_ptr());
    let mut maxsize = 0x20000;
    let i = M_CheckParmStr("-maxdemo");
    if i != 0 && i < myargc - 1 {
        maxsize = c_atoi(*myargv.add((i + 1) as usize)) * 1024;
    }
    demobuffer = Z_Malloc(maxsize, PU_STATIC, std::ptr::null_mut()) as *mut byte;
    demoend = demobuffer.add(maxsize as usize);

    demorecording = true_;
}

pub unsafe fn G_BeginRecording() {
    demo_p = demobuffer;

    *demo_p = VERSION as byte;
    demo_p = demo_p.add(1);
    *demo_p = gameskill as byte;
    demo_p = demo_p.add(1);
    *demo_p = gameepisode as byte;
    demo_p = demo_p.add(1);
    *demo_p = gamemap as byte;
    demo_p = demo_p.add(1);
    *demo_p = deathmatch as byte;
    demo_p = demo_p.add(1);
    *demo_p = respawnparm as byte;
    demo_p = demo_p.add(1);
    *demo_p = fastparm as byte;
    demo_p = demo_p.add(1);
    *demo_p = nomonsters as byte;
    demo_p = demo_p.add(1);
    *demo_p = consoleplayer as byte;
    demo_p = demo_p.add(1);

    for i in 0..MAXPLAYERS {
        *demo_p = playeringame[i] as byte;
        demo_p = demo_p.add(1);
    }
}

pub static mut defdemoname: *const u8 = std::ptr::null();

pub unsafe fn G_DeferedPlayDemo(name: *const u8) {
    defdemoname = name;
    gameaction = ga_playdemo;
}

pub unsafe fn G_DoPlayDemo() {
    gameaction = ga_nothing;
    demobuffer = W_CacheLumpName(defdemoname, PU_STATIC) as *mut byte;
    demo_p = demobuffer;

    let demov = *demo_p;
    demo_p = demo_p.add(1);
    if (demov as i32) != VERSION && demov != 109 {
        eprintln!("Demo is from a different game version!");
        gameaction = ga_nothing;
        return;
    }

    let skill = *demo_p as skill_t;
    demo_p = demo_p.add(1);
    let episode = *demo_p as i32;
    demo_p = demo_p.add(1);
    let map = *demo_p as i32;
    demo_p = demo_p.add(1);
    deathmatch = *demo_p as boolean;
    demo_p = demo_p.add(1);
    respawnparm = *demo_p as boolean;
    demo_p = demo_p.add(1);
    fastparm = *demo_p as boolean;
    demo_p = demo_p.add(1);
    nomonsters = *demo_p as boolean;
    demo_p = demo_p.add(1);
    consoleplayer = *demo_p as i32;
    demo_p = demo_p.add(1);

    for i in 0..MAXPLAYERS {
        playeringame[i] = *demo_p as boolean;
        demo_p = demo_p.add(1);
    }
    if playeringame[1] != 0 {
        netgame = true_;
        netdemo = true_;
    }

    precache = false_;
    G_InitNew(skill, episode, map);
    precache = true_;

    usergame = false_;
    demoplayback = true_;
}

pub unsafe fn G_TimeDemo(name: *const u8) {
    nodrawers = M_CheckParmStr("-nodraw");
    noblit = M_CheckParmStr("-noblit");
    timingdemo = true_;
    singletics = true_;

    defdemoname = name;
    gameaction = ga_playdemo;
}

pub unsafe fn G_CheckDemoStatus() -> boolean {
    if timingdemo != 0 {
        let endtime = I_GetTime();
        I_Error(&format!(
            "timed {} gametics in {} realtics",
            gametic,
            endtime - starttime
        ));
    }

    if demoplayback != 0 {
        if singledemo != 0 {
            I_Quit();
        }

        Z_ChangeTag(demobuffer as *mut c_void, PU_CACHE);
        demoplayback = false_;
        netdemo = false_;
        netgame = false_;
        deathmatch = false_;
        playeringame[1] = 0;
        playeringame[2] = 0;
        playeringame[3] = 0;
        respawnparm = false_;
        fastparm = false_;
        nomonsters = false_;
        consoleplayer = 0;
        D_AdvanceDemo();
        return true_;
    }

    if demorecording != 0 {
        *demo_p = DEMOMARKER;
        demo_p = demo_p.add(1);
        M_WriteFile(
            demoname.as_ptr(),
            demobuffer as *const c_void,
            (demo_p as usize - demobuffer as usize) as i32,
        );
        Z_Free(demobuffer as *mut c_void);
        demorecording = false_;
        I_Error(&format!("Demo {} recorded", c_str(demoname.as_ptr())));
    }

    false_
}
