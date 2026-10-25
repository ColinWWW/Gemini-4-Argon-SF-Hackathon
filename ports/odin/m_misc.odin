// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

M_DrawText :: proc(x_in: int, y: int, direct: boolean, str: cstring) -> int {
	x := x_in
	s := ([^]u8)(str)
	for s[0] != 0 {
		c := int(libc.toupper(i32(s[0]))) - int(HU_FONTSTART)
		s = &s[1]
		if c < 0 || c > HU_FONTSIZE {
			x += 4
			continue
		}

		w := int(SHORT(hu_font[c].width))
		if x + w > SCREENWIDTH {
			break
		}
		if direct {
			V_DrawPatchDirect(x, y, 0, hu_font[c])
		} else {
			V_DrawPatch(x, y, 0, hu_font[c])
		}
		x += w
	}

	return x
}

M_WriteFile :: proc(name: cstring, source: rawptr, length: int) -> boolean {
	f := libc.fopen(name, "wb")
	if f == nil {
		return false
	}
	count := int(libc.fwrite(source, 1, uint(length), f))
	libc.fclose(f)
	if count < length {
		return false
	}
	return true
}

M_ReadFile :: proc(name: cstring, buffer: ^[^]byte) -> int {
	f := libc.fopen(name, "rb")
	if f == nil {
		I_Error("Couldn't read file %s", name)
	}
	libc.fseek(f, 0, libc.SEEK_END)
	length := int(libc.ftell(f))
	libc.fseek(f, 0, libc.SEEK_SET)
	buf := ([^]byte)(Z_Malloc(length, PU_STATIC, nil))
	count := int(libc.fread(buf, 1, uint(length), f))
	libc.fclose(f)
	if count < length {
		I_Error("Couldn't read file %s", name)
	}
	buffer^ = buf
	return length
}

usemouse: int
usejoystick: int

default_t :: struct {
	name:         cstring,
	location:     ^int,
	str_location: ^cstring,
	defaultvalue: int,
	defaultstr:   cstring,
	is_string:    boolean,
}

defaults: [34]default_t
numdefaults: int
defaultfile: cstring

init_defaults_table :: proc() {
	defaults = [34]default_t{
		{"mouse_sensitivity", &mouseSensitivity, nil, 5, nil, false},
		{"sfx_volume", &snd_SfxVolume, nil, 8, nil, false},
		{"music_volume", &snd_MusicVolume, nil, 8, nil, false},
		{"show_messages", &showMessages, nil, 1, nil, false},
		{"key_right", &key_right, nil, KEY_RIGHTARROW, nil, false},
		{"key_left", &key_left, nil, KEY_LEFTARROW, nil, false},
		{"key_up", &key_up, nil, KEY_UPARROW, nil, false},
		{"key_down", &key_down, nil, KEY_DOWNARROW, nil, false},
		{"key_strafeleft", &key_strafeleft, nil, int(','), nil, false},
		{"key_straferight", &key_straferight, nil, int('.'), nil, false},
		{"key_fire", &key_fire, nil, KEY_RCTRL, nil, false},
		{"key_use", &key_use, nil, int(' '), nil, false},
		{"key_strafe", &key_strafe, nil, KEY_RALT, nil, false},
		{"key_speed", &key_speed, nil, KEY_RSHIFT, nil, false},
		{"sndserver", nil, &sndserver_filename, 0, "sndserver", true},
		{"mb_used", &mb_used, nil, 2, nil, false},
		{"use_mouse", &usemouse, nil, 1, nil, false},
		{"mouseb_fire", &mousebfire, nil, 0, nil, false},
		{"mouseb_strafe", &mousebstrafe, nil, 1, nil, false},
		{"mouseb_forward", &mousebforward, nil, 2, nil, false},
		{"use_joystick", &usejoystick, nil, 0, nil, false},
		{"joyb_fire", &joybfire, nil, 0, nil, false},
		{"joyb_strafe", &joybstrafe, nil, 1, nil, false},
		{"joyb_use", &joybuse, nil, 3, nil, false},
		{"joyb_speed", &joybspeed, nil, 2, nil, false},
		{"screenblocks", &screenblocks, nil, 9, nil, false},
		{"detaillevel", &detailLevel, nil, 0, nil, false},
		{"snd_channels", &numChannels, nil, 3, nil, false},
		{"usegamma", &usegamma, nil, 0, nil, false},
		{"chatmacro0", nil, &chat_macros[0], 0, HUSTR_CHATMACRO0, true},
		{"chatmacro1", nil, &chat_macros[1], 0, HUSTR_CHATMACRO1, true},
		{"chatmacro2", nil, &chat_macros[2], 0, HUSTR_CHATMACRO2, true},
		{"chatmacro3", nil, &chat_macros[3], 0, HUSTR_CHATMACRO3, true},
		{"chatmacro4", nil, &chat_macros[4], 0, HUSTR_CHATMACRO4, true},
	}
	numdefaults = len(defaults)
}

M_SaveDefaults :: proc() {
	f := libc.fopen(defaultfile, "w")
	if f == nil {
		return
	}

	for i := 0; i < numdefaults; i += 1 {
		if !defaults[i].is_string {
			v := defaults[i].location^
			libc.fprintf(f, "%s\t\t%i\n", defaults[i].name, i32(v))
		} else {
			libc.fprintf(f, "%s\t\t\"%s\"\n", defaults[i].name, defaults[i].str_location^)
		}
	}

	libc.fclose(f)
}

M_LoadDefaults :: proc() {
	def: [80]u8
	strparm: [100]u8

	init_defaults_table()

	for i := 0; i < numdefaults; i += 1 {
		if defaults[i].is_string {
			defaults[i].str_location^ = defaults[i].defaultstr
		} else {
			defaults[i].location^ = defaults[i].defaultvalue
		}
	}

	i := M_CheckParm("-config")
	if i != 0 && i < myargc - 1 {
		defaultfile = myargv[i + 1]
		libc.printf("\tdefault file: %s\n", defaultfile)
	} else {
		defaultfile = cstring(&basedefault[0])
	}

	f := libc.fopen(defaultfile, "r")
	if f != nil {
		for libc.feof(f) == 0 {
			isstring := false
			parm: i32 = 0
			newstring: cstring = nil
			if libc.fscanf(f, "%79s %[^\n]\n", &def[0], &strparm[0]) == 2 {
				if strparm[0] == '"' {
					isstring = true
					length := int(libc.strlen(cstring(&strparm[0])))
					ns := ([^]u8)(libc.malloc(uint(length)))
					strparm[length - 1] = 0
					libc.strcpy(ns, cstring(&strparm[1]))
					newstring = cstring(ns)
				} else if strparm[0] == '0' && strparm[1] == 'x' {
					libc.sscanf(cstring(&strparm[2]), "%x", &parm)
				} else {
					libc.sscanf(cstring(&strparm[0]), "%i", &parm)
				}
				for j := 0; j < numdefaults; j += 1 {
					if libc.strcmp(cstring(&def[0]), defaults[j].name) == 0 {
						if !isstring && !defaults[j].is_string {
							defaults[j].location^ = int(parm)
						} else if isstring && defaults[j].is_string {
							defaults[j].str_location^ = newstring
						}
						break
					}
				}
			}
		}
		libc.fclose(f)
	}
}

pcx_t :: struct #packed {
	manufacturer:   u8,
	version:        u8,
	encoding:       u8,
	bits_per_pixel: u8,
	xmin:           u16,
	ymin:           u16,
	xmax:           u16,
	ymax:           u16,
	hres:           u16,
	vres:           u16,
	palette:        [48]u8,
	reserved:       u8,
	color_planes:   u8,
	bytes_per_line: u16,
	palette_type:   u16,
	filler:         [58]u8,
	data:           u8,
}

WritePCXfile :: proc(filename: cstring, data_in: [^]byte, width: int, height: int, palette_in: [^]byte) {
	data := data_in
	palette := palette_in
	pcx := (^pcx_t)(Z_Malloc(width * height * 2 + 1000, PU_STATIC, nil))

	pcx.manufacturer = 0x0a
	pcx.version = 5
	pcx.encoding = 1
	pcx.bits_per_pixel = 8
	pcx.xmin = 0
	pcx.ymin = 0
	pcx.xmax = u16(SHORT(i16(width - 1)))
	pcx.ymax = u16(SHORT(i16(height - 1)))
	pcx.hres = u16(SHORT(i16(width)))
	pcx.vres = u16(SHORT(i16(height)))
	libc.memset(&pcx.palette[0], 0, 48)
	pcx.color_planes = 1
	pcx.bytes_per_line = u16(SHORT(i16(width)))
	pcx.palette_type = u16(SHORT(2))
	libc.memset(&pcx.filler[0], 0, 58)

	pack := ([^]byte)(&pcx.data)

	for i := 0; i < width * height; i += 1 {
		if (data[0] & 0xc0) != 0xc0 {
			pack[0] = data[0]
			pack = &pack[1]
			data = &data[1]
		} else {
			pack[0] = 0xc1
			pack = &pack[1]
			pack[0] = data[0]
			pack = &pack[1]
			data = &data[1]
		}
	}

	pack[0] = 0x0c
	pack = &pack[1]
	for i := 0; i < 768; i += 1 {
		pack[0] = palette[0]
		pack = &pack[1]
		palette = &palette[1]
	}

	length := int(uintptr(pack) - uintptr(pcx))
	M_WriteFile(filename, pcx, length)
	Z_Free(pcx)
}

M_ScreenShot :: proc() {
	lbmname: [12]u8
	linear := screens[2]
	I_ReadScreen(linear)

	libc.strcpy(&lbmname[0], "DOOM00.pcx")

	i := 0
	for ; i <= 99; i += 1 {
		lbmname[4] = u8(i / 10 + '0')
		lbmname[5] = u8(i % 10 + '0')
		if c_access(cstring(&lbmname[0]), 0) == -1 {
			break
		}
	}
	if i == 100 {
		I_Error("M_ScreenShot: Couldn't create a PCX")
	}

	WritePCXfile(cstring(&lbmname[0]), linear, SCREENWIDTH, SCREENHEIGHT, ([^]byte)(W_CacheLumpName("PLAYPAL", PU_CACHE)))
	players[consoleplayer].message = "screen shot"
}
