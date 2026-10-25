// Based on id Software's linuxdoom-1.10 (GNU GPL)

pub use crate::am_map::automapactive;
pub use crate::d_main::{
    autostart, basedefault, debugfile, devparm, fastparm, nomonsters, respawnparm, singletics,
    startepisode, startmap, startskill,
};
pub use crate::d_net::{doomcom, localcmds, maketic, netbuffer, netcmds, nettics, ticdup};
pub use crate::doomdef::*;
pub use crate::doomtype::*;
pub use crate::g_game::{
    bodyqueslot, consoleplayer, deathmatch, demoplayback, demorecording, displayplayer,
    gameepisode, gamemap, gameskill, gamestate, gametic, levelstarttic, netgame, noblit,
    nodrawers, paused, playeringame, players, precache, respawnmonsters, singledemo, totalitems,
    totalkills, totalsecret, usergame, viewactive, wminfo,
};
pub use crate::m_menu::{menuactive, mouseSensitivity};
pub use crate::m_random::rndindex;
pub use crate::p_inter::maxammo;
pub use crate::p_setup::{deathmatch_p, deathmatchstarts, playerstarts};
pub use crate::p_tick::leveltime;
pub use crate::r_draw::{scaledviewwidth, viewheight, viewwidth, viewwindowx, viewwindowy};
pub use crate::r_main::viewangleoffset;
pub use crate::r_sky::skyflatnum;
pub use crate::s_sound::{snd_MusicVolume, snd_SfxVolume};

pub static mut gamemode: GameMode_t = indetermined;
pub static mut gamemission: GameMission_t = doom;
pub static mut language: Language_t = english;
pub static mut modifiedgame: boolean = false_;
pub static mut statusbaractive: boolean = false_;
pub static mut snd_MusicDevice: i32 = 0;
pub static mut snd_SfxDevice: i32 = 0;
pub static mut snd_DesiredMusicDevice: i32 = 0;
pub static mut snd_DesiredSfxDevice: i32 = 0;

pub const MAX_DM_STARTS: usize = 10;
