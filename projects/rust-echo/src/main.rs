use clap::{Arg, Command,ArgAction};
use std::string::String;

fn main() {
    let matches = Command::new("rust-echo")
        .version("0.1")
        .author("kalki")
        .about("simpler version of command --echo--")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .help("Do not print newline")
                .short('n')
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
