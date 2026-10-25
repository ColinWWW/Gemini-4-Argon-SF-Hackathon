// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::fs::File;
use std::io::Read;
use crate::d_event::*;
use crate::d_main::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::m_argv::*;
use crate::m_swap::*;
use crate::r_defs::*;
use crate::r_main::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::{c_strcmp, c_strcpy, c_strlen, c_strncpy, write_c_str};

pub static mut mouseSensitivity: i32 = 5;
pub static mut showMessages: i32 = 1;
pub static mut detailLevel: i32 = 0;
pub static mut screenblocks: i32 = 9;
pub static mut screenSize: i32 = 0;
pub static mut quickSaveSlot: i32 = -1;
pub static mut messageToPrint: i32 = 0;
pub static mut messageString: *const u8 = std::ptr::null();
pub static mut messx: i32 = 0;
pub static mut messy: i32 = 0;
pub static mut messageLastMenuActive: i32 = 0;
pub static mut messageNeedsInput: boolean = 0;
pub static mut messageRoutine: Option<unsafe fn(i32)> = None;

pub const SAVESTRINGSIZE: usize = 24;

pub static mut gammamsg: [&'static [u8]; 5] = [
    b"Gamma correction OFF\0",
    b"Gamma correction level 1\0",
    b"Gamma correction level 2\0",
    b"Gamma correction level 3\0",
    b"Gamma correction level 4\0",
];

pub static mut saveStringEnter: i32 = 0;
pub static mut saveSlot: i32 = 0;
pub static mut saveCharIndex: i32 = 0;
pub static mut saveOldString: [u8; SAVESTRINGSIZE] = [0; SAVESTRINGSIZE];

pub static mut inhelpscreens: boolean = 0;
pub static mut menuactive: boolean = 0;

pub const SKULLXOFF: i32 = -32;
pub const LINEHEIGHT: i32 = 16;

pub static mut savegamestrings: [[u8; SAVESTRINGSIZE]; 10] = [[0; SAVESTRINGSIZE]; 10];
pub static mut endstring: [u8; 160] = [0; 160];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menuitem_t {
    pub status: i16,
    pub name: &'static str,
    pub routine: Option<unsafe fn(i32)>,
    pub alphaKey: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menu_t {
    pub numitems: i16,
    pub prevMenu: *mut menu_t,
    pub menuitems: *mut menuitem_t,
    pub routine: Option<unsafe fn()>,
    pub x: i16,
    pub y: i16,
    pub lastOn: i16,
}

pub static mut itemOn: i16 = 0;
pub static mut skullAnimCounter: i16 = 0;
pub static mut whichSkull: i16 = 0;
pub static mut skullName: [&'static str; 2] = ["M_SKULL1", "M_SKULL2"];
pub static mut currentMenu: *mut menu_t = std::ptr::null_mut();

pub const newgame: usize = 0;
pub const options: usize = 1;
pub const loadgame: usize = 2;
pub const savegame: usize = 3;
pub const readthis: usize = 4;
pub const quitdoom: usize = 5;
pub const main_end: i16 = 6;

pub static mut MainMenu: [menuitem_t; 6] = [
    menuitem_t {
        status: 1,
        name: "M_NGAME",
        routine: Some(M_NewGame),
        alphaKey: b'n',
    },
    menuitem_t {
        status: 1,
        name: "M_OPTION",
        routine: Some(M_Options),
        alphaKey: b'o',
    },
    menuitem_t {
        status: 1,
        name: "M_LOADG",
        routine: Some(M_LoadGame),
        alphaKey: b'l',
    },
    menuitem_t {
        status: 1,
        name: "M_SAVEG",
        routine: Some(M_SaveGame),
        alphaKey: b's',
    },
    menuitem_t {
        status: 1,
        name: "M_RDTHIS",
        routine: Some(M_ReadThis),
        alphaKey: b'r',
    },
    menuitem_t {
        status: 1,
        name: "M_QUITG",
        routine: Some(M_QuitDOOM),
        alphaKey: b'q',
    },
];

pub static mut MainDef: menu_t = menu_t {
    numitems: main_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawMainMenu),
    x: 97,
    y: 64,
    lastOn: 0,
};

pub const ep1: i16 = 0;
pub const ep2: i16 = 1;
pub const ep3: i16 = 2;
pub const ep4: i16 = 3;
pub const ep_end: i16 = 4;

pub static mut EpisodeMenu: [menuitem_t; 4] = [
    menuitem_t {
        status: 1,
        name: "M_EPI1",
        routine: Some(M_Episode),
        alphaKey: b'k',
    },
    menuitem_t {
        status: 1,
        name: "M_EPI2",
        routine: Some(M_Episode),
        alphaKey: b't',
    },
    menuitem_t {
        status: 1,
        name: "M_EPI3",
        routine: Some(M_Episode),
        alphaKey: b'i',
    },
    menuitem_t {
        status: 1,
        name: "M_EPI4",
        routine: Some(M_Episode),
        alphaKey: b't',
    },
];

pub static mut EpiDef: menu_t = menu_t {
    numitems: ep_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawEpisode),
    x: 48,
    y: 63,
    lastOn: ep1,
};

pub const killthings: i16 = 0;
pub const toorough: i16 = 1;
pub const hurtme: i16 = 2;
pub const violence: i16 = 3;
pub const nightmare: i16 = 4;
pub const newg_end: i16 = 5;

pub static mut NewGameMenu: [menuitem_t; 5] = [
    menuitem_t {
        status: 1,
        name: "M_JKILL",
        routine: Some(M_ChooseSkill),
        alphaKey: b'i',
    },
    menuitem_t {
        status: 1,
        name: "M_ROUGH",
        routine: Some(M_ChooseSkill),
        alphaKey: b'h',
    },
    menuitem_t {
        status: 1,
        name: "M_HURT",
        routine: Some(M_ChooseSkill),
        alphaKey: b'h',
    },
    menuitem_t {
        status: 1,
        name: "M_ULTRA",
        routine: Some(M_ChooseSkill),
        alphaKey: b'u',
    },
    menuitem_t {
        status: 1,
        name: "M_NMARE",
        routine: Some(M_ChooseSkill),
        alphaKey: b'n',
    },
];

pub static mut NewDef: menu_t = menu_t {
    numitems: newg_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawNewGame),
    x: 48,
    y: 63,
    lastOn: hurtme,
};

pub const endgame: i32 = 0;
pub const messages: i32 = 1;
pub const detail: i32 = 2;
pub const scrnsize: i32 = 3;
pub const option_empty1: i32 = 4;
pub const mousesens: i32 = 5;
pub const option_empty2: i32 = 6;
pub const soundvol: i32 = 7;
pub const opt_end: i16 = 8;

pub static mut OptionsMenu: [menuitem_t; 8] = [
    menuitem_t {
        status: 1,
        name: "M_ENDGAM",
        routine: Some(M_EndGame),
        alphaKey: b'e',
    },
    menuitem_t {
        status: 1,
        name: "M_MESSG",
        routine: Some(M_ChangeMessages),
        alphaKey: b'm',
    },
    menuitem_t {
        status: 1,
        name: "M_DETAIL",
        routine: Some(M_ChangeDetail),
        alphaKey: b'g',
    },
    menuitem_t {
        status: 2,
        name: "M_SCRNSZ",
        routine: Some(M_SizeDisplay),
        alphaKey: b's',
    },
    menuitem_t {
        status: -1,
        name: "",
        routine: None,
        alphaKey: 0,
    },
    menuitem_t {
        status: 2,
        name: "M_MSENS",
        routine: Some(M_ChangeSensitivity),
        alphaKey: b'm',
    },
    menuitem_t {
        status: -1,
        name: "",
        routine: None,
        alphaKey: 0,
    },
    menuitem_t {
        status: 1,
        name: "M_SVOL",
        routine: Some(M_Sound),
        alphaKey: b's',
    },
];

pub static mut OptionsDef: menu_t = menu_t {
    numitems: opt_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawOptions),
    x: 60,
    y: 37,
    lastOn: 0,
};

pub const read1_end: i16 = 1;
pub static mut ReadMenu1: [menuitem_t; 1] = [menuitem_t {
    status: 1,
    name: "",
    routine: Some(M_ReadThis2),
    alphaKey: 0,
}];

pub static mut ReadDef1: menu_t = menu_t {
    numitems: read1_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawReadThis1),
    x: 280,
    y: 185,
    lastOn: 0,
};

pub const read2_end: i16 = 1;
pub static mut ReadMenu2: [menuitem_t; 1] = [menuitem_t {
    status: 1,
    name: "",
    routine: Some(M_FinishReadThis),
    alphaKey: 0,
}];

pub static mut ReadDef2: menu_t = menu_t {
    numitems: read2_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawReadThis2),
    x: 330,
    y: 175,
    lastOn: 0,
};

pub const sfx_vol: i32 = 0;
pub const sfx_empty1: i32 = 1;
pub const music_vol: i32 = 2;
pub const sfx_empty2: i32 = 3;
pub const sound_end: i16 = 4;

pub static mut SoundMenu: [menuitem_t; 4] = [
    menuitem_t {
        status: 2,
        name: "M_SFXVOL",
        routine: Some(M_SfxVol),
        alphaKey: b's',
    },
    menuitem_t {
        status: -1,
        name: "",
        routine: None,
        alphaKey: 0,
    },
    menuitem_t {
        status: 2,
        name: "M_MUSVOL",
        routine: Some(M_MusicVol),
        alphaKey: b'm',
    },
    menuitem_t {
        status: -1,
        name: "",
        routine: None,
        alphaKey: 0,
    },
];

pub static mut SoundDef: menu_t = menu_t {
    numitems: sound_end,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawSound),
    x: 80,
    y: 64,
    lastOn: 0,
};

pub const load_end: usize = 6;

pub static mut LoadMenu: [menuitem_t; 6] = [
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_LoadSelect),
        alphaKey: b'1',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_LoadSelect),
        alphaKey: b'2',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_LoadSelect),
        alphaKey: b'3',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_LoadSelect),
        alphaKey: b'4',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_LoadSelect),
        alphaKey: b'5',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_LoadSelect),
        alphaKey: b'6',
    },
];

pub static mut LoadDef: menu_t = menu_t {
    numitems: load_end as i16,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawLoad),
    x: 80,
    y: 54,
    lastOn: 0,
};

pub static mut SaveMenu: [menuitem_t; 6] = [
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_SaveSelect),
        alphaKey: b'1',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_SaveSelect),
        alphaKey: b'2',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_SaveSelect),
        alphaKey: b'3',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_SaveSelect),
        alphaKey: b'4',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_SaveSelect),
        alphaKey: b'5',
    },
    menuitem_t {
        status: 1,
        name: "",
        routine: Some(M_SaveSelect),
        alphaKey: b'6',
    },
];

pub static mut SaveDef: menu_t = menu_t {
    numitems: load_end as i16,
    prevMenu: std::ptr::null_mut(),
    menuitems: std::ptr::null_mut(),
    routine: Some(M_DrawSave),
    x: 80,
    y: 54,
    lastOn: 0,
};

pub unsafe fn M_ReadSaveStrings() {
    for i in 0..load_end {
        let name = if M_CheckParmStr("-cdrom") != 0 {
            format!("c:\\doomdata\\{}{}.dsg", SAVEGAMENAME, i)
        } else {
            format!("{}{}.dsg", SAVEGAMENAME, i)
        };

        match File::open(&name) {
            Ok(mut f) => {
                let _ = f.read(&mut savegamestrings[i]);
                LoadMenu[i].status = 1;
            }
            Err(_) => {
                write_c_str(savegamestrings[i].as_mut_ptr(), EMPTYSTRING);
                LoadMenu[i].status = 0;
            }
        }
    }
}

pub unsafe fn M_DrawLoad() {
    V_DrawPatchDirect(
        72,
        28,
        0,
        W_CacheLumpNameStr("M_LOADG", PU_CACHE) as *mut patch_t,
    );
    for i in 0..load_end {
        M_DrawSaveLoadBorder(LoadDef.x as i32, (LoadDef.y as i32) + LINEHEIGHT * (i as i32));
        M_WriteText(
            LoadDef.x as i32,
            (LoadDef.y as i32) + LINEHEIGHT * (i as i32),
            savegamestrings[i].as_ptr(),
        );
    }
}

pub unsafe fn M_DrawSaveLoadBorder(mut x: i32, y: i32) {
    V_DrawPatchDirect(
        x - 8,
        y + 7,
        0,
        W_CacheLumpNameStr("M_LSLEFT", PU_CACHE) as *mut patch_t,
    );

    for _ in 0..24 {
        V_DrawPatchDirect(
            x,
            y + 7,
            0,
            W_CacheLumpNameStr("M_LSCNTR", PU_CACHE) as *mut patch_t,
        );
        x += 8;
    }

    V_DrawPatchDirect(
        x,
        y + 7,
        0,
        W_CacheLumpNameStr("M_LSRGHT", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_LoadSelect(choice: i32) {
    let mut name = [0u8; 256];
    let s = if M_CheckParmStr("-cdrom") != 0 {
        format!("c:\\doomdata\\{}{}.dsg", SAVEGAMENAME, choice)
    } else {
        format!("{}{}.dsg", SAVEGAMENAME, choice)
    };
    write_c_str(name.as_mut_ptr(), &s);
    G_LoadGame(name.as_ptr());
    M_ClearMenus();
}

pub unsafe fn M_LoadGame(_choice: i32) {
    if netgame != 0 {
        M_StartMessage(
            b"you can't do load while in a net game!\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        return;
    }
    M_SetupNextMenu(&mut LoadDef);
    M_ReadSaveStrings();
}

pub unsafe fn M_DrawSave() {
    V_DrawPatchDirect(
        72,
        28,
        0,
        W_CacheLumpNameStr("M_SAVEG", PU_CACHE) as *mut patch_t,
    );
    for i in 0..load_end {
        M_DrawSaveLoadBorder(LoadDef.x as i32, (LoadDef.y as i32) + LINEHEIGHT * (i as i32));
        M_WriteText(
            LoadDef.x as i32,
            (LoadDef.y as i32) + LINEHEIGHT * (i as i32),
            savegamestrings[i].as_ptr(),
        );
    }

    if saveStringEnter != 0 {
        let i = M_StringWidth(savegamestrings[saveSlot as usize].as_ptr());
        M_WriteText(
            (LoadDef.x as i32) + i,
            (LoadDef.y as i32) + LINEHEIGHT * saveSlot,
            b"_\0".as_ptr(),
        );
    }
}

pub unsafe fn M_DoSave(slot: i32) {
    G_SaveGame(slot, savegamestrings[slot as usize].as_ptr());
    M_ClearMenus();
    if quickSaveSlot == -2 {
        quickSaveSlot = slot;
    }
}

pub unsafe fn M_SaveSelect(choice: i32) {
    saveStringEnter = 1;
    saveSlot = choice;
    c_strcpy(
        saveOldString.as_mut_ptr(),
        savegamestrings[choice as usize].as_ptr(),
    );
    if c_strcmp(
        savegamestrings[choice as usize].as_ptr(),
        b"empty slot\0".as_ptr(),
    ) == 0
    {
        savegamestrings[choice as usize][0] = 0;
    }
    saveCharIndex = c_strlen(savegamestrings[choice as usize].as_ptr()) as i32;
}

pub unsafe fn M_SaveGame(_choice: i32) {
    if usergame == 0 {
        M_StartMessage(
            b"you can't save if you aren't playing!\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        return;
    }
    if gamestate != GS_LEVEL {
        return;
    }
    M_SetupNextMenu(&mut SaveDef);
    M_ReadSaveStrings();
}

pub static mut tempstring: [u8; 80] = [0; 80];

pub unsafe fn M_QuickSaveResponse(ch: i32) {
    if ch == b'y' as i32 {
        M_DoSave(quickSaveSlot);
        S_StartSound(std::ptr::null_mut(), sfx_swtchx);
    }
}

pub unsafe fn M_QuickSave() {
    if usergame == 0 {
        S_StartSound(std::ptr::null_mut(), sfx_oof);
        return;
    }
    if gamestate != GS_LEVEL {
        return;
    }
    if quickSaveSlot < 0 {
        M_StartControlPanel();
        M_ReadSaveStrings();
        M_SetupNextMenu(&mut SaveDef);
        quickSaveSlot = -2;
        return;
    }
    let s = format!(
        "quicksave over your game named\n\n'{}'?\n\npress y or n.",
        crate::c_str(savegamestrings[quickSaveSlot as usize].as_ptr())
    );
    write_c_str(tempstring.as_mut_ptr(), &s);
    M_StartMessage(tempstring.as_ptr(), Some(M_QuickSaveResponse), true_);
}

pub unsafe fn M_QuickLoadResponse(ch: i32) {
    if ch == b'y' as i32 {
        M_LoadSelect(quickSaveSlot);
        S_StartSound(std::ptr::null_mut(), sfx_swtchx);
    }
}

pub unsafe fn M_QuickLoad() {
    if netgame != 0 {
        M_StartMessage(
            b"you can't quickload during a netgame!\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        return;
    }
    if quickSaveSlot < 0 {
        M_StartMessage(
            b"you haven't picked a quicksave slot yet!\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        return;
    }
    let s = format!(
        "do you want to quickload the game named\n\n'{}'?\n\npress y or n.",
        crate::c_str(savegamestrings[quickSaveSlot as usize].as_ptr())
    );
    write_c_str(tempstring.as_mut_ptr(), &s);
    M_StartMessage(tempstring.as_ptr(), Some(M_QuickLoadResponse), true_);
}

pub unsafe fn M_DrawReadThis1() {
    inhelpscreens = true_;
    match gamemode {
        commercial => {
            V_DrawPatchDirect(
                0,
                0,
                0,
                W_CacheLumpNameStr("HELP", PU_CACHE) as *mut patch_t,
            );
        }
        shareware | registered | retail => {
            V_DrawPatchDirect(
                0,
                0,
                0,
                W_CacheLumpNameStr("HELP1", PU_CACHE) as *mut patch_t,
            );
        }
        _ => {}
    }
}

pub unsafe fn M_DrawReadThis2() {
    inhelpscreens = true_;
    match gamemode {
        retail | commercial => {
            V_DrawPatchDirect(
                0,
                0,
                0,
                W_CacheLumpNameStr("CREDIT", PU_CACHE) as *mut patch_t,
            );
        }
        shareware | registered => {
            V_DrawPatchDirect(
                0,
                0,
                0,
                W_CacheLumpNameStr("HELP2", PU_CACHE) as *mut patch_t,
            );
        }
        _ => {}
    }
}

pub unsafe fn M_DrawSound() {
    V_DrawPatchDirect(
        60,
        38,
        0,
        W_CacheLumpNameStr("M_SVOL", PU_CACHE) as *mut patch_t,
    );
    M_DrawThermo(
        SoundDef.x as i32,
        (SoundDef.y as i32) + LINEHEIGHT * (sfx_vol + 1),
        16,
        snd_SfxVolume,
    );
    M_DrawThermo(
        SoundDef.x as i32,
        (SoundDef.y as i32) + LINEHEIGHT * (music_vol + 1),
        16,
        snd_MusicVolume,
    );
}

pub unsafe fn M_Sound(_choice: i32) {
    M_SetupNextMenu(&mut SoundDef);
}

pub unsafe fn M_SfxVol(choice: i32) {
    match choice {
        0 => {
            if snd_SfxVolume != 0 {
                snd_SfxVolume -= 1;
            }
        }
        1 => {
            if snd_SfxVolume < 15 {
                snd_SfxVolume += 1;
            }
        }
        _ => {}
    }
    S_SetSfxVolume(snd_SfxVolume);
}

pub unsafe fn M_MusicVol(choice: i32) {
    match choice {
        0 => {
            if snd_MusicVolume != 0 {
                snd_MusicVolume -= 1;
            }
        }
        1 => {
            if snd_MusicVolume < 15 {
                snd_MusicVolume += 1;
            }
        }
        _ => {}
    }
    S_SetMusicVolume(snd_MusicVolume);
}

pub unsafe fn M_DrawMainMenu() {
    V_DrawPatchDirect(
        94,
        2,
        0,
        W_CacheLumpNameStr("M_DOOM", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_DrawNewGame() {
    V_DrawPatchDirect(
        96,
        14,
        0,
        W_CacheLumpNameStr("M_NEWG", PU_CACHE) as *mut patch_t,
    );
    V_DrawPatchDirect(
        54,
        38,
        0,
        W_CacheLumpNameStr("M_SKILL", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_NewGame(_choice: i32) {
    if netgame != 0 && demoplayback == 0 {
        M_StartMessage(
            b"you can't start a new game\nwhile in a network game.\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        return;
    }

    if gamemode == commercial {
        M_SetupNextMenu(&mut NewDef);
    } else {
        M_SetupNextMenu(&mut EpiDef);
    }
}

pub static mut epi: i32 = 0;

pub unsafe fn M_DrawEpisode() {
    V_DrawPatchDirect(
        54,
        38,
        0,
        W_CacheLumpNameStr("M_EPISOD", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_VerifyNightmare(ch: i32) {
    if ch != b'y' as i32 {
        return;
    }
    G_DeferedInitNew(nightmare as i32, epi + 1, 1);
    M_ClearMenus();
}

pub unsafe fn M_ChooseSkill(choice: i32) {
    if choice == (nightmare as i32) {
        M_StartMessage(
            b"are you sure? this skill level\nisn't even remotely fair.\n\npress y or n.\0"
                .as_ptr(),
            Some(M_VerifyNightmare),
            true_,
        );
        return;
    }
    G_DeferedInitNew(choice, epi + 1, 1);
    M_ClearMenus();
}

pub unsafe fn M_Episode(mut choice: i32) {
    if gamemode == shareware && choice != 0 {
        M_StartMessage(
            b"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        M_SetupNextMenu(&mut ReadDef1);
        return;
    }

    if gamemode == registered && choice > 2 {
        eprintln!("M_Episode: 4th episode requires UltimateDOOM");
        choice = 0;
    }

    epi = choice;
    M_SetupNextMenu(&mut NewDef);
}

pub static mut detailNames: [&'static str; 2] = ["M_GDHIGH", "M_GDLOW"];
pub static mut msgNames: [&'static str; 2] = ["M_MSGOFF", "M_MSGON"];

pub unsafe fn M_DrawOptions() {
    V_DrawPatchDirect(
        108,
        15,
        0,
        W_CacheLumpNameStr("M_OPTTTL", PU_CACHE) as *mut patch_t,
    );

    V_DrawPatchDirect(
        (OptionsDef.x as i32) + 175,
        (OptionsDef.y as i32) + LINEHEIGHT * detail,
        0,
        W_CacheLumpNameStr(detailNames[detailLevel as usize], PU_CACHE) as *mut patch_t,
    );

    V_DrawPatchDirect(
        (OptionsDef.x as i32) + 120,
        (OptionsDef.y as i32) + LINEHEIGHT * messages,
        0,
        W_CacheLumpNameStr(msgNames[showMessages as usize], PU_CACHE) as *mut patch_t,
    );

    M_DrawThermo(
        OptionsDef.x as i32,
        (OptionsDef.y as i32) + LINEHEIGHT * (mousesens + 1),
        10,
        mouseSensitivity,
    );

    M_DrawThermo(
        OptionsDef.x as i32,
        (OptionsDef.y as i32) + LINEHEIGHT * (scrnsize + 1),
        9,
        screenSize,
    );
}

pub unsafe fn M_Options(_choice: i32) {
    M_SetupNextMenu(&mut OptionsDef);
}

pub unsafe fn M_ChangeMessages(_choice: i32) {
    showMessages = 1 - showMessages;
    if showMessages == 0 {
        players[consoleplayer as usize].message = b"Messages OFF\0".as_ptr();
    } else {
        players[consoleplayer as usize].message = b"Messages ON\0".as_ptr();
    }
    message_dontfuckwithme = true_;
}

pub unsafe fn M_EndGameResponse(ch: i32) {
    if ch != b'y' as i32 {
        return;
    }
    (*currentMenu).lastOn = itemOn;
    M_ClearMenus();
    D_StartTitle();
}

pub unsafe fn M_EndGame(_choice: i32) {
    if usergame == 0 {
        S_StartSound(std::ptr::null_mut(), sfx_oof);
        return;
    }
    if netgame != 0 {
        M_StartMessage(
            b"you can't end a netgame!\n\npress a key.\0".as_ptr(),
            None,
            false_,
        );
        return;
    }
    M_StartMessage(
        b"are you sure you want to end the game?\n\npress y or n.\0".as_ptr(),
        Some(M_EndGameResponse),
        true_,
    );
}

pub unsafe fn M_Read氛围This(_choice: i32) {
    M_SetupNextMenu(&mut ReadDef1);
}

pub unsafe fn M_ReadThis(_choice: i32) {
    M_SetupNextMenu(&mut ReadDef1);
}

pub unsafe fn M_ReadThis2(_choice: i32) {
    M_SetupNextMenu(&mut ReadDef2);
}

pub unsafe fn M_FinishReadThis(_choice: i32) {
    M_SetupNextMenu(&mut MainDef);
}

pub static mut quitsounds: [i32; 8] = [
    sfx_pldeth, sfx_dmpain, sfx_popain, sfx_slop, sfx_telept, sfx_posit1, sfx_posit3, sfx_sgtatk,
];

pub static mut quitsounds2: [i32; 8] = [
    sfx_vilact, sfx_getpow, sfx_boscub, sfx_slop, sfx_skeswg, sfx_kntdth, sfx_bspact, sfx_sgtatk,
];

pub unsafe fn M_QuitResponse(ch: i32) {
    if ch != b'y' as i32 {
        return;
    }
    if netgame == 0 {
        if gamemode == commercial {
            S_StartSound(
                std::ptr::null_mut(),
                quitsounds2[((gametic >> 2) & 7) as usize],
            );
        } else {
            S_StartSound(
                std::ptr::null_mut(),
                quitsounds[((gametic >> 2) & 7) as usize],
            );
        }
        I_WaitVBL(105);
    }
    I_Quit();
}

pub unsafe fn M_QuitDOOM(_choice: i32) {
    let msg = if language != english {
        endmsg[0]
    } else {
        endmsg[((gametic as usize) % (NUM_QUITMESSAGES - 2)) + 1]
    };
    let s = format!("{}\n\n{}", msg, DOSY);
    write_c_str(endstring.as_mut_ptr(), &s);
    M_StartMessage(endstring.as_ptr(), Some(M_QuitResponse), true_);
}

pub unsafe fn M_ChangeSensitivity(choice: i32) {
    match choice {
        0 => {
            if mouseSensitivity != 0 {
                mouseSensitivity -= 1;
            }
        }
        1 => {
            if mouseSensitivity < 9 {
                mouseSensitivity += 1;
            }
        }
        _ => {}
    }
}

pub unsafe fn M_ChangeDetail(_choice: i32) {
    detailLevel = 1 - detailLevel;
    eprintln!("M_ChangeDetail: low detail mode n.a.");
}

pub unsafe fn M_SizeDisplay(choice: i32) {
    match choice {
        0 => {
            if screenSize > 0 {
                screenblocks -= 1;
                screenSize -= 1;
            }
        }
        1 => {
            if screenSize < 8 {
                screenblocks += 1;
                screenSize += 1;
            }
        }
        _ => {}
    }
    R_SetViewSize(screenblocks, detailLevel);
}

pub unsafe fn M_DrawThermo(x: i32, y: i32, thermWidth: i32, thermDot: i32) {
    let mut xx = x;
    V_DrawPatchDirect(
        xx,
        y,
        0,
        W_CacheLumpNameStr("M_THERML", PU_CACHE) as *mut patch_t,
    );
    xx += 8;
    for _ in 0..thermWidth {
        V_DrawPatchDirect(
            xx,
            y,
            0,
            W_CacheLumpNameStr("M_THERMM", PU_CACHE) as *mut patch_t,
        );
        xx += 8;
    }
    V_DrawPatchDirect(
        xx,
        y,
        0,
        W_CacheLumpNameStr("M_THERMR", PU_CACHE) as *mut patch_t,
    );
    V_DrawPatchDirect(
        (x + 8) + thermDot * 8,
        y,
        0,
        W_CacheLumpNameStr("M_THERMO", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_DrawEmptyCell(menu: *mut menu_t, item: i32) {
    V_DrawPatchDirect(
        ((*menu).x as i32) - 10,
        ((*menu).y as i32) + item * LINEHEIGHT - 1,
        0,
        W_CacheLumpNameStr("M_CELL1", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_DrawSelCell(menu: *mut menu_t, item: i32) {
    V_DrawPatchDirect(
        ((*menu).x as i32) - 10,
        ((*menu).y as i32) + item * LINEHEIGHT - 1,
        0,
        W_CacheLumpNameStr("M_CELL2", PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_StartMessage(
    string: *const u8,
    routine: Option<unsafe fn(i32)>,
    input: boolean,
) {
    messageLastMenuActive = menuactive;
    messageToPrint = 1;
    messageString = string;
    messageRoutine = routine;
    messageNeedsInput = input;
    menuactive = true_;
}

pub unsafe fn M_StopMessage() {
    menuactive = messageLastMenuActive;
    messageToPrint = 0;
}

pub unsafe fn M_StringWidth(string: *const u8) -> i32 {
    let mut w = 0;
    let len = c_strlen(string);
    for i in 0..len {
        let c = ((*string.add(i)).to_ascii_uppercase() as i32) - (HU_FONTSTART as i32);
        if c < 0 || (c as usize) >= HU_FONTSIZE {
            w += 4;
        } else {
            w += SHORT((*hu_font[c as usize]).width) as i32;
        }
    }
    w
}

pub unsafe fn M_StringHeight(string: *const u8) -> i32 {
    let height = SHORT((*hu_font[0]).height) as i32;
    let mut h = height;
    let len = c_strlen(string);
    for i in 0..len {
        if *string.add(i) == b'\n' {
            h += height;
        }
    }
    h
}

pub unsafe fn M_WriteText(x: i32, y: i32, string: *const u8) {
    let mut ch = string;
    let mut cx = x;
    let mut cy = y;

    loop {
        let c = *ch;
        ch = ch.add(1);
        if c == 0 {
            break;
        }
        if c == b'\n' {
            cx = x;
            cy += 12;
            continue;
        }

        let glyph = (c.to_ascii_uppercase() as i32) - (HU_FONTSTART as i32);
        if glyph < 0 || (glyph as usize) >= HU_FONTSIZE {
            cx += 4;
            continue;
        }

        let w = SHORT((*hu_font[glyph as usize]).width) as i32;
        if cx + w > SCREENWIDTH {
            break;
        }
        V_DrawPatchDirect(cx, cy, 0, hu_font[glyph as usize]);
        cx += w;
    }
}

static mut joywait: i32 = 0;
static mut mousewait: i32 = 0;
static mut m_mousey: i32 = 0;
static mut lasty: i32 = 0;
static mut m_mousex: i32 = 0;
static mut lastx: i32 = 0;

pub unsafe fn M_Responder(ev: *mut event_t) -> boolean {
    let mut ch = -1i32;

    if (*ev).type_ == ev_joystick && joywait < I_GetTime() {
        if (*ev).data3 == -1 {
            ch = KEY_UPARROW;
            joywait = I_GetTime() + 5;
        } else if (*ev).data3 == 1 {
            ch = KEY_DOWNARROW;
            joywait = I_GetTime() + 5;
        }

        if (*ev).data2 == -1 {
            ch = KEY_LEFTARROW;
            joywait = I_GetTime() + 2;
        } else if (*ev).data2 == 1 {
            ch = KEY_RIGHTARROW;
            joywait = I_GetTime() + 2;
        }

        if (*ev).data1 & 1 != 0 {
            ch = KEY_ENTER;
            joywait = I_GetTime() + 5;
        }
        if (*ev).data1 & 2 != 0 {
            ch = KEY_BACKSPACE;
            joywait = I_GetTime() + 5;
        }
    } else if (*ev).type_ == ev_mouse && mousewait < I_GetTime() {
        m_mousey += (*ev).data3;
        if m_mousey < lasty - 30 {
            ch = KEY_DOWNARROW;
            mousewait = I_GetTime() + 5;
            lasty -= 30;
            m_mousey = lasty;
        } else if m_mousey > lasty + 30 {
            ch = KEY_UPARROW;
            mousewait = I_GetTime() + 5;
            lasty += 30;
            m_mousey = lasty;
        }

        m_mousex += (*ev).data2;
        if m_mousex < lastx - 30 {
            ch = KEY_LEFTARROW;
            mousewait = I_GetTime() + 5;
            lastx -= 30;
            m_mousex = lastx;
        } else if m_mousex > lastx + 30 {
            ch = KEY_RIGHTARROW;
            mousewait = I_GetTime() + 5;
            lastx += 30;
            m_mousex = lastx;
        }

        if (*ev).data1 & 1 != 0 {
            ch = KEY_ENTER;
            mousewait = I_GetTime() + 15;
        }
        if (*ev).data1 & 2 != 0 {
            ch = KEY_BACKSPACE;
            mousewait = I_GetTime() + 15;
        }
    } else if (*ev).type_ == ev_keydown {
        ch = (*ev).data1;
    }

    if ch == -1 {
        return false_;
    }

    if saveStringEnter != 0 {
        match ch {
            KEY_BACKSPACE => {
                if saveCharIndex > 0 {
                    saveCharIndex -= 1;
                    savegamestrings[saveSlot as usize][saveCharIndex as usize] = 0;
                }
            }
            KEY_ESCAPE => {
                saveStringEnter = 0;
                c_strcpy(
                    savegamestrings[saveSlot as usize].as_mut_ptr(),
                    saveOldString.as_ptr(),
                );
            }
            KEY_ENTER => {
                saveStringEnter = 0;
                if savegamestrings[saveSlot as usize][0] != 0 {
                    M_DoSave(saveSlot);
                }
            }
            _ => {
                let uch = (ch as u8).to_ascii_uppercase();
                if uch != 32 {
                    let diff = (uch as i32) - (HU_FONTSTART as i32);
                    if diff < 0 || (diff as usize) >= HU_FONTSIZE {
                        return true_;
                    }
                }
                if uch >= 32
                    && uch <= 127
                    && (saveCharIndex as usize) < SAVESTRINGSIZE - 1
                    && M_StringWidth(savegamestrings[saveSlot as usize].as_ptr())
                        < ((SAVESTRINGSIZE - 2) * 8) as i32
                {
                    savegamestrings[saveSlot as usize][saveCharIndex as usize] = uch;
                    saveCharIndex += 1;
                    savegamestrings[saveSlot as usize][saveCharIndex as usize] = 0;
                }
            }
        }
        return true_;
    }

    if messageToPrint != 0 {
        if messageNeedsInput == true_
            && !(ch == b' ' as i32 || ch == b'n' as i32 || ch == b'y' as i32 || ch == KEY_ESCAPE)
        {
            return false_;
        }

        menuactive = messageLastMenuActive;
        messageToPrint = 0;
        if let Some(routine) = messageRoutine {
            routine(ch);
        }

        menuactive = false_;
        S_StartSound(std::ptr::null_mut(), sfx_swtchx);
        return true_;
    }

    if devparm != 0 && ch == KEY_F1 {
        G_ScreenShot();
        return true_;
    }

    if menuactive == 0 {
        match ch {
            KEY_MINUS => {
                if automapactive != 0 || chat_on != 0 {
                    return false_;
                }
                M_SizeDisplay(0);
                S_StartSound(std::ptr::null_mut(), sfx_stnmov);
                return true_;
            }
            KEY_EQUALS => {
                if automapactive != 0 || chat_on != 0 {
                    return false_;
                }
                M_SizeDisplay(1);
                S_StartSound(std::ptr::null_mut(), sfx_stnmov);
                return true_;
            }
            KEY_F1 => {
                M_StartControlPanel();
                if gamemode == retail {
                    currentMenu = &mut ReadDef2;
                } else {
                    currentMenu = &mut ReadDef1;
                }
                itemOn = 0;
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                return true_;
            }
            KEY_F2 => {
                M_StartControlPanel();
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                M_SaveGame(0);
                return true_;
            }
            KEY_F3 => {
                M_StartControlPanel();
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                M_LoadGame(0);
                return true_;
            }
            KEY_F4 => {
                M_StartControlPanel();
                currentMenu = &mut SoundDef;
                itemOn = sfx_vol as i16;
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                return true_;
            }
            KEY_F5 => {
                M_ChangeDetail(0);
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                return true_;
            }
            KEY_F6 => {
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                M_QuickSave();
                return true_;
            }
            KEY_F7 => {
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                M_EndGame(0);
                return true_;
            }
            KEY_F8 => {
                M_ChangeMessages(0);
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                return true_;
            }
            KEY_F9 => {
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                M_QuickLoad();
                return true_;
            }
            KEY_F10 => {
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
                M_QuitDOOM(0);
                return true_;
            }
            KEY_F11 => {
                usegamma += 1;
                if usegamma > 4 {
                    usegamma = 0;
                }
                players[consoleplayer as usize].message = gammamsg[usegamma as usize].as_ptr();
                I_SetPalette(W_CacheLumpNameStr("PLAYPAL", PU_CACHE) as *mut byte);
                return true_;
            }
            _ => {}
        }
    }

    if menuactive == 0 {
        if ch == KEY_ESCAPE {
            M_StartControlPanel();
            S_StartSound(std::ptr::null_mut(), sfx_swtchn);
            return true_;
        }
        return false_;
    }

    match ch {
        KEY_DOWNARROW => {
            loop {
                if itemOn + 1 > (*currentMenu).numitems - 1 {
                    itemOn = 0;
                } else {
                    itemOn += 1;
                }
                S_StartSound(std::ptr::null_mut(), sfx_pstop);
                if (*(*currentMenu).menuitems.add(itemOn as usize)).status != -1 {
                    break;
                }
            }
            return true_;
        }
        KEY_UPARROW => {
            loop {
                if itemOn == 0 {
                    itemOn = (*currentMenu).numitems - 1;
                } else {
                    itemOn -= 1;
                }
                S_StartSound(std::ptr::null_mut(), sfx_pstop);
                if (*(*currentMenu).menuitems.add(itemOn as usize)).status != -1 {
                    break;
                }
            }
            return true_;
        }
        KEY_LEFTARROW => {
            let item = (*currentMenu).menuitems.add(itemOn as usize);
            if (*item).routine.is_some() && (*item).status == 2 {
                S_StartSound(std::ptr::null_mut(), sfx_stnmov);
                if let Some(r) = (*item).routine {
                    r(0);
                }
            }
            return true_;
        }
        KEY_RIGHTARROW => {
            let item = (*currentMenu).menuitems.add(itemOn as usize);
            if (*item).routine.is_some() && (*item).status == 2 {
                S_StartSound(std::ptr::null_mut(), sfx_stnmov);
                if let Some(r) = (*item).routine {
                    r(1);
                }
            }
            return true_;
        }
        KEY_ENTER => {
            let item = (*currentMenu).menuitems.add(itemOn as usize);
            if (*item).routine.is_some() && (*item).status != 0 {
                (*currentMenu).lastOn = itemOn;
                if (*item).status == 2 {
                    if let Some(r) = (*item).routine {
                        r(1);
                    }
                    S_StartSound(std::ptr::null_mut(), sfx_stnmov);
                } else {
                    if let Some(r) = (*item).routine {
                        r(itemOn as i32);
                    }
                    S_StartSound(std::ptr::null_mut(), sfx_pistol);
                }
            }
            return true_;
        }
        KEY_ESCAPE => {
            (*currentMenu).lastOn = itemOn;
            M_ClearMenus();
            S_StartSound(std::ptr::null_mut(), sfx_swtchx);
            return true_;
        }
        KEY_BACKSPACE => {
            (*currentMenu).lastOn = itemOn;
            if !(*currentMenu).prevMenu.is_null() {
                currentMenu = (*currentMenu).prevMenu;
                itemOn = (*currentMenu).lastOn;
                S_StartSound(std::ptr::null_mut(), sfx_swtchn);
            }
            return true_;
        }
        _ => {
            for i in ((itemOn + 1) as usize)..((*currentMenu).numitems as usize) {
                if (*(*currentMenu).menuitems.add(i)).alphaKey as i32 == ch {
                    itemOn = i as i16;
                    S_StartSound(std::ptr::null_mut(), sfx_pstop);
                    return true_;
                }
            }
            for i in 0..=(itemOn as usize) {
                if (*(*currentMenu).menuitems.add(i)).alphaKey as i32 == ch {
                    itemOn = i as i16;
                    S_StartSound(std::ptr::null_mut(), sfx_pstop);
                    return true_;
                }
            }
        }
    }

    false_
}

pub unsafe fn M_StartControlPanel() {
    if menuactive != 0 {
        return;
    }
    menuactive = 1;
    currentMenu = &mut MainDef;
    itemOn = (*currentMenu).lastOn;
}

pub unsafe fn M_Drawer() {
    inhelpscreens = false_;

    if messageToPrint != 0 {
        let mut start = 0usize;
        let mut y = 100 - M_StringHeight(messageString) / 2;
        while *messageString.add(start) != 0 {
            let mut string = [0u8; 40];
            let rem_len = c_strlen(messageString.add(start));
            let mut i = 0;
            while i < rem_len {
                if *messageString.add(start + i) == b'\n' {
                    c_strncpy(string.as_mut_ptr(), messageString.add(start), i);
                    start += i + 1;
                    break;
                }
                i += 1;
            }
            if i == rem_len {
                c_strcpy(string.as_mut_ptr(), messageString.add(start));
                start += i;
            }

            let x = 160 - M_StringWidth(string.as_ptr()) / 2;
            M_WriteText(x, y, string.as_ptr());
            y += SHORT((*hu_font[0]).height) as i32;
        }
        return;
    }

    if menuactive == 0 {
        return;
    }

    if let Some(routine) = (*currentMenu).routine {
        routine();
    }

    let x = (*currentMenu).x as i32;
    let mut y = (*currentMenu).y as i32;
    let max = (*currentMenu).numitems as usize;

    for i in 0..max {
        let item = (*currentMenu).menuitems.add(i);
        if !(&(*item).name).is_empty() {
            V_DrawPatchDirect(
                x,
                y,
                0,
                W_CacheLumpNameStr((*item).name, PU_CACHE) as *mut patch_t,
            );
        }
        y += LINEHEIGHT;
    }

    V_DrawPatchDirect(
        x + SKULLXOFF,
        ((*currentMenu).y as i32) - 5 + (itemOn as i32) * LINEHEIGHT,
        0,
        W_CacheLumpNameStr(skullName[whichSkull as usize], PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn M_ClearMenus() {
    menuactive = 0;
}

pub unsafe fn M_SetupNextMenu(menudef: *mut menu_t) {
    currentMenu = menudef;
    itemOn = (*currentMenu).lastOn;
}

pub unsafe fn M_Ticker() {
    skullAnimCounter -= 1;
    if skullAnimCounter <= 0 {
        whichSkull ^= 1;
        skullAnimCounter = 8;
    }
}

pub unsafe fn M_Init() {
    MainDef.menuitems = MainMenu.as_mut_ptr();
    EpiDef.prevMenu = &mut MainDef;
    EpiDef.menuitems = EpisodeMenu.as_mut_ptr();
    NewDef.prevMenu = &mut EpiDef;
    NewDef.menuitems = NewGameMenu.as_mut_ptr();
    OptionsDef.prevMenu = &mut MainDef;
    OptionsDef.menuitems = OptionsMenu.as_mut_ptr();
    ReadDef1.prevMenu = &mut MainDef;
    ReadDef1.menuitems = ReadMenu1.as_mut_ptr();
    ReadDef2.prevMenu = &mut ReadDef1;
    ReadDef2.menuitems = ReadMenu2.as_mut_ptr();
    SoundDef.prevMenu = &mut OptionsDef;
    SoundDef.menuitems = SoundMenu.as_mut_ptr();
    LoadDef.prevMenu = &mut MainDef;
    LoadDef.menuitems = LoadMenu.as_mut_ptr();
    SaveDef.prevMenu = &mut MainDef;
    SaveDef.menuitems = SaveMenu.as_mut_ptr();

    currentMenu = &mut MainDef;
    menuactive = 0;
    itemOn = (*currentMenu).lastOn;
    whichSkull = 0;
    skullAnimCounter = 10;
    screenSize = screenblocks - 3;
    messageToPrint = 0;
    messageString = std::ptr::null();
    messageLastMenuActive = menuactive;
    quickSaveSlot = -1;

    match gamemode {
        commercial => {
            MainMenu[readthis] = MainMenu[quitdoom];
            MainDef.numitems -= 1;
            MainDef.y += 8;
            NewDef.prevMenu = &mut MainDef;
            ReadDef1.routine = Some(M_DrawReadThis1);
            ReadDef1.x = 330;
            ReadDef1.y = 165;
            ReadMenu1[0].routine = Some(M_FinishReadThis);
        }
        shareware | registered => {
            EpiDef.numitems -= 1;
        }
        _ => {}
    }
}