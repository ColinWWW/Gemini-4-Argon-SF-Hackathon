// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

NUMEPISODES :: 4
NUMMAPS     :: 9

WI_TITLEY   :: 2
WI_SPACINGY :: 33

SP_STATSX :: 50
SP_STATSY :: 50

SP_TIMEX :: 16
SP_TIMEY :: SCREENHEIGHT - 32

NG_STATSY   :: 50
NG_SPACINGX :: 64

DM_MATRIXX  :: 42
DM_MATRIXY  :: 68
DM_SPACINGX :: 40
DM_TOTALSX  :: 269
DM_KILLERSX :: 10
DM_KILLERSY :: 100
DM_VICTIMSX :: 5
DM_VICTIMSY :: 50

animenum_t :: enum i32 {
	ANIM_ALWAYS,
	ANIM_RANDOM,
	ANIM_LEVEL,
}

point_t :: struct {
	x: int,
	y: int,
}

wi_anim_t :: struct {
	type:      animenum_t,
	period:    int,
	nanims:    int,
	loc:       point_t,
	data1:     int,
	data2:     int,
	p:         [3]^patch_t,
	nexttic:   int,
	lastdrawn: int,
	ctr:       int,
	state:     int,
}

lnodes := [NUMEPISODES][NUMMAPS]point_t{
	{
		{ 185, 164 },
		{ 148, 143 },
		{ 69, 122 },
		{ 209, 102 },
		{ 116, 89 },
		{ 166, 55 },
		{ 71, 56 },
		{ 135, 29 },
		{ 71, 24 },
	},
	{
		{ 254, 25 },
		{ 97, 50 },
		{ 188, 64 },
		{ 128, 78 },
		{ 214, 92 },
		{ 133, 130 },
		{ 208, 136 },
		{ 148, 140 },
		{ 235, 158 },
	},
	{
		{ 156, 168 },
		{ 48, 154 },
		{ 174, 95 },
		{ 265, 75 },
		{ 130, 48 },
		{ 279, 23 },
		{ 198, 48 },
		{ 140, 25 },
		{ 281, 136 },
	},
	{},
}

epsd0animinfo := [10]wi_anim_t{
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 224, 104 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 184, 160 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 112, 136 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 72, 112 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 88, 96 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 64, 48 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 192, 40 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 136, 16 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 80, 16 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 64, 24 }, 0, 0, {}, 0, 0, 0, 0 },
}

epsd1animinfo := [9]wi_anim_t{
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 1, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 2, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 3, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 4, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 5, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 6, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 7, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 3, { 192, 144 }, 8, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_LEVEL, TICRATE / 3, 1, { 128, 136 }, 8, 0, {}, 0, 0, 0, 0 },
}

epsd2animinfo := [6]wi_anim_t{
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 104, 168 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 40, 136 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 160, 96 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 104, 80 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 3, 3, { 120, 32 }, 0, 0, {}, 0, 0, 0, 0 },
	{ .ANIM_ALWAYS, TICRATE / 4, 3, { 40, 0 }, 0, 0, {}, 0, 0, 0, 0 },
}

NUMANIMS := [NUMEPISODES]int{
	len(epsd0animinfo),
	len(epsd1animinfo),
	len(epsd2animinfo),
	0,
}

wi_anims := [NUMEPISODES][^]wi_anim_t{
	&epsd0animinfo[0],
	&epsd1animinfo[0],
	&epsd2animinfo[0],
	nil,
}

SHOWNEXTLOCDELAY :: 4

acceleratestage: int
me: int
wi_state: stateenum_t
wbs: ^wbstartstruct_t
plrs: [^]wbplayerstruct_t
cnt: int
bcnt: int
firstrefresh: int

cnt_kills: [MAXPLAYERS]int
cnt_items: [MAXPLAYERS]int
cnt_secret: [MAXPLAYERS]int
cnt_time: int
cnt_par: int
cnt_pause: int

NUMCMAPS: int

bg: ^patch_t
yah: [2]^patch_t
splat: ^patch_t
percent: ^patch_t
colon: ^patch_t
num: [10]^patch_t
wiminus: ^patch_t
finished: ^patch_t
entering: ^patch_t
sp_secret: ^patch_t
kills: ^patch_t
secret: ^patch_t
items: ^patch_t
frags: ^patch_t
wi_time_patch: ^patch_t
par: ^patch_t
sucks: ^patch_t
killers: ^patch_t
victims: ^patch_t
total: ^patch_t
star: ^patch_t
bstar: ^patch_t
wi_p: [MAXPLAYERS]^patch_t
bp: [MAXPLAYERS]^patch_t
lnames: [^]^patch_t

snl_pointeron: boolean = false

dm_state: int
dm_frags: [MAXPLAYERS][MAXPLAYERS]int
dm_totals: [MAXPLAYERS]int

cnt_frags: [MAXPLAYERS]int
dofrags: int
ng_state: int

sp_state: int

WI_slamBackground :: proc() {
	libc.memcpy(screens[0], screens[1], SCREENWIDTH * SCREENHEIGHT)
	V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT)
}

WI_Responder :: proc(ev: ^event_t) -> boolean {
	return false
}

WI_drawLF :: proc() {
	y := WI_TITLEY
	V_DrawPatch((SCREENWIDTH - int(SHORT(lnames[wbs.last].width))) / 2, y, FB, lnames[wbs.last])
	y += (5 * int(SHORT(lnames[wbs.last].height))) / 4
	V_DrawPatch((SCREENWIDTH - int(SHORT(finished.width))) / 2, y, FB, finished)
}

WI_drawEL :: proc() {
	y := WI_TITLEY
	V_DrawPatch((SCREENWIDTH - int(SHORT(entering.width))) / 2, y, FB, entering)
	y += (5 * int(SHORT(lnames[wbs.next].height))) / 4
	V_DrawPatch((SCREENWIDTH - int(SHORT(lnames[wbs.next].width))) / 2, y, FB, lnames[wbs.next])
}

WI_drawOnLnode :: proc(n: int, c: [^]^patch_t) {
	fits := false
	i := 0
	for {
		left := lnodes[wbs.epsd][n].x - int(SHORT(c[i].leftoffset))
		top := lnodes[wbs.epsd][n].y - int(SHORT(c[i].topoffset))
		right := left + int(SHORT(c[i].width))
		bottom := top + int(SHORT(c[i].height))

		if left >= 0 && right < SCREENWIDTH && top >= 0 && bottom < SCREENHEIGHT {
			fits = true
		} else {
			i += 1
		}

		if fits || i == 2 {
			break
		}
	}

	if fits && i < 2 {
		V_DrawPatch(lnodes[wbs.epsd][n].x, lnodes[wbs.epsd][n].y, FB, c[i])
	} else {
		libc.printf("Could not place patch on level %d", i32(n + 1))
	}
}

WI_initAnimatedBack :: proc() {
	if gamemode == .commercial {
		return
	}
	if wbs.epsd > 2 {
		return
	}

	for i := 0; i < NUMANIMS[wbs.epsd]; i += 1 {
		a := &wi_anims[wbs.epsd][i]
		a.ctr = -1

		if a.type == .ANIM_ALWAYS {
			a.nexttic = bcnt + 1 + (M_Random() % a.period)
		} else if a.type == .ANIM_RANDOM {
			a.nexttic = bcnt + 1 + a.data2 + (M_Random() % a.data1)
		} else if a.type == .ANIM_LEVEL {
			a.nexttic = bcnt + 1
		}
	}
}

WI_updateAnimatedBack :: proc() {
	if gamemode == .commercial {
		return
	}
	if wbs.epsd > 2 {
		return
	}

	for i := 0; i < NUMANIMS[wbs.epsd]; i += 1 {
		a := &wi_anims[wbs.epsd][i]

		if bcnt == a.nexttic {
			switch a.type {
			case .ANIM_ALWAYS:
				a.ctr += 1
				if a.ctr >= a.nanims {
					a.ctr = 0
				}
				a.nexttic = bcnt + a.period
			case .ANIM_RANDOM:
				a.ctr += 1
				if a.ctr == a.nanims {
					a.ctr = -1
					a.nexttic = bcnt + a.data2 + (M_Random() % a.data1)
				} else {
					a.nexttic = bcnt + a.period
				}
			case .ANIM_LEVEL:
				if !(wi_state == .StatCount && i == 7) && wbs.next == a.data1 {
					a.ctr += 1
					if a.ctr == a.nanims {
						a.ctr -= 1
					}
					a.nexttic = bcnt + a.period
				}
			}
		}
	}
}

WI_drawAnimatedBack :: proc() {
	// Note: in original C `if (commercial)` evaluated the enum constant `commercial` (which is 2, true!)
	return
}

WI_drawNum :: proc(x_in: int, y: int, n_in: int, digits_in: int) -> int {
	x := x_in
	n := n_in
	digits := digits_in

	fontwidth := int(SHORT(num[0].width))

	if digits < 0 {
		if n == 0 {
			digits = 1
		} else {
			digits = 0
			temp := n
			for temp != 0 {
				temp /= 10
				digits += 1
			}
		}
	}

	neg := n < 0
	if neg {
		n = -n
	}

	if n == 1994 {
		return 0
	}

	for digits > 0 {
		digits -= 1
		x -= fontwidth
		V_DrawPatch(x, y, FB, num[n % 10])
		n /= 10
	}

	if neg {
		x -= 8
		V_DrawPatch(x, y, FB, wiminus)
	}

	return x
}

WI_drawPercent :: proc(x: int, y: int, p: int) {
	if p < 0 {
		return
	}
	V_DrawPatch(x, y, FB, percent)
	WI_drawNum(x, y, p, -1)
}

WI_drawTime :: proc(x_in: int, y: int, t: int) {
	x := x_in
	if t < 0 {
		return
	}

	if t <= 61 * 59 {
		div := 1
		for {
			n := (t / div) % 60
			x = WI_drawNum(x, y, n, 2) - int(SHORT(colon.width))
			div *= 60

			if div == 60 || (t / div) != 0 {
				V_DrawPatch(x, y, FB, colon)
			}

			if (t / div) == 0 {
				break
			}
		}
	} else {
		V_DrawPatch(x - int(SHORT(sucks.width)), y, FB, sucks)
	}
}

WI_End :: proc() {
	WI_unloadData()
}

WI_initNoState :: proc() {
	wi_state = .NoState
	acceleratestage = 0
	cnt = 10
}

WI_updateNoState :: proc() {
	WI_updateAnimatedBack()
	cnt -= 1
	if cnt == 0 {
		WI_End()
		G_WorldDone()
	}
}

WI_initShowNextLoc :: proc() {
	wi_state = .ShowNextLoc
	acceleratestage = 0
	cnt = SHOWNEXTLOCDELAY * TICRATE
	WI_initAnimatedBack()
}

WI_updateShowNextLoc :: proc() {
	WI_updateAnimatedBack()
	cnt -= 1
	if cnt == 0 || acceleratestage != 0 {
		WI_initNoState()
	} else {
		snl_pointeron = (cnt & 31) < 20
	}
}

WI_drawShowNextLoc :: proc() {
	WI_slamBackground()
	WI_drawAnimatedBack()

	if gamemode != .commercial {
		if wbs.epsd > 2 {
			WI_drawEL()
			return
		}

		last := (wbs.last == 8) ? wbs.next - 1 : wbs.last

		for i := 0; i <= last; i += 1 {
			WI_drawOnLnode(i, &splat)
		}

		if wbs.didsecret {
			WI_drawOnLnode(8, &splat)
		}

		if snl_pointeron {
			WI_drawOnLnode(wbs.next, &yah[0])
		}
	}

	if gamemode != .commercial || wbs.next != 30 {
		WI_drawEL()
	}
}

WI_drawNoState :: proc() {
	snl_pointeron = true
	WI_drawShowNextLoc()
}

WI_fragSum :: proc(playernum: int) -> int {
	frags := 0
	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] && i != playernum {
			frags += plrs[playernum].frags[i]
		}
	}
	frags -= plrs[playernum].frags[playernum]
	return frags
}

WI_initDeathmatchStats :: proc() {
	wi_state = .StatCount
	acceleratestage = 0
	dm_state = 1
	cnt_pause = TICRATE

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			for j := 0; j < MAXPLAYERS; j += 1 {
				if playeringame[j] {
					dm_frags[i][j] = 0
				}
			}
			dm_totals[i] = 0
		}
	}

	WI_initAnimatedBack()
}

WI_updateDeathmatchStats :: proc() {
	WI_updateAnimatedBack()

	if acceleratestage != 0 && dm_state != 4 {
		acceleratestage = 0
		for i := 0; i < MAXPLAYERS; i += 1 {
			if playeringame[i] {
				for j := 0; j < MAXPLAYERS; j += 1 {
					if playeringame[j] {
						dm_frags[i][j] = plrs[i].frags[j]
					}
				}
				dm_totals[i] = WI_fragSum(i)
			}
		}
		S_StartSound(nil, int(sfxenum_t.sfx_barexp))
		dm_state = 4
	}

	if dm_state == 2 {
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}

		stillticking := false

		for i := 0; i < MAXPLAYERS; i += 1 {
			if playeringame[i] {
				for j := 0; j < MAXPLAYERS; j += 1 {
					if playeringame[j] && dm_frags[i][j] != plrs[i].frags[j] {
						if plrs[i].frags[j] < 0 {
							dm_frags[i][j] -= 1
						} else {
							dm_frags[i][j] += 1
						}

						if dm_frags[i][j] > 99 {
							dm_frags[i][j] = 99
						}
						if dm_frags[i][j] < -99 {
							dm_frags[i][j] = -99
						}
						stillticking = true
					}
				}
				dm_totals[i] = WI_fragSum(i)
				if dm_totals[i] > 99 {
					dm_totals[i] = 99
				}
				if dm_totals[i] < -99 {
					dm_totals[i] = -99
				}
			}
		}

		if !stillticking {
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			dm_state += 1
		}
	} else if dm_state == 4 {
		if acceleratestage != 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_slop))
			if gamemode == .commercial {
				WI_initNoState()
			} else {
				WI_initShowNextLoc()
			}
		}
	} else if (dm_state & 1) != 0 {
		cnt_pause -= 1
		if cnt_pause == 0 {
			dm_state += 1
			cnt_pause = TICRATE
		}
	}
}

WI_drawDeathmatchStats :: proc() {
	WI_slamBackground()
	WI_drawAnimatedBack()
	WI_drawLF()

	V_DrawPatch(DM_TOTALSX - int(SHORT(total.width)) / 2, DM_MATRIXY - WI_SPACINGY + 10, FB, total)
	V_DrawPatch(DM_KILLERSX, DM_KILLERSY, FB, killers)
	V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, FB, victims)

	x := DM_MATRIXX + DM_SPACINGX
	y := DM_MATRIXY

	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			V_DrawPatch(x - int(SHORT(wi_p[i].width)) / 2, DM_MATRIXY - WI_SPACINGY, FB, wi_p[i])
			V_DrawPatch(DM_MATRIXX - int(SHORT(wi_p[i].width)) / 2, y, FB, wi_p[i])

			if i == me {
				V_DrawPatch(x - int(SHORT(wi_p[i].width)) / 2, DM_MATRIXY - WI_SPACINGY, FB, bstar)
				V_DrawPatch(DM_MATRIXX - int(SHORT(wi_p[i].width)) / 2, y, FB, star)
			}
		}
		x += DM_SPACINGX
		y += WI_SPACINGY
	}

	y = DM_MATRIXY + 10
	w := int(SHORT(num[0].width))

	for i := 0; i < MAXPLAYERS; i += 1 {
		x = DM_MATRIXX + DM_SPACINGX
		if playeringame[i] {
			for j := 0; j < MAXPLAYERS; j += 1 {
				if playeringame[j] {
					WI_drawNum(x + w, y, dm_frags[i][j], 2)
				}
				x += DM_SPACINGX
			}
			WI_drawNum(DM_TOTALSX + w, y, dm_totals[i], 2)
		}
		y += WI_SPACINGY
	}
}

WI_initNetgameStats :: proc() {
	wi_state = .StatCount
	acceleratestage = 0
	ng_state = 1
	cnt_pause = TICRATE

	for i := 0; i < MAXPLAYERS; i += 1 {
		if !playeringame[i] {
			continue
		}
		cnt_kills[i] = 0
		cnt_items[i] = 0
		cnt_secret[i] = 0
		cnt_frags[i] = 0
		dofrags += WI_fragSum(i)
	}

	dofrags = dofrags != 0 ? 1 : 0
	WI_initAnimatedBack()
}

WI_updateNetgameStats :: proc() {
	WI_updateAnimatedBack()

	if acceleratestage != 0 && ng_state != 10 {
		acceleratestage = 0
		for i := 0; i < MAXPLAYERS; i += 1 {
			if !playeringame[i] {
				continue
			}
			cnt_kills[i] = (plrs[i].skills * 100) / wbs.maxkills
			cnt_items[i] = (plrs[i].sitems * 100) / wbs.maxitems
			cnt_secret[i] = (plrs[i].ssecret * 100) / wbs.maxsecret
			if dofrags != 0 {
				cnt_frags[i] = WI_fragSum(i)
			}
		}
		S_StartSound(nil, int(sfxenum_t.sfx_barexp))
		ng_state = 10
	}

	if ng_state == 2 {
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		stillticking := false
		for i := 0; i < MAXPLAYERS; i += 1 {
			if !playeringame[i] {
				continue
			}
			cnt_kills[i] += 2
			if cnt_kills[i] >= (plrs[i].skills * 100) / wbs.maxkills {
				cnt_kills[i] = (plrs[i].skills * 100) / wbs.maxkills
			} else {
				stillticking = true
			}
		}
		if !stillticking {
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			ng_state += 1
		}
	} else if ng_state == 4 {
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		stillticking := false
		for i := 0; i < MAXPLAYERS; i += 1 {
			if !playeringame[i] {
				continue
			}
			cnt_items[i] += 2
			if cnt_items[i] >= (plrs[i].sitems * 100) / wbs.maxitems {
				cnt_items[i] = (plrs[i].sitems * 100) / wbs.maxitems
			} else {
				stillticking = true
			}
		}
		if !stillticking {
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			ng_state += 1
		}
	} else if ng_state == 6 {
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		stillticking := false
		for i := 0; i < MAXPLAYERS; i += 1 {
			if !playeringame[i] {
				continue
			}
			cnt_secret[i] += 2
			if cnt_secret[i] >= (plrs[i].ssecret * 100) / wbs.maxsecret {
				cnt_secret[i] = (plrs[i].ssecret * 100) / wbs.maxsecret
			} else {
				stillticking = true
			}
		}
		if !stillticking {
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			ng_state += 1 + 2 * (dofrags == 0 ? 1 : 0)
		}
	} else if ng_state == 8 {
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		stillticking := false
		for i := 0; i < MAXPLAYERS; i += 1 {
			if !playeringame[i] {
				continue
			}
			cnt_frags[i] += 1
			fsum := WI_fragSum(i)
			if cnt_frags[i] >= fsum {
				cnt_frags[i] = fsum
			} else {
				stillticking = true
			}
		}
		if !stillticking {
			S_StartSound(nil, int(sfxenum_t.sfx_pldeth))
			ng_state += 1
		}
	} else if ng_state == 10 {
		if acceleratestage != 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_sgcock))
			if gamemode == .commercial {
				WI_initNoState()
			} else {
				WI_initShowNextLoc()
			}
		}
	} else if (ng_state & 1) != 0 {
		cnt_pause -= 1
		if cnt_pause == 0 {
			ng_state += 1
			cnt_pause = TICRATE
		}
	}
}

WI_drawNetgameStats :: proc() {
	pwidth := int(SHORT(percent.width))

	WI_slamBackground()
	WI_drawAnimatedBack()
	WI_drawLF()

	ng_statsx := 32 + int(SHORT(star.width)) / 2 + 32 * (dofrags == 0 ? 1 : 0)

	V_DrawPatch(ng_statsx + NG_SPACINGX - int(SHORT(kills.width)), NG_STATSY, FB, kills)
	V_DrawPatch(ng_statsx + 2 * NG_SPACINGX - int(SHORT(items.width)), NG_STATSY, FB, items)
	V_DrawPatch(ng_statsx + 3 * NG_SPACINGX - int(SHORT(secret.width)), NG_STATSY, FB, secret)

	if dofrags != 0 {
		V_DrawPatch(ng_statsx + 4 * NG_SPACINGX - int(SHORT(frags.width)), NG_STATSY, FB, frags)
	}

	y := NG_STATSY + int(SHORT(kills.height))

	for i := 0; i < MAXPLAYERS; i += 1 {
		if !playeringame[i] {
			continue
		}

		x := ng_statsx
		V_DrawPatch(x - int(SHORT(wi_p[i].width)), y, FB, wi_p[i])

		if i == me {
			V_DrawPatch(x - int(SHORT(wi_p[i].width)), y, FB, star)
		}

		x += NG_SPACINGX
		WI_drawPercent(x - pwidth, y + 10, cnt_kills[i])
		x += NG_SPACINGX
		WI_drawPercent(x - pwidth, y + 10, cnt_items[i])
		x += NG_SPACINGX
		WI_drawPercent(x - pwidth, y + 10, cnt_secret[i])
		x += NG_SPACINGX

		if dofrags != 0 {
			WI_drawNum(x, y + 10, cnt_frags[i], -1)
		}

		y += WI_SPACINGY
	}
}

WI_initStats :: proc() {
	wi_state = .StatCount
	acceleratestage = 0
	sp_state = 1
	cnt_kills[0] = -1
	cnt_items[0] = -1
	cnt_secret[0] = -1
	cnt_time = -1
	cnt_par = -1
	cnt_pause = TICRATE

	WI_initAnimatedBack()
}

WI_updateStats :: proc() {
	WI_updateAnimatedBack()

	if acceleratestage != 0 && sp_state != 10 {
		acceleratestage = 0
		cnt_kills[0] = (plrs[me].skills * 100) / wbs.maxkills
		cnt_items[0] = (plrs[me].sitems * 100) / wbs.maxitems
		cnt_secret[0] = (plrs[me].ssecret * 100) / wbs.maxsecret
		cnt_time = plrs[me].stime / TICRATE
		cnt_par = wbs.partime / TICRATE
		S_StartSound(nil, int(sfxenum_t.sfx_barexp))
		sp_state = 10
	}

	if sp_state == 2 {
		cnt_kills[0] += 2
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		if cnt_kills[0] >= (plrs[me].skills * 100) / wbs.maxkills {
			cnt_kills[0] = (plrs[me].skills * 100) / wbs.maxkills
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			sp_state += 1
		}
	} else if sp_state == 4 {
		cnt_items[0] += 2
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		if cnt_items[0] >= (plrs[me].sitems * 100) / wbs.maxitems {
			cnt_items[0] = (plrs[me].sitems * 100) / wbs.maxitems
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			sp_state += 1
		}
	} else if sp_state == 6 {
		cnt_secret[0] += 2
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}
		if cnt_secret[0] >= (plrs[me].ssecret * 100) / wbs.maxsecret {
			cnt_secret[0] = (plrs[me].ssecret * 100) / wbs.maxsecret
			S_StartSound(nil, int(sfxenum_t.sfx_barexp))
			sp_state += 1
		}
	} else if sp_state == 8 {
		if (bcnt & 3) == 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_pistol))
		}

		cnt_time += 3
		if cnt_time >= plrs[me].stime / TICRATE {
			cnt_time = plrs[me].stime / TICRATE
		}

		cnt_par += 3
		if cnt_par >= wbs.partime / TICRATE {
			cnt_par = wbs.partime / TICRATE
			if cnt_time >= plrs[me].stime / TICRATE {
				S_StartSound(nil, int(sfxenum_t.sfx_barexp))
				sp_state += 1
			}
		}
	} else if sp_state == 10 {
		if acceleratestage != 0 {
			S_StartSound(nil, int(sfxenum_t.sfx_sgcock))
			if gamemode == .commercial {
				WI_initNoState()
			} else {
				WI_initShowNextLoc()
			}
		}
	} else if (sp_state & 1) != 0 {
		cnt_pause -= 1
		if cnt_pause == 0 {
			sp_state += 1
			cnt_pause = TICRATE
		}
	}
}

WI_drawStats :: proc() {
	lh := (3 * int(SHORT(num[0].height))) / 2

	WI_slamBackground()
	WI_drawAnimatedBack()
	WI_drawLF()

	V_DrawPatch(SP_STATSX, SP_STATSY, FB, kills)
	WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY, cnt_kills[0])

	V_DrawPatch(SP_STATSX, SP_STATSY + lh, FB, items)
	WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + lh, cnt_items[0])

	V_DrawPatch(SP_STATSX, SP_STATSY + 2 * lh, FB, sp_secret)
	WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + 2 * lh, cnt_secret[0])

	V_DrawPatch(SP_TIMEX, SP_TIMEY, FB, wi_time_patch)
	WI_drawTime(SCREENWIDTH / 2 - SP_TIMEX, SP_TIMEY, cnt_time)

	if wbs.epsd < 3 {
		V_DrawPatch(SCREENWIDTH / 2 + SP_TIMEX, SP_TIMEY, FB, par)
		WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par)
	}
}

WI_checkForAccelerate :: proc() {
	for i := 0; i < MAXPLAYERS; i += 1 {
		player := &players[i]
		if playeringame[i] {
			if (player.cmd.buttons & BT_ATTACK) != 0 {
				if player.attackdown == 0 {
					acceleratestage = 1
				}
				player.attackdown = 1
			} else {
				player.attackdown = 0
			}
			if (player.cmd.buttons & BT_USE) != 0 {
				if player.usedown == 0 {
					acceleratestage = 1
				}
				player.usedown = 1
			} else {
				player.usedown = 0
			}
		}
	}
}

WI_Ticker :: proc() {
	bcnt += 1

	if bcnt == 1 {
		if gamemode == .commercial {
			S_ChangeMusic(int(musicenum_t.mus_dm2int), 1)
		} else {
			S_ChangeMusic(int(musicenum_t.mus_inter), 1)
		}
	}

	WI_checkForAccelerate()

	switch wi_state {
	case .StatCount:
		if deathmatch != 0 {
			WI_updateDeathmatchStats()
		} else if netgame {
			WI_updateNetgameStats()
		} else {
			WI_updateStats()
		}
	case .ShowNextLoc:
		WI_updateShowNextLoc()
	case .NoState:
		WI_updateNoState()
	}
}

WI_loadData :: proc() {
	name: [9]u8

	if gamemode == .commercial {
		libc.strcpy(&name[0], "INTERPIC")
	} else {
		libc.snprintf(&name[0], size_of(name), "WIMAP%d", i32(wbs.epsd))
	}

	if gamemode == .retail {
		if wbs.epsd == 3 {
			libc.strcpy(&name[0], "INTERPIC")
		}
	}

	bg = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_CACHE))
	V_DrawPatch(0, 0, 1, bg)

	if gamemode == .commercial {
		NUMCMAPS = 32
		lnames = ([^]^patch_t)(Z_Malloc(size_of(^patch_t) * NUMCMAPS, PU_STATIC, nil))
		for i := 0; i < NUMCMAPS; i += 1 {
			libc.snprintf(&name[0], size_of(name), "CWILV%2.2d", i32(i))
			lnames[i] = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_STATIC))
		}
	} else {
		lnames = ([^]^patch_t)(Z_Malloc(size_of(^patch_t) * NUMMAPS, PU_STATIC, nil))
		for i := 0; i < NUMMAPS; i += 1 {
			libc.snprintf(&name[0], size_of(name), "WILV%d%d", i32(wbs.epsd), i32(i))
			lnames[i] = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_STATIC))
		}

		yah[0] = (^patch_t)(W_CacheLumpName("WIURH0", PU_STATIC))
		yah[1] = (^patch_t)(W_CacheLumpName("WIURH1", PU_STATIC))
		splat = (^patch_t)(W_CacheLumpName("WISPLAT", PU_STATIC))

		if wbs.epsd < 3 {
			for j := 0; j < NUMANIMS[wbs.epsd]; j += 1 {
				a := &wi_anims[wbs.epsd][j]
				for i := 0; i < a.nanims; i += 1 {
					if wbs.epsd != 1 || j != 8 {
						libc.snprintf(&name[0], size_of(name), "WIA%d%.2d%.2d", i32(wbs.epsd), i32(j), i32(i))
						a.p[i] = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_STATIC))
					} else {
						a.p[i] = wi_anims[1][4].p[i]
					}
				}
			}
		}
	}

	wiminus = (^patch_t)(W_CacheLumpName("WIMINUS", PU_STATIC))

	for i := 0; i < 10; i += 1 {
		libc.snprintf(&name[0], size_of(name), "WINUM%d", i32(i))
		num[i] = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_STATIC))
	}

	percent = (^patch_t)(W_CacheLumpName("WIPCNT", PU_STATIC))
	finished = (^patch_t)(W_CacheLumpName("WIF", PU_STATIC))
	entering = (^patch_t)(W_CacheLumpName("WIENTER", PU_STATIC))
	kills = (^patch_t)(W_CacheLumpName("WIOSTK", PU_STATIC))
	secret = (^patch_t)(W_CacheLumpName("WIOSTS", PU_STATIC))
	sp_secret = (^patch_t)(W_CacheLumpName("WISCRT2", PU_STATIC))

	// Note: in original C `if (french)` evaluated enum constant `french` (1, true!)
	if netgame && deathmatch == 0 {
		items = (^patch_t)(W_CacheLumpName("WIOBJ", PU_STATIC))
	} else {
		items = (^patch_t)(W_CacheLumpName("WIOSTI", PU_STATIC))
	}

	frags = (^patch_t)(W_CacheLumpName("WIFRGS", PU_STATIC))
	colon = (^patch_t)(W_CacheLumpName("WICOLON", PU_STATIC))
	wi_time_patch = (^patch_t)(W_CacheLumpName("WITIME", PU_STATIC))
	sucks = (^patch_t)(W_CacheLumpName("WISUCKS", PU_STATIC))
	par = (^patch_t)(W_CacheLumpName("WIPAR", PU_STATIC))
	killers = (^patch_t)(W_CacheLumpName("WIKILRS", PU_STATIC))
	victims = (^patch_t)(W_CacheLumpName("WIVCTMS", PU_STATIC))
	total = (^patch_t)(W_CacheLumpName("WIMSTT", PU_STATIC))
	star = (^patch_t)(W_CacheLumpName("STFST01", PU_STATIC))
	bstar = (^patch_t)(W_CacheLumpName("STFDEAD0", PU_STATIC))

	for i := 0; i < MAXPLAYERS; i += 1 {
		libc.snprintf(&name[0], size_of(name), "STPB%d", i32(i))
		wi_p[i] = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_STATIC))

		libc.snprintf(&name[0], size_of(name), "WIBP%d", i32(i + 1))
		bp[i] = (^patch_t)(W_CacheLumpName(cstring(&name[0]), PU_STATIC))
	}
}

WI_unloadData :: proc() {
	Z_ChangeTag(wiminus, PU_CACHE)

	for i := 0; i < 10; i += 1 {
		Z_ChangeTag(num[i], PU_CACHE)
	}

	if gamemode == .commercial {
		for i := 0; i < NUMCMAPS; i += 1 {
			Z_ChangeTag(lnames[i], PU_CACHE)
		}
	} else {
		Z_ChangeTag(yah[0], PU_CACHE)
		Z_ChangeTag(yah[1], PU_CACHE)
		Z_ChangeTag(splat, PU_CACHE)

		for i := 0; i < NUMMAPS; i += 1 {
			Z_ChangeTag(lnames[i], PU_CACHE)
		}

		if wbs.epsd < 3 {
			for j := 0; j < NUMANIMS[wbs.epsd]; j += 1 {
				if wbs.epsd != 1 || j != 8 {
					for i := 0; i < wi_anims[wbs.epsd][j].nanims; i += 1 {
						Z_ChangeTag(wi_anims[wbs.epsd][j].p[i], PU_CACHE)
					}
				}
			}
		}
	}

	Z_Free(lnames)

	Z_ChangeTag(percent, PU_CACHE)
	Z_ChangeTag(colon, PU_CACHE)
	Z_ChangeTag(finished, PU_CACHE)
	Z_ChangeTag(entering, PU_CACHE)
	Z_ChangeTag(kills, PU_CACHE)
	Z_ChangeTag(secret, PU_CACHE)
	Z_ChangeTag(sp_secret, PU_CACHE)
	Z_ChangeTag(items, PU_CACHE)
	Z_ChangeTag(frags, PU_CACHE)
	Z_ChangeTag(wi_time_patch, PU_CACHE)
	Z_ChangeTag(sucks, PU_CACHE)
	Z_ChangeTag(par, PU_CACHE)

	Z_ChangeTag(victims, PU_CACHE)
	Z_ChangeTag(killers, PU_CACHE)
	Z_ChangeTag(total, PU_CACHE)

	for i := 0; i < MAXPLAYERS; i += 1 {
		Z_ChangeTag(wi_p[i], PU_CACHE)
	}

	for i := 0; i < MAXPLAYERS; i += 1 {
		Z_ChangeTag(bp[i], PU_CACHE)
	}
}

WI_Drawer :: proc() {
	switch wi_state {
	case .StatCount:
		if deathmatch != 0 {
			WI_drawDeathmatchStats()
		} else if netgame {
			WI_drawNetgameStats()
		} else {
			WI_drawStats()
		}
	case .ShowNextLoc:
		WI_drawShowNextLoc()
	case .NoState:
		WI_drawNoState()
	}
}

WI_initVariables :: proc(wbstartstruct: ^wbstartstruct_t) {
	wbs = wbstartstruct

	acceleratestage = 0
	cnt = 0
	bcnt = 0
	firstrefresh = 1
	me = wbs.pnum
	plrs = &wbs.plyr[0]

	if wbs.maxkills == 0 {
		wbs.maxkills = 1
	}
	if wbs.maxitems == 0 {
		wbs.maxitems = 1
	}
	if wbs.maxsecret == 0 {
		wbs.maxsecret = 1
	}

	if gamemode != .retail {
		if wbs.epsd > 2 {
			wbs.epsd -= 3
		}
	}
}

WI_Start :: proc(wbstartstruct: ^wbstartstruct_t) {
	WI_initVariables(wbstartstruct)
	WI_loadData()

	if deathmatch != 0 {
		WI_initDeathmatchStats()
	} else if netgame {
		WI_initNetgameStats()
	} else {
		WI_initStats()
	}
}