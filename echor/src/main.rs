use clap::{App, Arg};

fn main() {
    // Defining CLI flags and arguments
    let matches = App::new("echor")
        .version("0.1.0")
        .author("J. Andrew Sharp Luevano")
        .about("A recreation of the echo command, written in Rust.")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    // Lossy returns Option<Vec<String>>
    // We can assume unwrap succeeds since get_matches
    // did not fail.
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // Note that if ... else ... is an expression that returns 
    // a value.
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
