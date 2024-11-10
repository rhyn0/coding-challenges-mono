use std::{collections::HashMap, fmt::Write, fs, path::PathBuf};

#[allow(dead_code)]
pub fn write_huffman_header(
    mapping: &HashMap<char, String>,
    file_path: String,
) -> std::io::Result<()> {
    let path = PathBuf::from(file_path);
    if !path.exists() {
        if let Some(parent_path) = path.parent() {
            fs::create_dir_all(parent_path)?;
        }
    }
    let file_data = mapping
        .iter()
        .fold(String::new(), |mut prev, (c, byte_string)| {
            writeln!(&mut prev, "{c}::{byte_string}").expect("Unable to add entry to file data");
            prev
        });
    fs::write(path, file_data)
}
