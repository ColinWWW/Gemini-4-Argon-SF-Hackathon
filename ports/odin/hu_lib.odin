// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

HUlib_lastautomapactive: boolean = true

HUlib_init :: proc() {
}

HUlib_clearTextLine :: proc(t: ^hu_textline_t) {
	t.len = 0
	t.l[0] = 0
	t.needsupdate = 1
}

HUlib_initTextLine :: proc(t: ^hu_textline_t, x: int, y: int, f: [^]^patch_t, sc: int) {
	t.x = x
	t.y = y
	t.f = f
	t.sc = sc
	HUlib_clearTextLine(t)
}

HUlib_addCharToTextLine :: proc(t: ^hu_textline_t, ch: u8) -> boolean {
	if t.len == HU_MAXLINELENGTH {
		return false
	} else {
		t.l[t.len] = ch
		t.len += 1
		t.l[t.len] = 0
		t.needsupdate = 4
		return true
	}
}

HUlib_delCharFromTextLine :: proc(t: ^hu_textline_t) -> boolean {
	if t.len == 0 {
		return false
	} else {
		t.len -= 1
		t.l[t.len] = 0
		t.needsupdate = 4
		return true
	}
}

HUlib_drawTextLine :: proc(l: ^hu_textline_t, drawcursor: boolean) {
	x := l.x
	for i := 0; i < l.len; i += 1 {
		c := u8(libc.toupper(i32(l.l[i])))
		if c != ' ' && int(c) >= l.sc && c <= '_' {
			w := int(SHORT(l.f[int(c) - l.sc].width))
			if x + w > SCREENWIDTH {
				break
			}
			V_DrawPatchDirect(x, l.y, HU_FG, l.f[int(c) - l.sc])
			x += w
		} else {
			x += 4
			if x >= SCREENWIDTH {
				break
			}
		}
	}

	if drawcursor && x + int(SHORT(l.f[int('_') - l.sc].width)) <= SCREENWIDTH {
		V_DrawPatchDirect(x, l.y, HU_FG, l.f[int('_') - l.sc])
	}
}

HUlib_eraseTextLine :: proc(l: ^hu_textline_t) {
	if !automapactive && viewwindowx != 0 && l.needsupdate != 0 {
		lh := int(SHORT(l.f[0].height)) + 1
		y := l.y
		yoffset := y * SCREENWIDTH
		for ; y < l.y + lh; y += 1 {
			if y < viewwindowy || y >= viewwindowy + viewheight {
				R_VideoErase(uint(yoffset), SCREENWIDTH)
			} else {
				R_VideoErase(uint(yoffset), viewwindowx)
				R_VideoErase(uint(yoffset + viewwindowx + viewwidth), viewwindowx)
			}
			yoffset += SCREENWIDTH
		}
	}

	HUlib_lastautomapactive = automapactive
	if l.needsupdate != 0 {
		l.needsupdate -= 1
	}
}

HUlib_initSText :: proc(s: ^hu_stext_t, x: int, y: int, h: int, font: [^]^patch_t, startchar: int, on: ^boolean) {
	s.h = h
	s.on = on
	s.laston = true
	s.cl = 0
	for i := 0; i < h; i += 1 {
		HUlib_initTextLine(&s.l[i], x, y - i * (int(SHORT(font[0].height)) + 1), font, startchar)
	}
}

HUlib_addLineToSText :: proc(s: ^hu_stext_t) {
	s.cl += 1
	if s.cl == s.h {
		s.cl = 0
	}
	HUlib_clearTextLine(&s.l[s.cl])

	for i := 0; i < s.h; i += 1 {
		s.l[i].needsupdate = 4
	}
}

HUlib_addMessageToSText :: proc(s: ^hu_stext_t, prefix: cstring, msg: cstring) {
	HUlib_addLineToSText(s)
	if prefix != nil {
		p := ([^]u8)(prefix)
		for p[0] != 0 {
			HUlib_addCharToTextLine(&s.l[s.cl], p[0])
			p = &p[1]
		}
	}

	m := ([^]u8)(msg)
	for m[0] != 0 {
		HUlib_addCharToTextLine(&s.l[s.cl], m[0])
		m = &m[1]
	}
}

HUlib_drawSText :: proc(s: ^hu_stext_t) {
	if !s.on^ {
		return
	}

	for i := 0; i < s.h; i += 1 {
		idx := s.cl - i
		if idx < 0 {
			idx += s.h
		}
		l := &s.l[idx]
		HUlib_drawTextLine(l, false)
	}
}

HUlib_eraseSText :: proc(s: ^hu_stext_t) {
	for i := 0; i < s.h; i += 1 {
		if s.laston && !s.on^ {
			s.l[i].needsupdate = 4
		}
		HUlib_eraseTextLine(&s.l[i])
	}
	s.laston = s.on^
}

HUlib_initIText :: proc(it: ^hu_itext_t, x: int, y: int, font: [^]^patch_t, startchar: int, on: ^boolean) {
	it.lm = 0
	it.on = on
	it.laston = true
	HUlib_initTextLine(&it.l, x, y, font, startchar)
}

HUlib_delCharFromIText :: proc(it: ^hu_itext_t) {
	if it.l.len != it.lm {
		HUlib_delCharFromTextLine(&it.l)
	}
}

HUlib_eraseLineFromIText :: proc(it: ^hu_itext_t) {
	for it.lm != it.l.len {
		HUlib_delCharFromTextLine(&it.l)
	}
}

HUlib_resetIText :: proc(it: ^hu_itext_t) {
	it.lm = 0
	HUlib_clearTextLine(&it.l)
}

HUlib_addPrefixToIText :: proc(it: ^hu_itext_t, str: cstring) {
	p := ([^]u8)(str)
	for p[0] != 0 {
		HUlib_addCharToTextLine(&it.l, p[0])
		p = &p[1]
	}
	it.lm = it.l.len
}

HUlib_keyInIText :: proc(it: ^hu_itext_t, ch: u8) -> boolean {
	if ch >= ' ' && ch <= '_' {
		HUlib_addCharToTextLine(&it.l, ch)
	} else if ch == KEY_BACKSPACE {
		HUlib_delCharFromIText(it)
	} else if ch != KEY_ENTER {
		return false
	}
	return true
}

HUlib_drawIText :: proc(it: ^hu_itext_t) {
	if !it.on^ {
		return
	}
	HUlib_drawTextLine(&it.l, true)
}

HUlib_eraseIText :: proc(it: ^hu_itext_t) {
	if it.laston && !it.on^ {
		it.l.needsupdate = 4
	}
	HUlib_eraseTextLine(&it.l)
	it.laston = it.on^
}
