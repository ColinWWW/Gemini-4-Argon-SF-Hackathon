// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

FixedMul :: #force_inline proc(a: fixed_t, b: fixed_t) -> fixed_t {
	return fixed_t((i64(a) * i64(b)) >> FRACBITS)
}

FixedDiv :: #force_inline proc(a: fixed_t, b: fixed_t) -> fixed_t {
	if (abs(a) >> 14) >= abs(b) {
		return (a ~ b) < 0 ? fixed_t(MININT) : fixed_t(MAXINT)
	}
	return FixedDiv2(a, b)
}

FixedDiv2 :: #force_inline proc(a: fixed_t, b: fixed_t) -> fixed_t {
	c := (f64(a) / f64(b)) * f64(FRACUNIT)
	if c >= 2147483648.0 || c < -2147483648.0 {
		I_Error("FixedDiv: divide by zero")
	}
	return fixed_t(c)
}
