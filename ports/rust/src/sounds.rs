// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::doomtype::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfxinfo_t {
    pub name: &'static str,
    pub singularity: i32,
    pub priority: i32,
    pub link_idx: isize,
    pub pitch: i32,
    pub volume: i32,
    pub data: *mut c_void,
    pub usefulness: i32,
    pub lumpnum: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musicinfo_t {
    pub name: &'static str,
    pub lumpnum: i32,
    pub data: *mut c_void,
    pub handle: i32,
}

pub type musicenum_t = i32;
pub const mus_None: musicenum_t = 0;
pub const mus_e1m1: musicenum_t = 1;
pub const mus_e1m2: musicenum_t = 2;
pub const mus_e1m3: musicenum_t = 3;
pub const mus_e1m4: musicenum_t = 4;
pub const mus_e1m5: musicenum_t = 5;
pub const mus_e1m6: musicenum_t = 6;
pub const mus_e1m7: musicenum_t = 7;
pub const mus_e1m8: musicenum_t = 8;
pub const mus_e1m9: musicenum_t = 9;
pub const mus_e2m1: musicenum_t = 10;
pub const mus_e2m2: musicenum_t = 11;
pub const mus_e2m3: musicenum_t = 12;
pub const mus_e2m4: musicenum_t = 13;
pub const mus_e2m5: musicenum_t = 14;
pub const mus_e2m6: musicenum_t = 15;
pub const mus_e2m7: musicenum_t = 16;
pub const mus_e2m8: musicenum_t = 17;
pub const mus_e2m9: musicenum_t = 18;
pub const mus_e3m1: musicenum_t = 19;
pub const mus_e3m2: musicenum_t = 20;
pub const mus_e3m3: musicenum_t = 21;
pub const mus_e3m4: musicenum_t = 22;
pub const mus_e3m5: musicenum_t = 23;
pub const mus_e3m6: musicenum_t = 24;
pub const mus_e3m7: musicenum_t = 25;
pub const mus_e3m8: musicenum_t = 26;
pub const mus_e3m9: musicenum_t = 27;
pub const mus_inter: musicenum_t = 28;
pub const mus_intro: musicenum_t = 29;
pub const mus_bunny: musicenum_t = 30;
pub const mus_victor: musicenum_t = 31;
pub const mus_introa: musicenum_t = 32;
pub const mus_runnin: musicenum_t = 33;
pub const mus_stalks: musicenum_t = 34;
pub const mus_countd: musicenum_t = 35;
pub const mus_betwee: musicenum_t = 36;
pub const mus_doom: musicenum_t = 37;
pub const mus_the_da: musicenum_t = 38;
pub const mus_shawn: musicenum_t = 39;
pub const mus_ddtblu: musicenum_t = 40;
pub const mus_in_cit: musicenum_t = 41;
pub const mus_dead: musicenum_t = 42;
pub const mus_stlks2: musicenum_t = 43;
pub const mus_theda2: musicenum_t = 44;
pub const mus_doom2: musicenum_t = 45;
pub const mus_ddtbl2: musicenum_t = 46;
pub const mus_runni2: musicenum_t = 47;
pub const mus_dead2: musicenum_t = 48;
pub const mus_stlks3: musicenum_t = 49;
pub const mus_romero: musicenum_t = 50;
pub const mus_shawn2: musicenum_t = 51;
pub const mus_messag: musicenum_t = 52;
pub const mus_count2: musicenum_t = 53;
pub const mus_ddtbl3: musicenum_t = 54;
pub const mus_ampie: musicenum_t = 55;
pub const mus_theda3: musicenum_t = 56;
pub const mus_adrian: musicenum_t = 57;
pub const mus_messg2: musicenum_t = 58;
pub const mus_romer2: musicenum_t = 59;
pub const mus_tense: musicenum_t = 60;
pub const mus_shawn3: musicenum_t = 61;
pub const mus_openin: musicenum_t = 62;
pub const mus_evil: musicenum_t = 63;
pub const mus_ultima: musicenum_t = 64;
pub const mus_read_m: musicenum_t = 65;
pub const mus_dm2ttl: musicenum_t = 66;
pub const mus_dm2int: musicenum_t = 67;
pub const NUMMUSIC: usize = 68;

pub type sfxenum_t = i32;
pub const sfx_None: sfxenum_t = 0;
pub const sfx_pistol: sfxenum_t = 1;
pub const sfx_shotgn: sfxenum_t = 2;
pub const sfx_sgcock: sfxenum_t = 3;
pub const sfx_dshtgn: sfxenum_t = 4;
pub const sfx_dbopn: sfxenum_t = 5;
pub const sfx_dbcls: sfxenum_t = 6;
pub const sfx_dbload: sfxenum_t = 7;
pub const sfx_plasma: sfxenum_t = 8;
pub const sfx_bfg: sfxenum_t = 9;
pub const sfx_sawup: sfxenum_t = 10;
pub const sfx_sawidl: sfxenum_t = 11;
pub const sfx_sawful: sfxenum_t = 12;
pub const sfx_sawhit: sfxenum_t = 13;
pub const sfx_rlaunc: sfxenum_t = 14;
pub const sfx_rxplod: sfxenum_t = 15;
pub const sfx_firsht: sfxenum_t = 16;
pub const sfx_firxpl: sfxenum_t = 17;
pub const sfx_pstart: sfxenum_t = 18;
pub const sfx_pstop: sfxenum_t = 19;
pub const sfx_doropn: sfxenum_t = 20;
pub const sfx_dorcls: sfxenum_t = 21;
pub const sfx_stnmov: sfxenum_t = 22;
pub const sfx_swtchn: sfxenum_t = 23;
pub const sfx_swtchx: sfxenum_t = 24;
pub const sfx_plpain: sfxenum_t = 25;
pub const sfx_dmpain: sfxenum_t = 26;
pub const sfx_popain: sfxenum_t = 27;
pub const sfx_vipain: sfxenum_t = 28;
pub const sfx_mnpain: sfxenum_t = 29;
pub const sfx_pepain: sfxenum_t = 30;
pub const sfx_slop: sfxenum_t = 31;
pub const sfx_itemup: sfxenum_t = 32;
pub const sfx_wpnup: sfxenum_t = 33;
pub const sfx_oof: sfxenum_t = 34;
pub const sfx_telept: sfxenum_t = 35;
pub const sfx_posit1: sfxenum_t = 36;
pub const sfx_posit2: sfxenum_t = 37;
pub const sfx_posit3: sfxenum_t = 38;
pub const sfx_bgsit1: sfxenum_t = 39;
pub const sfx_bgsit2: sfxenum_t = 40;
pub const sfx_sgtsit: sfxenum_t = 41;
pub const sfx_cacsit: sfxenum_t = 42;
pub const sfx_brssit: sfxenum_t = 43;
pub const sfx_cybsit: sfxenum_t = 44;
pub const sfx_spisit: sfxenum_t = 45;
pub const sfx_bspsit: sfxenum_t = 46;
pub const sfx_kntsit: sfxenum_t = 47;
pub const sfx_vilsit: sfxenum_t = 48;
pub const sfx_mansit: sfxenum_t = 49;
pub const sfx_pesit: sfxenum_t = 50;
pub const sfx_sklatk: sfxenum_t = 51;
pub const sfx_sgtatk: sfxenum_t = 52;
pub const sfx_skepch: sfxenum_t = 53;
pub const sfx_vilatk: sfxenum_t = 54;
pub const sfx_claw: sfxenum_t = 55;
pub const sfx_skeswg: sfxenum_t = 56;
pub const sfx_pldeth: sfxenum_t = 57;
pub const sfx_pdiehi: sfxenum_t = 58;
pub const sfx_podth1: sfxenum_t = 59;
pub const sfx_podth2: sfxenum_t = 60;
pub const sfx_podth3: sfxenum_t = 61;
pub const sfx_bgdth1: sfxenum_t = 62;
pub const sfx_bgdth2: sfxenum_t = 63;
pub const sfx_sgtdth: sfxenum_t = 64;
pub const sfx_cacdth: sfxenum_t = 65;
pub const sfx_skldth: sfxenum_t = 66;
pub const sfx_brsdth: sfxenum_t = 67;
pub const sfx_cybdth: sfxenum_t = 68;
pub const sfx_spidth: sfxenum_t = 69;
pub const sfx_bspdth: sfxenum_t = 70;
pub const sfx_vildth: sfxenum_t = 71;
pub const sfx_kntdth: sfxenum_t = 72;
pub const sfx_pedth: sfxenum_t = 73;
pub const sfx_skedth: sfxenum_t = 74;
pub const sfx_posact: sfxenum_t = 75;
pub const sfx_bgact: sfxenum_t = 76;
pub const sfx_dmact: sfxenum_t = 77;
pub const sfx_bspact: sfxenum_t = 78;
pub const sfx_bspwlk: sfxenum_t = 79;
pub const sfx_vilact: sfxenum_t = 80;
pub const sfx_noway: sfxenum_t = 81;
pub const sfx_barexp: sfxenum_t = 82;
pub const sfx_punch: sfxenum_t = 83;
pub const sfx_hoof: sfxenum_t = 84;
pub const sfx_metal: sfxenum_t = 85;
pub const sfx_chgun: sfxenum_t = 86;
pub const sfx_tink: sfxenum_t = 87;
pub const sfx_bdopn: sfxenum_t = 88;
pub const sfx_bdcls: sfxenum_t = 89;
pub const sfx_itmbk: sfxenum_t = 90;
pub const sfx_flame: sfxenum_t = 91;
pub const sfx_flamst: sfxenum_t = 92;
pub const sfx_getpow: sfxenum_t = 93;
pub const sfx_bospit: sfxenum_t = 94;
pub const sfx_boscub: sfxenum_t = 95;
pub const sfx_bossit: sfxenum_t = 96;
pub const sfx_bospn: sfxenum_t = 97;
pub const sfx_bosdth: sfxenum_t = 98;
pub const sfx_manatk: sfxenum_t = 99;
pub const sfx_mandth: sfxenum_t = 100;
pub const sfx_sssit: sfxenum_t = 101;
pub const sfx_ssdth: sfxenum_t = 102;
pub const sfx_keenpn: sfxenum_t = 103;
pub const sfx_keendt: sfxenum_t = 104;
pub const sfx_skeact: sfxenum_t = 105;
pub const sfx_skesit: sfxenum_t = 106;
pub const sfx_skeatk: sfxenum_t = 107;
pub const sfx_radio: sfxenum_t = 108;
pub const NUMSFX: usize = 109;

const fn mus(name: &'static str) -> musicinfo_t {
    musicinfo_t {
        name,
        lumpnum: 0,
        data: std::ptr::null_mut(),
        handle: 0,
    }
}

pub static mut S_music: [musicinfo_t; NUMMUSIC] = [
    mus(""),
    mus("e1m1"),
    mus("e1m2"),
    mus("e1m3"),
    mus("e1m4"),
    mus("e1m5"),
    mus("e1m6"),
    mus("e1m7"),
    mus("e1m8"),
    mus("e1m9"),
    mus("e2m1"),
    mus("e2m2"),
    mus("e2m3"),
    mus("e2m4"),
    mus("e2m5"),
    mus("e2m6"),
    mus("e2m7"),
    mus("e2m8"),
    mus("e2m9"),
    mus("e3m1"),
    mus("e3m2"),
    mus("e3m3"),
    mus("e3m4"),
    mus("e3m5"),
    mus("e3m6"),
    mus("e3m7"),
    mus("e3m8"),
    mus("e3m9"),
    mus("inter"),
    mus("intro"),
    mus("bunny"),
    mus("victor"),
    mus("introa"),
    mus("runnin"),
    mus("stalks"),
    mus("countd"),
    mus("betwee"),
    mus("doom"),
    mus("the_da"),
    mus("shawn"),
    mus("ddtblu"),
    mus("in_cit"),
    mus("dead"),
    mus("stlks2"),
    mus("theda2"),
    mus("doom2"),
    mus("ddtbl2"),
    mus("runni2"),
    mus("dead2"),
    mus("stlks3"),
    mus("romero"),
    mus("shawn2"),
    mus("messag"),
    mus("count2"),
    mus("ddtbl3"),
    mus("ampie"),
    mus("theda3"),
    mus("adrian"),
    mus("messg2"),
    mus("romer2"),
    mus("tense"),
    mus("shawn3"),
    mus("openin"),
    mus("evil"),
    mus("ultima"),
    mus("read_m"),
    mus("dm2ttl"),
    mus("dm2int"),
];

const fn sfx(
    name: &'static str,
    singularity: i32,
    priority: i32,
    link_idx: isize,
    pitch: i32,
    volume: i32,
) -> sfxinfo_t {
    sfxinfo_t {
        name,
        singularity,
        priority,
        link_idx,
        pitch,
        volume,
        data: std::ptr::null_mut(),
        usefulness: 0,
        lumpnum: 0,
    }
}

pub static mut S_sfx: [sfxinfo_t; NUMSFX] = [
    sfx("none", false_, 0, -1, -1, -1),
    sfx("pistol", false_, 64, -1, -1, -1),
    sfx("shotgn", false_, 64, -1, -1, -1),
    sfx("sgcock", false_, 64, -1, -1, -1),
    sfx("dshtgn", false_, 64, -1, -1, -1),
    sfx("dbopn", false_, 64, -1, -1, -1),
    sfx("dbcls", false_, 64, -1, -1, -1),
    sfx("dbload", false_, 64, -1, -1, -1),
    sfx("plasma", false_, 64, -1, -1, -1),
    sfx("bfg", false_, 64, -1, -1, -1),
    sfx("sawup", false_, 64, -1, -1, -1),
    sfx("sawidl", false_, 118, -1, -1, -1),
    sfx("sawful", false_, 64, -1, -1, -1),
    sfx("sawhit", false_, 64, -1, -1, -1),
    sfx("rlaunc", false_, 64, -1, -1, -1),
    sfx("rxplod", false_, 70, -1, -1, -1),
    sfx("firsht", false_, 70, -1, -1, -1),
    sfx("firxpl", false_, 70, -1, -1, -1),
    sfx("pstart", false_, 100, -1, -1, -1),
    sfx("pstop", false_, 100, -1, -1, -1),
    sfx("doropn", false_, 100, -1, -1, -1),
    sfx("dorcls", false_, 100, -1, -1, -1),
    sfx("stnmov", false_, 119, -1, -1, -1),
    sfx("swtchn", false_, 78, -1, -1, -1),
    sfx("swtchx", false_, 78, -1, -1, -1),
    sfx("plpain", false_, 96, -1, -1, -1),
    sfx("dmpain", false_, 96, -1, -1, -1),
    sfx("popain", false_, 96, -1, -1, -1),
    sfx("vipain", false_, 96, -1, -1, -1),
    sfx("mnpain", false_, 96, -1, -1, -1),
    sfx("pepain", false_, 96, -1, -1, -1),
    sfx("slop", false_, 78, -1, -1, -1),
    sfx("itemup", true_, 78, -1, -1, -1),
    sfx("wpnup", true_, 78, -1, -1, -1),
    sfx("oof", false_, 96, -1, -1, -1),
    sfx("telept", false_, 32, -1, -1, -1),
    sfx("posit1", true_, 98, -1, -1, -1),
    sfx("posit2", true_, 98, -1, -1, -1),
    sfx("posit3", true_, 98, -1, -1, -1),
    sfx("bgsit1", true_, 98, -1, -1, -1),
    sfx("bgsit2", true_, 98, -1, -1, -1),
    sfx("sgtsit", true_, 98, -1, -1, -1),
    sfx("cacsit", true_, 98, -1, -1, -1),
    sfx("brssit", true_, 94, -1, -1, -1),
    sfx("cybsit", true_, 92, -1, -1, -1),
    sfx("spisit", true_, 90, -1, -1, -1),
    sfx("bspsit", true_, 90, -1, -1, -1),
    sfx("kntsit", true_, 90, -1, -1, -1),
    sfx("vilsit", true_, 90, -1, -1, -1),
    sfx("mansit", true_, 90, -1, -1, -1),
    sfx("pesit", true_, 90, -1, -1, -1),
    sfx("sklatk", false_, 70, -1, -1, -1),
    sfx("sgtatk", false_, 70, -1, -1, -1),
    sfx("skepch", false_, 70, -1, -1, -1),
    sfx("vilatk", false_, 70, -1, -1, -1),
    sfx("claw", false_, 70, -1, -1, -1),
    sfx("skeswg", false_, 70, -1, -1, -1),
    sfx("pldeth", false_, 32, -1, -1, -1),
    sfx("pdiehi", false_, 32, -1, -1, -1),
    sfx("podth1", false_, 70, -1, -1, -1),
    sfx("podth2", false_, 70, -1, -1, -1),
    sfx("podth3", false_, 70, -1, -1, -1),
    sfx("bgdth1", false_, 70, -1, -1, -1),
    sfx("bgdth2", false_, 70, -1, -1, -1),
    sfx("sgtdth", false_, 70, -1, -1, -1),
    sfx("cacdth", false_, 70, -1, -1, -1),
    sfx("skldth", false_, 70, -1, -1, -1),
    sfx("brsdth", false_, 32, -1, -1, -1),
    sfx("cybdth", false_, 32, -1, -1, -1),
    sfx("spidth", false_, 32, -1, -1, -1),
    sfx("bspdth", false_, 32, -1, -1, -1),
    sfx("vildth", false_, 32, -1, -1, -1),
    sfx("kntdth", false_, 32, -1, -1, -1),
    sfx("pedth", false_, 32, -1, -1, -1),
    sfx("skedth", false_, 32, -1, -1, -1),
    sfx("posact", true_, 120, -1, -1, -1),
    sfx("bgact", true_, 120, -1, -1, -1),
    sfx("dmact", true_, 120, -1, -1, -1),
    sfx("bspact", true_, 100, -1, -1, -1),
    sfx("bspwlk", true_, 100, -1, -1, -1),
    sfx("vilact", true_, 100, -1, -1, -1),
    sfx("noway", false_, 78, -1, -1, -1),
    sfx("barexp", false_, 60, -1, -1, -1),
    sfx("punch", false_, 64, -1, -1, -1),
    sfx("hoof", false_, 70, -1, -1, -1),
    sfx("metal", false_, 70, -1, -1, -1),
    sfx("chgun", false_, 64, sfx_pistol as isize, 150, 0),
    sfx("tink", false_, 60, -1, -1, -1),
    sfx("bdopn", false_, 100, -1, -1, -1),
    sfx("bdcls", false_, 100, -1, -1, -1),
    sfx("itmbk", false_, 100, -1, -1, -1),
    sfx("flame", false_, 32, -1, -1, -1),
    sfx("flamst", false_, 32, -1, -1, -1),
    sfx("getpow", false_, 60, -1, -1, -1),
    sfx("bospit", false_, 70, -1, -1, -1),
    sfx("boscub", false_, 70, -1, -1, -1),
    sfx("bossit", false_, 70, -1, -1, -1),
    sfx("bospn", false_, 70, -1, -1, -1),
    sfx("bosdth", false_, 70, -1, -1, -1),
    sfx("manatk", false_, 70, -1, -1, -1),
    sfx("mandth", false_, 70, -1, -1, -1),
    sfx("sssit", false_, 70, -1, -1, -1),
    sfx("ssdth", false_, 70, -1, -1, -1),
    sfx("keenpn", false_, 70, -1, -1, -1),
    sfx("keendt", false_, 70, -1, -1, -1),
    sfx("skeact", false_, 70, -1, -1, -1),
    sfx("skesit", false_, 70, -1, -1, -1),
    sfx("skeatk", false_, 70, -1, -1, -1),
    sfx("radio", false_, 60, -1, -1, -1),
];
