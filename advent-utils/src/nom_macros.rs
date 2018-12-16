#[macro_export]
macro_rules! integer_parser {
    ($name:ident, $t:ty) => {
        named!($name<CompleteStr, $t>,
            map_res!(
                digit,
                |s: CompleteStr| str::parse(s.0)
            )
        );
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use nom::{call, digit, error_position, map_res, named, take_while, types::CompleteStr};

    #[test]
    fn test_integer_parser() {
        integer_parser!(parse_u64, u64);

        assert_eq!(parse_u64(CompleteStr("123")), Ok((CompleteStr(""), 123)));
    }
}
