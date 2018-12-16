use crate::reactor::perform_reactions;

fn filter_polymer(polymer: impl Iterator<Item = char>, c: char) -> impl Iterator<Item = char> {
    polymer.filter(move |ch| !ch.eq_ignore_ascii_case(&c))
}

pub(crate) fn find_bad_unit(polymer: &(impl Iterator<Item = char> + Clone)) -> (char, String) {
    let chars = b'a'..=b'z';

    chars
        .map(char::from)
        .map(|c| (c, perform_reactions(filter_polymer(polymer.clone(), c))))
        .min_by_key(|(_, polymer)| polymer.len())
        .unwrap() // safe because initial set is statically non-empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        macro_rules! test_filter {
            ($input:expr, $ch: expr, $expected:expr) => {
                assert_eq!(
                    filter_polymer($input.chars(), $ch).collect::<String>(),
                    $expected
                );
            };
        }

        test_filter!("dabAcCaCBAcCcaDA", 'a', "dbcCCBcCcD");
        test_filter!("dabAcCaCBAcCcaDA", 'b', "daAcCaCAcCcaDA");
        test_filter!("dabAcCaCBAcCcaDA", 'C', "dabAaBAaDA");
        test_filter!("dabAcCaCBAcCcaDA", 'D', "abAcCaCBAcCcaA");
    }

    #[test]
    fn test_finder() {
        assert_eq!(
            find_bad_unit(&"dabAcCaCBAcCcaDA".chars()),
            ('c', "daDA".to_owned())
        );
    }
}
