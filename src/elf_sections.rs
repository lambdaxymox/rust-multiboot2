// The size of an ELF header tag, in bytes. This was calculated 
// from reading the ELF format specification.
const ELF_SECTION_HEADER_SIZE: u64 = 64;


#[derive(Debug)]
#[repr(C, packed)]
pub struct ElfSectionTag {
    tag_type: u32,
    size: u32,
    num: u32,
    entsize: u32,
    shndx: u16,
    reserved: u16,
    first_section: ElfSectionHeader
}

impl ElfSectionTag {
    pub fn elf_sections(&'static self) -> ElfSectionIter {
        ElfSectionIter {
            current_section: &self.first_section,
            section_index: 0,
            total_sections: self.num as u32,
            entry_size: self.entsize as u64
        }
    }

    pub fn section_count(&self) -> usize {
        self.num as usize
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    pub fn entry_size(&self) -> usize {
        self.entsize as usize
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ElfSectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64
}

impl ElfSectionHeader {
    pub fn section_type(&self) -> usize {
        self.sh_type as usize
    }

    pub fn section_start_address(&self) -> usize {
        self.sh_addr as usize
    }

    pub fn section_end_address(&self) -> usize {
        (self.sh_addr + self.sh_size) as usize
    }

    pub fn size(&self) -> usize {
        self.sh_size as usize
    }

    pub fn flags(&self) -> usize {
        self.sh_flags as usize
    }

    pub fn entry_size(&self) -> usize {
        self.sh_entsize as usize
    }
}

pub struct ElfSectionIter {
    current_section: &'static ElfSectionHeader,
    section_index: u32,
    total_sections: u32,
    entry_size: u64
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSectionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.section_index >= self.total_sections {
            return None;
        } else {
            let section = self.current_section;
            let next_section_addr = 
                (self.current_section as *const ElfSectionHeader as u64) + self.entry_size;
            let next_section = unsafe { 
                &*(next_section_addr as *const ElfSectionHeader) 
            };

            self.current_section = next_section;
            self.section_index += 1;

            Some(section)
        }
    }
}
