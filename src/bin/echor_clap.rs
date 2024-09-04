use clap::{arg, Command};
use echor_bin::print_arg;
use std::io::Write;

fn main() {
    let matches = Command::new("echor_clap")
        .version("0.1.0")
        .author("Adrien MALLET <adrien.mallet@gmail.com>")
        .about("Rust implementation of echo")
        .args([
            arg!(omit_newline: -n "remove trailing newline"),
            arg!(enable: -e "enable interpretation of backslashes escapes"),
            arg!(disable: -E "disable interpretation of backslashes escapes"),
            arg!(text: [TEXT] ... "list of argument to display"),
        ])
        .get_matches();

    let omit_newline = matches.get_flag("omit_newline");
    let mut enable = matches.get_flag("enable");
    if enable && matches.get_flag("disable") {
        enable = false;
    }
    if let Some(arg_text) = matches.get_many("text") {
        let text: Vec<String> = arg_text.cloned().collect();
        for (index, arg) in text.into_iter().enumerate() {
            if index != 0 {
                print!(" ");
            }
            print_arg(arg, enable);
        }
    }

    std::io::stdout().flush().unwrap();

    if !omit_newline {
        println!("");
    }
}
