// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdef::*;
use crate::doomtype::*;
use crate::m_swap::*;
use crate::r_defs::*;
use crate::r_draw::*;
use crate::v_video::*;

pub const BG: i32 = 1;
pub const FG: i32 = 0;

pub const HU_CHARERASE: u8 = KEY_BACKSPACE as u8;
pub const HU_MAXLINES: usize = 4;
pub const HU_MAXLINELENGTH: usize = 80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_textline_t {
    pub x: i32,
    pub y: i32,
    pub f: *mut *mut patch_t,
    pub sc: i32,
    pub l: [u8; HU_MAXLINELENGTH + 1],
    pub len: i32,
    pub needsupdate: i32,
}

impl Default for hu_textline_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_stext_t {
    pub l: [hu_textline_t; HU_MAXLINES],
    pub h: i32,
    pub cl: i32,
    pub on: *mut boolean,
    pub laston: boolean,
}

impl Default for hu_stext_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hu_itext_t {
    pub l: hu_textline_t,
    pub lm: i32,
    pub on: *mut boolean,
    pub laston: boolean,
}

impl Default for hu_itext_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub unsafe fn HUlib_init() {}

pub unsafe fn HUlib_clearTextLine(t: *mut hu_textline_t) {
    (*t).len = 0;
    (*t).l[0] = 0;
    (*t).needsupdate = true_;
}

pub unsafe fn HUlib_initTextLine(
    t: *mut hu_textline_t,
    x: i32,
    y: i32,
    f: *mut *mut patch_t,
    sc: i32,
) {
    (*t).x = x;
    (*t).y = y;
    (*t).f = f;
    (*t).sc = sc;
    HUlib_clearTextLine(t);
}

pub unsafe fn HUlib_addCharToTextLine(t: *mut hu_textline_t, ch: u8) -> boolean {
    if (*t).len as usize == HU_MAXLINELENGTH {
        false_
    } else {
        (*t).l[(*t).len as usize] = ch;
        (*t).len += 1;
        (*t).l[(*t).len as usize] = 0;
        (*t).needsupdate = 4;
        true_
    }
}

pub unsafe fn HUlib_delCharFromTextLine(t: *mut hu_textline_t) -> boolean {
    if (*t).len == 0 {
        false_
    } else {
        (*t).len -= 1;
        (*t).l[(*t).len as usize] = 0;
        (*t).needsupdate = 4;
        true_
    }
}

pub unsafe fn HUlib_drawTextLine(l: *mut hu_textline_t, drawcursor: boolean) {
    let mut x = (*l).x;
    for i in 0..((*l).len as usize) {
        let c = (*l).l[i].to_ascii_uppercase();
        if c != b' ' && (c as i32) >= (*l).sc && c <= b'_' {
            let patch = *(*l).f.add(((c as i32) - (*l).sc) as usize);
            let w = SHORT((*patch).width) as i32;
            if x + w > SCREENWIDTH {
                break;
            }
            V_DrawPatchDirect(x, (*l).y, FG, patch);
            x += w;
        } else {
            x += 4;
            if x >= SCREENWIDTH {
                break;
            }
        }
    }

    let us_patch = *(*l).f.add((b'_' as i32 - (*l).sc) as usize);
    if drawcursor != 0 && x + (SHORT((*us_patch).width) as i32) <= SCREENWIDTH {
        V_DrawPatchDirect(x, (*l).y, FG, us_patch);
    }
}

pub unsafe fn HUlib_eraseTextLine(l: *mut hu_textline_t) {
    if crate::am_map::automapactive == 0 && viewwindowx != 0 && (*l).needsupdate != 0 {
        let lh = (SHORT((*(*(*l).f)).height) as i32) + 1;
        let mut y = (*l).y;
        let mut yoffset = y * SCREENWIDTH;
        while y < (*l).y + lh {
            if y < viewwindowy || y >= viewwindowy + viewheight {
                R_VideoErase(yoffset as u32, SCREENWIDTH);
            } else {
                R_VideoErase(yoffset as u32, viewwindowx);
                R_VideoErase((yoffset + viewwindowx + viewwidth) as u32, viewwindowx);
            }
            y += 1;
            yoffset += SCREENWIDTH;
        }
    }

    if (*l).needsupdate != 0 {
        (*l).needsupdate -= 1;
    }
}

pub unsafe fn HUlib_initSText(
    s: *mut hu_stext_t,
    x: i32,
    y: i32,
    h: i32,
    font: *mut *mut patch_t,
    startchar: i32,
    on: *mut boolean,
) {
    (*s).h = h;
    (*s).on = on;
    (*s).laston = true_;
    (*s).cl = 0;
    let fh = (SHORT((*(*font)).height) as i32) + 1;
    for i in 0..h {
        HUlib_initTextLine(&mut (*s).l[i as usize], x, y - i * fh, font, startchar);
    }
}

pub unsafe fn HUlib_addLineToSText(s: *mut hu_stext_t) {
    (*s).cl += 1;
    if (*s).cl == (*s).h {
        (*s).cl = 0;
    }
    HUlib_clearTextLine(&mut (*s).l[(*s).cl as usize]);
    for i in 0..(*s).h {
        (*s).l[i as usize].needsupdate = 4;
    }
}

pub unsafe fn HUlib_addMessageToSText(
    s: *mut hu_stext_t,
    mut prefix: *const u8,
    mut msg: *const u8,
) {
    HUlib_addLineToSText(s);
    if !prefix.is_null() {
        while *prefix != 0 {
            HUlib_addCharToTextLine(&mut (*s).l[(*s).cl as usize], *prefix);
            prefix = prefix.add(1);
        }
    }
    while *msg != 0 {
        HUlib_addCharToTextLine(&mut (*s).l[(*s).cl as usize], *msg);
        msg = msg.add(1);
    }
}

pub unsafe fn HUlib_drawSText(s: *mut hu_stext_t) {
    if *(*s).on == 0 {
        return;
    }

    for i in 0..(*s).h {
        let mut idx = (*s).cl - i;
        if idx < 0 {
            idx += (*s).h;
        }
        let l = &mut (*s).l[idx as usize];
        HUlib_drawTextLine(l, false_);
    }
}

pub unsafe fn HUlib_eraseSText(s: *mut hu_stext_t) {
    for i in 0..(*s).h {
        if (*s).laston != 0 && *(*s).on == 0 {
            (*s).l[i as usize].needsupdate = 4;
        }
        HUlib_eraseTextLine(&mut (*s).l[i as usize]);
    }
    (*s).laston = *(*s).on;
}

pub unsafe fn HUlib_initIText(
    it: *mut hu_itext_t,
    x: i32,
    y: i32,
    font: *mut *mut patch_t,
    startchar: i32,
    on: *mut boolean,
) {
    (*it).lm = 0;
    (*it).on = on;
    (*it).laston = true_;
    HUlib_initTextLine(&mut (*it).l, x, y, font, startchar);
}

pub unsafe fn HUlib_delCharFromIText(it: *mut hu_itext_t) {
    if (*it).l.len != (*it).lm {
        HUlib_delCharFromTextLine(&mut (*it).l);
    }
}

pub unsafe fn HUlib_eraseLineFromIText(it: *mut hu_itext_t) {
    while (*it).lm != (*it).l.len {
        HUlib_delCharFromTextLine(&mut (*it).l);
    }
}

pub unsafe fn HUlib_resetIText(it: *mut hu_itext_t) {
    (*it).lm = 0;
    HUlib_clearTextLine(&mut (*it).l);
}

pub unsafe fn HUlib_addPrefixToIText(it: *mut hu_itext_t, mut str_: *const u8) {
    while *str_ != 0 {
        HUlib_addCharToTextLine(&mut (*it).l, *str_);
        str_ = str_.add(1);
    }
    (*it).lm = (*it).l.len;
}

pub unsafe fn HUlib_keyInIText(it: *mut hu_itext_t, ch: u8) -> boolean {
    if ch >= b' ' && ch <= b'_' {
        HUlib_addCharToTextLine(&mut (*it).l, ch);
    } else if ch == KEY_BACKSPACE as u8 {
        HUlib_delCharFromIText(it);
    } else if ch != KEY_ENTER as u8 {
        return false_;
    }
    true_
}

pub unsafe fn HUlib_drawIText(it: *mut hu_itext_t) {
    if *(*it).on == 0 {
        return;
    }
    HUlib_drawTextLine(&mut (*it).l, true_);
}

pub unsafe fn HUlib_eraseIText(it: *mut hu_itext_t) {
    if (*it).laston != 0 && *(*it).on == 0 {
        (*it).l.needsupdate = 4;
    }
    HUlib_eraseTextLine(&mut (*it).l);
    (*it).laston = *(*it).on;
}
