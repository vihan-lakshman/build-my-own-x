/* A shell implementation in Rust */

use std::process::Command;
use std::io;
use std::fs;

fn print_filename(file_name: String) {
    if file_name.starts_with(".") {
        return;
    }
    println!("{:?}", file_name);
}

fn ls() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let str_file_name = file_name.into_string();
        match str_file_name {
            Ok(str_file_name) => print_filename(str_file_name),
            Err(_str_file_name) => (), // invalid unicode name
        }
    }
    Ok(())
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();

    // read_line leaves a trailing newline
    let command = input.trim();

    if command == "ls" {
        let output = ls();
        if let Err(output) = output {
            println!("{:?}", output);
        }
    } else {
        Command::new(command)
            .spawn()
            .unwrap();
    }
}
