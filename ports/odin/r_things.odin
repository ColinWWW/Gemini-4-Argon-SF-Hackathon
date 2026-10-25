// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

MINZ        :: FRACUNIT * 4
BASEYCENTER :: 100

maskdraw_t :: struct {
	x1:         int,
	x2:         int,
	column:     int,
	topclip:    int,
	bottomclip: int,
}

pspritescale: fixed_t
pspriteiscale: fixed_t

spritelights: [^][^]lighttable_t

negonearray: [SCREENWIDTH]i16
screenheightarray: [SCREENWIDTH]i16

sprites: [^]spritedef_t
numsprites: int

sprtemp: [29]spriteframe_t
maxframe: int
spritename: cstring

vissprites: [MAXVISSPRITES]vissprite_t
vissprite_p: [^]vissprite_t
newvissprite: int

overflowsprite: vissprite_t

mfloorclip: [^]i16
mceilingclip: [^]i16

spryscale: fixed_t
sprtopscreen: fixed_t

vsprsortedhead: vissprite_t

R_InstallSpriteLump :: proc(lump: int, frame: uint, rotation_in: uint, flipped: boolean) {
	rotation := rotation_in
	if frame >= 29 || rotation > 8 {
		I_Error("R_InstallSpriteLump: Bad frame characters in lump %i", i32(lump))
	}

	if int(frame) > maxframe {
		maxframe = int(frame)
	}

	if rotation == 0 {
		if sprtemp[frame].rotate == 0 {
			I_Error("R_InitSprites: Sprite %s frame %c has multip rot=0 lump", spritename, i32('A' + frame))
		}
		if sprtemp[frame].rotate == 1 {
			I_Error("R_InitSprites: Sprite %s frame %c has rotations and a rot=0 lump", spritename, i32('A' + frame))
		}

		sprtemp[frame].rotate = 0
		for r := 0; r < 8; r += 1 {
			sprtemp[frame].lump[r] = i16(lump - firstspritelump)
			sprtemp[frame].flip[r] = flipped ? 1 : 0
		}
		return
	}

	if sprtemp[frame].rotate == 0 {
		I_Error("R_InitSprites: Sprite %s frame %c has rotations and a rot=0 lump", spritename, i32('A' + frame))
	}

	sprtemp[frame].rotate = 1

	rotation -= 1
	if sprtemp[frame].lump[rotation] != -1 {
		I_Error("R_InitSprites: Sprite %s : %c : %c has two lumps mapped to it", spritename, i32('A' + frame), i32('1' + rotation))
	}

	sprtemp[frame].lump[rotation] = i16(lump - firstspritelump)
	sprtemp[frame].flip[rotation] = flipped ? 1 : 0
}

R_InitSpriteDefs :: proc(namelist: [^]cstring) {
	numsprites = NUMSPRITES
	if numsprites == 0 {
		return
	}

	sprites = ([^]spritedef_t)(Z_Malloc(numsprites * size_of(spritedef_t), PU_STATIC, nil))

	start := firstspritelump - 1
	end := lastspritelump + 1

	for i := 0; i < numsprites; i += 1 {
		spritename = namelist[i]
		libc.memset(&sprtemp[0], -1, size_of(sprtemp))

		maxframe = -1
		intname := (^u32)(rawptr(namelist[i]))^

		for l := start + 1; l < end; l += 1 {
			if (^u32)(&lumpinfo[l].name[0])^ == intname {
				frame := uint(lumpinfo[l].name[4] - 'A')
				rotation := uint(lumpinfo[l].name[5] - '0')

				patched: int
				if modifiedgame {
					patched = W_GetNumForName(cstring(&lumpinfo[l].name[0]))
				} else {
					patched = l
				}

				R_InstallSpriteLump(patched, frame, rotation, false)

				if lumpinfo[l].name[6] != 0 {
					frame = uint(lumpinfo[l].name[6] - 'A')
					rotation = uint(lumpinfo[l].name[7] - '0')
					R_InstallSpriteLump(l, frame, rotation, true)
				}
			}
		}

		if maxframe == -1 {
			sprites[i].numframes = 0
			continue
		}

		maxframe += 1

		for frame := 0; frame < maxframe; frame += 1 {
			switch sprtemp[frame].rotate {
			case -1:
				I_Error("R_InitSprites: No patches found for %s frame %c", namelist[i], i32(frame + 'A'))
			case 0:
			case 1:
				for rotation := 0; rotation < 8; rotation += 1 {
					if sprtemp[frame].lump[rotation] == -1 {
						I_Error("R_InitSprites: Sprite %s frame %c is missing rotations", namelist[i], i32(frame + 'A'))
					}
				}
			}
		}

		sprites[i].numframes = maxframe
		sprites[i].spriteframes = ([^]spriteframe_t)(Z_Malloc(maxframe * size_of(spriteframe_t), PU_STATIC, nil))
		libc.memcpy(sprites[i].spriteframes, &sprtemp[0], uint(maxframe * size_of(spriteframe_t)))
	}
}

R_InitSprites :: proc(namelist: [^]cstring) {
	for i := 0; i < SCREENWIDTH; i += 1 {
		negonearray[i] = -1
	}
	R_InitSpriteDefs(namelist)
}

R_ClearSprites :: proc() {
	vissprite_p = &vissprites[0]
}

R_NewVisSprite :: proc() -> ^vissprite_t {
	if vissprite_p == &vissprites[MAXVISSPRITES] {
		return &overflowsprite
	}
	res := &vissprite_p[0]
	vissprite_p = &vissprite_p[1]
	return res
}

R_DrawMaskedColumn :: proc(column_in: ^column_t) {
	column := column_in
	basetexturemid := dc_texturemid

	for column.topdelta != 0xff {
		topscreen := sprtopscreen + spryscale * fixed_t(column.topdelta)
		bottomscreen := topscreen + spryscale * fixed_t(column.length)

		dc_yl = int((topscreen + FRACUNIT - 1) >> FRACBITS)
		dc_yh = int((bottomscreen - 1) >> FRACBITS)

		mfc := int(mfloorclip[dc_x])
		mcc := int(mceilingclip[dc_x])

		if dc_yh >= mfc {
			dc_yh = mfc - 1
		}
		if dc_yl <= mcc {
			dc_yl = mcc + 1
		}

		if dc_yl <= dc_yh {
			dc_source = ([^]byte)(uintptr(column) + 3)
			dc_texturemid = basetexturemid - fixed_t(i32(column.topdelta) << FRACBITS)
			colfunc()
		}
		column = (^column_t)(uintptr(column) + uintptr(column.length) + 4)
	}

	dc_texturemid = basetexturemid
}

R_DrawVisSprite :: proc(vis: ^vissprite_t, x1: int, x2: int) {
	patch := (^patch_t)(W_CacheLumpNum(vis.patch + firstspritelump, PU_CACHE))

	dc_colormap = vis.colormap

	if dc_colormap == nil {
		colfunc = fuzzcolfunc
	} else if (vis.mobjflags & MF_TRANSLATION) != 0 {
		colfunc = R_DrawTranslatedColumn
		offset := -256 + ((vis.mobjflags & MF_TRANSLATION) >> (MF_TRANSSHIFT - 8))
		dc_translation = ([^]byte)(uintptr(int(uintptr(translationtables)) + offset))
	}

	dc_iscale = abs(vis.xiscale) >> uint(detailshift)
	dc_texturemid = vis.texturemid
	frac := vis.startfrac
	spryscale = vis.scale
	sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale)

	colofs := ([^]i32)(&patch.columnofs[0])
	for dc_x = vis.x1; dc_x <= vis.x2; dc_x += 1 {
		texturecolumn := int(frac >> FRACBITS)
		if texturecolumn < 0 || texturecolumn >= int(SHORT(patch.width)) {
			I_Error("R_DrawSpriteRange: bad texturecolumn")
		}
		column := (^column_t)(uintptr(patch) + uintptr(LONG(colofs[texturecolumn])))
		R_DrawMaskedColumn(column)
		frac += vis.xiscale
	}

	colfunc = basecolfunc
}

R_ProjectSprite :: proc(thing: ^mobj_t) {
	tr_x := thing.x - viewx
	tr_y := thing.y - viewy

	gxt := FixedMul(tr_x, viewcos)
	gyt := -FixedMul(tr_y, viewsin)

	tz := gxt - gyt

	if tz < MINZ {
		return
	}

	xscale := FixedDiv(projection, tz)

	gxt = -FixedMul(tr_x, viewsin)
	gyt = FixedMul(tr_y, viewcos)
	tx := -(gyt + gxt)

	if abs(tx) > (tz << 2) {
		return
	}

	if uint(thing.sprite) >= uint(numsprites) {
		I_Error("R_ProjectSprite: invalid sprite number %i ", i32(thing.sprite))
	}
	sprdef := &sprites[thing.sprite]
	if (thing.frame & FF_FRAMEMASK) >= sprdef.numframes {
		I_Error("R_ProjectSprite: invalid sprite frame %i : %i ", i32(thing.sprite), i32(thing.frame))
	}
	sprframe := &sprdef.spriteframes[thing.frame & FF_FRAMEMASK]

	lump: int
	flip: boolean
	if sprframe.rotate != 0 {
		ang := R_PointToAngle(thing.x, thing.y)
		rot := (ang - thing.angle + angle_t(ANG45 / 2) * 9) >> 29
		lump = int(sprframe.lump[rot])
		flip = sprframe.flip[rot] != 0
	} else {
		lump = int(sprframe.lump[0])
		flip = sprframe.flip[0] != 0
	}

	tx -= spriteoffset[lump]
	x1 := int((centerxfrac + FixedMul(tx, xscale)) >> FRACBITS)

	if x1 > viewwidth {
		return
	}

	tx += spritewidth[lump]
	x2 := int((centerxfrac + FixedMul(tx, xscale)) >> FRACBITS) - 1

	if x2 < 0 {
		return
	}

	vis := R_NewVisSprite()
	vis.mobjflags = thing.flags
	vis.scale = xscale << uint(detailshift)
	vis.gx = thing.x
	vis.gy = thing.y
	vis.gz = thing.z
	vis.gzt = thing.z + spritetopoffset[lump]
	vis.texturemid = vis.gzt - viewz
	vis.x1 = x1 < 0 ? 0 : x1
	vis.x2 = x2 >= viewwidth ? viewwidth - 1 : x2
	iscale := FixedDiv(FRACUNIT, xscale)

	if flip {
		vis.startfrac = spritewidth[lump] - 1
		vis.xiscale = -iscale
	} else {
		vis.startfrac = 0
		vis.xiscale = iscale
	}

	if vis.x1 > x1 {
		vis.startfrac += vis.xiscale * fixed_t(vis.x1 - x1)
	}
	vis.patch = lump

	if (thing.flags & MF_SHADOW) != 0 {
		vis.colormap = nil
	} else if fixedcolormap != nil {
		vis.colormap = fixedcolormap
	} else if (thing.frame & FF_FULLBRIGHT) != 0 {
		vis.colormap = colormaps
	} else {
		index := int(xscale >> uint(LIGHTSCALESHIFT - detailshift))
		if index >= MAXLIGHTSCALE {
			index = MAXLIGHTSCALE - 1
		}
		vis.colormap = spritelights[index]
	}
}

R_AddSprites :: proc(sec: ^sector_t) {
	if sec.validcount == validcount {
		return
	}
	sec.validcount = validcount

	lightnum := (int(sec.lightlevel) >> LIGHTSEGSHIFT) + extralight

	if lightnum < 0 {
		spritelights = &scalelight[0][0]
	} else if lightnum >= LIGHTLEVELS {
		spritelights = &scalelight[LIGHTLEVELS - 1][0]
	} else {
		spritelights = &scalelight[lightnum][0]
	}

	for thing := sec.thinglist; thing != nil; thing = thing.snext {
		R_ProjectSprite(thing)
	}
}

R_DrawPSprite :: proc(psp: ^pspdef_t) {
	avis: vissprite_t

	if uint(psp.state.sprite) >= uint(numsprites) {
		I_Error("R_ProjectSprite: invalid sprite number %i ", i32(psp.state.sprite))
	}
	sprdef := &sprites[psp.state.sprite]
	if (psp.state.frame & FF_FRAMEMASK) >= sprdef.numframes {
		I_Error("R_ProjectSprite: invalid sprite frame %i : %i ", i32(psp.state.sprite), i32(psp.state.frame))
	}
	sprframe := &sprdef.spriteframes[psp.state.frame & FF_FRAMEMASK]

	lump := int(sprframe.lump[0])
	flip := sprframe.flip[0] != 0

	tx := psp.sx - 160 * FRACUNIT
	tx -= spriteoffset[lump]
	x1 := int((centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS)

	if x1 > viewwidth {
		return
	}

	tx += spritewidth[lump]
	x2 := int((centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS) - 1

	if x2 < 0 {
		return
	}

	vis := &avis
	vis.mobjflags = 0
	vis.texturemid = fixed_t((BASEYCENTER << FRACBITS) + FRACUNIT / 2) - (psp.sy - spritetopoffset[lump])
	vis.x1 = x1 < 0 ? 0 : x1
	vis.x2 = x2 >= viewwidth ? viewwidth - 1 : x2
	vis.scale = pspritescale << uint(detailshift)

	if flip {
		vis.xiscale = -pspriteiscale
		vis.startfrac = spritewidth[lump] - 1
	} else {
		vis.xiscale = pspriteiscale
		vis.startfrac = 0
	}

	if vis.x1 > x1 {
		vis.startfrac += vis.xiscale * fixed_t(vis.x1 - x1)
	}

	vis.patch = lump

	if viewplayer.powers[pw_invisibility] > 4 * 32 || (viewplayer.powers[pw_invisibility] & 8) != 0 {
		vis.colormap = nil
	} else if fixedcolormap != nil {
		vis.colormap = fixedcolormap
	} else if (psp.state.frame & FF_FULLBRIGHT) != 0 {
		vis.colormap = colormaps
	} else {
		vis.colormap = spritelights[MAXLIGHTSCALE - 1]
	}

	R_DrawVisSprite(vis, vis.x1, vis.x2)
}

R_DrawPlayerSprites :: proc() {
	lightnum := (int(viewplayer.mo.subsector.sector.lightlevel) >> LIGHTSEGSHIFT) + extralight

	if lightnum < 0 {
		spritelights = &scalelight[0][0]
	} else if lightnum >= LIGHTLEVELS {
		spritelights = &scalelight[LIGHTLEVELS - 1][0]
	} else {
		spritelights = &scalelight[lightnum][0]
	}

	mfloorclip = &screenheightarray[0]
	mceilingclip = &negonearray[0]

	for i := 0; i < NUMPSPRITES; i += 1 {
		psp := &viewplayer.psprites[i]
		if psp.state != nil {
			R_DrawPSprite(psp)
		}
	}
}

R_SortVisSprites :: proc() {
	unsorted: vissprite_t
	count := int(uintptr(vissprite_p) - uintptr(&vissprites[0])) / size_of(vissprite_t)

	unsorted.next = &unsorted
	unsorted.prev = &unsorted

	if count == 0 {
		return
	}

	for i := 0; i < count; i += 1 {
		ds := &vissprites[i]
		if i + 1 < count {
			ds.next = &vissprites[i + 1]
		}
		if i > 0 {
			ds.prev = &vissprites[i - 1]
		}
	}

	vissprites[0].prev = &unsorted
	unsorted.next = &vissprites[0]
	vissprites[count - 1].next = &unsorted
	unsorted.prev = &vissprites[count - 1]

	vsprsortedhead.next = &vsprsortedhead
	vsprsortedhead.prev = &vsprsortedhead

	for i := 0; i < count; i += 1 {
		bestscale := fixed_t(max(i32))
		best: ^vissprite_t = nil
		for ds := unsorted.next; ds != &unsorted; ds = ds.next {
			if ds.scale < bestscale {
				bestscale = ds.scale
				best = ds
			}
		}
		best.next.prev = best.prev
		best.prev.next = best.next
		best.next = &vsprsortedhead
		best.prev = vsprsortedhead.prev
		vsprsortedhead.prev.next = best
		vsprsortedhead.prev = best
	}
}

R_DrawSprite :: proc(spr: ^vissprite_t) {
	clipbot: [SCREENWIDTH]i16
	cliptop: [SCREENWIDTH]i16

	for x := spr.x1; x <= spr.x2; x += 1 {
		clipbot[x] = -2
		cliptop[x] = -2
	}

	num_ds := int(uintptr(ds_p) - uintptr(&drawsegs[0])) / size_of(drawseg_t)
	for idx := num_ds - 1; idx >= 0; idx -= 1 {
		ds := &drawsegs[idx]

		if ds.x1 > spr.x2 || ds.x2 < spr.x1 || (ds.silhouette == 0 && ds.maskedtexturecol == nil) {
			continue
		}

		r1 := ds.x1 < spr.x1 ? spr.x1 : ds.x1
		r2 := ds.x2 > spr.x2 ? spr.x2 : ds.x2

		lowscale, scale: fixed_t
		if ds.scale1 > ds.scale2 {
			lowscale = ds.scale2
			scale = ds.scale1
		} else {
			lowscale = ds.scale1
			scale = ds.scale2
		}

		if scale < spr.scale || (lowscale < spr.scale && R_PointOnSegSide(spr.gx, spr.gy, ds.curline) == 0) {
			if ds.maskedtexturecol != nil {
				R_RenderMaskedSegRange(ds, r1, r2)
			}
			continue
		}

		silhouette := ds.silhouette

		if spr.gz >= ds.bsilheight {
			silhouette &= ~int(SIL_BOTTOM)
		}
		if spr.gzt <= ds.tsilheight {
			silhouette &= ~int(SIL_TOP)
		}

		if silhouette == 1 {
			for x := r1; x <= r2; x += 1 {
				if clipbot[x] == -2 {
					clipbot[x] = ([^]i16)(ds.sprbottomclip)[x]
				}
			}
		} else if silhouette == 2 {
			for x := r1; x <= r2; x += 1 {
				if cliptop[x] == -2 {
					cliptop[x] = ([^]i16)(ds.sprtopclip)[x]
				}
			}
		} else if silhouette == 3 {
			for x := r1; x <= r2; x += 1 {
				if clipbot[x] == -2 {
					clipbot[x] = ([^]i16)(ds.sprbottomclip)[x]
				}
				if cliptop[x] == -2 {
					cliptop[x] = ([^]i16)(ds.sprtopclip)[x]
				}
			}
		}
	}

	for x := spr.x1; x <= spr.x2; x += 1 {
		if clipbot[x] == -2 {
			clipbot[x] = i16(viewheight)
		}
		if cliptop[x] == -2 {
			cliptop[x] = -1
		}
	}

	mfloorclip = &clipbot[0]
	mceilingclip = &cliptop[0]
	R_DrawVisSprite(spr, spr.x1, spr.x2)
}

R_DrawMasked :: proc() {
	R_SortVisSprites()

	if uintptr(vissprite_p) > uintptr(&vissprites[0]) {
		for spr := vsprsortedhead.next; spr != &vsprsortedhead; spr = spr.next {
			R_DrawSprite(spr)
		}
	}

	num_ds := int(uintptr(ds_p) - uintptr(&drawsegs[0])) / size_of(drawseg_t)
	for idx := num_ds - 1; idx >= 0; idx -= 1 {
		ds := &drawsegs[idx]
		if ds.maskedtexturecol != nil {
			R_RenderMaskedSegRange(ds, ds.x1, ds.x2)
		}
	}

	if viewangleoffset == 0 {
		R_DrawPlayerSprites()
	}
}