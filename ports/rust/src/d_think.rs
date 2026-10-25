// Based on id Software's linuxdoom-1.10 (GNU GPL)

use std::ffi::c_void;

pub type actionf_v = Option<unsafe fn()>;
pub type actionf_p1 = Option<unsafe fn(*mut c_void)>;
pub type actionf_p2 = Option<unsafe fn(*mut c_void, *mut c_void)>;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum actionf_t {
    None,
    Removed,
    acv(unsafe fn()),
    acp1(unsafe fn(*mut c_void)),
    acp2(unsafe fn(*mut c_void, *mut c_void)),
}

impl Default for actionf_t {
    fn default() -> Self {
        actionf_t::None
    }
}

pub type think_t = actionf_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thinker_t {
    pub prev: *mut thinker_t,
    pub next: *mut thinker_t,
    pub function: think_t,
}

impl Default for thinker_t {
    fn default() -> Self {
        Self {
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
            function: actionf_t::None,
        }
    }
}
