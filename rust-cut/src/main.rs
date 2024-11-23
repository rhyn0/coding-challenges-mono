mod cli;
mod range;
use clap::{error::ErrorKind as ClapErrorKind, CommandFactory, Parser};
use range::Selector;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Write},
};

use env_logger::Builder;
use log::{debug, info, LevelFilter};

fn handle_field_fields<F, W: Write>(
    reader: &mut Box<dyn BufRead>,
    writer: &mut W,
    delimiter: char,
    output_delimiter: &str,
    suppress_non_delimited: bool,
    line_delim: char,
    selector: F,
) -> io::Result<()>
where
    F: Fn(usize) -> bool,
{
    let mut buffer = Vec::new();
    let mut prev_selected_field = false;
    while let Ok(read_len) = reader.read_until(line_delim as u8, &mut buffer) {
        if read_len == 0 {
            // EOF
            info!("Hit EOF condition for");
            break;
        }
        let buffer_string = String::from_utf8(buffer.clone())
            .expect("valid utf8 strings delimited by {line_delim}");
        for (field_idx, part) in buffer_string.split(delimiter).enumerate() {
            debug!("Checking if index '{field_idx}' is selected with data {part}");
            // if first field and the partition is a full line
            // print it as is
            if field_idx == 0 && part.ends_with(line_delim) {
                if suppress_non_delimited {
                    // do nothing
                } else {
                    write!(writer, "{part}")?;
                }
                break;
            } else if selector(field_idx + 1) {
                if prev_selected_field {
                    write!(writer, "{output_delimiter}")?;
                }
                prev_selected_field = true;
                write!(writer, "{part}")?;
            }
        }
        write!(writer, "{line_delim}")?;
        buffer.clear();
        prev_selected_field = false;
    }
    Ok(())
}

fn handle_byte_fields<F, W: Write>(
    reader: &mut Box<dyn BufRead>,
    writer: &mut W,
    line_delim: char,
    selector: F,
) -> io::Result<()>
where
    F: Fn(usize) -> bool,
{
    let mut buffer = String::new();
    while let Ok(read_len) = reader.read_line(&mut buffer) {
        if read_len == 0 {
            // EOF
            info!("Hit EOF condition for");
            break;
        }
        for (field_idx, part) in buffer.bytes().enumerate() {
            debug!("Checking if index '{field_idx}' is selected with data {part}");
            // if first field and the partition is a full line
            // print it as is
            if part == line_delim as u8 {
                write!(writer, "{line_delim}")?;
            } else if selector(field_idx + 1) {
                write!(writer, "{}", part as char)?;
            }
        }
        write!(writer, "{line_delim}")?;
        buffer.clear();
    }
    Ok(())
}

fn handle_char_fields<F, W: Write>(
    reader: &mut Box<dyn BufRead>,
    writer: &mut W,
    line_delim: char,
    selector: F,
) -> io::Result<()>
where
    F: Fn(usize) -> bool,
{
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
            if part == line_delim {
                write!(writer, "{line_delim}")?;
            } else if selector(field_idx + 1) {
                write!(writer, "{part}")?;
            }
        }
        write!(writer, "{line_delim}")?;
        buffer.clear();
    }
    Ok(())
}

/// Verify that a valid command was passed in.
fn verify_args(cli: &cli::Cli) -> Result<(), String> {
    match (
        cli.selectors.bytes.is_some() || cli.selectors.characters.is_some(),
        cli.delimiter,
        cli.only_delimited,
    ) {
        (true, '\t', false) => Ok(()),
        (true, '\t', true) => Err(
            "Suppressing non-delimited lines makes sense only when operating on fields".to_string(),
        ),
        (true, _, _) => Err(
            "An input delimiter may only be specified only when operating on fields".to_string(),
        ),
        _ => Ok(()),
    }
}

fn determine_output_delimiter(args: &cli::Cli) -> String {
    args.output_delimiter.as_ref().map_or_else(
        || args.delimiter.to_string(),
        |s| {
            if s.is_empty() {
                // Use null byte as the delimiter
                "\0".to_string()
            } else {
                s.to_owned()
            }
        },
    )
}

type OutputHandlerT = dyn FnMut(&mut Box<dyn BufRead>) -> io::Result<()>;
fn main() {
    let mut cli = cli::Cli::parse();
    match cli.verbose {
        0 => Builder::new().filter_level(LevelFilter::Error).init(),
        1 => Builder::new().filter_level(LevelFilter::Warn).init(),
        2 => Builder::new().filter_level(LevelFilter::Info).init(),
        3.. => Builder::new().filter_level(LevelFilter::max()).init(),
    };
    match verify_args(&cli) {
        Ok(()) => {}
        Err(msg) => {
            let mut cmd = cli::Cli::command();
            cmd.error(ClapErrorKind::ArgumentConflict, msg).exit();
        }
    }
    let delimiter = determine_output_delimiter(&cli);
    let line_delimiter = if cli.zero_terminated { '\0' } else { '\n' };
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = io::BufWriter::new(handle);

    let mut cut_func: Box<OutputHandlerT> = if let Some(byte_sel) = cli.selectors.bytes.clone() {
        debug!("Using bytes selectors");
        Box::new(move |reader| {
            handle_byte_fields(reader, &mut writer, line_delimiter, |val| {
                byte_sel.is_selected(val) != cli.complement
            })
        })
    } else if let Some(field_sel) = cli.selectors.fields {
        debug!("Using fields selectors");
        Box::new(move |reader| {
            handle_field_fields(
                reader,
                &mut writer,
                cli.delimiter,
                &delimiter,
                cli.only_delimited,
                line_delimiter,
                |val| field_sel.is_selected(val) != cli.complement,
            )
        })
    } else {
        let char_sel = cli.selectors.characters.unwrap();
        debug!("Using character selectors");
        Box::new(move |reader| {
            handle_char_fields(reader, &mut writer, line_delimiter, |val| {
                char_sel.is_selected(val) != cli.complement
            })
        })
    };
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
        let writing_result = cut_func(&mut reader);
        if let Err(e) = writing_result {
            eprintln!("Failed to work on {filename} - {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[allow(dead_code)]
    fn get_reader(value: String) -> BufReader<Cursor<String>> {
        BufReader::new(Cursor::new(value))
    }

    #[test]
    fn test_byte_suppress_delimited() {
        let args = cli::Cli::parse_from("oxcut -b1 -s -".split_whitespace());
        let result = verify_args(&args);
        assert!(result.is_err_and(|msg| msg.starts_with("Suppressing")));
    }
    #[test]
    fn test_byte_input_delimiter() {
        let args = cli::Cli::parse_from("oxcut -b1 -d: -".split_whitespace());
        let result = verify_args(&args);
        assert!(result.is_err_and(|msg| msg.starts_with("An input")));
    }
    #[test]
    fn test_char_suppress_delimited() {
        let args = cli::Cli::parse_from("oxcut -c1 -s -".split_whitespace());
        let result = verify_args(&args);
        assert!(result.is_err_and(|msg| msg.starts_with("Suppressing")));
    }
    #[test]
    fn test_char_input_delimiter() {
        let args = cli::Cli::parse_from("oxcut -c1 -d: -".split_whitespace());
        let result = verify_args(&args);
        assert!(result.is_err_and(|msg| msg.starts_with("An input")));
    }
}
