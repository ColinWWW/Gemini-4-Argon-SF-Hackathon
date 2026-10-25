// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

ST_HEIGHT :: 32 * SCREEN_MUL
ST_WIDTH  :: SCREENWIDTH
ST_Y      :: SCREENHEIGHT - ST_HEIGHT

st_stateenum_t :: enum i32 {
	AutomapState,
	FirstPersonState,
}

st_chatstateenum_t :: enum i32 {
	StartChatState,
	WaitDestState,
	GetChatState,
}
