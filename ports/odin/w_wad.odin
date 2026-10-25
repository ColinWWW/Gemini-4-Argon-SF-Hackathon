// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

lumpinfo: [^]lumpinfo_t
numlumps: int
lumpcache: [^]rawptr

reloadlump: int
reloadname: cstring

strupr :: proc(s_in: [^]u8) {
	s := s_in
	for s[0] != 0 {
		s[0] = u8(libc.toupper(i32(s[0])))
		s = &s[1]
	}
}

ExtractFileBase :: proc(path: cstring, dest_in: [^]u8) {
	dest := dest_in
	p := ([^]u8)(path)
	plen := int(libc.strlen(path))
	src := &p[plen - 1]

	for uintptr(src) != uintptr(p) {
		prev := (^u8)(uintptr(src) - 1)^
		if prev == '\\' || prev == '/' {
			break
		}
		src = ([^]u8)(uintptr(src) - 1)
	}

	libc.memset(dest, 0, 8)
	length := 0

	for src[0] != 0 && src[0] != '.' {
		length += 1
		if length == 9 {
			I_Error("Filename base of %s >8 chars", path)
		}
		dest[0] = u8(libc.toupper(i32(src[0])))
		dest = &dest[1]
		src = &src[1]
	}
}

W_AddFile :: proc(filename_in: cstring) {
	filename := filename_in
	header: wadinfo_t
	singleinfo: filelump_t

	fn_bytes := ([^]u8)(filename)
	if fn_bytes[0] == '~' {
		filename = cstring(&fn_bytes[1])
		reloadname = filename
		reloadlump = numlumps
	}

	handle := libc.fopen(filename, "rb")
	if handle == nil {
		libc.printf(" couldn't open %s\n", filename)
		return
	}

	libc.printf(" adding %s\n", filename)
	startlump := numlumps

	fn_len := int(libc.strlen(filename))
	ext_ptr := cstring(&(([^]u8)(filename))[fn_len - 3])

	fileinfo: [^]filelump_t
	allocated_fileinfo: rawptr = nil
	defer if allocated_fileinfo != nil { libc.free(allocated_fileinfo) }

	if c_strcasecmp(ext_ptr, "wad") != 0 {
		fileinfo = &singleinfo
		singleinfo.filepos = 0
		libc.fseek(handle, 0, .END)
		singleinfo.size = LONG(i32(libc.ftell(handle)))
		libc.fseek(handle, 0, .SET)
		ExtractFileBase(filename, &singleinfo.name[0])
		numlumps += 1
	} else {
		libc.fread(&header, size_of(wadinfo_t), 1, handle)
		if libc.strncmp(cstring(&header.identification[0]), "IWAD", 4) != 0 {
			if libc.strncmp(cstring(&header.identification[0]), "PWAD", 4) != 0 {
				I_Error("Wad file %s doesn't have IWAD or PWAD id\n", filename)
			}
		}
		header.numlumps = LONG(header.numlumps)
		header.infotableofs = LONG(header.infotableofs)
		length := int(header.numlumps) * size_of(filelump_t)
		allocated_fileinfo = libc.malloc(uint(length))
		fileinfo = ([^]filelump_t)(allocated_fileinfo)
		libc.fseek(handle, libc.long(header.infotableofs), .SET)
		libc.fread(fileinfo, uint(length), 1, handle)
		numlumps += int(header.numlumps)
	}

	lumpinfo = ([^]lumpinfo_t)(libc.realloc(lumpinfo, uint(numlumps * size_of(lumpinfo_t))))
	if lumpinfo == nil {
		I_Error("Couldn't realloc lumpinfo")
	}

	storehandle := reloadname != nil ? nil : handle

	for i := startlump; i < numlumps; i += 1 {
		lump_p := &lumpinfo[i]
		fi := &fileinfo[i - startlump]
		lump_p.handle = storehandle
		lump_p.position = int(LONG(fi.filepos))
		lump_p.size = int(LONG(fi.size))
		libc.strncpy(&lump_p.name[0], cstring(&fi.name[0]), 8)
	}

	if reloadname != nil {
		libc.fclose(handle)
	}
}

W_Reload :: proc() {
	header: wadinfo_t

	if reloadname == nil {
		return
	}

	handle := libc.fopen(reloadname, "rb")
	if handle == nil {
		I_Error("W_Reload: couldn't open %s", reloadname)
	}

	libc.fread(&header, size_of(wadinfo_t), 1, handle)
	lumpcount := int(LONG(header.numlumps))
	header.infotableofs = LONG(header.infotableofs)
	length := lumpcount * size_of(filelump_t)
	fileinfo := ([^]filelump_t)(libc.malloc(uint(length)))
	defer libc.free(fileinfo)
	libc.fseek(handle, libc.long(header.infotableofs), .SET)
	libc.fread(fileinfo, uint(length), 1, handle)

	for i := reloadlump; i < reloadlump + lumpcount; i += 1 {
		if lumpcache[i] != nil {
			Z_Free(lumpcache[i])
		}
		fi := &fileinfo[i - reloadlump]
		lumpinfo[i].position = int(LONG(fi.filepos))
		lumpinfo[i].size = int(LONG(fi.size))
	}

	libc.fclose(handle)
}

W_InitMultipleFiles :: proc(filenames_in: [^]cstring) {
	numlumps = 0
	lumpinfo = ([^]lumpinfo_t)(libc.malloc(1))

	for filenames := filenames_in; filenames[0] != nil; filenames = &filenames[1] {
		W_AddFile(filenames[0])
	}

	if numlumps == 0 {
		I_Error("W_InitFiles: no files found")
	}

	size := numlumps * size_of(rawptr)
	lumpcache = ([^]rawptr)(libc.malloc(uint(size)))
	if lumpcache == nil {
		I_Error("Couldn't allocate lumpcache")
	}
	libc.memset(lumpcache, 0, uint(size))
}

W_InitFile :: proc(filename: cstring) {
	names := [2]cstring{filename, nil}
	W_InitMultipleFiles(&names[0])
}

W_NumLumps :: proc() -> int {
	return numlumps
}

W_CheckNumForName :: proc(name: cstring) -> int {
	name8: [9]u8
	libc.memset(&name8[0], 0, 9)
	libc.strncpy(&name8[0], name, 8)
	name8[8] = 0
	strupr(&name8[0])

	v1 := (^u32)(&name8[0])^
	v2 := (^u32)(&name8[4])^

	for i := numlumps - 1; i >= 0; i -= 1 {
		lp := &lumpinfo[i]
		if (^u32)(&lp.name[0])^ == v1 && (^u32)(&lp.name[4])^ == v2 {
			return i
		}
	}

	return -1
}

W_GetNumForName :: proc(name: cstring) -> int {
	i := W_CheckNumForName(name)
	if i == -1 {
		I_Error("W_GetNumForName: %s not found!", name)
	}
	return i
}

W_LumpLength :: proc(lump: int) -> int {
	if lump >= numlumps {
		I_Error("W_LumpLength: %i >= numlumps", i32(lump))
	}
	return lumpinfo[lump].size
}

W_ReadLump :: proc(lump: int, dest: rawptr) {
	if lump >= numlumps {
		I_Error("W_ReadLump: %i >= numlumps", i32(lump))
	}

	l := &lumpinfo[lump]

	handle: ^libc.FILE
	if l.handle == nil {
		handle = libc.fopen(reloadname, "rb")
		if handle == nil {
			I_Error("W_ReadLump: couldn't open %s", reloadname)
		}
	} else {
		handle = l.handle
	}

	libc.fseek(handle, libc.long(l.position), .SET)
	c := int(libc.fread(dest, 1, uint(l.size), handle))

	if c < l.size {
		I_Error("W_ReadLump: only read %i of %i on lump %i", i32(c), i32(l.size), i32(lump))
	}

	if l.handle == nil {
		libc.fclose(handle)
	}
}

W_CacheLumpNum :: proc(lump: int, tag: int) -> rawptr {
	if uint(lump) >= uint(numlumps) {
		I_Error("W_CacheLumpNum: %i >= numlumps", i32(lump))
	}

	if lumpcache[lump] == nil {
		Z_Malloc(W_LumpLength(lump), tag, &lumpcache[lump])
		W_ReadLump(lump, lumpcache[lump])
	} else {
		Z_ChangeTag(lumpcache[lump], tag)
	}

	return lumpcache[lump]
}

W_CacheLumpName :: proc(name: cstring, tag: int) -> rawptr {
	return W_CacheLumpNum(W_GetNumForName(name), tag)
}

w_profile_info: [2500][10]int
profilecount: int

W_Profile :: proc() {
	name: [9]u8

	for i := 0; i < numlumps; i += 1 {
		ptr := lumpcache[i]
		ch: int
		if ptr == nil {
			ch = int(' ')
			continue
		} else {
			block := (^memblock_t)(uintptr(ptr) - size_of(memblock_t))
			if block.tag < PU_PURGELEVEL {
				ch = int('S')
			} else {
				ch = int('P')
			}
		}
		w_profile_info[i][profilecount] = ch
	}
	profilecount += 1

	f := libc.fopen("waddump.txt", "w")
	name[8] = 0

	for i := 0; i < numlumps; i += 1 {
		libc.memcpy(&name[0], &lumpinfo[i].name[0], 8)
		j := 0
		for ; j < 8; j += 1 {
			if name[j] == 0 {
				break
			}
		}
		for ; j < 8; j += 1 {
			name[j] = ' '
		}
		libc.fprintf(f, "%s ", &name[0])
		for j = 0; j < profilecount; j += 1 {
			libc.fprintf(f, "    %c", i32(w_profile_info[i][j]))
		}
		libc.fprintf(f, "\n")
	}
	libc.fclose(f)
}