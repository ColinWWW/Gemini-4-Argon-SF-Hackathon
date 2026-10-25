// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

wipe_go: boolean = false
wipe_scr_start: [^]byte
wipe_scr_end: [^]byte
wipe_scr: [^]byte
wipe_y: [^]int

wipe_shittyColMajorXform :: proc(array: [^]i16, width: int, height: int) {
	dest := ([^]i16)(Z_Malloc(width * height * 2, PU_STATIC, nil))

	for y := 0; y < height; y += 1 {
		for x := 0; x < width; x += 1 {
			dest[x * height + y] = array[y * width + x]
		}
	}

	libc.memcpy(array, dest, uint(width * height * 2))
	Z_Free(dest)
}

wipe_initColorXForm :: proc(width: int, height: int, ticks: int) -> int {
	libc.memcpy(wipe_scr, wipe_scr_start, uint(width * height))
	return 0
}

wipe_doColorXForm :: proc(width: int, height: int, ticks: int) -> int {
	changed := false
	w := wipe_scr
	e := wipe_scr_end
	end_ptr := &wipe_scr[width * height]

	for w != end_ptr {
		if w[0] != e[0] {
			if w[0] > e[0] {
				newval := int(w[0]) - ticks
				if newval < int(e[0]) {
					w[0] = e[0]
				} else {
					w[0] = byte(newval)
				}
				changed = true
			} else if w[0] < e[0] {
				newval := int(w[0]) + ticks
				if newval > int(e[0]) {
					w[0] = e[0]
				} else {
					w[0] = byte(newval)
				}
				changed = true
			}
		}
		w = &w[1]
		e = &e[1]
	}

	return !changed ? 1 : 0
}

wipe_exitColorXForm :: proc(width: int, height: int, ticks: int) -> int {
	return 0
}

wipe_initMelt :: proc(width: int, height: int, ticks: int) -> int {
	libc.memcpy(wipe_scr, wipe_scr_start, uint(width * height))

	wipe_shittyColMajorXform(([^]i16)(wipe_scr_start), width / 2, height)
	wipe_shittyColMajorXform(([^]i16)(wipe_scr_end), width / 2, height)

	wipe_y = ([^]int)(Z_Malloc(width * size_of(int), PU_STATIC, nil))
	wipe_y[0] = -(M_Random() % 16)
	for i := 1; i < width; i += 1 {
		r := (M_Random() % 3) - 1
		wipe_y[i] = wipe_y[i - 1] + r
		if wipe_y[i] > 0 {
			wipe_y[i] = 0
		} else if wipe_y[i] == -16 {
			wipe_y[i] = -15
		}
	}

	return 0
}

wipe_doMelt :: proc(width: int, height: int, ticks: int) -> int {
	done := true
	w := width / 2
	t := ticks

	for t > 0 {
		t -= 1
		for i := 0; i < w; i += 1 {
			if wipe_y[i] < 0 {
				wipe_y[i] += 1
				done = false
			} else if wipe_y[i] < height {
				dy := (wipe_y[i] < 16) ? wipe_y[i] + 1 : 8
				if wipe_y[i] + dy >= height {
					dy = height - wipe_y[i]
				}
				s := &(([^]i16)(wipe_scr_end))[i * height + wipe_y[i]]
				d := &(([^]i16)(wipe_scr))[wipe_y[i] * w + i]
				idx := 0
				for j := dy; j != 0; j -= 1 {
					d[idx] = s[0]
					s = &s[1]
					idx += w
				}
				wipe_y[i] += dy
				s = &(([^]i16)(wipe_scr_start))[i * height]
				d = &(([^]i16)(wipe_scr))[wipe_y[i] * w + i]
				idx = 0
				for j := height - wipe_y[i]; j != 0; j -= 1 {
					d[idx] = s[0]
					s = &s[1]
					idx += w
				}
				done = false
			}
		}
	}

	return done ? 1 : 0
}

wipe_exitMelt :: proc(width: int, height: int, ticks: int) -> int {
	Z_Free(wipe_y)
	return 0
}

wipe_StartScreen :: proc(x: int, y: int, width: int, height: int) -> int {
	wipe_scr_start = screens[2]
	I_ReadScreen(wipe_scr_start)
	return 0
}

wipe_EndScreen :: proc(x: int, y: int, width: int, height: int) -> int {
	wipe_scr_end = screens[3]
	I_ReadScreen(wipe_scr_end)
	V_DrawBlock(x, y, 0, width, height, wipe_scr_start)
	return 0
}

wipe_ScreenWipe :: proc(wipeno: int, x: int, y: int, width: int, height: int, ticks: int) -> int {
	wipes := [6]#type proc(int, int, int) -> int{
		wipe_initColorXForm, wipe_doColorXForm, wipe_exitColorXForm,
		wipe_initMelt, wipe_doMelt, wipe_exitMelt,
	}

	if !wipe_go {
		wipe_go = true
		wipe_scr = screens[0]
		wipes[wipeno * 3](width, height, ticks)
	}

	V_MarkRect(0, 0, width, height)
	rc := wipes[wipeno * 3 + 1](width, height, ticks)

	if rc != 0 {
		wipe_go = false
		wipes[wipeno * 3 + 2](width, height, ticks)
	}

	return !wipe_go ? 1 : 0
}
