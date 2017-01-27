use bios_boot_device::{BIOSBootDeviceTag, BootDevice};
use boot_command_line::BootCommandLineTag;
use boot_loader_name::BootLoaderNameTag;
use memory_map::{MemoryMapTag, MemoryMapIter};
use tag::{TagType, Tag, TagIter};
use basic_memory_information::BasicMemoryInformationTag;


pub unsafe fn load(multiboot_addr: u32) -> &'static MultiBootInfo  {
    let multiboot_info = MultiBootInfo::from_raw_parts(multiboot_addr);
    assert!(multiboot_info.has_valid_end_tag());
    multiboot_info
}


#[derive(Debug)]
#[repr(C)]
pub struct MultiBootInfo {
    total_size: u32,
    reserved: u32,      // Should always be 0.
    first_tag: Tag
}

impl MultiBootInfo {
    unsafe fn from_raw_parts(multiboot_addr: u32) -> &'static MultiBootInfo {
        &*(multiboot_addr as *const MultiBootInfo)
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
                    tag_ptr.cast::<BasicMemoryInformationTag>()
                }
            })
            .map(|tag| { tag.mem_lower() })
    }

    pub fn mem_upper(&self) -> Option<usize> {
        self.find_tag(TagType::MemoryInformation)
            .map(|tag_ptr| {
                unsafe {
                    tag_ptr.cast::<BasicMemoryInformationTag>()
                }
            })
            .map(|tag| { tag.mem_upper() })
    }

    pub fn bios_boot_device(&self) -> Option<BootDevice> {
        self.find_tag(TagType::BIOSBootDevice)
            .map(|tag_ptr| { 
                unsafe {
                    tag_ptr.cast::<BIOSBootDeviceTag>()
                }
            })
            .map(|tag| {
                BootDevice::new(tag.biosdev, tag.partition, tag.sub_partition)
            })
    }

    pub fn boot_cmd_line(&self) -> Option<&str> {
        self.find_tag(TagType::BootCommandLine)
            .map(|tag_ptr| {
                unsafe {
                    tag_ptr.cast::<BootCommandLineTag>()
                }
            })
            .map(|tag| { tag.string() })
    }

    pub fn boot_loader_name(&self) -> Option<&str> {
        self.find_tag(TagType::BootCommandLine)
            .map(|tag_ptr| {
                unsafe {
                    tag_ptr.cast::<BootLoaderNameTag>()
                }
            })
            .map(|tag| { tag.string() })
    }

    pub fn memory_map(&self) -> Option<MemoryMapIter> {
        self.find_tag(TagType::MemoryMap)
            .map(|tag_ptr| {
                unsafe {
                    tag_ptr.cast::<MemoryMapTag>()
                }
            })
            .map(|tag| {
                tag.memory_map()
            })
    }

    fn has_valid_end_tag(&self) -> bool {
        true
    }

    fn tags(&self) -> TagIter {
        TagIter::new(&self.first_tag)
    }

    fn find_tag(&self, tag_type: TagType) -> Option<&'static Tag> {
        self.tags().find(|tag| { tag.tag_type() == tag_type as usize })
    }

}
