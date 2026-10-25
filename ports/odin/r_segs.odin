// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

segtextured: boolean
markfloor: boolean
markceiling: boolean

maskedtexture: boolean
toptexture: int
bottomtexture: int
midtexture: int

rw_normalangle: angle_t
rw_angle1: int

rw_x: int
rw_stopx: int
rw_centerangle: angle_t
rw_offset: fixed_t
rw_distance: fixed_t
rw_scale: fixed_t
rw_scalestep: fixed_t
rw_midtexturemid: fixed_t
rw_toptexturemid: fixed_t
rw_bottomtexturemid: fixed_t

worldtop: int
worldbottom: int
worldhigh: int
worldlow: int

pixhigh: fixed_t
pixlow: fixed_t
pixhighstep: fixed_t
pixlowstep: fixed_t

topfrac: fixed_t
topstep: fixed_t

bottomfrac: fixed_t
bottomstep: fixed_t

walllights: [^][^]lighttable_t
maskedtexturecol: [^]i16

HEIGHTBITS :: 12
HEIGHTUNIT :: 1 << HEIGHTBITS

R_RenderMaskedSegRange :: proc(ds: ^drawseg_t, x1: int, x2: int) {
	curline = ds.curline
	frontsector = curline.frontsector
	backsector = curline.backsector
	texnum := texturetranslation[curline.sidedef.midtexture]

	lightnum := (int(frontsector.lightlevel) >> LIGHTSEGSHIFT) + extralight

	if curline.v1.y == curline.v2.y {
		lightnum -= 1
	} else if curline.v1.x == curline.v2.x {
		lightnum += 1
	}

	if lightnum < 0 {
		walllights = &scalelight[0][0]
	} else if lightnum >= LIGHTLEVELS {
		walllights = &scalelight[LIGHTLEVELS - 1][0]
	} else {
		walllights = &scalelight[lightnum][0]
	}

	maskedtexturecol = ds.maskedtexturecol

	rw_scalestep = ds.scalestep
	spryscale = ds.scale1 + fixed_t(x1 - ds.x1) * rw_scalestep
	mfloorclip = ds.sprbottomclip
	mceilingclip = ds.sprtopclip

	if (int(curline.linedef.flags) & ML_DONTPEGBOTTOM) != 0 {
		dc_texturemid = frontsector.floorheight > backsector.floorheight ? frontsector.floorheight : backsector.floorheight
		dc_texturemid = dc_texturemid + textureheight[texnum] - viewz
	} else {
		dc_texturemid = frontsector.ceilingheight < backsector.ceilingheight ? frontsector.ceilingheight : backsector.ceilingheight
		dc_texturemid = dc_texturemid - viewz
	}
	dc_texturemid += curline.sidedef.rowoffset

	if fixedcolormap != nil {
		dc_colormap = fixedcolormap
	}

	for dc_x = x1; dc_x <= x2; dc_x += 1 {
		mcol := maskedtexturecol[dc_x]
		if mcol != MAXSHORT {
			if fixedcolormap == nil {
				index := u32(spryscale) >> LIGHTSCALESHIFT
				if index >= MAXLIGHTSCALE {
					index = MAXLIGHTSCALE - 1
				}
				dc_colormap = walllights[index]
			}

			sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale)
			dc_iscale = fixed_t(0xffffffff / u32(spryscale))

			col_bytes := R_GetColumn(texnum, int(mcol))
			col := (^column_t)(uintptr(col_bytes) - 3)

			R_DrawMaskedColumn(col)
			maskedtexturecol[dc_x] = MAXSHORT
		}
		spryscale += rw_scalestep
	}
}

R_RenderSegLoop :: proc() {
	texturecolumn: fixed_t = 0

	for ; rw_x < rw_stopx; rw_x += 1 {
		yl := int((topfrac + HEIGHTUNIT - 1) >> HEIGHTBITS)

		if yl < int(ceilingclip[rw_x]) + 1 {
			yl = int(ceilingclip[rw_x]) + 1
		}

		if markceiling {
			top := int(ceilingclip[rw_x]) + 1
			bottom := yl - 1

			if bottom >= int(floorclip[rw_x]) {
				bottom = int(floorclip[rw_x]) - 1
			}

			if top <= bottom {
				ceilingplane.top[rw_x] = byte(top)
				ceilingplane.bottom[rw_x] = byte(bottom)
			}
		}

		yh := int(bottomfrac >> HEIGHTBITS)

		if yh >= int(floorclip[rw_x]) {
			yh = int(floorclip[rw_x]) - 1
		}

		if markfloor {
			top := yh + 1
			bottom := int(floorclip[rw_x]) - 1
			if top <= int(ceilingclip[rw_x]) {
				top = int(ceilingclip[rw_x]) + 1
			}
			if top <= bottom {
				floorplane.top[rw_x] = byte(top)
				floorplane.bottom[rw_x] = byte(bottom)
			}
		}

		if segtextured {
			angle := (rw_centerangle + xtoviewangle[rw_x]) >> ANGLETOFINESHIFT
			texturecolumn = rw_offset - FixedMul(finetangent[angle], rw_distance)
			texturecolumn >>= FRACBITS
			index := u32(rw_scale) >> LIGHTSCALESHIFT

			if index >= MAXLIGHTSCALE {
				index = MAXLIGHTSCALE - 1
			}

			dc_colormap = walllights[index]
			dc_x = rw_x
			dc_iscale = fixed_t(0xffffffff / u32(rw_scale))
		}

		if midtexture != 0 {
			dc_yl = yl
			dc_yh = yh
			dc_texturemid = rw_midtexturemid
			dc_source = R_GetColumn(midtexture, int(texturecolumn))
			colfunc()
			ceilingclip[rw_x] = i16(viewheight)
			floorclip[rw_x] = -1
		} else {
			if toptexture != 0 {
				mid := int(pixhigh >> HEIGHTBITS)
				pixhigh += pixhighstep

				if mid >= int(floorclip[rw_x]) {
					mid = int(floorclip[rw_x]) - 1
				}

				if mid >= yl {
					dc_yl = yl
					dc_yh = mid
					dc_texturemid = rw_toptexturemid
					dc_source = R_GetColumn(toptexture, int(texturecolumn))
					colfunc()
					ceilingclip[rw_x] = i16(mid)
				} else {
					ceilingclip[rw_x] = i16(yl - 1)
				}
			} else {
				if markceiling {
					ceilingclip[rw_x] = i16(yl - 1)
				}
			}

			if bottomtexture != 0 {
				mid := int((pixlow + HEIGHTUNIT - 1) >> HEIGHTBITS)
				pixlow += pixlowstep

				if mid <= int(ceilingclip[rw_x]) {
					mid = int(ceilingclip[rw_x]) + 1
				}

				if mid <= yh {
					dc_yl = mid
					dc_yh = yh
					dc_texturemid = rw_bottomtexturemid
					dc_source = R_GetColumn(bottomtexture, int(texturecolumn))
					colfunc()
					floorclip[rw_x] = i16(mid)
				} else {
					floorclip[rw_x] = i16(yh + 1)
				}
			} else {
				if markfloor {
					floorclip[rw_x] = i16(yh + 1)
				}
			}

			if maskedtexture {
				maskedtexturecol[rw_x] = i16(texturecolumn)
			}
		}

		rw_scale += rw_scalestep
		topfrac += topstep
		bottomfrac += bottomstep
	}
}

R_StoreWallRange :: proc(start: int, stop: int) {
	if ds_p == &drawsegs[MAXDRAWSEGS] {
		return
	}

	if start >= viewwidth || start > stop {
		I_Error("Bad R_RenderWallRange: %i to %i", i32(start), i32(stop))
	}

	sidedef = curline.sidedef
	linedef = curline.linedef

	linedef.flags |= ML_MAPPED

	rw_normalangle = curline.angle + ANG90
	offsetangle := angle_t(abs(i32(rw_normalangle) - i32(rw_angle1)))

	if offsetangle > ANG90 {
		offsetangle = ANG90
	}

	distangle := ANG90 - offsetangle
	hyp := R_PointToDist(curline.v1.x, curline.v1.y)
	sineval := finesine[distangle >> ANGLETOFINESHIFT]
	rw_distance = FixedMul(hyp, sineval)

	rw_x = start
	ds_p[0].x1 = start
	ds_p[0].x2 = stop
	ds_p[0].curline = curline
	rw_stopx = stop + 1

	rw_scale = R_ScaleFromGlobalAngle(viewangle + xtoviewangle[start])
	ds_p[0].scale1 = rw_scale

	if stop > start {
		ds_p[0].scale2 = R_ScaleFromGlobalAngle(viewangle + xtoviewangle[stop])
		rw_scalestep = (ds_p[0].scale2 - rw_scale) / fixed_t(stop - start)
		ds_p[0].scalestep = rw_scalestep
	} else {
		ds_p[0].scale2 = ds_p[0].scale1
	}

	worldtop = int(frontsector.ceilingheight - viewz)
	worldbottom = int(frontsector.floorheight - viewz)

	midtexture = 0
	toptexture = 0
	bottomtexture = 0
	maskedtexture = false
	ds_p[0].maskedtexturecol = nil

	if backsector == nil {
		midtexture = texturetranslation[sidedef.midtexture]
		markfloor = true
		markceiling = true
		if (int(linedef.flags) & ML_DONTPEGBOTTOM) != 0 {
			vtop := frontsector.floorheight + textureheight[sidedef.midtexture]
			rw_midtexturemid = vtop - viewz
		} else {
			rw_midtexturemid = fixed_t(worldtop)
		}
		rw_midtexturemid += sidedef.rowoffset

		ds_p[0].silhouette = SIL_BOTH
		ds_p[0].sprtopclip = &screenheightarray[0]
		ds_p[0].sprbottomclip = &negonearray[0]
		ds_p[0].bsilheight = fixed_t(MAXINT)
		ds_p[0].tsilheight = fixed_t(MININT)
	} else {
		ds_p[0].sprtopclip = nil
		ds_p[0].sprbottomclip = nil
		ds_p[0].silhouette = 0

		if frontsector.floorheight > backsector.floorheight {
			ds_p[0].silhouette = SIL_BOTTOM
			ds_p[0].bsilheight = frontsector.floorheight
		} else if backsector.floorheight > viewz {
			ds_p[0].silhouette = SIL_BOTTOM
			ds_p[0].bsilheight = fixed_t(MAXINT)
		}

		if frontsector.ceilingheight < backsector.ceilingheight {
			ds_p[0].silhouette |= SIL_TOP
			ds_p[0].tsilheight = frontsector.ceilingheight
		} else if backsector.ceilingheight < viewz {
			ds_p[0].silhouette |= SIL_TOP
			ds_p[0].tsilheight = fixed_t(MININT)
		}

		if backsector.ceilingheight <= frontsector.floorheight {
			ds_p[0].sprbottomclip = &negonearray[0]
			ds_p[0].bsilheight = fixed_t(MAXINT)
			ds_p[0].silhouette |= SIL_BOTTOM
		}

		if backsector.floorheight >= frontsector.ceilingheight {
			ds_p[0].sprtopclip = &screenheightarray[0]
			ds_p[0].tsilheight = fixed_t(MININT)
			ds_p[0].silhouette |= SIL_TOP
		}

		worldhigh = int(backsector.ceilingheight - viewz)
		worldlow = int(backsector.floorheight - viewz)

		if int(frontsector.ceilingpic) == skyflatnum && int(backsector.ceilingpic) == skyflatnum {
			worldtop = worldhigh
		}

		if worldlow != worldbottom || backsector.floorpic != frontsector.floorpic || backsector.lightlevel != frontsector.lightlevel {
			markfloor = true
		} else {
			markfloor = false
		}

		if worldhigh != worldtop || backsector.ceilingpic != frontsector.ceilingpic || backsector.lightlevel != frontsector.lightlevel {
			markceiling = true
		} else {
			markceiling = false
		}

		if backsector.ceilingheight <= frontsector.floorheight || backsector.floorheight >= frontsector.ceilingheight {
			markceiling = true
			markfloor = true
		}

		if worldhigh < worldtop {
			toptexture = texturetranslation[sidedef.toptexture]
			if (int(linedef.flags) & ML_DONTPEGTOP) != 0 {
				rw_toptexturemid = fixed_t(worldtop)
			} else {
				vtop := backsector.ceilingheight + textureheight[sidedef.toptexture]
				rw_toptexturemid = vtop - viewz
			}
		}
		if worldlow > worldbottom {
			bottomtexture = texturetranslation[sidedef.bottomtexture]
			if (int(linedef.flags) & ML_DONTPEGBOTTOM) != 0 {
				rw_bottomtexturemid = fixed_t(worldtop)
			} else {
				rw_bottomtexturemid = fixed_t(worldlow)
			}
		}
		rw_toptexturemid += sidedef.rowoffset
		rw_bottomtexturemid += sidedef.rowoffset

		if sidedef.midtexture != 0 {
			maskedtexture = true
			maskedtexturecol = ([^]i16)(uintptr(lastopening) - uintptr(rw_x * size_of(i16)))
			ds_p[0].maskedtexturecol = maskedtexturecol
			lastopening = &lastopening[rw_stopx - rw_x]
		}
	}

	segtextured = (midtexture != 0) || (toptexture != 0) || (bottomtexture != 0) || maskedtexture

	if segtextured {
		offsetangle = rw_normalangle - angle_t(rw_angle1)
		if offsetangle > ANG180 {
			offsetangle = -offsetangle
		}
		if offsetangle > ANG90 {
			offsetangle = ANG90
		}

		sineval = finesine[offsetangle >> ANGLETOFINESHIFT]
		rw_offset = FixedMul(hyp, sineval)

		if rw_normalangle - angle_t(rw_angle1) < ANG180 {
			rw_offset = -rw_offset
		}

		rw_offset += sidedef.textureoffset + curline.offset
		rw_centerangle = ANG90 + viewangle - rw_normalangle

		if fixedcolormap == nil {
			lightnum := (int(frontsector.lightlevel) >> LIGHTSEGSHIFT) + extralight

			if curline.v1.y == curline.v2.y {
				lightnum -= 1
			} else if curline.v1.x == curline.v2.x {
				lightnum += 1
			}

			if lightnum < 0 {
				walllights = &scalelight[0][0]
			} else if lightnum >= LIGHTLEVELS {
				walllights = &scalelight[LIGHTLEVELS - 1][0]
			} else {
				walllights = &scalelight[lightnum][0]
			}
		}
	}

	if frontsector.floorheight >= viewz {
		markfloor = false
	}

	if frontsector.ceilingheight <= viewz && int(frontsector.ceilingpic) != skyflatnum {
		markceiling = false
	}

	worldtop >>= 4
	worldbottom >>= 4

	topstep = -FixedMul(rw_scalestep, fixed_t(worldtop))
	topfrac = (centeryfrac >> 4) - FixedMul(fixed_t(worldtop), rw_scale)

	bottomstep = -FixedMul(rw_scalestep, fixed_t(worldbottom))
	bottomfrac = (centeryfrac >> 4) - FixedMul(fixed_t(worldbottom), rw_scale)

	if backsector != nil {
		worldhigh >>= 4
		worldlow >>= 4

		if worldhigh < worldtop {
			pixhigh = (centeryfrac >> 4) - FixedMul(fixed_t(worldhigh), rw_scale)
			pixhighstep = -FixedMul(rw_scalestep, fixed_t(worldhigh))
		}

		if worldlow > worldbottom {
			pixlow = (centeryfrac >> 4) - FixedMul(fixed_t(worldlow), rw_scale)
			pixlowstep = -FixedMul(rw_scalestep, fixed_t(worldlow))
		}
	}

	if markceiling {
		ceilingplane = R_CheckPlane(ceilingplane, rw_x, rw_stopx - 1)
	}

	if markfloor {
		floorplane = R_CheckPlane(floorplane, rw_x, rw_stopx - 1)
	}

	R_RenderSegLoop()

	if ((ds_p[0].silhouette & SIL_TOP) != 0 || maskedtexture) && ds_p[0].sprtopclip == nil {
		libc.memcpy(lastopening, &ceilingclip[start], uint(2 * (rw_stopx - start)))
		ds_p[0].sprtopclip = ([^]i16)(uintptr(lastopening) - uintptr(start * size_of(i16)))
		lastopening = &lastopening[rw_stopx - start]
	}

	if ((ds_p[0].silhouette & SIL_BOTTOM) != 0 || maskedtexture) && ds_p[0].sprbottomclip == nil {
		libc.memcpy(lastopening, &floorclip[start], uint(2 * (rw_stopx - start)))
		ds_p[0].sprbottomclip = ([^]i16)(uintptr(lastopening) - uintptr(start * size_of(i16)))
		lastopening = &lastopening[rw_stopx - start]
	}

	if maskedtexture && (ds_p[0].silhouette & SIL_TOP) == 0 {
		ds_p[0].silhouette |= SIL_TOP
		ds_p[0].tsilheight = fixed_t(MININT)
	}
	if maskedtexture && (ds_p[0].silhouette & SIL_BOTTOM) == 0 {
		ds_p[0].silhouette |= SIL_BOTTOM
		ds_p[0].bsilheight = fixed_t(MAXINT)
	}
	ds_p = &ds_p[1]
}