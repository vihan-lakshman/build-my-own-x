/* A shell implementation in Rust */

use std::process::Command;
use std::io;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();

    // read_line leaves a trailing newline
    let command = input.trim();

    Command::new(command)
        .spawn()
        .unwrap();
}
