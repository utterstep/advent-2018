use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use advent_utils::integer_parser;

use nom::{digit, types::CompleteStr};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub enum Action {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct LogEntry {
    pub datetime: NaiveDateTime,
    pub action: Action,
}

impl FromStr for LogEntry {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_log_entry_from_str(s)
    }
}

integer_parser!(parse_u32, u32);

named!(parse_date<CompleteStr, NaiveDate>,
    map_res!(
        separated_list!(tag!("-"), parse_u32),
        |ymd: Vec<u32>| {
            if ymd.len() < 3 {
                return Err(format!("invalid YMD spec: {:?}", ymd));
            }

            NaiveDate::from_ymd_opt(ymd[0] as i32, ymd[1], ymd[2])
                .ok_or_else(|| format!("invalid YMD spec: {:?}", ymd))
        }
    )
);

named!(parse_time<CompleteStr, NaiveTime>,
    map_res!(
        separated_pair!(parse_u32, tag!(":"), parse_u32),
        |(hours, minutes): (u32, u32)| {
            NaiveTime::from_hms_opt(hours, minutes, 0)
                .ok_or_else(|| format!("invalid time spec: {}:{}", hours, minutes))
        }
    )
);

named!(parse_datetime<CompleteStr, NaiveDateTime>,
    map!(
        separated_pair!(parse_date, tag!(" "), parse_time),
        |(date, time)| date.and_time(time)
    )
);

named!(wakes_up<CompleteStr, Action>,
    map!(
        tag!("wakes up"),
        |_| Action::WakesUp
    )
);

named!(falls_asleep<CompleteStr, Action>,
    map!(
        tag!("falls asleep"),
        |_| Action::FallsAsleep
    )
);

// Guard #10 begins shift
named!(begins_shift<CompleteStr, Action>,
    do_parse!(
        tag!("Guard #") >>
        guard_id: terminated!(parse_u32, tag!(" begins shift")) >>
        (
            Action::BeginsShift(guard_id)
        )
    )
);

named!(parse_action<CompleteStr, Action>,
    alt!(falls_asleep | wakes_up | begins_shift)
);

named!(parse_log_entry<CompleteStr, LogEntry>,
    do_parse!(
        datetime: delimited!(tag!("["), parse_datetime, tag!("]")) >>
        tag!(" ") >>
        action: parse_action >>
        (
            LogEntry {
                datetime,
                action
            }
        )
    )
);

fn parse_log_entry_from_str(input: &str) -> Result<LogEntry, nom::ErrorKind> {
    parse_log_entry(CompleteStr(input))
        .map(|(_, result)| result)
        .map_err(|e| e.into_error_kind())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_datetime() {
        let good_cases = [
            (
                "1518-11-01 00:00",
                NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 0, 0),
            ),
            (
                "1518-11-03 00:29",
                NaiveDate::from_ymd(1518, 11, 03).and_hms(0, 29, 0),
            ),
            (
                "2018-11-01 00:00",
                NaiveDate::from_ymd(2018, 11, 01).and_hms(0, 0, 0),
            ),
            (
                "1118-11-11 20:00",
                NaiveDate::from_ymd(1118, 11, 11).and_hms(20, 0, 0),
            ),
        ];

        let bad_cases = ["1518-11-01 25:00", "1518-11 00:00", "1518-11-32 00:00"];

        for (input, expected) in good_cases.iter() {
            assert_eq!(
                parse_datetime(CompleteStr(input)),
                Ok((CompleteStr(""), *expected))
            );
        }

        for case in bad_cases.iter() {
            assert!(parse_datetime(CompleteStr(case)).is_err());
        }
    }

    #[test]
    fn test_parse_action() {
        let cases = [
            ("Guard #10 begins shift", Action::BeginsShift(10)),
            ("falls asleep", Action::FallsAsleep),
            ("wakes up", Action::WakesUp),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(
                parse_action(CompleteStr(input)),
                Ok((CompleteStr(""), *expected))
            );
        }
    }

    #[test]
    fn test_parse_log_entry() {
        let cases = [
            (
                "[1518-11-01 00:00] Guard #10 begins shift",
                LogEntry {
                    datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 0, 0),
                    action: Action::BeginsShift(10),
                },
            ),
            (
                "[1518-11-01 00:05] falls asleep",
                LogEntry {
                    datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 5, 0),
                    action: Action::FallsAsleep,
                },
            ),
            (
                "[1518-11-01 00:25] wakes up",
                LogEntry {
                    datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 25, 0),
                    action: Action::WakesUp,
                },
            ),
            (
                "[1518-11-01 00:30] falls asleep",
                LogEntry {
                    datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 30, 0),
                    action: Action::FallsAsleep,
                },
            ),
            (
                "[1518-11-01 00:55] wakes up",
                LogEntry {
                    datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 55, 0),
                    action: Action::WakesUp,
                },
            ),
            (
                "[1518-11-01 23:58] Guard #99 begins shift",
                LogEntry {
                    datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(23, 58, 0),
                    action: Action::BeginsShift(99),
                },
            ),
        ];

        for (input, expected) in cases.iter() {
            assert_eq!(
                parse_log_entry(CompleteStr(input)),
                Ok((CompleteStr(""), *expected))
            );
        }
    }

    #[test]
    fn test_parse() {
        let correct_log_entry = "[1518-11-01 23:58] Guard #99 begins shift";
        let correct_parsed = LogEntry {
            datetime: NaiveDate::from_ymd(1518, 11, 01).and_hms(23, 58, 0),
            action: Action::BeginsShift(99),
        };

        let incorrect_log_entry = "[1518-11-01 23:58] Guards #99 begins shift";

        assert_eq!(correct_log_entry.parse(), Ok(correct_parsed));
        assert_eq!(
            incorrect_log_entry.parse::<LogEntry>(),
            Err(nom::ErrorKind::Alt)
        );
    }

    #[test]
    fn test_log_order() {
        let log_second = "[1518-11-01 00:30] falls asleep"
            .parse::<LogEntry>()
            .unwrap();
        let log_third = "[1518-11-01 00:55] wakes up".parse::<LogEntry>().unwrap();
        let log_first = "[1518-11-01 00:25] wakes up".parse::<LogEntry>().unwrap();

        assert!(log_first < log_second);
        assert!(log_second < log_third);
        assert!(log_first < log_third);
        assert!(log_second > log_first);
        assert!(log_third > log_second);
        assert!(log_third > log_first);
    }
}
