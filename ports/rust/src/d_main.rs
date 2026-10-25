// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::fs::File;
use std::io::Read;
use crate::am_map::*;
use crate::d_event::*;
use crate::d_net::*;
use crate::d_player::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
use crate::f_finale::*;
use crate::f_wipe::*;
use crate::g_game::*;
use crate::hu_stuff::*;
use crate::i_sound::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::m_argv::*;
use crate::m_menu::*;
use crate::m_misc::*;
use crate::p_setup::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::r_main::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::st_stuff::*;
use crate::v_video::*;
use crate::w_wad::*;
use crate::wi_stuff::*;
use crate::z_zone::*;
use crate::{c_atoi, c_str, c_strcpy, c_strlen, write_c_str};

pub const MAXWADFILES: usize = 20;
pub static mut wadfiles: [*mut u8; MAXWADFILES] = [std::ptr::null_mut(); MAXWADFILES];

pub static mut devparm: boolean = 0;
pub static mut nomonsters: boolean = 0;
pub static mut respawnparm: boolean = 0;
pub static mut fastparm: boolean = 0;
pub static mut drone: boolean = 0;
pub static mut singletics: boolean = false_;

pub static mut startskill: skill_t = 0;
pub static mut startepisode: i32 = 0;
pub static mut startmap: i32 = 0;
pub static mut autostart: boolean = 0;

pub static mut debugfile: *mut File = std::ptr::null_mut();
pub static mut advancedemo: boolean = 0;

pub static mut wadfile: [u8; 1024] = [0; 1024];
pub static mut mapdir: [u8; 1024] = [0; 1024];
pub static mut basedefault: [u8; 1024] = [0; 1024];

pub static mut events: [event_t; MAXEVENTS] = [event_t {
    type_: 0,
    data1: 0,
    data2: 0,
    data3: 0,
}; MAXEVENTS];
pub static mut eventhead: usize = 0;
pub static mut eventtail: usize = 0;

pub unsafe fn D_PostEvent(ev: *mut event_t) {
    events[eventhead] = *ev;
    eventhead = (eventhead + 1) & (MAXEVENTS - 1);
}

pub unsafe fn D_ProcessEvents() {
    if gamemode == commercial && W_CheckNumForNameStr("map01") < 0 {
        return;
    }

    while eventtail != eventhead {
        let ev = &mut events[eventtail] as *mut event_t;
        if M_Responder(ev) == 0 {
            G_Responder(ev);
        }
        eventtail = (eventtail + 1) & (MAXEVENTS - 1);
    }
}

pub static mut wipegamestate: gamestate_t = GS_DEMOSCREEN;
static mut viewactivestate: boolean = false_;
static mut menuactivestate: boolean = false_;
static mut inhelpscreensstate: boolean = false_;
static mut fullscreen: boolean = false_;
static mut oldgamestate: gamestate_t = -1;
static mut borderdrawcount: i32 = 0;

pub unsafe fn D_Display() {
    if nodrawers != 0 {
        return;
    }

    let mut redrawsbar = false_;

    if setsizeneeded != 0 {
        R_ExecuteSetViewSize();
        oldgamestate = -1;
        borderdrawcount = 3;
    }

    let wipe = if gamestate != wipegamestate {
        wipe_StartScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);
        true_
    } else {
        false_
    };

    if gamestate == GS_LEVEL && gametic != 0 {
        HU_Erase();
    }

    match gamestate {
        GS_LEVEL => {
            if gametic != 0 {
                if automapactive != 0 {
                    AM_Drawer();
                }
                if wipe != 0 || (viewheight != 200 && fullscreen != 0) {
                    redrawsbar = true_;
                }
                if inhelpscreensstate != 0 && inhelpscreens == 0 {
                    redrawsbar = true_;
                }
                ST_Drawer((viewheight == 200) as boolean, redrawsbar);
                fullscreen = (viewheight == 200) as boolean;
            }
        }
        GS_INTERMISSION => WI_Drawer(),
        GS_FINALE => F_Drawer(),
        GS_DEMOSCREEN => D_PageDrawer(),
        _ => {}
    }

    I_UpdateNoBlit();

    if gamestate == GS_LEVEL && automapactive == 0 && gametic != 0 {
        R_RenderPlayerView(&mut players[displayplayer as usize]);
    }

    if gamestate == GS_LEVEL && gametic != 0 {
        HU_Drawer();
    }

    if gamestate != oldgamestate && gamestate != GS_LEVEL {
        I_SetPalette(W_CacheLumpNameStr("PLAYPAL", PU_CACHE) as *mut byte);
    }

    if gamestate == GS_LEVEL && oldgamestate != GS_LEVEL {
        viewactivestate = false_;
        R_FillBackScreen();
    }

    if gamestate == GS_LEVEL && automapactive == 0 && scaledviewwidth != 320 {
        if menuactive != 0 || menuactivestate != 0 || viewactivestate == 0 {
            borderdrawcount = 3;
        }
        if borderdrawcount != 0 {
            R_DrawViewBorder();
            borderdrawcount -= 1;
        }
    }

    menuactivestate = menuactive;
    viewactivestate = viewactive;
    inhelpscreensstate = inhelpscreens;
    oldgamestate = gamestate;
    wipegamestate = gamestate;

    if paused != 0 {
        let y = if automapactive != 0 {
            4
        } else {
            viewwindowy + 4
        };
        V_DrawPatchDirect(
            viewwindowx + (scaledviewwidth - 68) / 2,
            y,
            0,
            W_CacheLumpNameStr("M_PAUSE", PU_CACHE) as *mut patch_t,
        );
    }

    M_Drawer();
    NetUpdate();

    if wipe == 0 {
        I_FinishUpdate();
        return;
    }

    wipe_EndScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);

    let mut wipestart = I_GetTime() - 1;

    loop {
        let mut tics;
        let mut nowtime;
        loop {
            nowtime = I_GetTime();
            tics = nowtime - wipestart;
            if tics != 0 {
                break;
            }
        }
        wipestart = nowtime;
        let done = wipe_ScreenWipe(wipe_Melt, 0, 0, SCREENWIDTH, SCREENHEIGHT, tics);
        I_UpdateNoBlit();
        M_Drawer();
        I_FinishUpdate();
        if done != 0 {
            break;
        }
    }
}

pub unsafe fn D_DoomLoop() -> ! {
    if demorecording != 0 {
        G_BeginRecording();
    }

    if M_CheckParmStr("-debugfile") != 0 {
        let filename = format!("debug{}.txt", consoleplayer);
        println!("debug output to: {}", filename);
        if let Ok(f) = File::create(&filename) {
            debugfile = Box::into_raw(Box::new(f));
        }
    }

    I_InitGraphics();

    loop {
        I_StartFrame();

        if singletics != 0 {
            I_StartTic();
            D_ProcessEvents();
            G_BuildTiccmd(&mut netcmds[consoleplayer as usize][(maketic as usize) % BACKUPTICS]);
            if advancedemo != 0 {
                D_DoAdvanceDemo();
            }
            M_Ticker();
            G_Ticker();
            gametic += 1;
            maketic += 1;
        } else {
            TryRunTics();
        }

        S_UpdateSounds(players[consoleplayer as usize].mo as *mut std::ffi::c_void);
        D_Display();
        I_SubmitSound();
    }
}

pub static mut demosequence: i32 = 0;
pub static mut pagetic: i32 = 0;
pub static mut pagename: &'static str = "";

pub unsafe fn D_PageTicker() {
    pagetic -= 1;
    if pagetic < 0 {
        D_AdvanceDemo();
    }
}

pub unsafe fn D_PageDrawer() {
    V_DrawPatch(
        0,
        0,
        0,
        W_CacheLumpNameStr(pagename, PU_CACHE) as *mut patch_t,
    );
}

pub unsafe fn D_AdvanceDemo() {
    advancedemo = true_;
}

pub unsafe fn D_DoAdvanceDemo() {
    players[consoleplayer as usize].playerstate = PST_LIVE;
    advancedemo = false_;
    usergame = false_;
    paused = false_;
    gameaction = ga_nothing;

    if gamemode == retail {
        demosequence = (demosequence + 1) % 7;
    } else {
        demosequence = (demosequence + 1) % 6;
    }

    match demosequence {
        0 => {
            if gamemode == commercial {
                pagetic = 35 * 11;
            } else {
                pagetic = 170;
            }
            gamestate = GS_DEMOSCREEN;
            pagename = "TITLEPIC";
            if gamemode == commercial {
                S_StartMusic(mus_dm2ttl);
            } else {
                S_StartMusic(mus_intro);
            }
        }
        1 => {
            G_DeferedPlayDemo(b"demo1\0".as_ptr());
        }
        2 => {
            pagetic = 200;
            gamestate = GS_DEMOSCREEN;
            pagename = "CREDIT";
        }
        3 => {
            G_DeferedPlayDemo(b"demo2\0".as_ptr());
        }
        4 => {
            gamestate = GS_DEMOSCREEN;
            if gamemode == commercial {
                pagetic = 35 * 11;
                pagename = "TITLEPIC";
                S_StartMusic(mus_dm2ttl);
            } else {
                pagetic = 200;
                if gamemode == retail {
                    pagename = "CREDIT";
                } else {
                    pagename = "HELP2";
                }
            }
        }
        5 => {
            G_DeferedPlayDemo(b"demo3\0".as_ptr());
        }
        6 => {
            G_DeferedPlayDemo(b"demo4\0".as_ptr());
        }
        _ => {}
    }
}

pub unsafe fn D_StartTitle() {
    gameaction = ga_nothing;
    demosequence = -1;
    D_AdvanceDemo();
}

pub static mut title: [u8; 128] = [0; 128];

pub unsafe fn D_AddFile(file: *const u8) {
    let mut numwadfiles = 0;
    while !wadfiles[numwadfiles].is_null() {
        numwadfiles += 1;
    }

    let len = c_strlen(file) + 1;
    let layout = std::alloc::Layout::from_size_align(len, 1).unwrap();
    let newfile = std::alloc::alloc(layout);
    c_strcpy(newfile, file);

    wadfiles[numwadfiles] = newfile;
}

pub unsafe fn D_AddFileStr(file: &str) {
    let mut buf = Vec::with_capacity(file.len() + 1);
    buf.extend_from_slice(file.as_bytes());
    buf.push(0);
    D_AddFile(buf.as_ptr());
}

pub unsafe fn IdentifyVersion() {
    let doomwaddir = std::env::var("DOOMWADDIR").unwrap_or_else(|_| ".".to_string());

    let doom2wad = format!("{}/doom2.wad", doomwaddir);
    let doomuwad = format!("{}/doomu.wad", doomwaddir);
    let doomwad = format!("{}/doom.wad", doomwaddir);
    let doom1wad = format!("{}/doom1.wad", doomwaddir);
    let plutoniawad = format!("{}/plutonia.wad", doomwaddir);
    let tntwad = format!("{}/tnt.wad", doomwaddir);
    let doom2fwad = format!("{}/doom2f.wad", doomwaddir);

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let bdef = format!("{}/.doomrc", home);
    write_c_str(basedefault.as_mut_ptr(), &bdef);

    let iwad_p = M_CheckParmStr("-iwad");
    if iwad_p != 0 && iwad_p < myargc - 1 {
        gamemode = shareware;
        D_AddFile(*myargv.add((iwad_p + 1) as usize));
        return;
    }

    if M_CheckParmStr("-shdev") != 0 {
        gamemode = shareware;
        devparm = true_;
        D_AddFileStr(&format!("{}doom1.wad", DEVDATA));
        D_AddFileStr(&format!("{}data_se/texture1.lmp", DEVMAPS));
        D_AddFileStr(&format!("{}data_se/pnames.lmp", DEVMAPS));
        write_c_str(basedefault.as_mut_ptr(), &format!("{}default.cfg", DEVDATA));
        return;
    }

    if M_CheckParmStr("-regdev") != 0 {
        gamemode = registered;
        devparm = true_;
        D_AddFileStr(&format!("{}doom.wad", DEVDATA));
        D_AddFileStr(&format!("{}data_se/texture1.lmp", DEVMAPS));
        D_AddFileStr(&format!("{}data_se/texture2.lmp", DEVMAPS));
        D_AddFileStr(&format!("{}data_se/pnames.lmp", DEVMAPS));
        write_c_str(basedefault.as_mut_ptr(), &format!("{}default.cfg", DEVDATA));
        return;
    }

    if M_CheckParmStr("-comdev") != 0 {
        gamemode = commercial;
        devparm = true_;
        D_AddFileStr(&format!("{}doom2.wad", DEVDATA));
        D_AddFileStr(&format!("{}cdata/texture1.lmp", DEVMAPS));
        D_AddFileStr(&format!("{}cdata/pnames.lmp", DEVMAPS));
        write_c_str(basedefault.as_mut_ptr(), &format!("{}default.cfg", DEVDATA));
        return;
    }

    if std::path::Path::new(&doom2fwad).exists() {
        gamemode = commercial;
        language = french;
        println!("French version");
        D_AddFileStr(&doom2fwad);
        return;
    }

    if std::path::Path::new(&doom2wad).exists() {
        gamemode = commercial;
        D_AddFileStr(&doom2wad);
        return;
    }

    if std::path::Path::new(&plutoniawad).exists() {
        gamemode = commercial;
        D_AddFileStr(&plutoniawad);
        return;
    }

    if std::path::Path::new(&tntwad).exists() {
        gamemode = commercial;
        D_AddFileStr(&tntwad);
        return;
    }

    if std::path::Path::new(&doomuwad).exists() {
        gamemode = retail;
        D_AddFileStr(&doomuwad);
        return;
    }

    if std::path::Path::new(&doomwad).exists() {
        gamemode = registered;
        D_AddFileStr(&doomwad);
        return;
    }

    if std::path::Path::new(&doom1wad).exists() {
        gamemode = shareware;
        D_AddFileStr(&doom1wad);
        return;
    }

    println!("Game mode indeterminate.");
    gamemode = indetermined;
}

pub unsafe fn FindResponseFile() {
    const MAXARGVS: usize = 100;

    for i in 1..(myargc as usize) {
        let arg = *myargv.add(i);
        if *arg == b'@' {
            let fname = c_str(arg.add(1));
            let mut handle = match File::open(fname) {
                Ok(f) => f,
                Err(_) => {
                    print!("\nNo such response file!");
                    std::process::exit(1);
                }
            };
            println!("Found response file {}!", fname);

            let mut file_bytes = Vec::new();
            let _ = handle.read_to_end(&mut file_bytes);
            let size = file_bytes.len();

            let layout = std::alloc::Layout::from_size_align(size + 1, 1).unwrap();
            let file_ptr = std::alloc::alloc(layout);
            std::ptr::copy_nonoverlapping(file_bytes.as_ptr(), file_ptr, size);
            *file_ptr.add(size) = 0;

            let mut moreargs = [std::ptr::null_mut::<u8>(); 20];
            let mut index = 0;
            for k in (i + 1)..(myargc as usize) {
                moreargs[index] = *myargv.add(k);
                index += 1;
            }

            let firstargv = *myargv.add(0);
            let argv_layout =
                std::alloc::Layout::from_size_align(MAXARGVS * std::mem::size_of::<*mut u8>(), 8)
                    .unwrap();
            myargv = std::alloc::alloc_zeroed(argv_layout) as *mut *mut u8;
            *myargv.add(0) = firstargv;

            let infile = file_ptr;
            let mut indexinfile = 1usize;
            let mut k = 0usize;
            loop {
                *myargv.add(indexinfile) = infile.add(k);
                indexinfile += 1;
                while k < size && *infile.add(k) >= (b' ' + 1) && *infile.add(k) <= b'z' {
                    k += 1;
                }
                *infile.add(k) = 0;
                while k < size && (*infile.add(k) <= b' ' || *infile.add(k) > b'z') {
                    k += 1;
                }
                if k >= size {
                    break;
                }
            }

            for m in 0..index {
                *myargv.add(indexinfile) = moreargs[m];
                indexinfile += 1;
            }
            myargc = indexinfile as i32;

            println!("{} command-line args:", myargc);
            for m in 1..(myargc as usize) {
                println!("{}", c_str(*myargv.add(m)));
            }
            break;
        }
    }
}

pub unsafe fn D_DoomMain() {
    FindResponseFile();
    IdentifyVersion();

    modifiedgame = false_;

    nomonsters = M_CheckParmStr("-nomonsters");
    respawnparm = M_CheckParmStr("-respawn");
    fastparm = M_CheckParmStr("-fast");
    devparm = M_CheckParmStr("-devparm");
    if M_CheckParmStr("-altdeath") != 0 {
        deathmatch = 2;
    } else if M_CheckParmStr("-deathmatch") != 0 {
        deathmatch = 1;
    }

    let t_str = match gamemode {
        retail => format!(
            "                         The Ultimate DOOM Startup v{}.{}                           ",
            VERSION / 100,
            VERSION % 100
        ),
        shareware => format!(
            "                            DOOM Shareware Startup v{}.{}                           ",
            VERSION / 100,
            VERSION % 100
        ),
        registered => format!(
            "                            DOOM Registered Startup v{}.{}                           ",
            VERSION / 100,
            VERSION % 100
        ),
        commercial => format!(
            "                         DOOM 2: Hell on Earth v{}.{}                           ",
            VERSION / 100,
            VERSION % 100
        ),
        _ => format!(
            "                     Public DOOM - v{}.{}                           ",
            VERSION / 100,
            VERSION % 100
        ),
    };
    write_c_str(title.as_mut_ptr(), &t_str);
    println!("{}", t_str);

    if devparm != 0 {
        print!("{}", D_DEVSTR);
    }

    if M_CheckParmStr("-cdrom") != 0 {
        print!("{}", D_CDROM);
        let _ = std::fs::create_dir_all("c:\\doomdata");
        write_c_str(basedefault.as_mut_ptr(), "c:/doomdata/default.cfg");
    }

    let p_turbo = M_CheckParmStr("-turbo");
    if p_turbo != 0 {
        let mut scale = 200;
        if p_turbo < myargc - 1 {
            scale = c_atoi(*myargv.add((p_turbo + 1) as usize));
        }
        if scale < 10 {
            scale = 10;
        }
        if scale > 400 {
            scale = 400;
        }
        println!("turbo scale: {}%", scale);
        forwardmove[0] = forwardmove[0] * scale / 100;
        forwardmove[1] = forwardmove[1] * scale / 100;
        sidemove[0] = sidemove[0] * scale / 100;
        sidemove[1] = sidemove[1] * scale / 100;
    }

    let p_wart = M_CheckParmStr("-wart");
    if p_wart != 0 {
        *(*myargv.add(p_wart as usize)).add(4) = b'p';
        let file = match gamemode {
            shareware | retail | registered => {
                let ep = *(*myargv.add((p_wart + 1) as usize)) as char;
                let mp = *(*myargv.add((p_wart + 2) as usize)) as char;
                println!(
                    "Warping to Episode {}, Map {}.",
                    c_str(*myargv.add((p_wart + 1) as usize)),
                    c_str(*myargv.add((p_wart + 2) as usize))
                );
                format!("~{}E{}M{}.wad", DEVMAPS, ep, mp)
            }
            _ => {
                let m = c_atoi(*myargv.add((p_wart + 1) as usize));
                if m < 10 {
                    format!("~{}cdata/map0{}.wad", DEVMAPS, m)
                } else {
                    format!("~{}cdata/map{}.wad", DEVMAPS, m)
                }
            }
        };
        D_AddFileStr(&file);
    }

    let mut p = M_CheckParmStr("-file");
    if p != 0 {
        modifiedgame = true_;
        loop {
            p += 1;
            if p == myargc || *(*myargv.add(p as usize)) == b'-' {
                break;
            }
            D_AddFile(*myargv.add(p as usize));
        }
    }

    p = M_CheckParmStr("-playdemo");
    if p == 0 {
        p = M_CheckParmStr("-timedemo");
    }
    if p != 0 && p < myargc - 1 {
        let dname = c_str(*myargv.add((p + 1) as usize));
        let file = format!("{}.lmp", dname);
        D_AddFileStr(&file);
        println!("Playing demo {}.lmp.", dname);
    }

    startskill = sk_medium;
    startepisode = 1;
    startmap = 1;
    autostart = false_;

    p = M_CheckParmStr("-skill");
    if p != 0 && p < myargc - 1 {
        startskill = (*(*myargv.add((p + 1) as usize)) - b'1') as skill_t;
        autostart = true_;
    }

    p = M_CheckParmStr("-episode");
    if p != 0 && p < myargc - 1 {
        startepisode = (*(*myargv.add((p + 1) as usize)) - b'0') as i32;
        startmap = 1;
        autostart = true_;
    }

    p = M_CheckParmStr("-timer");
    if p != 0 && p < myargc - 1 && deathmatch != 0 {
        let time = c_atoi(*myargv.add((p + 1) as usize));
        print!("Levels will end after {} minute", time);
        if time > 1 {
            print!("s");
        }
        println!(".");
    }

    p = M_CheckParmStr("-avg");
    if p != 0 && p < myargc - 1 && deathmatch != 0 {
        println!("Austin Virtual Gaming: Levels will end after 20 minutes");
    }

    p = M_CheckParmStr("-warp");
    if p != 0 && p < myargc - 1 {
        if gamemode == commercial {
            startmap = c_atoi(*myargv.add((p + 1) as usize));
        } else {
            startepisode = (*(*myargv.add((p + 1) as usize)) - b'0') as i32;
            startmap = (*(*myargv.add((p + 2) as usize)) - b'0') as i32;
        }
        autostart = true_;
    }

    println!("V_Init: allocate screens.");
    V_Init();

    println!("M_LoadDefaults: Load system defaults.");
    M_LoadDefaults();

    println!("Z_Init: Init zone memory allocation daemon. ");
    Z_Init();

    println!("W_Init: Init WADfiles.");
    W_InitMultipleFiles(wadfiles.as_mut_ptr());

    if modifiedgame != 0 {
        let names = [
            "e2m1", "e2m2", "e2m3", "e2m4", "e2m5", "e2m6", "e2m7", "e2m8", "e2m9", "e3m1",
            "e3m3", "e3m3", "e3m4", "e3m5", "e3m6", "e3m7", "e3m8", "e3m9", "dphoof", "bfgga0",
            "heada1", "cybra1", "spida1d1",
        ];

        if gamemode == shareware {
            I_Error("\nYou cannot -file with the shareware version. Register!");
        }

        if gamemode == registered {
            for n in &names {
                if W_CheckNumForNameStr(n) < 0 {
                    I_Error("\nThis is not the registered version.");
                }
            }
        }
    }

    if modifiedgame != 0 {
        println!("===========================================================================");
        println!("ATTENTION:  This version of DOOM has been modified.  If you would like to");
        println!("get a copy of the original game, call 1-800-IDGAMES or see the readme file.");
        println!("        You will not receive technical support for modified games.");
        println!("                      press enter to continue");
        println!("===========================================================================");
        let mut _s = String::new();
        let _ = std::io::stdin().read_line(&mut _s);
    }

    match gamemode {
        shareware | indetermined => {
            println!("===========================================================================");
            println!("                                Shareware!");
            println!("===========================================================================");
        }
        registered | retail | commercial => {
            println!("===========================================================================");
            println!("                 Commercial product - do not distribute!");
            println!("         Please report software piracy to the SPA: 1-800-388-PIR8");
            println!("===========================================================================");
        }
        _ => {}
    }

    println!("M_Init: Init miscellaneous info.");
    M_Init();

    print!("R_Init: Init DOOM refresh daemon - ");
    R_Init();

    println!("\nP_Init: Init Playloop state.");
    P_Init();

    println!("I_Init: Setting up machine state.");
    I_Init();

    println!("D_CheckNetGame: Checking network game status.");
    D_CheckNetGame();

    println!("S_Init: Setting up sound.");
    S_Init(snd_SfxVolume, snd_MusicVolume);

    println!("HU_Init: Setting up heads up display.");
    HU_Init();

    println!("ST_Init: Init status bar.");
    ST_Init();

    p = M_CheckParmStr("-statcopy");
    if p != 0 && p < myargc - 1 {
        statcopy = c_atoi(*myargv.add((p + 1) as usize)) as usize as *mut std::ffi::c_void;
        println!("External statistics registered.");
    }

    p = M_CheckParmStr("-record");
    if p != 0 && p < myargc - 1 {
        G_RecordDemo(*myargv.add((p + 1) as usize));
        autostart = true_;
    }

    p = M_CheckParmStr("-playdemo");
    if p != 0 && p < myargc - 1 {
        singledemo = true_;
        G_DeferedPlayDemo(*myargv.add((p + 1) as usize));
        D_DoomLoop();
    }

    p = M_CheckParmStr("-timedemo");
    if p != 0 && p < myargc - 1 {
        G_TimeDemo(*myargv.add((p + 1) as usize));
        D_DoomLoop();
    }

    p = M_CheckParmStr("-loadgame");
    if p != 0 && p < myargc - 1 {
        let mut file = [0u8; 256];
        let ch = *(*myargv.add((p + 1) as usize)) as char;
        let s = if M_CheckParmStr("-cdrom") != 0 {
            format!("c:\\doomdata\\{}{}.dsg", SAVEGAMENAME, ch)
        } else {
            format!("{}{}.dsg", SAVEGAMENAME, ch)
        };
        write_c_str(file.as_mut_ptr(), &s);
        G_LoadGame(file.as_ptr());
    }

    if gameaction != ga_loadgame {
        if autostart != 0 || netgame != 0 {
            G_InitNew(startskill, startepisode, startmap);
        } else {
            D_StartTitle();
        }
    }

    D_DoomLoop();
}
