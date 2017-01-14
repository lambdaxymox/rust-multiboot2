use tag::TagType;


struct BasicMemoryInformationTag {
	tag_type: u32,
	size: u32,
	mem_lower: u32,
	mem_upper: u32,
}

impl BasicMemoryInformationTag {
	fn is_valid(&self) -> bool {
		(self.mem_lower <= 640) && (self.tag_type == TagType::MemoryInformation as u32)
	}
}