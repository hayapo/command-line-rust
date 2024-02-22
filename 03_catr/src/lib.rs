use std::error::Error;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cmd = Command::new("catr")
        .version("0.1.0")
        .author("hayapo <example@examle.com")
        .about("Rust cat");

    let matches = cmd.arg(
        Arg::new("files")
            .value_name("FILE")
            .help("Input file(s)")
            .default_value("-")
            .num_args(1..),
    )
    .arg(
        Arg::new("number")
            .long("number")
            .short('n')
            .help("Number lines")
            .action(clap::ArgAction::SetTrue),
    )
    .arg(
        Arg::new("number-nonblank")
            .long("number-nonblank")
            .short('b')
            .help("Number nonblank lines")
            .conflicts_with("number")
            .action(clap::ArgAction::SetTrue),
    )
    .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    
    Ok(Config {
        files,
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number-nonblank")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                for line_result in file.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        line_num += 1;
                        println!("{:6}\t{}", line_num, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            line_num += 1;
                            println!("{:6}\t{}", line_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
