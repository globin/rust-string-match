#[cfg(test)]
extern crate test;

pub use levenshtein::{levenshtein_distance, levenshtein_similarity};
pub use lcs::{lcs_length, lcs_similarity};

mod levenshtein;
mod lcs;
