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

    /// number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank", overrides_with = "number")]
    number_nonblank: bool,
}

fn main() {
    let args = Args::parse();

    let contents = read_file(&args.file_path);
    print_lines(&contents, args.number, args.number_nonblank);
    
    let contents1 = read_file(&args.file_path);
    print_nonblank(&contents1, args.number_nonblank, args.number);
}

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file {:?}", path))
}

fn print_lines(contents: &str, number: bool, number_nonblank: bool) {
    if !number_nonblank {
        for (i, line) in contents.lines().enumerate() {
            if number {
                println!("{:>4}\t{}", i + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }
}

fn print_nonblank(contents1: &str, number_nonblank: bool, number: bool) {
    let mut counter = 0;
    if !number {
        for line in contents1.lines() {
            if number_nonblank && !line.is_empty() {
                counter += 1;
                println!("{:>4}\t{}",counter , line);
            } else {
                println!("{}", line);
            }
        }
    }
}
