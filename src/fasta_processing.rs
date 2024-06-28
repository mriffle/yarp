// src/fasta_processing.rs

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::config::Config;
use crate::decoy_generation::{write_decoy_entry, fix_sequence};

pub fn process_fasta(config: &Config, _log_file: &mut File) -> std::io::Result<usize> {
    let input_file = File::open(&config.input_path)?;
    let reader = BufReader::new(input_file);

    let mut writer = std::io::stdout();

    let mut current_header = String::new();
    let mut current_sequence = String::new();
    let mut entry_count = 0;

    let mut rng = StdRng::seed_from_u64(config.seed);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_header.is_empty() {
                let fixed_sequence = fix_sequence(&current_sequence);
                write_entry(&mut writer, &current_header, &fixed_sequence)?;
                write_decoy_entry(config, &mut writer, &current_header, &fixed_sequence, &mut rng)?;
                entry_count += 1;
                current_sequence.clear();
            }
            current_header = line;
        } else {
            current_sequence.push_str(line.trim());
        }
    }

    if !current_header.is_empty() {
        let fixed_sequence = fix_sequence(&current_sequence);
        write_entry(&mut writer, &current_header, &fixed_sequence)?;
        write_decoy_entry(config, &mut writer, &current_header, &fixed_sequence, &mut rng)?;
        entry_count += 1;
    }

    writer.flush()?;
    Ok(entry_count)
}

fn write_entry<W: Write>(writer: &mut W, header: &str, sequence: &str) -> std::io::Result<()> {
    writeln!(writer, "{}", header)?;
    writeln!(writer, "{}", sequence)?;
    Ok(())
}
