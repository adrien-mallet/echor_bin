pub fn print_arg(arg: String, enable: bool) {
    let mut chars = arg.chars();
    while let Some(c) = chars.next() {
        if !enable || c != '\\' {
            print!("{}", c);
            continue;
        }

        if let Some(n) = chars.next() {
            match n {
                'n' => print!("\n"),
                't' => print!("\t"),
                'v' => print!("\x0b"),
                _ => print!("\\{}", n),
            };
            continue;
        }

        print!("\\");
    }
}
