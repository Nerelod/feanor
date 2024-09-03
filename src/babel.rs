use crate::elf::e_types;

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
