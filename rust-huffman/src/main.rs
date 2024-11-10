use clap::Parser;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};
use thiserror::Error;

mod cli;
mod counter;
mod decoder;
mod encoder;
mod huffman;
mod utils;

#[derive(Error, Debug)]
enum TopError {
    #[error("Failed the thing with files - {0}")]
    Io(#[from] io::Error),
    #[error("Error encoding the data - {0}")]
    Encoding(#[from] encoder::EncodingError),
    #[error("Error decoding the data - {0}")]
    Decoding(#[from] decoder::DecodeError),
}

fn open_new_file(filepath: &str) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filepath)
}

fn handle_decode(input_file: &str, output_file: &str) -> Result<(), TopError> {
    let mut file_reader = BufReader::new(File::open(input_file)?);
    let mut buffer = Vec::new();
    match file_reader.read_until(b'\n', &mut buffer) {
        Ok(_) => {}
        Err(e) => return Err(TopError::Io(e)),
    }
    let lossy_string = String::from_utf8_lossy(&buffer);
    let (table, padding) = match decoder::get_huffman_table(&lossy_string) {
        Ok(x) => x,
        Err(e) => return Err(TopError::Decoding(e)),
    };

    let text = decoder::decode(file_reader, table, padding).expect("Able to decode it");
    let mut file = open_new_file(output_file)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
fn handle_encode(input_file: &str, output_file: &str) -> Result<(), TopError> {
    let file_reader = BufReader::new(File::open(input_file)?);
    let counts = counter::Characters::new(file_reader).read_all()?;
    let tree = huffman::HuffmanTree::new(counts.clone());
    let codes = tree.get_huffman_codes();
    // reset to beginning
    let file_reader = BufReader::new(File::open(input_file)?);
    let data = encoder::encode(file_reader, &codes)?;
    let mut file = open_new_file(output_file)?;
    let header_bytes = format!("{counts:?}::{}\n", data.padding.unwrap());
    let hlen_bits = format!("{:b}", header_bytes.len());
    let mut hlen_container = "0".repeat(32 - hlen_bits.len());
    hlen_container.push_str(&hlen_bits);

    file.write_all(hlen_container.as_bytes())?;
    file.write_all(header_bytes.as_bytes())?;
    file.write_all(&data.data)?;
    Ok(())
}

fn main() {
    let args = cli::Cli::parse();
    let result = match args.command {
        cli::Commands::Decode {
            input_file,
            output_file,
        } => handle_decode(&input_file, &output_file),
        cli::Commands::Encode {
            input_file,
            output_file,
        } => handle_encode(&input_file, &output_file),
    };
    if let Err(e) = result {
        eprintln!("{e}");
    }
}
