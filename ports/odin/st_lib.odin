// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

sttminus: ^patch_t

STlib_init :: proc() {
	sttminus = (^patch_t)(W_CacheLumpName("STTMINUS", PU_STATIC))
}

STlib_initNum :: proc(n: ^st_number_t, x: int, y: int, pl: [^]^patch_t, num: ^int, on: ^boolean, width: int) {
	n.x = x
	n.y = y
	n.oldnum = 0
	n.width = width
	n.num = num
	n.on = on
	n.p = pl
}

STlib_drawNum :: proc(n: ^st_number_t, refresh: boolean) {
	numdigits := n.width
	num := n.num^

	w := int(SHORT(n.p[0].width))
	h := int(SHORT(n.p[0].height))

	n.oldnum = n.num^

	neg := num < 0

	if neg {
		if numdigits == 2 && num < -9 {
			num = -9
		} else if numdigits == 3 && num < -99 {
			num = -99
		}
		num = -num
	}

	x := n.x - numdigits * w

	if n.y - ST_Y < 0 {
		I_Error("drawNum: n->y - ST_Y < 0")
	}

	V_CopyRect(x, n.y - ST_Y, ST_BG, w * numdigits, h, x, n.y, ST_FG)

	if num == 1994 {
		return
	}

	x = n.x

	if num == 0 {
		V_DrawPatch(x - w, n.y, ST_FG, n.p[0])
	}

	for num != 0 && numdigits != 0 {
		numdigits -= 1
		x -= w
		V_DrawPatch(x, n.y, ST_FG, n.p[num % 10])
		num /= 10
	}

	if neg {
		V_DrawPatch(x - 8, n.y, ST_FG, sttminus)
	}
}

STlib_updateNum :: proc(n: ^st_number_t, refresh: boolean) {
	if n.on^ {
		STlib_drawNum(n, refresh)
	}
}

STlib_initPercent :: proc(p: ^st_percent_t, x: int, y: int, pl: [^]^patch_t, num: ^int, on: ^boolean, percent: ^patch_t) {
	STlib_initNum(&p.n, x, y, pl, num, on, 3)
	p.p = percent
}

STlib_updatePercent :: proc(per: ^st_percent_t, refresh: int) {
	if refresh != 0 && per.n.on^ {
		V_DrawPatch(per.n.x, per.n.y, ST_FG, per.p)
	}
	STlib_updateNum(&per.n, refresh != 0)
}

STlib_initMultIcon :: proc(i: ^st_multicon_t, x: int, y: int, il: [^]^patch_t, inum: ^int, on: ^boolean) {
	i.x = x
	i.y = y
	i.oldinum = -1
	i.inum = inum
	i.on = on
	i.p = il
}

STlib_updateMultIcon :: proc(mi: ^st_multicon_t, refresh: boolean) {
	if mi.on^ && (mi.oldinum != mi.inum^ || refresh) && mi.inum^ != -1 {
		if mi.oldinum != -1 {
			x := mi.x - int(SHORT(mi.p[mi.oldinum].leftoffset))
			y := mi.y - int(SHORT(mi.p[mi.oldinum].topoffset))
			w := int(SHORT(mi.p[mi.oldinum].width))
			h := int(SHORT(mi.p[mi.oldinum].height))

			if y - ST_Y < 0 {
				I_Error("updateMultIcon: y - ST_Y < 0")
			}

			V_CopyRect(x, y - ST_Y, ST_BG, w, h, x, y, ST_FG)
		}
		V_DrawPatch(mi.x, mi.y, ST_FG, mi.p[mi.inum^])
		mi.oldinum = mi.inum^
	}
}

STlib_initBinIcon :: proc(b: ^st_binicon_t, x: int, y: int, i: ^patch_t, val: ^boolean, on: ^boolean) {
	b.x = x
	b.y = y
	b.oldval = 0
	b.val = val
	b.on = on
	b.p = i
}

STlib_updateBinIcon :: proc(bi: ^st_binicon_t, refresh: boolean) {
	val_int := bi.val^ ? 1 : 0
	if bi.on^ && (bi.oldval != val_int || refresh) {
		x := bi.x - int(SHORT(bi.p.leftoffset))
		y := bi.y - int(SHORT(bi.p.topoffset))
		w := int(SHORT(bi.p.width))
		h := int(SHORT(bi.p.height))

		if y - ST_Y < 0 {
			I_Error("updateBinIcon: y - ST_Y < 0")
		}

		if bi.val^ {
			V_DrawPatch(bi.x, bi.y, ST_FG, bi.p)
		} else {
			V_CopyRect(x, y - ST_Y, ST_BG, w, h, x, y, ST_FG)
		}

		bi.oldval = val_int
	}
}
