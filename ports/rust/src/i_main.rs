// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::d_main::*;
use crate::m_argv::*;

pub unsafe fn main_entry() {
    let args: Vec<String> = std::env::args().collect();
    myargc = args.len() as i32;

    let mut ptrs: Vec<*mut u8> = Vec::with_capacity(args.len() + 1);
    for arg in &args {
        let layout = std::alloc::Layout::from_size_align(arg.len() + 1, 1).unwrap();
        let p = std::alloc::alloc(layout);
        std::ptr::copy_nonoverlapping(arg.as_ptr(), p, arg.len());
        *p.add(arg.len()) = 0;
        ptrs.push(p);
    }
    ptrs.push(std::ptr::null_mut());

    let argv_layout =
        std::alloc::Layout::from_size_align(ptrs.len() * std::mem::size_of::<*mut u8>(), 8)
            .unwrap();
    myargv = std::alloc::alloc(argv_layout) as *mut *mut u8;
    for (i, &p) in ptrs.iter().enumerate() {
        *myargv.add(i) = p;
    }

    D_DoomMain();
}
