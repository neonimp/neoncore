//! This example shows how to read an ELF file and print some information about it.
#![allow(dead_code)]

use std::io::Read;

use neoncore::const_fn::ascii_to_u32_le;
use neoncore::streams::advanced_readers::StructReader;
use neoncore::streams::AnyInt;

const ELF_MAGIC: u32 = ascii_to_u32_le(b"\x7fELF");

#[derive(Debug)]
struct EIdent {
    ei_mag: u32,
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
}

#[derive(Debug)]
struct EHeader {
    e_ident: EIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl EHeader {
    fn read<R: Read>(mut s: R) -> Self {
        let ident_reader = StructReader::new_le()
            .add_u32_field("ei_mag")
            .add_u8_field("ei_class")
            .add_u8_field("ei_data")
            .add_u8_field("ei_version")
            .add_u8_field("ei_osabi")
            .add_u8_field("ei_abiversion")
            .add_padding(7)
            .read(&mut s)
            .unwrap();

        let header_reader = StructReader::new_le()
            .add_u16_field("e_type")
            .add_u16_field("e_machine")
            .add_u32_field("e_version");

        let header_reader = if ident_reader["ei_class"] == AnyInt::U8(2) {
            header_reader
                .add_u64_field("e_entry")
                .add_u64_field("e_phoff")
                .add_u64_field("e_shoff")
        } else {
            header_reader
                .add_u32_field("e_entry")
                .add_u32_field("e_phoff")
                .add_u32_field("e_shoff")
        };

        let header_reader = header_reader
            .add_u32_field("e_flags")
            .add_u16_field("e_ehsize")
            .add_u16_field("e_phentsize")
            .add_u16_field("e_phnum")
            .add_u16_field("e_shentsize")
            .add_u16_field("e_shnum")
            .add_u16_field("e_shstrndx");

        let header = header_reader.read(&mut s).unwrap();
        println!("Header: {:#?}", header);

        let e_ident = EIdent {
            ei_mag: ident_reader["ei_mag"].try_into().unwrap(),
            ei_class: ident_reader["ei_class"].try_into().unwrap(),
            ei_data: ident_reader["ei_data"].try_into().unwrap(),
            ei_version: ident_reader["ei_version"].try_into().unwrap(),
            ei_osabi: ident_reader["ei_osabi"].try_into().unwrap(),
            ei_abiversion: ident_reader["ei_abiversion"].try_into().unwrap(),
        };
        EHeader {
            e_ident,
            e_type: header["e_type"].try_into().unwrap(),
            e_machine: header["e_machine"].try_into().unwrap(),
            e_version: header["e_version"].try_into().unwrap(),
            e_entry: header["e_entry"].try_into().unwrap(),
            e_phoff: header["e_phoff"].try_into().unwrap(),
            e_shoff: header["e_shoff"].try_into().unwrap(),
            e_flags: header["e_flags"].try_into().unwrap(),
            e_ehsize: header["e_ehsize"].try_into().unwrap(),
            e_phentsize: header["e_phentsize"].try_into().unwrap(),
            e_phnum: header["e_phnum"].try_into().unwrap(),
            e_shentsize: header["e_shentsize"].try_into().unwrap(),
            e_shnum: header["e_shnum"].try_into().unwrap(),
            e_shstrndx: header["e_shstrndx"].try_into().unwrap(),
        }
    }
}

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

    let mut reader = std::io::BufReader::new(file);

    let header = EHeader::read(&mut reader);

    println!("Magic: {:#x}", header.e_ident.ei_mag);

    println!("{:#?}", header);
}
