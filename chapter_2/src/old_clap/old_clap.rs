// This file is implementation for old clap version. Does not work!

use clap::App;

fn main() {
    println!("Hello, old clap!");
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("hayapo <example@example.com>")
        .about("Rust echo with old version of clap")
        .get_matches();
}