use clap::Parser;
use std::{fs, path::PathBuf};

/// Read a file and optionally add line numbers
#[derive(Parser, Debug)]
#[command(author = "Andy Hughes", version = "0.1", about = "A cat clone")]
struct Args {
    /// File to read
    #[arg(value_name = "PATH")]
    file_path: PathBuf,

    /// number all output lines
    #[arg(short = 'n', long = "number")]
    number: bool,
}

fn main() {
    let args = Args::parse();

    let contents = read_file(&args.file_path);
    print_lines(&contents, args.number);
}

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file {:?}", path))
}

fn print_lines(contents: &str, number: bool) {
    for (i, line) in contents.lines().enumerate() {
        if number {
            println!("{:>6}\t{}", i + 1, line);
        } else {
            println!("{}", line);
        }
    }
}
