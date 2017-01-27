#![allow(dead_code)]
use core::mem;

/// A Multiboot tag structure is a queryable blob of bytes. The implementation presently
/// assumes that the size is at least 8 bytes (for the end tag), and does not check this.
#[derive(Copy, Clone, Debug)]
pub enum TagType {
	EndTag            = 0,
	BootCommandLine   = 1,
	BootLoaderName    = 2,
	Module            = 3,
	MemoryInformation = 4,
	BIOSBootDevice    = 5,
	MemoryMap         = 6,
	VBEInfo           = 7,
	FrameBufferInfo   = 8,
	ElfSymbols        = 9,
	APMTable          = 10,
}


#[derive(Debug)]
#[repr(C)]
pub struct Tag {
	tag_type: u32,
	size: u32,
	// The tag data follows these two fields.
}

impl Tag {
	fn is_end_tag(&self) -> bool {
		self.tag_type == TagType::EndTag as u32
	}

	pub fn size(&self) -> usize {
		self.size as usize
	}

	pub fn tag_type(&self) -> usize {
		self.tag_type as usize
	}

	/// The function `cast` casts a generic `Tag` to a particular
	/// multiboot header tag. This function is really dangerous and 
	/// should only be used for parsing the multiboot info struct.
	pub unsafe fn cast<T>(&self) -> &T {
		mem::transmute::<&Tag, &T>(self)
	}
}

pub struct TagIter {
	current: *const Tag
}

impl TagIter {
	pub fn new(first_tag: *const Tag) -> TagIter {
		TagIter {
			current: first_tag 
		}
	}
}

impl Iterator for TagIter {
	type Item = &'static Tag;

	fn next(&mut self) -> Option<&'static Tag> {
		let current = unsafe { &*self.current };
		match current {
			&Tag { tag_type: 0, size: 8 } => None, // End tag.
			tag => {
				// Jump to the next tag.
				let mut tag_address = self.current as usize;
				tag_address += tag.size as usize;
				// Align tag to 64 bit address.
				tag_address = ((tag_address - 1) & !0x07) + 0x08;
				self.current = tag_address as *const Tag;

				Some(tag)
			}
		}
	}
}
