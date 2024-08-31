use std::fs::File;
use std::io::{self, Read};
use std::mem;
use std::path::Path;
use crate::elf::Elf64Ehdr;

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
}
