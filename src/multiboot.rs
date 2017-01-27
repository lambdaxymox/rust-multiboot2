use core::mem;
use bios_boot_device::BIOSBootDeviceTag;
use tag::{TagType, Tag, TagIter};
use basic_memory_information::BasicMemoryInformationTag;


#[derive(Debug)]
#[repr(C)]
pub struct MultiBootInfo {
    total_size: u32,
    reserved: u32,      // Should always be 0.
    first_tag: *const Tag
}

impl MultiBootInfo {
    unsafe fn new(multiboot_addr: u32) -> MultiBootInfo {
        let total_size = *(multiboot_addr as *const u32);
        let reserved = *((multiboot_addr + 4) as *const u32);
        let first_tag = (multiboot_addr + 8) as *const Tag;
        
        MultiBootInfo {
            total_size: total_size,
            reserved: reserved,
            first_tag: first_tag
        }

    }

    pub fn start_address(&self) -> usize {
        self as *const _ as usize
    }

    pub fn end_address(&self) -> usize {
        self.start_address() + self.total_size()
    }

    pub fn total_size(&self) -> usize {
        self.total_size as usize
    }

    pub fn mem_lower(&self) -> Option<usize> {
        self.find_tag(TagType::MemoryInformation)
            .map(|tag_ptr| { 
                unsafe {
                    mem::transmute::<&Tag, &BasicMemoryInformationTag>(tag_ptr)
                }
            })
            .map(|tag| { tag.mem_lower() })
    }

    pub fn mem_upper(&self) -> Option<usize> {
        self.find_tag(TagType::MemoryInformation)
            .map(|tag_ptr| {
                unsafe {
                    mem::transmute::<&Tag, &BasicMemoryInformationTag>(tag_ptr)
                }
            })
            .map(|tag| { tag.mem_upper() })
    }

    fn tags(&self) -> TagIter {
        TagIter::new(self.first_tag)
    }

    fn find_tag(&self, tag_type: TagType) -> Option<&'static Tag> {
        self.tags().find(|tag| { tag.tag_type() == tag_type as usize })
    }

}
