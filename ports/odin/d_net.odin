// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

NCMD_EXIT       :: 0x80000000
NCMD_RETRANSMIT :: 0x40000000
NCMD_SETUP      :: 0x20000000
NCMD_KILL       :: 0x10000000
NCMD_CHECKSUM   :: 0x0fffffff

doomcom: ^doomcom_t
netbuffer: ^doomdata_t

RESENDCOUNT :: 10
PL_DRONE    :: 0x80

localcmds: [BACKUPTICS]ticcmd_t
netcmds: [MAXPLAYERS][BACKUPTICS]ticcmd_t
nettics: [MAXNETNODES]int
nodeingame: [MAXNETNODES]boolean
remoteresend: [MAXNETNODES]boolean
resendto: [MAXNETNODES]int
resendcount: [MAXNETNODES]int
nodeforplayer: [MAXPLAYERS]int

maketic: int
lastnettic: int
skiptics: int
ticdup: int
maxsend: int

reboundpacket: boolean
reboundstore: doomdata_t

exitmsg: [80]u8
gametime: int

frametics: [4]int
frameon: int
frameskip: [4]boolean
oldnettics: int
oldentertics: int

NetbufferSize :: proc() -> int {
	return int(offset_of(doomdata_t, cmds)) + int(netbuffer.numtics) * size_of(ticcmd_t)
}

NetbufferChecksum :: proc() -> u32 {
	return 0
}

ExpandTics :: proc(low: int) -> int {
	delta := low - (maketic & 0xff)
	if delta >= -64 && delta <= 64 {
		return (maketic & ~int(0xff)) + low
	}
	if delta > 64 {
		return (maketic & ~int(0xff)) - 256 + low
	}
	if delta < -64 {
		return (maketic & ~int(0xff)) + 256 + low
	}
	I_Error("ExpandTics: strange value %i at maketic %i", i32(low), i32(maketic))
	return 0
}

HSendPacket :: proc(node: int, flags: u32) {
	netbuffer.checksum = NetbufferChecksum() | flags

	if node == 0 {
		reboundstore = netbuffer^
		reboundpacket = true
		return
	}

	if demoplayback {
		return
	}

	if !netgame {
		I_Error("Tried to transmit to another node")
	}

	doomcom.command = i16(CMD_SEND)
	doomcom.remotenode = i16(node)
	doomcom.datalength = i16(NetbufferSize())

	if debugfile != nil {
		realretrans: int
		if (netbuffer.checksum & NCMD_RETRANSMIT) != 0 {
			realretrans = ExpandTics(int(netbuffer.retransmitfrom))
		} else {
			realretrans = -1
		}
		libc.fprintf(debugfile, "send (%i + %i, R %i) [%i] ", i32(ExpandTics(int(netbuffer.starttic))), i32(netbuffer.numtics), i32(realretrans), i32(doomcom.datalength))
		nb_bytes := ([^]byte)(netbuffer)
		for i := 0; i < int(doomcom.datalength); i += 1 {
			libc.fprintf(debugfile, "%i ", i32(nb_bytes[i]))
		}
		libc.fprintf(debugfile, "\n")
	}

	I_NetCmd()
}

HGetPacket :: proc() -> boolean {
	if reboundpacket {
		netbuffer^ = reboundstore
		doomcom.remotenode = 0
		reboundpacket = false
		return true
	}

	if !netgame {
		return false
	}

	if demoplayback {
		return false
	}

	doomcom.command = i16(CMD_GET)
	I_NetCmd()

	if doomcom.remotenode == -1 {
		return false
	}

	if int(doomcom.datalength) != NetbufferSize() {
		if debugfile != nil {
			libc.fprintf(debugfile, "bad packet length %i\n", i32(doomcom.datalength))
		}
		return false
	}

	if NetbufferChecksum() != (netbuffer.checksum & NCMD_CHECKSUM) {
		if debugfile != nil {
			libc.fprintf(debugfile, "bad packet checksum\n")
		}
		return false
	}

	if debugfile != nil {
		if (netbuffer.checksum & NCMD_SETUP) != 0 {
			libc.fprintf(debugfile, "setup packet\n")
		} else {
			realretrans: int
			if (netbuffer.checksum & NCMD_RETRANSMIT) != 0 {
				realretrans = ExpandTics(int(netbuffer.retransmitfrom))
			} else {
				realretrans = -1
			}
			libc.fprintf(debugfile, "get %i = (%i + %i, R %i)[%i] ", i32(doomcom.remotenode), i32(ExpandTics(int(netbuffer.starttic))), i32(netbuffer.numtics), i32(realretrans), i32(doomcom.datalength))
			nb_bytes := ([^]byte)(netbuffer)
			for i := 0; i < int(doomcom.datalength); i += 1 {
				libc.fprintf(debugfile, "%i ", i32(nb_bytes[i]))
			}
			libc.fprintf(debugfile, "\n")
		}
	}
	return true
}

GetPackets :: proc() {
	for HGetPacket() {
		if (netbuffer.checksum & NCMD_SETUP) != 0 {
			continue
		}

		netconsole := int(netbuffer.player & ~u8(PL_DRONE))
		netnode := int(doomcom.remotenode)

		realstart := ExpandTics(int(netbuffer.starttic))
		realend := realstart + int(netbuffer.numtics)

		if (netbuffer.checksum & NCMD_EXIT) != 0 {
			if !nodeingame[netnode] {
				continue
			}
			nodeingame[netnode] = false
			playeringame[netconsole] = false
			libc.strcpy(&exitmsg[0], "Player 1 left the game")
			exitmsg[7] += u8(netconsole)
			players[consoleplayer].message = cstring(&exitmsg[0])
			if demorecording {
				G_CheckDemoStatus()
			}
			continue
		}

		if (netbuffer.checksum & NCMD_KILL) != 0 {
			I_Error("Killed by network driver")
		}

		nodeforplayer[netconsole] = netnode

		if resendcount[netnode] <= 0 && (netbuffer.checksum & NCMD_RETRANSMIT) != 0 {
			resendto[netnode] = ExpandTics(int(netbuffer.retransmitfrom))
			if debugfile != nil {
				libc.fprintf(debugfile, "retransmit from %i\n", i32(resendto[netnode]))
			}
			resendcount[netnode] = RESENDCOUNT
		} else {
			resendcount[netnode] -= 1
		}

		if realend == nettics[netnode] {
			continue
		}

		if realend < nettics[netnode] {
			if debugfile != nil {
				libc.fprintf(debugfile, "out of order packet (%i + %i)\n", i32(realstart), i32(netbuffer.numtics))
			}
			continue
		}

		if realstart > nettics[netnode] {
			if debugfile != nil {
				libc.fprintf(debugfile, "missed tics from %i (%i - %i)\n", i32(netnode), i32(realstart), i32(nettics[netnode]))
			}
			remoteresend[netnode] = true
			continue
		}

		remoteresend[netnode] = false
		start := nettics[netnode] - realstart
		src_idx := start
		for nettics[netnode] < realend {
			dest := &netcmds[netconsole][nettics[netnode] % BACKUPTICS]
			nettics[netnode] += 1
			dest^ = netbuffer.cmds[src_idx]
			src_idx += 1
		}
	}
}

NetUpdate :: proc() {
	nowtime := I_GetTime() / ticdup
	newtics := nowtime - gametime
	gametime = nowtime

	if newtics > 0 {
		if skiptics <= newtics {
			newtics -= skiptics
			skiptics = 0
		} else {
			skiptics -= newtics
			newtics = 0
		}

		netbuffer.player = byte(consoleplayer)

		gameticdiv := gametic / ticdup
		for i := 0; i < newtics; i += 1 {
			I_StartTic()
			D_ProcessEvents()
			if maketic - gameticdiv >= BACKUPTICS / 2 - 1 {
				break
			}
			G_BuildTiccmd(&localcmds[maketic % BACKUPTICS])
			maketic += 1
		}

		if singletics {
			return
		}

		for i := 0; i < int(doomcom.numnodes); i += 1 {
			if nodeingame[i] {
				realstart := resendto[i]
				netbuffer.starttic = byte(realstart)
				netbuffer.numtics = byte(maketic - realstart)
				if int(netbuffer.numtics) > BACKUPTICS {
					I_Error("NetUpdate: netbuffer->numtics > BACKUPTICS")
				}

				resendto[i] = maketic - int(doomcom.extratics)

				for j := 0; j < int(netbuffer.numtics); j += 1 {
					netbuffer.cmds[j] = localcmds[(realstart + j) % BACKUPTICS]
				}

				if remoteresend[i] {
					netbuffer.retransmitfrom = byte(nettics[i])
					HSendPacket(i, NCMD_RETRANSMIT)
				} else {
					netbuffer.retransmitfrom = 0
					HSendPacket(i, 0)
				}
			}
		}
	}

	GetPackets()
}

CheckAbort :: proc() {
	stoptic := I_GetTime() + 2
	for I_GetTime() < stoptic {
		I_StartTic()
	}

	I_StartTic()
	for ; eventtail != eventhead; eventtail = (eventtail + 1) & (MAXEVENTS - 1) {
		ev := &events[eventtail]
		if ev.type == .ev_keydown && ev.data1 == KEY_ESCAPE {
			I_Error("Network game synchronization aborted.")
		}
	}
}

D_ArbitrateNetStart :: proc() {
	gotinfo: [MAXNETNODES]boolean
	autostart = true

	if doomcom.consoleplayer != 0 {
		libc.printf("listening for network start info...\n")
		for {
			CheckAbort()
			if !HGetPacket() {
				continue
			}
			if (netbuffer.checksum & NCMD_SETUP) != 0 {
				if int(netbuffer.player) != VERSION {
					I_Error("Different DOOM versions cannot play a net game!")
				}
				startskill = skill_t(netbuffer.retransmitfrom & 15)
				deathmatch = int((netbuffer.retransmitfrom & 0xc0) >> 6)
				nomonsters = (netbuffer.retransmitfrom & 0x20) > 0
				respawnparm = (netbuffer.retransmitfrom & 0x10) > 0
				startmap = int(netbuffer.starttic & 0x3f)
				startepisode = int(netbuffer.starttic >> 6)
				return
			}
		}
	} else {
		libc.printf("sending network start info...\n")
		for {
			CheckAbort()
			for i := 0; i < int(doomcom.numnodes); i += 1 {
				netbuffer.retransmitfrom = byte(startskill)
				if deathmatch != 0 {
					netbuffer.retransmitfrom |= byte(deathmatch << 6)
				}
				if nomonsters {
					netbuffer.retransmitfrom |= 0x20
				}
				if respawnparm {
					netbuffer.retransmitfrom |= 0x10
				}
				netbuffer.starttic = byte(startepisode * 64 + startmap)
				netbuffer.player = byte(VERSION)
				netbuffer.numtics = 0
				HSendPacket(i, NCMD_SETUP)
			}

			for i := 10; i != 0 && HGetPacket(); i -= 1 {
				if (netbuffer.player & 0x7f) < MAXNETNODES {
					gotinfo[netbuffer.player & 0x7f] = true
				}
			}

			i := 1
			for ; i < int(doomcom.numnodes); i += 1 {
				if !gotinfo[i] {
					break
				}
			}
			if i >= int(doomcom.numnodes) {
				break
			}
		}
	}
}

D_CheckNetGame :: proc() {
	for i := 0; i < MAXNETNODES; i += 1 {
		nodeingame[i] = false
		nettics[i] = 0
		remoteresend[i] = false
		resendto[i] = 0
	}

	I_InitNetwork()
	if doomcom.id != DOOMCOM_ID {
		I_Error("Doomcom buffer invalid!")
	}

	netbuffer = &doomcom.data
	consoleplayer = int(doomcom.consoleplayer)
	displayplayer = consoleplayer
	if netgame {
		D_ArbitrateNetStart()
	}

	libc.printf("startskill %i  deathmatch: %i  startmap: %i  startepisode: %i\n", i32(startskill), i32(deathmatch), i32(startmap), i32(startepisode))

	ticdup = int(doomcom.ticdup)
	maxsend = BACKUPTICS / (2 * ticdup) - 1
	if maxsend < 1 {
		maxsend = 1
	}

	for i := 0; i < int(doomcom.numplayers); i += 1 {
		playeringame[i] = true
	}
	for i := 0; i < int(doomcom.numnodes); i += 1 {
		nodeingame[i] = true
	}

	libc.printf("player %i of %i (%i nodes)\n", i32(consoleplayer + 1), i32(doomcom.numplayers), i32(doomcom.numnodes))
}

D_QuitNetGame :: proc() {
	if debugfile != nil {
		libc.fclose(debugfile)
	}

	if !netgame || !usergame || consoleplayer == -1 || demoplayback {
		return
	}

	netbuffer.player = byte(consoleplayer)
	netbuffer.numtics = 0
	for i := 0; i < 4; i += 1 {
		for j := 1; j < int(doomcom.numnodes); j += 1 {
			if nodeingame[j] {
				HSendPacket(j, NCMD_EXIT)
			}
		}
		I_WaitVBL(1)
	}
}

TryRunTics :: proc() {
	entertic := I_GetTime() / ticdup
	realtics := entertic - oldentertics
	oldentertics = entertic

	NetUpdate()

	lowtic := MAXINT
	numplaying := 0
	for i := 0; i < int(doomcom.numnodes); i += 1 {
		if nodeingame[i] {
			numplaying += 1
			if nettics[i] < lowtic {
				lowtic = nettics[i]
			}
		}
	}
	availabletics := lowtic - gametic / ticdup

	counts: int
	if realtics < availabletics - 1 {
		counts = realtics + 1
	} else if realtics < availabletics {
		counts = realtics
	} else {
		counts = availabletics
	}

	if counts < 1 {
		counts = 1
	}

	frameon += 1

	if debugfile != nil {
		libc.fprintf(debugfile, "=======real: %i  avail: %i  game: %i\n", i32(realtics), i32(availabletics), i32(counts))
	}

	if !demoplayback {
		i := 0
		for ; i < MAXPLAYERS; i += 1 {
			if playeringame[i] {
				break
			}
		}
		if consoleplayer != i {
			if nettics[0] <= nettics[nodeforplayer[i]] {
				gametime -= 1
			}
			frameskip[frameon & 3] = (oldnettics > nettics[nodeforplayer[i]])
			oldnettics = nettics[0]
			if frameskip[0] && frameskip[1] && frameskip[2] && frameskip[3] {
				skiptics = 1
			}
		}
	}

	for lowtic < gametic / ticdup + counts {
		NetUpdate()
		lowtic = MAXINT

		for i := 0; i < int(doomcom.numnodes); i += 1 {
			if nodeingame[i] && nettics[i] < lowtic {
				lowtic = nettics[i]
			}
		}

		if lowtic < gametic / ticdup {
			I_Error("TryRunTics: lowtic < gametic")
		}

		if I_GetTime() / ticdup - entertic >= 20 {
			M_Ticker()
			return
		}
	}

	for counts > 0 {
		counts -= 1
		for i := 0; i < ticdup; i += 1 {
			if gametic / ticdup > lowtic {
				I_Error("gametic>lowtic")
			}
			if advancedemo {
				D_DoAdvanceDemo()
			}
			M_Ticker()
			G_Ticker()
			gametic += 1

			if i != ticdup - 1 {
				buf := (gametic / ticdup) % BACKUPTICS
				for j := 0; j < MAXPLAYERS; j += 1 {
					cmd := &netcmds[j][buf]
					cmd.chatchar = 0
					if (cmd.buttons & BT_SPECIAL) != 0 {
						cmd.buttons = 0
					}
				}
			}
		}
		NetUpdate()
	}
}
