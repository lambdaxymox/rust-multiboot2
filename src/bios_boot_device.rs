use tag::TagType;


const UNUSED_PARTITION_NUMBER: u32 = 0xFFFFFFFF;
const BIOS_BOOT_DEVICE_TAG_SIZE: u32 = 20;

struct BIOSBootDeviceTag {
	tag_type: u32,
	size: u32,
	biosdev: u32,
	partition: u32,
	sub_partition: u32,
}

impl BIOSBootDeviceTag {
	fn is_valid_partition(&self) -> bool {
		self.partition != UNUSED_PARTITION_NUMBER
	}

	fn is_valid_sub_partition(&self) -> bool {
		self.sub_partition != UNUSED_PARTITION_NUMBER
	}

	fn is_valid(&self) -> bool {
		(self.size == BIOS_BOOT_DEVICE_TAG_SIZE) 
		           && (self.tag_type == TagType::BIOSBootDevice as u32)
				   && self.is_valid_partition()
				   && self.is_valid_sub_partition()
	}
}