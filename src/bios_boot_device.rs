use tag::TagType;


struct BIOSBootDeviceTag {
	tag_type: u32,
	size: u32,
	biosdev: u32,
	partition: u32,
	sub_partition: u32,
}

impl BIOSBootDeviceTag {
	fn is_valid_partition(&self) -> bool {
		self.partition != 0xFFFFFFFF
	}

	fn is_valid_sub_partition(&self) -> bool {
		self.partition != 0xFFFFFFFF
	}

	fn is_valid(&self) -> bool {
		(self.size == 20) && (self.tag_type == TagType::BIOSBootDevice as u32)
						  && self.is_valid_partition()
						  && self.is_valid_sub_partition()
	}
}