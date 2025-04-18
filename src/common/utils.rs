use std::io::{self, Write};

pub fn get_input() -> String {
    print!("$> ");
    io::stdout().flush().expect("could not flush output stream");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("could not take in input");
    buf
}
