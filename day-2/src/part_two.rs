use std::fmt::Debug;

fn sort_input<T, ItemT>(input: T) -> Vec<ItemT>
where
    T: Iterator<Item = ItemT>,
    ItemT: AsRef<str> + Sized + Ord,
{
    let mut input = input.collect::<Vec<_>>();

    input.sort_unstable_by_key(|s| {
        let mut chars_vec = s.as_ref().chars().collect::<Vec<_>>();
        chars_vec.sort_unstable();

        chars_vec
    });

    input
}

fn common_substring(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect::<String>()
}

fn get_valid(s1: &str, s2: &str) -> Option<String> {
    let common = common_substring(s1, s2);

    if common.len() == s1.len() - 1 {
        Some(common)
    } else {
        None
    }
}

pub fn find_valid_id<T, ItemT>(input: T) -> Option<String>
where
    T: Iterator<Item = ItemT>,
    ItemT: AsRef<str> + Debug + Sized + Ord + ToString,
{
    let sorted_input = sort_input(input);

    let valid_id = sorted_input
        .windows(2)
        .find_map(|window| {
            get_valid(window[0].as_ref(), window[1].as_ref())
        });

    valid_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid() {
        assert_eq!(
            get_valid("abcde", "axcye"),
            None
        );

        assert_eq!(
            get_valid("fghij", "fguij"),
            Some("fgij".to_owned())
        );
    }

    #[test]
    fn test_find_valid_id() {
        assert_eq!(
            find_valid_id(vec![
                "abcde",
                "fghij",
                "klmno",
                "pqrst",
                "fguij",
                "axcye",
                "wvxyz",
            ].iter()),
            Some("fgij".to_owned())
        )
    }
}
