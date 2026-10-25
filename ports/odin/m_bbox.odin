// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

M_ClearBox :: proc(box: [^]fixed_t) {
	box[BOXTOP] = fixed_t(MININT)
	box[BOXRIGHT] = fixed_t(MININT)
	box[BOXBOTTOM] = fixed_t(MAXINT)
	box[BOXLEFT] = fixed_t(MAXINT)
}

M_AddToBox :: proc(box: [^]fixed_t, x: fixed_t, y: fixed_t) {
	if x < box[BOXLEFT] {
		box[BOXLEFT] = x
	} else if x > box[BOXRIGHT] {
		box[BOXRIGHT] = x
	}
	if y < box[BOXBOTTOM] {
		box[BOXBOTTOM] = y
	} else if y > box[BOXTOP] {
		box[BOXTOP] = y
	}
}
