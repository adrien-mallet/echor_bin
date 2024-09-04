use std::io::Write;

use echor_bin::print_arg;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut is_option = true;
    let mut omit_newline = false;
    let mut escape_char = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" if is_option => omit_newline = true,
            "-e" if is_option => escape_char = true,
            "-E" if is_option => escape_char = false,
            _ if is_option => {
                is_option = false;
                print_arg(arg, escape_char);
            }
            _ => {
                print!(" ");
                print_arg(arg, escape_char)
            }
        }
    }

    std::io::stdout().flush().unwrap();

    if !omit_newline {
        println!("");
    }
}
