// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

REDS        :: 256 - 5*16
REDRANGE    :: 16
BLUES       :: 256 - 4*16 + 8
BLUERANGE   :: 8
GREENS      :: 7 * 16
GREENRANGE  :: 16
GRAYS       :: 6 * 16
GRAYSRANGE  :: 16
BROWNS      :: 4 * 16
BROWNRANGE  :: 16
YELLOWS     :: 256 - 32 + 7
YELLOWRANGE :: 1
BLACK       :: 0
WHITE       :: 256 - 47

BACKGROUND       :: BLACK
YOURCOLORS       :: WHITE
YOURRANGE        :: 0
WALLCOLORS       :: REDS
WALLRANGE        :: REDRANGE
TSWALLCOLORS     :: GRAYS
TSWALLRANGE      :: GRAYSRANGE
FDWALLCOLORS     :: BROWNS
FDWALLRANGE      :: BROWNRANGE
CDWALLCOLORS     :: YELLOWS
CDWALLRANGE      :: YELLOWRANGE
THINGCOLORS      :: GREENS
THINGRANGE       :: GREENRANGE
SECRETWALLCOLORS :: WALLCOLORS
SECRETWALLRANGE  :: WALLRANGE
GRIDCOLORS       :: GRAYS + GRAYSRANGE/2
GRIDRANGE        :: 0
XHAIRCOLORS      :: GRAYS

FB :: 0

AM_PANDOWNKEY   :: KEY_DOWNARROW
AM_PANUPKEY     :: KEY_UPARROW
AM_PANRIGHTKEY  :: KEY_RIGHTARROW
AM_PANLEFTKEY   :: KEY_LEFTARROW
AM_ZOOMINKEY    :: int('=')
AM_ZOOMOUTKEY   :: int('-')
AM_STARTKEY     :: KEY_TAB
AM_ENDKEY       :: KEY_TAB
AM_GOBIGKEY     :: int('0')
AM_FOLLOWKEY    :: int('f')
AM_GRIDKEY      :: int('g')
AM_MARKKEY      :: int('m')
AM_CLEARMARKKEY :: int('c')

AM_NUMMARKPOINTS :: 10

INITSCALEMTOF :: (FRACUNIT * 2) / 10
F_PANINC      :: 4
M_ZOOMIN      :: (102 * FRACUNIT) / 100
M_ZOOMOUT     :: (100 * FRACUNIT) / 102

FTOM :: #force_inline proc(x: int) -> fixed_t {
	return FixedMul(fixed_t(i32(x) << 16), scale_ftom)
}

MTOF :: #force_inline proc(x: fixed_t) -> int {
	return int(FixedMul(x, scale_mtof) >> 16)
}

CXMTOF :: #force_inline proc(x: fixed_t) -> int {
	return f_x + MTOF(x - m_x)
}

CYMTOF :: #force_inline proc(y: fixed_t) -> int {
	return f_y + (f_h - MTOF(y - m_y))
}

LINE_NEVERSEE :: ML_DONTDRAW

fpoint_t :: struct {
	x, y: int,
}

fline_t :: struct {
	a, b: fpoint_t,
}

mpoint_t :: struct {
	x, y: fixed_t,
}

mline_t :: struct {
	a, b: mpoint_t,
}

islope_t :: struct {
	slp, islp: fixed_t,
}

AM_R :: (8 * PLAYERRADIUS) / 7

player_arrow := [7]mline_t{
	{ { -AM_R + AM_R/8, 0 }, { AM_R, 0 } },
	{ { AM_R, 0 }, { AM_R - AM_R/2, AM_R/4 } },
	{ { AM_R, 0 }, { AM_R - AM_R/2, -AM_R/4 } },
	{ { -AM_R + AM_R/8, 0 }, { -AM_R - AM_R/8, AM_R/4 } },
	{ { -AM_R + AM_R/8, 0 }, { -AM_R - AM_R/8, -AM_R/4 } },
	{ { -AM_R + 3*AM_R/8, 0 }, { -AM_R + AM_R/8, AM_R/4 } },
	{ { -AM_R + 3*AM_R/8, 0 }, { -AM_R + AM_R/8, -AM_R/4 } },
}
NUMPLYRLINES :: len(player_arrow)

cheat_player_arrow := [16]mline_t{
	{ { -AM_R + AM_R/8, 0 }, { AM_R, 0 } },
	{ { AM_R, 0 }, { AM_R - AM_R/2, AM_R/6 } },
	{ { AM_R, 0 }, { AM_R - AM_R/2, -AM_R/6 } },
	{ { -AM_R + AM_R/8, 0 }, { -AM_R - AM_R/8, AM_R/6 } },
	{ { -AM_R + AM_R/8, 0 }, { -AM_R - AM_R/8, -AM_R/6 } },
	{ { -AM_R + 3*AM_R/8, 0 }, { -AM_R + AM_R/8, AM_R/6 } },
	{ { -AM_R + 3*AM_R/8, 0 }, { -AM_R + AM_R/8, -AM_R/6 } },
	{ { -AM_R/2, 0 }, { -AM_R/2, -AM_R/6 } },
	{ { -AM_R/2, -AM_R/6 }, { -AM_R/2 + AM_R/6, -AM_R/6 } },
	{ { -AM_R/2 + AM_R/6, -AM_R/6 }, { -AM_R/2 + AM_R/6, AM_R/4 } },
	{ { -AM_R/6, 0 }, { -AM_R/6, -AM_R/6 } },
	{ { -AM_R/6, -AM_R/6 }, { 0, -AM_R/6 } },
	{ { 0, -AM_R/6 }, { 0, AM_R/4 } },
	{ { AM_R/6, AM_R/4 }, { AM_R/6, -AM_R/7 } },
	{ { AM_R/6, -AM_R/7 }, { AM_R/6 + AM_R/32, -AM_R/7 - AM_R/32 } },
	{ { AM_R/6 + AM_R/32, -AM_R/7 - AM_R/32 }, { AM_R/6 + AM_R/10, -AM_R/7 } },
}
NUMCHEATPLYRLINES :: len(cheat_player_arrow)

triangle_guy := [3]mline_t{
	{ { (-867 * FRACUNIT) / 1000, -FRACUNIT / 2 }, { (867 * FRACUNIT) / 1000, -FRACUNIT / 2 } },
	{ { (867 * FRACUNIT) / 1000, -FRACUNIT / 2 }, { 0, FRACUNIT } },
	{ { 0, FRACUNIT }, { (-867 * FRACUNIT) / 1000, -FRACUNIT / 2 } },
}
NUMTRIANGLEGUYLINES :: len(triangle_guy)

thintriangle_guy := [3]mline_t{
	{ { -FRACUNIT / 2, (-7 * FRACUNIT) / 10 }, { FRACUNIT, 0 } },
	{ { FRACUNIT, 0 }, { -FRACUNIT / 2, (7 * FRACUNIT) / 10 } },
	{ { -FRACUNIT / 2, (7 * FRACUNIT) / 10 }, { -FRACUNIT / 2, (-7 * FRACUNIT) / 10 } },
}
NUMTHINTRIANGLEGUYLINES :: len(thintriangle_guy)

cheating: int = 0
grid: int = 0
leveljuststarted: int = 1
automapactive: boolean = false
finit_width: int = SCREENWIDTH
finit_height: int = SCREENHEIGHT - 32

f_x: int
f_y: int
f_w: int
f_h: int

lightlev: int
fb: [^]byte
amclock: int

m_paninc: mpoint_t
mtof_zoommul: fixed_t
ftom_zoommul: fixed_t

m_x, m_y: fixed_t
m_x2, m_y2: fixed_t
m_w: fixed_t
m_h: fixed_t

min_x: fixed_t
min_y: fixed_t
max_x: fixed_t
max_y: fixed_t
max_w: fixed_t
max_h: fixed_t

min_w: fixed_t
min_h: fixed_t

min_scale_mtof: fixed_t
max_scale_mtof: fixed_t

old_m_w, old_m_h: fixed_t
old_m_x, old_m_y: fixed_t

f_oldloc: mpoint_t

scale_mtof: fixed_t = INITSCALEMTOF
scale_ftom: fixed_t

plr: ^player_t

marknums: [10]^patch_t
markpoints: [AM_NUMMARKPOINTS]mpoint_t
markpointnum: int = 0

followplayer: int = 1

cheat_amap_seq := [5]u8{ 0xb2, 0x26, 0x26, 0x2e, 0xff }
cheat_amap := cheatseq_t{ &cheat_amap_seq[0], nil }

stopped: boolean = true

am_lastlevel: int = -1
am_lastepisode: int = -1
am_cheatstate: int = 0
am_bigstate: int = 0
am_buffer: [20]u8
am_nexttic: int = 0
am_litelevels := [8]int{ 0, 4, 7, 10, 12, 14, 15, 15 }
am_litelevelscnt: int = 0
am_fuck: int = 0
am_their_colors := [4]int{ GREENS, GRAYS, BROWNS, REDS }

AM_getIslope :: proc(ml: ^mline_t, is: ^islope_t) {
	dy := ml.a.y - ml.b.y
	dx := ml.b.x - ml.a.x
	if dy == 0 {
		is.islp = dx < 0 ? -MAXINT : MAXINT
	} else {
		is.islp = FixedDiv(dx, dy)
	}
	if dx == 0 {
		is.slp = dy < 0 ? -MAXINT : MAXINT
	} else {
		is.slp = FixedDiv(dy, dx)
	}
}

AM_activateNewScale :: proc() {
	m_x += m_w / 2
	m_y += m_h / 2
	m_w = FTOM(f_w)
	m_h = FTOM(f_h)
	m_x -= m_w / 2
	m_y -= m_h / 2
	m_x2 = m_x + m_w
	m_y2 = m_y + m_h
}

AM_saveScaleAndLoc :: proc() {
	old_m_x = m_x
	old_m_y = m_y
	old_m_w = m_w
	old_m_h = m_h
}

AM_restoreScaleAndLoc :: proc() {
	m_w = old_m_w
	m_h = old_m_h
	if followplayer == 0 {
		m_x = old_m_x
		m_y = old_m_y
	} else {
		m_x = plr.mo.x - m_w / 2
		m_y = plr.mo.y - m_h / 2
	}
	m_x2 = m_x + m_w
	m_y2 = m_y + m_h

	scale_mtof = FixedDiv(fixed_t(i32(f_w) << FRACBITS), m_w)
	scale_ftom = FixedDiv(FRACUNIT, scale_mtof)
}

AM_addMark :: proc() {
	markpoints[markpointnum].x = m_x + m_w / 2
	markpoints[markpointnum].y = m_y + m_h / 2
	markpointnum = (markpointnum + 1) % AM_NUMMARKPOINTS
}

AM_findMinMaxBoundaries :: proc() {
	min_x = MAXINT
	min_y = MAXINT
	max_x = -MAXINT
	max_y = -MAXINT

	for i := 0; i < numvertexes; i += 1 {
		if vertexes[i].x < min_x {
			min_x = vertexes[i].x
		} else if vertexes[i].x > max_x {
			max_x = vertexes[i].x
		}

		if vertexes[i].y < min_y {
			min_y = vertexes[i].y
		} else if vertexes[i].y > max_y {
			max_y = vertexes[i].y
		}
	}

	max_w = max_x - min_x
	max_h = max_y - min_y

	min_w = 2 * PLAYERRADIUS
	min_h = 2 * PLAYERRADIUS

	a := FixedDiv(fixed_t(i32(f_w) << FRACBITS), max_w)
	b := FixedDiv(fixed_t(i32(f_h) << FRACBITS), max_h)

	min_scale_mtof = a < b ? a : b
	max_scale_mtof = FixedDiv(fixed_t(i32(f_h) << FRACBITS), 2 * PLAYERRADIUS)
}

AM_changeWindowLoc :: proc() {
	if m_paninc.x != 0 || m_paninc.y != 0 {
		followplayer = 0
		f_oldloc.x = MAXINT
	}

	m_x += m_paninc.x
	m_y += m_paninc.y

	if m_x + m_w/2 > max_x {
		m_x = max_x - m_w/2
	} else if m_x + m_w/2 < min_x {
		m_x = min_x - m_w/2
	}

	if m_y + m_h/2 > max_y {
		m_y = max_y - m_h/2
	} else if m_y + m_h/2 < min_y {
		m_y = min_y - m_h/2
	}

	m_x2 = m_x + m_w
	m_y2 = m_y + m_h
}

AM_initVariables :: proc() {
	st_notify := event_t{ type = .ev_keyup, data1 = AM_MSGENTERED }

	automapactive = true
	fb = screens[0]

	f_oldloc.x = MAXINT
	amclock = 0
	lightlev = 0

	m_paninc.x = 0
	m_paninc.y = 0
	ftom_zoommul = FRACUNIT
	mtof_zoommul = FRACUNIT

	m_w = FTOM(f_w)
	m_h = FTOM(f_h)

	pnum := consoleplayer
	if !playeringame[pnum] {
		for pnum = 0; pnum < MAXPLAYERS; pnum += 1 {
			if playeringame[pnum] {
				break
			}
		}
	}

	plr = &players[pnum]
	m_x = plr.mo.x - m_w/2
	m_y = plr.mo.y - m_h/2
	AM_changeWindowLoc()

	old_m_x = m_x
	old_m_y = m_y
	old_m_w = m_w
	old_m_h = m_h

	ST_Responder(&st_notify)
}

AM_loadPics :: proc() {
	namebuf: [9]u8
	for i := 0; i < 10; i += 1 {
		libc.sprintf(&namebuf[0], "AMMNUM%d", i32(i))
		marknums[i] = (^patch_t)(W_CacheLumpName(cstring(&namebuf[0]), PU_STATIC))
	}
}

AM_unloadPics :: proc() {
	for i := 0; i < 10; i += 1 {
		Z_ChangeTag(marknums[i], PU_CACHE)
	}
}

AM_clearMarks :: proc() {
	for i := 0; i < AM_NUMMARKPOINTS; i += 1 {
		markpoints[i].x = -1
	}
	markpointnum = 0
}

AM_LevelInit :: proc() {
	leveljuststarted = 0

	f_x = 0
	f_y = 0
	f_w = finit_width
	f_h = finit_height

	AM_clearMarks()

	AM_findMinMaxBoundaries()
	scale_mtof = FixedDiv(min_scale_mtof, (7 * FRACUNIT) / 10)
	if scale_mtof > max_scale_mtof {
		scale_mtof = min_scale_mtof
	}
	scale_ftom = FixedDiv(FRACUNIT, scale_mtof)
}

AM_Stop :: proc() {
	st_notify := event_t{ type = evtype_t(0), data1 = int(evtype_t.ev_keyup), data2 = AM_MSGEXITED }
	AM_unloadPics()
	automapactive = false
	ST_Responder(&st_notify)
	stopped = true
}

AM_Start :: proc() {
	if !stopped {
		AM_Stop()
	}
	stopped = false
	if am_lastlevel != gamemap || am_lastepisode != gameepisode {
		AM_LevelInit()
		am_lastlevel = gamemap
		am_lastepisode = gameepisode
	}
	AM_initVariables()
	AM_loadPics()
}

AM_minOutWindowScale :: proc() {
	scale_mtof = min_scale_mtof
	scale_ftom = FixedDiv(FRACUNIT, scale_mtof)
	AM_activateNewScale()
}

AM_maxOutWindowScale :: proc() {
	scale_mtof = max_scale_mtof
	scale_ftom = FixedDiv(FRACUNIT, scale_mtof)
	AM_activateNewScale()
}

AM_Responder :: proc(ev: ^event_t) -> boolean {
	rc: boolean = false

	if !automapactive {
		if ev.type == .ev_keydown && ev.data1 == AM_STARTKEY {
			AM_Start()
			viewactive = false
			rc = true
		}
	} else if ev.type == .ev_keydown {
		rc = true
		switch ev.data1 {
		case AM_PANRIGHTKEY:
			if followplayer == 0 {
				m_paninc.x = FTOM(F_PANINC)
			} else {
				rc = false
			}
		case AM_PANLEFTKEY:
			if followplayer == 0 {
				m_paninc.x = -FTOM(F_PANINC)
			} else {
				rc = false
			}
		case AM_PANUPKEY:
			if followplayer == 0 {
				m_paninc.y = FTOM(F_PANINC)
			} else {
				rc = false
			}
		case AM_PANDOWNKEY:
			if followplayer == 0 {
				m_paninc.y = -FTOM(F_PANINC)
			} else {
				rc = false
			}
		case AM_ZOOMOUTKEY:
			mtof_zoommul = M_ZOOMOUT
			ftom_zoommul = M_ZOOMIN
		case AM_ZOOMINKEY:
			mtof_zoommul = M_ZOOMIN
			ftom_zoommul = M_ZOOMOUT
		case AM_ENDKEY:
			am_bigstate = 0
			viewactive = true
			AM_Stop()
		case AM_GOBIGKEY:
			am_bigstate = am_bigstate == 0 ? 1 : 0
			if am_bigstate != 0 {
				AM_saveScaleAndLoc()
				AM_minOutWindowScale()
			} else {
				AM_restoreScaleAndLoc()
			}
		case AM_FOLLOWKEY:
			followplayer = followplayer == 0 ? 1 : 0
			f_oldloc.x = MAXINT
			plr.message = followplayer != 0 ? AMSTR_FOLLOWON : AMSTR_FOLLOWOFF
		case AM_GRIDKEY:
			grid = grid == 0 ? 1 : 0
			plr.message = grid != 0 ? AMSTR_GRIDON : AMSTR_GRIDOFF
		case AM_MARKKEY:
			libc.sprintf(&am_buffer[0], "%s %d", AMSTR_MARKEDSPOT, i32(markpointnum))
			plr.message = cstring(&am_buffer[0])
			AM_addMark()
		case AM_CLEARMARKKEY:
			AM_clearMarks()
			plr.message = AMSTR_MARKSCLEARED
		case:
			am_cheatstate = 0
			rc = false
		}
		if deathmatch == 0 && cht_CheckCheat(&cheat_amap, u8(ev.data1)) != 0 {
			rc = false
			cheating = (cheating + 1) % 3
		}
	} else if ev.type == .ev_keyup {
		rc = false
		switch ev.data1 {
		case AM_PANRIGHTKEY:
			if followplayer == 0 { m_paninc.x = 0 }
		case AM_PANLEFTKEY:
			if followplayer == 0 { m_paninc.x = 0 }
		case AM_PANUPKEY:
			if followplayer == 0 { m_paninc.y = 0 }
		case AM_PANDOWNKEY:
			if followplayer == 0 { m_paninc.y = 0 }
		case AM_ZOOMOUTKEY, AM_ZOOMINKEY:
			mtof_zoommul = FRACUNIT
			ftom_zoommul = FRACUNIT
		}
	}

	return rc
}

AM_changeWindowScale :: proc() {
	scale_mtof = FixedMul(scale_mtof, mtof_zoommul)
	scale_ftom = FixedDiv(FRACUNIT, scale_mtof)

	if scale_mtof < min_scale_mtof {
		AM_minOutWindowScale()
	} else if scale_mtof > max_scale_mtof {
		AM_maxOutWindowScale()
	} else {
		AM_activateNewScale()
	}
}

AM_doFollowPlayer :: proc() {
	if f_oldloc.x != plr.mo.x || f_oldloc.y != plr.mo.y {
		m_x = FTOM(MTOF(plr.mo.x)) - m_w/2
		m_y = FTOM(MTOF(plr.mo.y)) - m_h/2
		m_x2 = m_x + m_w
		m_y2 = m_y + m_h
		f_oldloc.x = plr.mo.x
		f_oldloc.y = plr.mo.y
	}
}

AM_updateLightLev :: proc() {
	if amclock > am_nexttic {
		lightlev = am_litelevels[am_litelevelscnt]
		am_litelevelscnt += 1
		if am_litelevelscnt == len(am_litelevels) {
			am_litelevelscnt = 0
		}
		am_nexttic = amclock + 6 - (amclock % 6)
	}
}

AM_Ticker :: proc() {
	if !automapactive {
		return
	}

	amclock += 1

	if followplayer != 0 {
		AM_doFollowPlayer()
	}

	if ftom_zoommul != FRACUNIT {
		AM_changeWindowScale()
	}

	if m_paninc.x != 0 || m_paninc.y != 0 {
		AM_changeWindowLoc()
	}
}

AM_clearFB :: proc(color: int) {
	libc.memset(fb, i32(color), uint(f_w * f_h))
}

AM_clipMline :: proc(ml: ^mline_t, fl: ^fline_t) -> boolean {
	LEFT   :: 1
	RIGHT  :: 2
	BOTTOM :: 4
	TOP    :: 8

	outcode1 := 0
	outcode2 := 0
	outside: int
	tmp: fpoint_t
	dx, dy: int

	dooutcode :: #force_inline proc(mx, my: int) -> int {
		oc := 0
		if my < 0 { oc |= TOP } else if my >= f_h { oc |= BOTTOM }
		if mx < 0 { oc |= LEFT } else if mx >= f_w { oc |= RIGHT }
		return oc
	}

	if ml.a.y > m_y2 {
		outcode1 = TOP
	} else if ml.a.y < m_y {
		outcode1 = BOTTOM
	}

	if ml.b.y > m_y2 {
		outcode2 = TOP
	} else if ml.b.y < m_y {
		outcode2 = BOTTOM
	}

	if (outcode1 & outcode2) != 0 {
		return false
	}

	if ml.a.x < m_x {
		outcode1 |= LEFT
	} else if ml.a.x > m_x2 {
		outcode1 |= RIGHT
	}

	if ml.b.x < m_x {
		outcode2 |= LEFT
	} else if ml.b.x > m_x2 {
		outcode2 |= RIGHT
	}

	if (outcode1 & outcode2) != 0 {
		return false
	}

	fl.a.x = CXMTOF(ml.a.x)
	fl.a.y = CYMTOF(ml.a.y)
	fl.b.x = CXMTOF(ml.b.x)
	fl.b.y = CYMTOF(ml.b.y)

	outcode1 = dooutcode(fl.a.x, fl.a.y)
	outcode2 = dooutcode(fl.b.x, fl.b.y)

	if (outcode1 & outcode2) != 0 {
		return false
	}

	for (outcode1 | outcode2) != 0 {
		if outcode1 != 0 {
			outside = outcode1
		} else {
			outside = outcode2
		}

		if (outside & TOP) != 0 {
			dy = fl.a.y - fl.b.y
			dx = fl.b.x - fl.a.x
			tmp.x = fl.a.x + (dx * fl.a.y) / dy
			tmp.y = 0
		} else if (outside & BOTTOM) != 0 {
			dy = fl.a.y - fl.b.y
			dx = fl.b.x - fl.a.x
			tmp.x = fl.a.x + (dx * (fl.a.y - f_h)) / dy
			tmp.y = f_h - 1
		} else if (outside & RIGHT) != 0 {
			dy = fl.b.y - fl.a.y
			dx = fl.b.x - fl.a.x
			tmp.y = fl.a.y + (dy * (f_w - 1 - fl.a.x)) / dx
			tmp.x = f_w - 1
		} else if (outside & LEFT) != 0 {
			dy = fl.b.y - fl.a.y
			dx = fl.b.x - fl.a.x
			tmp.y = fl.a.y + (dy * (-fl.a.x)) / dx
			tmp.x = 0
		}

		if outside == outcode1 {
			fl.a = tmp
			outcode1 = dooutcode(fl.a.x, fl.a.y)
		} else {
			fl.b = tmp
			outcode2 = dooutcode(fl.b.x, fl.b.y)
		}

		if (outcode1 & outcode2) != 0 {
			return false
		}
	}

	return true
}

AM_drawFline :: proc(fl: ^fline_t, color: int) {
	if fl.a.x < 0 || fl.a.x >= f_w ||
	   fl.a.y < 0 || fl.a.y >= f_h ||
	   fl.b.x < 0 || fl.b.x >= f_w ||
	   fl.b.y < 0 || fl.b.y >= f_h {
		libc.fprintf(libc.stderr, "fuck %d \r", i32(am_fuck))
		am_fuck += 1
		return
	}

	dx := fl.b.x - fl.a.x
	ax := 2 * (dx < 0 ? -dx : dx)
	sx := dx < 0 ? -1 : 1

	dy := fl.b.y - fl.a.y
	ay := 2 * (dy < 0 ? -dy : dy)
	sy := dy < 0 ? -1 : 1

	x := fl.a.x
	y := fl.a.y

	if ax > ay {
		d := ay - ax/2
		for {
			fb[y * f_w + x] = byte(color)
			if x == fl.b.x { return }
			if d >= 0 {
				y += sy
				d -= ax
			}
			x += sx
			d += ay
		}
	} else {
		d := ax - ay/2
		for {
			fb[y * f_w + x] = byte(color)
			if y == fl.b.y { return }
			if d >= 0 {
				x += sx
				d -= ay
			}
			y += sy
			d += ax
		}
	}
}

AM_drawMline :: proc(ml: ^mline_t, color: int) {
	fl: fline_t
	if AM_clipMline(ml, &fl) {
		AM_drawFline(&fl, color)
	}
}

AM_drawGrid :: proc(color: int) {
	ml: mline_t

	start := m_x
	if (start - bmaporgx) % (MAPBLOCKUNITS << FRACBITS) != 0 {
		start += (MAPBLOCKUNITS << FRACBITS) - ((start - bmaporgx) % (MAPBLOCKUNITS << FRACBITS))
	}
	end := m_x + m_w

	ml.a.y = m_y
	ml.b.y = m_y + m_h
	for x := start; x < end; x += (MAPBLOCKUNITS << FRACBITS) {
		ml.a.x = x
		ml.b.x = x
		AM_drawMline(&ml, color)
	}

	start = m_y
	if (start - bmaporgy) % (MAPBLOCKUNITS << FRACBITS) != 0 {
		start += (MAPBLOCKUNITS << FRACBITS) - ((start - bmaporgy) % (MAPBLOCKUNITS << FRACBITS))
	}
	end = m_y + m_h

	ml.a.x = m_x
	ml.b.x = m_x + m_w
	for y := start; y < end; y += (MAPBLOCKUNITS << FRACBITS) {
		ml.a.y = y
		ml.b.y = y
		AM_drawMline(&ml, color)
	}
}

AM_drawWalls :: proc() {
	l: mline_t

	for i := 0; i < numlines; i += 1 {
		l.a.x = lines[i].v1.x
		l.a.y = lines[i].v1.y
		l.b.x = lines[i].v2.x
		l.b.y = lines[i].v2.y
		if cheating != 0 || (int(lines[i].flags) & ML_MAPPED) != 0 {
			if (int(lines[i].flags) & LINE_NEVERSEE) != 0 && cheating == 0 {
				continue
			}
			if lines[i].backsector == nil {
				AM_drawMline(&l, WALLCOLORS + lightlev)
			} else {
				if lines[i].special == 39 {
					AM_drawMline(&l, WALLCOLORS + WALLRANGE/2)
				} else if (int(lines[i].flags) & ML_SECRET) != 0 {
					if cheating != 0 {
						AM_drawMline(&l, SECRETWALLCOLORS + lightlev)
					} else {
						AM_drawMline(&l, WALLCOLORS + lightlev)
					}
				} else if lines[i].backsector.floorheight != lines[i].frontsector.floorheight {
					AM_drawMline(&l, FDWALLCOLORS + lightlev)
				} else if lines[i].backsector.ceilingheight != lines[i].frontsector.ceilingheight {
					AM_drawMline(&l, CDWALLCOLORS + lightlev)
				} else if cheating != 0 {
					AM_drawMline(&l, TSWALLCOLORS + lightlev)
				}
			}
		} else if plr.powers[pw_allmap] != 0 {
			if (int(lines[i].flags) & LINE_NEVERSEE) == 0 {
				AM_drawMline(&l, GRAYS + 3)
			}
		}
	}
}

AM_rotate :: proc(x: ^fixed_t, y: ^fixed_t, a: angle_t) {
	tmpx := FixedMul(x^, finecosine[a >> ANGLETOFINESHIFT]) - FixedMul(y^, finesine[a >> ANGLETOFINESHIFT])
	y^ = FixedMul(x^, finesine[a >> ANGLETOFINESHIFT]) + FixedMul(y^, finecosine[a >> ANGLETOFINESHIFT])
	x^ = tmpx
}

AM_drawLineCharacter :: proc(lineguy: [^]mline_t, lineguylines: int, scale: fixed_t, angle: angle_t, color: int, x: fixed_t, y: fixed_t) {
	l: mline_t

	for i := 0; i < lineguylines; i += 1 {
		l.a.x = lineguy[i].a.x
		l.a.y = lineguy[i].a.y

		if scale != 0 {
			l.a.x = FixedMul(scale, l.a.x)
			l.a.y = FixedMul(scale, l.a.y)
		}

		if angle != 0 {
			AM_rotate(&l.a.x, &l.a.y, angle)
		}

		l.a.x += x
		l.a.y += y

		l.b.x = lineguy[i].b.x
		l.b.y = lineguy[i].b.y

		if scale != 0 {
			l.b.x = FixedMul(scale, l.b.x)
			l.b.y = FixedMul(scale, l.b.y)
		}

		if angle != 0 {
			AM_rotate(&l.b.x, &l.b.y, angle)
		}

		l.b.x += x
		l.b.y += y

		AM_drawMline(&l, color)
	}
}

AM_drawPlayers :: proc() {
	their_color := -1
	color: int

	if !netgame {
		if cheating != 0 {
			AM_drawLineCharacter(&cheat_player_arrow[0], NUMCHEATPLYRLINES, 0, plr.mo.angle, WHITE, plr.mo.x, plr.mo.y)
		} else {
			AM_drawLineCharacter(&player_arrow[0], NUMPLYRLINES, 0, plr.mo.angle, WHITE, plr.mo.x, plr.mo.y)
		}
		return
	}

	for i := 0; i < MAXPLAYERS; i += 1 {
		their_color += 1
		p := &players[i]

		if (deathmatch != 0 && !singledemo) && p != plr {
			continue
		}

		if !playeringame[i] {
			continue
		}

		if p.powers[pw_invisibility] != 0 {
			color = 246
		} else {
			color = am_their_colors[their_color]
		}

		AM_drawLineCharacter(&player_arrow[0], NUMPLYRLINES, 0, p.mo.angle, color, p.mo.x, p.mo.y)
	}
}

AM_drawThings :: proc(colors: int, colorrange: int) {
	for i := 0; i < numsectors; i += 1 {
		t := sectors[i].thinglist
		for t != nil {
			AM_drawLineCharacter(&thintriangle_guy[0], NUMTHINTRIANGLEGUYLINES, 16 << FRACBITS, t.angle, colors + lightlev, t.x, t.y)
			t = t.snext
		}
	}
}

AM_drawMarks :: proc() {
	for i := 0; i < AM_NUMMARKPOINTS; i += 1 {
		if markpoints[i].x != -1 {
			w := 5
			h := 6
			fx := CXMTOF(markpoints[i].x)
			fy := CYMTOF(markpoints[i].y)
			if fx >= f_x && fx <= f_w - w && fy >= f_y && fy <= f_h - h {
				V_DrawPatch(fx, fy, FB, marknums[i])
			}
		}
	}
}

AM_drawCrosshair :: proc(color: int) {
	fb[(f_w * (f_h + 1)) / 2] = byte(color)
}

AM_Drawer :: proc() {
	if !automapactive {
		return
	}

	AM_clearFB(BACKGROUND)
	if grid != 0 {
		AM_drawGrid(GRIDCOLORS)
	}
	AM_drawWalls()
	AM_drawPlayers()
	if cheating == 2 {
		AM_drawThings(THINGCOLORS, THINGRANGE)
	}
	AM_drawCrosshair(XHAIRCOLORS)
	AM_drawMarks()
	V_MarkRect(f_x, f_y, f_w, f_h)
}