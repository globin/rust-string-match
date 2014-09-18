use std::cmp::{min, max};

pub fn levenshtein_distance(s1: &str, s2: &str) -> u32 {
    if s1 == s2 { return 0u32; }
    if s1.is_empty() { return s2.len() as u32; }
    if s2.is_empty() { return s1.len() as u32; }

    let mut v0 : Vec<u32> = Vec::from_fn(s2.len() + 1, |idx| idx as u32);
    let mut v1 : Vec<u32> = Vec::from_elem(s2.len() + 1, 0);

    for i in range(0, s1.len()) {
        *v1.get_mut(0) = i as u32 + 1;

        for j in range(0, s2.len()) {
            let cost = if s1.char_at(i) == s2.char_at(j) { 0u32 } else { 1 };
            *v1.get_mut(j + 1) = min(min(v1[j] + 1, v0[j + 1] + 1), v0[j] + cost);
        }

        v0 = v1.clone();
    }

    return v1[s2.len()];
}

pub fn levenshtein_similarity(s1: &str, s2: &str) ->f64 {
    match max(s1.len(), s2.len()) {
        0 => 1f64,
        max_len @ _ => 1f64 - levenshtein_distance(s1, s2) as f64 / max_len as f64
    }
}

#[cfg(test)]
mod test {
    use test::Bencher;
    use super::{levenshtein_distance, levenshtein_similarity};

    #[test]
    fn same() {
        assert_eq!(0u32, levenshtein_distance("foo", "foo"))
    }

    #[test]
    fn empty() {
        assert_eq!(4u32, levenshtein_distance("test", ""))
        assert_eq!(4u32, levenshtein_distance("", "test"))
    }

    #[test]
    fn distance_test() {
        assert_eq!(3u32, levenshtein_distance("fnord", "foo"))
        assert_eq!(1u32, levenshtein_distance("test", "test1"))
    }

    #[test]
    fn similarity_test() {
        assert_eq!(1f64, levenshtein_similarity("test", "test"))
        assert_eq!(0.8f64, levenshtein_similarity("test", "test1"))
    }


    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| levenshtein_distance("alarmismus", "alarminmay"))
    }
}

