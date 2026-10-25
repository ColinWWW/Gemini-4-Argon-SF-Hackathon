// DOOM info LUT tables mechanically converted to Rust by harness/tables_to_rust.py
// Based on id Software's linuxdoom-1.10 info.c (GNU GPL)

use crate::d_think::actionf_t;
use crate::info::*;
use crate::p_mobj::*;
use crate::p_pspr::*;
use crate::sounds::*;
use crate::m_fixed::*;

pub static sprnames: [&'static str; 138] = [
    "TROO", "SHTG", "PUNG", "PISG", "PISF", "SHTF", "SHT2", "CHGG", "CHGF", "MISG",
    "MISF", "SAWG", "PLSG", "PLSF", "BFGG", "BFGF", "BLUD", "PUFF", "BAL1", "BAL2",
    "PLSS", "PLSE", "MISL", "BFS1", "BFE1", "BFE2", "TFOG", "IFOG", "PLAY", "POSS",
    "SPOS", "VILE", "FIRE", "FATB", "FBXP", "SKEL", "MANF", "FATT", "CPOS", "SARG",
    "HEAD", "BAL7", "BOSS", "BOS2", "SKUL", "SPID", "BSPI", "APLS", "APBX", "CYBR",
    "PAIN", "SSWV", "KEEN", "BBRN", "BOSF", "ARM1", "ARM2", "BAR1", "BEXP", "FCAN",
    "BON1", "BON2", "BKEY", "RKEY", "YKEY", "BSKU", "RSKU", "YSKU", "STIM", "MEDI",
    "SOUL", "PINV", "PSTR", "PINS", "MEGA", "SUIT", "PMAP", "PVIS", "CLIP", "AMMO",
    "ROCK", "BROK", "CELL", "CELP", "SHEL", "SBOX", "BPAK", "BFUG", "MGUN", "CSAW",
    "LAUN", "PLAS", "SHOT", "SGN2", "COLU", "SMT2", "GOR1", "POL2", "POL5", "POL4",
    "POL3", "POL1", "POL6", "GOR2", "GOR3", "GOR4", "GOR5", "SMIT", "COL1", "COL2",
    "COL3", "COL4", "CAND", "CBRA", "COL6", "TRE1", "TRE2", "ELEC", "CEYE", "FSKU",
    "COL5", "TBLU", "TGRN", "TRED", "SMBT", "SMGT", "SMRT", "HDB1", "HDB2", "HDB3",
    "HDB4", "HDB5", "HDB6", "POB1", "POB2", "BRS1", "TLMP", "TLP2",
];

pub static mut states: [state_t; 967] = [
    state_t { sprite: SPR_TROO, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_NULL
    state_t { sprite: SPR_SHTG, frame: 4, tics: 0, action: actionf_t::acp2(crate::p_pspr::A_Light0), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_LIGHTDONE
    state_t { sprite: SPR_PUNG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_PUNCH, misc1: 0, misc2: 0 }, // S_PUNCH
    state_t { sprite: SPR_PUNG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_PUNCHDOWN, misc1: 0, misc2: 0 }, // S_PUNCHDOWN
    state_t { sprite: SPR_PUNG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_PUNCHUP, misc1: 0, misc2: 0 }, // S_PUNCHUP
    state_t { sprite: SPR_PUNG, frame: 1, tics: 4, action: actionf_t::None, nextstate: S_PUNCH2, misc1: 0, misc2: 0 }, // S_PUNCH1
    state_t { sprite: SPR_PUNG, frame: 2, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Punch), nextstate: S_PUNCH3, misc1: 0, misc2: 0 }, // S_PUNCH2
    state_t { sprite: SPR_PUNG, frame: 3, tics: 5, action: actionf_t::None, nextstate: S_PUNCH4, misc1: 0, misc2: 0 }, // S_PUNCH3
    state_t { sprite: SPR_PUNG, frame: 2, tics: 4, action: actionf_t::None, nextstate: S_PUNCH5, misc1: 0, misc2: 0 }, // S_PUNCH4
    state_t { sprite: SPR_PUNG, frame: 1, tics: 5, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_PUNCH, misc1: 0, misc2: 0 }, // S_PUNCH5
    state_t { sprite: SPR_PISG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_PISTOL, misc1: 0, misc2: 0 }, // S_PISTOL
    state_t { sprite: SPR_PISG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_PISTOLDOWN, misc1: 0, misc2: 0 }, // S_PISTOLDOWN
    state_t { sprite: SPR_PISG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_PISTOLUP, misc1: 0, misc2: 0 }, // S_PISTOLUP
    state_t { sprite: SPR_PISG, frame: 0, tics: 4, action: actionf_t::None, nextstate: S_PISTOL2, misc1: 0, misc2: 0 }, // S_PISTOL1
    state_t { sprite: SPR_PISG, frame: 1, tics: 6, action: actionf_t::acp2(crate::p_pspr::A_FirePistol), nextstate: S_PISTOL3, misc1: 0, misc2: 0 }, // S_PISTOL2
    state_t { sprite: SPR_PISG, frame: 2, tics: 4, action: actionf_t::None, nextstate: S_PISTOL4, misc1: 0, misc2: 0 }, // S_PISTOL3
    state_t { sprite: SPR_PISG, frame: 1, tics: 5, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_PISTOL, misc1: 0, misc2: 0 }, // S_PISTOL4
    state_t { sprite: SPR_PISF, frame: 32768, tics: 7, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_PISTOLFLASH
    state_t { sprite: SPR_SHTG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_SGUN, misc1: 0, misc2: 0 }, // S_SGUN
    state_t { sprite: SPR_SHTG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_SGUNDOWN, misc1: 0, misc2: 0 }, // S_SGUNDOWN
    state_t { sprite: SPR_SHTG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_SGUNUP, misc1: 0, misc2: 0 }, // S_SGUNUP
    state_t { sprite: SPR_SHTG, frame: 0, tics: 3, action: actionf_t::None, nextstate: S_SGUN2, misc1: 0, misc2: 0 }, // S_SGUN1
    state_t { sprite: SPR_SHTG, frame: 0, tics: 7, action: actionf_t::acp2(crate::p_pspr::A_FireShotgun), nextstate: S_SGUN3, misc1: 0, misc2: 0 }, // S_SGUN2
    state_t { sprite: SPR_SHTG, frame: 1, tics: 5, action: actionf_t::None, nextstate: S_SGUN4, misc1: 0, misc2: 0 }, // S_SGUN3
    state_t { sprite: SPR_SHTG, frame: 2, tics: 5, action: actionf_t::None, nextstate: S_SGUN5, misc1: 0, misc2: 0 }, // S_SGUN4
    state_t { sprite: SPR_SHTG, frame: 3, tics: 4, action: actionf_t::None, nextstate: S_SGUN6, misc1: 0, misc2: 0 }, // S_SGUN5
    state_t { sprite: SPR_SHTG, frame: 2, tics: 5, action: actionf_t::None, nextstate: S_SGUN7, misc1: 0, misc2: 0 }, // S_SGUN6
    state_t { sprite: SPR_SHTG, frame: 1, tics: 5, action: actionf_t::None, nextstate: S_SGUN8, misc1: 0, misc2: 0 }, // S_SGUN7
    state_t { sprite: SPR_SHTG, frame: 0, tics: 3, action: actionf_t::None, nextstate: S_SGUN9, misc1: 0, misc2: 0 }, // S_SGUN8
    state_t { sprite: SPR_SHTG, frame: 0, tics: 7, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_SGUN, misc1: 0, misc2: 0 }, // S_SGUN9
    state_t { sprite: SPR_SHTF, frame: 32768, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_SGUNFLASH2, misc1: 0, misc2: 0 }, // S_SGUNFLASH1
    state_t { sprite: SPR_SHTF, frame: 32769, tics: 3, action: actionf_t::acp2(crate::p_pspr::A_Light2), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_SGUNFLASH2
    state_t { sprite: SPR_SHT2, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_DSGUN, misc1: 0, misc2: 0 }, // S_DSGUN
    state_t { sprite: SPR_SHT2, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_DSGUNDOWN, misc1: 0, misc2: 0 }, // S_DSGUNDOWN
    state_t { sprite: SPR_SHT2, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_DSGUNUP, misc1: 0, misc2: 0 }, // S_DSGUNUP
    state_t { sprite: SPR_SHT2, frame: 0, tics: 3, action: actionf_t::None, nextstate: S_DSGUN2, misc1: 0, misc2: 0 }, // S_DSGUN1
    state_t { sprite: SPR_SHT2, frame: 0, tics: 7, action: actionf_t::acp2(crate::p_pspr::A_FireShotgun2), nextstate: S_DSGUN3, misc1: 0, misc2: 0 }, // S_DSGUN2
    state_t { sprite: SPR_SHT2, frame: 1, tics: 7, action: actionf_t::None, nextstate: S_DSGUN4, misc1: 0, misc2: 0 }, // S_DSGUN3
    state_t { sprite: SPR_SHT2, frame: 2, tics: 7, action: actionf_t::acp2(crate::p_pspr::A_CheckReload), nextstate: S_DSGUN5, misc1: 0, misc2: 0 }, // S_DSGUN4
    state_t { sprite: SPR_SHT2, frame: 3, tics: 7, action: actionf_t::acp2(crate::p_enemy::A_OpenShotgun2), nextstate: S_DSGUN6, misc1: 0, misc2: 0 }, // S_DSGUN5
    state_t { sprite: SPR_SHT2, frame: 4, tics: 7, action: actionf_t::None, nextstate: S_DSGUN7, misc1: 0, misc2: 0 }, // S_DSGUN6
    state_t { sprite: SPR_SHT2, frame: 5, tics: 7, action: actionf_t::acp2(crate::p_enemy::A_LoadShotgun2), nextstate: S_DSGUN8, misc1: 0, misc2: 0 }, // S_DSGUN7
    state_t { sprite: SPR_SHT2, frame: 6, tics: 6, action: actionf_t::None, nextstate: S_DSGUN9, misc1: 0, misc2: 0 }, // S_DSGUN8
    state_t { sprite: SPR_SHT2, frame: 7, tics: 6, action: actionf_t::acp2(crate::p_enemy::A_CloseShotgun2), nextstate: S_DSGUN10, misc1: 0, misc2: 0 }, // S_DSGUN9
    state_t { sprite: SPR_SHT2, frame: 0, tics: 5, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_DSGUN, misc1: 0, misc2: 0 }, // S_DSGUN10
    state_t { sprite: SPR_SHT2, frame: 1, tics: 7, action: actionf_t::None, nextstate: S_DSNR2, misc1: 0, misc2: 0 }, // S_DSNR1
    state_t { sprite: SPR_SHT2, frame: 0, tics: 3, action: actionf_t::None, nextstate: S_DSGUNDOWN, misc1: 0, misc2: 0 }, // S_DSNR2
    state_t { sprite: SPR_SHT2, frame: 32776, tics: 5, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_DSGUNFLASH2, misc1: 0, misc2: 0 }, // S_DSGUNFLASH1
    state_t { sprite: SPR_SHT2, frame: 32777, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Light2), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_DSGUNFLASH2
    state_t { sprite: SPR_CHGG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_CHAIN, misc1: 0, misc2: 0 }, // S_CHAIN
    state_t { sprite: SPR_CHGG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_CHAINDOWN, misc1: 0, misc2: 0 }, // S_CHAINDOWN
    state_t { sprite: SPR_CHGG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_CHAINUP, misc1: 0, misc2: 0 }, // S_CHAINUP
    state_t { sprite: SPR_CHGG, frame: 0, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_FireCGun), nextstate: S_CHAIN2, misc1: 0, misc2: 0 }, // S_CHAIN1
    state_t { sprite: SPR_CHGG, frame: 1, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_FireCGun), nextstate: S_CHAIN3, misc1: 0, misc2: 0 }, // S_CHAIN2
    state_t { sprite: SPR_CHGG, frame: 1, tics: 0, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_CHAIN, misc1: 0, misc2: 0 }, // S_CHAIN3
    state_t { sprite: SPR_CHGF, frame: 32768, tics: 5, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_CHAINFLASH1
    state_t { sprite: SPR_CHGF, frame: 32769, tics: 5, action: actionf_t::acp2(crate::p_pspr::A_Light2), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_CHAINFLASH2
    state_t { sprite: SPR_MISG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_MISSILE, misc1: 0, misc2: 0 }, // S_MISSILE
    state_t { sprite: SPR_MISG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_MISSILEDOWN, misc1: 0, misc2: 0 }, // S_MISSILEDOWN
    state_t { sprite: SPR_MISG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_MISSILEUP, misc1: 0, misc2: 0 }, // S_MISSILEUP
    state_t { sprite: SPR_MISG, frame: 1, tics: 8, action: actionf_t::acp2(crate::p_pspr::A_GunFlash), nextstate: S_MISSILE2, misc1: 0, misc2: 0 }, // S_MISSILE1
    state_t { sprite: SPR_MISG, frame: 1, tics: 12, action: actionf_t::acp2(crate::p_pspr::A_FireMissile), nextstate: S_MISSILE3, misc1: 0, misc2: 0 }, // S_MISSILE2
    state_t { sprite: SPR_MISG, frame: 1, tics: 0, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_MISSILE, misc1: 0, misc2: 0 }, // S_MISSILE3
    state_t { sprite: SPR_MISF, frame: 32768, tics: 3, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_MISSILEFLASH2, misc1: 0, misc2: 0 }, // S_MISSILEFLASH1
    state_t { sprite: SPR_MISF, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_MISSILEFLASH3, misc1: 0, misc2: 0 }, // S_MISSILEFLASH2
    state_t { sprite: SPR_MISF, frame: 32770, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Light2), nextstate: S_MISSILEFLASH4, misc1: 0, misc2: 0 }, // S_MISSILEFLASH3
    state_t { sprite: SPR_MISF, frame: 32771, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Light2), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_MISSILEFLASH4
    state_t { sprite: SPR_SAWG, frame: 2, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_SAWB, misc1: 0, misc2: 0 }, // S_SAW
    state_t { sprite: SPR_SAWG, frame: 3, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_SAW, misc1: 0, misc2: 0 }, // S_SAWB
    state_t { sprite: SPR_SAWG, frame: 2, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_SAWDOWN, misc1: 0, misc2: 0 }, // S_SAWDOWN
    state_t { sprite: SPR_SAWG, frame: 2, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_SAWUP, misc1: 0, misc2: 0 }, // S_SAWUP
    state_t { sprite: SPR_SAWG, frame: 0, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Saw), nextstate: S_SAW2, misc1: 0, misc2: 0 }, // S_SAW1
    state_t { sprite: SPR_SAWG, frame: 1, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Saw), nextstate: S_SAW3, misc1: 0, misc2: 0 }, // S_SAW2
    state_t { sprite: SPR_SAWG, frame: 1, tics: 0, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_SAW, misc1: 0, misc2: 0 }, // S_SAW3
    state_t { sprite: SPR_PLSG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_PLASMA, misc1: 0, misc2: 0 }, // S_PLASMA
    state_t { sprite: SPR_PLSG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_PLASMADOWN, misc1: 0, misc2: 0 }, // S_PLASMADOWN
    state_t { sprite: SPR_PLSG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_PLASMAUP, misc1: 0, misc2: 0 }, // S_PLASMAUP
    state_t { sprite: SPR_PLSG, frame: 0, tics: 3, action: actionf_t::acp2(crate::p_pspr::A_FirePlasma), nextstate: S_PLASMA2, misc1: 0, misc2: 0 }, // S_PLASMA1
    state_t { sprite: SPR_PLSG, frame: 1, tics: 20, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_PLASMA, misc1: 0, misc2: 0 }, // S_PLASMA2
    state_t { sprite: SPR_PLSF, frame: 32768, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_PLASMAFLASH1
    state_t { sprite: SPR_PLSF, frame: 32769, tics: 4, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_PLASMAFLASH2
    state_t { sprite: SPR_BFGG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_WeaponReady), nextstate: S_BFG, misc1: 0, misc2: 0 }, // S_BFG
    state_t { sprite: SPR_BFGG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Lower), nextstate: S_BFGDOWN, misc1: 0, misc2: 0 }, // S_BFGDOWN
    state_t { sprite: SPR_BFGG, frame: 0, tics: 1, action: actionf_t::acp2(crate::p_pspr::A_Raise), nextstate: S_BFGUP, misc1: 0, misc2: 0 }, // S_BFGUP
    state_t { sprite: SPR_BFGG, frame: 0, tics: 20, action: actionf_t::acp2(crate::p_pspr::A_BFGsound), nextstate: S_BFG2, misc1: 0, misc2: 0 }, // S_BFG1
    state_t { sprite: SPR_BFGG, frame: 1, tics: 10, action: actionf_t::acp2(crate::p_pspr::A_GunFlash), nextstate: S_BFG3, misc1: 0, misc2: 0 }, // S_BFG2
    state_t { sprite: SPR_BFGG, frame: 1, tics: 10, action: actionf_t::acp2(crate::p_pspr::A_FireBFG), nextstate: S_BFG4, misc1: 0, misc2: 0 }, // S_BFG3
    state_t { sprite: SPR_BFGG, frame: 1, tics: 20, action: actionf_t::acp2(crate::p_pspr::A_ReFire), nextstate: S_BFG, misc1: 0, misc2: 0 }, // S_BFG4
    state_t { sprite: SPR_BFGF, frame: 32768, tics: 11, action: actionf_t::acp2(crate::p_pspr::A_Light1), nextstate: S_BFGFLASH2, misc1: 0, misc2: 0 }, // S_BFGFLASH1
    state_t { sprite: SPR_BFGF, frame: 32769, tics: 6, action: actionf_t::acp2(crate::p_pspr::A_Light2), nextstate: S_LIGHTDONE, misc1: 0, misc2: 0 }, // S_BFGFLASH2
    state_t { sprite: SPR_BLUD, frame: 2, tics: 8, action: actionf_t::None, nextstate: S_BLOOD2, misc1: 0, misc2: 0 }, // S_BLOOD1
    state_t { sprite: SPR_BLUD, frame: 1, tics: 8, action: actionf_t::None, nextstate: S_BLOOD3, misc1: 0, misc2: 0 }, // S_BLOOD2
    state_t { sprite: SPR_BLUD, frame: 0, tics: 8, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BLOOD3
    state_t { sprite: SPR_PUFF, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_PUFF2, misc1: 0, misc2: 0 }, // S_PUFF1
    state_t { sprite: SPR_PUFF, frame: 1, tics: 4, action: actionf_t::None, nextstate: S_PUFF3, misc1: 0, misc2: 0 }, // S_PUFF2
    state_t { sprite: SPR_PUFF, frame: 2, tics: 4, action: actionf_t::None, nextstate: S_PUFF4, misc1: 0, misc2: 0 }, // S_PUFF3
    state_t { sprite: SPR_PUFF, frame: 3, tics: 4, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PUFF4
    state_t { sprite: SPR_BAL1, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_TBALL2, misc1: 0, misc2: 0 }, // S_TBALL1
    state_t { sprite: SPR_BAL1, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_TBALL1, misc1: 0, misc2: 0 }, // S_TBALL2
    state_t { sprite: SPR_BAL1, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_TBALLX2, misc1: 0, misc2: 0 }, // S_TBALLX1
    state_t { sprite: SPR_BAL1, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_TBALLX3, misc1: 0, misc2: 0 }, // S_TBALLX2
    state_t { sprite: SPR_BAL1, frame: 32772, tics: 6, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TBALLX3
    state_t { sprite: SPR_BAL2, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_RBALL2, misc1: 0, misc2: 0 }, // S_RBALL1
    state_t { sprite: SPR_BAL2, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_RBALL1, misc1: 0, misc2: 0 }, // S_RBALL2
    state_t { sprite: SPR_BAL2, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_RBALLX2, misc1: 0, misc2: 0 }, // S_RBALLX1
    state_t { sprite: SPR_BAL2, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_RBALLX3, misc1: 0, misc2: 0 }, // S_RBALLX2
    state_t { sprite: SPR_BAL2, frame: 32772, tics: 6, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_RBALLX3
    state_t { sprite: SPR_PLSS, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_PLASBALL2, misc1: 0, misc2: 0 }, // S_PLASBALL
    state_t { sprite: SPR_PLSS, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_PLASBALL, misc1: 0, misc2: 0 }, // S_PLASBALL2
    state_t { sprite: SPR_PLSE, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_PLASEXP2, misc1: 0, misc2: 0 }, // S_PLASEXP
    state_t { sprite: SPR_PLSE, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_PLASEXP3, misc1: 0, misc2: 0 }, // S_PLASEXP2
    state_t { sprite: SPR_PLSE, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_PLASEXP4, misc1: 0, misc2: 0 }, // S_PLASEXP3
    state_t { sprite: SPR_PLSE, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_PLASEXP5, misc1: 0, misc2: 0 }, // S_PLASEXP4
    state_t { sprite: SPR_PLSE, frame: 32772, tics: 4, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PLASEXP5
    state_t { sprite: SPR_MISL, frame: 32768, tics: 1, action: actionf_t::None, nextstate: S_ROCKET, misc1: 0, misc2: 0 }, // S_ROCKET
    state_t { sprite: SPR_BFS1, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_BFGSHOT2, misc1: 0, misc2: 0 }, // S_BFGSHOT
    state_t { sprite: SPR_BFS1, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_BFGSHOT, misc1: 0, misc2: 0 }, // S_BFGSHOT2
    state_t { sprite: SPR_BFE1, frame: 32768, tics: 8, action: actionf_t::None, nextstate: S_BFGLAND2, misc1: 0, misc2: 0 }, // S_BFGLAND
    state_t { sprite: SPR_BFE1, frame: 32769, tics: 8, action: actionf_t::None, nextstate: S_BFGLAND3, misc1: 0, misc2: 0 }, // S_BFGLAND2
    state_t { sprite: SPR_BFE1, frame: 32770, tics: 8, action: actionf_t::acp1(crate::p_pspr::A_BFGSpray), nextstate: S_BFGLAND4, misc1: 0, misc2: 0 }, // S_BFGLAND3
    state_t { sprite: SPR_BFE1, frame: 32771, tics: 8, action: actionf_t::None, nextstate: S_BFGLAND5, misc1: 0, misc2: 0 }, // S_BFGLAND4
    state_t { sprite: SPR_BFE1, frame: 32772, tics: 8, action: actionf_t::None, nextstate: S_BFGLAND6, misc1: 0, misc2: 0 }, // S_BFGLAND5
    state_t { sprite: SPR_BFE1, frame: 32773, tics: 8, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BFGLAND6
    state_t { sprite: SPR_BFE2, frame: 32768, tics: 8, action: actionf_t::None, nextstate: S_BFGEXP2, misc1: 0, misc2: 0 }, // S_BFGEXP
    state_t { sprite: SPR_BFE2, frame: 32769, tics: 8, action: actionf_t::None, nextstate: S_BFGEXP3, misc1: 0, misc2: 0 }, // S_BFGEXP2
    state_t { sprite: SPR_BFE2, frame: 32770, tics: 8, action: actionf_t::None, nextstate: S_BFGEXP4, misc1: 0, misc2: 0 }, // S_BFGEXP3
    state_t { sprite: SPR_BFE2, frame: 32771, tics: 8, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BFGEXP4
    state_t { sprite: SPR_MISL, frame: 32769, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Explode), nextstate: S_EXPLODE2, misc1: 0, misc2: 0 }, // S_EXPLODE1
    state_t { sprite: SPR_MISL, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_EXPLODE3, misc1: 0, misc2: 0 }, // S_EXPLODE2
    state_t { sprite: SPR_MISL, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_EXPLODE3
    state_t { sprite: SPR_TFOG, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_TFOG01, misc1: 0, misc2: 0 }, // S_TFOG
    state_t { sprite: SPR_TFOG, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_TFOG02, misc1: 0, misc2: 0 }, // S_TFOG01
    state_t { sprite: SPR_TFOG, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_TFOG2, misc1: 0, misc2: 0 }, // S_TFOG02
    state_t { sprite: SPR_TFOG, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_TFOG3, misc1: 0, misc2: 0 }, // S_TFOG2
    state_t { sprite: SPR_TFOG, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_TFOG4, misc1: 0, misc2: 0 }, // S_TFOG3
    state_t { sprite: SPR_TFOG, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_TFOG5, misc1: 0, misc2: 0 }, // S_TFOG4
    state_t { sprite: SPR_TFOG, frame: 32772, tics: 6, action: actionf_t::None, nextstate: S_TFOG6, misc1: 0, misc2: 0 }, // S_TFOG5
    state_t { sprite: SPR_TFOG, frame: 32773, tics: 6, action: actionf_t::None, nextstate: S_TFOG7, misc1: 0, misc2: 0 }, // S_TFOG6
    state_t { sprite: SPR_TFOG, frame: 32774, tics: 6, action: actionf_t::None, nextstate: S_TFOG8, misc1: 0, misc2: 0 }, // S_TFOG7
    state_t { sprite: SPR_TFOG, frame: 32775, tics: 6, action: actionf_t::None, nextstate: S_TFOG9, misc1: 0, misc2: 0 }, // S_TFOG8
    state_t { sprite: SPR_TFOG, frame: 32776, tics: 6, action: actionf_t::None, nextstate: S_TFOG10, misc1: 0, misc2: 0 }, // S_TFOG9
    state_t { sprite: SPR_TFOG, frame: 32777, tics: 6, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TFOG10
    state_t { sprite: SPR_IFOG, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_IFOG01, misc1: 0, misc2: 0 }, // S_IFOG
    state_t { sprite: SPR_IFOG, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_IFOG02, misc1: 0, misc2: 0 }, // S_IFOG01
    state_t { sprite: SPR_IFOG, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_IFOG2, misc1: 0, misc2: 0 }, // S_IFOG02
    state_t { sprite: SPR_IFOG, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_IFOG3, misc1: 0, misc2: 0 }, // S_IFOG2
    state_t { sprite: SPR_IFOG, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_IFOG4, misc1: 0, misc2: 0 }, // S_IFOG3
    state_t { sprite: SPR_IFOG, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_IFOG5, misc1: 0, misc2: 0 }, // S_IFOG4
    state_t { sprite: SPR_IFOG, frame: 32772, tics: 6, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_IFOG5
    state_t { sprite: SPR_PLAY, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PLAY
    state_t { sprite: SPR_PLAY, frame: 0, tics: 4, action: actionf_t::None, nextstate: S_PLAY_RUN2, misc1: 0, misc2: 0 }, // S_PLAY_RUN1
    state_t { sprite: SPR_PLAY, frame: 1, tics: 4, action: actionf_t::None, nextstate: S_PLAY_RUN3, misc1: 0, misc2: 0 }, // S_PLAY_RUN2
    state_t { sprite: SPR_PLAY, frame: 2, tics: 4, action: actionf_t::None, nextstate: S_PLAY_RUN4, misc1: 0, misc2: 0 }, // S_PLAY_RUN3
    state_t { sprite: SPR_PLAY, frame: 3, tics: 4, action: actionf_t::None, nextstate: S_PLAY_RUN1, misc1: 0, misc2: 0 }, // S_PLAY_RUN4
    state_t { sprite: SPR_PLAY, frame: 4, tics: 12, action: actionf_t::None, nextstate: S_PLAY, misc1: 0, misc2: 0 }, // S_PLAY_ATK1
    state_t { sprite: SPR_PLAY, frame: 32773, tics: 6, action: actionf_t::None, nextstate: S_PLAY_ATK1, misc1: 0, misc2: 0 }, // S_PLAY_ATK2
    state_t { sprite: SPR_PLAY, frame: 6, tics: 4, action: actionf_t::None, nextstate: S_PLAY_PAIN2, misc1: 0, misc2: 0 }, // S_PLAY_PAIN
    state_t { sprite: SPR_PLAY, frame: 6, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_PLAY, misc1: 0, misc2: 0 }, // S_PLAY_PAIN2
    state_t { sprite: SPR_PLAY, frame: 7, tics: 10, action: actionf_t::None, nextstate: S_PLAY_DIE2, misc1: 0, misc2: 0 }, // S_PLAY_DIE1
    state_t { sprite: SPR_PLAY, frame: 8, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_PlayerScream), nextstate: S_PLAY_DIE3, misc1: 0, misc2: 0 }, // S_PLAY_DIE2
    state_t { sprite: SPR_PLAY, frame: 9, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_PLAY_DIE4, misc1: 0, misc2: 0 }, // S_PLAY_DIE3
    state_t { sprite: SPR_PLAY, frame: 10, tics: 10, action: actionf_t::None, nextstate: S_PLAY_DIE5, misc1: 0, misc2: 0 }, // S_PLAY_DIE4
    state_t { sprite: SPR_PLAY, frame: 11, tics: 10, action: actionf_t::None, nextstate: S_PLAY_DIE6, misc1: 0, misc2: 0 }, // S_PLAY_DIE5
    state_t { sprite: SPR_PLAY, frame: 12, tics: 10, action: actionf_t::None, nextstate: S_PLAY_DIE7, misc1: 0, misc2: 0 }, // S_PLAY_DIE6
    state_t { sprite: SPR_PLAY, frame: 13, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PLAY_DIE7
    state_t { sprite: SPR_PLAY, frame: 14, tics: 5, action: actionf_t::None, nextstate: S_PLAY_XDIE2, misc1: 0, misc2: 0 }, // S_PLAY_XDIE1
    state_t { sprite: SPR_PLAY, frame: 15, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_XScream), nextstate: S_PLAY_XDIE3, misc1: 0, misc2: 0 }, // S_PLAY_XDIE2
    state_t { sprite: SPR_PLAY, frame: 16, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_PLAY_XDIE4, misc1: 0, misc2: 0 }, // S_PLAY_XDIE3
    state_t { sprite: SPR_PLAY, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_PLAY_XDIE5, misc1: 0, misc2: 0 }, // S_PLAY_XDIE4
    state_t { sprite: SPR_PLAY, frame: 18, tics: 5, action: actionf_t::None, nextstate: S_PLAY_XDIE6, misc1: 0, misc2: 0 }, // S_PLAY_XDIE5
    state_t { sprite: SPR_PLAY, frame: 19, tics: 5, action: actionf_t::None, nextstate: S_PLAY_XDIE7, misc1: 0, misc2: 0 }, // S_PLAY_XDIE6
    state_t { sprite: SPR_PLAY, frame: 20, tics: 5, action: actionf_t::None, nextstate: S_PLAY_XDIE8, misc1: 0, misc2: 0 }, // S_PLAY_XDIE7
    state_t { sprite: SPR_PLAY, frame: 21, tics: 5, action: actionf_t::None, nextstate: S_PLAY_XDIE9, misc1: 0, misc2: 0 }, // S_PLAY_XDIE8
    state_t { sprite: SPR_PLAY, frame: 22, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PLAY_XDIE9
    state_t { sprite: SPR_POSS, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_POSS_STND2, misc1: 0, misc2: 0 }, // S_POSS_STND
    state_t { sprite: SPR_POSS, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_POSS_STND, misc1: 0, misc2: 0 }, // S_POSS_STND2
    state_t { sprite: SPR_POSS, frame: 0, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN2, misc1: 0, misc2: 0 }, // S_POSS_RUN1
    state_t { sprite: SPR_POSS, frame: 0, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN3, misc1: 0, misc2: 0 }, // S_POSS_RUN2
    state_t { sprite: SPR_POSS, frame: 1, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN4, misc1: 0, misc2: 0 }, // S_POSS_RUN3
    state_t { sprite: SPR_POSS, frame: 1, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN5, misc1: 0, misc2: 0 }, // S_POSS_RUN4
    state_t { sprite: SPR_POSS, frame: 2, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN6, misc1: 0, misc2: 0 }, // S_POSS_RUN5
    state_t { sprite: SPR_POSS, frame: 2, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN7, misc1: 0, misc2: 0 }, // S_POSS_RUN6
    state_t { sprite: SPR_POSS, frame: 3, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN8, misc1: 0, misc2: 0 }, // S_POSS_RUN7
    state_t { sprite: SPR_POSS, frame: 3, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_POSS_RUN1, misc1: 0, misc2: 0 }, // S_POSS_RUN8
    state_t { sprite: SPR_POSS, frame: 4, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_POSS_ATK2, misc1: 0, misc2: 0 }, // S_POSS_ATK1
    state_t { sprite: SPR_POSS, frame: 5, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_PosAttack), nextstate: S_POSS_ATK3, misc1: 0, misc2: 0 }, // S_POSS_ATK2
    state_t { sprite: SPR_POSS, frame: 4, tics: 8, action: actionf_t::None, nextstate: S_POSS_RUN1, misc1: 0, misc2: 0 }, // S_POSS_ATK3
    state_t { sprite: SPR_POSS, frame: 6, tics: 3, action: actionf_t::None, nextstate: S_POSS_PAIN2, misc1: 0, misc2: 0 }, // S_POSS_PAIN
    state_t { sprite: SPR_POSS, frame: 6, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_POSS_RUN1, misc1: 0, misc2: 0 }, // S_POSS_PAIN2
    state_t { sprite: SPR_POSS, frame: 7, tics: 5, action: actionf_t::None, nextstate: S_POSS_DIE2, misc1: 0, misc2: 0 }, // S_POSS_DIE1
    state_t { sprite: SPR_POSS, frame: 8, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_POSS_DIE3, misc1: 0, misc2: 0 }, // S_POSS_DIE2
    state_t { sprite: SPR_POSS, frame: 9, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_POSS_DIE4, misc1: 0, misc2: 0 }, // S_POSS_DIE3
    state_t { sprite: SPR_POSS, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_POSS_DIE5, misc1: 0, misc2: 0 }, // S_POSS_DIE4
    state_t { sprite: SPR_POSS, frame: 11, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_POSS_DIE5
    state_t { sprite: SPR_POSS, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_POSS_XDIE2, misc1: 0, misc2: 0 }, // S_POSS_XDIE1
    state_t { sprite: SPR_POSS, frame: 13, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_XScream), nextstate: S_POSS_XDIE3, misc1: 0, misc2: 0 }, // S_POSS_XDIE2
    state_t { sprite: SPR_POSS, frame: 14, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_POSS_XDIE4, misc1: 0, misc2: 0 }, // S_POSS_XDIE3
    state_t { sprite: SPR_POSS, frame: 15, tics: 5, action: actionf_t::None, nextstate: S_POSS_XDIE5, misc1: 0, misc2: 0 }, // S_POSS_XDIE4
    state_t { sprite: SPR_POSS, frame: 16, tics: 5, action: actionf_t::None, nextstate: S_POSS_XDIE6, misc1: 0, misc2: 0 }, // S_POSS_XDIE5
    state_t { sprite: SPR_POSS, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_POSS_XDIE7, misc1: 0, misc2: 0 }, // S_POSS_XDIE6
    state_t { sprite: SPR_POSS, frame: 18, tics: 5, action: actionf_t::None, nextstate: S_POSS_XDIE8, misc1: 0, misc2: 0 }, // S_POSS_XDIE7
    state_t { sprite: SPR_POSS, frame: 19, tics: 5, action: actionf_t::None, nextstate: S_POSS_XDIE9, misc1: 0, misc2: 0 }, // S_POSS_XDIE8
    state_t { sprite: SPR_POSS, frame: 20, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_POSS_XDIE9
    state_t { sprite: SPR_POSS, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_POSS_RAISE2, misc1: 0, misc2: 0 }, // S_POSS_RAISE1
    state_t { sprite: SPR_POSS, frame: 9, tics: 5, action: actionf_t::None, nextstate: S_POSS_RAISE3, misc1: 0, misc2: 0 }, // S_POSS_RAISE2
    state_t { sprite: SPR_POSS, frame: 8, tics: 5, action: actionf_t::None, nextstate: S_POSS_RAISE4, misc1: 0, misc2: 0 }, // S_POSS_RAISE3
    state_t { sprite: SPR_POSS, frame: 7, tics: 5, action: actionf_t::None, nextstate: S_POSS_RUN1, misc1: 0, misc2: 0 }, // S_POSS_RAISE4
    state_t { sprite: SPR_SPOS, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SPOS_STND2, misc1: 0, misc2: 0 }, // S_SPOS_STND
    state_t { sprite: SPR_SPOS, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SPOS_STND, misc1: 0, misc2: 0 }, // S_SPOS_STND2
    state_t { sprite: SPR_SPOS, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN2, misc1: 0, misc2: 0 }, // S_SPOS_RUN1
    state_t { sprite: SPR_SPOS, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN3, misc1: 0, misc2: 0 }, // S_SPOS_RUN2
    state_t { sprite: SPR_SPOS, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN4, misc1: 0, misc2: 0 }, // S_SPOS_RUN3
    state_t { sprite: SPR_SPOS, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN5, misc1: 0, misc2: 0 }, // S_SPOS_RUN4
    state_t { sprite: SPR_SPOS, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN6, misc1: 0, misc2: 0 }, // S_SPOS_RUN5
    state_t { sprite: SPR_SPOS, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN7, misc1: 0, misc2: 0 }, // S_SPOS_RUN6
    state_t { sprite: SPR_SPOS, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN8, misc1: 0, misc2: 0 }, // S_SPOS_RUN7
    state_t { sprite: SPR_SPOS, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPOS_RUN1, misc1: 0, misc2: 0 }, // S_SPOS_RUN8
    state_t { sprite: SPR_SPOS, frame: 4, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SPOS_ATK2, misc1: 0, misc2: 0 }, // S_SPOS_ATK1
    state_t { sprite: SPR_SPOS, frame: 32773, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_SPosAttack), nextstate: S_SPOS_ATK3, misc1: 0, misc2: 0 }, // S_SPOS_ATK2
    state_t { sprite: SPR_SPOS, frame: 4, tics: 10, action: actionf_t::None, nextstate: S_SPOS_RUN1, misc1: 0, misc2: 0 }, // S_SPOS_ATK3
    state_t { sprite: SPR_SPOS, frame: 6, tics: 3, action: actionf_t::None, nextstate: S_SPOS_PAIN2, misc1: 0, misc2: 0 }, // S_SPOS_PAIN
    state_t { sprite: SPR_SPOS, frame: 6, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_SPOS_RUN1, misc1: 0, misc2: 0 }, // S_SPOS_PAIN2
    state_t { sprite: SPR_SPOS, frame: 7, tics: 5, action: actionf_t::None, nextstate: S_SPOS_DIE2, misc1: 0, misc2: 0 }, // S_SPOS_DIE1
    state_t { sprite: SPR_SPOS, frame: 8, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_SPOS_DIE3, misc1: 0, misc2: 0 }, // S_SPOS_DIE2
    state_t { sprite: SPR_SPOS, frame: 9, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SPOS_DIE4, misc1: 0, misc2: 0 }, // S_SPOS_DIE3
    state_t { sprite: SPR_SPOS, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_SPOS_DIE5, misc1: 0, misc2: 0 }, // S_SPOS_DIE4
    state_t { sprite: SPR_SPOS, frame: 11, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SPOS_DIE5
    state_t { sprite: SPR_SPOS, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_SPOS_XDIE2, misc1: 0, misc2: 0 }, // S_SPOS_XDIE1
    state_t { sprite: SPR_SPOS, frame: 13, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_XScream), nextstate: S_SPOS_XDIE3, misc1: 0, misc2: 0 }, // S_SPOS_XDIE2
    state_t { sprite: SPR_SPOS, frame: 14, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SPOS_XDIE4, misc1: 0, misc2: 0 }, // S_SPOS_XDIE3
    state_t { sprite: SPR_SPOS, frame: 15, tics: 5, action: actionf_t::None, nextstate: S_SPOS_XDIE5, misc1: 0, misc2: 0 }, // S_SPOS_XDIE4
    state_t { sprite: SPR_SPOS, frame: 16, tics: 5, action: actionf_t::None, nextstate: S_SPOS_XDIE6, misc1: 0, misc2: 0 }, // S_SPOS_XDIE5
    state_t { sprite: SPR_SPOS, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_SPOS_XDIE7, misc1: 0, misc2: 0 }, // S_SPOS_XDIE6
    state_t { sprite: SPR_SPOS, frame: 18, tics: 5, action: actionf_t::None, nextstate: S_SPOS_XDIE8, misc1: 0, misc2: 0 }, // S_SPOS_XDIE7
    state_t { sprite: SPR_SPOS, frame: 19, tics: 5, action: actionf_t::None, nextstate: S_SPOS_XDIE9, misc1: 0, misc2: 0 }, // S_SPOS_XDIE8
    state_t { sprite: SPR_SPOS, frame: 20, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SPOS_XDIE9
    state_t { sprite: SPR_SPOS, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_SPOS_RAISE2, misc1: 0, misc2: 0 }, // S_SPOS_RAISE1
    state_t { sprite: SPR_SPOS, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_SPOS_RAISE3, misc1: 0, misc2: 0 }, // S_SPOS_RAISE2
    state_t { sprite: SPR_SPOS, frame: 9, tics: 5, action: actionf_t::None, nextstate: S_SPOS_RAISE4, misc1: 0, misc2: 0 }, // S_SPOS_RAISE3
    state_t { sprite: SPR_SPOS, frame: 8, tics: 5, action: actionf_t::None, nextstate: S_SPOS_RAISE5, misc1: 0, misc2: 0 }, // S_SPOS_RAISE4
    state_t { sprite: SPR_SPOS, frame: 7, tics: 5, action: actionf_t::None, nextstate: S_SPOS_RUN1, misc1: 0, misc2: 0 }, // S_SPOS_RAISE5
    state_t { sprite: SPR_VILE, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_VILE_STND2, misc1: 0, misc2: 0 }, // S_VILE_STND
    state_t { sprite: SPR_VILE, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_VILE_STND, misc1: 0, misc2: 0 }, // S_VILE_STND2
    state_t { sprite: SPR_VILE, frame: 0, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN2, misc1: 0, misc2: 0 }, // S_VILE_RUN1
    state_t { sprite: SPR_VILE, frame: 0, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN3, misc1: 0, misc2: 0 }, // S_VILE_RUN2
    state_t { sprite: SPR_VILE, frame: 1, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN4, misc1: 0, misc2: 0 }, // S_VILE_RUN3
    state_t { sprite: SPR_VILE, frame: 1, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN5, misc1: 0, misc2: 0 }, // S_VILE_RUN4
    state_t { sprite: SPR_VILE, frame: 2, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN6, misc1: 0, misc2: 0 }, // S_VILE_RUN5
    state_t { sprite: SPR_VILE, frame: 2, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN7, misc1: 0, misc2: 0 }, // S_VILE_RUN6
    state_t { sprite: SPR_VILE, frame: 3, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN8, misc1: 0, misc2: 0 }, // S_VILE_RUN7
    state_t { sprite: SPR_VILE, frame: 3, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN9, misc1: 0, misc2: 0 }, // S_VILE_RUN8
    state_t { sprite: SPR_VILE, frame: 4, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN10, misc1: 0, misc2: 0 }, // S_VILE_RUN9
    state_t { sprite: SPR_VILE, frame: 4, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN11, misc1: 0, misc2: 0 }, // S_VILE_RUN10
    state_t { sprite: SPR_VILE, frame: 5, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN12, misc1: 0, misc2: 0 }, // S_VILE_RUN11
    state_t { sprite: SPR_VILE, frame: 5, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_VileChase), nextstate: S_VILE_RUN1, misc1: 0, misc2: 0 }, // S_VILE_RUN12
    state_t { sprite: SPR_VILE, frame: 32774, tics: 0, action: actionf_t::acp1(crate::p_enemy::A_VileStart), nextstate: S_VILE_ATK2, misc1: 0, misc2: 0 }, // S_VILE_ATK1
    state_t { sprite: SPR_VILE, frame: 32774, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK3, misc1: 0, misc2: 0 }, // S_VILE_ATK2
    state_t { sprite: SPR_VILE, frame: 32775, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_VileTarget), nextstate: S_VILE_ATK4, misc1: 0, misc2: 0 }, // S_VILE_ATK3
    state_t { sprite: SPR_VILE, frame: 32776, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK5, misc1: 0, misc2: 0 }, // S_VILE_ATK4
    state_t { sprite: SPR_VILE, frame: 32777, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK6, misc1: 0, misc2: 0 }, // S_VILE_ATK5
    state_t { sprite: SPR_VILE, frame: 32778, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK7, misc1: 0, misc2: 0 }, // S_VILE_ATK6
    state_t { sprite: SPR_VILE, frame: 32779, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK8, misc1: 0, misc2: 0 }, // S_VILE_ATK7
    state_t { sprite: SPR_VILE, frame: 32780, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK9, misc1: 0, misc2: 0 }, // S_VILE_ATK8
    state_t { sprite: SPR_VILE, frame: 32781, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_VILE_ATK10, misc1: 0, misc2: 0 }, // S_VILE_ATK9
    state_t { sprite: SPR_VILE, frame: 32782, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_VileAttack), nextstate: S_VILE_ATK11, misc1: 0, misc2: 0 }, // S_VILE_ATK10
    state_t { sprite: SPR_VILE, frame: 32783, tics: 20, action: actionf_t::None, nextstate: S_VILE_RUN1, misc1: 0, misc2: 0 }, // S_VILE_ATK11
    state_t { sprite: SPR_VILE, frame: 32794, tics: 10, action: actionf_t::None, nextstate: S_VILE_HEAL2, misc1: 0, misc2: 0 }, // S_VILE_HEAL1
    state_t { sprite: SPR_VILE, frame: 32795, tics: 10, action: actionf_t::None, nextstate: S_VILE_HEAL3, misc1: 0, misc2: 0 }, // S_VILE_HEAL2
    state_t { sprite: SPR_VILE, frame: 32796, tics: 10, action: actionf_t::None, nextstate: S_VILE_RUN1, misc1: 0, misc2: 0 }, // S_VILE_HEAL3
    state_t { sprite: SPR_VILE, frame: 16, tics: 5, action: actionf_t::None, nextstate: S_VILE_PAIN2, misc1: 0, misc2: 0 }, // S_VILE_PAIN
    state_t { sprite: SPR_VILE, frame: 16, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_VILE_RUN1, misc1: 0, misc2: 0 }, // S_VILE_PAIN2
    state_t { sprite: SPR_VILE, frame: 16, tics: 7, action: actionf_t::None, nextstate: S_VILE_DIE2, misc1: 0, misc2: 0 }, // S_VILE_DIE1
    state_t { sprite: SPR_VILE, frame: 17, tics: 7, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_VILE_DIE3, misc1: 0, misc2: 0 }, // S_VILE_DIE2
    state_t { sprite: SPR_VILE, frame: 18, tics: 7, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_VILE_DIE4, misc1: 0, misc2: 0 }, // S_VILE_DIE3
    state_t { sprite: SPR_VILE, frame: 19, tics: 7, action: actionf_t::None, nextstate: S_VILE_DIE5, misc1: 0, misc2: 0 }, // S_VILE_DIE4
    state_t { sprite: SPR_VILE, frame: 20, tics: 7, action: actionf_t::None, nextstate: S_VILE_DIE6, misc1: 0, misc2: 0 }, // S_VILE_DIE5
    state_t { sprite: SPR_VILE, frame: 21, tics: 7, action: actionf_t::None, nextstate: S_VILE_DIE7, misc1: 0, misc2: 0 }, // S_VILE_DIE6
    state_t { sprite: SPR_VILE, frame: 22, tics: 7, action: actionf_t::None, nextstate: S_VILE_DIE8, misc1: 0, misc2: 0 }, // S_VILE_DIE7
    state_t { sprite: SPR_VILE, frame: 23, tics: 5, action: actionf_t::None, nextstate: S_VILE_DIE9, misc1: 0, misc2: 0 }, // S_VILE_DIE8
    state_t { sprite: SPR_VILE, frame: 24, tics: 5, action: actionf_t::None, nextstate: S_VILE_DIE10, misc1: 0, misc2: 0 }, // S_VILE_DIE9
    state_t { sprite: SPR_VILE, frame: 25, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_VILE_DIE10
    state_t { sprite: SPR_FIRE, frame: 32768, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_StartFire), nextstate: S_FIRE2, misc1: 0, misc2: 0 }, // S_FIRE1
    state_t { sprite: SPR_FIRE, frame: 32769, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE3, misc1: 0, misc2: 0 }, // S_FIRE2
    state_t { sprite: SPR_FIRE, frame: 32768, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE4, misc1: 0, misc2: 0 }, // S_FIRE3
    state_t { sprite: SPR_FIRE, frame: 32769, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE5, misc1: 0, misc2: 0 }, // S_FIRE4
    state_t { sprite: SPR_FIRE, frame: 32770, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_FireCrackle), nextstate: S_FIRE6, misc1: 0, misc2: 0 }, // S_FIRE5
    state_t { sprite: SPR_FIRE, frame: 32769, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE7, misc1: 0, misc2: 0 }, // S_FIRE6
    state_t { sprite: SPR_FIRE, frame: 32770, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE8, misc1: 0, misc2: 0 }, // S_FIRE7
    state_t { sprite: SPR_FIRE, frame: 32769, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE9, misc1: 0, misc2: 0 }, // S_FIRE8
    state_t { sprite: SPR_FIRE, frame: 32770, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE10, misc1: 0, misc2: 0 }, // S_FIRE9
    state_t { sprite: SPR_FIRE, frame: 32771, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE11, misc1: 0, misc2: 0 }, // S_FIRE10
    state_t { sprite: SPR_FIRE, frame: 32770, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE12, misc1: 0, misc2: 0 }, // S_FIRE11
    state_t { sprite: SPR_FIRE, frame: 32771, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE13, misc1: 0, misc2: 0 }, // S_FIRE12
    state_t { sprite: SPR_FIRE, frame: 32770, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE14, misc1: 0, misc2: 0 }, // S_FIRE13
    state_t { sprite: SPR_FIRE, frame: 32771, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE15, misc1: 0, misc2: 0 }, // S_FIRE14
    state_t { sprite: SPR_FIRE, frame: 32772, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE16, misc1: 0, misc2: 0 }, // S_FIRE15
    state_t { sprite: SPR_FIRE, frame: 32771, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE17, misc1: 0, misc2: 0 }, // S_FIRE16
    state_t { sprite: SPR_FIRE, frame: 32772, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE18, misc1: 0, misc2: 0 }, // S_FIRE17
    state_t { sprite: SPR_FIRE, frame: 32771, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE19, misc1: 0, misc2: 0 }, // S_FIRE18
    state_t { sprite: SPR_FIRE, frame: 32772, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_FireCrackle), nextstate: S_FIRE20, misc1: 0, misc2: 0 }, // S_FIRE19
    state_t { sprite: SPR_FIRE, frame: 32773, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE21, misc1: 0, misc2: 0 }, // S_FIRE20
    state_t { sprite: SPR_FIRE, frame: 32772, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE22, misc1: 0, misc2: 0 }, // S_FIRE21
    state_t { sprite: SPR_FIRE, frame: 32773, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE23, misc1: 0, misc2: 0 }, // S_FIRE22
    state_t { sprite: SPR_FIRE, frame: 32772, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE24, misc1: 0, misc2: 0 }, // S_FIRE23
    state_t { sprite: SPR_FIRE, frame: 32773, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE25, misc1: 0, misc2: 0 }, // S_FIRE24
    state_t { sprite: SPR_FIRE, frame: 32774, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE26, misc1: 0, misc2: 0 }, // S_FIRE25
    state_t { sprite: SPR_FIRE, frame: 32775, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE27, misc1: 0, misc2: 0 }, // S_FIRE26
    state_t { sprite: SPR_FIRE, frame: 32774, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE28, misc1: 0, misc2: 0 }, // S_FIRE27
    state_t { sprite: SPR_FIRE, frame: 32775, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE29, misc1: 0, misc2: 0 }, // S_FIRE28
    state_t { sprite: SPR_FIRE, frame: 32774, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_FIRE30, misc1: 0, misc2: 0 }, // S_FIRE29
    state_t { sprite: SPR_FIRE, frame: 32775, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_FIRE30
    state_t { sprite: SPR_PUFF, frame: 1, tics: 4, action: actionf_t::None, nextstate: S_SMOKE2, misc1: 0, misc2: 0 }, // S_SMOKE1
    state_t { sprite: SPR_PUFF, frame: 2, tics: 4, action: actionf_t::None, nextstate: S_SMOKE3, misc1: 0, misc2: 0 }, // S_SMOKE2
    state_t { sprite: SPR_PUFF, frame: 1, tics: 4, action: actionf_t::None, nextstate: S_SMOKE4, misc1: 0, misc2: 0 }, // S_SMOKE3
    state_t { sprite: SPR_PUFF, frame: 2, tics: 4, action: actionf_t::None, nextstate: S_SMOKE5, misc1: 0, misc2: 0 }, // S_SMOKE4
    state_t { sprite: SPR_PUFF, frame: 3, tics: 4, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SMOKE5
    state_t { sprite: SPR_FATB, frame: 32768, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Tracer), nextstate: S_TRACER2, misc1: 0, misc2: 0 }, // S_TRACER
    state_t { sprite: SPR_FATB, frame: 32769, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Tracer), nextstate: S_TRACER, misc1: 0, misc2: 0 }, // S_TRACER2
    state_t { sprite: SPR_FBXP, frame: 32768, tics: 8, action: actionf_t::None, nextstate: S_TRACEEXP2, misc1: 0, misc2: 0 }, // S_TRACEEXP1
    state_t { sprite: SPR_FBXP, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_TRACEEXP3, misc1: 0, misc2: 0 }, // S_TRACEEXP2
    state_t { sprite: SPR_FBXP, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TRACEEXP3
    state_t { sprite: SPR_SKEL, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SKEL_STND2, misc1: 0, misc2: 0 }, // S_SKEL_STND
    state_t { sprite: SPR_SKEL, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SKEL_STND, misc1: 0, misc2: 0 }, // S_SKEL_STND2
    state_t { sprite: SPR_SKEL, frame: 0, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN2, misc1: 0, misc2: 0 }, // S_SKEL_RUN1
    state_t { sprite: SPR_SKEL, frame: 0, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN3, misc1: 0, misc2: 0 }, // S_SKEL_RUN2
    state_t { sprite: SPR_SKEL, frame: 1, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN4, misc1: 0, misc2: 0 }, // S_SKEL_RUN3
    state_t { sprite: SPR_SKEL, frame: 1, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN5, misc1: 0, misc2: 0 }, // S_SKEL_RUN4
    state_t { sprite: SPR_SKEL, frame: 2, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN6, misc1: 0, misc2: 0 }, // S_SKEL_RUN5
    state_t { sprite: SPR_SKEL, frame: 2, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN7, misc1: 0, misc2: 0 }, // S_SKEL_RUN6
    state_t { sprite: SPR_SKEL, frame: 3, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN8, misc1: 0, misc2: 0 }, // S_SKEL_RUN7
    state_t { sprite: SPR_SKEL, frame: 3, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN9, misc1: 0, misc2: 0 }, // S_SKEL_RUN8
    state_t { sprite: SPR_SKEL, frame: 4, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN10, misc1: 0, misc2: 0 }, // S_SKEL_RUN9
    state_t { sprite: SPR_SKEL, frame: 4, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN11, misc1: 0, misc2: 0 }, // S_SKEL_RUN10
    state_t { sprite: SPR_SKEL, frame: 5, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN12, misc1: 0, misc2: 0 }, // S_SKEL_RUN11
    state_t { sprite: SPR_SKEL, frame: 5, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKEL_RUN1, misc1: 0, misc2: 0 }, // S_SKEL_RUN12
    state_t { sprite: SPR_SKEL, frame: 6, tics: 0, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SKEL_FIST2, misc1: 0, misc2: 0 }, // S_SKEL_FIST1
    state_t { sprite: SPR_SKEL, frame: 6, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_SkelWhoosh), nextstate: S_SKEL_FIST3, misc1: 0, misc2: 0 }, // S_SKEL_FIST2
    state_t { sprite: SPR_SKEL, frame: 7, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SKEL_FIST4, misc1: 0, misc2: 0 }, // S_SKEL_FIST3
    state_t { sprite: SPR_SKEL, frame: 8, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_SkelFist), nextstate: S_SKEL_RUN1, misc1: 0, misc2: 0 }, // S_SKEL_FIST4
    state_t { sprite: SPR_SKEL, frame: 32777, tics: 0, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SKEL_MISS2, misc1: 0, misc2: 0 }, // S_SKEL_MISS1
    state_t { sprite: SPR_SKEL, frame: 32777, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SKEL_MISS3, misc1: 0, misc2: 0 }, // S_SKEL_MISS2
    state_t { sprite: SPR_SKEL, frame: 10, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_SkelMissile), nextstate: S_SKEL_MISS4, misc1: 0, misc2: 0 }, // S_SKEL_MISS3
    state_t { sprite: SPR_SKEL, frame: 10, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SKEL_RUN1, misc1: 0, misc2: 0 }, // S_SKEL_MISS4
    state_t { sprite: SPR_SKEL, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_SKEL_PAIN2, misc1: 0, misc2: 0 }, // S_SKEL_PAIN
    state_t { sprite: SPR_SKEL, frame: 11, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_SKEL_RUN1, misc1: 0, misc2: 0 }, // S_SKEL_PAIN2
    state_t { sprite: SPR_SKEL, frame: 11, tics: 7, action: actionf_t::None, nextstate: S_SKEL_DIE2, misc1: 0, misc2: 0 }, // S_SKEL_DIE1
    state_t { sprite: SPR_SKEL, frame: 12, tics: 7, action: actionf_t::None, nextstate: S_SKEL_DIE3, misc1: 0, misc2: 0 }, // S_SKEL_DIE2
    state_t { sprite: SPR_SKEL, frame: 13, tics: 7, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_SKEL_DIE4, misc1: 0, misc2: 0 }, // S_SKEL_DIE3
    state_t { sprite: SPR_SKEL, frame: 14, tics: 7, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SKEL_DIE5, misc1: 0, misc2: 0 }, // S_SKEL_DIE4
    state_t { sprite: SPR_SKEL, frame: 15, tics: 7, action: actionf_t::None, nextstate: S_SKEL_DIE6, misc1: 0, misc2: 0 }, // S_SKEL_DIE5
    state_t { sprite: SPR_SKEL, frame: 16, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SKEL_DIE6
    state_t { sprite: SPR_SKEL, frame: 16, tics: 5, action: actionf_t::None, nextstate: S_SKEL_RAISE2, misc1: 0, misc2: 0 }, // S_SKEL_RAISE1
    state_t { sprite: SPR_SKEL, frame: 15, tics: 5, action: actionf_t::None, nextstate: S_SKEL_RAISE3, misc1: 0, misc2: 0 }, // S_SKEL_RAISE2
    state_t { sprite: SPR_SKEL, frame: 14, tics: 5, action: actionf_t::None, nextstate: S_SKEL_RAISE4, misc1: 0, misc2: 0 }, // S_SKEL_RAISE3
    state_t { sprite: SPR_SKEL, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_SKEL_RAISE5, misc1: 0, misc2: 0 }, // S_SKEL_RAISE4
    state_t { sprite: SPR_SKEL, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_SKEL_RAISE6, misc1: 0, misc2: 0 }, // S_SKEL_RAISE5
    state_t { sprite: SPR_SKEL, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_SKEL_RUN1, misc1: 0, misc2: 0 }, // S_SKEL_RAISE6
    state_t { sprite: SPR_MANF, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_FATSHOT2, misc1: 0, misc2: 0 }, // S_FATSHOT1
    state_t { sprite: SPR_MANF, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_FATSHOT1, misc1: 0, misc2: 0 }, // S_FATSHOT2
    state_t { sprite: SPR_MISL, frame: 32769, tics: 8, action: actionf_t::None, nextstate: S_FATSHOTX2, misc1: 0, misc2: 0 }, // S_FATSHOTX1
    state_t { sprite: SPR_MISL, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_FATSHOTX3, misc1: 0, misc2: 0 }, // S_FATSHOTX2
    state_t { sprite: SPR_MISL, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_FATSHOTX3
    state_t { sprite: SPR_FATT, frame: 0, tics: 15, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_FATT_STND2, misc1: 0, misc2: 0 }, // S_FATT_STND
    state_t { sprite: SPR_FATT, frame: 1, tics: 15, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_FATT_STND, misc1: 0, misc2: 0 }, // S_FATT_STND2
    state_t { sprite: SPR_FATT, frame: 0, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN2, misc1: 0, misc2: 0 }, // S_FATT_RUN1
    state_t { sprite: SPR_FATT, frame: 0, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN3, misc1: 0, misc2: 0 }, // S_FATT_RUN2
    state_t { sprite: SPR_FATT, frame: 1, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN4, misc1: 0, misc2: 0 }, // S_FATT_RUN3
    state_t { sprite: SPR_FATT, frame: 1, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN5, misc1: 0, misc2: 0 }, // S_FATT_RUN4
    state_t { sprite: SPR_FATT, frame: 2, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN6, misc1: 0, misc2: 0 }, // S_FATT_RUN5
    state_t { sprite: SPR_FATT, frame: 2, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN7, misc1: 0, misc2: 0 }, // S_FATT_RUN6
    state_t { sprite: SPR_FATT, frame: 3, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN8, misc1: 0, misc2: 0 }, // S_FATT_RUN7
    state_t { sprite: SPR_FATT, frame: 3, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN9, misc1: 0, misc2: 0 }, // S_FATT_RUN8
    state_t { sprite: SPR_FATT, frame: 4, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN10, misc1: 0, misc2: 0 }, // S_FATT_RUN9
    state_t { sprite: SPR_FATT, frame: 4, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN11, misc1: 0, misc2: 0 }, // S_FATT_RUN10
    state_t { sprite: SPR_FATT, frame: 5, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN12, misc1: 0, misc2: 0 }, // S_FATT_RUN11
    state_t { sprite: SPR_FATT, frame: 5, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_FATT_RUN1, misc1: 0, misc2: 0 }, // S_FATT_RUN12
    state_t { sprite: SPR_FATT, frame: 6, tics: 20, action: actionf_t::acp1(crate::p_enemy::A_FatRaise), nextstate: S_FATT_ATK2, misc1: 0, misc2: 0 }, // S_FATT_ATK1
    state_t { sprite: SPR_FATT, frame: 32775, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FatAttack1), nextstate: S_FATT_ATK3, misc1: 0, misc2: 0 }, // S_FATT_ATK2
    state_t { sprite: SPR_FATT, frame: 8, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_FATT_ATK4, misc1: 0, misc2: 0 }, // S_FATT_ATK3
    state_t { sprite: SPR_FATT, frame: 6, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_FATT_ATK5, misc1: 0, misc2: 0 }, // S_FATT_ATK4
    state_t { sprite: SPR_FATT, frame: 32775, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FatAttack2), nextstate: S_FATT_ATK6, misc1: 0, misc2: 0 }, // S_FATT_ATK5
    state_t { sprite: SPR_FATT, frame: 8, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_FATT_ATK7, misc1: 0, misc2: 0 }, // S_FATT_ATK6
    state_t { sprite: SPR_FATT, frame: 6, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_FATT_ATK8, misc1: 0, misc2: 0 }, // S_FATT_ATK7
    state_t { sprite: SPR_FATT, frame: 32775, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FatAttack3), nextstate: S_FATT_ATK9, misc1: 0, misc2: 0 }, // S_FATT_ATK8
    state_t { sprite: SPR_FATT, frame: 8, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_FATT_ATK10, misc1: 0, misc2: 0 }, // S_FATT_ATK9
    state_t { sprite: SPR_FATT, frame: 6, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_FATT_RUN1, misc1: 0, misc2: 0 }, // S_FATT_ATK10
    state_t { sprite: SPR_FATT, frame: 9, tics: 3, action: actionf_t::None, nextstate: S_FATT_PAIN2, misc1: 0, misc2: 0 }, // S_FATT_PAIN
    state_t { sprite: SPR_FATT, frame: 9, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_FATT_RUN1, misc1: 0, misc2: 0 }, // S_FATT_PAIN2
    state_t { sprite: SPR_FATT, frame: 10, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE2, misc1: 0, misc2: 0 }, // S_FATT_DIE1
    state_t { sprite: SPR_FATT, frame: 11, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_FATT_DIE3, misc1: 0, misc2: 0 }, // S_FATT_DIE2
    state_t { sprite: SPR_FATT, frame: 12, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_FATT_DIE4, misc1: 0, misc2: 0 }, // S_FATT_DIE3
    state_t { sprite: SPR_FATT, frame: 13, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE5, misc1: 0, misc2: 0 }, // S_FATT_DIE4
    state_t { sprite: SPR_FATT, frame: 14, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE6, misc1: 0, misc2: 0 }, // S_FATT_DIE5
    state_t { sprite: SPR_FATT, frame: 15, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE7, misc1: 0, misc2: 0 }, // S_FATT_DIE6
    state_t { sprite: SPR_FATT, frame: 16, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE8, misc1: 0, misc2: 0 }, // S_FATT_DIE7
    state_t { sprite: SPR_FATT, frame: 17, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE9, misc1: 0, misc2: 0 }, // S_FATT_DIE8
    state_t { sprite: SPR_FATT, frame: 18, tics: 6, action: actionf_t::None, nextstate: S_FATT_DIE10, misc1: 0, misc2: 0 }, // S_FATT_DIE9
    state_t { sprite: SPR_FATT, frame: 19, tics: -1, action: actionf_t::acp1(crate::p_enemy::A_BossDeath), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_FATT_DIE10
    state_t { sprite: SPR_FATT, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE2, misc1: 0, misc2: 0 }, // S_FATT_RAISE1
    state_t { sprite: SPR_FATT, frame: 16, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE3, misc1: 0, misc2: 0 }, // S_FATT_RAISE2
    state_t { sprite: SPR_FATT, frame: 15, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE4, misc1: 0, misc2: 0 }, // S_FATT_RAISE3
    state_t { sprite: SPR_FATT, frame: 14, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE5, misc1: 0, misc2: 0 }, // S_FATT_RAISE4
    state_t { sprite: SPR_FATT, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE6, misc1: 0, misc2: 0 }, // S_FATT_RAISE5
    state_t { sprite: SPR_FATT, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE7, misc1: 0, misc2: 0 }, // S_FATT_RAISE6
    state_t { sprite: SPR_FATT, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_FATT_RAISE8, misc1: 0, misc2: 0 }, // S_FATT_RAISE7
    state_t { sprite: SPR_FATT, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_FATT_RUN1, misc1: 0, misc2: 0 }, // S_FATT_RAISE8
    state_t { sprite: SPR_CPOS, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_CPOS_STND2, misc1: 0, misc2: 0 }, // S_CPOS_STND
    state_t { sprite: SPR_CPOS, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_CPOS_STND, misc1: 0, misc2: 0 }, // S_CPOS_STND2
    state_t { sprite: SPR_CPOS, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN2, misc1: 0, misc2: 0 }, // S_CPOS_RUN1
    state_t { sprite: SPR_CPOS, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN3, misc1: 0, misc2: 0 }, // S_CPOS_RUN2
    state_t { sprite: SPR_CPOS, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN4, misc1: 0, misc2: 0 }, // S_CPOS_RUN3
    state_t { sprite: SPR_CPOS, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN5, misc1: 0, misc2: 0 }, // S_CPOS_RUN4
    state_t { sprite: SPR_CPOS, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN6, misc1: 0, misc2: 0 }, // S_CPOS_RUN5
    state_t { sprite: SPR_CPOS, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN7, misc1: 0, misc2: 0 }, // S_CPOS_RUN6
    state_t { sprite: SPR_CPOS, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN8, misc1: 0, misc2: 0 }, // S_CPOS_RUN7
    state_t { sprite: SPR_CPOS, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CPOS_RUN1, misc1: 0, misc2: 0 }, // S_CPOS_RUN8
    state_t { sprite: SPR_CPOS, frame: 4, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_CPOS_ATK2, misc1: 0, misc2: 0 }, // S_CPOS_ATK1
    state_t { sprite: SPR_CPOS, frame: 32773, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_CPosAttack), nextstate: S_CPOS_ATK3, misc1: 0, misc2: 0 }, // S_CPOS_ATK2
    state_t { sprite: SPR_CPOS, frame: 32772, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_CPosAttack), nextstate: S_CPOS_ATK4, misc1: 0, misc2: 0 }, // S_CPOS_ATK3
    state_t { sprite: SPR_CPOS, frame: 5, tics: 1, action: actionf_t::acp1(crate::p_enemy::A_CPosRefire), nextstate: S_CPOS_ATK2, misc1: 0, misc2: 0 }, // S_CPOS_ATK4
    state_t { sprite: SPR_CPOS, frame: 6, tics: 3, action: actionf_t::None, nextstate: S_CPOS_PAIN2, misc1: 0, misc2: 0 }, // S_CPOS_PAIN
    state_t { sprite: SPR_CPOS, frame: 6, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_CPOS_RUN1, misc1: 0, misc2: 0 }, // S_CPOS_PAIN2
    state_t { sprite: SPR_CPOS, frame: 7, tics: 5, action: actionf_t::None, nextstate: S_CPOS_DIE2, misc1: 0, misc2: 0 }, // S_CPOS_DIE1
    state_t { sprite: SPR_CPOS, frame: 8, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_CPOS_DIE3, misc1: 0, misc2: 0 }, // S_CPOS_DIE2
    state_t { sprite: SPR_CPOS, frame: 9, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_CPOS_DIE4, misc1: 0, misc2: 0 }, // S_CPOS_DIE3
    state_t { sprite: SPR_CPOS, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_CPOS_DIE5, misc1: 0, misc2: 0 }, // S_CPOS_DIE4
    state_t { sprite: SPR_CPOS, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_CPOS_DIE6, misc1: 0, misc2: 0 }, // S_CPOS_DIE5
    state_t { sprite: SPR_CPOS, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_CPOS_DIE7, misc1: 0, misc2: 0 }, // S_CPOS_DIE6
    state_t { sprite: SPR_CPOS, frame: 13, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CPOS_DIE7
    state_t { sprite: SPR_CPOS, frame: 14, tics: 5, action: actionf_t::None, nextstate: S_CPOS_XDIE2, misc1: 0, misc2: 0 }, // S_CPOS_XDIE1
    state_t { sprite: SPR_CPOS, frame: 15, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_XScream), nextstate: S_CPOS_XDIE3, misc1: 0, misc2: 0 }, // S_CPOS_XDIE2
    state_t { sprite: SPR_CPOS, frame: 16, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_CPOS_XDIE4, misc1: 0, misc2: 0 }, // S_CPOS_XDIE3
    state_t { sprite: SPR_CPOS, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_CPOS_XDIE5, misc1: 0, misc2: 0 }, // S_CPOS_XDIE4
    state_t { sprite: SPR_CPOS, frame: 18, tics: 5, action: actionf_t::None, nextstate: S_CPOS_XDIE6, misc1: 0, misc2: 0 }, // S_CPOS_XDIE5
    state_t { sprite: SPR_CPOS, frame: 19, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CPOS_XDIE6
    state_t { sprite: SPR_CPOS, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RAISE2, misc1: 0, misc2: 0 }, // S_CPOS_RAISE1
    state_t { sprite: SPR_CPOS, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RAISE3, misc1: 0, misc2: 0 }, // S_CPOS_RAISE2
    state_t { sprite: SPR_CPOS, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RAISE4, misc1: 0, misc2: 0 }, // S_CPOS_RAISE3
    state_t { sprite: SPR_CPOS, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RAISE5, misc1: 0, misc2: 0 }, // S_CPOS_RAISE4
    state_t { sprite: SPR_CPOS, frame: 9, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RAISE6, misc1: 0, misc2: 0 }, // S_CPOS_RAISE5
    state_t { sprite: SPR_CPOS, frame: 8, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RAISE7, misc1: 0, misc2: 0 }, // S_CPOS_RAISE6
    state_t { sprite: SPR_CPOS, frame: 7, tics: 5, action: actionf_t::None, nextstate: S_CPOS_RUN1, misc1: 0, misc2: 0 }, // S_CPOS_RAISE7
    state_t { sprite: SPR_TROO, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_TROO_STND2, misc1: 0, misc2: 0 }, // S_TROO_STND
    state_t { sprite: SPR_TROO, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_TROO_STND, misc1: 0, misc2: 0 }, // S_TROO_STND2
    state_t { sprite: SPR_TROO, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN2, misc1: 0, misc2: 0 }, // S_TROO_RUN1
    state_t { sprite: SPR_TROO, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN3, misc1: 0, misc2: 0 }, // S_TROO_RUN2
    state_t { sprite: SPR_TROO, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN4, misc1: 0, misc2: 0 }, // S_TROO_RUN3
    state_t { sprite: SPR_TROO, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN5, misc1: 0, misc2: 0 }, // S_TROO_RUN4
    state_t { sprite: SPR_TROO, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN6, misc1: 0, misc2: 0 }, // S_TROO_RUN5
    state_t { sprite: SPR_TROO, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN7, misc1: 0, misc2: 0 }, // S_TROO_RUN6
    state_t { sprite: SPR_TROO, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN8, misc1: 0, misc2: 0 }, // S_TROO_RUN7
    state_t { sprite: SPR_TROO, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_TROO_RUN1, misc1: 0, misc2: 0 }, // S_TROO_RUN8
    state_t { sprite: SPR_TROO, frame: 4, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_TROO_ATK2, misc1: 0, misc2: 0 }, // S_TROO_ATK1
    state_t { sprite: SPR_TROO, frame: 5, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_TROO_ATK3, misc1: 0, misc2: 0 }, // S_TROO_ATK2
    state_t { sprite: SPR_TROO, frame: 6, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_TroopAttack), nextstate: S_TROO_RUN1, misc1: 0, misc2: 0 }, // S_TROO_ATK3
    state_t { sprite: SPR_TROO, frame: 7, tics: 2, action: actionf_t::None, nextstate: S_TROO_PAIN2, misc1: 0, misc2: 0 }, // S_TROO_PAIN
    state_t { sprite: SPR_TROO, frame: 7, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_TROO_RUN1, misc1: 0, misc2: 0 }, // S_TROO_PAIN2
    state_t { sprite: SPR_TROO, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_TROO_DIE2, misc1: 0, misc2: 0 }, // S_TROO_DIE1
    state_t { sprite: SPR_TROO, frame: 9, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_TROO_DIE3, misc1: 0, misc2: 0 }, // S_TROO_DIE2
    state_t { sprite: SPR_TROO, frame: 10, tics: 6, action: actionf_t::None, nextstate: S_TROO_DIE4, misc1: 0, misc2: 0 }, // S_TROO_DIE3
    state_t { sprite: SPR_TROO, frame: 11, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_TROO_DIE5, misc1: 0, misc2: 0 }, // S_TROO_DIE4
    state_t { sprite: SPR_TROO, frame: 12, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TROO_DIE5
    state_t { sprite: SPR_TROO, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_TROO_XDIE2, misc1: 0, misc2: 0 }, // S_TROO_XDIE1
    state_t { sprite: SPR_TROO, frame: 14, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_XScream), nextstate: S_TROO_XDIE3, misc1: 0, misc2: 0 }, // S_TROO_XDIE2
    state_t { sprite: SPR_TROO, frame: 15, tics: 5, action: actionf_t::None, nextstate: S_TROO_XDIE4, misc1: 0, misc2: 0 }, // S_TROO_XDIE3
    state_t { sprite: SPR_TROO, frame: 16, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_TROO_XDIE5, misc1: 0, misc2: 0 }, // S_TROO_XDIE4
    state_t { sprite: SPR_TROO, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_TROO_XDIE6, misc1: 0, misc2: 0 }, // S_TROO_XDIE5
    state_t { sprite: SPR_TROO, frame: 18, tics: 5, action: actionf_t::None, nextstate: S_TROO_XDIE7, misc1: 0, misc2: 0 }, // S_TROO_XDIE6
    state_t { sprite: SPR_TROO, frame: 19, tics: 5, action: actionf_t::None, nextstate: S_TROO_XDIE8, misc1: 0, misc2: 0 }, // S_TROO_XDIE7
    state_t { sprite: SPR_TROO, frame: 20, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TROO_XDIE8
    state_t { sprite: SPR_TROO, frame: 12, tics: 8, action: actionf_t::None, nextstate: S_TROO_RAISE2, misc1: 0, misc2: 0 }, // S_TROO_RAISE1
    state_t { sprite: SPR_TROO, frame: 11, tics: 8, action: actionf_t::None, nextstate: S_TROO_RAISE3, misc1: 0, misc2: 0 }, // S_TROO_RAISE2
    state_t { sprite: SPR_TROO, frame: 10, tics: 6, action: actionf_t::None, nextstate: S_TROO_RAISE4, misc1: 0, misc2: 0 }, // S_TROO_RAISE3
    state_t { sprite: SPR_TROO, frame: 9, tics: 6, action: actionf_t::None, nextstate: S_TROO_RAISE5, misc1: 0, misc2: 0 }, // S_TROO_RAISE4
    state_t { sprite: SPR_TROO, frame: 8, tics: 6, action: actionf_t::None, nextstate: S_TROO_RUN1, misc1: 0, misc2: 0 }, // S_TROO_RAISE5
    state_t { sprite: SPR_SARG, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SARG_STND2, misc1: 0, misc2: 0 }, // S_SARG_STND
    state_t { sprite: SPR_SARG, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SARG_STND, misc1: 0, misc2: 0 }, // S_SARG_STND2
    state_t { sprite: SPR_SARG, frame: 0, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN2, misc1: 0, misc2: 0 }, // S_SARG_RUN1
    state_t { sprite: SPR_SARG, frame: 0, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN3, misc1: 0, misc2: 0 }, // S_SARG_RUN2
    state_t { sprite: SPR_SARG, frame: 1, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN4, misc1: 0, misc2: 0 }, // S_SARG_RUN3
    state_t { sprite: SPR_SARG, frame: 1, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN5, misc1: 0, misc2: 0 }, // S_SARG_RUN4
    state_t { sprite: SPR_SARG, frame: 2, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN6, misc1: 0, misc2: 0 }, // S_SARG_RUN5
    state_t { sprite: SPR_SARG, frame: 2, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN7, misc1: 0, misc2: 0 }, // S_SARG_RUN6
    state_t { sprite: SPR_SARG, frame: 3, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN8, misc1: 0, misc2: 0 }, // S_SARG_RUN7
    state_t { sprite: SPR_SARG, frame: 3, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SARG_RUN1, misc1: 0, misc2: 0 }, // S_SARG_RUN8
    state_t { sprite: SPR_SARG, frame: 4, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SARG_ATK2, misc1: 0, misc2: 0 }, // S_SARG_ATK1
    state_t { sprite: SPR_SARG, frame: 5, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SARG_ATK3, misc1: 0, misc2: 0 }, // S_SARG_ATK2
    state_t { sprite: SPR_SARG, frame: 6, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_SargAttack), nextstate: S_SARG_RUN1, misc1: 0, misc2: 0 }, // S_SARG_ATK3
    state_t { sprite: SPR_SARG, frame: 7, tics: 2, action: actionf_t::None, nextstate: S_SARG_PAIN2, misc1: 0, misc2: 0 }, // S_SARG_PAIN
    state_t { sprite: SPR_SARG, frame: 7, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_SARG_RUN1, misc1: 0, misc2: 0 }, // S_SARG_PAIN2
    state_t { sprite: SPR_SARG, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_SARG_DIE2, misc1: 0, misc2: 0 }, // S_SARG_DIE1
    state_t { sprite: SPR_SARG, frame: 9, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_SARG_DIE3, misc1: 0, misc2: 0 }, // S_SARG_DIE2
    state_t { sprite: SPR_SARG, frame: 10, tics: 4, action: actionf_t::None, nextstate: S_SARG_DIE4, misc1: 0, misc2: 0 }, // S_SARG_DIE3
    state_t { sprite: SPR_SARG, frame: 11, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SARG_DIE5, misc1: 0, misc2: 0 }, // S_SARG_DIE4
    state_t { sprite: SPR_SARG, frame: 12, tics: 4, action: actionf_t::None, nextstate: S_SARG_DIE6, misc1: 0, misc2: 0 }, // S_SARG_DIE5
    state_t { sprite: SPR_SARG, frame: 13, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SARG_DIE6
    state_t { sprite: SPR_SARG, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_SARG_RAISE2, misc1: 0, misc2: 0 }, // S_SARG_RAISE1
    state_t { sprite: SPR_SARG, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_SARG_RAISE3, misc1: 0, misc2: 0 }, // S_SARG_RAISE2
    state_t { sprite: SPR_SARG, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_SARG_RAISE4, misc1: 0, misc2: 0 }, // S_SARG_RAISE3
    state_t { sprite: SPR_SARG, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_SARG_RAISE5, misc1: 0, misc2: 0 }, // S_SARG_RAISE4
    state_t { sprite: SPR_SARG, frame: 9, tics: 5, action: actionf_t::None, nextstate: S_SARG_RAISE6, misc1: 0, misc2: 0 }, // S_SARG_RAISE5
    state_t { sprite: SPR_SARG, frame: 8, tics: 5, action: actionf_t::None, nextstate: S_SARG_RUN1, misc1: 0, misc2: 0 }, // S_SARG_RAISE6
    state_t { sprite: SPR_HEAD, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_HEAD_STND, misc1: 0, misc2: 0 }, // S_HEAD_STND
    state_t { sprite: SPR_HEAD, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_HEAD_RUN1, misc1: 0, misc2: 0 }, // S_HEAD_RUN1
    state_t { sprite: SPR_HEAD, frame: 1, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_HEAD_ATK2, misc1: 0, misc2: 0 }, // S_HEAD_ATK1
    state_t { sprite: SPR_HEAD, frame: 2, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_HEAD_ATK3, misc1: 0, misc2: 0 }, // S_HEAD_ATK2
    state_t { sprite: SPR_HEAD, frame: 32771, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_HeadAttack), nextstate: S_HEAD_RUN1, misc1: 0, misc2: 0 }, // S_HEAD_ATK3
    state_t { sprite: SPR_HEAD, frame: 4, tics: 3, action: actionf_t::None, nextstate: S_HEAD_PAIN2, misc1: 0, misc2: 0 }, // S_HEAD_PAIN
    state_t { sprite: SPR_HEAD, frame: 4, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_HEAD_PAIN3, misc1: 0, misc2: 0 }, // S_HEAD_PAIN2
    state_t { sprite: SPR_HEAD, frame: 5, tics: 6, action: actionf_t::None, nextstate: S_HEAD_RUN1, misc1: 0, misc2: 0 }, // S_HEAD_PAIN3
    state_t { sprite: SPR_HEAD, frame: 6, tics: 8, action: actionf_t::None, nextstate: S_HEAD_DIE2, misc1: 0, misc2: 0 }, // S_HEAD_DIE1
    state_t { sprite: SPR_HEAD, frame: 7, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_HEAD_DIE3, misc1: 0, misc2: 0 }, // S_HEAD_DIE2
    state_t { sprite: SPR_HEAD, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_HEAD_DIE4, misc1: 0, misc2: 0 }, // S_HEAD_DIE3
    state_t { sprite: SPR_HEAD, frame: 9, tics: 8, action: actionf_t::None, nextstate: S_HEAD_DIE5, misc1: 0, misc2: 0 }, // S_HEAD_DIE4
    state_t { sprite: SPR_HEAD, frame: 10, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_HEAD_DIE6, misc1: 0, misc2: 0 }, // S_HEAD_DIE5
    state_t { sprite: SPR_HEAD, frame: 11, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HEAD_DIE6
    state_t { sprite: SPR_HEAD, frame: 11, tics: 8, action: actionf_t::None, nextstate: S_HEAD_RAISE2, misc1: 0, misc2: 0 }, // S_HEAD_RAISE1
    state_t { sprite: SPR_HEAD, frame: 10, tics: 8, action: actionf_t::None, nextstate: S_HEAD_RAISE3, misc1: 0, misc2: 0 }, // S_HEAD_RAISE2
    state_t { sprite: SPR_HEAD, frame: 9, tics: 8, action: actionf_t::None, nextstate: S_HEAD_RAISE4, misc1: 0, misc2: 0 }, // S_HEAD_RAISE3
    state_t { sprite: SPR_HEAD, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_HEAD_RAISE5, misc1: 0, misc2: 0 }, // S_HEAD_RAISE4
    state_t { sprite: SPR_HEAD, frame: 7, tics: 8, action: actionf_t::None, nextstate: S_HEAD_RAISE6, misc1: 0, misc2: 0 }, // S_HEAD_RAISE5
    state_t { sprite: SPR_HEAD, frame: 6, tics: 8, action: actionf_t::None, nextstate: S_HEAD_RUN1, misc1: 0, misc2: 0 }, // S_HEAD_RAISE6
    state_t { sprite: SPR_BAL7, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_BRBALL2, misc1: 0, misc2: 0 }, // S_BRBALL1
    state_t { sprite: SPR_BAL7, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_BRBALL1, misc1: 0, misc2: 0 }, // S_BRBALL2
    state_t { sprite: SPR_BAL7, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_BRBALLX2, misc1: 0, misc2: 0 }, // S_BRBALLX1
    state_t { sprite: SPR_BAL7, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_BRBALLX3, misc1: 0, misc2: 0 }, // S_BRBALLX2
    state_t { sprite: SPR_BAL7, frame: 32772, tics: 6, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BRBALLX3
    state_t { sprite: SPR_BOSS, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BOSS_STND2, misc1: 0, misc2: 0 }, // S_BOSS_STND
    state_t { sprite: SPR_BOSS, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BOSS_STND, misc1: 0, misc2: 0 }, // S_BOSS_STND2
    state_t { sprite: SPR_BOSS, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN2, misc1: 0, misc2: 0 }, // S_BOSS_RUN1
    state_t { sprite: SPR_BOSS, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN3, misc1: 0, misc2: 0 }, // S_BOSS_RUN2
    state_t { sprite: SPR_BOSS, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN4, misc1: 0, misc2: 0 }, // S_BOSS_RUN3
    state_t { sprite: SPR_BOSS, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN5, misc1: 0, misc2: 0 }, // S_BOSS_RUN4
    state_t { sprite: SPR_BOSS, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN6, misc1: 0, misc2: 0 }, // S_BOSS_RUN5
    state_t { sprite: SPR_BOSS, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN7, misc1: 0, misc2: 0 }, // S_BOSS_RUN6
    state_t { sprite: SPR_BOSS, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN8, misc1: 0, misc2: 0 }, // S_BOSS_RUN7
    state_t { sprite: SPR_BOSS, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOSS_RUN1, misc1: 0, misc2: 0 }, // S_BOSS_RUN8
    state_t { sprite: SPR_BOSS, frame: 4, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_BOSS_ATK2, misc1: 0, misc2: 0 }, // S_BOSS_ATK1
    state_t { sprite: SPR_BOSS, frame: 5, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_BOSS_ATK3, misc1: 0, misc2: 0 }, // S_BOSS_ATK2
    state_t { sprite: SPR_BOSS, frame: 6, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_BruisAttack), nextstate: S_BOSS_RUN1, misc1: 0, misc2: 0 }, // S_BOSS_ATK3
    state_t { sprite: SPR_BOSS, frame: 7, tics: 2, action: actionf_t::None, nextstate: S_BOSS_PAIN2, misc1: 0, misc2: 0 }, // S_BOSS_PAIN
    state_t { sprite: SPR_BOSS, frame: 7, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_BOSS_RUN1, misc1: 0, misc2: 0 }, // S_BOSS_PAIN2
    state_t { sprite: SPR_BOSS, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_BOSS_DIE2, misc1: 0, misc2: 0 }, // S_BOSS_DIE1
    state_t { sprite: SPR_BOSS, frame: 9, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_BOSS_DIE3, misc1: 0, misc2: 0 }, // S_BOSS_DIE2
    state_t { sprite: SPR_BOSS, frame: 10, tics: 8, action: actionf_t::None, nextstate: S_BOSS_DIE4, misc1: 0, misc2: 0 }, // S_BOSS_DIE3
    state_t { sprite: SPR_BOSS, frame: 11, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_BOSS_DIE5, misc1: 0, misc2: 0 }, // S_BOSS_DIE4
    state_t { sprite: SPR_BOSS, frame: 12, tics: 8, action: actionf_t::None, nextstate: S_BOSS_DIE6, misc1: 0, misc2: 0 }, // S_BOSS_DIE5
    state_t { sprite: SPR_BOSS, frame: 13, tics: 8, action: actionf_t::None, nextstate: S_BOSS_DIE7, misc1: 0, misc2: 0 }, // S_BOSS_DIE6
    state_t { sprite: SPR_BOSS, frame: 14, tics: -1, action: actionf_t::acp1(crate::p_enemy::A_BossDeath), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BOSS_DIE7
    state_t { sprite: SPR_BOSS, frame: 14, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RAISE2, misc1: 0, misc2: 0 }, // S_BOSS_RAISE1
    state_t { sprite: SPR_BOSS, frame: 13, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RAISE3, misc1: 0, misc2: 0 }, // S_BOSS_RAISE2
    state_t { sprite: SPR_BOSS, frame: 12, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RAISE4, misc1: 0, misc2: 0 }, // S_BOSS_RAISE3
    state_t { sprite: SPR_BOSS, frame: 11, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RAISE5, misc1: 0, misc2: 0 }, // S_BOSS_RAISE4
    state_t { sprite: SPR_BOSS, frame: 10, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RAISE6, misc1: 0, misc2: 0 }, // S_BOSS_RAISE5
    state_t { sprite: SPR_BOSS, frame: 9, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RAISE7, misc1: 0, misc2: 0 }, // S_BOSS_RAISE6
    state_t { sprite: SPR_BOSS, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_BOSS_RUN1, misc1: 0, misc2: 0 }, // S_BOSS_RAISE7
    state_t { sprite: SPR_BOS2, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BOS2_STND2, misc1: 0, misc2: 0 }, // S_BOS2_STND
    state_t { sprite: SPR_BOS2, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BOS2_STND, misc1: 0, misc2: 0 }, // S_BOS2_STND2
    state_t { sprite: SPR_BOS2, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN2, misc1: 0, misc2: 0 }, // S_BOS2_RUN1
    state_t { sprite: SPR_BOS2, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN3, misc1: 0, misc2: 0 }, // S_BOS2_RUN2
    state_t { sprite: SPR_BOS2, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN4, misc1: 0, misc2: 0 }, // S_BOS2_RUN3
    state_t { sprite: SPR_BOS2, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN5, misc1: 0, misc2: 0 }, // S_BOS2_RUN4
    state_t { sprite: SPR_BOS2, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN6, misc1: 0, misc2: 0 }, // S_BOS2_RUN5
    state_t { sprite: SPR_BOS2, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN7, misc1: 0, misc2: 0 }, // S_BOS2_RUN6
    state_t { sprite: SPR_BOS2, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN8, misc1: 0, misc2: 0 }, // S_BOS2_RUN7
    state_t { sprite: SPR_BOS2, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BOS2_RUN1, misc1: 0, misc2: 0 }, // S_BOS2_RUN8
    state_t { sprite: SPR_BOS2, frame: 4, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_BOS2_ATK2, misc1: 0, misc2: 0 }, // S_BOS2_ATK1
    state_t { sprite: SPR_BOS2, frame: 5, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_BOS2_ATK3, misc1: 0, misc2: 0 }, // S_BOS2_ATK2
    state_t { sprite: SPR_BOS2, frame: 6, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_BruisAttack), nextstate: S_BOS2_RUN1, misc1: 0, misc2: 0 }, // S_BOS2_ATK3
    state_t { sprite: SPR_BOS2, frame: 7, tics: 2, action: actionf_t::None, nextstate: S_BOS2_PAIN2, misc1: 0, misc2: 0 }, // S_BOS2_PAIN
    state_t { sprite: SPR_BOS2, frame: 7, tics: 2, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_BOS2_RUN1, misc1: 0, misc2: 0 }, // S_BOS2_PAIN2
    state_t { sprite: SPR_BOS2, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_BOS2_DIE2, misc1: 0, misc2: 0 }, // S_BOS2_DIE1
    state_t { sprite: SPR_BOS2, frame: 9, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_BOS2_DIE3, misc1: 0, misc2: 0 }, // S_BOS2_DIE2
    state_t { sprite: SPR_BOS2, frame: 10, tics: 8, action: actionf_t::None, nextstate: S_BOS2_DIE4, misc1: 0, misc2: 0 }, // S_BOS2_DIE3
    state_t { sprite: SPR_BOS2, frame: 11, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_BOS2_DIE5, misc1: 0, misc2: 0 }, // S_BOS2_DIE4
    state_t { sprite: SPR_BOS2, frame: 12, tics: 8, action: actionf_t::None, nextstate: S_BOS2_DIE6, misc1: 0, misc2: 0 }, // S_BOS2_DIE5
    state_t { sprite: SPR_BOS2, frame: 13, tics: 8, action: actionf_t::None, nextstate: S_BOS2_DIE7, misc1: 0, misc2: 0 }, // S_BOS2_DIE6
    state_t { sprite: SPR_BOS2, frame: 14, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BOS2_DIE7
    state_t { sprite: SPR_BOS2, frame: 14, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RAISE2, misc1: 0, misc2: 0 }, // S_BOS2_RAISE1
    state_t { sprite: SPR_BOS2, frame: 13, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RAISE3, misc1: 0, misc2: 0 }, // S_BOS2_RAISE2
    state_t { sprite: SPR_BOS2, frame: 12, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RAISE4, misc1: 0, misc2: 0 }, // S_BOS2_RAISE3
    state_t { sprite: SPR_BOS2, frame: 11, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RAISE5, misc1: 0, misc2: 0 }, // S_BOS2_RAISE4
    state_t { sprite: SPR_BOS2, frame: 10, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RAISE6, misc1: 0, misc2: 0 }, // S_BOS2_RAISE5
    state_t { sprite: SPR_BOS2, frame: 9, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RAISE7, misc1: 0, misc2: 0 }, // S_BOS2_RAISE6
    state_t { sprite: SPR_BOS2, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_BOS2_RUN1, misc1: 0, misc2: 0 }, // S_BOS2_RAISE7
    state_t { sprite: SPR_SKUL, frame: 32768, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SKULL_STND2, misc1: 0, misc2: 0 }, // S_SKULL_STND
    state_t { sprite: SPR_SKUL, frame: 32769, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SKULL_STND, misc1: 0, misc2: 0 }, // S_SKULL_STND2
    state_t { sprite: SPR_SKUL, frame: 32768, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKULL_RUN2, misc1: 0, misc2: 0 }, // S_SKULL_RUN1
    state_t { sprite: SPR_SKUL, frame: 32769, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SKULL_RUN1, misc1: 0, misc2: 0 }, // S_SKULL_RUN2
    state_t { sprite: SPR_SKUL, frame: 32770, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SKULL_ATK2, misc1: 0, misc2: 0 }, // S_SKULL_ATK1
    state_t { sprite: SPR_SKUL, frame: 32771, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_SkullAttack), nextstate: S_SKULL_ATK3, misc1: 0, misc2: 0 }, // S_SKULL_ATK2
    state_t { sprite: SPR_SKUL, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_SKULL_ATK4, misc1: 0, misc2: 0 }, // S_SKULL_ATK3
    state_t { sprite: SPR_SKUL, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_SKULL_ATK3, misc1: 0, misc2: 0 }, // S_SKULL_ATK4
    state_t { sprite: SPR_SKUL, frame: 32772, tics: 3, action: actionf_t::None, nextstate: S_SKULL_PAIN2, misc1: 0, misc2: 0 }, // S_SKULL_PAIN
    state_t { sprite: SPR_SKUL, frame: 32772, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_SKULL_RUN1, misc1: 0, misc2: 0 }, // S_SKULL_PAIN2
    state_t { sprite: SPR_SKUL, frame: 32773, tics: 6, action: actionf_t::None, nextstate: S_SKULL_DIE2, misc1: 0, misc2: 0 }, // S_SKULL_DIE1
    state_t { sprite: SPR_SKUL, frame: 32774, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_SKULL_DIE3, misc1: 0, misc2: 0 }, // S_SKULL_DIE2
    state_t { sprite: SPR_SKUL, frame: 32775, tics: 6, action: actionf_t::None, nextstate: S_SKULL_DIE4, misc1: 0, misc2: 0 }, // S_SKULL_DIE3
    state_t { sprite: SPR_SKUL, frame: 32776, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SKULL_DIE5, misc1: 0, misc2: 0 }, // S_SKULL_DIE4
    state_t { sprite: SPR_SKUL, frame: 9, tics: 6, action: actionf_t::None, nextstate: S_SKULL_DIE6, misc1: 0, misc2: 0 }, // S_SKULL_DIE5
    state_t { sprite: SPR_SKUL, frame: 10, tics: 6, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SKULL_DIE6
    state_t { sprite: SPR_SPID, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SPID_STND2, misc1: 0, misc2: 0 }, // S_SPID_STND
    state_t { sprite: SPR_SPID, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SPID_STND, misc1: 0, misc2: 0 }, // S_SPID_STND2
    state_t { sprite: SPR_SPID, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Metal), nextstate: S_SPID_RUN2, misc1: 0, misc2: 0 }, // S_SPID_RUN1
    state_t { sprite: SPR_SPID, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN3, misc1: 0, misc2: 0 }, // S_SPID_RUN2
    state_t { sprite: SPR_SPID, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN4, misc1: 0, misc2: 0 }, // S_SPID_RUN3
    state_t { sprite: SPR_SPID, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN5, misc1: 0, misc2: 0 }, // S_SPID_RUN4
    state_t { sprite: SPR_SPID, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Metal), nextstate: S_SPID_RUN6, misc1: 0, misc2: 0 }, // S_SPID_RUN5
    state_t { sprite: SPR_SPID, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN7, misc1: 0, misc2: 0 }, // S_SPID_RUN6
    state_t { sprite: SPR_SPID, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN8, misc1: 0, misc2: 0 }, // S_SPID_RUN7
    state_t { sprite: SPR_SPID, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN9, misc1: 0, misc2: 0 }, // S_SPID_RUN8
    state_t { sprite: SPR_SPID, frame: 4, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Metal), nextstate: S_SPID_RUN10, misc1: 0, misc2: 0 }, // S_SPID_RUN9
    state_t { sprite: SPR_SPID, frame: 4, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN11, misc1: 0, misc2: 0 }, // S_SPID_RUN10
    state_t { sprite: SPR_SPID, frame: 5, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN12, misc1: 0, misc2: 0 }, // S_SPID_RUN11
    state_t { sprite: SPR_SPID, frame: 5, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SPID_RUN1, misc1: 0, misc2: 0 }, // S_SPID_RUN12
    state_t { sprite: SPR_SPID, frame: 32768, tics: 20, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SPID_ATK2, misc1: 0, misc2: 0 }, // S_SPID_ATK1
    state_t { sprite: SPR_SPID, frame: 32774, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_SPosAttack), nextstate: S_SPID_ATK3, misc1: 0, misc2: 0 }, // S_SPID_ATK2
    state_t { sprite: SPR_SPID, frame: 32775, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_SPosAttack), nextstate: S_SPID_ATK4, misc1: 0, misc2: 0 }, // S_SPID_ATK3
    state_t { sprite: SPR_SPID, frame: 32775, tics: 1, action: actionf_t::acp1(crate::p_enemy::A_SpidRefire), nextstate: S_SPID_ATK2, misc1: 0, misc2: 0 }, // S_SPID_ATK4
    state_t { sprite: SPR_SPID, frame: 8, tics: 3, action: actionf_t::None, nextstate: S_SPID_PAIN2, misc1: 0, misc2: 0 }, // S_SPID_PAIN
    state_t { sprite: SPR_SPID, frame: 8, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_SPID_RUN1, misc1: 0, misc2: 0 }, // S_SPID_PAIN2
    state_t { sprite: SPR_SPID, frame: 9, tics: 20, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_SPID_DIE2, misc1: 0, misc2: 0 }, // S_SPID_DIE1
    state_t { sprite: SPR_SPID, frame: 10, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SPID_DIE3, misc1: 0, misc2: 0 }, // S_SPID_DIE2
    state_t { sprite: SPR_SPID, frame: 11, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE4, misc1: 0, misc2: 0 }, // S_SPID_DIE3
    state_t { sprite: SPR_SPID, frame: 12, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE5, misc1: 0, misc2: 0 }, // S_SPID_DIE4
    state_t { sprite: SPR_SPID, frame: 13, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE6, misc1: 0, misc2: 0 }, // S_SPID_DIE5
    state_t { sprite: SPR_SPID, frame: 14, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE7, misc1: 0, misc2: 0 }, // S_SPID_DIE6
    state_t { sprite: SPR_SPID, frame: 15, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE8, misc1: 0, misc2: 0 }, // S_SPID_DIE7
    state_t { sprite: SPR_SPID, frame: 16, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE9, misc1: 0, misc2: 0 }, // S_SPID_DIE8
    state_t { sprite: SPR_SPID, frame: 17, tics: 10, action: actionf_t::None, nextstate: S_SPID_DIE10, misc1: 0, misc2: 0 }, // S_SPID_DIE9
    state_t { sprite: SPR_SPID, frame: 18, tics: 30, action: actionf_t::None, nextstate: S_SPID_DIE11, misc1: 0, misc2: 0 }, // S_SPID_DIE10
    state_t { sprite: SPR_SPID, frame: 18, tics: -1, action: actionf_t::acp1(crate::p_enemy::A_BossDeath), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SPID_DIE11
    state_t { sprite: SPR_BSPI, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BSPI_STND2, misc1: 0, misc2: 0 }, // S_BSPI_STND
    state_t { sprite: SPR_BSPI, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BSPI_STND, misc1: 0, misc2: 0 }, // S_BSPI_STND2
    state_t { sprite: SPR_BSPI, frame: 0, tics: 20, action: actionf_t::None, nextstate: S_BSPI_RUN1, misc1: 0, misc2: 0 }, // S_BSPI_SIGHT
    state_t { sprite: SPR_BSPI, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_BabyMetal), nextstate: S_BSPI_RUN2, misc1: 0, misc2: 0 }, // S_BSPI_RUN1
    state_t { sprite: SPR_BSPI, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN3, misc1: 0, misc2: 0 }, // S_BSPI_RUN2
    state_t { sprite: SPR_BSPI, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN4, misc1: 0, misc2: 0 }, // S_BSPI_RUN3
    state_t { sprite: SPR_BSPI, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN5, misc1: 0, misc2: 0 }, // S_BSPI_RUN4
    state_t { sprite: SPR_BSPI, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN6, misc1: 0, misc2: 0 }, // S_BSPI_RUN5
    state_t { sprite: SPR_BSPI, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN7, misc1: 0, misc2: 0 }, // S_BSPI_RUN6
    state_t { sprite: SPR_BSPI, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_BabyMetal), nextstate: S_BSPI_RUN8, misc1: 0, misc2: 0 }, // S_BSPI_RUN7
    state_t { sprite: SPR_BSPI, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN9, misc1: 0, misc2: 0 }, // S_BSPI_RUN8
    state_t { sprite: SPR_BSPI, frame: 4, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN10, misc1: 0, misc2: 0 }, // S_BSPI_RUN9
    state_t { sprite: SPR_BSPI, frame: 4, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN11, misc1: 0, misc2: 0 }, // S_BSPI_RUN10
    state_t { sprite: SPR_BSPI, frame: 5, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN12, misc1: 0, misc2: 0 }, // S_BSPI_RUN11
    state_t { sprite: SPR_BSPI, frame: 5, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_BSPI_RUN1, misc1: 0, misc2: 0 }, // S_BSPI_RUN12
    state_t { sprite: SPR_BSPI, frame: 32768, tics: 20, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_BSPI_ATK2, misc1: 0, misc2: 0 }, // S_BSPI_ATK1
    state_t { sprite: SPR_BSPI, frame: 32774, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_BspiAttack), nextstate: S_BSPI_ATK3, misc1: 0, misc2: 0 }, // S_BSPI_ATK2
    state_t { sprite: SPR_BSPI, frame: 32775, tics: 4, action: actionf_t::None, nextstate: S_BSPI_ATK4, misc1: 0, misc2: 0 }, // S_BSPI_ATK3
    state_t { sprite: SPR_BSPI, frame: 32775, tics: 1, action: actionf_t::acp1(crate::p_enemy::A_SpidRefire), nextstate: S_BSPI_ATK2, misc1: 0, misc2: 0 }, // S_BSPI_ATK4
    state_t { sprite: SPR_BSPI, frame: 8, tics: 3, action: actionf_t::None, nextstate: S_BSPI_PAIN2, misc1: 0, misc2: 0 }, // S_BSPI_PAIN
    state_t { sprite: SPR_BSPI, frame: 8, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_BSPI_RUN1, misc1: 0, misc2: 0 }, // S_BSPI_PAIN2
    state_t { sprite: SPR_BSPI, frame: 9, tics: 20, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_BSPI_DIE2, misc1: 0, misc2: 0 }, // S_BSPI_DIE1
    state_t { sprite: SPR_BSPI, frame: 10, tics: 7, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_BSPI_DIE3, misc1: 0, misc2: 0 }, // S_BSPI_DIE2
    state_t { sprite: SPR_BSPI, frame: 11, tics: 7, action: actionf_t::None, nextstate: S_BSPI_DIE4, misc1: 0, misc2: 0 }, // S_BSPI_DIE3
    state_t { sprite: SPR_BSPI, frame: 12, tics: 7, action: actionf_t::None, nextstate: S_BSPI_DIE5, misc1: 0, misc2: 0 }, // S_BSPI_DIE4
    state_t { sprite: SPR_BSPI, frame: 13, tics: 7, action: actionf_t::None, nextstate: S_BSPI_DIE6, misc1: 0, misc2: 0 }, // S_BSPI_DIE5
    state_t { sprite: SPR_BSPI, frame: 14, tics: 7, action: actionf_t::None, nextstate: S_BSPI_DIE7, misc1: 0, misc2: 0 }, // S_BSPI_DIE6
    state_t { sprite: SPR_BSPI, frame: 15, tics: -1, action: actionf_t::acp1(crate::p_enemy::A_BossDeath), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BSPI_DIE7
    state_t { sprite: SPR_BSPI, frame: 15, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RAISE2, misc1: 0, misc2: 0 }, // S_BSPI_RAISE1
    state_t { sprite: SPR_BSPI, frame: 14, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RAISE3, misc1: 0, misc2: 0 }, // S_BSPI_RAISE2
    state_t { sprite: SPR_BSPI, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RAISE4, misc1: 0, misc2: 0 }, // S_BSPI_RAISE3
    state_t { sprite: SPR_BSPI, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RAISE5, misc1: 0, misc2: 0 }, // S_BSPI_RAISE4
    state_t { sprite: SPR_BSPI, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RAISE6, misc1: 0, misc2: 0 }, // S_BSPI_RAISE5
    state_t { sprite: SPR_BSPI, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RAISE7, misc1: 0, misc2: 0 }, // S_BSPI_RAISE6
    state_t { sprite: SPR_BSPI, frame: 9, tics: 5, action: actionf_t::None, nextstate: S_BSPI_RUN1, misc1: 0, misc2: 0 }, // S_BSPI_RAISE7
    state_t { sprite: SPR_APLS, frame: 32768, tics: 5, action: actionf_t::None, nextstate: S_ARACH_PLAZ2, misc1: 0, misc2: 0 }, // S_ARACH_PLAZ
    state_t { sprite: SPR_APLS, frame: 32769, tics: 5, action: actionf_t::None, nextstate: S_ARACH_PLAZ, misc1: 0, misc2: 0 }, // S_ARACH_PLAZ2
    state_t { sprite: SPR_APBX, frame: 32768, tics: 5, action: actionf_t::None, nextstate: S_ARACH_PLEX2, misc1: 0, misc2: 0 }, // S_ARACH_PLEX
    state_t { sprite: SPR_APBX, frame: 32769, tics: 5, action: actionf_t::None, nextstate: S_ARACH_PLEX3, misc1: 0, misc2: 0 }, // S_ARACH_PLEX2
    state_t { sprite: SPR_APBX, frame: 32770, tics: 5, action: actionf_t::None, nextstate: S_ARACH_PLEX4, misc1: 0, misc2: 0 }, // S_ARACH_PLEX3
    state_t { sprite: SPR_APBX, frame: 32771, tics: 5, action: actionf_t::None, nextstate: S_ARACH_PLEX5, misc1: 0, misc2: 0 }, // S_ARACH_PLEX4
    state_t { sprite: SPR_APBX, frame: 32772, tics: 5, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_ARACH_PLEX5
    state_t { sprite: SPR_CYBR, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_CYBER_STND2, misc1: 0, misc2: 0 }, // S_CYBER_STND
    state_t { sprite: SPR_CYBR, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_CYBER_STND, misc1: 0, misc2: 0 }, // S_CYBER_STND2
    state_t { sprite: SPR_CYBR, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Hoof), nextstate: S_CYBER_RUN2, misc1: 0, misc2: 0 }, // S_CYBER_RUN1
    state_t { sprite: SPR_CYBR, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CYBER_RUN3, misc1: 0, misc2: 0 }, // S_CYBER_RUN2
    state_t { sprite: SPR_CYBR, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CYBER_RUN4, misc1: 0, misc2: 0 }, // S_CYBER_RUN3
    state_t { sprite: SPR_CYBR, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CYBER_RUN5, misc1: 0, misc2: 0 }, // S_CYBER_RUN4
    state_t { sprite: SPR_CYBR, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CYBER_RUN6, misc1: 0, misc2: 0 }, // S_CYBER_RUN5
    state_t { sprite: SPR_CYBR, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CYBER_RUN7, misc1: 0, misc2: 0 }, // S_CYBER_RUN6
    state_t { sprite: SPR_CYBR, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Metal), nextstate: S_CYBER_RUN8, misc1: 0, misc2: 0 }, // S_CYBER_RUN7
    state_t { sprite: SPR_CYBR, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_CYBER_RUN1, misc1: 0, misc2: 0 }, // S_CYBER_RUN8
    state_t { sprite: SPR_CYBR, frame: 4, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_CYBER_ATK2, misc1: 0, misc2: 0 }, // S_CYBER_ATK1
    state_t { sprite: SPR_CYBR, frame: 5, tics: 12, action: actionf_t::acp1(crate::p_enemy::A_CyberAttack), nextstate: S_CYBER_ATK3, misc1: 0, misc2: 0 }, // S_CYBER_ATK2
    state_t { sprite: SPR_CYBR, frame: 4, tics: 12, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_CYBER_ATK4, misc1: 0, misc2: 0 }, // S_CYBER_ATK3
    state_t { sprite: SPR_CYBR, frame: 5, tics: 12, action: actionf_t::acp1(crate::p_enemy::A_CyberAttack), nextstate: S_CYBER_ATK5, misc1: 0, misc2: 0 }, // S_CYBER_ATK4
    state_t { sprite: SPR_CYBR, frame: 4, tics: 12, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_CYBER_ATK6, misc1: 0, misc2: 0 }, // S_CYBER_ATK5
    state_t { sprite: SPR_CYBR, frame: 5, tics: 12, action: actionf_t::acp1(crate::p_enemy::A_CyberAttack), nextstate: S_CYBER_RUN1, misc1: 0, misc2: 0 }, // S_CYBER_ATK6
    state_t { sprite: SPR_CYBR, frame: 6, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_CYBER_RUN1, misc1: 0, misc2: 0 }, // S_CYBER_PAIN
    state_t { sprite: SPR_CYBR, frame: 7, tics: 10, action: actionf_t::None, nextstate: S_CYBER_DIE2, misc1: 0, misc2: 0 }, // S_CYBER_DIE1
    state_t { sprite: SPR_CYBR, frame: 8, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_CYBER_DIE3, misc1: 0, misc2: 0 }, // S_CYBER_DIE2
    state_t { sprite: SPR_CYBR, frame: 9, tics: 10, action: actionf_t::None, nextstate: S_CYBER_DIE4, misc1: 0, misc2: 0 }, // S_CYBER_DIE3
    state_t { sprite: SPR_CYBR, frame: 10, tics: 10, action: actionf_t::None, nextstate: S_CYBER_DIE5, misc1: 0, misc2: 0 }, // S_CYBER_DIE4
    state_t { sprite: SPR_CYBR, frame: 11, tics: 10, action: actionf_t::None, nextstate: S_CYBER_DIE6, misc1: 0, misc2: 0 }, // S_CYBER_DIE5
    state_t { sprite: SPR_CYBR, frame: 12, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_CYBER_DIE7, misc1: 0, misc2: 0 }, // S_CYBER_DIE6
    state_t { sprite: SPR_CYBR, frame: 13, tics: 10, action: actionf_t::None, nextstate: S_CYBER_DIE8, misc1: 0, misc2: 0 }, // S_CYBER_DIE7
    state_t { sprite: SPR_CYBR, frame: 14, tics: 10, action: actionf_t::None, nextstate: S_CYBER_DIE9, misc1: 0, misc2: 0 }, // S_CYBER_DIE8
    state_t { sprite: SPR_CYBR, frame: 15, tics: 30, action: actionf_t::None, nextstate: S_CYBER_DIE10, misc1: 0, misc2: 0 }, // S_CYBER_DIE9
    state_t { sprite: SPR_CYBR, frame: 15, tics: -1, action: actionf_t::acp1(crate::p_enemy::A_BossDeath), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CYBER_DIE10
    state_t { sprite: SPR_PAIN, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_PAIN_STND, misc1: 0, misc2: 0 }, // S_PAIN_STND
    state_t { sprite: SPR_PAIN, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_PAIN_RUN2, misc1: 0, misc2: 0 }, // S_PAIN_RUN1
    state_t { sprite: SPR_PAIN, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_PAIN_RUN3, misc1: 0, misc2: 0 }, // S_PAIN_RUN2
    state_t { sprite: SPR_PAIN, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_PAIN_RUN4, misc1: 0, misc2: 0 }, // S_PAIN_RUN3
    state_t { sprite: SPR_PAIN, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_PAIN_RUN5, misc1: 0, misc2: 0 }, // S_PAIN_RUN4
    state_t { sprite: SPR_PAIN, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_PAIN_RUN6, misc1: 0, misc2: 0 }, // S_PAIN_RUN5
    state_t { sprite: SPR_PAIN, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_PAIN_RUN1, misc1: 0, misc2: 0 }, // S_PAIN_RUN6
    state_t { sprite: SPR_PAIN, frame: 3, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_PAIN_ATK2, misc1: 0, misc2: 0 }, // S_PAIN_ATK1
    state_t { sprite: SPR_PAIN, frame: 4, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_PAIN_ATK3, misc1: 0, misc2: 0 }, // S_PAIN_ATK2
    state_t { sprite: SPR_PAIN, frame: 32773, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_PAIN_ATK4, misc1: 0, misc2: 0 }, // S_PAIN_ATK3
    state_t { sprite: SPR_PAIN, frame: 32773, tics: 0, action: actionf_t::acp1(crate::p_enemy::A_PainAttack), nextstate: S_PAIN_RUN1, misc1: 0, misc2: 0 }, // S_PAIN_ATK4
    state_t { sprite: SPR_PAIN, frame: 6, tics: 6, action: actionf_t::None, nextstate: S_PAIN_PAIN2, misc1: 0, misc2: 0 }, // S_PAIN_PAIN
    state_t { sprite: SPR_PAIN, frame: 6, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_PAIN_RUN1, misc1: 0, misc2: 0 }, // S_PAIN_PAIN2
    state_t { sprite: SPR_PAIN, frame: 32775, tics: 8, action: actionf_t::None, nextstate: S_PAIN_DIE2, misc1: 0, misc2: 0 }, // S_PAIN_DIE1
    state_t { sprite: SPR_PAIN, frame: 32776, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_PAIN_DIE3, misc1: 0, misc2: 0 }, // S_PAIN_DIE2
    state_t { sprite: SPR_PAIN, frame: 32777, tics: 8, action: actionf_t::None, nextstate: S_PAIN_DIE4, misc1: 0, misc2: 0 }, // S_PAIN_DIE3
    state_t { sprite: SPR_PAIN, frame: 32778, tics: 8, action: actionf_t::None, nextstate: S_PAIN_DIE5, misc1: 0, misc2: 0 }, // S_PAIN_DIE4
    state_t { sprite: SPR_PAIN, frame: 32779, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_PainDie), nextstate: S_PAIN_DIE6, misc1: 0, misc2: 0 }, // S_PAIN_DIE5
    state_t { sprite: SPR_PAIN, frame: 32780, tics: 8, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PAIN_DIE6
    state_t { sprite: SPR_PAIN, frame: 12, tics: 8, action: actionf_t::None, nextstate: S_PAIN_RAISE2, misc1: 0, misc2: 0 }, // S_PAIN_RAISE1
    state_t { sprite: SPR_PAIN, frame: 11, tics: 8, action: actionf_t::None, nextstate: S_PAIN_RAISE3, misc1: 0, misc2: 0 }, // S_PAIN_RAISE2
    state_t { sprite: SPR_PAIN, frame: 10, tics: 8, action: actionf_t::None, nextstate: S_PAIN_RAISE4, misc1: 0, misc2: 0 }, // S_PAIN_RAISE3
    state_t { sprite: SPR_PAIN, frame: 9, tics: 8, action: actionf_t::None, nextstate: S_PAIN_RAISE5, misc1: 0, misc2: 0 }, // S_PAIN_RAISE4
    state_t { sprite: SPR_PAIN, frame: 8, tics: 8, action: actionf_t::None, nextstate: S_PAIN_RAISE6, misc1: 0, misc2: 0 }, // S_PAIN_RAISE5
    state_t { sprite: SPR_PAIN, frame: 7, tics: 8, action: actionf_t::None, nextstate: S_PAIN_RUN1, misc1: 0, misc2: 0 }, // S_PAIN_RAISE6
    state_t { sprite: SPR_SSWV, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SSWV_STND2, misc1: 0, misc2: 0 }, // S_SSWV_STND
    state_t { sprite: SPR_SSWV, frame: 1, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_SSWV_STND, misc1: 0, misc2: 0 }, // S_SSWV_STND2
    state_t { sprite: SPR_SSWV, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN2, misc1: 0, misc2: 0 }, // S_SSWV_RUN1
    state_t { sprite: SPR_SSWV, frame: 0, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN3, misc1: 0, misc2: 0 }, // S_SSWV_RUN2
    state_t { sprite: SPR_SSWV, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN4, misc1: 0, misc2: 0 }, // S_SSWV_RUN3
    state_t { sprite: SPR_SSWV, frame: 1, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN5, misc1: 0, misc2: 0 }, // S_SSWV_RUN4
    state_t { sprite: SPR_SSWV, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN6, misc1: 0, misc2: 0 }, // S_SSWV_RUN5
    state_t { sprite: SPR_SSWV, frame: 2, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN7, misc1: 0, misc2: 0 }, // S_SSWV_RUN6
    state_t { sprite: SPR_SSWV, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN8, misc1: 0, misc2: 0 }, // S_SSWV_RUN7
    state_t { sprite: SPR_SSWV, frame: 3, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Chase), nextstate: S_SSWV_RUN1, misc1: 0, misc2: 0 }, // S_SSWV_RUN8
    state_t { sprite: SPR_SSWV, frame: 4, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SSWV_ATK2, misc1: 0, misc2: 0 }, // S_SSWV_ATK1
    state_t { sprite: SPR_SSWV, frame: 5, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SSWV_ATK3, misc1: 0, misc2: 0 }, // S_SSWV_ATK2
    state_t { sprite: SPR_SSWV, frame: 32774, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_CPosAttack), nextstate: S_SSWV_ATK4, misc1: 0, misc2: 0 }, // S_SSWV_ATK3
    state_t { sprite: SPR_SSWV, frame: 5, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_FaceTarget), nextstate: S_SSWV_ATK5, misc1: 0, misc2: 0 }, // S_SSWV_ATK4
    state_t { sprite: SPR_SSWV, frame: 32774, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_CPosAttack), nextstate: S_SSWV_ATK6, misc1: 0, misc2: 0 }, // S_SSWV_ATK5
    state_t { sprite: SPR_SSWV, frame: 5, tics: 1, action: actionf_t::acp1(crate::p_enemy::A_CPosRefire), nextstate: S_SSWV_ATK2, misc1: 0, misc2: 0 }, // S_SSWV_ATK6
    state_t { sprite: SPR_SSWV, frame: 7, tics: 3, action: actionf_t::None, nextstate: S_SSWV_PAIN2, misc1: 0, misc2: 0 }, // S_SSWV_PAIN
    state_t { sprite: SPR_SSWV, frame: 7, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_SSWV_RUN1, misc1: 0, misc2: 0 }, // S_SSWV_PAIN2
    state_t { sprite: SPR_SSWV, frame: 8, tics: 5, action: actionf_t::None, nextstate: S_SSWV_DIE2, misc1: 0, misc2: 0 }, // S_SSWV_DIE1
    state_t { sprite: SPR_SSWV, frame: 9, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_SSWV_DIE3, misc1: 0, misc2: 0 }, // S_SSWV_DIE2
    state_t { sprite: SPR_SSWV, frame: 10, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SSWV_DIE4, misc1: 0, misc2: 0 }, // S_SSWV_DIE3
    state_t { sprite: SPR_SSWV, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_SSWV_DIE5, misc1: 0, misc2: 0 }, // S_SSWV_DIE4
    state_t { sprite: SPR_SSWV, frame: 12, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SSWV_DIE5
    state_t { sprite: SPR_SSWV, frame: 13, tics: 5, action: actionf_t::None, nextstate: S_SSWV_XDIE2, misc1: 0, misc2: 0 }, // S_SSWV_XDIE1
    state_t { sprite: SPR_SSWV, frame: 14, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_XScream), nextstate: S_SSWV_XDIE3, misc1: 0, misc2: 0 }, // S_SSWV_XDIE2
    state_t { sprite: SPR_SSWV, frame: 15, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Fall), nextstate: S_SSWV_XDIE4, misc1: 0, misc2: 0 }, // S_SSWV_XDIE3
    state_t { sprite: SPR_SSWV, frame: 16, tics: 5, action: actionf_t::None, nextstate: S_SSWV_XDIE5, misc1: 0, misc2: 0 }, // S_SSWV_XDIE4
    state_t { sprite: SPR_SSWV, frame: 17, tics: 5, action: actionf_t::None, nextstate: S_SSWV_XDIE6, misc1: 0, misc2: 0 }, // S_SSWV_XDIE5
    state_t { sprite: SPR_SSWV, frame: 18, tics: 5, action: actionf_t::None, nextstate: S_SSWV_XDIE7, misc1: 0, misc2: 0 }, // S_SSWV_XDIE6
    state_t { sprite: SPR_SSWV, frame: 19, tics: 5, action: actionf_t::None, nextstate: S_SSWV_XDIE8, misc1: 0, misc2: 0 }, // S_SSWV_XDIE7
    state_t { sprite: SPR_SSWV, frame: 20, tics: 5, action: actionf_t::None, nextstate: S_SSWV_XDIE9, misc1: 0, misc2: 0 }, // S_SSWV_XDIE8
    state_t { sprite: SPR_SSWV, frame: 21, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SSWV_XDIE9
    state_t { sprite: SPR_SSWV, frame: 12, tics: 5, action: actionf_t::None, nextstate: S_SSWV_RAISE2, misc1: 0, misc2: 0 }, // S_SSWV_RAISE1
    state_t { sprite: SPR_SSWV, frame: 11, tics: 5, action: actionf_t::None, nextstate: S_SSWV_RAISE3, misc1: 0, misc2: 0 }, // S_SSWV_RAISE2
    state_t { sprite: SPR_SSWV, frame: 10, tics: 5, action: actionf_t::None, nextstate: S_SSWV_RAISE4, misc1: 0, misc2: 0 }, // S_SSWV_RAISE3
    state_t { sprite: SPR_SSWV, frame: 9, tics: 5, action: actionf_t::None, nextstate: S_SSWV_RAISE5, misc1: 0, misc2: 0 }, // S_SSWV_RAISE4
    state_t { sprite: SPR_SSWV, frame: 8, tics: 5, action: actionf_t::None, nextstate: S_SSWV_RUN1, misc1: 0, misc2: 0 }, // S_SSWV_RAISE5
    state_t { sprite: SPR_KEEN, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_KEENSTND, misc1: 0, misc2: 0 }, // S_KEENSTND
    state_t { sprite: SPR_KEEN, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN2, misc1: 0, misc2: 0 }, // S_COMMKEEN
    state_t { sprite: SPR_KEEN, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN3, misc1: 0, misc2: 0 }, // S_COMMKEEN2
    state_t { sprite: SPR_KEEN, frame: 2, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_COMMKEEN4, misc1: 0, misc2: 0 }, // S_COMMKEEN3
    state_t { sprite: SPR_KEEN, frame: 3, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN5, misc1: 0, misc2: 0 }, // S_COMMKEEN4
    state_t { sprite: SPR_KEEN, frame: 4, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN6, misc1: 0, misc2: 0 }, // S_COMMKEEN5
    state_t { sprite: SPR_KEEN, frame: 5, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN7, misc1: 0, misc2: 0 }, // S_COMMKEEN6
    state_t { sprite: SPR_KEEN, frame: 6, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN8, misc1: 0, misc2: 0 }, // S_COMMKEEN7
    state_t { sprite: SPR_KEEN, frame: 7, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN9, misc1: 0, misc2: 0 }, // S_COMMKEEN8
    state_t { sprite: SPR_KEEN, frame: 8, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN10, misc1: 0, misc2: 0 }, // S_COMMKEEN9
    state_t { sprite: SPR_KEEN, frame: 9, tics: 6, action: actionf_t::None, nextstate: S_COMMKEEN11, misc1: 0, misc2: 0 }, // S_COMMKEEN10
    state_t { sprite: SPR_KEEN, frame: 10, tics: 6, action: actionf_t::acp1(crate::p_enemy::A_KeenDie), nextstate: S_COMMKEEN12, misc1: 0, misc2: 0 }, // S_COMMKEEN11
    state_t { sprite: SPR_KEEN, frame: 11, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_COMMKEEN12
    state_t { sprite: SPR_KEEN, frame: 12, tics: 4, action: actionf_t::None, nextstate: S_KEENPAIN2, misc1: 0, misc2: 0 }, // S_KEENPAIN
    state_t { sprite: SPR_KEEN, frame: 12, tics: 8, action: actionf_t::acp1(crate::p_enemy::A_Pain), nextstate: S_KEENSTND, misc1: 0, misc2: 0 }, // S_KEENPAIN2
    state_t { sprite: SPR_BBRN, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BRAIN
    state_t { sprite: SPR_BBRN, frame: 1, tics: 36, action: actionf_t::acp1(crate::p_enemy::A_BrainPain), nextstate: S_BRAIN, misc1: 0, misc2: 0 }, // S_BRAIN_PAIN
    state_t { sprite: SPR_BBRN, frame: 0, tics: 100, action: actionf_t::acp1(crate::p_enemy::A_BrainScream), nextstate: S_BRAIN_DIE2, misc1: 0, misc2: 0 }, // S_BRAIN_DIE1
    state_t { sprite: SPR_BBRN, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_BRAIN_DIE3, misc1: 0, misc2: 0 }, // S_BRAIN_DIE2
    state_t { sprite: SPR_BBRN, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_BRAIN_DIE4, misc1: 0, misc2: 0 }, // S_BRAIN_DIE3
    state_t { sprite: SPR_BBRN, frame: 0, tics: -1, action: actionf_t::acp1(crate::p_enemy::A_BrainDie), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BRAIN_DIE4
    state_t { sprite: SPR_SSWV, frame: 0, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Look), nextstate: S_BRAINEYE, misc1: 0, misc2: 0 }, // S_BRAINEYE
    state_t { sprite: SPR_SSWV, frame: 0, tics: 181, action: actionf_t::acp1(crate::p_enemy::A_BrainAwake), nextstate: S_BRAINEYE1, misc1: 0, misc2: 0 }, // S_BRAINEYESEE
    state_t { sprite: SPR_SSWV, frame: 0, tics: 150, action: actionf_t::acp1(crate::p_enemy::A_BrainSpit), nextstate: S_BRAINEYE1, misc1: 0, misc2: 0 }, // S_BRAINEYE1
    state_t { sprite: SPR_BOSF, frame: 32768, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_SpawnSound), nextstate: S_SPAWN2, misc1: 0, misc2: 0 }, // S_SPAWN1
    state_t { sprite: SPR_BOSF, frame: 32769, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_SpawnFly), nextstate: S_SPAWN3, misc1: 0, misc2: 0 }, // S_SPAWN2
    state_t { sprite: SPR_BOSF, frame: 32770, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_SpawnFly), nextstate: S_SPAWN4, misc1: 0, misc2: 0 }, // S_SPAWN3
    state_t { sprite: SPR_BOSF, frame: 32771, tics: 3, action: actionf_t::acp1(crate::p_enemy::A_SpawnFly), nextstate: S_SPAWN1, misc1: 0, misc2: 0 }, // S_SPAWN4
    state_t { sprite: SPR_FIRE, frame: 32768, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE2, misc1: 0, misc2: 0 }, // S_SPAWNFIRE1
    state_t { sprite: SPR_FIRE, frame: 32769, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE3, misc1: 0, misc2: 0 }, // S_SPAWNFIRE2
    state_t { sprite: SPR_FIRE, frame: 32770, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE4, misc1: 0, misc2: 0 }, // S_SPAWNFIRE3
    state_t { sprite: SPR_FIRE, frame: 32771, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE5, misc1: 0, misc2: 0 }, // S_SPAWNFIRE4
    state_t { sprite: SPR_FIRE, frame: 32772, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE6, misc1: 0, misc2: 0 }, // S_SPAWNFIRE5
    state_t { sprite: SPR_FIRE, frame: 32773, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE7, misc1: 0, misc2: 0 }, // S_SPAWNFIRE6
    state_t { sprite: SPR_FIRE, frame: 32774, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_SPAWNFIRE8, misc1: 0, misc2: 0 }, // S_SPAWNFIRE7
    state_t { sprite: SPR_FIRE, frame: 32775, tics: 4, action: actionf_t::acp1(crate::p_enemy::A_Fire), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SPAWNFIRE8
    state_t { sprite: SPR_MISL, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_BRAINEXPLODE2, misc1: 0, misc2: 0 }, // S_BRAINEXPLODE1
    state_t { sprite: SPR_MISL, frame: 32770, tics: 10, action: actionf_t::None, nextstate: S_BRAINEXPLODE3, misc1: 0, misc2: 0 }, // S_BRAINEXPLODE2
    state_t { sprite: SPR_MISL, frame: 32771, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_BrainExplode), nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BRAINEXPLODE3
    state_t { sprite: SPR_ARM1, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_ARM1A, misc1: 0, misc2: 0 }, // S_ARM1
    state_t { sprite: SPR_ARM1, frame: 32769, tics: 7, action: actionf_t::None, nextstate: S_ARM1, misc1: 0, misc2: 0 }, // S_ARM1A
    state_t { sprite: SPR_ARM2, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_ARM2A, misc1: 0, misc2: 0 }, // S_ARM2
    state_t { sprite: SPR_ARM2, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_ARM2, misc1: 0, misc2: 0 }, // S_ARM2A
    state_t { sprite: SPR_BAR1, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_BAR2, misc1: 0, misc2: 0 }, // S_BAR1
    state_t { sprite: SPR_BAR1, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_BAR1, misc1: 0, misc2: 0 }, // S_BAR2
    state_t { sprite: SPR_BEXP, frame: 32768, tics: 5, action: actionf_t::None, nextstate: S_BEXP2, misc1: 0, misc2: 0 }, // S_BEXP
    state_t { sprite: SPR_BEXP, frame: 32769, tics: 5, action: actionf_t::acp1(crate::p_enemy::A_Scream), nextstate: S_BEXP3, misc1: 0, misc2: 0 }, // S_BEXP2
    state_t { sprite: SPR_BEXP, frame: 32770, tics: 5, action: actionf_t::None, nextstate: S_BEXP4, misc1: 0, misc2: 0 }, // S_BEXP3
    state_t { sprite: SPR_BEXP, frame: 32771, tics: 10, action: actionf_t::acp1(crate::p_enemy::A_Explode), nextstate: S_BEXP5, misc1: 0, misc2: 0 }, // S_BEXP4
    state_t { sprite: SPR_BEXP, frame: 32772, tics: 10, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BEXP5
    state_t { sprite: SPR_FCAN, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_BBAR2, misc1: 0, misc2: 0 }, // S_BBAR1
    state_t { sprite: SPR_FCAN, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_BBAR3, misc1: 0, misc2: 0 }, // S_BBAR2
    state_t { sprite: SPR_FCAN, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_BBAR1, misc1: 0, misc2: 0 }, // S_BBAR3
    state_t { sprite: SPR_BON1, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_BON1A, misc1: 0, misc2: 0 }, // S_BON1
    state_t { sprite: SPR_BON1, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_BON1B, misc1: 0, misc2: 0 }, // S_BON1A
    state_t { sprite: SPR_BON1, frame: 2, tics: 6, action: actionf_t::None, nextstate: S_BON1C, misc1: 0, misc2: 0 }, // S_BON1B
    state_t { sprite: SPR_BON1, frame: 3, tics: 6, action: actionf_t::None, nextstate: S_BON1D, misc1: 0, misc2: 0 }, // S_BON1C
    state_t { sprite: SPR_BON1, frame: 2, tics: 6, action: actionf_t::None, nextstate: S_BON1E, misc1: 0, misc2: 0 }, // S_BON1D
    state_t { sprite: SPR_BON1, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_BON1, misc1: 0, misc2: 0 }, // S_BON1E
    state_t { sprite: SPR_BON2, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_BON2A, misc1: 0, misc2: 0 }, // S_BON2
    state_t { sprite: SPR_BON2, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_BON2B, misc1: 0, misc2: 0 }, // S_BON2A
    state_t { sprite: SPR_BON2, frame: 2, tics: 6, action: actionf_t::None, nextstate: S_BON2C, misc1: 0, misc2: 0 }, // S_BON2B
    state_t { sprite: SPR_BON2, frame: 3, tics: 6, action: actionf_t::None, nextstate: S_BON2D, misc1: 0, misc2: 0 }, // S_BON2C
    state_t { sprite: SPR_BON2, frame: 2, tics: 6, action: actionf_t::None, nextstate: S_BON2E, misc1: 0, misc2: 0 }, // S_BON2D
    state_t { sprite: SPR_BON2, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_BON2, misc1: 0, misc2: 0 }, // S_BON2E
    state_t { sprite: SPR_BKEY, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_BKEY2, misc1: 0, misc2: 0 }, // S_BKEY
    state_t { sprite: SPR_BKEY, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_BKEY, misc1: 0, misc2: 0 }, // S_BKEY2
    state_t { sprite: SPR_RKEY, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_RKEY2, misc1: 0, misc2: 0 }, // S_RKEY
    state_t { sprite: SPR_RKEY, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_RKEY, misc1: 0, misc2: 0 }, // S_RKEY2
    state_t { sprite: SPR_YKEY, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_YKEY2, misc1: 0, misc2: 0 }, // S_YKEY
    state_t { sprite: SPR_YKEY, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_YKEY, misc1: 0, misc2: 0 }, // S_YKEY2
    state_t { sprite: SPR_BSKU, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_BSKULL2, misc1: 0, misc2: 0 }, // S_BSKULL
    state_t { sprite: SPR_BSKU, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_BSKULL, misc1: 0, misc2: 0 }, // S_BSKULL2
    state_t { sprite: SPR_RSKU, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_RSKULL2, misc1: 0, misc2: 0 }, // S_RSKULL
    state_t { sprite: SPR_RSKU, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_RSKULL, misc1: 0, misc2: 0 }, // S_RSKULL2
    state_t { sprite: SPR_YSKU, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_YSKULL2, misc1: 0, misc2: 0 }, // S_YSKULL
    state_t { sprite: SPR_YSKU, frame: 32769, tics: 10, action: actionf_t::None, nextstate: S_YSKULL, misc1: 0, misc2: 0 }, // S_YSKULL2
    state_t { sprite: SPR_STIM, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_STIM
    state_t { sprite: SPR_MEDI, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_MEDI
    state_t { sprite: SPR_SOUL, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_SOUL2, misc1: 0, misc2: 0 }, // S_SOUL
    state_t { sprite: SPR_SOUL, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_SOUL3, misc1: 0, misc2: 0 }, // S_SOUL2
    state_t { sprite: SPR_SOUL, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_SOUL4, misc1: 0, misc2: 0 }, // S_SOUL3
    state_t { sprite: SPR_SOUL, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_SOUL5, misc1: 0, misc2: 0 }, // S_SOUL4
    state_t { sprite: SPR_SOUL, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_SOUL6, misc1: 0, misc2: 0 }, // S_SOUL5
    state_t { sprite: SPR_SOUL, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_SOUL, misc1: 0, misc2: 0 }, // S_SOUL6
    state_t { sprite: SPR_PINV, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_PINV2, misc1: 0, misc2: 0 }, // S_PINV
    state_t { sprite: SPR_PINV, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_PINV3, misc1: 0, misc2: 0 }, // S_PINV2
    state_t { sprite: SPR_PINV, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_PINV4, misc1: 0, misc2: 0 }, // S_PINV3
    state_t { sprite: SPR_PINV, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_PINV, misc1: 0, misc2: 0 }, // S_PINV4
    state_t { sprite: SPR_PSTR, frame: 32768, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PSTR
    state_t { sprite: SPR_PINS, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_PINS2, misc1: 0, misc2: 0 }, // S_PINS
    state_t { sprite: SPR_PINS, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_PINS3, misc1: 0, misc2: 0 }, // S_PINS2
    state_t { sprite: SPR_PINS, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_PINS4, misc1: 0, misc2: 0 }, // S_PINS3
    state_t { sprite: SPR_PINS, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_PINS, misc1: 0, misc2: 0 }, // S_PINS4
    state_t { sprite: SPR_MEGA, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_MEGA2, misc1: 0, misc2: 0 }, // S_MEGA
    state_t { sprite: SPR_MEGA, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_MEGA3, misc1: 0, misc2: 0 }, // S_MEGA2
    state_t { sprite: SPR_MEGA, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_MEGA4, misc1: 0, misc2: 0 }, // S_MEGA3
    state_t { sprite: SPR_MEGA, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_MEGA, misc1: 0, misc2: 0 }, // S_MEGA4
    state_t { sprite: SPR_SUIT, frame: 32768, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SUIT
    state_t { sprite: SPR_PMAP, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_PMAP2, misc1: 0, misc2: 0 }, // S_PMAP
    state_t { sprite: SPR_PMAP, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_PMAP3, misc1: 0, misc2: 0 }, // S_PMAP2
    state_t { sprite: SPR_PMAP, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_PMAP4, misc1: 0, misc2: 0 }, // S_PMAP3
    state_t { sprite: SPR_PMAP, frame: 32771, tics: 6, action: actionf_t::None, nextstate: S_PMAP5, misc1: 0, misc2: 0 }, // S_PMAP4
    state_t { sprite: SPR_PMAP, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_PMAP6, misc1: 0, misc2: 0 }, // S_PMAP5
    state_t { sprite: SPR_PMAP, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_PMAP, misc1: 0, misc2: 0 }, // S_PMAP6
    state_t { sprite: SPR_PVIS, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_PVIS2, misc1: 0, misc2: 0 }, // S_PVIS
    state_t { sprite: SPR_PVIS, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_PVIS, misc1: 0, misc2: 0 }, // S_PVIS2
    state_t { sprite: SPR_CLIP, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CLIP
    state_t { sprite: SPR_AMMO, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_AMMO
    state_t { sprite: SPR_ROCK, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_ROCK
    state_t { sprite: SPR_BROK, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BROK
    state_t { sprite: SPR_CELL, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CELL
    state_t { sprite: SPR_CELP, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CELP
    state_t { sprite: SPR_SHEL, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SHEL
    state_t { sprite: SPR_SBOX, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SBOX
    state_t { sprite: SPR_BPAK, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BPAK
    state_t { sprite: SPR_BFUG, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BFUG
    state_t { sprite: SPR_MGUN, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_MGUN
    state_t { sprite: SPR_CSAW, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CSAW
    state_t { sprite: SPR_LAUN, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_LAUN
    state_t { sprite: SPR_PLAS, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_PLAS
    state_t { sprite: SPR_SHOT, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SHOT
    state_t { sprite: SPR_SGN2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SHOT2
    state_t { sprite: SPR_COLU, frame: 32768, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_COLU
    state_t { sprite: SPR_SMT2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_STALAG
    state_t { sprite: SPR_GOR1, frame: 0, tics: 10, action: actionf_t::None, nextstate: S_BLOODYTWITCH2, misc1: 0, misc2: 0 }, // S_BLOODYTWITCH
    state_t { sprite: SPR_GOR1, frame: 1, tics: 15, action: actionf_t::None, nextstate: S_BLOODYTWITCH3, misc1: 0, misc2: 0 }, // S_BLOODYTWITCH2
    state_t { sprite: SPR_GOR1, frame: 2, tics: 8, action: actionf_t::None, nextstate: S_BLOODYTWITCH4, misc1: 0, misc2: 0 }, // S_BLOODYTWITCH3
    state_t { sprite: SPR_GOR1, frame: 1, tics: 6, action: actionf_t::None, nextstate: S_BLOODYTWITCH, misc1: 0, misc2: 0 }, // S_BLOODYTWITCH4
    state_t { sprite: SPR_PLAY, frame: 13, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_DEADTORSO
    state_t { sprite: SPR_PLAY, frame: 18, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_DEADBOTTOM
    state_t { sprite: SPR_POL2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HEADSONSTICK
    state_t { sprite: SPR_POL5, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_GIBS
    state_t { sprite: SPR_POL4, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HEADONASTICK
    state_t { sprite: SPR_POL3, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_HEADCANDLES2, misc1: 0, misc2: 0 }, // S_HEADCANDLES
    state_t { sprite: SPR_POL3, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_HEADCANDLES, misc1: 0, misc2: 0 }, // S_HEADCANDLES2
    state_t { sprite: SPR_POL1, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_DEADSTICK
    state_t { sprite: SPR_POL6, frame: 0, tics: 6, action: actionf_t::None, nextstate: S_LIVESTICK2, misc1: 0, misc2: 0 }, // S_LIVESTICK
    state_t { sprite: SPR_POL6, frame: 1, tics: 8, action: actionf_t::None, nextstate: S_LIVESTICK, misc1: 0, misc2: 0 }, // S_LIVESTICK2
    state_t { sprite: SPR_GOR2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_MEAT2
    state_t { sprite: SPR_GOR3, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_MEAT3
    state_t { sprite: SPR_GOR4, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_MEAT4
    state_t { sprite: SPR_GOR5, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_MEAT5
    state_t { sprite: SPR_SMIT, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_STALAGTITE
    state_t { sprite: SPR_COL1, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TALLGRNCOL
    state_t { sprite: SPR_COL2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SHRTGRNCOL
    state_t { sprite: SPR_COL3, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TALLREDCOL
    state_t { sprite: SPR_COL4, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SHRTREDCOL
    state_t { sprite: SPR_CAND, frame: 32768, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CANDLESTIK
    state_t { sprite: SPR_CBRA, frame: 32768, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_CANDELABRA
    state_t { sprite: SPR_COL6, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SKULLCOL
    state_t { sprite: SPR_TRE1, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TORCHTREE
    state_t { sprite: SPR_TRE2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BIGTREE
    state_t { sprite: SPR_ELEC, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_TECHPILLAR
    state_t { sprite: SPR_CEYE, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_EVILEYE2, misc1: 0, misc2: 0 }, // S_EVILEYE
    state_t { sprite: SPR_CEYE, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_EVILEYE3, misc1: 0, misc2: 0 }, // S_EVILEYE2
    state_t { sprite: SPR_CEYE, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_EVILEYE4, misc1: 0, misc2: 0 }, // S_EVILEYE3
    state_t { sprite: SPR_CEYE, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_EVILEYE, misc1: 0, misc2: 0 }, // S_EVILEYE4
    state_t { sprite: SPR_FSKU, frame: 32768, tics: 6, action: actionf_t::None, nextstate: S_FLOATSKULL2, misc1: 0, misc2: 0 }, // S_FLOATSKULL
    state_t { sprite: SPR_FSKU, frame: 32769, tics: 6, action: actionf_t::None, nextstate: S_FLOATSKULL3, misc1: 0, misc2: 0 }, // S_FLOATSKULL2
    state_t { sprite: SPR_FSKU, frame: 32770, tics: 6, action: actionf_t::None, nextstate: S_FLOATSKULL, misc1: 0, misc2: 0 }, // S_FLOATSKULL3
    state_t { sprite: SPR_COL5, frame: 0, tics: 14, action: actionf_t::None, nextstate: S_HEARTCOL2, misc1: 0, misc2: 0 }, // S_HEARTCOL
    state_t { sprite: SPR_COL5, frame: 1, tics: 14, action: actionf_t::None, nextstate: S_HEARTCOL, misc1: 0, misc2: 0 }, // S_HEARTCOL2
    state_t { sprite: SPR_TBLU, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_BLUETORCH2, misc1: 0, misc2: 0 }, // S_BLUETORCH
    state_t { sprite: SPR_TBLU, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_BLUETORCH3, misc1: 0, misc2: 0 }, // S_BLUETORCH2
    state_t { sprite: SPR_TBLU, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_BLUETORCH4, misc1: 0, misc2: 0 }, // S_BLUETORCH3
    state_t { sprite: SPR_TBLU, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_BLUETORCH, misc1: 0, misc2: 0 }, // S_BLUETORCH4
    state_t { sprite: SPR_TGRN, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_GREENTORCH2, misc1: 0, misc2: 0 }, // S_GREENTORCH
    state_t { sprite: SPR_TGRN, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_GREENTORCH3, misc1: 0, misc2: 0 }, // S_GREENTORCH2
    state_t { sprite: SPR_TGRN, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_GREENTORCH4, misc1: 0, misc2: 0 }, // S_GREENTORCH3
    state_t { sprite: SPR_TGRN, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_GREENTORCH, misc1: 0, misc2: 0 }, // S_GREENTORCH4
    state_t { sprite: SPR_TRED, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_REDTORCH2, misc1: 0, misc2: 0 }, // S_REDTORCH
    state_t { sprite: SPR_TRED, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_REDTORCH3, misc1: 0, misc2: 0 }, // S_REDTORCH2
    state_t { sprite: SPR_TRED, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_REDTORCH4, misc1: 0, misc2: 0 }, // S_REDTORCH3
    state_t { sprite: SPR_TRED, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_REDTORCH, misc1: 0, misc2: 0 }, // S_REDTORCH4
    state_t { sprite: SPR_SMBT, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_BTORCHSHRT2, misc1: 0, misc2: 0 }, // S_BTORCHSHRT
    state_t { sprite: SPR_SMBT, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_BTORCHSHRT3, misc1: 0, misc2: 0 }, // S_BTORCHSHRT2
    state_t { sprite: SPR_SMBT, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_BTORCHSHRT4, misc1: 0, misc2: 0 }, // S_BTORCHSHRT3
    state_t { sprite: SPR_SMBT, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_BTORCHSHRT, misc1: 0, misc2: 0 }, // S_BTORCHSHRT4
    state_t { sprite: SPR_SMGT, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_GTORCHSHRT2, misc1: 0, misc2: 0 }, // S_GTORCHSHRT
    state_t { sprite: SPR_SMGT, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_GTORCHSHRT3, misc1: 0, misc2: 0 }, // S_GTORCHSHRT2
    state_t { sprite: SPR_SMGT, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_GTORCHSHRT4, misc1: 0, misc2: 0 }, // S_GTORCHSHRT3
    state_t { sprite: SPR_SMGT, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_GTORCHSHRT, misc1: 0, misc2: 0 }, // S_GTORCHSHRT4
    state_t { sprite: SPR_SMRT, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_RTORCHSHRT2, misc1: 0, misc2: 0 }, // S_RTORCHSHRT
    state_t { sprite: SPR_SMRT, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_RTORCHSHRT3, misc1: 0, misc2: 0 }, // S_RTORCHSHRT2
    state_t { sprite: SPR_SMRT, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_RTORCHSHRT4, misc1: 0, misc2: 0 }, // S_RTORCHSHRT3
    state_t { sprite: SPR_SMRT, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_RTORCHSHRT, misc1: 0, misc2: 0 }, // S_RTORCHSHRT4
    state_t { sprite: SPR_HDB1, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HANGNOGUTS
    state_t { sprite: SPR_HDB2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HANGBNOBRAIN
    state_t { sprite: SPR_HDB3, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HANGTLOOKDN
    state_t { sprite: SPR_HDB4, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HANGTSKULL
    state_t { sprite: SPR_HDB5, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HANGTLOOKUP
    state_t { sprite: SPR_HDB6, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_HANGTNOBRAIN
    state_t { sprite: SPR_POB1, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_COLONGIBS
    state_t { sprite: SPR_POB2, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_SMALLPOOL
    state_t { sprite: SPR_BRS1, frame: 0, tics: -1, action: actionf_t::None, nextstate: S_NULL, misc1: 0, misc2: 0 }, // S_BRAINSTEM
    state_t { sprite: SPR_TLMP, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_TECHLAMP2, misc1: 0, misc2: 0 }, // S_TECHLAMP
    state_t { sprite: SPR_TLMP, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_TECHLAMP3, misc1: 0, misc2: 0 }, // S_TECHLAMP2
    state_t { sprite: SPR_TLMP, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_TECHLAMP4, misc1: 0, misc2: 0 }, // S_TECHLAMP3
    state_t { sprite: SPR_TLMP, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_TECHLAMP, misc1: 0, misc2: 0 }, // S_TECHLAMP4
    state_t { sprite: SPR_TLP2, frame: 32768, tics: 4, action: actionf_t::None, nextstate: S_TECH2LAMP2, misc1: 0, misc2: 0 }, // S_TECH2LAMP
    state_t { sprite: SPR_TLP2, frame: 32769, tics: 4, action: actionf_t::None, nextstate: S_TECH2LAMP3, misc1: 0, misc2: 0 }, // S_TECH2LAMP2
    state_t { sprite: SPR_TLP2, frame: 32770, tics: 4, action: actionf_t::None, nextstate: S_TECH2LAMP4, misc1: 0, misc2: 0 }, // S_TECH2LAMP3
    state_t { sprite: SPR_TLP2, frame: 32771, tics: 4, action: actionf_t::None, nextstate: S_TECH2LAMP, misc1: 0, misc2: 0 }, // S_TECH2LAMP4
];

pub static mut mobjinfo: [mobjinfo_t; 137] = [
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_PLAY,                   // spawnstate
        spawnhealth: 100,                     // spawnhealth
        seestate: S_PLAY_RUN1,                // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 0,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_PLAY_PAIN,               // painstate
        painchance: 255,                      // painchance
        painsound: sfx_plpain,                // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_PLAY_ATK1,            // missilestate
        deathstate: S_PLAY_DIE1,              // deathstate
        xdeathstate: S_PLAY_XDIE1,            // xdeathstate
        deathsound: sfx_pldeth,               // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_DROPOFF | MF_PICKUP | MF_NOTDMATCH, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 3004,                      // doomednum
        spawnstate: S_POSS_STND,              // spawnstate
        spawnhealth: 20,                      // spawnhealth
        seestate: S_POSS_RUN1,                // seestate
        seesound: sfx_posit1,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_pistol,              // attacksound
        painstate: S_POSS_PAIN,               // painstate
        painchance: 200,                      // painchance
        painsound: sfx_popain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_POSS_ATK1,            // missilestate
        deathstate: S_POSS_DIE1,              // deathstate
        xdeathstate: S_POSS_XDIE1,            // xdeathstate
        deathsound: sfx_podth1,               // deathsound
        speed: 8,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_posact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_POSS_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 9,                         // doomednum
        spawnstate: S_SPOS_STND,              // spawnstate
        spawnhealth: 30,                      // spawnhealth
        seestate: S_SPOS_RUN1,                // seestate
        seesound: sfx_posit2,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_SPOS_PAIN,               // painstate
        painchance: 170,                      // painchance
        painsound: sfx_popain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_SPOS_ATK1,            // missilestate
        deathstate: S_SPOS_DIE1,              // deathstate
        xdeathstate: S_SPOS_XDIE1,            // xdeathstate
        deathsound: sfx_podth2,               // deathsound
        speed: 8,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_posact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_SPOS_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 64,                        // doomednum
        spawnstate: S_VILE_STND,              // spawnstate
        spawnhealth: 700,                     // spawnhealth
        seestate: S_VILE_RUN1,                // seestate
        seesound: sfx_vilsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_VILE_PAIN,               // painstate
        painchance: 10,                       // painchance
        painsound: sfx_vipain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_VILE_ATK1,            // missilestate
        deathstate: S_VILE_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_vildth,               // deathsound
        speed: 15,                            // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 500,                            // mass
        damage: 0,                            // damage
        activesound: sfx_vilact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_FIRE1,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 66,                        // doomednum
        spawnstate: S_SKEL_STND,              // spawnstate
        spawnhealth: 300,                     // spawnhealth
        seestate: S_SKEL_RUN1,                // seestate
        seesound: sfx_skesit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_SKEL_PAIN,               // painstate
        painchance: 100,                      // painchance
        painsound: sfx_popain,                // painsound
        meleestate: S_SKEL_FIST1,             // meleestate
        missilestate: S_SKEL_MISS1,           // missilestate
        deathstate: S_SKEL_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_skedth,               // deathsound
        speed: 10,                            // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 500,                            // mass
        damage: 0,                            // damage
        activesound: sfx_skeact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_SKEL_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_TRACER,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_skeatk,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_TRACEEXP1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_barexp,               // deathsound
        speed: 10*FRACUNIT,                   // speed
        radius: 11*FRACUNIT,                  // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 10,                           // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_SMOKE1,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 67,                        // doomednum
        spawnstate: S_FATT_STND,              // spawnstate
        spawnhealth: 600,                     // spawnhealth
        seestate: S_FATT_RUN1,                // seestate
        seesound: sfx_mansit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_FATT_PAIN,               // painstate
        painchance: 80,                       // painchance
        painsound: sfx_mnpain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_FATT_ATK1,            // missilestate
        deathstate: S_FATT_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_mandth,               // deathsound
        speed: 8,                             // speed
        radius: 48*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 1000,                           // mass
        damage: 0,                            // damage
        activesound: sfx_posact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_FATT_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_FATSHOT1,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_firsht,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_FATSHOTX1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 20*FRACUNIT,                   // speed
        radius: 6*FRACUNIT,                   // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 8,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 65,                        // doomednum
        spawnstate: S_CPOS_STND,              // spawnstate
        spawnhealth: 70,                      // spawnhealth
        seestate: S_CPOS_RUN1,                // seestate
        seesound: sfx_posit2,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_CPOS_PAIN,               // painstate
        painchance: 170,                      // painchance
        painsound: sfx_popain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_CPOS_ATK1,            // missilestate
        deathstate: S_CPOS_DIE1,              // deathstate
        xdeathstate: S_CPOS_XDIE1,            // xdeathstate
        deathsound: sfx_podth2,               // deathsound
        speed: 8,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_posact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_CPOS_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 3001,                      // doomednum
        spawnstate: S_TROO_STND,              // spawnstate
        spawnhealth: 60,                      // spawnhealth
        seestate: S_TROO_RUN1,                // seestate
        seesound: sfx_bgsit1,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_TROO_PAIN,               // painstate
        painchance: 200,                      // painchance
        painsound: sfx_popain,                // painsound
        meleestate: S_TROO_ATK1,              // meleestate
        missilestate: S_TROO_ATK1,            // missilestate
        deathstate: S_TROO_DIE1,              // deathstate
        xdeathstate: S_TROO_XDIE1,            // xdeathstate
        deathsound: sfx_bgdth1,               // deathsound
        speed: 8,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_bgact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_TROO_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 3002,                      // doomednum
        spawnstate: S_SARG_STND,              // spawnstate
        spawnhealth: 150,                     // spawnhealth
        seestate: S_SARG_RUN1,                // seestate
        seesound: sfx_sgtsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_sgtatk,              // attacksound
        painstate: S_SARG_PAIN,               // painstate
        painchance: 180,                      // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: S_SARG_ATK1,              // meleestate
        missilestate: 0,                      // missilestate
        deathstate: S_SARG_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_sgtdth,               // deathsound
        speed: 10,                            // speed
        radius: 30*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 400,                            // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_SARG_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 58,                        // doomednum
        spawnstate: S_SARG_STND,              // spawnstate
        spawnhealth: 150,                     // spawnhealth
        seestate: S_SARG_RUN1,                // seestate
        seesound: sfx_sgtsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_sgtatk,              // attacksound
        painstate: S_SARG_PAIN,               // painstate
        painchance: 180,                      // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: S_SARG_ATK1,              // meleestate
        missilestate: 0,                      // missilestate
        deathstate: S_SARG_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_sgtdth,               // deathsound
        speed: 10,                            // speed
        radius: 30*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 400,                            // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_SHADOW | MF_COUNTKILL, // flags
        raisestate: S_SARG_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 3005,                      // doomednum
        spawnstate: S_HEAD_STND,              // spawnstate
        spawnhealth: 400,                     // spawnhealth
        seestate: S_HEAD_RUN1,                // seestate
        seesound: sfx_cacsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_HEAD_PAIN,               // painstate
        painchance: 128,                      // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_HEAD_ATK1,            // missilestate
        deathstate: S_HEAD_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_cacdth,               // deathsound
        speed: 8,                             // speed
        radius: 31*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 400,                            // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY | MF_COUNTKILL, // flags
        raisestate: S_HEAD_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 3003,                      // doomednum
        spawnstate: S_BOSS_STND,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_BOSS_RUN1,                // seestate
        seesound: sfx_brssit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_BOSS_PAIN,               // painstate
        painchance: 50,                       // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: S_BOSS_ATK1,              // meleestate
        missilestate: S_BOSS_ATK1,            // missilestate
        deathstate: S_BOSS_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_brsdth,               // deathsound
        speed: 8,                             // speed
        radius: 24*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 1000,                           // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_BOSS_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_BRBALL1,                // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_firsht,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_BRBALLX1,               // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 15*FRACUNIT,                   // speed
        radius: 6*FRACUNIT,                   // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 8,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 69,                        // doomednum
        spawnstate: S_BOS2_STND,              // spawnstate
        spawnhealth: 500,                     // spawnhealth
        seestate: S_BOS2_RUN1,                // seestate
        seesound: sfx_kntsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_BOS2_PAIN,               // painstate
        painchance: 50,                       // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: S_BOS2_ATK1,              // meleestate
        missilestate: S_BOS2_ATK1,            // missilestate
        deathstate: S_BOS2_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_kntdth,               // deathsound
        speed: 8,                             // speed
        radius: 24*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 1000,                           // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_BOS2_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 3006,                      // doomednum
        spawnstate: S_SKULL_STND,             // spawnstate
        spawnhealth: 100,                     // spawnhealth
        seestate: S_SKULL_RUN1,               // seestate
        seesound: 0,                          // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_sklatk,              // attacksound
        painstate: S_SKULL_PAIN,              // painstate
        painchance: 256,                      // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_SKULL_ATK1,           // missilestate
        deathstate: S_SKULL_DIE1,             // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 8,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 50,                             // mass
        damage: 3,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 7,                         // doomednum
        spawnstate: S_SPID_STND,              // spawnstate
        spawnhealth: 3000,                    // spawnhealth
        seestate: S_SPID_RUN1,                // seestate
        seesound: sfx_spisit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_shotgn,              // attacksound
        painstate: S_SPID_PAIN,               // painstate
        painchance: 40,                       // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_SPID_ATK1,            // missilestate
        deathstate: S_SPID_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_spidth,               // deathsound
        speed: 12,                            // speed
        radius: 128*FRACUNIT,                 // radius
        height: 100*FRACUNIT,                 // height
        mass: 1000,                           // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 68,                        // doomednum
        spawnstate: S_BSPI_STND,              // spawnstate
        spawnhealth: 500,                     // spawnhealth
        seestate: S_BSPI_SIGHT,               // seestate
        seesound: sfx_bspsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_BSPI_PAIN,               // painstate
        painchance: 128,                      // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_BSPI_ATK1,            // missilestate
        deathstate: S_BSPI_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_bspdth,               // deathsound
        speed: 12,                            // speed
        radius: 64*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 600,                            // mass
        damage: 0,                            // damage
        activesound: sfx_bspact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_BSPI_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 16,                        // doomednum
        spawnstate: S_CYBER_STND,             // spawnstate
        spawnhealth: 4000,                    // spawnhealth
        seestate: S_CYBER_RUN1,               // seestate
        seesound: sfx_cybsit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_CYBER_PAIN,              // painstate
        painchance: 20,                       // painchance
        painsound: sfx_dmpain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_CYBER_ATK1,           // missilestate
        deathstate: S_CYBER_DIE1,             // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_cybdth,               // deathsound
        speed: 16,                            // speed
        radius: 40*FRACUNIT,                  // radius
        height: 110*FRACUNIT,                 // height
        mass: 1000,                           // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 71,                        // doomednum
        spawnstate: S_PAIN_STND,              // spawnstate
        spawnhealth: 400,                     // spawnhealth
        seestate: S_PAIN_RUN1,                // seestate
        seesound: sfx_pesit,                  // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_PAIN_PAIN,               // painstate
        painchance: 128,                      // painchance
        painsound: sfx_pepain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_PAIN_ATK1,            // missilestate
        deathstate: S_PAIN_DIE1,              // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_pedth,                // deathsound
        speed: 8,                             // speed
        radius: 31*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 400,                            // mass
        damage: 0,                            // damage
        activesound: sfx_dmact,               // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY | MF_COUNTKILL, // flags
        raisestate: S_PAIN_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 84,                        // doomednum
        spawnstate: S_SSWV_STND,              // spawnstate
        spawnhealth: 50,                      // spawnhealth
        seestate: S_SSWV_RUN1,                // seestate
        seesound: sfx_sssit,                  // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: 0,                       // attacksound
        painstate: S_SSWV_PAIN,               // painstate
        painchance: 170,                      // painchance
        painsound: sfx_popain,                // painsound
        meleestate: 0,                        // meleestate
        missilestate: S_SSWV_ATK1,            // missilestate
        deathstate: S_SSWV_DIE1,              // deathstate
        xdeathstate: S_SSWV_XDIE1,            // xdeathstate
        deathsound: sfx_ssdth,                // deathsound
        speed: 8,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 56*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_posact,              // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_SSWV_RAISE1,            // raisestate
    },
    mobjinfo_t {
        doomednum: 72,                        // doomednum
        spawnstate: S_KEENSTND,               // spawnstate
        spawnhealth: 100,                     // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_KEENPAIN,                // painstate
        painchance: 256,                      // painchance
        painsound: sfx_keenpn,                // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_COMMKEEN,               // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_keendt,               // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 72*FRACUNIT,                  // height
        mass: 10000000,                       // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY | MF_SHOOTABLE | MF_COUNTKILL, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 88,                        // doomednum
        spawnstate: S_BRAIN,                  // spawnstate
        spawnhealth: 250,                     // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_BRAIN_PAIN,              // painstate
        painchance: 255,                      // painchance
        painsound: sfx_bospn,                 // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_BRAIN_DIE1,             // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_bosdth,               // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 10000000,                       // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SHOOTABLE,       // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 89,                        // doomednum
        spawnstate: S_BRAINEYE,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_BRAINEYESEE,              // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 32*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOSECTOR,   // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 87,                        // doomednum
        spawnstate: S_NULL,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 32*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOSECTOR,   // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_SPAWN1,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_bospit,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 10*FRACUNIT,                   // speed
        radius: 6*FRACUNIT,                   // radius
        height: 32*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 3,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY | MF_NOCLIP, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_SPAWNFIRE1,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2035,                      // doomednum
        spawnstate: S_BAR1,                   // spawnstate
        spawnhealth: 20,                      // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_BEXP,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_barexp,               // deathsound
        speed: 0,                             // speed
        radius: 10*FRACUNIT,                  // radius
        height: 42*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SHOOTABLE | MF_NOBLOOD, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_TBALL1,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_firsht,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_TBALLX1,                // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 10*FRACUNIT,                   // speed
        radius: 6*FRACUNIT,                   // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 3,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_RBALL1,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_firsht,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_RBALLX1,                // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 10*FRACUNIT,                   // speed
        radius: 6*FRACUNIT,                   // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 5,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_ROCKET,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_rlaunc,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_EXPLODE1,               // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_barexp,               // deathsound
        speed: 20*FRACUNIT,                   // speed
        radius: 11*FRACUNIT,                  // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 20,                           // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_PLASBALL,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_plasma,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_PLASEXP,                // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 25*FRACUNIT,                   // speed
        radius: 13*FRACUNIT,                  // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 5,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_BFGSHOT,                // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: 0,                          // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_BFGLAND,                // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_rxplod,               // deathsound
        speed: 25*FRACUNIT,                   // speed
        radius: 13*FRACUNIT,                  // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 100,                          // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_ARACH_PLAZ,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_plasma,                 // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_ARACH_PLEX,             // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_firxpl,               // deathsound
        speed: 25*FRACUNIT,                   // speed
        radius: 13*FRACUNIT,                  // radius
        height: 8*FRACUNIT,                   // height
        mass: 100,                            // mass
        damage: 5,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_PUFF1,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_BLOOD1,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP,                 // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_TFOG,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_IFOG,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 14,                        // doomednum
        spawnstate: S_NULL,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOSECTOR,   // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: -1,                        // doomednum
        spawnstate: S_BFGEXP,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP | MF_NOGRAVITY,  // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2018,                      // doomednum
        spawnstate: S_ARM1,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2019,                      // doomednum
        spawnstate: S_ARM2,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2014,                      // doomednum
        spawnstate: S_BON1,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2015,                      // doomednum
        spawnstate: S_BON2,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 5,                         // doomednum
        spawnstate: S_BKEY,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_NOTDMATCH,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 13,                        // doomednum
        spawnstate: S_RKEY,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_NOTDMATCH,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 6,                         // doomednum
        spawnstate: S_YKEY,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_NOTDMATCH,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 39,                        // doomednum
        spawnstate: S_YSKULL,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_NOTDMATCH,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 38,                        // doomednum
        spawnstate: S_RSKULL,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_NOTDMATCH,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 40,                        // doomednum
        spawnstate: S_BSKULL,                 // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_NOTDMATCH,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2011,                      // doomednum
        spawnstate: S_STIM,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2012,                      // doomednum
        spawnstate: S_MEDI,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2013,                      // doomednum
        spawnstate: S_SOUL,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2022,                      // doomednum
        spawnstate: S_PINV,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2023,                      // doomednum
        spawnstate: S_PSTR,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2024,                      // doomednum
        spawnstate: S_PINS,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2025,                      // doomednum
        spawnstate: S_SUIT,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2026,                      // doomednum
        spawnstate: S_PMAP,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2045,                      // doomednum
        spawnstate: S_PVIS,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 83,                        // doomednum
        spawnstate: S_MEGA,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL | MF_COUNTITEM,     // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2007,                      // doomednum
        spawnstate: S_CLIP,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2048,                      // doomednum
        spawnstate: S_AMMO,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2010,                      // doomednum
        spawnstate: S_ROCK,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2046,                      // doomednum
        spawnstate: S_BROK,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2047,                      // doomednum
        spawnstate: S_CELL,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 17,                        // doomednum
        spawnstate: S_CELP,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2008,                      // doomednum
        spawnstate: S_SHEL,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2049,                      // doomednum
        spawnstate: S_SBOX,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 8,                         // doomednum
        spawnstate: S_BPAK,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2006,                      // doomednum
        spawnstate: S_BFUG,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2002,                      // doomednum
        spawnstate: S_MGUN,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2005,                      // doomednum
        spawnstate: S_CSAW,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2003,                      // doomednum
        spawnstate: S_LAUN,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2004,                      // doomednum
        spawnstate: S_PLAS,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2001,                      // doomednum
        spawnstate: S_SHOT,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 82,                        // doomednum
        spawnstate: S_SHOT2,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPECIAL,                    // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 85,                        // doomednum
        spawnstate: S_TECHLAMP,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 86,                        // doomednum
        spawnstate: S_TECH2LAMP,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 2028,                      // doomednum
        spawnstate: S_COLU,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 30,                        // doomednum
        spawnstate: S_TALLGRNCOL,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 31,                        // doomednum
        spawnstate: S_SHRTGRNCOL,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 32,                        // doomednum
        spawnstate: S_TALLREDCOL,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 33,                        // doomednum
        spawnstate: S_SHRTREDCOL,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 37,                        // doomednum
        spawnstate: S_SKULLCOL,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 36,                        // doomednum
        spawnstate: S_HEARTCOL,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 41,                        // doomednum
        spawnstate: S_EVILEYE,                // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 42,                        // doomednum
        spawnstate: S_FLOATSKULL,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 43,                        // doomednum
        spawnstate: S_TORCHTREE,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 44,                        // doomednum
        spawnstate: S_BLUETORCH,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 45,                        // doomednum
        spawnstate: S_GREENTORCH,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 46,                        // doomednum
        spawnstate: S_REDTORCH,               // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 55,                        // doomednum
        spawnstate: S_BTORCHSHRT,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 56,                        // doomednum
        spawnstate: S_GTORCHSHRT,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 57,                        // doomednum
        spawnstate: S_RTORCHSHRT,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 47,                        // doomednum
        spawnstate: S_STALAGTITE,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 48,                        // doomednum
        spawnstate: S_TECHPILLAR,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 34,                        // doomednum
        spawnstate: S_CANDLESTIK,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 35,                        // doomednum
        spawnstate: S_CANDELABRA,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 49,                        // doomednum
        spawnstate: S_BLOODYTWITCH,           // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 68*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 50,                        // doomednum
        spawnstate: S_MEAT2,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 84*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 51,                        // doomednum
        spawnstate: S_MEAT3,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 84*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 52,                        // doomednum
        spawnstate: S_MEAT4,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 68*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 53,                        // doomednum
        spawnstate: S_MEAT5,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 52*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 59,                        // doomednum
        spawnstate: S_MEAT2,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 84*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 60,                        // doomednum
        spawnstate: S_MEAT4,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 68*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 61,                        // doomednum
        spawnstate: S_MEAT3,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 52*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 62,                        // doomednum
        spawnstate: S_MEAT5,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 52*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 63,                        // doomednum
        spawnstate: S_BLOODYTWITCH,           // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 68*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 22,                        // doomednum
        spawnstate: S_HEAD_DIE6,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 15,                        // doomednum
        spawnstate: S_PLAY_DIE7,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 18,                        // doomednum
        spawnstate: S_POSS_DIE5,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 21,                        // doomednum
        spawnstate: S_SARG_DIE6,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 23,                        // doomednum
        spawnstate: S_SKULL_DIE6,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 20,                        // doomednum
        spawnstate: S_TROO_DIE5,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 19,                        // doomednum
        spawnstate: S_SPOS_DIE5,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 10,                        // doomednum
        spawnstate: S_PLAY_XDIE9,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 12,                        // doomednum
        spawnstate: S_PLAY_XDIE9,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 28,                        // doomednum
        spawnstate: S_HEADSONSTICK,           // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 24,                        // doomednum
        spawnstate: S_GIBS,                   // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: 0,                             // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 27,                        // doomednum
        spawnstate: S_HEADONASTICK,           // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 29,                        // doomednum
        spawnstate: S_HEADCANDLES,            // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 25,                        // doomednum
        spawnstate: S_DEADSTICK,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 26,                        // doomednum
        spawnstate: S_LIVESTICK,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 54,                        // doomednum
        spawnstate: S_BIGTREE,                // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 32*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 70,                        // doomednum
        spawnstate: S_BBAR1,                  // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID,                      // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 73,                        // doomednum
        spawnstate: S_HANGNOGUTS,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 88*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 74,                        // doomednum
        spawnstate: S_HANGBNOBRAIN,           // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 88*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 75,                        // doomednum
        spawnstate: S_HANGTLOOKDN,            // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 76,                        // doomednum
        spawnstate: S_HANGTSKULL,             // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 77,                        // doomednum
        spawnstate: S_HANGTLOOKUP,            // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 78,                        // doomednum
        spawnstate: S_HANGTNOBRAIN,           // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 16*FRACUNIT,                  // radius
        height: 64*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY, // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 79,                        // doomednum
        spawnstate: S_COLONGIBS,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP,                 // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 80,                        // doomednum
        spawnstate: S_SMALLPOOL,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP,                 // flags
        raisestate: S_NULL,                   // raisestate
    },
    mobjinfo_t {
        doomednum: 81,                        // doomednum
        spawnstate: S_BRAINSTEM,              // spawnstate
        spawnhealth: 1000,                    // spawnhealth
        seestate: S_NULL,                     // seestate
        seesound: sfx_None,                   // seesound
        reactiontime: 8,                      // reactiontime
        attacksound: sfx_None,                // attacksound
        painstate: S_NULL,                    // painstate
        painchance: 0,                        // painchance
        painsound: sfx_None,                  // painsound
        meleestate: S_NULL,                   // meleestate
        missilestate: S_NULL,                 // missilestate
        deathstate: S_NULL,                   // deathstate
        xdeathstate: S_NULL,                  // xdeathstate
        deathsound: sfx_None,                 // deathsound
        speed: 0,                             // speed
        radius: 20*FRACUNIT,                  // radius
        height: 16*FRACUNIT,                  // height
        mass: 100,                            // mass
        damage: 0,                            // damage
        activesound: sfx_None,                // activesound
        flags: MF_NOBLOCKMAP,                 // flags
        raisestate: S_NULL,                   // raisestate
    },
];
