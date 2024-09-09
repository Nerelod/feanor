#[repr(C)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[derive(Debug)]
pub enum e_types {
    ET_NONE,
    ET_REL,
    ET_EXEC,
    ET_DYN,
    ET_CORE,
    ET_LOOS,
    ET_HIOS,
    ET_LOPROC,
    ET_HIPROC,
    undefined,
}

#[derive(Debug)]
pub enum e_machines {
    EM_X86_64,
    undefined,
}

#[derive(Debug)]
pub enum e_flags {
    EF_SPARCV9_MM,
    EF_SPARCV9_TSO,
    EF_SPARCV9_PSO,
    EF_SPARCV9_RMO,
    EF_SPARC_LEDATA	,
    EF_SPARC_EXT_MASK,
    EF_SPARC_32PLUS,
    EF_SPARC_SUN_US1,
    EF_SPARC_HAL_R1,
    EF_SPARC_SUN_US3,
    undefined,
}

pub struct silmaril {
    pub e_type: e_types,
}
