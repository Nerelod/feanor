mod elf;
mod parsing;

use std::path::Path;
use std::env;
use parsing::read_elf_header;
use parsing::print_elf_header;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let bin = Path::new(&args[1]);
    let header = read_elf_header(bin)?;

    print_elf_header(&header);

    Ok(())

}
