use bios_boot_device::BIOSBootDeviceTag;
use tag::{Tag, TagIter};


pub struct MultiBootInfo {
    total_size: u32,
    reserved: u32,      // Should always be 0
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    boot_cmd_line: u32,
    module_count: u32,
    modules: u32,
    elf_symbols: u32,
    memory_map: u32,
    boot_loader_name: u32,
    apm_table: u32,
    vbe_info: u32,
    framebuffer_info: u32
}

impl MultiBootInfo {
    unsafe fn new(multiboot_addr: u32) -> () {
        let total_size = *(multiboot_addr as *const u32);
        let reserved = *((multiboot_addr + 4) as *const u32);
        let first_tag = (multiboot_addr + 8) as *const Tag;
        let tag_iter = TagIter::new(first_tag);
    }
}