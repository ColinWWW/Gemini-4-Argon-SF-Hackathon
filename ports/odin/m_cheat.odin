// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

cht_firsttime: int = 1
cheat_xlate_table: [256]u8

cht_CheckCheat :: proc(cht: ^cheatseq_t, key: u8) -> int {
	rc := 0

	if cht_firsttime != 0 {
		cht_firsttime = 0
		for i := 0; i < 256; i += 1 {
			a := u8(i)
			cheat_xlate_table[i] = ((a & 1) << 7) | ((a & 2) << 5) | (a & 4) | ((a & 8) << 1) | ((a & 16) >> 1) | (a & 32) | ((a & 64) >> 5) | ((a & 128) >> 7)
		}
	}

	if cht.p == nil {
		cht.p = cht.sequence
	}

	if cht.p[0] == 0 {
		cht.p[0] = key
		cht.p = &cht.p[1]
	} else if cheat_xlate_table[key] == cht.p[0] {
		cht.p = &cht.p[1]
	} else {
		cht.p = cht.sequence
	}

	if cht.p[0] == 1 {
		cht.p = &cht.p[1]
	} else if cht.p[0] == 0xff {
		cht.p = cht.sequence
		rc = 1
	}

	return rc
}

cht_GetParam :: proc(cht: ^cheatseq_t, buffer: [^]u8) {
	p := cht.sequence
	for {
		v := p[0]
		p = &p[1]
		if v == 1 {
			break
		}
	}

	b := buffer
	for {
		c := p[0]
		b[0] = c
		b = &b[1]
		p[0] = 0
		p = &p[1]
		if c == 0 || p[0] == 0xff {
			break
		}
	}

	if p[0] == 0xff {
		b[0] = 0
	}
}
