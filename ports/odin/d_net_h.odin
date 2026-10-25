// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

DOOMCOM_ID  :: 0x12345678
MAXNETNODES :: 8
BACKUPTICS  :: 12

CMD_SEND :: 1
CMD_GET  :: 2

doomdata_t :: struct {
	checksum:       u32,
	retransmitfrom: byte,
	starttic:       byte,
	player:         byte,
	numtics:        byte,
	cmds:           [BACKUPTICS]ticcmd_t,
}

doomcom_t :: struct {
	id:            i32,
	intnum:        i16,
	command:       i16,
	remotenode:    i16,
	datalength:    i16,
	numnodes:      i16,
	ticdup:        i16,
	extratics:     i16,
	deathmatch:    i16,
	savegame:      i16,
	episode:       i16,
	map_num:       i16,
	skill:         i16,
	consoleplayer: i16,
	numplayers:    i16,
	angleoffset:   i16,
	drone:         i16,
	data:          doomdata_t,
}
