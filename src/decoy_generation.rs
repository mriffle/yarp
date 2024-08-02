// src/decoy_generation.rs

use std::io::Write;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use crate::config::{Config, DecoyMethod};
use crate::protease::digest_sequence;

// Type aliases for our data structures
type PeptideCache = HashMap<String, String>;
type AminoAcidMasses = HashMap<char, f64>;

pub fn write_decoy_entry<W: Write>(
    config: &Config,
    writer: &mut W,
    header: &str,
    sequence: &str,
    rng: &mut StdRng,
    peptide_cache: &mut PeptideCache
) -> std::io::Result<()> {
    let decoy_header = format!(">{}{}", config.decoy_prefix, &header[1..]);
    let peptides = digest_sequence(sequence, config.protease);
    let decoy_sequence = match config.decoy_method {
        DecoyMethod::Shuffle => best_shuffle_peptides(&peptides, config.num_shuffles, rng, peptide_cache),
        DecoyMethod::Reverse => reverse_peptides(&peptides),
    };
    writeln!(writer, "{}", decoy_header)?;
    writeln!(writer, "{}", decoy_sequence)?;
    Ok(())
}

pub fn fix_sequence(sequence: &str) -> String {
    sequence.trim_end_matches('*').to_string()
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

fn best_shuffle_peptides(
    peptides: &[String],
    num_shuffles: usize,
    rng: &mut StdRng,
    peptide_cache: &mut PeptideCache
) -> String {
    peptides.iter()
        .map(|peptide| {
            if let Some(cached) = peptide_cache.get(peptide) {
                cached.clone()
            } else {
                let shuffled = best_shuffle_peptide(peptide, num_shuffles, rng);
                peptide_cache.insert(peptide.clone(), shuffled.clone());
                shuffled
            }
        })
        .collect()
}

fn best_shuffle_peptide(peptide: &str, num_shuffles: usize, rng: &mut StdRng) -> String {
    if peptide.len() <= 1 {
        return peptide.to_string();
    }

    if peptide.len() == 2 && (peptide.ends_with('K') || peptide.ends_with('R')) {
        return peptide.to_string();
    }

    let original_fragment_ions = calculate_fragment_ion_masses(peptide);
    let mut best_shuffle = peptide.to_string();
    let mut best_score = usize::MAX;

    // Determine the minimum possible score
    let min_possible_score = if peptide.ends_with('K') || peptide.ends_with('R') {
        2
    } else {
        0
    };

    for _ in 0..num_shuffles {
        let shuffled = shuffle_single_peptide(peptide, rng);
        let score = calculate_similarity_with_original(&shuffled, &original_fragment_ions);
        if score < best_score {
            best_score = score;
            best_shuffle = shuffled;
        }

        // Stop if we find a shuffle with the minimum possible score
        if best_score == min_possible_score {
            break;
        }
    }

    best_shuffle
}

fn shuffle_single_peptide(peptide: &str, rng: &mut StdRng) -> String {
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
}

fn create_amino_acid_masses() -> AminoAcidMasses {
    [
        ('G', 57.021463735), ('A', 71.037113805), ('S', 87.032028435),
        ('P', 97.052763875), ('V', 99.068413945), ('T', 101.047678505),
        ('C', 103.009184505), ('L', 113.084064015), ('I', 113.084064015),
        ('N', 114.042927470), ('D', 115.026943065), ('Q', 128.058577540),
        ('K', 128.094963050), ('E', 129.042593135), ('M', 131.040484645),
        ('H', 137.058911875), ('F', 147.068413945), ('U', 150.953633405),
        ('R', 156.101111050), ('Y', 163.063328575), ('W', 186.079312980),
        ('O', 237.147726925),
    ].iter().cloned().collect()
}

fn calculate_fragment_ion_masses(peptide: &str) -> HashSet<i64> {
    let amino_acid_masses = create_amino_acid_masses();
    let mut fragment_ions = HashSet::new();
    let mut y_mass = 0.0;
    let mut b_mass = 0.0;
    let peptide_chars: Vec<char> = peptide.chars().collect();
    let peptide_len = peptide_chars.len();

    for i in 0..peptide_len - 1 {  // Changed to stop before the last amino acid
        let y_aa = peptide_chars[i];
        let b_aa = peptide_chars[peptide_len - 1 - i];

        y_mass += amino_acid_masses.get(&y_aa).copied().unwrap_or_else(|| {
            eprintln!("Unknown amino acid '{}' found at position {} in peptide. Using mass 0.0.", y_aa, i + 1);
            0.0
        });
        b_mass += amino_acid_masses.get(&b_aa).copied().unwrap_or_else(|| {
            eprintln!("Unknown amino acid '{}' found at position {} from end in peptide. Using mass 0.0.", b_aa, peptide_len - i);
            0.0
        });

        fragment_ions.insert(y_mass.round() as i64);
        fragment_ions.insert(b_mass.round() as i64);
    }

    fragment_ions
}

fn calculate_similarity_with_original(
    shuffled_peptide: &str,
    original_fragment_ions: &HashSet<i64>
) -> usize {
    let shuffled_fragment_ions = calculate_fragment_ion_masses(shuffled_peptide);

    let intersection = shuffled_fragment_ions.intersection(original_fragment_ions).cloned().collect::<HashSet<_>>();
    let intersection_size = intersection.len();

    intersection_size
}