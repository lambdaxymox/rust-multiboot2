use tag::TagType;


pub const END_TAG_SIZE: usize = 8;

pub struct EndTag {
	tag_type: u32,
	size: u32,
}

impl EndTag {
	pub fn is_valid(&self) -> bool {
		(self.tag_type == TagType::EndTag as u32) && (self.size as usize == END_TAG_SIZE)
	}
}
