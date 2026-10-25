// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_event::*;
use crate::d_main::*;
use crate::d_ticcmd::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::g_game::*;
use crate::i_net::*;
use crate::i_system::*;
use crate::i_video::*;
use crate::m_menu::*;
use crate::write_c_str;

pub const DOOMCOM_ID: i32 = 0x12345678;
pub const MAXNETNODES: usize = 8;
pub const BACKUPTICS: usize = 12;

pub const CMD_SEND: i16 = 1;
pub const CMD_GET: i16 = 2;

pub const NCMD_EXIT: u32 = 0x80000000;
pub const NCMD_RETRANSMIT: u32 = 0x40000000;
pub const NCMD_SETUP: u32 = 0x20000000;
pub const NCMD_KILL: u32 = 0x10000000;
pub const NCMD_CHECKSUM: u32 = 0x0fffffff;

pub const RESENDCOUNT: i32 = 10;
pub const PL_DRONE: u8 = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct doomdata_t {
    pub checksum: u32,
    pub retransmitfrom: byte,
    pub starttic: byte,
    pub player: byte,
    pub numtics: byte,
    pub cmds: [ticcmd_t; BACKUPTICS],
}

impl Default for doomdata_t {
    fn default() -> Self {
        Self {
            checksum: 0,
            retransmitfrom: 0,
            starttic: 0,
            player: 0,
            numtics: 0,
            cmds: [ticcmd_t::default(); BACKUPTICS],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct doomcom_t {
    pub id: i32,
    pub intnum: i16,
    pub command: i16,
    pub remotenode: i16,
    pub datalength: i16,
    pub numnodes: i16,
    pub ticdup: i16,
    pub extratics: i16,
    pub deathmatch: i16,
    pub savegame: i16,
    pub episode: i16,
    pub map: i16,
    pub skill: i16,
    pub consoleplayer: i16,
    pub numplayers: i16,
    pub angleoffset: i16,
    pub drone: i16,
    pub data: doomdata_t,
}

pub static mut doomcom: *mut doomcom_t = std::ptr::null_mut();
pub static mut netbuffer: *mut doomdata_t = std::ptr::null_mut();

pub static mut localcmds: [ticcmd_t; BACKUPTICS] = [ticcmd_t {
    forwardmove: 0,
    sidemove: 0,
    angleturn: 0,
    consistancy: 0,
    chatchar: 0,
    buttons: 0,
}; BACKUPTICS];

pub static mut netcmds: [[ticcmd_t; BACKUPTICS]; MAXPLAYERS] = [[ticcmd_t {
    forwardmove: 0,
    sidemove: 0,
    angleturn: 0,
    consistancy: 0,
    chatchar: 0,
    buttons: 0,
}; BACKUPTICS]; MAXPLAYERS];

pub static mut nettics: [i32; MAXNETNODES] = [0; MAXNETNODES];
pub static mut nodeingame: [boolean; MAXNETNODES] = [0; MAXNETNODES];
pub static mut remoteresend: [boolean; MAXNETNODES] = [0; MAXNETNODES];
pub static mut resendto: [i32; MAXNETNODES] = [0; MAXNETNODES];
pub static mut resendcount: [i32; MAXNETNODES] = [0; MAXNETNODES];

pub static mut nodeforplayer: [i32; MAXPLAYERS] = [0; MAXPLAYERS];

pub static mut maketic: i32 = 0;
pub static mut lastnettic: i32 = 0;
pub static mut skiptics: i32 = 0;
pub static mut ticdup: i32 = 1;
pub static mut maxsend: i32 = 0;

pub static mut reboundpacket: boolean = 0;
pub static mut reboundstore: doomdata_t = doomdata_t {
    checksum: 0,
    retransmitfrom: 0,
    starttic: 0,
    player: 0,
    numtics: 0,
    cmds: [ticcmd_t {
        forwardmove: 0,
        sidemove: 0,
        angleturn: 0,
        consistancy: 0,
        chatchar: 0,
        buttons: 0,
    }; BACKUPTICS],
};

pub unsafe fn NetbufferSize() -> i32 {
    (8 + ((*netbuffer).numtics as usize) * std::mem::size_of::<ticcmd_t>()) as i32
}

pub unsafe fn NetbufferChecksum() -> u32 {
    0
}

pub unsafe fn ExpandTics(low: i32) -> i32 {
    let delta = low - (maketic & 0xff);

    if delta >= -64 && delta <= 64 {
        return (maketic & !0xff) + low;
    }
    if delta > 64 {
        return (maketic & !0xff) - 256 + low;
    }
    if delta < -64 {
        return (maketic & !0xff) + 256 + low;
    }

    I_Error(&format!(
        "ExpandTics: strange value {} at maketic {}",
        low, maketic
    ));
}

pub unsafe fn HSendPacket(node: i32, flags: u32) {
    (*netbuffer).checksum = NetbufferChecksum() | flags;

    if node == 0 {
        reboundstore = *netbuffer;
        reboundpacket = true_;
        return;
    }

    if demoplayback != 0 {
        return;
    }

    if netgame == 0 {
        I_Error("Tried to transmit to another node");
    }

    (*doomcom).command = CMD_SEND;
    (*doomcom).remotenode = node as i16;
    (*doomcom).datalength = NetbufferSize() as i16;

    I_NetCmd();
}

pub unsafe fn HGetPacket() -> boolean {
    if reboundpacket != 0 {
        *netbuffer = reboundstore;
        (*doomcom).remotenode = 0;
        reboundpacket = false_;
        return true_;
    }

    if netgame == 0 {
        return false_;
    }

    if demoplayback != 0 {
        return false_;
    }

    (*doomcom).command = CMD_GET;
    I_NetCmd();

    if (*doomcom).remotenode == -1 {
        return false_;
    }

    if ((*doomcom).datalength as i32) != NetbufferSize() {
        return false_;
    }

    if NetbufferChecksum() != ((*netbuffer).checksum & NCMD_CHECKSUM) {
        return false_;
    }

    true_
}

pub static mut exitmsg: [u8; 80] = [0; 80];

pub unsafe fn GetPackets() {
    while HGetPacket() != 0 {
        if (*netbuffer).checksum & NCMD_SETUP != 0 {
            continue;
        }

        let netconsole = ((*netbuffer).player & !PL_DRONE) as usize;
        let netnode = (*doomcom).remotenode as usize;

        let realstart = ExpandTics((*netbuffer).starttic as i32);
        let realend = realstart + ((*netbuffer).numtics as i32);

        if (*netbuffer).checksum & NCMD_EXIT != 0 {
            if nodeingame[netnode] == 0 {
                continue;
            }
            nodeingame[netnode] = false_;
            playeringame[netconsole] = false_;
            write_c_str(exitmsg.as_mut_ptr(), "Player 1 left the game");
            exitmsg[7] += netconsole as u8;
            players[consoleplayer as usize].message = exitmsg.as_ptr();
            if demorecording != 0 {
                G_CheckDemoStatus();
            }
            continue;
        }

        if (*netbuffer).checksum & NCMD_KILL != 0 {
            I_Error("Killed by network driver");
        }

        nodeforplayer[netconsole] = netnode as i32;

        if resendcount[netnode] <= 0 && ((*netbuffer).checksum & NCMD_RETRANSMIT != 0) {
            resendto[netnode] = ExpandTics((*netbuffer).retransmitfrom as i32);
            resendcount[netnode] = RESENDCOUNT;
        } else {
            resendcount[netnode] -= 1;
        }

        if realend == nettics[netnode] {
            continue;
        }

        if realend < nettics[netnode] {
            continue;
        }

        if realstart > nettics[netnode] {
            remoteresend[netnode] = true_;
            continue;
        }

        remoteresend[netnode] = false_;

        let start = nettics[netnode] - realstart;
        let mut src = (*netbuffer).cmds.as_ptr().add(start as usize);

        while nettics[netnode] < realend {
            let dest =
                &mut netcmds[netconsole][(nettics[netnode] as usize) % BACKUPTICS] as *mut ticcmd_t;
            nettics[netnode] += 1;
            *dest = *src;
            src = src.add(1);
        }
    }
}

pub static mut gametime: i32 = 0;

pub unsafe fn NetUpdate() {
    let nowtime = I_GetTime() / ticdup;
    let mut newtics = nowtime - gametime;
    gametime = nowtime;

    if newtics <= 0 {
        GetPackets();
        return;
    }

    if skiptics <= newtics {
        newtics -= skiptics;
        skiptics = 0;
    } else {
        skiptics -= newtics;
        newtics = 0;
    }

    (*netbuffer).player = consoleplayer as byte;

    let gameticdiv = gametic / ticdup;
    for _ in 0..newtics {
        I_StartTic();
        D_ProcessEvents();
        if maketic - gameticdiv >= (BACKUPTICS as i32) / 2 - 1 {
            break;
        }
        G_BuildTiccmd(&mut localcmds[(maketic as usize) % BACKUPTICS]);
        maketic += 1;
    }

    if singletics != 0 {
        return;
    }

    for i in 0..((*doomcom).numnodes as usize) {
        if nodeingame[i] != 0 {
            let realstart = resendto[i];
            (*netbuffer).starttic = realstart as byte;
            (*netbuffer).numtics = (maketic - realstart) as byte;
            if ((*netbuffer).numtics as usize) > BACKUPTICS {
                I_Error("NetUpdate: netbuffer->numtics > BACKUPTICS");
            }

            resendto[i] = maketic - ((*doomcom).extratics as i32);

            for j in 0..((*netbuffer).numtics as usize) {
                (*netbuffer).cmds[j] = localcmds[((realstart as usize) + j) % BACKUPTICS];
            }

            if remoteresend[i] != 0 {
                (*netbuffer).retransmitfrom = nettics[i] as byte;
                HSendPacket(i as i32, NCMD_RETRANSMIT);
            } else {
                (*netbuffer).retransmitfrom = 0;
                HSendPacket(i as i32, 0);
            }
        }
    }

    GetPackets();
}

pub unsafe fn CheckAbort() {
    let stoptic = I_GetTime() + 2;
    while I_GetTime() < stoptic {
        I_StartTic();
    }

    I_StartTic();
    while eventtail != eventhead {
        let ev = &events[eventtail];
        if ev.type_ == ev_keydown && ev.data1 == KEY_ESCAPE {
            I_Error("Network game synchronization aborted.");
        }
        eventtail = (eventtail + 1) & (MAXEVENTS - 1);
    }
}

pub unsafe fn D_ArbitrateNetStart() {
    let mut gotinfo = [false_; MAXNETNODES];
    autostart = true_;

    if (*doomcom).consoleplayer != 0 {
        println!("listening for network start info...");
        loop {
            CheckAbort();
            if HGetPacket() == 0 {
                continue;
            }
            if (*netbuffer).checksum & NCMD_SETUP != 0 {
                if (*netbuffer).player != (VERSION as byte) {
                    I_Error("Different DOOM versions cannot play a net game!");
                }
                startskill = ((*netbuffer).retransmitfrom & 15) as skill_t;
                deathmatch = (((*netbuffer).retransmitfrom & 0xc0) >> 6) as boolean;
                nomonsters = (((*netbuffer).retransmitfrom & 0x20) > 0) as boolean;
                respawnparm = (((*netbuffer).retransmitfrom & 0x10) > 0) as boolean;
                startmap = ((*netbuffer).starttic & 0x3f) as i32;
                startepisode = ((*netbuffer).starttic >> 6) as i32;
                return;
            }
        }
    } else {
        println!("sending network start info...");
        loop {
            CheckAbort();
            for i in 0..((*doomcom).numnodes as i32) {
                (*netbuffer).retransmitfrom = startskill as byte;
                if deathmatch != 0 {
                    (*netbuffer).retransmitfrom |= (deathmatch as byte) << 6;
                }
                if nomonsters != 0 {
                    (*netbuffer).retransmitfrom |= 0x20;
                }
                if respawnparm != 0 {
                    (*netbuffer).retransmitfrom |= 0x10;
                }
                (*netbuffer).starttic = (startepisode * 64 + startmap) as byte;
                (*netbuffer).player = VERSION as byte;
                (*netbuffer).numtics = 0;
                HSendPacket(i, NCMD_SETUP);
            }

            let mut cnt = 10;
            while cnt > 0 && HGetPacket() != 0 {
                cnt -= 1;
                let p = ((*netbuffer).player & 0x7f) as usize;
                if p < MAXNETNODES {
                    gotinfo[p] = true_;
                }
            }

            let mut i = 1usize;
            while i < ((*doomcom).numnodes as usize) {
                if gotinfo[i] == 0 {
                    break;
                }
                i += 1;
            }
            if i >= ((*doomcom).numnodes as usize) {
                break;
            }
        }
    }
}

pub unsafe fn D_CheckNetGame() {
    for i in 0..MAXNETNODES {
        nodeingame[i] = false_;
        nettics[i] = 0;
        remoteresend[i] = false_;
        resendto[i] = 0;
    }

    I_InitNetwork();
    if (*doomcom).id != DOOMCOM_ID {
        I_Error("Doomcom buffer invalid!");
    }

    netbuffer = &mut (*doomcom).data;
    consoleplayer = (*doomcom).consoleplayer as i32;
    displayplayer = consoleplayer;
    if netgame != 0 {
        D_ArbitrateNetStart();
    }

    println!(
        "startskill {}  deathmatch: {}  startmap: {}  startepisode: {}",
        startskill, deathmatch, startmap, startepisode
    );

    ticdup = (*doomcom).ticdup as i32;
    maxsend = (BACKUPTICS as i32) / (2 * ticdup) - 1;
    if maxsend < 1 {
        maxsend = 1;
    }

    for i in 0..((*doomcom).numplayers as usize) {
        playeringame[i] = true_;
    }
    for i in 0..((*doomcom).numnodes as usize) {
        nodeingame[i] = true_;
    }

    println!(
        "player {} of {} ({} nodes)",
        consoleplayer + 1,
        (*doomcom).numplayers,
        (*doomcom).numnodes
    );
}

pub unsafe fn D_QuitNetGame() {
    if netgame == 0 || usergame == 0 || consoleplayer == -1 || demoplayback != 0 {
        return;
    }

    (*netbuffer).player = consoleplayer as byte;
    (*netbuffer).numtics = 0;
    for _ in 0..4 {
        for j in 1..((*doomcom).numnodes as usize) {
            if nodeingame[j] != 0 {
                HSendPacket(j as i32, NCMD_EXIT);
            }
        }
        I_WaitVBL(1);
    }
}

pub static mut frametics: [i32; 4] = [0; 4];
pub static mut frameon: i32 = 0;
pub static mut frameskip: [boolean; 4] = [0; 4];
pub static mut oldnettics: i32 = 0;
static mut oldentertics: i32 = 0;

pub unsafe fn TryRunTics() {
    let entertic = I_GetTime() / ticdup;
    let realtics = entertic - oldentertics;
    oldentertics = entertic;

    NetUpdate();

    let mut lowtic = MAXINT;
    let mut numplaying = 0;
    for i in 0..((*doomcom).numnodes as usize) {
        if nodeingame[i] != 0 {
            numplaying += 1;
            if nettics[i] < lowtic {
                lowtic = nettics[i];
            }
        }
    }
    let availabletics = lowtic - gametic / ticdup;

    let mut counts = if realtics < availabletics - 1 {
        realtics + 1
    } else if realtics < availabletics {
        realtics
    } else {
        availabletics
    };

    if counts < 1 {
        counts = 1;
    }

    frameon += 1;

    if demoplayback == 0 {
        let mut i = 0;
        while i < MAXPLAYERS {
            if playeringame[i] != 0 {
                break;
            }
            i += 1;
        }
        if consoleplayer != (i as i32) {
            if nettics[0] <= nettics[nodeforplayer[i] as usize] {
                gametime -= 1;
            }
            frameskip[(frameon & 3) as usize] =
                (oldnettics > nettics[nodeforplayer[i] as usize]) as boolean;
            oldnettics = nettics[0];
            if frameskip[0] != 0
                && frameskip[1] != 0
                && frameskip[2] != 0
                && frameskip[3] != 0
            {
                skiptics = 1;
            }
        }
    }

    while lowtic < gametic / ticdup + counts {
        NetUpdate();
        lowtic = MAXINT;

        for i in 0..((*doomcom).numnodes as usize) {
            if nodeingame[i] != 0 && nettics[i] < lowtic {
                lowtic = nettics[i];
            }
        }

        if lowtic < gametic / ticdup {
            I_Error("TryRunTics: lowtic < gametic");
        }

        if I_GetTime() / ticdup - entertic >= 20 {
            M_Ticker();
            return;
        }
    }

    while counts > 0 {
        counts -= 1;
        for i in 0..ticdup {
            if gametic / ticdup > lowtic {
                I_Error("gametic>lowtic");
            }
            if advancedemo != 0 {
                D_DoAdvanceDemo();
            }
            M_Ticker();
            G_Ticker();
            gametic += 1;

            if i != ticdup - 1 {
                let buf = ((gametic / ticdup) as usize) % BACKUPTICS;
                for j in 0..MAXPLAYERS {
                    let cmd = &mut netcmds[j][buf];
                    cmd.chatchar = 0;
                    if cmd.buttons & BT_SPECIAL != 0 {
                        cmd.buttons = 0;
                    }
                }
            }
        }
        NetUpdate();
    }
}
