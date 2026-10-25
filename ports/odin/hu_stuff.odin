// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

HU_TITLEHEIGHT :: 1
HU_TITLEX      :: 0
HU_INPUTTOGGLE :: int('t')
HU_INPUTX      :: HU_MSGX
HU_INPUTWIDTH  :: 64
HU_INPUTHEIGHT :: 1

chat_macros := [10]cstring{
	HUSTR_CHATMACRO0,
	HUSTR_CHATMACRO1,
	HUSTR_CHATMACRO2,
	HUSTR_CHATMACRO3,
	HUSTR_CHATMACRO4,
	HUSTR_CHATMACRO5,
	HUSTR_CHATMACRO6,
	HUSTR_CHATMACRO7,
	HUSTR_CHATMACRO8,
	HUSTR_CHATMACRO9,
}

player_names := [4]cstring{
	HUSTR_PLRGREEN,
	HUSTR_PLRINDIGO,
	HUSTR_PLRBROWN,
	HUSTR_PLRRED,
}

chat_char: u8
hu_plr: ^player_t
hu_font: [HU_FONTSIZE]^patch_t
w_title: hu_textline_t
chat_on: boolean
w_chat: hu_itext_t
always_off: boolean = false
chat_dest: [MAXPLAYERS]u8
w_inputbuffer: [MAXPLAYERS]hu_itext_t

message_on: boolean
message_dontfuckwithme: boolean
message_nottobefuckedwith: boolean

w_message: hu_stext_t
message_counter: int

headsupactive: boolean = false

mapnames := [45]cstring{
	HUSTR_E1M1, HUSTR_E1M2, HUSTR_E1M3, HUSTR_E1M4, HUSTR_E1M5, HUSTR_E1M6, HUSTR_E1M7, HUSTR_E1M8, HUSTR_E1M9,
	HUSTR_E2M1, HUSTR_E2M2, HUSTR_E2M3, HUSTR_E2M4, HUSTR_E2M5, HUSTR_E2M6, HUSTR_E2M7, HUSTR_E2M8, HUSTR_E2M9,
	HUSTR_E3M1, HUSTR_E3M2, HUSTR_E3M3, HUSTR_E3M4, HUSTR_E3M5, HUSTR_E3M6, HUSTR_E3M7, HUSTR_E3M8, HUSTR_E3M9,
	HUSTR_E4M1, HUSTR_E4M2, HUSTR_E4M3, HUSTR_E4M4, HUSTR_E4M5, HUSTR_E4M6, HUSTR_E4M7, HUSTR_E4M8, HUSTR_E4M9,
	"NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL",
}

mapnames2 := [32]cstring{
	HUSTR_1, HUSTR_2, HUSTR_3, HUSTR_4, HUSTR_5, HUSTR_6, HUSTR_7, HUSTR_8, HUSTR_9, HUSTR_10, HUSTR_11,
	HUSTR_12, HUSTR_13, HUSTR_14, HUSTR_15, HUSTR_16, HUSTR_17, HUSTR_18, HUSTR_19, HUSTR_20,
	HUSTR_21, HUSTR_22, HUSTR_23, HUSTR_24, HUSTR_25, HUSTR_26, HUSTR_27, HUSTR_28, HUSTR_29, HUSTR_30, HUSTR_31, HUSTR_32,
}

mapnamesp := [32]cstring{
	PHUSTR_1, PHUSTR_2, PHUSTR_3, PHUSTR_4, PHUSTR_5, PHUSTR_6, PHUSTR_7, PHUSTR_8, PHUSTR_9, PHUSTR_10, PHUSTR_11,
	PHUSTR_12, PHUSTR_13, PHUSTR_14, PHUSTR_15, PHUSTR_16, PHUSTR_17, PHUSTR_18, PHUSTR_19, PHUSTR_20,
	PHUSTR_21, PHUSTR_22, PHUSTR_23, PHUSTR_24, PHUSTR_25, PHUSTR_26, PHUSTR_27, PHUSTR_28, PHUSTR_29, PHUSTR_30, PHUSTR_31, PHUSTR_32,
}

mapnamest := [32]cstring{
	THUSTR_1, THUSTR_2, THUSTR_3, THUSTR_4, THUSTR_5, THUSTR_6, THUSTR_7, THUSTR_8, THUSTR_9, THUSTR_10, THUSTR_11,
	THUSTR_12, THUSTR_13, THUSTR_14, THUSTR_15, THUSTR_16, THUSTR_17, THUSTR_18, THUSTR_19, THUSTR_20,
	THUSTR_21, THUSTR_22, THUSTR_23, THUSTR_24, THUSTR_25, THUSTR_26, THUSTR_27, THUSTR_28, THUSTR_29, THUSTR_30, THUSTR_31, THUSTR_32,
}

shiftxform: [^]u8

french_shiftxform := [128]u8{
	0,
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
	11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
	21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
	31,
	' ', '!', '"', '#', '$', '%', '&',
	'"',
	'(', ')', '*', '+',
	'?',
	'_',
	'>',
	'?',
	'0',
	'1',
	'2',
	'3',
	'4',
	'5',
	'6',
	'7',
	'8',
	'9',
	'/',
	'.',
	'<',
	'+',
	'>', '?', '@',
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
	'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	'[',
	'!',
	']',
	'"', '_',
	'\'',
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
	'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	'{', '|', '}', '~', 127,
}

english_shiftxform := [128]u8{
	0,
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
	11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
	21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
	31,
	' ', '!', '"', '#', '$', '%', '&',
	'"',
	'(', ')', '*', '+',
	'<',
	'_',
	'>',
	'?',
	')',
	'!',
	'@',
	'#',
	'$',
	'%',
	'^',
	'&',
	'*',
	'(',
	':',
	':',
	'<',
	'+',
	'>', '?', '@',
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
	'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	'[',
	'!',
	']',
	'"', '_',
	'\'',
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
	'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	'{', '|', '}', '~', 127,
}

frenchKeyMap := [128]u8{
	0,
	1,2,3,4,5,6,7,8,9,10,
	11,12,13,14,15,16,17,18,19,20,
	21,22,23,24,25,26,27,28,29,30,
	31,
	' ','!','"','#','$','%','&','%','(',')','*','+',';','-',':','!',
	'0','1','2','3','4','5','6','7','8','9',':','M','<','=','>','?',
	'@','Q','B','C','D','E','F','G','H','I','J','K','L',',','N','O',
	'P','A','R','S','T','U','V','Z','X','Y','W','^','\\','$','^','_',
	'@','Q','B','C','D','E','F','G','H','I','J','K','L',',','N','O',
	'P','A','R','S','T','U','V','Z','X','Y','W','^','\\','$','^',127,
}

QUEUESIZE :: 128
chatchars: [QUEUESIZE]u8
chat_head: int = 0
chat_tail: int = 0

hu_lastmessage: [HU_MAXLINELENGTH + 1]u8
hu_shiftdown: boolean = false
hu_altdown: boolean = false
hu_destination_keys := [MAXPLAYERS]u8{
	HUSTR_KEYGREEN,
	HUSTR_KEYINDIGO,
	HUSTR_KEYBROWN,
	HUSTR_KEYRED,
}
hu_num_nobrainers: int = 0

ForeignTranslation :: proc(ch: u8) -> u8 {
	return ch < 128 ? frenchKeyMap[ch] : ch
}

HU_Init :: proc() {
	buffer: [9]u8

	// Note: in original C `if (french)` evaluated enum constant `french` (which is 1, true!)
	shiftxform = &french_shiftxform[0]

	j := int(HU_FONTSTART)
	for i := 0; i < HU_FONTSIZE; i += 1 {
		libc.snprintf(&buffer[0], len(buffer), "STCFN%.3d", i32(j))
		j += 1
		hu_font[i] = (^patch_t)(W_CacheLumpName(cstring(&buffer[0]), PU_STATIC))
	}
}

HU_Stop :: proc() {
	headsupactive = false
}

HU_Start :: proc() {
	if headsupactive {
		HU_Stop()
	}

	hu_plr = &players[consoleplayer]
	message_on = false
	message_dontfuckwithme = false
	message_nottobefuckedwith = false
	chat_on = false

	HUlib_initSText(&w_message, HU_MSGX, HU_MSGY, HU_MSGHEIGHT, &hu_font[0], int(HU_FONTSTART), &message_on)

	hu_titley := 167 - int(SHORT(hu_font[0].height))
	HUlib_initTextLine(&w_title, HU_TITLEX, hu_titley, &hu_font[0], int(HU_FONTSTART))

	s: cstring
	switch gamemode {
	case .shareware, .registered, .retail:
		s = mapnames[(gameepisode - 1) * 9 + gamemap - 1]
	case .commercial, .indetermined:
		s = mapnames2[gamemap - 1]
	}

	sp := ([^]u8)(s)
	for sp[0] != 0 {
		HUlib_addCharToTextLine(&w_title, sp[0])
		sp = &sp[1]
	}

	hu_inputy := HU_MSGY + HU_MSGHEIGHT * (int(SHORT(hu_font[0].height)) + 1)
	HUlib_initIText(&w_chat, HU_INPUTX, hu_inputy, &hu_font[0], int(HU_FONTSTART), &chat_on)

	for i := 0; i < MAXPLAYERS; i += 1 {
		HUlib_initIText(&w_inputbuffer[i], 0, 0, nil, 0, &always_off)
	}

	headsupactive = true
}

HU_Drawer :: proc() {
	HUlib_drawSText(&w_message)
	HUlib_drawIText(&w_chat)
	if automapactive {
		HUlib_drawTextLine(&w_title, false)
	}
}

HU_Erase :: proc() {
	HUlib_eraseSText(&w_message)
	HUlib_eraseIText(&w_chat)
	HUlib_eraseTextLine(&w_title)
}

HU_Ticker :: proc() {
	if message_counter != 0 {
		message_counter -= 1
		if message_counter == 0 {
			message_on = false
			message_nottobefuckedwith = false
		}
	}

	if showMessages != 0 || message_dontfuckwithme {
		if (hu_plr.message != nil && !message_nottobefuckedwith) || (hu_plr.message != nil && message_dontfuckwithme) {
			HUlib_addMessageToSText(&w_message, nil, hu_plr.message)
			hu_plr.message = nil
			message_on = true
			message_counter = HU_MSGTIMEOUT
			message_nottobefuckedwith = message_dontfuckwithme
			message_dontfuckwithme = false
		}
	}

	if netgame {
		for i := 0; i < MAXPLAYERS; i += 1 {
			if !playeringame[i] {
				continue
			}
			c := players[i].cmd.chatchar
			if i != consoleplayer && c != 0 {
				if c <= HU_BROADCAST {
					chat_dest[i] = c
				} else {
					if c >= 'a' && c <= 'z' {
						c = shiftxform[c]
					}
					rc := HUlib_keyInIText(&w_inputbuffer[i], c)
					if rc && c == KEY_ENTER {
						if w_inputbuffer[i].l.len != 0 && (int(chat_dest[i]) == consoleplayer + 1 || chat_dest[i] == HU_BROADCAST) {
							HUlib_addMessageToSText(&w_message, player_names[i], cstring(&w_inputbuffer[i].l.l[0]))
							message_nottobefuckedwith = true
							message_on = true
							message_counter = HU_MSGTIMEOUT
							if gamemode == .commercial {
								S_StartSound(nil, int(sfxenum_t.sfx_radio))
							} else {
								S_StartSound(nil, int(sfxenum_t.sfx_tink))
							}
						}
						HUlib_resetIText(&w_inputbuffer[i])
					}
				}
				players[i].cmd.chatchar = 0
			}
		}
	}
}

HU_queueChatChar :: proc(c: u8) {
	if ((chat_head + 1) & (QUEUESIZE - 1)) == chat_tail {
		hu_plr.message = HUSTR_MSGU
	} else {
		chatchars[chat_head] = c
		chat_head = (chat_head + 1) & (QUEUESIZE - 1)
	}
}

HU_dequeueChatChar :: proc() -> u8 {
	c: u8
	if chat_head != chat_tail {
		c = chatchars[chat_tail]
		chat_tail = (chat_tail + 1) & (QUEUESIZE - 1)
	} else {
		c = 0
	}
	return c
}

HU_Responder :: proc(ev: ^event_t) -> boolean {
	eatkey := false

	numplayers := 0
	for i := 0; i < MAXPLAYERS; i += 1 {
		if playeringame[i] {
			numplayers += 1
		}
	}

	if ev.data1 == KEY_RSHIFT {
		hu_shiftdown = ev.type == .ev_keydown
		return false
	} else if ev.data1 == KEY_RALT || ev.data1 == KEY_LALT {
		hu_altdown = ev.type == .ev_keydown
		return false
	}

	if ev.type != .ev_keydown {
		return false
	}

	if !chat_on {
		if ev.data1 == HU_MSGREFRESH {
			message_on = true
			message_counter = HU_MSGTIMEOUT
			eatkey = true
		} else if netgame && ev.data1 == HU_INPUTTOGGLE {
			eatkey = true
			chat_on = true
			HUlib_resetIText(&w_chat)
			HU_queueChatChar(HU_BROADCAST)
		} else if netgame && numplayers > 2 {
			for i := 0; i < MAXPLAYERS; i += 1 {
				if ev.data1 == int(hu_destination_keys[i]) {
					if playeringame[i] && i != consoleplayer {
						eatkey = true
						chat_on = true
						HUlib_resetIText(&w_chat)
						HU_queueChatChar(u8(i + 1))
						break
					} else if i == consoleplayer {
						hu_num_nobrainers += 1
						if hu_num_nobrainers < 3 {
							hu_plr.message = HUSTR_TALKTOSELF1
						} else if hu_num_nobrainers < 6 {
							hu_plr.message = HUSTR_TALKTOSELF2
						} else if hu_num_nobrainers < 9 {
							hu_plr.message = HUSTR_TALKTOSELF3
						} else if hu_num_nobrainers < 32 {
							hu_plr.message = HUSTR_TALKTOSELF4
						} else {
							hu_plr.message = HUSTR_TALKTOSELF5
						}
					}
				}
			}
		}
	} else {
		c := u8(ev.data1)
		if hu_altdown {
			c = c - '0'
			if c > 9 {
				return false
			}
			macromessage := ([^]u8)(chat_macros[c])
			HU_queueChatChar(KEY_ENTER)
			for macromessage[0] != 0 {
				HU_queueChatChar(macromessage[0])
				macromessage = &macromessage[1]
			}
			HU_queueChatChar(KEY_ENTER)
			chat_on = false
			libc.strcpy(&hu_lastmessage[0], chat_macros[c])
			hu_plr.message = cstring(&hu_lastmessage[0])
			eatkey = true
		} else {
			c = ForeignTranslation(c)
			if hu_shiftdown || (c >= 'a' && c <= 'z') {
				c = shiftxform[c]
			}
			eatkey = HUlib_keyInIText(&w_chat, c)
			if eatkey {
				HU_queueChatChar(c)
			}
			if c == KEY_ENTER {
				chat_on = false
				if w_chat.l.len != 0 {
					libc.strcpy(&hu_lastmessage[0], cstring(&w_chat.l.l[0]))
					hu_plr.message = cstring(&hu_lastmessage[0])
				}
			} else if c == KEY_ESCAPE {
				chat_on = false
			}
		}
	}

	return eatkey
}