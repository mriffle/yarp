// src/utils.rs

use std::fs::File;
use std::io::Write;

pub fn print_usage(program_name: &str, version: &str) {
    println!("{} v{}", program_name, version);
    println!("Usage: yarp --fasta=FASTA [OPTIONS]");
    println!("Options:");
    println!("  --fasta=FASTA        Input FASTA file (required)");
    println!("  --method=METHOD      Decoy generation method: 'shuffle' or 'reverse' (default: reverse)");
    println!("  --decoy-string=STRING Prefix for decoy sequence headers (default: DECOY_)");
    println!("  --seed=SEED          Random seed for shuffling (default: 1337)");
    println!("  --protease=PROTEASE  Protease for in silico digestion: 'trypsin' (default: trypsin)");
    println!("  --num-shuffles=N     Number of shuffles to perform when using 'shuffle' method (default: 1)");
    println!("  --help               Print this help message");
    println!("  --version            Print version information");
}

pub fn log_and_print(log_file: &mut File, message: &str) -> std::io::Result<()> {
    writeln!(log_file, "{}", message)?;
    eprintln!("{}", message);
    Ok(())
}
