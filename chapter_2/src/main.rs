use clap::{Command, Arg};

fn main() {
    let cmd = Command::new("echor")
        .version("0.1.0")
        .author("hayapo <example@examle.com")
        .about("Rust echo");

    let matches = cmd.arg(
        Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
            .short('n')
            .help("Do not print newline")
            .action(clap::ArgAction::SetTrue),
    )
    .get_matches();

    let text: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(String::as_str)
        .collect();
    
    let omit_newline = matches.get_flag("omit_newline");
    
    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}
