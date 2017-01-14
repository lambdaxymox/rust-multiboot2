use tag::TagType;


const END_TAG_SIZE: u32 = 8;

struct EndTag {
	tag_type: u32,
	size: u32,
}

impl EndTag {
	fn is_valid(&self) -> bool {
		(self.tag_type == TagType::EndTag as u32) && (self.size == END_TAG_SIZE)
	}
}