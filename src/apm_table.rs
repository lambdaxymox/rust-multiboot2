use tag::TagType;


const APM_TABLE_SIZE: u32 = 28;

struct APMTable {
	tag_type: u32,
	size: u32,
	version: u16,
	cseg: u16,
	offset: u32,
	cseg_16: u16,
	dseg: u16,
	flags: u16,
	cseg_len: u16,
	cseg_16_len: u16,
	dseg_len: u16,
}

impl APMTable {
	fn is_valid(&self) -> bool {
		(self.tag_type == (TagType::APMTable as u32)) && (self.size == APM_TABLE_SIZE)
	}
}