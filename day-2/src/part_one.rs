use crate::utils::{fill_counter, Counter};

fn has_exactly_n_repetitions(counter: &Counter, n: i64) -> bool {
    counter.iter().any(|(_, &count)| count == n)
}

pub fn compute_checksum<T, ItemT>(ids: T) -> u64
where
    T: Iterator<Item = ItemT>,
    ItemT: AsRef<str> + Sized,
{
    let mut doubles = 0;
    let mut triples = 0;

    for id in ids {
        let counter = fill_counter(id.as_ref());

        if has_exactly_n_repetitions(&counter, 2) {
            doubles += 1;
        }

        if has_exactly_n_repetitions(&counter, 3) {
            triples += 1;
        }
    }

    doubles * triples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_checksum() {
        let ids = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        assert_eq!(compute_checksum(ids.iter()), 12);
    }
}
