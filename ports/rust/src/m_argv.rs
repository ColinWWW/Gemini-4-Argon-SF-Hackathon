// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::c_strcasecmp;

pub static mut myargc: i32 = 0;
pub static mut myargv: *mut *mut u8 = std::ptr::null_mut();

pub unsafe fn M_CheckParm(check: *const u8) -> i32 {
    for i in 1..myargc {
        if c_strcasecmp(check, *myargv.add(i as usize)) == 0 {
            return i;
        }
    }
    0
}

pub unsafe fn M_CheckParmStr(check: &str) -> i32 {
    let mut buf = Vec::with_capacity(check.len() + 1);
    buf.extend_from_slice(check.as_bytes());
    buf.push(0);
    M_CheckParm(buf.as_ptr())
}
