use std::io::{self, Write, Read};


fn main() {
    let mut stdout = io::stdout();

    stdout.write_all(b"hello world!").unwrap();

    let stdin =  io::stdin().bytes();
}