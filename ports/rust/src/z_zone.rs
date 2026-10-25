// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use crate::doomtype::*;
use crate::i_system::*;

pub const PU_STATIC: i32 = 1;
pub const PU_SOUND: i32 = 2;
pub const PU_MUSIC: i32 = 3;
pub const PU_DAVE: i32 = 4;
pub const PU_LEVEL: i32 = 50;
pub const PU_LEVSPEC: i32 = 51;
pub const PU_PURGELEVEL: i32 = 100;
pub const PU_CACHE: i32 = 101;

pub const ZONEID: i32 = 0x1d4a11;
pub const MINFRAGMENT: i32 = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock_t {
    pub size: i32,
    pub user: *mut *mut c_void,
    pub tag: i32,
    pub id: i32,
    pub next: *mut memblock_t,
    pub prev: *mut memblock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memzone_t {
    pub size: i32,
    pub blocklist: memblock_t,
    pub rover: *mut memblock_t,
}

pub static mut mainzone: *mut memzone_t = std::ptr::null_mut();

pub unsafe fn Z_ClearZone(zone: *mut memzone_t) {
    let block = (zone as *mut byte).add(std::mem::size_of::<memzone_t>()) as *mut memblock_t;
    (*zone).blocklist.next = block;
    (*zone).blocklist.prev = block;
    (*zone).blocklist.user = zone as *mut *mut c_void;
    (*zone).blocklist.tag = PU_STATIC;
    (*zone).rover = block;

    (*block).prev = &mut (*zone).blocklist;
    (*block).next = &mut (*zone).blocklist;
    (*block).user = std::ptr::null_mut();
    (*block).size = (*zone).size - (std::mem::size_of::<memzone_t>() as i32);
}

pub unsafe fn Z_Init() {
    let mut size: i32 = 0;
    mainzone = I_ZoneBase(&mut size) as *mut memzone_t;
    (*mainzone).size = size;

    let block = (mainzone as *mut byte).add(std::mem::size_of::<memzone_t>()) as *mut memblock_t;
    (*mainzone).blocklist.next = block;
    (*mainzone).blocklist.prev = block;
    (*mainzone).blocklist.user = mainzone as *mut *mut c_void;
    (*mainzone).blocklist.tag = PU_STATIC;
    (*mainzone).rover = block;

    (*block).prev = &mut (*mainzone).blocklist;
    (*block).next = &mut (*mainzone).blocklist;
    (*block).user = std::ptr::null_mut();
    (*block).size = (*mainzone).size - (std::mem::size_of::<memzone_t>() as i32);
}

pub unsafe fn Z_Free(ptr: *mut c_void) {
    let mut block = (ptr as *mut byte).sub(std::mem::size_of::<memblock_t>()) as *mut memblock_t;

    if (*block).id != ZONEID {
        I_Error("Z_Free: freed a pointer without ZONEID");
    }

    if ((*block).user as usize) > 0x100 {
        *(*block).user = std::ptr::null_mut();
    }

    (*block).user = std::ptr::null_mut();
    (*block).tag = 0;
    (*block).id = 0;

    let mut other = (*block).prev;
    if (*other).user.is_null() {
        (*other).size += (*block).size;
        (*other).next = (*block).next;
        (*(*other).next).prev = other;

        if block == (*mainzone).rover {
            (*mainzone).rover = other;
        }
        block = other;
    }

    other = (*block).next;
    if (*other).user.is_null() {
        (*block).size += (*other).size;
        (*block).next = (*other).next;
        (*(*block).next).prev = block;

        if other == (*mainzone).rover {
            (*mainzone).rover = block;
        }
    }
}

pub unsafe fn Z_Malloc(mut size: i32, tag: i32, user: *mut c_void) -> *mut c_void {
    size = (size + 3) & !3;
    size += std::mem::size_of::<memblock_t>() as i32;

    let mut base = (*mainzone).rover;
    if (*(*base).prev).user.is_null() {
        base = (*base).prev;
    }

    let mut rover = base;
    let start = (*base).prev;

    loop {
        if rover == start {
            I_Error(&format!("Z_Malloc: failed on allocation of {} bytes", size));
        }

        if !(*rover).user.is_null() {
            if (*rover).tag < PU_PURGELEVEL {
                base = (*rover).next;
                rover = base;
            } else {
                base = (*base).prev;
                Z_Free((rover as *mut byte).add(std::mem::size_of::<memblock_t>()) as *mut c_void);
                base = (*base).next;
                rover = (*base).next;
            }
        } else {
            rover = (*rover).next;
        }

        if (*base).user.is_null() && (*base).size >= size {
            break;
        }
    }

    let extra = (*base).size - size;
    if extra > MINFRAGMENT {
        let newblock = (base as *mut byte).add(size as usize) as *mut memblock_t;
        (*newblock).size = extra;
        (*newblock).user = std::ptr::null_mut();
        (*newblock).tag = 0;
        (*newblock).prev = base;
        (*newblock).next = (*base).next;
        (*(*newblock).next).prev = newblock;

        (*base).next = newblock;
        (*base).size = size;
    }

    if !user.is_null() {
        (*base).user = user as *mut *mut c_void;
        *(user as *mut *mut c_void) =
            (base as *mut byte).add(std::mem::size_of::<memblock_t>()) as *mut c_void;
    } else {
        if tag >= PU_PURGELEVEL {
            I_Error("Z_Malloc: an owner is required for purgable blocks");
        }
        (*base).user = 2 as *mut *mut c_void;
    }
    (*base).tag = tag;

    (*mainzone).rover = (*base).next;
    (*base).id = ZONEID;

    (base as *mut byte).add(std::mem::size_of::<memblock_t>()) as *mut c_void
}

pub unsafe fn Z_FreeTags(lowtag: i32, hightag: i32) {
    let mut block = (*mainzone).blocklist.next;
    while block != &mut (*mainzone).blocklist {
        let next = (*block).next;
        if !(*block).user.is_null() {
            if (*block).tag >= lowtag && (*block).tag <= hightag {
                Z_Free((block as *mut byte).add(std::mem::size_of::<memblock_t>()) as *mut c_void);
            }
        }
        block = next;
    }
}

pub unsafe fn Z_DumpHeap(lowtag: i32, hightag: i32) {
    println!("zone size: {}  location: {:p}", (*mainzone).size, mainzone);
    println!("tag range: {} to {}", lowtag, hightag);

    let mut block = (*mainzone).blocklist.next;
    loop {
        if (*block).tag >= lowtag && (*block).tag <= hightag {
            println!(
                "block:{:p}    size:{:7}    user:{:p}    tag:{:3}",
                block,
                (*block).size,
                (*block).user,
                (*block).tag
            );
        }
        if (*block).next == &mut (*mainzone).blocklist {
            break;
        }
        if (block as *mut byte).add((*block).size as usize) != ((*block).next as *mut byte) {
            println!("ERROR: block size does not touch the next block");
        }
        if (*(*block).next).prev != block {
            println!("ERROR: next block doesn't have proper back link");
        }
        if (*block).user.is_null() && (*(*block).next).user.is_null() {
            println!("ERROR: two consecutive free blocks");
        }
        block = (*block).next;
    }
}

pub unsafe fn Z_FileDumpHeap(_f: *mut std::fs::File) {}

pub unsafe fn Z_CheckHeap() {
    let mut block = (*mainzone).blocklist.next;
    loop {
        if (*block).next == &mut (*mainzone).blocklist {
            break;
        }
        if (block as *mut byte).add((*block).size as usize) != ((*block).next as *mut byte) {
            I_Error("Z_CheckHeap: block size does not touch the next block\n");
        }
        if (*(*block).next).prev != block {
            I_Error("Z_CheckHeap: next block doesn't have proper back link\n");
        }
        if (*block).user.is_null() && (*(*block).next).user.is_null() {
            I_Error("Z_CheckHeap: two consecutive free blocks\n");
        }
        block = (*block).next;
    }
}

pub unsafe fn Z_ChangeTag2(ptr: *mut c_void, tag: i32) {
    let block = (ptr as *mut byte).sub(std::mem::size_of::<memblock_t>()) as *mut memblock_t;

    if (*block).id != ZONEID {
        I_Error("Z_ChangeTag: freed a pointer without ZONEID");
    }

    if tag >= PU_PURGELEVEL && ((*block).user as usize) < 0x100 {
        I_Error("Z_ChangeTag: an owner is required for purgable blocks");
    }

    (*block).tag = tag;
}

pub unsafe fn Z_ChangeTag(ptr: *mut c_void, tag: i32) {
    let block = (ptr as *mut byte).sub(std::mem::size_of::<memblock_t>()) as *mut memblock_t;
    if (*block).id != ZONEID {
        I_Error("Z_CT");
    }
    Z_ChangeTag2(ptr, tag);
}

pub unsafe fn Z_FreeMemory() -> i32 {
    let mut free = 0;
    let mut block = (*mainzone).blocklist.next;
    while block != &mut (*mainzone).blocklist {
        if (*block).user.is_null() || (*block).tag >= PU_PURGELEVEL {
            free += (*block).size;
        }
        block = (*block).next;
    }
    free
}
