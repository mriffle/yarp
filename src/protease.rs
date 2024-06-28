// src/protease.rs

use crate::config::Protease;

pub fn digest_sequence(sequence: &str, protease: Protease) -> Vec<String> {
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
