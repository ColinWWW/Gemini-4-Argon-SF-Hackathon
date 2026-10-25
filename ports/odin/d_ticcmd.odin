// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

ticcmd_t :: struct {
	forwardmove: i8,
	sidemove:    i8,
	angleturn:   i16,
	consistancy: i16,
	chatchar:    byte,
	buttons:     byte,
}
