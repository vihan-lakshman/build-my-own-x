/* A shell implementation in Rust */

use std::process::Command;
use std::io;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use colored::*;

fn print_filename(file_name: String, metadata: fs::Metadata) {
    if file_name.starts_with(".") {
        return;
    }
    if metadata.is_dir() {
        print!("{}  ", file_name.blue());
        return;
    }
    if metadata.file_type().is_symlink() {
        print!("{}  ", file_name.cyan());
        return;
    }
    // Executable; will only work on POSIX; I don't care about Windows
    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 != 0 {
        print!("{}  ", file_name.green());
        return;
    }
    print!("{}  ", file_name.white());
}

fn ls() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let metadata = entry.metadata()?;
        let str_file_name = file_name.into_string();
        match str_file_name {
            Ok(str_file_name) => print_filename(str_file_name, metadata),
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
