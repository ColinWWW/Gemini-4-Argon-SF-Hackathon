// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_event::*;
use crate::d_player::*;
use crate::doomdef::*;
use crate::doomstat::*;
use crate::doomtype::*;
use crate::dstrings::*;
use crate::hu_lib::*;
use crate::m_menu::showMessages;
use crate::m_swap::*;
use crate::r_defs::*;
use crate::s_sound::*;
use crate::sounds::*;
use crate::w_wad::*;
use crate::z_zone::*;
use crate::{c_strcpy, c_str};

pub const HU_FONTSTART: u8 = b'!';
pub const HU_FONTEND: u8 = b'_';
pub const HU_FONTSIZE: usize = (HU_FONTEND - HU_FONTSTART + 1) as usize;
pub const HU_BROADCAST: u8 = 5;
pub const HU_MSGREFRESH: i32 = KEY_ENTER;
pub const HU_MSGX: i32 = 0;
pub const HU_MSGY: i32 = 0;
pub const HU_MSGWIDTH: i32 = 64;
pub const HU_MSGHEIGHT: i32 = 1;
pub const HU_MSGTIMEOUT: i32 = 4 * TICRATE;
pub const HU_TITLEHEIGHT: i32 = 1;
pub const HU_TITLEX: i32 = 0;
pub const HU_INPUTTOGGLE: u8 = b't';
pub const HU_INPUTX: i32 = HU_MSGX;
pub const HU_INPUTWIDTH: i32 = 64;
pub const HU_INPUTHEIGHT: i32 = 1;

pub static mut chat_macros: [*const u8; 10] = [
    b"No\0".as_ptr(),
    b"I'm ready to kick butt!\0".as_ptr(),
    b"I'm OK.\0".as_ptr(),
    b"I'm not looking too good!\0".as_ptr(),
    b"Help!\0".as_ptr(),
    b"You suck!\0".as_ptr(),
    b"Next time, scumbag...\0".as_ptr(),
    b"Come here!\0".as_ptr(),
    b"I'll take care of it.\0".as_ptr(),
    b"Yes\0".as_ptr(),
];

pub static mut player_names: [*const u8; 4] = [
    b"Green: \0".as_ptr(),
    b"Indigo: \0".as_ptr(),
    b"Brown: \0".as_ptr(),
    b"Red: \0".as_ptr(),
];

pub static mut chat_char: u8 = 0;
static mut plr: *mut player_t = std::ptr::null_mut();
pub static mut hu_font: [*mut patch_t; HU_FONTSIZE] = [std::ptr::null_mut(); HU_FONTSIZE];
static mut w_title: hu_textline_t = hu_textline_t {
    x: 0,
    y: 0,
    f: std::ptr::null_mut(),
    sc: 0,
    l: [0; HU_MAXLINELENGTH + 1],
    len: 0,
    needsupdate: 0,
};
pub static mut chat_on: boolean = 0;
static mut w_chat: hu_itext_t = hu_itext_t {
    l: hu_textline_t {
        x: 0,
        y: 0,
        f: std::ptr::null_mut(),
        sc: 0,
        l: [0; HU_MAXLINELENGTH + 1],
        len: 0,
        needsupdate: 0,
    },
    lm: 0,
    on: std::ptr::null_mut(),
    laston: 0,
};
static mut always_off: boolean = false_;
static mut chat_dest: [u8; MAXPLAYERS] = [0; MAXPLAYERS];
static mut w_inputbuffer: [hu_itext_t; MAXPLAYERS] = [hu_itext_t {
    l: hu_textline_t {
        x: 0,
        y: 0,
        f: std::ptr::null_mut(),
        sc: 0,
        l: [0; HU_MAXLINELENGTH + 1],
        len: 0,
        needsupdate: 0,
    },
    lm: 0,
    on: std::ptr::null_mut(),
    laston: 0,
}; MAXPLAYERS];

static mut message_on: boolean = 0;
pub static mut message_dontfuckwithme: boolean = 0;
static mut message_nottobefuckedwith: boolean = 0;
static mut w_message: hu_stext_t = hu_stext_t {
    l: [hu_textline_t {
        x: 0,
        y: 0,
        f: std::ptr::null_mut(),
        sc: 0,
        l: [0; HU_MAXLINELENGTH + 1],
        len: 0,
        needsupdate: 0,
    }; HU_MAXLINES],
    h: 0,
    cl: 0,
    on: std::ptr::null_mut(),
    laston: 0,
};
static mut message_counter: i32 = 0;
static mut headsupactive: boolean = false_;

pub static mut mapnames: [&'static str; 45] = [
    HUSTR_E1M1,
    HUSTR_E1M2,
    HUSTR_E1M3,
    HUSTR_E1M4,
    HUSTR_E1M5,
    HUSTR_E1M6,
    HUSTR_E1M7,
    HUSTR_E1M8,
    HUSTR_E1M9,
    HUSTR_E2M1,
    HUSTR_E2M2,
    HUSTR_E2M3,
    HUSTR_E2M4,
    HUSTR_E2M5,
    HUSTR_E2M6,
    HUSTR_E2M7,
    HUSTR_E2M8,
    HUSTR_E2M9,
    HUSTR_E3M1,
    HUSTR_E3M2,
    HUSTR_E3M3,
    HUSTR_E3M4,
    HUSTR_E3M5,
    HUSTR_E3M6,
    HUSTR_E3M7,
    HUSTR_E3M8,
    HUSTR_E3M9,
    HUSTR_E4M1,
    HUSTR_E4M2,
    HUSTR_E4M3,
    HUSTR_E4M4,
    HUSTR_E4M5,
    HUSTR_E4M6,
    HUSTR_E4M7,
    HUSTR_E4M8,
    HUSTR_E4M9,
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
    "NEWLEVEL",
];

pub static mut mapnames2: [&'static str; 32] = [
    HUSTR_1, HUSTR_2, HUSTR_3, HUSTR_4, HUSTR_5, HUSTR_6, HUSTR_7, HUSTR_8, HUSTR_9, HUSTR_10,
    HUSTR_11, HUSTR_12, HUSTR_13, HUSTR_14, HUSTR_15, HUSTR_16, HUSTR_17, HUSTR_18, HUSTR_19,
    HUSTR_20, HUSTR_21, HUSTR_22, HUSTR_23, HUSTR_24, HUSTR_25, HUSTR_26, HUSTR_27, HUSTR_28,
    HUSTR_29, HUSTR_30, HUSTR_31, HUSTR_32,
];

pub static french_shiftxform: [u8; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'"', b'(', b')', b'*', b'+',
    b'?', b'_', b'>', b'?', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'/', b'.',
    b'<', b'+', b'>', b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[',
    b'!', b']', b'"', b'_', b'\'', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    b'{', b'|', b'}', b'~', 127,
];

pub static english_shiftxform: [u8; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'"', b'(', b')', b'*', b'+',
    b'<', b'_', b'>', b'?', b')', b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b':', b':',
    b'<', b'+', b'>', b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[',
    b'!', b']', b'"', b'_', b'\'', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    b'{', b'|', b'}', b'~', 127,
];

pub static mut shiftxform: *const u8 = english_shiftxform.as_ptr();

pub static frenchKeyMap: [u8; 128] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'%', b'(', b')', b'*', b'+',
    b';', b'-', b':', b'!', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b'M',
    b'<', b'=', b'>', b'?', b'@', b'Q', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
    b'L', b',', b'N', b'O', b'P', b'A', b'R', b'S', b'T', b'U', b'V', b'Z', b'X', b'Y', b'W', b'^',
    b'\\', b'$', b'^', b'_', b'@', b'Q', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b',', b'N', b'O', b'P', b'A', b'R', b'S', b'T', b'U', b'V', b'Z', b'X', b'Y', b'W',
    b'^', b'\\', b'$', b'^', 127,
];

pub fn ForeignTranslation(ch: u8) -> u8 {
    if ch < 128 {
        frenchKeyMap[ch as usize]
    } else {
        ch
    }
}

pub unsafe fn HU_Init() {
    shiftxform = french_shiftxform.as_ptr();

    let mut j = HU_FONTSTART as i32;
    for i in 0..HU_FONTSIZE {
        // Formatted lump name matches C sprintf(buffer, "STCFN%.3d", j) byte-for-byte
        let name = format!("STCFN{:03}", j);
        j += 1;
        hu_font[i] = W_CacheLumpNameStr(&name, PU_STATIC) as *mut patch_t;
    }
}

pub unsafe fn HU_Stop() {
    headsupactive = false_;
}

pub unsafe fn HU_Start() {
    if headsupactive != 0 {
        HU_Stop();
    }

    plr = &mut players[consoleplayer as usize];
    message_on = false_;
    message_dontfuckwithme = false_;
    message_nottobefuckedwith = false_;
    chat_on = false_;

    HUlib_initSText(
        &mut w_message,
        HU_MSGX,
        HU_MSGY,
        HU_MSGHEIGHT,
        hu_font.as_mut_ptr(),
        HU_FONTSTART as i32,
        &mut message_on,
    );

    let title_y = 167 - (SHORT((*hu_font[0]).height) as i32);
    HUlib_initTextLine(
        &mut w_title,
        HU_TITLEX,
        title_y,
        hu_font.as_mut_ptr(),
        HU_FONTSTART as i32,
    );

    let s = match gamemode {
        shareware | registered | retail => {
            let idx = ((gameepisode - 1) * 9 + gamemap - 1) as usize;
            if idx < mapnames.len() {
                mapnames[idx]
            } else {
                "NEWLEVEL"
            }
        }
        _ => {
            let idx = (gamemap - 1) as usize;
            if idx < mapnames2.len() {
                mapnames2[idx]
            } else {
                "NEWLEVEL"
            }
        }
    };

    for &b in s.as_bytes() {
        HUlib_addCharToTextLine(&mut w_title, b);
    }

    let input_y = HU_MSGY + HU_MSGHEIGHT * ((SHORT((*hu_font[0]).height) as i32) + 1);
    HUlib_initIText(
        &mut w_chat,
        HU_INPUTX,
        input_y,
        hu_font.as_mut_ptr(),
        HU_FONTSTART as i32,
        &mut chat_on,
    );

    for i in 0..MAXPLAYERS {
        HUlib_initIText(
            &mut w_inputbuffer[i],
            0,
            0,
            std::ptr::null_mut(),
            0,
            &mut always_off,
        );
    }

    headsupactive = true_;
}

pub unsafe fn HU_Drawer() {
    HUlib_drawSText(&mut w_message);
    HUlib_drawIText(&mut w_chat);
    if automapactive != 0 {
        HUlib_drawTextLine(&mut w_title, false_);
    }
}

pub unsafe fn HU_Erase() {
    HUlib_eraseSText(&mut w_message);
    HUlib_eraseIText(&mut w_chat);
    HUlib_eraseTextLine(&mut w_title);
}

pub unsafe fn HU_Ticker() {
    if message_counter != 0 {
        message_counter -= 1;
        if message_counter == 0 {
            message_on = false_;
            message_nottobefuckedwith = false_;
        }
    }

    if showMessages != 0 || message_dontfuckwithme != 0 {
        if (!(*plr).message.is_null() && message_nottobefuckedwith == 0)
            || (!(*plr).message.is_null() && message_dontfuckwithme != 0)
        {
            HUlib_addMessageToSText(&mut w_message, std::ptr::null(), (*plr).message);
            (*plr).message = std::ptr::null();
            message_on = true_;
            message_counter = HU_MSGTIMEOUT;
            message_nottobefuckedwith = message_dontfuckwithme;
            message_dontfuckwithme = 0;
        }
    }

    if netgame != 0 {
        for i in 0..MAXPLAYERS {
            if playeringame[i] == 0 {
                continue;
            }
            let mut c = players[i].cmd.chatchar;
            if i != (consoleplayer as usize) && c != 0 {
                if c <= HU_BROADCAST {
                    chat_dest[i] = c;
                } else {
                    if c >= b'a' && c <= b'z' {
                        c = *shiftxform.add(c as usize);
                    }
                    let rc = HUlib_keyInIText(&mut w_inputbuffer[i], c);
                    if rc != 0 && c == (KEY_ENTER as u8) {
                        if w_inputbuffer[i].l.len != 0
                            && (chat_dest[i] == (consoleplayer as u8) + 1
                                || chat_dest[i] == HU_BROADCAST)
                        {
                            HUlib_addMessageToSText(
                                &mut w_message,
                                player_names[i],
                                w_inputbuffer[i].l.l.as_ptr(),
                            );
                            message_nottobefuckedwith = true_;
                            message_on = true_;
                            message_counter = HU_MSGTIMEOUT;
                            if gamemode == commercial {
                                S_StartSound(std::ptr::null_mut(), sfx_radio);
                            } else {
                                S_StartSound(std::ptr::null_mut(), sfx_tink);
                            }
                        }
                        HUlib_resetIText(&mut w_inputbuffer[i]);
                    }
                }
                players[i].cmd.chatchar = 0;
            }
        }
    }
}

const QUEUESIZE: usize = 128;
static mut chatchars: [u8; QUEUESIZE] = [0; QUEUESIZE];
static mut head: usize = 0;
static mut tail: usize = 0;

pub unsafe fn HU_queueChatChar(c: u8) {
    if ((head + 1) & (QUEUESIZE - 1)) == tail {
        (*plr).message = b"[Message unsent]\0".as_ptr();
    } else {
        chatchars[head] = c;
        head = (head + 1) & (QUEUESIZE - 1);
    }
}

pub unsafe fn HU_dequeueChatChar() -> u8 {
    if head != tail {
        let c = chatchars[tail];
        tail = (tail + 1) & (QUEUESIZE - 1);
        c
    } else {
        0
    }
}

static mut lastmessage: [u8; HU_MAXLINELENGTH + 1] = [0; HU_MAXLINELENGTH + 1];
static mut shiftdown: boolean = false_;
static mut altdown: boolean = false_;
static mut num_nobrainers: i32 = 0;

pub unsafe fn HU_Responder(ev: *mut event_t) -> boolean {
    let mut eatkey = false_;
    let destination_keys: [u8; MAXPLAYERS] = [
        HUSTR_KEYGREEN,
        HUSTR_KEYINDIGO,
        HUSTR_KEYBROWN,
        HUSTR_KEYRED,
    ];

    let mut numplayers = 0;
    for i in 0..MAXPLAYERS {
        numplayers += playeringame[i];
    }

    if (*ev).data1 == KEY_RSHIFT {
        shiftdown = ((*ev).type_ == ev_keydown) as boolean;
        return false_;
    } else if (*ev).data1 == KEY_RALT || (*ev).data1 == KEY_LALT {
        altdown = ((*ev).type_ == ev_keydown) as boolean;
        return false_;
    }

    if (*ev).type_ != ev_keydown {
        return false_;
    }

    if chat_on == 0 {
        if (*ev).data1 == HU_MSGREFRESH {
            message_on = true_;
            message_counter = HU_MSGTIMEOUT;
            eatkey = true_;
        } else if netgame != 0 && (*ev).data1 == (HU_INPUTTOGGLE as i32) {
            eatkey = true_;
            chat_on = true_;
            HUlib_resetIText(&mut w_chat);
            HU_queueChatChar(HU_BROADCAST);
        } else if netgame != 0 && numplayers > 2 {
            for i in 0..MAXPLAYERS {
                if (*ev).data1 == (destination_keys[i] as i32) {
                    if playeringame[i] != 0 && i != (consoleplayer as usize) {
                        eatkey = true_;
                        chat_on = true_;
                        HUlib_resetIText(&mut w_chat);
                        HU_queueChatChar((i + 1) as u8);
                        break;
                    } else if i == (consoleplayer as usize) {
                        num_nobrainers += 1;
                        if num_nobrainers < 3 {
                            (*plr).message = b"You mumble to yourself\0".as_ptr();
                        } else if num_nobrainers < 6 {
                            (*plr).message = b"Who's there?\0".as_ptr();
                        } else if num_nobrainers < 9 {
                            (*plr).message = b"You scare yourself\0".as_ptr();
                        } else if num_nobrainers < 32 {
                            (*plr).message = b"You start to rave\0".as_ptr();
                        } else {
                            (*plr).message = b"You've lost it...\0".as_ptr();
                        }
                    }
                }
            }
        }
    } else {
        let mut c = (*ev).data1 as u8;
        if altdown != 0 {
            c = c.wrapping_sub(b'0');
            if c > 9 {
                return false_;
            }
            let mut macromessage = chat_macros[c as usize];
            HU_queueChatChar(KEY_ENTER as u8);
            while *macromessage != 0 {
                HU_queueChatChar(*macromessage);
                macromessage = macromessage.add(1);
            }
            HU_queueChatChar(KEY_ENTER as u8);
            chat_on = false_;
            c_strcpy(lastmessage.as_mut_ptr(), chat_macros[c as usize]);
            (*plr).message = lastmessage.as_ptr();
            eatkey = true_;
        } else {
            if french != 0 {
                c = ForeignTranslation(c);
            }
            if shiftdown != 0 || (c >= b'a' && c <= b'z') {
                if (c as usize) < 128 {
                    c = *shiftxform.add(c as usize);
                }
            }
            eatkey = HUlib_keyInIText(&mut w_chat, c);
            if eatkey != 0 {
                HU_queueChatChar(c);
            }
            if c == (KEY_ENTER as u8) {
                chat_on = false_;
                if w_chat.l.len != 0 {
                    c_strcpy(lastmessage.as_mut_ptr(), w_chat.l.l.as_ptr());
                    (*plr).message = lastmessage.as_ptr();
                }
            } else if c == (KEY_ESCAPE as u8) {
                chat_on = false_;
            }
        }
    }

    eatkey
}
