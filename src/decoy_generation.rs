// src/decoy_generation.rs

use std::io::Write;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use crate::config::{Config, DecoyMethod};
use crate::protease::digest_sequence;

pub fn write_decoy_entry<W: Write>(config: &Config, writer: &mut W, header: &str, sequence: &str, rng: &mut StdRng) -> std::io::Result<()> {
    let decoy_header = format!(">{}{}", config.decoy_prefix, &header[1..]);
    let peptides = digest_sequence(sequence, config.protease);
    let decoy_sequence = match config.decoy_method {
        DecoyMethod::Shuffle => best_shuffle_peptides(&peptides, sequence, config.num_shuffles, rng),
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

fn best_shuffle_peptides(peptides: &[String], original_sequence: &str, num_shuffles: usize, rng: &mut StdRng) -> String {
    let mut best_shuffle = String::new();
    let mut best_score = (usize::MAX, usize::MAX);

    for _ in 0..num_shuffles {
        let shuffled = shuffle_peptides(peptides, rng);
        let score = calculate_similarity(&shuffled, original_sequence);

        if score < best_score {
            best_score = score;
            best_shuffle = shuffled;
        }
    }

    best_shuffle
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

fn calculate_similarity(sequence1: &str, sequence2: &str) -> (usize, usize) {
    let same_adjacency = sequence1.chars().zip(sequence1.chars().skip(1))
        .zip(sequence2.chars().zip(sequence2.chars().skip(1)))
        .filter(|&((a1, a2), (b1, b2))| a1 == b1 && a2 == b2)
        .count();

    let same_position = sequence1.chars().zip(sequence2.chars()).filter(|&(a, b)| a == b).count();
    
    (same_adjacency, same_position)
}
