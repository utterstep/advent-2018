use chrono::DateTime;

use nom::{i32, call, do_parse, error_position, map_res, named, opt, tag, take_till, types::CompleteStr};

pub enum Action {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

pub struct LogEntry {
    action: Action,
    time: DateTime,
}

fn right_bracket(c: char) -> bool {
  c == ']'
}

named!(extract_date     <CompleteStr, &str>,
  do_parse!(
          tag!("[")                 >>
    name: take_till!(right_bracket) >>
          tag!("]")                 >>
          opt!(space_or_line_ending)  >>
    (name.0)
  )
);
