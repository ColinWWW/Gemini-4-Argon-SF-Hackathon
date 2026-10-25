// Based on id Software's linuxdoom-1.10 (GNU GPL)

use crate::doomdef::*;
use crate::doomtype::*;
use crate::i_system::*;
use crate::m_bbox::*;
use crate::m_swap::*;
use crate::r_defs::*;

pub const CENTERY: i32 = SCREENHEIGHT / 2;

pub static mut screens: [*mut byte; 5] = [std::ptr::null_mut(); 5];
pub static mut dirtybox: [i32; 4] = [0; 4];
pub static mut usegamma: i32 = 0;

pub static mut gammatable: [[byte; 256]; 5] = [
    [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
        95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
        114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
        167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
        185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
        203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
        221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
    ],
    [
        2, 4, 5, 7, 8, 10, 11, 12, 14, 15, 16, 18, 19, 20, 21, 23, 24, 25, 26, 27, 29, 30, 31, 32,
        33, 34, 36, 37, 38, 39, 40, 41, 42, 44, 45, 46, 47, 48, 49, 50, 51, 52, 54, 55, 56, 57, 58,
        59, 60, 61, 62, 63, 64, 65, 66, 67, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
        83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
        104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121,
        122, 123, 124, 125, 126, 127, 128, 129, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
        139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 148, 149, 150, 151, 152, 153, 154, 155,
        156, 157, 158, 159, 160, 161, 162, 163, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
        173, 174, 175, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 186, 187, 188,
        189, 190, 191, 192, 193, 194, 195, 196, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205,
        205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 214, 215, 216, 217, 218, 219, 220, 221,
        222, 222, 223, 224, 225, 226, 227, 228, 229, 230, 230, 231, 232, 233, 234, 235, 236, 237,
        237, 238, 239, 240, 241, 242, 243, 244, 245, 245, 246, 247, 248, 249, 250, 251, 252, 252,
        253, 254, 255,
    ],
    [
        4, 7, 9, 11, 13, 15, 17, 19, 21, 22, 24, 26, 27, 29, 30, 32, 33, 35, 36, 38, 39, 40, 42, 43,
        45, 46, 47, 48, 50, 51, 52, 54, 55, 56, 57, 59, 60, 61, 62, 63, 65, 66, 67, 68, 69, 70, 72,
        73, 74, 75, 76, 77, 78, 79, 80, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 114,
        115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
        133, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 144, 145, 146, 147, 148,
        149, 150, 151, 152, 153, 153, 154, 155, 156, 157, 158, 159, 160, 160, 161, 162, 163, 164,
        165, 166, 166, 167, 168, 169, 170, 171, 172, 172, 173, 174, 175, 176, 177, 178, 178, 179,
        180, 181, 182, 183, 183, 184, 185, 186, 187, 188, 188, 189, 190, 191, 192, 193, 193, 194,
        195, 196, 197, 197, 198, 199, 200, 201, 201, 202, 203, 204, 205, 206, 206, 207, 208, 209,
        210, 210, 211, 212, 213, 213, 214, 215, 216, 217, 217, 218, 219, 220, 221, 221, 222, 223,
        224, 224, 225, 226, 227, 228, 228, 229, 230, 231, 231, 232, 233, 234, 235, 235, 236, 237,
        238, 238, 239, 240, 241, 241, 242, 243, 244, 244, 245, 246, 247, 247, 248, 249, 250, 251,
        251, 252, 253, 254, 254, 255,
    ],
    [
        8, 12, 16, 19, 22, 24, 27, 29, 31, 34, 36, 38, 40, 41, 43, 45, 47, 49, 50, 52, 53, 55, 57,
        58, 60, 61, 63, 64, 65, 67, 68, 70, 71, 72, 74, 75, 76, 77, 79, 80, 81, 82, 84, 85, 86, 87,
        88, 90, 91, 92, 93, 94, 95, 96, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
        110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
        128, 129, 130, 131, 132, 133, 134, 135, 135, 136, 137, 138, 139, 140, 141, 142, 143, 143,
        144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158, 159,
        160, 160, 161, 162, 163, 164, 165, 165, 166, 167, 168, 169, 169, 170, 171, 172, 173, 173,
        174, 175, 176, 176, 177, 178, 179, 180, 180, 181, 182, 183, 183, 184, 185, 186, 186, 187,
        188, 189, 189, 190, 191, 192, 192, 193, 194, 195, 195, 196, 197, 197, 198, 199, 200, 200,
        201, 202, 202, 203, 204, 205, 205, 206, 207, 207, 208, 209, 210, 210, 211, 212, 212, 213,
        214, 214, 215, 216, 216, 217, 218, 219, 219, 220, 221, 221, 222, 223, 223, 224, 225, 225,
        226, 227, 227, 228, 229, 229, 230, 231, 231, 232, 233, 233, 234, 235, 235, 236, 237, 237,
        238, 238, 239, 240, 240, 241, 242, 242, 243, 244, 244, 245, 246, 246, 247, 247, 248, 249,
        249, 250, 251, 251, 252, 253, 253, 254, 254, 255,
    ],
    [
        16, 23, 28, 32, 36, 39, 42, 45, 48, 50, 53, 55, 57, 60, 62, 64, 66, 68, 69, 71, 73, 75, 76,
        78, 80, 81, 83, 84, 86, 87, 89, 90, 92, 93, 94, 96, 97, 98, 100, 101, 102, 103, 105, 106,
        107, 108, 109, 110, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
        126, 128, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
        143, 144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158,
        159, 159, 160, 161, 162, 163, 163, 164, 165, 166, 166, 167, 168, 169, 169, 170, 171, 172,
        172, 173, 174, 175, 175, 176, 177, 177, 178, 179, 180, 180, 181, 182, 182, 183, 184, 184,
        185, 186, 187, 187, 188, 189, 189, 190, 191, 191, 192, 193, 193, 194, 195, 195, 196, 196,
        197, 198, 198, 199, 200, 200, 201, 202, 202, 203, 203, 204, 205, 205, 206, 207, 207, 208,
        208, 209, 210, 210, 211, 211, 212, 213, 213, 214, 214, 215, 216, 216, 217, 217, 218, 219,
        219, 220, 220, 221, 221, 222, 223, 223, 224, 224, 225, 225, 226, 227, 227, 228, 228, 229,
        229, 230, 230, 231, 232, 232, 233, 233, 234, 234, 235, 235, 236, 236, 237, 237, 238, 239,
        239, 240, 240, 241, 241, 242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 248,
        248, 249, 249, 250, 250, 251, 251, 252, 252, 253, 254, 254, 255, 255,
    ],
];

pub unsafe fn V_MarkRect(x: i32, y: i32, width: i32, height: i32) {
    M_AddToBox(dirtybox.as_mut_ptr(), x, y);
    M_AddToBox(dirtybox.as_mut_ptr(), x + width - 1, y + height - 1);
}

pub unsafe fn V_CopyRect(
    srcx: i32,
    srcy: i32,
    srcscrn: i32,
    width: i32,
    height: i32,
    destx: i32,
    desty: i32,
    destscrn: i32,
) {
    if srcx < 0
        || srcx + width > SCREENWIDTH
        || srcy < 0
        || srcy + height > SCREENHEIGHT
        || destx < 0
        || destx + width > SCREENWIDTH
        || desty < 0
        || desty + height > SCREENHEIGHT
        || (srcscrn as u32) > 4
        || (destscrn as u32) > 4
    {
        I_Error("Bad V_CopyRect");
    }

    V_MarkRect(destx, desty, width, height);

    let mut src = screens[srcscrn as usize].add((SCREENWIDTH * srcy + srcx) as usize);
    let mut dest = screens[destscrn as usize].add((SCREENWIDTH * desty + destx) as usize);

    for _ in 0..height {
        std::ptr::copy_nonoverlapping(src, dest, width as usize);
        src = src.add(SCREENWIDTH as usize);
        dest = dest.add(SCREENWIDTH as usize);
    }
}

pub unsafe fn V_DrawPatch(mut x: i32, mut y: i32, scrn: i32, patch: *mut patch_t) {
    y -= SHORT((*patch).topoffset) as i32;
    x -= SHORT((*patch).leftoffset) as i32;

    if x < 0
        || x + (SHORT((*patch).width) as i32) > SCREENWIDTH
        || y < 0
        || y + (SHORT((*patch).height) as i32) > SCREENHEIGHT
        || (scrn as u32) > 4
    {
        eprintln!("Patch at {},{} exceeds LFB", x, y);
        eprintln!("V_DrawPatch: bad patch (ignored)");
        return;
    }

    if scrn == 0 {
        V_MarkRect(
            x,
            y,
            SHORT((*patch).width) as i32,
            SHORT((*patch).height) as i32,
        );
    }

    let mut col = 0i32;
    let mut desttop = screens[scrn as usize].add((y * SCREENWIDTH + x) as usize);
    let w = SHORT((*patch).width) as i32;
    let columnofs_ptr = (*patch).columnofs.as_ptr();

    while col < w {
        let mut column =
            (patch as *const byte).add(LONG(*columnofs_ptr.add(col as usize)) as usize)
                as *const column_t;

        while (*column).topdelta != 0xff {
            let mut source = (column as *const byte).add(3);
            let mut dest = desttop.add((((*column).topdelta as i32) * SCREENWIDTH) as usize);
            let mut count = (*column).length as i32;

            while count > 0 {
                count -= 1;
                *dest = *source;
                source = source.add(1);
                dest = dest.add(SCREENWIDTH as usize);
            }
            column =
                (column as *const byte).add(((*column).length as usize) + 4) as *const column_t;
        }

        x += 1;
        col += 1;
        desttop = desttop.add(1);
    }
}

pub unsafe fn V_DrawPatchFlipped(mut x: i32, mut y: i32, scrn: i32, patch: *mut patch_t) {
    y -= SHORT((*patch).topoffset) as i32;
    x -= SHORT((*patch).leftoffset) as i32;

    if x < 0
        || x + (SHORT((*patch).width) as i32) > SCREENWIDTH
        || y < 0
        || y + (SHORT((*patch).height) as i32) > SCREENHEIGHT
        || (scrn as u32) > 4
    {
        eprintln!("Patch origin {},{} exceeds LFB", x, y);
        I_Error("Bad V_DrawPatch in V_DrawPatchFlipped");
    }

    if scrn == 0 {
        V_MarkRect(
            x,
            y,
            SHORT((*patch).width) as i32,
            SHORT((*patch).height) as i32,
        );
    }

    let mut col = 0i32;
    let mut desttop = screens[scrn as usize].add((y * SCREENWIDTH + x) as usize);
    let w = SHORT((*patch).width) as i32;
    let columnofs_ptr = (*patch).columnofs.as_ptr();

    while col < w {
        let mut column = (patch as *const byte)
            .add(LONG(*columnofs_ptr.add((w - 1 - col) as usize)) as usize)
            as *const column_t;

        while (*column).topdelta != 0xff {
            let mut source = (column as *const byte).add(3);
            let mut dest = desttop.add((((*column).topdelta as i32) * SCREENWIDTH) as usize);
            let mut count = (*column).length as i32;

            while count > 0 {
                count -= 1;
                *dest = *source;
                source = source.add(1);
                dest = dest.add(SCREENWIDTH as usize);
            }
            column =
                (column as *const byte).add(((*column).length as usize) + 4) as *const column_t;
        }

        x += 1;
        col += 1;
        desttop = desttop.add(1);
    }
}

pub unsafe fn V_DrawPatchDirect(x: i32, y: i32, scrn: i32, patch: *mut patch_t) {
    V_DrawPatch(x, y, scrn, patch);
}

pub unsafe fn V_DrawBlock(
    x: i32,
    y: i32,
    scrn: i32,
    width: i32,
    mut height: i32,
    mut src: *mut byte,
) {
    if x < 0
        || x + width > SCREENWIDTH
        || y < 0
        || y + height > SCREENHEIGHT
        || (scrn as u32) > 4
    {
        I_Error("Bad V_DrawBlock");
    }

    V_MarkRect(x, y, width, height);

    let mut dest = screens[scrn as usize].add((y * SCREENWIDTH + x) as usize);

    while height > 0 {
        height -= 1;
        std::ptr::copy_nonoverlapping(src, dest, width as usize);
        src = src.add(width as usize);
        dest = dest.add(SCREENWIDTH as usize);
    }
}

pub unsafe fn V_GetBlock(
    x: i32,
    y: i32,
    scrn: i32,
    width: i32,
    mut height: i32,
    mut dest: *mut byte,
) {
    if x < 0
        || x + width > SCREENWIDTH
        || y < 0
        || y + height > SCREENHEIGHT
        || (scrn as u32) > 4
    {
        I_Error("Bad V_DrawBlock");
    }

    let mut src = screens[scrn as usize].add((y * SCREENWIDTH + x) as usize);

    while height > 0 {
        height -= 1;
        std::ptr::copy_nonoverlapping(src, dest, width as usize);
        src = src.add(SCREENWIDTH as usize);
        dest = dest.add(width as usize);
    }
}

pub unsafe fn V_Init() {
    let base = I_AllocLow(SCREENWIDTH * SCREENHEIGHT * 4);
    for i in 0..4 {
        screens[i] = base.add((i as i32 * SCREENWIDTH * SCREENHEIGHT) as usize);
    }
}
