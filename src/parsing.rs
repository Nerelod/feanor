use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::path::Path;
use crate::elf::*;
use crate::babel::*;

pub fn read_elf_header(file_path: &Path) -> io::Result<Elf64Ehdr> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; mem::size_of::<Elf64Ehdr>()];
    file.read_exact(&mut buffer);

    let header: Elf64Ehdr = unsafe { mem::transmute(buffer) };
    Ok(header)
}

pub fn print_elf_header(header: &Elf64Ehdr) {
    println!("ELF Header:");
    println!("\tMagic: {:x?}", header.e_ident);
    let t = determine_file_type(header.e_type);
    println!("\tType: {}\t{:?}", header.e_type, t);
    let m = determine_machine(header.e_machine);
    println!("\tMachine: {}\t{:?}", header.e_machine, m);
    println!("\tVersion: {}", header.e_version);
    println!("\tEntry: {:#x}", header.e_entry);
    println!("\tProgram Header Offset:\t{:#x}", header.e_phoff);
    println!("\tSection Header Offset:\t{:#x}", header.e_shoff);
    let f = determine_flags(header.e_flags);
    println!("\tFlags:\t{:#x}", header.e_flags);
}
