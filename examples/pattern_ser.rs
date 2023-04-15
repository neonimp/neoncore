use neoncore::streams::advanced_readers::StructReader;

fn main() {
    let header_reader = StructReader::new_le()
        .add_u32_field("e_mag")
        .add_u8_field("e_class")
        .add_u8_field("e_data")
        .add_u8_field("e_version")
        .add_u8_field("e_osabi")
        .add_u8_field("e_abiversion")
        .add_padding(8)
        .add_u16_field("e_type")
        .add_u16_field("e_machine")
        .add_u32_field("e_version");

    println!(
        "Pattern: {}",
        serde_json::to_string_pretty(&header_reader.get_inner_pattern()).unwrap()
    );
}
