use std::collections::HashMap;
use std::iter::AdditiveIterator;

use super::levenshtein_similarity;

type FuzzyOverlapMap = HashMap<usize, (usize, f64)>;

pub fn fuzzy_jaccard_similarity(s1: &str, s2: &str, delta: f64) -> f64 {
    let (overlap, s1_token_count, s2_token_count) = fuzzy_overlap(s1, s2, delta);
    let weight = fuzzy_overlap_weight(overlap);

    weight / (s1_token_count as f64 + s2_token_count as f64 - weight)
}

fn fuzzy_overlap(s1: &str, s2: &str, delta: f64) -> (FuzzyOverlapMap, usize, usize) {
    let s1_tokens = s1.split(|&: c: char| c.is_whitespace()).collect::<Vec<&str>>();
    let s2_tokens = s2.split(|&: c: char| c.is_whitespace()).collect::<Vec<&str>>();

    let mut similarities : FuzzyOverlapMap = HashMap::new();

    for (i, &t2) in s2_tokens.iter().enumerate() {
        let mut max_sim = 0f64;
        let mut max_j = 0;

        for (j, &t1) in s1_tokens.iter().enumerate() {
            let lev_sim = levenshtein_similarity(t1, t2);

            if lev_sim >= delta && lev_sim > max_sim {
                max_sim = lev_sim;
                max_j = j;
            }
        }

        // FIXME: clone
        match similarities.get(&max_j).clone() {
            Some(&(_, sim)) if sim < max_sim => similarities[max_j] = (i, max_sim),
            None => {
                similarities.insert(max_j, (i, max_sim));
            },
            _ => ()
        }
    }

    (similarities, s1_tokens.len(), s2_tokens.len())
}

fn fuzzy_overlap_weight(map: FuzzyOverlapMap) -> f64 {
    map.values().map(|&(_, sim)| sim).sum()
}

#[cfg(test)]
mod test {
    use test::Bencher;
    use super::fuzzy_jaccard_similarity;

    #[test]
    fn value_from_paper() {
        assert_eq!(15f64 / 17f64, fuzzy_jaccard_similarity("nba mcgrady", "macgrady nba", 0.8));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| fuzzy_jaccard_similarity("nba mcgrady", "macgrady nba", 0.8))
    }
}

