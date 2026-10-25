// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

planefunction_t :: #type proc(top: int, bottom: int)

floorfunc: planefunction_t
ceilingfunc: planefunction_t

MAXVISPLANES :: 128
visplanes: [MAXVISPLANES]visplane_t
lastvisplane: [^]visplane_t
floorplane: ^visplane_t
ceilingplane: ^visplane_t

MAXOPENINGS :: SCREENWIDTH * 64
openings: [MAXOPENINGS]i16
lastopening: [^]i16

floorclip: [SCREENWIDTH]i16
ceilingclip: [SCREENWIDTH]i16

spanstart: [SCREENHEIGHT]int
spanstop: [SCREENHEIGHT]int

planezlight: [^][^]lighttable_t
planeheight: fixed_t

yslope: [SCREENHEIGHT]fixed_t
distscale: [SCREENWIDTH]fixed_t
basexscale: fixed_t
baseyscale: fixed_t

cachedheight: [SCREENHEIGHT]fixed_t
cacheddistance: [SCREENHEIGHT]fixed_t
cachedxstep: [SCREENHEIGHT]fixed_t
cachedystep: [SCREENHEIGHT]fixed_t

R_InitPlanes :: proc() {
}

R_MapPlane :: proc(y: int, x1: int, x2: int) {
	if x2 < x1 || x1 < 0 || x2 >= viewwidth || uint(y) > uint(viewheight) {
		I_Error("R_MapPlane: %i, %i at %i", i32(x1), i32(x2), i32(y))
	}

	distance: fixed_t
	if planeheight != cachedheight[y] {
		cachedheight[y] = planeheight
		distance = FixedMul(planeheight, yslope[y])
		cacheddistance[y] = distance
		ds_xstep = FixedMul(distance, basexscale)
		cachedxstep[y] = ds_xstep
		ds_ystep = FixedMul(distance, baseyscale)
		cachedystep[y] = ds_ystep
	} else {
		distance = cacheddistance[y]
		ds_xstep = cachedxstep[y]
		ds_ystep = cachedystep[y]
	}

	length := FixedMul(distance, distscale[x1])
	angle := (viewangle + xtoviewangle[x1]) >> ANGLETOFINESHIFT
	ds_xfrac = viewx + FixedMul(finecosine[angle], length)
	ds_yfrac = -viewy - FixedMul(finesine[angle], length)

	if fixedcolormap != nil {
		ds_colormap = fixedcolormap
	} else {
		index := u32(distance) >> LIGHTZSHIFT
		if index >= MAXLIGHTZ {
			index = MAXLIGHTZ - 1
		}
		ds_colormap = planezlight[index]
	}

	ds_y = y
	ds_x1 = x1
	ds_x2 = x2

	spanfunc()
}

R_ClearPlanes :: proc() {
	for i := 0; i < viewwidth; i += 1 {
		floorclip[i] = i16(viewheight)
		ceilingclip[i] = -1
	}

	lastvisplane = &visplanes[0]
	lastopening = &openings[0]

	libc.memset(&cachedheight[0], 0, size_of(cachedheight))

	angle := (viewangle - ANG90) >> ANGLETOFINESHIFT

	basexscale = FixedDiv(finecosine[angle], centerxfrac)
	baseyscale = -FixedDiv(finesine[angle], centerxfrac)
}

R_FindPlane :: proc(height_in: fixed_t, picnum: int, lightlevel_in: int) -> ^visplane_t {
	height := height_in
	lightlevel := lightlevel_in

	if picnum == skyflatnum {
		height = 0
		lightlevel = 0
	}

	check := ([^]visplane_t)(&visplanes[0])
	for check != lastvisplane {
		if height == check[0].height && picnum == check[0].picnum && lightlevel == check[0].lightlevel {
			break
		}
		check = &check[1]
	}

	if check != lastvisplane {
		return &check[0]
	}

	if int(uintptr(lastvisplane) - uintptr(&visplanes[0])) / size_of(visplane_t) == MAXVISPLANES {
		I_Error("R_FindPlane: no more visplanes")
	}

	lastvisplane = &lastvisplane[1]

	check[0].height = height
	check[0].picnum = picnum
	check[0].lightlevel = lightlevel
	check[0].minx = SCREENWIDTH
	check[0].maxx = -1

	libc.memset(&check[0].top[0], 0xff, size_of(check[0].top))

	return &check[0]
}

R_CheckPlane :: proc(pl_in: ^visplane_t, start: int, stop: int) -> ^visplane_t {
	pl := pl_in
	intrl, intrh, unionl, unionh: int

	if start < pl.minx {
		intrl = pl.minx
		unionl = start
	} else {
		unionl = pl.minx
		intrl = start
	}

	if stop > pl.maxx {
		intrh = pl.maxx
		unionh = stop
	} else {
		unionh = pl.maxx
		intrh = stop
	}

	x := intrl
	for ; x <= intrh; x += 1 {
		if pl.top[x] != 0xff {
			break
		}
	}

	if x > intrh {
		pl.minx = unionl
		pl.maxx = unionh
		return pl
	}

	lastvisplane[0].height = pl.height
	lastvisplane[0].picnum = pl.picnum
	lastvisplane[0].lightlevel = pl.lightlevel

	pl = &lastvisplane[0]
	lastvisplane = &lastvisplane[1]
	pl.minx = start
	pl.maxx = stop

	libc.memset(&pl.top[0], 0xff, size_of(pl.top))

	return pl
}

R_MakeSpans :: proc(x: int, t1_in: int, b1_in: int, t2_in: int, b2_in: int) {
	t1 := t1_in
	b1 := b1_in
	t2 := t2_in
	b2 := b2_in

	for t1 < t2 && t1 <= b1 {
		R_MapPlane(t1, spanstart[t1], x - 1)
		t1 += 1
	}
	for b1 > b2 && b1 >= t1 {
		R_MapPlane(b1, spanstart[b1], x - 1)
		b1 -= 1
	}

	for t2 < t1 && t2 <= b2 {
		spanstart[t2] = x
		t2 += 1
	}
	for b2 > b1 && b2 >= t2 {
		spanstart[b2] = x
		b2 -= 1
	}
}

R_DrawPlanes :: proc() {
	if int(uintptr(ds_p) - uintptr(&drawsegs[0])) / size_of(drawseg_t) > MAXDRAWSEGS {
		I_Error("R_DrawPlanes: drawsegs overflow (%i)", i32((uintptr(ds_p) - uintptr(&drawsegs[0])) / size_of(drawseg_t)))
	}

	if int(uintptr(lastvisplane) - uintptr(&visplanes[0])) / size_of(visplane_t) > MAXVISPLANES {
		I_Error("R_DrawPlanes: visplane overflow (%i)", i32((uintptr(lastvisplane) - uintptr(&visplanes[0])) / size_of(visplane_t)))
	}

	if int(uintptr(lastopening) - uintptr(&openings[0])) / size_of(i16) > MAXOPENINGS {
		I_Error("R_DrawPlanes: opening overflow (%i)", i32((uintptr(lastopening) - uintptr(&openings[0])) / size_of(i16)))
	}

	for pl := ([^]visplane_t)(&visplanes[0]); pl != lastvisplane; pl = &pl[1] {
		if pl[0].minx > pl[0].maxx {
			continue
		}

		if pl[0].picnum == skyflatnum {
			dc_iscale = pspriteiscale >> uint(detailshift)
			dc_colormap = colormaps
			dc_texturemid = fixed_t(skytexturemid)
			for x := pl[0].minx; x <= pl[0].maxx; x += 1 {
				dc_yl = int(pl[0].top[x])
				dc_yh = int(pl[0].bottom[x])

				if dc_yl <= dc_yh {
					angle := int((viewangle + xtoviewangle[x]) >> ANGLETOSKYSHIFT)
					dc_x = x
					dc_source = R_GetColumn(skytexture, angle)
					colfunc()
				}
			}
			continue
		}

		ds_source = ([^]byte)(W_CacheLumpNum(firstflat + flattranslation[pl[0].picnum], PU_STATIC))

		planeheight = abs(pl[0].height - viewz)
		light := (pl[0].lightlevel >> LIGHTSEGSHIFT) + extralight

		if light >= LIGHTLEVELS {
			light = LIGHTLEVELS - 1
		}
		if light < 0 {
			light = 0
		}

		planezlight = &zlight[light][0]

		// Note: pl[0].top has pad1 before it and pad2 after it in visplane_t, so minx-1 and maxx+1 are safe via pointer indexing
		top_ptr := ([^]byte)(&pl[0].top[0])
		bottom_ptr := ([^]byte)(&pl[0].bottom[0])
		top_ptr[pl[0].maxx + 1] = 0xff
		top_b := (^byte)(int(uintptr(top_ptr)) + pl[0].minx - 1)
		top_b^ = 0xff

		stop := pl[0].maxx + 1

		for x := pl[0].minx; x <= stop; x += 1 {
			t1 := int((^byte)(int(uintptr(top_ptr)) + x - 1)^)
			b1 := int((^byte)(int(uintptr(bottom_ptr)) + x - 1)^)
			t2 := int(top_ptr[x])
			b2 := int(bottom_ptr[x])
			R_MakeSpans(x, t1, b1, t2, b2)
		}

		Z_ChangeTag(ds_source, PU_CACHE)
	}
}
