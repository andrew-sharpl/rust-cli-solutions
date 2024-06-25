use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("J. Andrew Sharp Luevano")
        .version("0.1.0")
        .arg(
            Arg::new("text")
                .action(ArgAction::Append) // Allows multiple vals
                .help("Input text")
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue), // Makes it a flag
        )
        .arg(
            Arg::new("join")
                .short('j')
                .long("join")
                .help("Joins text using given string")
                .required(false),
        )
        .help_template(
            // Template required to show author
            "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
",
        )
        .get_matches();

    // Get arguments and convert to a Vec of strings
    let text = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let omit_newline = matches.get_flag("omit_newline");
    let join = match matches.get_one::<String>("join") {
        Some(value) => value,
        None => " ",
    };
    print!(
        "{}{}",
        text.join(join),
        if omit_newline { "" } else { "\n" }
    );
}
