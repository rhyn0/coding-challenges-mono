mod cli;
mod uniq;
use clap::Parser;
use std::{
    fs::{File, OpenOptions},
    io::{self, stdin, stdout, BufReader, BufWriter, Read, Write},
};

fn create_reader(filename: &str) -> Result<BufReader<Box<dyn io::Read>>, io::Error> {
    if filename == "-" {
        Ok(BufReader::new(Box::new(stdin())))
    } else {
        match File::open(filename) {
            Ok(f) => Ok(BufReader::new(Box::new(f))),
            Err(e) => Err(e),
        }
    }
}

fn create_writer(output_path: Option<&String>) -> io::BufWriter<Box<dyn io::Write>> {
    output_path.map_or_else(
        || BufWriter::new(Box::new(stdout()) as Box<dyn Write>),
        |filename| {
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(filename)
                .expect("Able to create or truncate output file");
            BufWriter::new(Box::new(file))
        },
    )
}

fn handle_args<R>(args: &cli::Args, reader: BufReader<R>) -> Box<dyn Iterator<Item = String>>
where
    R: Read + 'static,
{
    if args.count {
        Box::new(uniq::prelude::line_counts(reader))
    } else {
        Box::new(uniq::prelude::read_lines(reader))
    }
}

fn main() {
    let args = cli::Args::parse();
    let filename = args.input_file.clone().unwrap_or_else(|| "-".into());
    let reader = match create_reader(&filename) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to open file {e:?}");
            return;
        }
    };
    let mut writer = create_writer(args.output_file.as_ref());
    let lines = handle_args(&args, reader);
    lines.for_each(|line| writeln!(&mut writer, "{line}").expect("write output"));
}
