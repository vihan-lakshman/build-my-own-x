/* A shell implementation in Rust */

use std::process::Command;
use std::io;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use colored::*;

fn calculate_num_columns(filenames: &Vec<ColoredString>) -> (usize, usize) {
    let termsize::Size {rows: _, cols} = termsize::get().unwrap();
    let mut longest_name_length: u16 = 0;
    for filename in filenames {
        let length = (*&filename).len() as u16;
        if length > longest_name_length {
            longest_name_length = length;
        }
    }
    let col_length = longest_name_length + 2;
    return ((cols/col_length) as usize, col_length as usize)
}

fn print_filenames(mut filenames: Vec<ColoredString>) {
    let (num_cols, col_length) = calculate_num_columns(&filenames);
    let single_row = num_cols > filenames.len();
    filenames.sort_by(|a, b| (*&a).cmp(*&b));
    let mut col_count = 0;
    for filename in &filenames {
        if single_row {
            print!("{}  ", filename);
        } else {
            print!("{:width$}", filename, width=col_length);
        }
        col_count += 1;
        if col_count == num_cols {
            print!("\n");
            col_count = 0;
        }
    }
}

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

fn ls() -> io::Result<Vec<ColoredString>> {
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
    Ok(filenames)
}


// fn grep(pattern: String, filenames: Vec<ColoredString>) {
//     return filenames;
// }

fn main() {
    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();

        // read_line leaves a trailing newline
        let commands = input.trim().split("|");

        let mut prev_output: Option<Vec<ColoredString>> = None;

        for command in commands {
            let mut command_and_flags = command.trim().split(" ");
            if let Some(command) = command_and_flags.nth(0) {
                match command {
                    "ls" => {
                        let output = ls();
                        if let Err(output) = output {
                            println!("{:?}", output);
                            return;
                        } else if let Ok(output) = output {
                            prev_output = Some(output)
                        }
                    },
                    "exit" => {
                        print!("\n");
                        return;
                    },
                    "" => {},
                    _ => {
                        Command::new(command)
                            .spawn()
                            .unwrap();
                    }
                }
            }
        }
        if let Some(output) = prev_output {
            print_filenames(output);
        }
        print!("\n");
    }
}
