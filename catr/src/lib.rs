use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("J. Andrew Sharp Luevano <jasharpluevano@gmail.com>")
        .about("cat written in Rust")
        .arg(
            // Positional arguments do not have long/short names
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Number Lines")
                .long("number")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .help("Number non-blank lines")
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many("files")
            .expect("files required")
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buffer) => {
                if !(config.number_lines || config.number_nonblank_lines) {
                    // Print without any numbering
                    for line in buffer.lines() {
                        let line = line?; // Handle errors
                        println!("{}", line);
                    }
                } else {
                    // Numbered lines
                    let mut line_num = 1;
                    for line in buffer.lines() {
                        let line = line?;
                        // Check if line is blank
                        if line.is_empty() && config.number_nonblank_lines {
                            // Skip empty line
                            println!();
                        } else {
                            // Print with line number
                            println!("{:>6}\t{}", line_num, line);
                            line_num = line_num + 1;
                        }
                    }
                }
            },
        }
    }
    Ok(())
}
