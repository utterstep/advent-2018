fn should_react(left: char, right: char) -> bool {
    (left.is_ascii_lowercase() ^ right.is_ascii_lowercase() && left.eq_ignore_ascii_case(&right))
}

pub(crate) fn perform_reactions(polymer: impl Iterator<Item = char>) -> String {
    let mut stack = Vec::new();

    for unit in polymer {
        match stack.last() {
            Some(other_unit) => {
                if should_react(unit, *other_unit) {
                    stack.pop();
                } else {
                    stack.push(unit);
                }
            }
            None => stack.push(unit),
        }
    }

    stack.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_checker() {
        assert!(should_react('a', 'A'));
        assert!(should_react('A', 'a'));
        assert!(should_react('z', 'Z'));
        assert!(should_react('F', 'f'));

        assert!(!should_react('a', 'a'));
        assert!(!should_react('a', 'b'));
        assert!(!should_react('a', 'B'));
        assert!(!should_react('A', 'A'));
        assert!(!should_react('B', 'a'));
    }

    #[test]
    fn test_reactor() {
        assert_eq!(perform_reactions("aA".chars()), "");
        assert_eq!(perform_reactions("abBA".chars()), "");
        assert_eq!(perform_reactions("abAB".chars()), "abAB");
        assert_eq!(perform_reactions("aabAAB".chars()), "aabAAB");
    }
}
