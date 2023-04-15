//! This example shows how to read an ELF file and print some information about it.

use neoncore::const_fn::ascii_to_u32_le;
use neoncore::streams::read::StructReader;
use neoncore::streams::AnyInt;

const ELF_MAGIC: u32 = ascii_to_u32_le(b"\x7fELF");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <elf-file>", args[0]);
        return;
    }

    let elf_file = &args[1];

    let file = match std::fs::File::open(elf_file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    };

    let header_reader = StructReader::new_le()
        .add_u32_field("e_mag")
        .add_expr_field("is_64bit", 1, |v| v == AnyInt::U8(2))
        .add_u8_field("e_data")
        .add_u8_field("e_version")
        .add_u8_field("e_osabi")
        .add_u8_field("e_abiversion")
        .add_padding(8)
        .add_u16_field("e_type")
        .add_u16_field("e_machine")
        .add_u32_field("e_version");

    println!("Pattern: {:#?}", header_reader.get_inner_pattern());

    let header = header_reader.read(file).unwrap();

    assert_eq!(
        TryInto::<u32>::try_into(header["e_mag"]).unwrap(),
        ELF_MAGIC
    );
    println!("ELF Header:");
    println!("{:#?}", header.into_vec());
}
