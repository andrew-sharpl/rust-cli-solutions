use clap::{Arg, ArgAction, Command};
use std::error::Error;


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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    dbg!(config);
    Ok(())
}
