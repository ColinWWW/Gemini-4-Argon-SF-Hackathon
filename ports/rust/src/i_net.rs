// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_net::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::i_system::*;

pub unsafe fn I_InitNetwork() {
    let layout = std::alloc::Layout::new::<doomcom_t>();
    doomcom = std::alloc::alloc_zeroed(layout) as *mut doomcom_t;

    netgame = false_;
    (*doomcom).id = DOOMCOM_ID;
    (*doomcom).numplayers = 1;
    (*doomcom).numnodes = 1;
    (*doomcom).deathmatch = false_ as i16;
    (*doomcom).consoleplayer = 0;
    (*doomcom).ticdup = 1;
    (*doomcom).extratics = 0;
}

pub unsafe fn I_NetCmd() {
    I_Error("I_NetCmd: Network play not supported in headless reference build");
}
