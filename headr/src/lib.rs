use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}


pub fn get_args() -> Result<Args, Box<dyn Error>> {
    let matches = Command::new("headr")
            .version("0.1.0")
            .author("J. Andrew Sharp Luevano <jasharpluevano@gmail.com>")
            .about("head written in Rust")
            .arg(
                Arg::new("lines")
                    .short('n')
                    .long("lines")
                    .value_name("LINES")
                    .help("Number of lines")
                    .value_parser(clap::value_parser!(u64).range(1..))
                    .default_value("10"),
            )
            .arg(
                Arg::new("bytes")
                    .short('c')
                    .long("bytes")
                    .value_name("BYTES")
                    .help("Number of bytes")
                    .conflicts_with("lines")
                    .value_parser(clap::value_parser!(u64).range(1..)),
            )
            .arg(
                Arg::new("files")
                    .value_name("FILE")
                    .help("Input file(s)")
                    .num_args(1..)
                    .default_value("-"),
            )
            .get_matches();

    Ok(
        Args {
            files: matches
                .get_many("files")
                .expect("files required")
                .cloned()
                .collect(),
            lines: matches.get_one("lines").cloned().unwrap(),
            bytes: matches.get_one("bytes").cloned(),
        }
    )
}


pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", args);
    Ok(())
}
