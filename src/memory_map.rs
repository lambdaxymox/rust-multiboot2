#![allow(dead_code)]


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
	Usable         = 1,
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

	fn is_usable_region(&self) -> bool {
		self.memory_type() == MemoryType::Usable
	}
}

struct MemoryMapIter {
	current_entry: *const MemoryMapEntry,
	final_entry: *const MemoryMapEntry,
	// We need to know the entry size so we can calculate the 
	// address of the next memory map entry.
	entry_size: u32,
}

impl MemoryMapIter {
	fn new(current_entry: *const MemoryMapEntry, 
		   final_entry: *const MemoryMapEntry, 
		   entry_size: u32) -> MemoryMapIter 
	{
		MemoryMapIter {
			current_entry: current_entry,
			final_entry: final_entry,
			entry_size: entry_size,
		}
	}
}

impl Iterator for MemoryMapIter {
	type Item = &'static MemoryMapEntry;

	fn next(&mut self) -> Option<Self::Item> {
		// Loop until the next usable memory region. We ignore the unusable regions
		// per the Multiboot2 standard requirements.
		loop {
			if self.current_entry <= self.final_entry {
				let current_entry = unsafe { &*self.current_entry };
				let next_entry = (self.current_entry as u64) + (self.entry_size as u64);
				
				self.current_entry = next_entry as *const MemoryMapEntry;
				
				if current_entry.is_usable_region() {
					return Some(current_entry);
				} 

			} else {
				// We have fallen off the end of the memory map.
				return None;
			}
		}
	}
}