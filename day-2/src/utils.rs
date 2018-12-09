use std::collections::HashMap;

pub type Counter = HashMap<char, i64>;

pub fn fill_counter(id: &str) -> Counter {
    let mut counter = HashMap::new();

    for c in id.chars() {
        counter
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_counter() {
        let counter = fill_counter("aaabccdaaa");

        assert_eq!(counter.get(&'a'), Some(&6));
        assert_eq!(counter.get(&'b'), Some(&1));
        assert_eq!(counter.get(&'c'), Some(&2));
        assert_eq!(counter.get(&'d'), Some(&1));
        assert_eq!(counter.get(&'e'), None);
    }
}
