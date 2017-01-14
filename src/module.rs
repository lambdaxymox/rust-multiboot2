use tag::TagType;


/// A module tag indicates to the kernel what boot was module was loaded along
/// with the kernel image, and where it is located. One tag appears per module, 
/// and there may be multiple module tags in a given multiboot info table.
#[repr(C, packed)]
struct ModuleTag {
	tag_type: u32,
	size: u32,
	/// The starting address of the boot module.
	mod_start: u32,
	/// The ending address of the boot module.
	mod_end: u32,    
	/// This is the first byte of the string.
	string: u8,
}

impl ModuleTag {
	fn string(&self) -> &str {
		use core::{mem, str, slice};

		let length = self.size as usize - mem::size_of::<ModuleTag>();
		unsafe {
			let byte_slice = slice::from_raw_parts((&self.string) as *const u8, length);

			str::from_utf8_unchecked(byte_slice)
		}
	}

	fn start_address(&self) -> u32 {
		self.mod_start
	}

	fn end_address(&self) -> u32 {
		self.mod_end
	}

	fn is_valid(&self) -> bool {
		self.tag_type == TagType::Module as u32
	}
}