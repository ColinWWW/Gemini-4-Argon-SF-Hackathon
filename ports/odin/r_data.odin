// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

mappatch_t :: struct #packed {
	originx:  i16,
	originy:  i16,
	patch:    i16,
	stepdir:  i16,
	colormap: i16,
}

maptexture_t :: struct #packed {
	name:            [8]u8,
	masked:          i32,
	width:           i16,
	height:          i16,
	columndirectory: i32,
	patchcount:      i16,
	patches:         [1]mappatch_t,
}

texpatch_t :: struct {
	originx: int,
	originy: int,
	patch:   int,
}

texture_t :: struct {
	name:       [8]u8,
	width:      i16,
	height:     i16,
	patchcount: i16,
	patches:    [1]texpatch_t,
}

firstflat: int
lastflat: int
numflats: int

firstpatch: int
lastpatch: int
numpatches: int

firstspritelump: int
lastspritelump: int
numspritelumps: int

numtextures: int
textures: [^]^texture_t

texturewidthmask: [^]int
textureheight: [^]fixed_t
texturecompositesize: [^]int
texturecolumnlump: [^][^]i16
texturecolumnofs: [^][^]u16
texturecomposite: [^][^]byte

flattranslation: [^]int
texturetranslation: [^]int

spritewidth: [^]fixed_t
spriteoffset: [^]fixed_t
spritetopoffset: [^]fixed_t

colormaps: [^]lighttable_t

flatmemory: int
texturememory: int
spritememory: int

R_DrawColumnInCache :: proc(patch_in: ^column_t, cache: [^]byte, originy: int, cacheheight: int) {
	patch := patch_in
	for patch.topdelta != 0xff {
		source := ([^]byte)(uintptr(patch) + 3)
		count := int(patch.length)
		position := originy + int(patch.topdelta)

		if position < 0 {
			count += position
			position = 0
		}

		if position + count > cacheheight {
			count = cacheheight - position
		}

		if count > 0 {
			libc.memcpy(&cache[position], source, uint(count))
		}

		patch = (^column_t)(uintptr(patch) + uintptr(patch.length) + 4)
	}
}

R_GenerateComposite :: proc(texnum: int) {
	texture := textures[texnum]

	block := ([^]byte)(Z_Malloc(texturecompositesize[texnum], PU_STATIC, &texturecomposite[texnum]))

	collump := texturecolumnlump[texnum]
	colofs := texturecolumnofs[texnum]

	patches := ([^]texpatch_t)(&texture.patches[0])
	for i := 0; i < int(texture.patchcount); i += 1 {
		patch := &patches[i]
		realpatch := (^patch_t)(W_CacheLumpNum(patch.patch, PU_CACHE))
		x1 := patch.originx
		x2 := x1 + int(SHORT(realpatch.width))

		x: int
		if x1 < 0 {
			x = 0
		} else {
			x = x1
		}

		if x2 > int(texture.width) {
			x2 = int(texture.width)
		}

		patch_colofs := ([^]i32)(&realpatch.columnofs[0])
		for ; x < x2; x += 1 {
			if collump[x] >= 0 {
				continue
			}

			patchcol := (^column_t)(uintptr(realpatch) + uintptr(LONG(patch_colofs[x - x1])))
			R_DrawColumnInCache(patchcol, &block[colofs[x]], patch.originy, int(texture.height))
		}
	}

	Z_ChangeTag(block, PU_CACHE)
}

R_GenerateLookup :: proc(texnum: int) {
	texture := textures[texnum]

	texturecomposite[texnum] = nil
	texturecompositesize[texnum] = 0
	collump := texturecolumnlump[texnum]
	colofs := texturecolumnofs[texnum]

	patchcount := ([^]byte)(libc.malloc(uint(texture.width)))
	defer libc.free(patchcount)
	libc.memset(patchcount, 0, uint(texture.width))

	patches := ([^]texpatch_t)(&texture.patches[0])
	for i := 0; i < int(texture.patchcount); i += 1 {
		patch := &patches[i]
		realpatch := (^patch_t)(W_CacheLumpNum(patch.patch, PU_CACHE))
		x1 := patch.originx
		x2 := x1 + int(SHORT(realpatch.width))

		x: int
		if x1 < 0 {
			x = 0
		} else {
			x = x1
		}

		if x2 > int(texture.width) {
			x2 = int(texture.width)
		}

		patch_colofs := ([^]i32)(&realpatch.columnofs[0])
		for ; x < x2; x += 1 {
			patchcount[x] += 1
			collump[x] = i16(patch.patch)
			colofs[x] = u16(LONG(patch_colofs[x - x1]) + 3)
		}
	}

	for x := 0; x < int(texture.width); x += 1 {
		if patchcount[x] == 0 {
			libc.printf("R_GenerateLookup: column without a patch (%s)\n", &texture.name[0])
			return
		}

		if patchcount[x] > 1 {
			collump[x] = -1
			colofs[x] = u16(texturecompositesize[texnum])

			if texturecompositesize[texnum] > 0x10000 - int(texture.height) {
				I_Error("R_GenerateLookup: texture %i is >64k", i32(texnum))
			}

			texturecompositesize[texnum] += int(texture.height)
		}
	}
}

R_GetColumn :: proc(tex: int, col_in: int) -> [^]byte {
	col := col_in & texturewidthmask[tex]
	lump := int(texturecolumnlump[tex][col])
	ofs := int(texturecolumnofs[tex][col])

	if lump > 0 {
		base := ([^]byte)(W_CacheLumpNum(lump, PU_CACHE))
		return &base[ofs]
	}

	if texturecomposite[tex] == nil {
		R_GenerateComposite(tex)
	}

	return &texturecomposite[tex][ofs]
}

R_InitTextures :: proc() {
	name: [9]u8
	name[8] = 0

	names := ([^]byte)(W_CacheLumpName("PNAMES", PU_STATIC))
	nummappatches := int(LONG((^i32)(names)^))
	name_p := &names[4]
	patchlookup := ([^]int)(libc.malloc(uint(nummappatches * size_of(int))))
	defer libc.free(patchlookup)

	for i := 0; i < nummappatches; i += 1 {
		libc.strncpy(&name[0], cstring(&name_p[i * 8]), 8)
		patchlookup[i] = W_CheckNumForName(cstring(&name[0]))
	}
	Z_Free(names)

	maptex1 := ([^]i32)(W_CacheLumpName("TEXTURE1", PU_STATIC))
	maptex := maptex1
	numtextures1 := int(LONG(maptex[0]))
	maxoff := W_LumpLength(W_GetNumForName("TEXTURE1"))
	directory := &maptex[1]

	maptex2: [^]i32 = nil
	numtextures2 := 0
	maxoff2 := 0
	if W_CheckNumForName("TEXTURE2") != -1 {
		maptex2 = ([^]i32)(W_CacheLumpName("TEXTURE2", PU_STATIC))
		numtextures2 = int(LONG(maptex2[0]))
		maxoff2 = W_LumpLength(W_GetNumForName("TEXTURE2"))
	}

	numtextures = numtextures1 + numtextures2

	textures = ([^]^texture_t)(Z_Malloc(numtextures * size_of(rawptr), PU_STATIC, nil))
	texturecolumnlump = ([^][^]i16)(Z_Malloc(numtextures * size_of(rawptr), PU_STATIC, nil))
	texturecolumnofs = ([^][^]u16)(Z_Malloc(numtextures * size_of(rawptr), PU_STATIC, nil))
	texturecomposite = ([^][^]byte)(Z_Malloc(numtextures * size_of(rawptr), PU_STATIC, nil))
	texturecompositesize = ([^]int)(Z_Malloc(numtextures * size_of(int), PU_STATIC, nil))
	texturewidthmask = ([^]int)(Z_Malloc(numtextures * size_of(int), PU_STATIC, nil))
	textureheight = ([^]fixed_t)(Z_Malloc(numtextures * size_of(fixed_t), PU_STATIC, nil))

	totalwidth := 0

	temp1 := W_GetNumForName("S_START")
	temp2 := W_GetNumForName("S_END") - 1
	temp3 := ((temp2 - temp1 + 63) / 64) + ((numtextures + 63) / 64)
	libc.printf("[")
	for i := 0; i < temp3; i += 1 {
		libc.printf(" ")
	}
	libc.printf("         ]")
	for i := 0; i < temp3; i += 1 {
		libc.printf("\x08")
	}
	libc.printf("\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08")

	for i := 0; i < numtextures; i += 1 {
		if (i & 63) == 0 {
			libc.printf(".")
		}

		if i == numtextures1 {
			maptex = maptex2
			maxoff = maxoff2
			directory = &maptex[1]
		}

		offset := int(LONG(directory[0]))
		directory = &directory[1]

		if offset > maxoff {
			I_Error("R_InitTextures: bad texture directory")
		}

		mtexture := (^maptexture_t)(uintptr(maptex) + uintptr(offset))
		pcount := int(SHORT(mtexture.patchcount))

		tex_alloc_size := size_of(texture_t) + size_of(texpatch_t) * (pcount - 1)
		if pcount < 1 {
			tex_alloc_size = size_of(texture_t)
		}
		texture := (^texture_t)(Z_Malloc(tex_alloc_size, PU_STATIC, nil))
		textures[i] = texture

		texture.width = SHORT(mtexture.width)
		texture.height = SHORT(mtexture.height)
		texture.patchcount = SHORT(mtexture.patchcount)

		texture.name = mtexture.name
		mpatches := ([^]mappatch_t)(&mtexture.patches[0])
		tpatches := ([^]texpatch_t)(&texture.patches[0])

		for j := 0; j < pcount; j += 1 {
			tpatches[j].originx = int(SHORT(mpatches[j].originx))
			tpatches[j].originy = int(SHORT(mpatches[j].originy))
			tpatches[j].patch = patchlookup[SHORT(mpatches[j].patch)]
			if tpatches[j].patch == -1 {
				I_Error("R_InitTextures: Missing patch in texture %s", &texture.name[0])
			}
		}

		texturecolumnlump[i] = ([^]i16)(Z_Malloc(int(texture.width) * 2, PU_STATIC, nil))
		texturecolumnofs[i] = ([^]u16)(Z_Malloc(int(texture.width) * 2, PU_STATIC, nil))

		j := 1
		for j * 2 <= int(texture.width) {
			j <<= 1
		}

		texturewidthmask[i] = j - 1
		textureheight[i] = fixed_t(i32(texture.height) << FRACBITS)

		totalwidth += int(texture.width)
	}

	Z_Free(maptex1)
	if maptex2 != nil {
		Z_Free(maptex2)
	}

	for i := 0; i < numtextures; i += 1 {
		R_GenerateLookup(i)
	}

	texturetranslation = ([^]int)(Z_Malloc((numtextures + 1) * size_of(int), PU_STATIC, nil))
	for i := 0; i < numtextures; i += 1 {
		texturetranslation[i] = i
	}
}

R_InitFlats :: proc() {
	firstflat = W_GetNumForName("F_START") + 1
	lastflat = W_GetNumForName("F_END") - 1
	numflats = lastflat - firstflat + 1

	flattranslation = ([^]int)(Z_Malloc((numflats + 1) * size_of(int), PU_STATIC, nil))
	for i := 0; i < numflats; i += 1 {
		flattranslation[i] = i
	}
}

R_InitSpriteLumps :: proc() {
	firstspritelump = W_GetNumForName("S_START") + 1
	lastspritelump = W_GetNumForName("S_END") - 1

	numspritelumps = lastspritelump - firstspritelump + 1
	spritewidth = ([^]fixed_t)(Z_Malloc(numspritelumps * size_of(fixed_t), PU_STATIC, nil))
	spriteoffset = ([^]fixed_t)(Z_Malloc(numspritelumps * size_of(fixed_t), PU_STATIC, nil))
	spritetopoffset = ([^]fixed_t)(Z_Malloc(numspritelumps * size_of(fixed_t), PU_STATIC, nil))

	for i := 0; i < numspritelumps; i += 1 {
		if (i & 63) == 0 {
			libc.printf(".")
		}

		patch := (^patch_t)(W_CacheLumpNum(firstspritelump + i, PU_CACHE))
		spritewidth[i] = fixed_t(i32(SHORT(patch.width)) << FRACBITS)
		spriteoffset[i] = fixed_t(i32(SHORT(patch.leftoffset)) << FRACBITS)
		spritetopoffset[i] = fixed_t(i32(SHORT(patch.topoffset)) << FRACBITS)
	}
}

R_InitColormaps :: proc() {
	lump := W_GetNumForName("COLORMAP")
	length := W_LumpLength(lump) + 255
	raw := Z_Malloc(length, PU_STATIC, nil)
	colormaps = ([^]lighttable_t)((uintptr(raw) + 255) & ~uintptr(0xff))
	W_ReadLump(lump, colormaps)
}

R_InitData :: proc() {
	R_InitTextures()
	libc.printf("\nInitTextures")
	R_InitFlats()
	libc.printf("\nInitFlats")
	R_InitSpriteLumps()
	libc.printf("\nInitSprites")
	R_InitColormaps()
	libc.printf("\nInitColormaps")
}

R_FlatNumForName :: proc(name: cstring) -> int {
	namet: [9]u8
	i := W_CheckNumForName(name)
	if i == -1 {
		namet[8] = 0
		libc.memcpy(&namet[0], name, 8)
		I_Error("R_FlatNumForName: %s not found", &namet[0])
	}
	return i - firstflat
}

R_CheckTextureNumForName :: proc(name: cstring) -> int {
	if ((^u8)(name))^ == '-' {
		return 0
	}

	for i := 0; i < numtextures; i += 1 {
		if c_strncasecmp(cstring(&textures[i].name[0]), name, 8) == 0 {
			return i
		}
	}
	return -1
}

R_TextureNumForName :: proc(name: cstring) -> int {
	i := R_CheckTextureNumForName(name)
	if i == -1 {
		I_Error("R_TextureNumForName: %s not found", name)
	}
	return i
}

R_PrecacheLevel :: proc() {
	if demoplayback {
		return
	}

	flatpresent := ([^]u8)(libc.malloc(uint(numflats)))
	defer libc.free(flatpresent)
	libc.memset(flatpresent, 0, uint(numflats))

	for i := 0; i < numsectors; i += 1 {
		flatpresent[sectors[i].floorpic] = 1
		flatpresent[sectors[i].ceilingpic] = 1
	}

	flatmemory = 0
	for i := 0; i < numflats; i += 1 {
		if flatpresent[i] != 0 {
			lump := firstflat + i
			flatmemory += lumpinfo[lump].size
			W_CacheLumpNum(lump, PU_CACHE)
		}
	}

	texturepresent := ([^]u8)(libc.malloc(uint(numtextures)))
	defer libc.free(texturepresent)
	libc.memset(texturepresent, 0, uint(numtextures))

	for i := 0; i < numsides; i += 1 {
		texturepresent[sides[i].toptexture] = 1
		texturepresent[sides[i].midtexture] = 1
		texturepresent[sides[i].bottomtexture] = 1
	}

	texturepresent[skytexture] = 1

	texturememory = 0
	for i := 0; i < numtextures; i += 1 {
		if texturepresent[i] == 0 {
			continue
		}
		texture := textures[i]
		tpatches := ([^]texpatch_t)(&texture.patches[0])
		for j := 0; j < int(texture.patchcount); j += 1 {
			lump := tpatches[j].patch
			texturememory += lumpinfo[lump].size
			W_CacheLumpNum(lump, PU_CACHE)
		}
	}

	spritepresent := ([^]u8)(libc.malloc(uint(numsprites)))
	defer libc.free(spritepresent)
	libc.memset(spritepresent, 0, uint(numsprites))

	for th := thinkercap.next; th != &thinkercap; th = th.next {
		if th.function.acp1 == P_MobjThinker {
			spritepresent[((^mobj_t)(th)).sprite] = 1
		}
	}

	spritememory = 0
	for i := 0; i < numsprites; i += 1 {
		if spritepresent[i] == 0 {
			continue
		}
		for j := 0; j < sprites[i].numframes; j += 1 {
			sf := &sprites[i].spriteframes[j]
			for k := 0; k < 8; k += 1 {
				lump := firstspritelump + int(sf.lump[k])
				spritememory += lumpinfo[lump].size
				W_CacheLumpNum(lump, PU_CACHE)
			}
		}
	}
}
