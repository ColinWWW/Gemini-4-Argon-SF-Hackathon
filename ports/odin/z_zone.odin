// Based on id Software's linuxdoom-1.10 (GNU GPL)
package main

import "core:c/libc"

ZONEID :: 0x1d4a11

memzone_t :: struct {
	size:      int,
	blocklist: memblock_t,
	rover:     ^memblock_t,
}

mainzone: ^memzone_t

MINFRAGMENT :: 64

Z_ClearZone :: proc(zone: ^memzone_t) {
	block := (^memblock_t)(uintptr(zone) + size_of(memzone_t))
	zone.blocklist.next = block
	zone.blocklist.prev = block

	zone.blocklist.user = (^rawptr)(zone)
	zone.blocklist.tag = PU_STATIC
	zone.rover = block

	block.prev = &zone.blocklist
	block.next = &zone.blocklist
	block.user = nil
	block.size = zone.size - size_of(memzone_t)
}

Z_Init :: proc() {
	size: int
	mainzone = (^memzone_t)(I_ZoneBase(&size))
	mainzone.size = size

	block := (^memblock_t)(uintptr(mainzone) + size_of(memzone_t))
	mainzone.blocklist.next = block
	mainzone.blocklist.prev = block

	mainzone.blocklist.user = (^rawptr)(mainzone)
	mainzone.blocklist.tag = PU_STATIC
	mainzone.rover = block

	block.prev = &mainzone.blocklist
	block.next = &mainzone.blocklist
	block.user = nil
	block.size = mainzone.size - size_of(memzone_t)
}

Z_Free :: proc(ptr: rawptr) {
	block := (^memblock_t)(uintptr(ptr) - size_of(memblock_t))

	if block.id != ZONEID {
		I_Error("Z_Free: freed a pointer without ZONEID")
	}

	if uintptr(block.user) > 0x100 {
		block.user^ = nil
	}

	block.user = nil
	block.tag = 0
	block.id = 0

	other := block.prev
	if other.user == nil {
		other.size += block.size
		other.next = block.next
		other.next.prev = other

		if block == mainzone.rover {
			mainzone.rover = other
		}
		block = other
	}

	other = block.next
	if other.user == nil {
		block.size += other.size
		block.next = other.next
		block.next.prev = block

		if other == mainzone.rover {
			mainzone.rover = block
		}
	}
}

Z_Malloc :: proc(size_in: int, tag: int, user: rawptr) -> rawptr {
	size := (size_in + 3) & ~int(3)
	size += size_of(memblock_t)

	base := mainzone.rover
	if base.prev.user == nil {
		base = base.prev
	}

	rover := base
	start := base.prev

	for {
		if rover == start {
			I_Error("Z_Malloc: failed on allocation of %i bytes", i32(size))
		}

		if rover.user != nil {
			if rover.tag < PU_PURGELEVEL {
				base = rover.next
				rover = base
			} else {
				base = base.prev
				Z_Free(rawptr(uintptr(rover) + size_of(memblock_t)))
				base = base.next
				rover = base.next
			}
		} else {
			rover = rover.next
		}

		if base.user == nil && base.size >= size {
			break
		}
	}

	extra := base.size - size

	if extra > MINFRAGMENT {
		newblock := (^memblock_t)(uintptr(base) + uintptr(size))
		newblock.size = extra
		newblock.user = nil
		newblock.tag = 0
		newblock.prev = base
		newblock.next = base.next
		newblock.next.prev = newblock

		base.next = newblock
		base.size = size
	}

	if user != nil {
		base.user = (^rawptr)(user)
		(^rawptr)(user)^ = rawptr(uintptr(base) + size_of(memblock_t))
	} else {
		if tag >= PU_PURGELEVEL {
			I_Error("Z_Malloc: an owner is required for purgable blocks")
		}
		base.user = (^rawptr)(uintptr(2))
	}
	base.tag = tag

	mainzone.rover = base.next
	base.id = ZONEID

	return rawptr(uintptr(base) + size_of(memblock_t))
}

Z_FreeTags :: proc(lowtag: int, hightag: int) {
	block := mainzone.blocklist.next
	for block != &mainzone.blocklist {
		next := block.next
		if block.user != nil {
			if block.tag >= lowtag && block.tag <= hightag {
				Z_Free(rawptr(uintptr(block) + size_of(memblock_t)))
			}
		}
		block = next
	}
}

Z_DumpHeap :: proc(lowtag: int, hightag: int) {
	libc.printf("zone size: %i  location: %p\n", i32(mainzone.size), mainzone)
	libc.printf("tag range: %i to %i\n", i32(lowtag), i32(hightag))

	for block := mainzone.blocklist.next; ; block = block.next {
		if block.tag >= lowtag && block.tag <= hightag {
			libc.printf("block:%p    size:%7i    user:%p    tag:%3i\n", block, i32(block.size), block.user, i32(block.tag))
		}

		if block.next == &mainzone.blocklist {
			break
		}

		if uintptr(block) + uintptr(block.size) != uintptr(block.next) {
			libc.printf("ERROR: block size does not touch the next block\n")
		}
		if block.next.prev != block {
			libc.printf("ERROR: next block doesn't have proper back link\n")
		}
		if block.user == nil && block.next.user == nil {
			libc.printf("ERROR: two consecutive free blocks\n")
		}
	}
}

Z_FileDumpHeap :: proc(f: ^libc.FILE) {
	libc.fprintf(f, "zone size: %i  location: %p\n", i32(mainzone.size), mainzone)

	for block := mainzone.blocklist.next; ; block = block.next {
		libc.fprintf(f, "block:%p    size:%7i    user:%p    tag:%3i\n", block, i32(block.size), block.user, i32(block.tag))

		if block.next == &mainzone.blocklist {
			break
		}

		if uintptr(block) + uintptr(block.size) != uintptr(block.next) {
			libc.fprintf(f, "ERROR: block size does not touch the next block\n")
		}
		if block.next.prev != block {
			libc.fprintf(f, "ERROR: next block doesn't have proper back link\n")
		}
		if block.user == nil && block.next.user == nil {
			libc.fprintf(f, "ERROR: two consecutive free blocks\n")
		}
	}
}

Z_CheckHeap :: proc() {
	for block := mainzone.blocklist.next; ; block = block.next {
		if block.next == &mainzone.blocklist {
			break
		}

		if uintptr(block) + uintptr(block.size) != uintptr(block.next) {
			I_Error("Z_CheckHeap: block size does not touch the next block\n")
		}
		if block.next.prev != block {
			I_Error("Z_CheckHeap: next block doesn't have proper back link\n")
		}
		if block.user == nil && block.next.user == nil {
			I_Error("Z_CheckHeap: two consecutive free blocks\n")
		}
	}
}

Z_ChangeTag2 :: proc(ptr: rawptr, tag: int) {
	block := (^memblock_t)(uintptr(ptr) - size_of(memblock_t))

	if block.id != ZONEID {
		I_Error("Z_ChangeTag: freed a pointer without ZONEID")
	}

	if tag >= PU_PURGELEVEL && uintptr(block.user) < 0x100 {
		I_Error("Z_ChangeTag: an owner is required for purgable blocks")
	}

	block.tag = tag
}

Z_ChangeTag :: proc(ptr: rawptr, tag: int) {
	block := (^memblock_t)(uintptr(ptr) - size_of(memblock_t))
	if block.id != 0x1d4a11 {
		I_Error("Z_CT")
	}
	Z_ChangeTag2(ptr, tag)
}

Z_FreeMemory :: proc() -> int {
	free := 0
	for block := mainzone.blocklist.next; block != &mainzone.blocklist; block = block.next {
		if block.user == nil || block.tag >= PU_PURGELEVEL {
			free += block.size
		}
	}
	return free
}
