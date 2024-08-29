mod args;
mod count;
use std::fmt::Write;

use args::{Cli, OutputOptions};
use clap::Parser;
use count::{get_counts, ResultOutput};

fn write_part(buffer: &mut String, value: usize) {
    let spacing = 8;
    if let Err(e) = write!(buffer, "{value:>spacing$}") {
        panic!("{e}")
    }
}
fn build_line(result: &ResultOutput, options: OutputOptions) -> String {
    let mut line = String::new();
    if options.line_count {
        write_part(&mut line, result.line_count);
    };
    if options.word_count {
        write_part(&mut line, result.word_count);
    };
    if options.byte_count {
        write_part(&mut line, result.byte_count);
    };
    if options.character_count {
        write_part(&mut line, result.character_count);
    }
    line.push_str(&format!(" {}", result.name));
    line.push('\n');
    line
}
fn print_results(arr: &[ResultOutput], options: OutputOptions) {
    let mut total = ResultOutput::new("total".to_string());
    let output = arr.iter().map(|r| (r.to_owned(), build_line(r, options)));
    for (result, line) in output {
        total += result;
        print!("{line}");
    }
    if arr.len() > 1 {
        print!("{}", build_line(&total, options));
    }
}
fn main() {
    let cli = Cli::parse();
    let output = cli.get_output_settings();
    let mut results = Vec::with_capacity(cli.files.len());
    if cli.files.is_empty() {
        // use stdin
        if let Ok(result) = get_counts("-".to_string()) {
            results.push(result);
        } else {
            eprintln!("Failed to read from stdin");
        }
    }
    for file in cli.files {
        if let Ok(result) = get_counts(file.clone()) {
            results.push(result);
        } else {
            eprintln!("Failed to open file {file}");
        }
    }
    print_results(&results, output);
}
