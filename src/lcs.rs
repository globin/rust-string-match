use std::cmp::max;

pub fn lcs_length(s1: &str, s2: &str) -> u32 {
    if s1.is_empty() || s2.is_empty() { return 0u32; }

    let mut table : Vec<Vec<uint>> = Vec::from_fn(s1.len() + 1, |_| Vec::from_elem(s2.len() + 1, 0));
    let mut result = 0u32;

    for i in range(0, s1.len()) {
        for j in range(0, s2.len()) {
            table[i + 1][j + 1] = if s1.char_at(i) == s2.char_at(j) {
                result = max(result, table[i][j] as u32 + 1);
                table[i][j] + 1
            } else {
                0
            };
        }
    }

    return result;
}

pub fn lcs_similarity(s1: &str, s2: &str) ->f64 {
    match max(s1.len(), s2.len()) {
        0 => 1f64,
        max_len @ _ => lcs_length(s1, s2) as f64 / max_len as f64
    }
}

#[cfg(test)]
mod test {
    use test::Bencher;
    use super::{lcs_length, lcs_similarity};

    #[test]
    fn same() {
        assert_eq!(3u32, lcs_length("foo", "foo"))
    }

    #[test]
    fn empty() {
        assert_eq!(0u32, lcs_length("test", ""));
        assert_eq!(0u32, lcs_length("", "test"))
    }

    #[test]
    fn distance_test() {
        assert_eq!(1u32, lcs_length("fnord", "foo"));
        assert_eq!(4u32, lcs_length("aatest", "bbtest1"))
    }

    #[test]
    fn similarity_test() {
        assert_eq!(1f64, lcs_similarity("test", "test"));
        assert_eq!(0.8f64, lcs_similarity("test", "test1"))
    }


    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| lcs_length("alarmismus", "alarminmay"))
    }
}

