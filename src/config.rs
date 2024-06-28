// src/config.rs

use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub enum DecoyMethod {
    Shuffle,
    Reverse,
}

#[derive(Clone, Copy, Debug)]
pub enum Protease {
    Trypsin,
}

#[derive(Debug)]
pub struct Config {
    pub input_path: String,
    pub decoy_method: DecoyMethod,
    pub decoy_prefix: String,
    pub seed: u64,
    pub protease: Protease,
    pub num_shuffles: usize,
}

pub fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut config = Config {
        input_path: String::new(),
        decoy_method: DecoyMethod::Reverse,
        decoy_prefix: String::from("DECOY_"),
        seed: 1337,
        protease: Protease::Trypsin,
        num_shuffles: 1,
    };

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if let Some((key, value)) = arg.split_once('=') {
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
                "--num-shuffles" => config.num_shuffles = value.parse().map_err(|_| format!("Invalid num-shuffles value: {}", value))?,
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        } else {
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
                "--num-shuffles" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing value for --num-shuffles".to_string());
                    }
                    config.num_shuffles = args[i].parse().map_err(|_| format!("Invalid num-shuffles value: {}", args[i]))?;
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
