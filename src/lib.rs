#[cfg(test)]
extern crate test;

pub use levenshtein::{levenshtein_distance, levenshtein_similarity};
pub use lcs::{lcs_length, lcs_similarity};
pub use fuzzy_jaccard::fuzzy_jaccard_similarity;

mod lcs;
mod fuzzy_jaccard;
mod levenshtein;
