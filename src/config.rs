use std::path::PathBuf;
use clap::Parser;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum DecoyMethod {
    Shuffle,
    Reverse,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Protease {
    Trypsin,
}

#[derive(Parser, Debug)]
#[command(name = "YARP")]
#[command(author = "Michael Riffle <mriffle@uw.edu>")]
#[command(version = "1.3.0")]
#[command(about = "Yet Another Rearranger of Peptides", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Config {
    /// Input FASTA file
    #[arg(long = "fasta-file", value_name = "FILE", required = true)]
    pub fasta_file: PathBuf,

    /// Decoy generation method
    #[arg(long, value_enum, default_value_t = DecoyMethod::Reverse)]
    pub decoy_method: DecoyMethod,

    /// Prefix for decoy sequence headers
    #[arg(long, default_value = "DECOY_")]
    pub decoy_prefix: String,

    /// Random seed for shuffling
    #[arg(long, default_value_t = 1337)]
    pub seed: u64,

    /// Protease for in silico digestion
    #[arg(long, value_enum, default_value_t = Protease::Trypsin)]
    pub protease: Protease,

    /// Number of shuffles to perform when using 'shuffle' method
    #[arg(long, default_value_t = 1)]
    pub num_shuffles: usize,
}

impl Config {
    pub fn new() -> Result<Self, clap::Error> {
        let config = Self::parse();
        if !config.fasta_file.exists() {
            return Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("Input FASTA file does not exist: {:?}", config.fasta_file),
            ));
        }
        Ok(config)
    }
}