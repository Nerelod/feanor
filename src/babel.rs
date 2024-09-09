use crate::elf::*;

pub fn determine_file_type(e_type: u16) -> e_types {
    let t: e_types; 
    match e_type{
        0=>t = e_types::ET_NONE,
        1=>t = e_types::ET_REL,
        2=>t = e_types::ET_EXEC,
        3=>t = e_types::ET_DYN,
        4=>t = e_types::ET_CORE,
        0xfe00=>t = e_types::ET_LOOS,
        0xfeff=>t = e_types::ET_HIOS,
        0xff00=>t = e_types::ET_LOPROC,
        0xffff=>t = e_types::ET_HIPROC,
        _=>t = e_types::undefined,
    };
    t
}

pub fn determine_machine(e_machine: u16) -> e_machines {
    let m: e_machines;
    match e_machine{
        62=>m = e_machines::EM_X86_64,
        _=>m = e_machines::undefined, 
    };
    m
}

pub fn determine_flags(e_flag: u32) -> e_flags {
    let f: e_flags;
    match e_flag{
        0xffff00=>f = e_flags::EF_SPARC_EXT_MASK,
        0x000100=>f = e_flags::EF_SPARC_32PLUS,
        0x000200=>f = e_flags::EF_SPARC_SUN_US1,
        0x000400=>f = e_flags::EF_SPARC_HAL_R1,
        0x000800=>f = e_flags::EF_SPARC_SUN_US3, 
        _=>f = e_flags::undefined, 
    }
    f
}
