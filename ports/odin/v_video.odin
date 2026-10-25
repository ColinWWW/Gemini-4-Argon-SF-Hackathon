// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

screens: [5][^]byte
dirtybox: [4]fixed_t

gammatable := [5][256]byte{
	{1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,
	 17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,
	 33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,
	 49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,
	 65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,
	 81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,
	 97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,
	 113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,
	 128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,
	 144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,
	 160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,
	 176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,
	 192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,
	 208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,
	 224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,
	 240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255},

	{2,4,5,7,8,10,11,12,14,15,16,18,19,20,21,23,24,25,26,27,29,30,31,
	 32,33,34,36,37,38,39,40,41,42,44,45,46,47,48,49,50,51,52,54,55,
	 56,57,58,59,60,61,62,63,64,65,66,67,69,70,71,72,73,74,75,76,77,
	 78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,
	 99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,
	 115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,129,
	 130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,
	 146,147,148,148,149,150,151,152,153,154,155,156,157,158,159,160,
	 161,162,163,163,164,165,166,167,168,169,170,171,172,173,174,175,
	 175,176,177,178,179,180,181,182,183,184,185,186,186,187,188,189,
	 190,191,192,193,194,195,196,196,197,198,199,200,201,202,203,204,
	 205,205,206,207,208,209,210,211,212,213,214,214,215,216,217,218,
	 219,220,221,222,222,223,224,225,226,227,228,229,230,230,231,232,
	 233,234,235,236,237,237,238,239,240,241,242,243,244,245,245,246,
	 247,248,249,250,251,252,252,253,254,255},

	{4,7,9,11,13,15,17,19,21,22,24,26,27,29,30,32,33,35,36,38,39,40,42,
	 43,45,46,47,48,50,51,52,54,55,56,57,59,60,61,62,63,65,66,67,68,69,
	 70,72,73,74,75,76,77,78,79,80,82,83,84,85,86,87,88,89,90,91,92,93,
	 94,95,96,97,98,100,101,102,103,104,105,106,107,108,109,110,111,112,
	 113,114,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,
	 129,130,131,132,133,133,134,135,136,137,138,139,140,141,142,143,144,
	 144,145,146,147,148,149,150,151,152,153,153,154,155,156,157,158,159,
	 160,160,161,162,163,164,165,166,166,167,168,169,170,171,172,172,173,
	 174,175,176,177,178,178,179,180,181,182,183,183,184,185,186,187,188,
	 188,189,190,191,192,193,193,194,195,196,197,197,198,199,200,201,201,
	 202,203,204,205,206,206,207,208,209,210,210,211,212,213,213,214,215,
	 216,217,217,218,219,220,221,221,222,223,224,224,225,226,227,228,228,
	 229,230,231,231,232,233,234,235,235,236,237,238,238,239,240,241,241,
	 242,243,244,244,245,246,247,247,248,249,250,251,251,252,253,254,254,
	 255},

	{8,12,16,19,22,24,27,29,31,34,36,38,40,41,43,45,47,49,50,52,53,55,
	 57,58,60,61,63,64,65,67,68,70,71,72,74,75,76,77,79,80,81,82,84,85,
	 86,87,88,90,91,92,93,94,95,96,98,99,100,101,102,103,104,105,106,107,
	 108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,
	 125,126,127,128,129,130,131,132,133,134,135,135,136,137,138,139,140,
	 141,142,143,143,144,145,146,147,148,149,150,150,151,152,153,154,155,
	 155,156,157,158,159,160,160,161,162,163,164,165,165,166,167,168,169,
	 169,170,171,172,173,173,174,175,176,176,177,178,179,180,180,181,182,
	 183,183,184,185,186,186,187,188,189,189,190,191,192,192,193,194,195,
	 195,196,197,197,198,199,200,200,201,202,202,203,204,205,205,206,207,
	 207,208,209,210,210,211,212,212,213,214,214,215,216,216,217,218,219,
	 219,220,221,221,222,223,223,224,225,225,226,227,227,228,229,229,230,
	 231,231,232,233,233,234,235,235,236,237,237,238,238,239,240,240,241,
	 242,242,243,244,244,245,246,246,247,247,248,249,249,250,251,251,252,
	 253,253,254,254,255},

	{16,23,28,32,36,39,42,45,48,50,53,55,57,60,62,64,66,68,69,71,73,75,76,
	 78,80,81,83,84,86,87,89,90,92,93,94,96,97,98,100,101,102,103,105,106,
	 107,108,109,110,112,113,114,115,116,117,118,119,120,121,122,123,124,
	 125,126,128,128,129,130,131,132,133,134,135,136,137,138,139,140,141,
	 142,143,143,144,145,146,147,148,149,150,150,151,152,153,154,155,155,
	 156,157,158,159,159,160,161,162,163,163,164,165,166,166,167,168,169,
	 169,170,171,172,172,173,174,175,175,176,177,177,178,179,180,180,181,
	 182,182,183,184,184,185,186,187,187,188,189,189,190,191,191,192,193,
	 193,194,195,195,196,196,197,198,198,199,200,200,201,202,202,203,203,
	 204,205,205,206,207,207,208,208,209,210,210,211,211,212,213,213,214,
	 214,215,216,216,217,217,218,219,219,220,220,221,221,222,223,223,224,
	 224,225,225,226,227,227,228,228,229,229,230,230,231,232,232,233,233,
	 234,234,235,235,236,236,237,237,238,239,239,240,240,241,241,242,242,
	 243,243,244,244,245,245,246,246,247,247,248,248,249,249,250,250,251,
	 251,252,252,253,254,254,255,255},
}

usegamma: int

V_MarkRect :: proc(x: int, y: int, width: int, height: int) {
	M_AddToBox(&dirtybox[0], fixed_t(x), fixed_t(y))
	M_AddToBox(&dirtybox[0], fixed_t(x + width - 1), fixed_t(y + height - 1))
}

V_CopyRect :: proc(srcx: int, srcy: int, srcscrn: int, width: int, height_in: int, destx: int, desty: int, destscrn: int) {
	height := height_in
	if srcx < 0 || srcx + width > SCREENWIDTH || srcy < 0 || srcy + height > SCREENHEIGHT ||
	   destx < 0 || destx + width > SCREENWIDTH || desty < 0 || desty + height > SCREENHEIGHT ||
	   uint(srcscrn) > 4 || uint(destscrn) > 4 {
		I_Error("Bad V_CopyRect")
	}

	V_MarkRect(destx, desty, width, height)

	src := &screens[srcscrn][SCREENWIDTH * srcy + srcx]
	dest := &screens[destscrn][SCREENWIDTH * desty + destx]

	for ; height > 0; height -= 1 {
		libc.memcpy(dest, src, uint(width))
		src = &src[SCREENWIDTH]
		dest = &dest[SCREENWIDTH]
	}
}

V_DrawPatch :: proc(x_in: int, y_in: int, scrn: int, patch: ^patch_t) {
	y := y_in - int(SHORT(patch.topoffset))
	x := x_in - int(SHORT(patch.leftoffset))

	if x < 0 || x + int(SHORT(patch.width)) > SCREENWIDTH || y < 0 || y + int(SHORT(patch.height)) > SCREENHEIGHT || uint(scrn) > 4 {
		libc.fprintf(libc.stderr, "Patch at %d,%d exceeds LFB\n", i32(x), i32(y))
		libc.fprintf(libc.stderr, "V_DrawPatch: bad patch (ignored)\n")
		return
	}

	if scrn == 0 {
		V_MarkRect(x, y, int(SHORT(patch.width)), int(SHORT(patch.height)))
	}

	col := 0
	desttop := &screens[scrn][y * SCREENWIDTH + x]

	w := int(SHORT(patch.width))
	colofs := ([^]i32)(&patch.columnofs[0])

	for ; col < w; col += 1 {
		column := (^column_t)(uintptr(patch) + uintptr(LONG(colofs[col])))

		for column.topdelta != 0xff {
			source := ([^]byte)(uintptr(column) + 3)
			dest := ([^]byte)(uintptr(desttop) + uintptr(column.topdelta) * SCREENWIDTH)
			count := int(column.length)

			for count > 0 {
				count -= 1
				dest[0] = source[0]
				source = &source[1]
				dest = &dest[SCREENWIDTH]
			}
			column = (^column_t)(uintptr(column) + uintptr(column.length) + 4)
		}
		x += 1
		desttop = &desttop[1]
	}
}

V_DrawPatchFlipped :: proc(x_in: int, y_in: int, scrn: int, patch: ^patch_t) {
	y := y_in - int(SHORT(patch.topoffset))
	x := x_in - int(SHORT(patch.leftoffset))

	if x < 0 || x + int(SHORT(patch.width)) > SCREENWIDTH || y < 0 || y + int(SHORT(patch.height)) > SCREENHEIGHT || uint(scrn) > 4 {
		libc.fprintf(libc.stderr, "Patch origin %d,%d exceeds LFB\n", i32(x), i32(y))
		I_Error("Bad V_DrawPatch in V_DrawPatchFlipped")
	}

	if scrn == 0 {
		V_MarkRect(x, y, int(SHORT(patch.width)), int(SHORT(patch.height)))
	}

	col := 0
	desttop := &screens[scrn][y * SCREENWIDTH + x]

	w := int(SHORT(patch.width))
	colofs := ([^]i32)(&patch.columnofs[0])

	for ; col < w; col += 1 {
		column := (^column_t)(uintptr(patch) + uintptr(LONG(colofs[w - 1 - col])))

		for column.topdelta != 0xff {
			source := ([^]byte)(uintptr(column) + 3)
			dest := ([^]byte)(uintptr(desttop) + uintptr(column.topdelta) * SCREENWIDTH)
			count := int(column.length)

			for count > 0 {
				count -= 1
				dest[0] = source[0]
				source = &source[1]
				dest = &dest[SCREENWIDTH]
			}
			column = (^column_t)(uintptr(column) + uintptr(column.length) + 4)
		}
		x += 1
		desttop = &desttop[1]
	}
}

V_DrawPatchDirect :: proc(x: int, y: int, scrn: int, patch: ^patch_t) {
	V_DrawPatch(x, y, scrn, patch)
}

V_DrawBlock :: proc(x: int, y: int, scrn: int, width: int, height_in: int, src_in: [^]byte) {
	height := height_in
	src := src_in

	if x < 0 || x + width > SCREENWIDTH || y < 0 || y + height > SCREENHEIGHT || uint(scrn) > 4 {
		I_Error("Bad V_DrawBlock")
	}

	V_MarkRect(x, y, width, height)

	dest := &screens[scrn][y * SCREENWIDTH + x]

	for height > 0 {
		height -= 1
		libc.memcpy(dest, src, uint(width))
		src = &src[width]
		dest = &dest[SCREENWIDTH]
	}
}

V_GetBlock :: proc(x: int, y: int, scrn: int, width: int, height_in: int, dest_in: [^]byte) {
	height := height_in
	dest := dest_in

	if x < 0 || x + width > SCREENWIDTH || y < 0 || y + height > SCREENHEIGHT || uint(scrn) > 4 {
		I_Error("Bad V_DrawBlock")
	}

	src := &screens[scrn][y * SCREENWIDTH + x]

	for height > 0 {
		height -= 1
		libc.memcpy(dest, src, uint(width))
		src = &src[SCREENWIDTH]
		dest = &dest[width]
	}
}

V_Init :: proc() {
	base := I_AllocLow(SCREENWIDTH * SCREENHEIGHT * 4)
	for i := 0; i < 4; i += 1 {
		screens[i] = &base[i * SCREENWIDTH * SCREENHEIGHT]
	}
}
