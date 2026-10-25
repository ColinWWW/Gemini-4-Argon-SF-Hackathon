// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

T_FireFlicker :: proc(flick_raw: rawptr) {
	flick := (^fireflicker_t)(flick_raw)
	flick.count -= 1
	if flick.count != 0 {
		return
	}

	amount := (P_Random() & 3) * 16

	if int(flick.sector.lightlevel) - amount < flick.minlight {
		flick.sector.lightlevel = i16(flick.minlight)
	} else {
		flick.sector.lightlevel = i16(flick.maxlight - amount)
	}

	flick.count = 4
}

P_SpawnFireFlicker :: proc(sector: ^sector_t) {
	sector.special = 0

	flick := (^fireflicker_t)(Z_Malloc(size_of(fireflicker_t), PU_LEVSPEC, nil))
	P_AddThinker(&flick.thinker)

	flick.thinker.function.acp1 = T_FireFlicker
	flick.sector = sector
	flick.maxlight = int(sector.lightlevel)
	flick.minlight = P_FindMinSurroundingLight(sector, int(sector.lightlevel)) + 16
	flick.count = 4
}

T_LightFlash :: proc(flash_raw: rawptr) {
	flash := (^lightflash_t)(flash_raw)
	flash.count -= 1
	if flash.count != 0 {
		return
	}

	if int(flash.sector.lightlevel) == flash.maxlight {
		flash.sector.lightlevel = i16(flash.minlight)
		flash.count = (P_Random() & flash.mintime) + 1
	} else {
		flash.sector.lightlevel = i16(flash.maxlight)
		flash.count = (P_Random() & flash.maxtime) + 1
	}
}

P_SpawnLightFlash :: proc(sector: ^sector_t) {
	sector.special = 0

	flash := (^lightflash_t)(Z_Malloc(size_of(lightflash_t), PU_LEVSPEC, nil))
	P_AddThinker(&flash.thinker)

	flash.thinker.function.acp1 = T_LightFlash
	flash.sector = sector
	flash.maxlight = int(sector.lightlevel)
	flash.minlight = P_FindMinSurroundingLight(sector, int(sector.lightlevel))
	flash.maxtime = 64
	flash.mintime = 7
	flash.count = (P_Random() & flash.maxtime) + 1
}

T_StrobeFlash :: proc(flash_raw: rawptr) {
	flash := (^strobe_t)(flash_raw)
	flash.count -= 1
	if flash.count != 0 {
		return
	}

	if int(flash.sector.lightlevel) == flash.minlight {
		flash.sector.lightlevel = i16(flash.maxlight)
		flash.count = flash.brighttime
	} else {
		flash.sector.lightlevel = i16(flash.minlight)
		flash.count = flash.darktime
	}
}

P_SpawnStrobeFlash :: proc(sector: ^sector_t, fastOrSlow: int, inSync: int) {
	flash := (^strobe_t)(Z_Malloc(size_of(strobe_t), PU_LEVSPEC, nil))
	P_AddThinker(&flash.thinker)

	flash.sector = sector
	flash.darktime = fastOrSlow
	flash.brighttime = STROBEBRIGHT
	flash.thinker.function.acp1 = T_StrobeFlash
	flash.maxlight = int(sector.lightlevel)
	flash.minlight = P_FindMinSurroundingLight(sector, int(sector.lightlevel))

	if flash.minlight == flash.maxlight {
		flash.minlight = 0
	}

	sector.special = 0

	if inSync == 0 {
		flash.count = (P_Random() & 7) + 1
	} else {
		flash.count = 1
	}
}

EV_StartLightStrobing :: proc(line: ^line_t) {
	secnum := -1
	for {
		secnum = P_FindSectorFromLineTag(line, secnum)
		if secnum < 0 {
			break
		}
		sec := &sectors[secnum]
		if sec.specialdata != nil {
			continue
		}
		P_SpawnStrobeFlash(sec, SLOWDARK, 0)
	}
}

EV_TurnTagLightsOff :: proc(line: ^line_t) {
	for j := 0; j < numsectors; j += 1 {
		sector := &sectors[j]
		if sector.tag == line.tag {
			min_light := sector.lightlevel
			for i := 0; i < sector.linecount; i += 1 {
				templine := sector.lines[i]
				tsec := getNextSector(templine, sector)
				if tsec == nil {
					continue
				}
				if tsec.lightlevel < min_light {
					min_light = tsec.lightlevel
				}
			}
			sector.lightlevel = min_light
		}
	}
}

EV_LightTurnOn :: proc(line: ^line_t, bright_in: int) {
	bright := bright_in
	for i := 0; i < numsectors; i += 1 {
		sector := &sectors[i]
		if sector.tag == line.tag {
			if bright == 0 {
				for j := 0; j < sector.linecount; j += 1 {
					templine := sector.lines[j]
					temp := getNextSector(templine, sector)
					if temp == nil {
						continue
					}
					if int(temp.lightlevel) > bright {
						bright = int(temp.lightlevel)
					}
				}
			}
			sector.lightlevel = i16(bright)
		}
	}
}

T_Glow :: proc(g_raw: rawptr) {
	g := (^glow_t)(g_raw)
	switch g.direction {
	case -1:
		g.sector.lightlevel -= GLOWSPEED
		if int(g.sector.lightlevel) <= g.minlight {
			g.sector.lightlevel += GLOWSPEED
			g.direction = 1
		}
	case 1:
		g.sector.lightlevel += GLOWSPEED
		if int(g.sector.lightlevel) >= g.maxlight {
			g.sector.lightlevel -= GLOWSPEED
			g.direction = -1
		}
	}
}

P_SpawnGlowingLight :: proc(sector: ^sector_t) {
	g := (^glow_t)(Z_Malloc(size_of(glow_t), PU_LEVSPEC, nil))
	P_AddThinker(&g.thinker)

	g.sector = sector
	g.minlight = P_FindMinSurroundingLight(sector, int(sector.lightlevel))
	g.maxlight = int(sector.lightlevel)
	g.thinker.function.acp1 = T_Glow
	g.direction = -1

	sector.special = 0
}
