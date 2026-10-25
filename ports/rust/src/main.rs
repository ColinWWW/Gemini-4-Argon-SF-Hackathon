// Based on id Software's linuxdoom-1.10 (GNU GPL)
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_unsafe,
    static_mut_refs,
    clashing_extern_declarations,
    improper_ctypes_definitions,
    ambiguous_glob_reexports,
    clippy::all
)]

pub mod alloca;
pub mod am_map;
pub mod d_englsh;
pub mod d_event;
pub mod d_french;
pub mod d_items;
pub mod d_main;
pub mod d_net;
pub mod d_player;
pub mod d_textur;
pub mod d_think;
pub mod d_ticcmd;
pub mod doomdata;
pub mod doomdef;
pub mod doomstat;
pub mod doomtype;
pub mod dstrings;
pub mod f_finale;
pub mod f_wipe;
pub mod g_game;
pub mod hu_lib;
pub mod hu_stuff;
pub mod i_main;
pub mod i_net;
pub mod i_sound;
pub mod i_system;
pub mod i_video;
pub mod info;
#[path = "info_data.rs"]
pub mod info_data;
pub mod m_argv;
pub mod m_bbox;
pub mod m_cheat;
pub mod m_fixed;
pub mod m_menu;
pub mod m_misc;
pub mod m_random;
pub mod m_swap;
pub mod p_ceilng;
pub mod p_doors;
pub mod p_enemy;
pub mod p_floor;
pub mod p_inter;
pub mod p_lights;
pub mod p_local;
pub mod p_map;
pub mod p_maputl;
pub mod p_mobj;
pub mod p_plats;
pub mod p_pspr;
pub mod p_saveg;
pub mod p_setup;
pub mod p_sight;
pub mod p_spec;
pub mod p_switch;
pub mod p_telept;
pub mod p_tick;
pub mod p_user;
pub mod r_bsp;
pub mod r_data;
pub mod r_defs;
pub mod r_draw;
pub mod r_local;
pub mod r_main;
pub mod r_plane;
pub mod r_segs;
pub mod r_sky;
pub mod r_state;
pub mod r_things;
pub mod s_sound;
pub mod sounds;
pub mod st_lib;
pub mod st_stuff;
pub mod tables;
#[path = "tables_data.rs"]
pub mod tables_data;
pub mod v_video;
pub mod values;
pub mod w_wad;
pub mod wi_stuff;
pub mod z_zone;

pub unsafe fn c_str(ptr: *const u8) -> &'static str {
    if ptr.is_null() {
        return "";
    }
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
}

pub unsafe fn c_strlen(ptr: *const u8) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    len
}

pub unsafe fn c_strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;
    loop {
        let ch = *src.add(i);
        *dst.add(i) = ch;
        if ch == 0 {
            break;
        }
        i += 1;
    }
    dst
}

pub unsafe fn c_strncpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        let ch = *src.add(i);
        *dst.add(i) = ch;
        i += 1;
        if ch == 0 {
            while i < n {
                *dst.add(i) = 0;
                i += 1;
            }
            break;
        }
    }
    dst
}

pub unsafe fn c_strcat(dst: *mut u8, src: *const u8) -> *mut u8 {
    let len = c_strlen(dst);
    c_strcpy(dst.add(len), src);
    dst
}

pub unsafe fn c_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
        i += 1;
    }
}

pub unsafe fn c_strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
    }
    0
}

pub unsafe fn c_strcasecmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let c1 = (*s1.add(i)).to_ascii_lowercase();
        let c2 = (*s2.add(i)).to_ascii_lowercase();
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
        i += 1;
    }
}

pub unsafe fn c_strncasecmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let c1 = (*s1.add(i)).to_ascii_lowercase();
        let c2 = (*s2.add(i)).to_ascii_lowercase();
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
    }
    0
}

pub unsafe fn c_atoi(s: *const u8) -> i32 {
    let st = c_str(s).trim();
    let mut sign = 1i32;
    let mut chars = st.chars().peekable();
    if let Some(&c) = chars.peek() {
        if c == '-' {
            sign = -1;
            chars.next();
        } else if c == '+' {
            chars.next();
        }
    }
    let mut val = 0i32;
    while let Some(&c) = chars.peek() {
        if let Some(d) = c.to_digit(10) {
            val = val.wrapping_mul(10).wrapping_add(d as i32);
            chars.next();
        } else {
            break;
        }
    }
    val * sign
}

pub unsafe fn write_c_str(dst: *mut u8, s: &str) {
    std::ptr::copy_nonoverlapping(s.as_ptr(), dst, s.len());
    *dst.add(s.len()) = 0;
}

fn main() {
    unsafe {
        i_main::main_entry();
    }
}
