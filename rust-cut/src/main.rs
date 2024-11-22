mod cli;
mod list;
use clap::Parser;
use list::CutSelector;
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
};

use env_logger::Builder;
use log::{debug, info, LevelFilter};

fn handle_file_fields(f: File, delimiter: char, selectors: &[list::CutRange]) {
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut prev_selected_field = false;
    while let Ok(read_len) = reader.read_line(&mut buffer) {
        if read_len == 0 {
            // EOF
            info!("Hit EOF condition for");
            break;
        }
        for (field_idx, part) in buffer.split(delimiter).enumerate() {
            debug!("Checking if index '{field_idx}' is selected with data {part}");
            // if first field and the partition is a full line
            // print it as is
            if field_idx == 0 && part.ends_with('\n') {
                print!("{part}");
                break;
            } else if selectors.iter().any(|sel| sel.is_selected(field_idx + 1)) {
                if prev_selected_field {
                    print!("{delimiter}");
                }
                prev_selected_field = true;
                print!("{part}");
            }
        }
        println!();
        buffer.clear();
        prev_selected_field = false;
    }
}

fn main() {
    let cli = cli::Cli::parse();
    match cli.verbose {
        0 => Builder::new().filter_level(LevelFilter::Error).init(),
        1 => Builder::new().filter_level(LevelFilter::Warn).init(),
        2 => Builder::new().filter_level(LevelFilter::Info).init(),
        3.. => Builder::new().filter_level(LevelFilter::max()).init(),
    };
    debug!(
        "Selectors are bytes {0:?} or fields {1:?}",
        cli.bytes, cli.fields
    );
    for filename in cli.files {
        debug!("Running for files  {filename}");
        match File::open(&filename) {
            Ok(f) => {
                if cli.bytes.is_empty() {
                    handle_file_fields(f, cli.delimiter, &cli.fields);
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => eprintln!("{filename}: No such file or directory"),
                _ => panic!("Unhandled error with {filename} {e}"),
            },
        }
    }
}
