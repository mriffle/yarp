mod config;
mod fasta_processing;
mod decoy_generation;
mod protease;
mod utils;

use std::process;
use std::time::Instant;
use std::fs::OpenOptions;

use crate::config::Config;
use crate::fasta_processing::process_fasta;
use crate::utils::log_and_print;

const VERSION: &str = "1.1.0";
const PROGRAM_NAME: &str = "YARP (Yet Another Rearranger of Peptides)";

fn main() {
    let start_time = Instant::now();

    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let log_path = format!("{}.log", config.fasta_file.display());
    let mut log_file = match OpenOptions::new().create(true).write(true).truncate(true).open(&log_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening log file: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = log_and_print(&mut log_file, &format!("{} v{}", PROGRAM_NAME, VERSION)) {
        eprintln!("Error writing to log: {}", e);
    }
    if let Err(e) = log_and_print(&mut log_file, &format!("Configuration: {:?}", config)) {
        eprintln!("Error writing to log: {}", e);
    }

    match process_fasta(&config, &mut log_file) {
        Ok(count) => {
            let duration = start_time.elapsed();
            if let Err(e) = log_and_print(&mut log_file, &format!("Processed {} FASTA entries", count)) {
                eprintln!("Error writing to log: {}", e);
            }
            if let Err(e) = log_and_print(&mut log_file, &format!("Total runtime: {:?}", duration)) {
                eprintln!("Error writing to log: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error processing FASTA file: {}", e);
            process::exit(1);
        }
    }
}