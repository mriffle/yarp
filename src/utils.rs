// src/utils.rs

use std::fs::File;
use std::io::Write;

pub fn log_and_print(log_file: &mut File, message: &str) -> std::io::Result<()> {
    writeln!(log_file, "{}", message)?;
    eprintln!("{}", message);
    Ok(())
}