// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "base:runtime"

main :: proc() {
	myargc = len(runtime.args__)
	myargv = raw_data(runtime.args__)
	D_DoomMain()
}
