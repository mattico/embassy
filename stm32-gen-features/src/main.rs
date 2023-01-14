use gen_features::{chip_data, embassy_stm32_needed_data, generate_cargo_toml_file, stm32_metapac_needed_data};

fn main() {
    let data = chip_data();
    update_cargo_file("../embassy-stm32/Cargo.toml", &embassy_stm32_needed_data(&data));
    update_cargo_file("../stm32-metapac/Cargo.toml", &stm32_metapac_needed_data(&data));
}

/// Update a Cargo.toml file
///
/// Update the content between "# BEGIN GENERATED FEATURES" and "# END GENERATED FEATURES"
/// with the given content
fn update_cargo_file(path: &str, new_contents: &str) {
    let previous_text = std::fs::read_to_string(path).unwrap();
    let new_text = generate_cargo_toml_file(&previous_text, new_contents);
    std::fs::write(path, new_text).unwrap();
}
