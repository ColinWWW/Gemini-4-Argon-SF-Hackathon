// Based on id Software's linuxdoom-1.10 (GNU GPL)

#[inline]
pub fn SwapSHORT(x: u16) -> u16 {
    (x >> 8) | (x << 8)
}

#[inline]
pub fn SwapLONG(x: u32) -> u32 {
    (x >> 24) | ((x >> 8) & 0xff00) | ((x << 8) & 0xff0000) | (x << 24)
}

#[inline]
pub fn SHORT(x: i16) -> i16 {
    x
}

#[inline]
pub fn USHORT(x: u16) -> u16 {
    x
}

#[inline]
pub fn LONG(x: i32) -> i32 {
    x
}

#[inline]
pub fn ULONG(x: u32) -> u32 {
    x
}
