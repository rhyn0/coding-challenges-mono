mod cli;
mod range;
use clap::Parser;
use range::Selector;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind},
};

use env_logger::Builder;
use log::{debug, info, LevelFilter};

fn handle_file_fields(reader: &mut Box<dyn BufRead>, delimiter: char, selectors: &range::CutList) {
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
            } else if selectors.is_selected(field_idx + 1) {
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

fn handle_byte_fields(reader: &mut Box<dyn BufRead>, selectors: &range::CutList) {
    let mut buffer = String::new();
    while let Ok(read_len) = reader.read_line(&mut buffer) {
        if read_len == 0 {
            // EOF
            info!("Hit EOF condition for");
            break;
        }
        for (field_idx, part) in buffer.char_indices() {
            debug!("Checking if index '{field_idx}' is selected with data {part}");
            // if first field and the partition is a full line
            // print it as is
            if part == '\n' {
                println!();
            } else if selectors.is_selected(field_idx + 1) {
                print!("{part}");
            }
        }
        println!();
        buffer.clear();
    }
}

fn main() {
    let mut cli = cli::Cli::parse();
    match cli.verbose {
        0 => Builder::new().filter_level(LevelFilter::Error).init(),
        1 => Builder::new().filter_level(LevelFilter::Warn).init(),
        2 => Builder::new().filter_level(LevelFilter::Info).init(),
        3.. => Builder::new().filter_level(LevelFilter::max()).init(),
    };
    let bytes_selector = cli.selectors.bytes.unwrap_or_default();
    let fields_selector = cli.selectors.fields.unwrap_or_default();
    debug!(
        "Selectors are bytes {0:?} or fields {1:?}",
        bytes_selector, fields_selector,
    );
    if cli.files.is_empty() {
        cli.files.push("-".to_string());
    }
    info!("Working on {0} files", cli.files.len());
    for filename in cli.files {
        debug!("Running for files  {filename}");
        let mut reader: Box<dyn BufRead> = if filename == "-" {
            Box::new(BufReader::new(io::stdin()))
        } else {
            let f = match File::open(&filename) {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("{filename}: No such file or directory");
                        continue;
                    }
                    _ => panic!("Unhandled error with {filename} {e}"),
                },
            };
            Box::new(BufReader::new(f))
        };
        if fields_selector.is_empty() {
            // if fields isnt set, then use bytes
            handle_byte_fields(&mut reader, &bytes_selector);
        } else {
            // if fields is set, then use fields
            handle_file_fields(&mut reader, cli.delimiter, &fields_selector);
        };
    }
}
