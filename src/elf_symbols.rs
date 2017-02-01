

// Elf section headers have a fixed size.
const ELF_SECTION_HEADER_SIZE: usize = 56;

#[repr(packed)]
struct ElfSymbolTag {
    tag_type: u32,
    size: u32,
    num: u16,
    entsize: u16,
    shndx: u16,
    reserved: u16,
    first_section: ElfSectionHeader
}

struct ElfSectionHeader {
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

struct ElfSectionIter {
    current_section: &'static ElfSectionHeader,
    final_section: &'static ElfSectionHeader, 
    remaining_sections: u32,
    entry_size: u32
}