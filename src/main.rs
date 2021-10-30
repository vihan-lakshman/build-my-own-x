/* A shell implementation in Rust */

use std::process::Command;
use std::io;
use std::fs;

fn ls() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        // if file_name.into_string()?.starts_with(".") {
        //     continue;
        // }
        println!("{:?}", file_name);
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
