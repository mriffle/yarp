use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, stdout};
use std::path::Path;
use std::env;
use std::process;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

const VERSION: &str = "0.0.3";
const PROGRAM_NAME: &str = "YARP (Yet Another Rearranger of Peptides)";

#[derive(Clone, Copy, Debug)]
enum DecoyMethod {
    Shuffle,
    Reverse,
}

#[derive(Clone, Copy, Debug)]
enum Protease {
    Trypsin,
}

#[derive(Debug)]
struct Config {
    input_path: String,
    decoy_method: DecoyMethod,
    decoy_prefix: String,
    seed: u64,
    protease: Protease,
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "--help" => {
            print_usage();
            process::exit(0);
        },
        "--version" => {
            println!("{} v{}", PROGRAM_NAME, VERSION);
            process::exit(0);
        },
        _ => {}
    }

    let config = match parse_args(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let log_path = format!("{}.log", config.input_path);
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
            let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            if let Err(e) = log_and_print(&mut log_file, &format!("Processed {} FASTA entries", count)) {
                eprintln!("Error writing to log: {}", e);
            }
            if let Err(e) = log_and_print(&mut log_file, &format!("Started at: {}", start_time.elapsed().as_secs())) {
                eprintln!("Error writing to log: {}", e);
            }
            if let Err(e) = log_and_print(&mut log_file, &format!("Completed at: {}", end_time)) {
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

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut config = Config {
        input_path: String::new(),
        decoy_method: DecoyMethod::Reverse,
        decoy_prefix: String::from("DECOY_"),
        seed: 1337, // Default seed
        protease: Protease::Trypsin,
    };

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if let Some((key, value)) = arg.split_once('=') {
            // Handle arguments in the format --key=value
            match key {
                "--fasta" => config.input_path = value.to_string(),
                "--method" => {
                    config.decoy_method = match value {
                        "shuffle" => DecoyMethod::Shuffle,
                        "reverse" => DecoyMethod::Reverse,
                        _ => return Err(format!("Invalid decoy method: {}. Use 'shuffle' or 'reverse'.", value)),
                    }
                },
                "--decoy-string" => config.decoy_prefix = value.to_string(),
                "--seed" => config.seed = value.parse().map_err(|_| format!("Invalid seed value: {}", value))?,
                "--protease" => {
                    config.protease = match value {
                        "trypsin" => Protease::Trypsin,
                        _ => return Err(format!("Invalid protease: {}. Only 'trypsin' is currently supported.", value)),
                    }
                },
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        } else {
            // Handle arguments in the format --key value
            match arg.as_str() {
                "--fasta" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing value for --fasta".to_string());
                    }
                    config.input_path = args[i].clone();
                },
                "--method" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing value for --method".to_string());
                    }
                    config.decoy_method = match args[i].as_str() {
                        "shuffle" => DecoyMethod::Shuffle,
                        "reverse" => DecoyMethod::Reverse,
                        _ => return Err(format!("Invalid decoy method: {}. Use 'shuffle' or 'reverse'.", args[i])),
                    };
                },
                "--decoy-string" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing value for --decoy-string".to_string());
                    }
                    config.decoy_prefix = args[i].clone();
                },
                "--seed" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing value for --seed".to_string());
                    }
                    config.seed = args[i].parse().map_err(|_| format!("Invalid seed value: {}", args[i]))?;
                },
                "--protease" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing value for --protease".to_string());
                    }
                    config.protease = match args[i].as_str() {
                        "trypsin" => Protease::Trypsin,
                        _ => return Err(format!("Invalid protease: {}. Only 'trypsin' is currently supported.", args[i])),
                    };
                },
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }
        i += 1;
    }

    if config.input_path.is_empty() {
        return Err("Input FASTA file must be specified with --fasta=FASTA".to_string());
    }

    if !Path::new(&config.input_path).exists() {
        return Err(format!("FASTA file does not exist: {}", config.input_path));
    }

    Ok(config)
}

fn process_fasta(config: &Config, log_file: &mut File) -> std::io::Result<usize> {
    let input_file = File::open(&config.input_path)?;
    let reader = BufReader::new(input_file);
    
    let mut writer = stdout();

    let mut current_header = String::new();
    let mut current_sequence = String::new();
    let mut entry_count = 0;

    // Initialize the RNG with the seed
    let mut rng = StdRng::seed_from_u64(config.seed);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_header.is_empty() {
                // Fix and process the previous entry
                let fixed_sequence = fix_sequence(&current_sequence);
                write_entry(&mut writer, &current_header, &fixed_sequence)?;
                write_decoy_entry(config, &mut writer, &current_header, &fixed_sequence, &mut rng)?;
                entry_count += 1;
                if let Err(e) = log_and_print(log_file, &format!("Processed entry: {}", current_header)) {
                    eprintln!("Error writing to log: {}", e);
                }
                current_sequence.clear();
            }
            current_header = line;
        } else {
            current_sequence.push_str(line.trim());
        }
    }

    // Fix, process and write the last entry
    if !current_header.is_empty() {
        let fixed_sequence = fix_sequence(&current_sequence);
        write_entry(&mut writer, &current_header, &fixed_sequence)?;
        write_decoy_entry(config, &mut writer, &current_header, &fixed_sequence, &mut rng)?;
        entry_count += 1;
        if let Err(e) = log_and_print(log_file, &format!("Processed entry: {}", current_header)) {
            eprintln!("Error writing to log: {}", e);
        }
    }

    writer.flush()?;
    Ok(entry_count)
}

fn fix_sequence(sequence: &str) -> String {
    sequence.trim_end_matches('*').to_string()
}

fn write_entry<W: Write>(writer: &mut W, header: &str, sequence: &str) -> std::io::Result<()> {
    writeln!(writer, "{}", header)?;
    for chunk in sequence.as_bytes().chunks(60) {
        writeln!(writer, "{}", std::str::from_utf8(chunk).unwrap())?;
    }
    Ok(())
}

fn write_decoy_entry<W: Write>(config: &Config, writer: &mut W, header: &str, sequence: &str, rng: &mut StdRng) -> std::io::Result<()> {
    let decoy_header = format!("{}{}", config.decoy_prefix, &header[1..]);
    let peptides = digest_sequence(sequence, config.protease);
    let decoy_sequence = match config.decoy_method {
        DecoyMethod::Shuffle => shuffle_peptides(&peptides, rng),
        DecoyMethod::Reverse => reverse_peptides(&peptides),
    };
    
    writeln!(writer, "{}", decoy_header)?;
    for chunk in decoy_sequence.as_bytes().chunks(60) {
        writeln!(writer, "{}", std::str::from_utf8(chunk).unwrap())?;
    }
    Ok(())
}

fn digest_sequence(sequence: &str, protease: Protease) -> Vec<String> {
    match protease {
        Protease::Trypsin => digest_trypsin(sequence),
    }
}

fn digest_trypsin(sequence: &str) -> Vec<String> {
    let mut peptides = Vec::new();
    let mut current_peptide = String::new();

    for (i, c) in sequence.chars().enumerate() {
        current_peptide.push(c);
        if (c == 'K' || c == 'R') && sequence.chars().nth(i + 1) != Some('P') {
            peptides.push(current_peptide);
            current_peptide = String::new();
        }
    }

    if !current_peptide.is_empty() {
        peptides.push(current_peptide);
    }

    peptides
}

fn reverse_peptides(peptides: &[String]) -> String {
    peptides.iter().map(|peptide| {
        if peptide.ends_with('K') || peptide.ends_with('R') {
            let (body, last) = peptide.split_at(peptide.len() - 1);
            format!("{}{}", body.chars().rev().collect::<String>(), last)
        } else {
            peptide.chars().rev().collect()
        }
    }).collect()
}

fn shuffle_peptides(peptides: &[String], rng: &mut StdRng) -> String {
    peptides.iter().map(|peptide| {
        if peptide.ends_with('K') || peptide.ends_with('R') {
            let (body, last) = peptide.split_at(peptide.len() - 1);
            let mut chars: Vec<char> = body.chars().collect();
            chars.shuffle(rng);
            format!("{}{}", chars.into_iter().collect::<String>(), last)
        } else {
            let mut chars: Vec<char> = peptide.chars().collect();
            chars.shuffle(rng);
            chars.into_iter().collect()
        }
    }).collect()
}

fn print_usage() {
    println!("{} v{}", PROGRAM_NAME, VERSION);
    println!("Usage: yarp --fasta=FASTA [OPTIONS]");
    println!("Options:");
    println!("  --fasta=FASTA        Input FASTA file (required)");
    println!("  --method=METHOD      Decoy generation method: 'shuffle' or 'reverse' (default: reverse)");
    println!("  --decoy-string=STRING Prefix for decoy sequence headers (default: DECOY_)");
    println!("  --seed=SEED          Random seed for shuffling (default: 1337)");
    println!("  --protease=PROTEASE  Protease for in silico digestion: 'trypsin' (default: trypsin)");
    println!("  --help               Print this help message");
    println!("  --version            Print version information");
}

fn log_and_print(log_file: &mut File, message: &str) -> std::io::Result<()> {
    writeln!(log_file, "{}", message)?;
    eprintln!("{}", message);
    Ok(())
}

