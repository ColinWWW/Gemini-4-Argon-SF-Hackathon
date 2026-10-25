// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

actionf_v  :: #type proc()
actionf_p1 :: #type proc(p1: rawptr)
actionf_p2 :: #type proc(p1: rawptr, p2: rawptr)

actionf_t :: struct #raw_union {
	acp1: actionf_p1,
	acv:  actionf_v,
	acp2: actionf_p2,
	int_val: int,
}

think_t :: actionf_t

thinker_t :: struct {
	prev:     ^thinker_t,
	next:     ^thinker_t,
	function: think_t,
}
