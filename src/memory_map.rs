use tag::TagType;


#[repr(C)]
struct MemoryMapTag {
	tag_type: u32,
	size: u32,
	entry_size: u32,
	/// This is currently set to `0`.
	entry_version: u32,
	first_entry: MemoryMapEntry
}

#[derive(Debug)]
#[repr(C, packed)]
struct MemoryMapEntry {
	/// The starting physical address
	base_addr: u64,
	/// The size of the memory region, in bytes.
	length: u64,
	/// The type of memory region contained.
	entry_type: u32,
	/// This should be set to `0` and ignored by the bootloader.
	reserved: u32
}

#[derive(Debug, PartialEq, Eq)]
enum MemoryType {
	Usable      = 1,
	UsableWithACPI = 3,
	Unusable       = 4,
}

impl MemoryMapEntry {
	fn base_address(&self) -> u64 {
		self.base_addr
	}

	fn length(&self) -> u64 {
		self.length
	}

	fn memory_type(&self) -> MemoryType {
		match self.entry_type {
			1 => MemoryType::Usable,
			3 => MemoryType::UsableWithACPI,
			_ => MemoryType::Unusable
		}
	}
}

struct MemoryMapIter {
	current_entry: *const MemoryMapEntry,
	// We need to know the entry size so we can calculate the 
	// address of the next memory map entry.
	entry_size: u32,
}