// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

AM_MSGHEADER  :: (int('a') << 24) + (int('m') << 16)
AM_MSGENTERED :: AM_MSGHEADER | (int('e') << 8)
AM_MSGEXITED  :: AM_MSGHEADER | (int('x') << 8)
