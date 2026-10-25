// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_swap::*;
use crate::z_zone::*;
use crate::{c_str, c_strcasecmp, c_strlen, c_strncpy};

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct wadinfo_t {
    pub identification: [u8; 4],
    pub numlumps: i32,
    pub infotableofs: i32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct filelump_t {
    pub filepos: i32,
    pub size: i32,
    pub name: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lumpinfo_t {
    pub name: [u8; 8],
    pub handle: i32,
    pub position: i32,
    pub size: i32,
}

pub static mut lumpinfo: *mut lumpinfo_t = std::ptr::null_mut();
pub static mut numlumps: i32 = 0;
pub static mut lumpcache: *mut *mut c_void = std::ptr::null_mut();

pub static mut reloadlump: i32 = 0;
pub static mut reloadname: *const u8 = std::ptr::null();

static mut wad_files: Option<Vec<File>> = None;

unsafe fn open_wad_file(path: &str) -> i32 {
    if wad_files.is_none() {
        wad_files = Some(Vec::new());
    }
    match File::open(path) {
        Ok(f) => {
            let list = wad_files.as_mut().unwrap();
            let idx = list.len() as i32;
            list.push(f);
            idx
        }
        Err(_) => -1,
    }
}

unsafe fn get_wad_file(handle: i32) -> &'static mut File {
    let list = wad_files.as_mut().unwrap();
    &mut list[handle as usize]
}

unsafe fn close_wad_file(_handle: i32) {}

pub unsafe fn strupr(mut s: *mut u8) {
    while *s != 0 {
        *s = (*s).to_ascii_uppercase();
        s = s.add(1);
    }
}

pub unsafe fn filelength(handle: i32) -> i32 {
    let f = get_wad_file(handle);
    match f.metadata() {
        Ok(m) => m.len() as i32,
        Err(_) => {
            I_Error("Error fstating");
            0
        }
    }
}

pub unsafe fn ExtractFileBase(path: *const u8, mut dest: *mut u8) {
    let len = c_strlen(path);
    let mut src = path.add(if len > 0 { len - 1 } else { 0 });

    while src != path && *src.sub(1) != b'\\' && *src.sub(1) != b'/' {
        src = src.sub(1);
    }

    std::ptr::write_bytes(dest, 0, 8);
    let mut length = 0;

    while *src != 0 && *src != b'.' {
        length += 1;
        if length == 9 {
            I_Error(&format!("Filename base of {} >8 chars", c_str(path)));
        }
        *dest = (*src).to_ascii_uppercase();
        dest = dest.add(1);
        src = src.add(1);
    }
}

pub unsafe fn W_AddFile(mut filename: *const u8) {
    let mut header = wadinfo_t::default();
    let mut singleinfo = filelump_t::default();

    if *filename == b'~' {
        filename = filename.add(1);
        reloadname = filename;
        reloadlump = numlumps;
    }

    let fname_str = c_str(filename);
    let handle = open_wad_file(fname_str);
    if handle == -1 {
        println!(" couldn't open {}", fname_str);
        return;
    }

    println!(" adding {}", fname_str);
    let startlump = numlumps as usize;

    let mut fileinfo_vec: Vec<filelump_t> = Vec::new();
    let flen = c_strlen(filename);
    let is_wad = if flen >= 3 {
        c_strcasecmp(filename.add(flen - 3), b"wad\0".as_ptr()) == 0
    } else {
        false
    };

    if !is_wad {
        singleinfo.filepos = 0;
        singleinfo.size = LONG(filelength(handle));
        ExtractFileBase(filename, singleinfo.name.as_mut_ptr());
        fileinfo_vec.push(singleinfo);
        numlumps += 1;
    } else {
        let f = get_wad_file(handle);
        let hdr_slice = std::slice::from_raw_parts_mut(
            &mut header as *mut wadinfo_t as *mut u8,
            std::mem::size_of::<wadinfo_t>(),
        );
        let _ = f.read_exact(hdr_slice);

        if &header.identification != b"IWAD" && &header.identification != b"PWAD" {
            I_Error(&format!(
                "Wad file {} doesn't have IWAD or PWAD id\n",
                fname_str
            ));
        }

        header.numlumps = LONG(header.numlumps);
        header.infotableofs = LONG(header.infotableofs);
        let count = header.numlumps as usize;
        fileinfo_vec.resize(count, filelump_t::default());
        let _ = f.seek(SeekFrom::Start(header.infotableofs as u64));
        let info_slice = std::slice::from_raw_parts_mut(
            fileinfo_vec.as_mut_ptr() as *mut u8,
            count * std::mem::size_of::<filelump_t>(),
        );
        let _ = f.read_exact(info_slice);
        numlumps += header.numlumps;
    }

    let new_size = (numlumps as usize) * std::mem::size_of::<lumpinfo_t>();
    if lumpinfo.is_null() {
        let layout = std::alloc::Layout::from_size_align(new_size.max(1), 8).unwrap();
        lumpinfo = std::alloc::alloc_zeroed(layout) as *mut lumpinfo_t;
    } else {
        let old_size = startlump * std::mem::size_of::<lumpinfo_t>();
        let old_layout = std::alloc::Layout::from_size_align(old_size.max(1), 8).unwrap();
        lumpinfo = std::alloc::realloc(lumpinfo as *mut u8, old_layout, new_size.max(1))
            as *mut lumpinfo_t;
    }

    if lumpinfo.is_null() {
        I_Error("Couldn't realloc lumpinfo");
    }

    let storehandle = if !reloadname.is_null() { -1 } else { handle };

    for (idx, fi) in fileinfo_vec.iter().enumerate() {
        let lump_p = lumpinfo.add(startlump + idx);
        (*lump_p).handle = storehandle;
        (*lump_p).position = LONG(fi.filepos);
        (*lump_p).size = LONG(fi.size);
        c_strncpy((*lump_p).name.as_mut_ptr(), fi.name.as_ptr(), 8);
    }

    if !reloadname.is_null() {
        close_wad_file(handle);
    }
}

pub unsafe fn W_Reload() {
    if reloadname.is_null() {
        return;
    }
    let rname = c_str(reloadname);
    let handle = open_wad_file(rname);
    if handle == -1 {
        I_Error(&format!("W_Reload: couldn't open {}", rname));
    }

    let mut header = wadinfo_t::default();
    let f = get_wad_file(handle);
    let hdr_slice = std::slice::from_raw_parts_mut(
        &mut header as *mut wadinfo_t as *mut u8,
        std::mem::size_of::<wadinfo_t>(),
    );
    let _ = f.read_exact(hdr_slice);
    let lumpcount = LONG(header.numlumps) as usize;
    header.infotableofs = LONG(header.infotableofs);

    let mut fileinfo_vec = vec![filelump_t::default(); lumpcount];
    let _ = f.seek(SeekFrom::Start(header.infotableofs as u64));
    let info_slice = std::slice::from_raw_parts_mut(
        fileinfo_vec.as_mut_ptr() as *mut u8,
        lumpcount * std::mem::size_of::<filelump_t>(),
    );
    let _ = f.read_exact(info_slice);

    for i in 0..lumpcount {
        let idx = (reloadlump as usize) + i;
        let lump_p = lumpinfo.add(idx);
        if !(*lumpcache.add(idx)).is_null() {
            Z_Free(*lumpcache.add(idx));
        }
        (*lump_p).position = LONG(fileinfo_vec[i].filepos);
        (*lump_p).size = LONG(fileinfo_vec[i].size);
    }

    close_wad_file(handle);
}

pub unsafe fn W_InitMultipleFiles(mut filenames: *mut *mut u8) {
    numlumps = 0;
    lumpinfo = std::ptr::null_mut();

    while !(*filenames).is_null() {
        W_AddFile(*filenames);
        filenames = filenames.add(1);
    }

    if numlumps == 0 {
        I_Error("W_InitFiles: no files found");
    }

    let size = (numlumps as usize) * std::mem::size_of::<*mut c_void>();
    let layout = std::alloc::Layout::from_size_align(size.max(1), 8).unwrap();
    lumpcache = std::alloc::alloc_zeroed(layout) as *mut *mut c_void;
    if lumpcache.is_null() {
        I_Error("Couldn't allocate lumpcache");
    }
}

pub unsafe fn W_InitFile(filename: *mut u8) {
    let mut names: [*mut u8; 2] = [filename, std::ptr::null_mut()];
    W_InitMultipleFiles(names.as_mut_ptr());
}

pub unsafe fn W_NumLumps() -> i32 {
    numlumps
}

pub unsafe fn W_CheckNumForName(name: *const u8) -> i32 {
    let mut name8 = [0u8; 9];
    c_strncpy(name8.as_mut_ptr(), name, 8);
    name8[8] = 0;
    strupr(name8.as_mut_ptr());

    let mut i = numlumps;
    while i > 0 {
        i -= 1;
        let lump_p = lumpinfo.add(i as usize);
        let mut matched = true;
        for k in 0..8 {
            if (*lump_p).name[k].to_ascii_uppercase() != name8[k] {
                matched = false;
                break;
            }
            if name8[k] == 0 {
                break;
            }
        }
        if matched {
            return i;
        }
    }
    -1
}

pub unsafe fn W_CheckNumForNameStr(name: &str) -> i32 {
    let mut buf = [0u8; 9];
    let bytes = name.as_bytes();
    let len = bytes.len().min(8);
    buf[..len].copy_from_slice(&bytes[..len]);
    W_CheckNumForName(buf.as_ptr())
}

pub unsafe fn W_GetNumForName(name: *const u8) -> i32 {
    let i = W_CheckNumForName(name);
    if i == -1 {
        I_Error(&format!("W_GetNumForName: {} not found!", c_str(name)));
    }
    i
}

pub unsafe fn W_GetNumForNameStr(name: &str) -> i32 {
    let mut buf = [0u8; 9];
    let bytes = name.as_bytes();
    let len = bytes.len().min(8);
    buf[..len].copy_from_slice(&bytes[..len]);
    W_GetNumForName(buf.as_ptr())
}

pub unsafe fn W_LumpLength(lump: i32) -> i32 {
    if lump >= numlumps {
        I_Error(&format!("W_LumpLength: {} >= numlumps", lump));
    }
    (*lumpinfo.add(lump as usize)).size
}

pub unsafe fn W_ReadLump(lump: i32, dest: *mut c_void) {
    if lump >= numlumps {
        I_Error(&format!("W_ReadLump: {} >= numlumps", lump));
    }
    let l = lumpinfo.add(lump as usize);
    let handle = if (*l).handle == -1 {
        let h = open_wad_file(c_str(reloadname));
        if h == -1 {
            I_Error(&format!("W_ReadLump: couldn't open {}", c_str(reloadname)));
        }
        h
    } else {
        (*l).handle
    };

    let f = get_wad_file(handle);
    let _ = f.seek(SeekFrom::Start((*l).position as u64));
    let slice = std::slice::from_raw_parts_mut(dest as *mut u8, (*l).size as usize);
    match f.read_exact(slice) {
        Ok(_) => {}
        Err(_) => {
            I_Error(&format!("W_ReadLump: error reading lump {}", lump));
        }
    }
}

pub unsafe fn W_CacheLumpNum(lump: i32, tag: i32) -> *mut c_void {
    if (lump as u32) >= (numlumps as u32) {
        I_Error(&format!("W_CacheLumpNum: {} >= numlumps", lump));
    }

    let cache_slot = lumpcache.add(lump as usize);
    if (*cache_slot).is_null() {
        let _ptr = Z_Malloc(W_LumpLength(lump), tag, cache_slot as *mut c_void);
        W_ReadLump(lump, *cache_slot);
    } else {
        Z_ChangeTag(*cache_slot, tag);
    }

    *cache_slot
}

pub unsafe fn W_CacheLumpName(name: *const u8, tag: i32) -> *mut c_void {
    W_CacheLumpNum(W_GetNumForName(name), tag)
}

pub unsafe fn W_CacheLumpNameStr(name: &str, tag: i32) -> *mut c_void {
    W_CacheLumpNum(W_GetNumForNameStr(name), tag)
}

pub unsafe fn W_Profile() {}
