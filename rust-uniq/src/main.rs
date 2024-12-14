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

fn handle_args<R>(args: &cli::Args, reader: BufReader<R>) -> uniq::prelude::LineCounts
where
    R: Read + 'static,
{
    let mut uniq_reader = uniq::prelude::UniqueReader::new(reader);
    if args.repeated {
        uniq_reader = uniq_reader.repeated();
    } else if args.unique {
        uniq_reader = uniq_reader.unique();
    }
    let mut counts = uniq_reader.into_line_counts();
    if args.count {
        counts = counts.include_counts();
    }
    counts
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
    lines
        .into_lines()
        .for_each(|line| write!(&mut writer, "{line}").expect("write output"));
}
