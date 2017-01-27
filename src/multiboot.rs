use bios_boot_device::BIOSBootDeviceTag;
use tag::{Tag, TagIter};


#[derive(Debug)]
#[repr(C, packed)]
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


}