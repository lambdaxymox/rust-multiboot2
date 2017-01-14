#![allow(dead_code)]
use core::mem;

/// A Multiboot tag structure is just a queryable blob of bytes. The implementation presently
/// assumes that the size is at least 8 bytes (for the end tag), and does not check this.
const BOOT_COMMAND_LINE: u32   = 1;
const BOOT_LOADER_NAME: u32    = 2;
const MODULE: u32            = 3;
const MEMORY_INFORMATION: u32 = 4;
const BIOS_BOOT_DEVICE: u32    = 5;
const MEMORY_MAP: u32         = 6;
const VBE_INFO: u32           = 7;
const FRAME_BUFFER_INFO: u32   = 8;
const ELF_SYMBOLS: u32        = 9;
const APM_TABLE: u32          = 10;


#[repr(C)]
struct Tag {
	// Pointer to the first byte of the Tag block.
	ptr: *const u8,
}

impl Tag {
	fn tag_type(&self) -> u32 {
		let ptr = self.ptr as *const u32;
		unsafe {
			*ptr
		}
	}

	fn size(&self) -> u32 {
		let ptr = ((self.ptr as usize) + 4) as *const u32;
		unsafe {
			*ptr
		}
	}
}