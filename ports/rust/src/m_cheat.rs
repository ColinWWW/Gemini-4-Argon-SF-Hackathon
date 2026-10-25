// Based on id Software's linuxdoom-1.10 (GNU GPL)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cheatseq_t {
    pub sequence: *mut u8,
    pub p: *mut u8,
}

#[inline]
pub const fn SCRAMBLE(a: u8) -> u8 {
    ((a & 1) << 7)
        + ((a & 2) << 5)
        + (a & 4)
        + ((a & 8) << 1)
        + ((a & 16) >> 1)
        + (a & 32)
        + ((a & 64) >> 5)
        + ((a & 128) >> 7)
}

static mut firsttime: i32 = 1;
static mut cheat_xlate_table: [u8; 256] = [0; 256];

pub unsafe fn cht_CheckCheat(cht: *mut cheatseq_t, key: u8) -> i32 {
    let mut rc = 0;

    if firsttime != 0 {
        firsttime = 0;
        for i in 0..256 {
            cheat_xlate_table[i] = SCRAMBLE(i as u8);
        }
    }

    if (*cht).p.is_null() {
        (*cht).p = (*cht).sequence;
    }

    if *(*cht).p == 0 {
        *(*cht).p = key;
        (*cht).p = (*cht).p.add(1);
    } else if cheat_xlate_table[key as usize] == *(*cht).p {
        (*cht).p = (*cht).p.add(1);
    } else {
        (*cht).p = (*cht).sequence;
    }

    if *(*cht).p == 1 {
        (*cht).p = (*cht).p.add(1);
    } else if *(*cht).p == 0xff {
        (*cht).p = (*cht).sequence;
        rc = 1;
    }

    rc
}

pub unsafe fn cht_GetParam(cht: *mut cheatseq_t, mut buffer: *mut u8) {
    let mut p = (*cht).sequence;
    while *p != 1 {
        p = p.add(1);
    }
    p = p.add(1);

    loop {
        let c = *p;
        *buffer = c;
        buffer = buffer.add(1);
        *p = 0;
        p = p.add(1);
        if c == 0 || *p == 0xff {
            break;
        }
    }

    if *p == 0xff {
        *buffer = 0;
    }
}
