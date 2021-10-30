/* A shell implementation in Rust */

use std::process::Command;
use std::io;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use colored::*;

fn printable_filename(file_name: String, metadata: fs::Metadata) -> Option<ColoredString>  {
    if file_name.starts_with(".") {
        return None;
    }
    if metadata.is_dir() {
        return Some(file_name.blue());
    }
    if metadata.file_type().is_symlink() {
        return Some(file_name.cyan());
    }
    // Executable; will only work on POSIX; I don't care about Windows
    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 != 0 {
        return Some(file_name.green());
    }
    return Some(file_name.white());
}

fn ls() -> io::Result<()> {
    let mut filenames = Vec::new();
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let metadata = entry.metadata()?;
        let str_file_name = file_name.into_string();
        let formatted_file_name = match str_file_name {
            Ok(str_file_name) => printable_filename(str_file_name, metadata),
            Err(_str_file_name) => continue, // invalid unicode name
        };
        if let Some(formatted_file_name) = formatted_file_name {
            filenames.push(formatted_file_name);
        }
    }
    filenames.sort_by(|a, b| (*&a).cmp(*&b));
    for file_name in &filenames {
        print!("{}  ", file_name);
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
    print!("\n");
}
