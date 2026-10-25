// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdef::*;
use crate::info::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct weaponinfo_t {
    pub ammo: ammotype_t,
    pub upstate: i32,
    pub downstate: i32,
    pub readystate: i32,
    pub atkstate: i32,
    pub flashstate: i32,
}

pub static mut weaponinfo: [weaponinfo_t; NUMWEAPONS] = [
    weaponinfo_t {
        ammo: am_noammo,
        upstate: S_PUNCHUP,
        downstate: S_PUNCHDOWN,
        readystate: S_PUNCH,
        atkstate: S_PUNCH1,
        flashstate: S_NULL,
    },
    weaponinfo_t {
        ammo: am_clip,
        upstate: S_PISTOLUP,
        downstate: S_PISTOLDOWN,
        readystate: S_PISTOL,
        atkstate: S_PISTOL1,
        flashstate: S_PISTOLFLASH,
    },
    weaponinfo_t {
        ammo: am_shell,
        upstate: S_SGUNUP,
        downstate: S_SGUNDOWN,
        readystate: S_SGUN,
        atkstate: S_SGUN1,
        flashstate: S_SGUNFLASH1,
    },
    weaponinfo_t {
        ammo: am_clip,
        upstate: S_CHAINUP,
        downstate: S_CHAINDOWN,
        readystate: S_CHAIN,
        atkstate: S_CHAIN1,
        flashstate: S_CHAINFLASH1,
    },
    weaponinfo_t {
        ammo: am_misl,
        upstate: S_MISSILEUP,
        downstate: S_MISSILEDOWN,
        readystate: S_MISSILE,
        atkstate: S_MISSILE1,
        flashstate: S_MISSILEFLASH1,
    },
    weaponinfo_t {
        ammo: am_cell,
        upstate: S_PLASMAUP,
        downstate: S_PLASMADOWN,
        readystate: S_PLASMA,
        atkstate: S_PLASMA1,
        flashstate: S_PLASMAFLASH1,
    },
    weaponinfo_t {
        ammo: am_cell,
        upstate: S_BFGUP,
        downstate: S_BFGDOWN,
        readystate: S_BFG,
        atkstate: S_BFG1,
        flashstate: S_BFGFLASH1,
    },
    weaponinfo_t {
        ammo: am_noammo,
        upstate: S_SAWUP,
        downstate: S_SAWDOWN,
        readystate: S_SAW,
        atkstate: S_SAW1,
        flashstate: S_NULL,
    },
    weaponinfo_t {
        ammo: am_shell,
        upstate: S_DSGUNUP,
        downstate: S_DSGUNDOWN,
        readystate: S_DSGUN,
        atkstate: S_DSGUN1,
        flashstate: S_DSGUNFLASH1,
    },
];
