use tag::{TagType, VerifyTag};


const BASIC_MEMORY_INFORMATION_TAG_SIZE: usize = 16;

#[repr(packed)]
pub struct BasicMemoryInformationTag {
    tag_type: u32,
    size: u32,
    mem_lower: u32,
    mem_upper: u32,
}

impl BasicMemoryInformationTag {
    pub fn mem_lower(&self) -> usize {
        self.mem_lower as usize
    }

    pub fn mem_upper(&self) -> usize {
        self.mem_upper as usize
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

impl VerifyTag for BasicMemoryInformationTag {
    fn is_valid(&self) -> bool {
        (self.mem_lower <= 640) && 
        (self.tag_type == TagType::MemoryInformation as u32) && 
        (self.size() == BASIC_MEMORY_INFORMATION_TAG_SIZE)
    }
}