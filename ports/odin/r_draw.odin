// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

MAXWIDTH   :: 1120
MAXHEIGHT  :: 832
SBARHEIGHT :: 32

viewimage: [^]byte
viewwidth: int
scaledviewwidth: int
viewheight: int
viewwindowx: int
viewwindowy: int
ylookup: [MAXHEIGHT][^]byte
columnofs: [MAXWIDTH]int

translations: [3][256]byte

dc_colormap: [^]lighttable_t
dc_x: int
dc_yl: int
dc_yh: int
dc_iscale: fixed_t
dc_texturemid: fixed_t
dc_source: [^]byte
dccount: int

FUZZTABLE :: 50
FUZZOFF   :: SCREENWIDTH

fuzzoffset := [FUZZTABLE]int{
	FUZZOFF,-FUZZOFF,FUZZOFF,-FUZZOFF,FUZZOFF,FUZZOFF,-FUZZOFF,
	FUZZOFF,FUZZOFF,-FUZZOFF,FUZZOFF,FUZZOFF,FUZZOFF,-FUZZOFF,
	FUZZOFF,FUZZOFF,FUZZOFF,-FUZZOFF,-FUZZOFF,-FUZZOFF,-FUZZOFF,
	FUZZOFF,-FUZZOFF,-FUZZOFF,FUZZOFF,FUZZOFF,FUZZOFF,FUZZOFF,-FUZZOFF,
	FUZZOFF,-FUZZOFF,FUZZOFF,FUZZOFF,-FUZZOFF,-FUZZOFF,FUZZOFF,
	FUZZOFF,-FUZZOFF,-FUZZOFF,-FUZZOFF,-FUZZOFF,FUZZOFF,FUZZOFF,
	FUZZOFF,FUZZOFF,-FUZZOFF,FUZZOFF,FUZZOFF,-FUZZOFF,FUZZOFF,
}

fuzzpos: int = 0

dc_translation: [^]byte
translationtables: [^]byte

ds_y: int
ds_x1: int
ds_x2: int
ds_colormap: [^]lighttable_t
ds_xfrac: fixed_t
ds_yfrac: fixed_t
ds_xstep: fixed_t
ds_ystep: fixed_t
ds_source: [^]byte
dscount: int

R_DrawColumn :: proc() {
	count := dc_yh - dc_yl
	if count < 0 {
		return
	}

	if uint(dc_x) >= SCREENWIDTH || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
		I_Error("R_DrawColumn: %i to %i at %i", i32(dc_yl), i32(dc_yh), i32(dc_x))
	}

	dest: [^]byte = &ylookup[dc_yl][columnofs[dc_x]]

	fracstep := dc_iscale
	frac := dc_texturemid + fixed_t(dc_yl - centery) * fracstep

	for {
		dest[0] = dc_colormap[dc_source[(frac >> FRACBITS) & 127]]
		dest = &dest[SCREENWIDTH]
		frac += fracstep
		if count == 0 {
			break
		}
		count -= 1
	}
}

R_DrawColumnLow :: proc() {
	count := dc_yh - dc_yl
	if count < 0 {
		return
	}

	if uint(dc_x) >= SCREENWIDTH || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
		I_Error("R_DrawColumn: %i to %i at %i", i32(dc_yl), i32(dc_yh), i32(dc_x))
	}

	dc_x <<= 1

	dest: [^]byte = &ylookup[dc_yl][columnofs[dc_x]]
	dest2: [^]byte = &ylookup[dc_yl][columnofs[dc_x + 1]]

	fracstep := dc_iscale
	frac := dc_texturemid + fixed_t(dc_yl - centery) * fracstep

	for {
		val := dc_colormap[dc_source[(frac >> FRACBITS) & 127]]
		dest[0] = val
		dest2[0] = val
		dest = &dest[SCREENWIDTH]
		dest2 = &dest2[SCREENWIDTH]
		frac += fracstep
		if count == 0 {
			break
		}
		count -= 1
	}
}

R_DrawFuzzColumn :: proc() {
	if dc_yl == 0 {
		dc_yl = 1
	}
	if dc_yh == viewheight - 1 {
		dc_yh = viewheight - 2
	}

	count := dc_yh - dc_yl
	if count < 0 {
		return
	}

	if uint(dc_x) >= SCREENWIDTH || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
		I_Error("R_DrawFuzzColumn: %i to %i at %i", i32(dc_yl), i32(dc_yh), i32(dc_x))
	}

	dest: [^]byte = &ylookup[dc_yl][columnofs[dc_x]]

	fracstep := dc_iscale
	frac := dc_texturemid + fixed_t(dc_yl - centery) * fracstep

	for {
		fo := fuzzoffset[fuzzpos]
		sample := (^byte)(int(uintptr(dest)) + fo)^
		dest[0] = colormaps[6 * 256 + int(sample)]

		fuzzpos += 1
		if fuzzpos == FUZZTABLE {
			fuzzpos = 0
		}

		dest = &dest[SCREENWIDTH]
		frac += fracstep
		if count == 0 {
			break
		}
		count -= 1
	}
}

R_DrawTranslatedColumn :: proc() {
	count := dc_yh - dc_yl
	if count < 0 {
		return
	}

	if uint(dc_x) >= SCREENWIDTH || dc_yl < 0 || dc_yh >= SCREENHEIGHT {
		I_Error("R_DrawColumn: %i to %i at %i", i32(dc_yl), i32(dc_yh), i32(dc_x))
	}

	dest: [^]byte = &ylookup[dc_yl][columnofs[dc_x]]

	fracstep := dc_iscale
	frac := dc_texturemid + fixed_t(dc_yl - centery) * fracstep

	for {
		dest[0] = dc_colormap[dc_translation[dc_source[frac >> FRACBITS]]]
		dest = &dest[SCREENWIDTH]
		frac += fracstep
		if count == 0 {
			break
		}
		count -= 1
	}
}

R_InitTranslationTables :: proc() {
	raw := Z_Malloc(256 * 3 + 255, PU_STATIC, nil)
	translationtables = ([^]byte)((uintptr(raw) + 255) & ~uintptr(255))

	for i := 0; i < 256; i += 1 {
		if i >= 0x70 && i <= 0x7f {
			translationtables[i] = byte(0x60 + (i & 0xf))
			translationtables[i + 256] = byte(0x40 + (i & 0xf))
			translationtables[i + 512] = byte(0x20 + (i & 0xf))
		} else {
			translationtables[i] = byte(i)
			translationtables[i + 256] = byte(i)
			translationtables[i + 512] = byte(i)
		}
	}
}

R_DrawSpan :: proc() {
	if ds_x2 < ds_x1 || ds_x1 < 0 || ds_x2 >= SCREENWIDTH || uint(ds_y) > SCREENHEIGHT {
		I_Error("R_DrawSpan: %i to %i at %i", i32(ds_x1), i32(ds_x2), i32(ds_y))
	}

	xfrac := ds_xfrac
	yfrac := ds_yfrac

	dest: [^]byte = &ylookup[ds_y][columnofs[ds_x1]]

	count := ds_x2 - ds_x1

	for {
		spot := int((u32(yfrac) >> (16 - 6)) & (63 * 64)) + int((u32(xfrac) >> 16) & 63)
		dest[0] = ds_colormap[ds_source[spot]]
		dest = &dest[1]

		xfrac += ds_xstep
		yfrac += ds_ystep
		if count == 0 {
			break
		}
		count -= 1
	}
}

R_DrawSpanLow :: proc() {
	if ds_x2 < ds_x1 || ds_x1 < 0 || ds_x2 >= SCREENWIDTH || uint(ds_y) > SCREENHEIGHT {
		I_Error("R_DrawSpan: %i to %i at %i", i32(ds_x1), i32(ds_x2), i32(ds_y))
	}

	xfrac := ds_xfrac
	yfrac := ds_yfrac

	ds_x1 <<= 1
	ds_x2 <<= 1

	dest: [^]byte = &ylookup[ds_y][columnofs[ds_x1]]

	count := ds_x2 - ds_x1
	for {
		spot := int((u32(yfrac) >> (16 - 6)) & (63 * 64)) + int((u32(xfrac) >> 16) & 63)
		val := ds_colormap[ds_source[spot]]
		dest[0] = val
		dest[1] = val
		dest = &dest[2]

		xfrac += ds_xstep
		yfrac += ds_ystep
		if count == 0 {
			break
		}
		count -= 1
	}
}

R_InitBuffer :: proc(width: int, height: int) {
	viewwindowx = (SCREENWIDTH - width) >> 1

	for i := 0; i < width; i += 1 {
		columnofs[i] = viewwindowx + i
	}

	if width == SCREENWIDTH {
		viewwindowy = 0
	} else {
		viewwindowy = (SCREENHEIGHT - SBARHEIGHT - height) >> 1
	}

	for i := 0; i < height; i += 1 {
		ylookup[i] = &screens[0][(i + viewwindowy) * SCREENWIDTH]
	}
}

R_FillBackScreen :: proc() {
	if scaledviewwidth == 320 {
		return
	}

	name: cstring
	if gamemode == .commercial {
		name = "GRNROCK"
	} else {
		name = "FLOOR7_2"
	}

	src := ([^]byte)(W_CacheLumpName(name, PU_CACHE))
	dest := screens[1]

	for y := 0; y < SCREENHEIGHT - SBARHEIGHT; y += 1 {
		for x := 0; x < SCREENWIDTH / 64; x += 1 {
			libc.memcpy(dest, &src[(y & 63) << 6], 64)
			dest = &dest[64]
		}
		if (SCREENWIDTH & 63) != 0 {
			libc.memcpy(dest, &src[(y & 63) << 6], SCREENWIDTH & 63)
			dest = &dest[SCREENWIDTH & 63]
		}
	}

	patch := (^patch_t)(W_CacheLumpName("brdr_t", PU_CACHE))
	for x := 0; x < scaledviewwidth; x += 8 {
		V_DrawPatch(viewwindowx + x, viewwindowy - 8, 1, patch)
	}

	patch = (^patch_t)(W_CacheLumpName("brdr_b", PU_CACHE))
	for x := 0; x < scaledviewwidth; x += 8 {
		V_DrawPatch(viewwindowx + x, viewwindowy + viewheight, 1, patch)
	}

	patch = (^patch_t)(W_CacheLumpName("brdr_l", PU_CACHE))
	for y := 0; y < viewheight; y += 8 {
		V_DrawPatch(viewwindowx - 8, viewwindowy + y, 1, patch)
	}

	patch = (^patch_t)(W_CacheLumpName("brdr_r", PU_CACHE))
	for y := 0; y < viewheight; y += 8 {
		V_DrawPatch(viewwindowx + scaledviewwidth, viewwindowy + y, 1, patch)
	}

	V_DrawPatch(viewwindowx - 8, viewwindowy - 8, 1, (^patch_t)(W_CacheLumpName("brdr_tl", PU_CACHE)))
	V_DrawPatch(viewwindowx + scaledviewwidth, viewwindowy - 8, 1, (^patch_t)(W_CacheLumpName("brdr_tr", PU_CACHE)))
	V_DrawPatch(viewwindowx - 8, viewwindowy + viewheight, 1, (^patch_t)(W_CacheLumpName("brdr_bl", PU_CACHE)))
	V_DrawPatch(viewwindowx + scaledviewwidth, viewwindowy + viewheight, 1, (^patch_t)(W_CacheLumpName("brdr_br", PU_CACHE)))
}

R_VideoErase :: proc(ofs: uint, count: int) {
	libc.memcpy(&screens[0][ofs], &screens[1][ofs], uint(count))
}

R_DrawViewBorder :: proc() {
	if scaledviewwidth == SCREENWIDTH {
		return
	}

	top := ((SCREENHEIGHT - SBARHEIGHT) - viewheight) / 2
	side := (SCREENWIDTH - scaledviewwidth) / 2

	R_VideoErase(0, top * SCREENWIDTH + side)

	ofs := (viewheight + top) * SCREENWIDTH - side
	R_VideoErase(uint(ofs), top * SCREENWIDTH + side)

	ofs = top * SCREENWIDTH + SCREENWIDTH - side
	side <<= 1

	for i := 1; i < viewheight; i += 1 {
		R_VideoErase(uint(ofs), side)
		ofs += SCREENWIDTH
	}

	V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT - SBARHEIGHT)
}